// This file is part of Substrate.

// Copyright (C) 2018-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This crate provides means to instantiate and execute wasm modules.
//!
//! It works even when the user of this library executes from
//! inside the wasm VM. In this case the same VM is used for execution
//! of both the sandbox owner and the sandboxed module, without compromising security
//! and without the performance penalty of full wasm emulation inside wasm.
//!
//! This is achieved by using bindings to the wasm VM, which are published by the host API.
//! This API is thin and consists of only a handful functions. It contains functions for
//! instantiating modules and executing them, but doesn't contain functions for inspecting the
//! module structure. The user of this library is supposed to read the wasm module.
//!
//! When this crate is used in the `std` environment all these functions are implemented by directly
//! calling the wasm VM.
//!
//! Examples of possible use-cases for this library are not limited to the following:
//!
//! - implementing smart-contract runtimes that use wasm for contract code
//! - executing a wasm substrate runtime inside of a wasm parachain

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod embedded_executor;
pub mod env;

use frame_support::dispatch::DispatchError;
use sp_core::RuntimeDebug;
use sp_std::{prelude::*, vec};

pub use sp_wasm_interface::{ReturnValue, Value};
use wasmi::{ExternVal, NotStartedModuleRef};

#[cfg(not(all(feature = "wasmer-sandbox", not(feature = "std"))))]
pub use self::embedded_executor as default_executor;
pub use self::env::HostError;

/// The target used for logging.
const TARGET: &str = "runtime::sandbox";

/// Describes some data allocated in Wasm's linear memory.
/// A pointer to an instance of this can be returned over FFI boundaries.
///
/// This is the same as `cosmwasm_std::memory::Region`
/// but defined here to allow Wasmer specific implementation.
#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub struct Region {
	/// The beginning of the region expressed as bytes from the beginning of the linear memory
	offset: u32,
	/// The number of bytes available in this region
	capacity: u32,
	/// The number of bytes used in this region
	length: u32,
}

impl Region {
	/// Performs plausibility checks in the given Region. Regions are always created by the
	/// contract and this can be used to detect problems in the standard library of the contract.
	pub fn validate(&self) -> Result<(), MemoryError> {
		if self.offset == 0 {
			return Err(MemoryError::RegionIsNull)
		}
		if self.length > self.capacity {
			return Err(MemoryError::RegionInconsistent)
		}
		if self.capacity > (u32::MAX - self.offset) {
			return Err(MemoryError::RegionInconsistent)
		}
		Ok(())
	}
}

#[derive(RuntimeDebug)]
pub enum MemoryError {
	RegionLengthMismatch,
	RegionIsNull,
	RegionInconsistent,
	MemoryWriteFailure,
	MemoryReadFailure,
}

impl Into<DispatchError> for MemoryError {
	fn into(self) -> DispatchError {
		DispatchError::Other(match self {
			MemoryError::RegionLengthMismatch => "region length mismatch",
			MemoryError::RegionIsNull => "region is null",
			MemoryError::RegionInconsistent => "region is inconsistent",
			MemoryError::MemoryWriteFailure => "memory write failure",
			MemoryError::MemoryReadFailure => "memory read failure",
		})
	}
}

/// Error that can occur while using this crate.
#[derive(RuntimeDebug)]
pub enum Error {
	/// Module is not valid, couldn't be instantiated.
	Module,

	/// Access to a memory or table was made with an address or an index which is out of bounds.
	///
	/// Note that if wasm module makes an out-of-bounds access then trap will occur.
	OutOfBounds,

	/// Failed to invoke the start function or an exported function for some reason.
	Execution,

	Memory(MemoryError),
}

impl From<Error> for HostError {
	fn from(_e: Error) -> HostError {
		HostError
	}
}

/// Function pointer for specifying functions by the
/// supervisor in [`EnvironmentDefinitionBuilder`].
///
/// [`EnvironmentDefinitionBuilder`]: struct.EnvironmentDefinitionBuilder.html
pub type HostFuncType<T> = fn(&mut T, &[Value]) -> Result<ReturnValue, HostError>;

/// Reference to a sandboxed linear memory, that
/// will be used by the guest module.
///
/// The memory can't be directly accessed by supervisor, but only
/// through designated functions [`get`](SandboxMemory::get) and [`set`](SandboxMemory::set).
pub trait SandboxMemory: Sized + Clone {
	/// Construct a new linear memory instance.
	///
	/// The memory allocated with initial number of pages specified by `initial`.
	/// Minimal possible value for `initial` is 0 and maximum possible is `65536`.
	/// (Since maximum addressable memory is 2<sup>32</sup> = 4GiB = 65536 * 64KiB).
	///
	/// It is possible to limit maximum number of pages this memory instance can have by specifying
	/// `maximum`. If not specified, this memory instance would be able to allocate up to 4GiB.
	///
	/// Allocated memory is always zeroed.
	fn new(initial: u32, maximum: Option<u32>) -> Result<Self, Error>;

	/// Read a memory area at the address `ptr` with the size of the provided slice `buf`.
	///
	/// Returns `Err` if the range is out-of-bounds.
	fn get(&self, ptr: u32, buf: &mut [u8]) -> Result<(), MemoryError>;

	/// Write a memory area at the address `ptr` with contents of the provided slice `buf`.
	///
	/// Returns `Err` if the range is out-of-bounds.
	fn set(&self, ptr: u32, value: &[u8]) -> Result<(), MemoryError>;

	/// Read a value at memory address ptr
	fn typed_get<T>(&self, ptr: u32) -> Result<T, MemoryError> {
		if core::mem::size_of::<T>() == 0 {
			Err(MemoryError::RegionIsNull)
		} else {
			let mut t: T = unsafe { core::mem::zeroed() };
			let b = unsafe {
				core::slice::from_raw_parts_mut(
					&mut t as *mut T as *mut u8,
					core::mem::size_of::<T>(),
				)
			};
			self.get(ptr, b)?;
			Ok(t)
		}
	}

	/// Set a value at memory address ptr
	fn typed_set<T>(&self, ptr: u32, t: &T) -> Result<(), MemoryError> {
		if core::mem::size_of::<T>() == 0 {
			Err(MemoryError::RegionIsNull)
		} else {
			let b = unsafe {
				core::slice::from_raw_parts(t as *const T as *const u8, core::mem::size_of::<T>())
			};
			self.set(ptr, b)?;
			Ok(())
		}
	}

	/// Expects a (fixed size) Region struct at ptr, which is read. This links to the
	/// memory region, which is copied in the second step.
	/// Errors if the length of the region exceeds `max_length`.
	fn read_region(&self, ptr: u32, max_length: usize) -> Result<Vec<u8>, MemoryError> {
		let region = self.get_region(ptr)?;
		if region.length > max_length as u32 {
			return Err(MemoryError::RegionLengthMismatch)
		}
		log::debug!(target: "runtime::contracts", "Region: {:?}", region);
		let mut data = vec![0u8; region.length as usize];
		self.get(region.offset, &mut data)
			.map_err(|_| (MemoryError::MemoryReadFailure))?;
		Ok(data)
	}

	/// A prepared and sufficiently large memory Region is expected at ptr that points to
	/// pre-allocated memory.
	///
	/// Returns number of bytes written on success.
	fn write_region(&self, ptr: u32, data: &[u8]) -> Result<(), MemoryError> {
		let mut region = self.get_region(ptr)?;
		let region_capacity = region.capacity as usize;
		if data.len() > region_capacity {
			return Err(MemoryError::RegionLengthMismatch)
		}
		log::debug!(target: "runtime::contracts", "Region: {:?}", region);
		self.set(region.offset, data).map_err(|_| MemoryError::MemoryWriteFailure)?;
		region.length = data.len() as u32;
		self.set_region(ptr, &region)?;
		Ok(())
	}

	/// Reads in a Region at ptr in wasm memory and returns a copy of it
	fn get_region(&self, ptr: u32) -> Result<Region, MemoryError> {
		self.typed_get::<Region>(ptr).map_err(|_| MemoryError::MemoryReadFailure)
	}

	/// Overrides a Region at ptr in wasm memory with data
	fn set_region(&self, ptr: u32, data: &Region) -> Result<(), MemoryError> {
		self.typed_set::<Region>(ptr, data).map_err(|_| MemoryError::MemoryWriteFailure)
	}
}

/// Struct that can be used for defining an environment for a sandboxed module.
///
/// The sandboxed module can access only the entities which were defined and passed
/// to the module at the instantiation time.
pub trait SandboxEnvironmentBuilder<State, Memory>: Sized {
	/// Construct a new `EnvironmentDefinitionBuilder`.
	fn new() -> Self;

	/// Register a host function in this environment definition.
	///
	/// NOTE that there is no constraints on type of this function. An instance
	/// can import function passed here with any signature it wants. It can even import
	/// the same function (i.e. with same `module` and `field`) several times. It's up to
	/// the user code to check or constrain the types of signatures.
	fn add_host_func<N1, N2>(&mut self, module: N1, field: N2, f: HostFuncType<State>)
	where
		N1: Into<Vec<u8>>,
		N2: Into<Vec<u8>>;

	/// Register a memory in this environment definition.
	fn add_memory<N1, N2>(&mut self, module: N1, field: N2, mem: Memory)
	where
		N1: Into<Vec<u8>>,
		N2: Into<Vec<u8>>;
}

/// Sandboxed instance of a wasm module.
///
/// This instance can be used for invoking exported functions.
pub trait SandboxInstance<State>: Sized {
	/// The memory type used for this sandbox.
	type Memory: SandboxMemory;

	/// The environment builder used to construct this sandbox.
	type EnvironmentBuilder: SandboxEnvironmentBuilder<State, Self::Memory>;

	/// Instantiate a module with the given [`EnvironmentDefinitionBuilder`]. It will
	/// run the `start` function (if it is present in the module) with the given `state`.
	///
	/// Returns `Err(Error::Module)` if this module can't be instantiated with the given
	/// environment. If execution of `start` function generated a trap, then `Err(Error::Execution)`
	/// will be returned.
	///
	/// [`EnvironmentDefinitionBuilder`]: struct.EnvironmentDefinitionBuilder.html
	fn new(
		code: &[u8],
		env_def_builder: &Self::EnvironmentBuilder,
		mk_state: impl FnOnce(&NotStartedModuleRef) -> State,
	) -> Result<(Self, State), Error>;

	/// Invoke an exported function with the given name.
	///
	/// # Errors
	///
	/// Returns `Err(Error::Execution)` if:
	///
	/// - An export function name isn't a proper utf8 byte sequence,
	/// - This module doesn't have an exported function with the given name,
	/// - If types of the arguments passed to the function doesn't match function signature then
	///   trap occurs (as if the exported function was called via call_indirect),
	/// - Trap occurred at the execution time.
	fn invoke(
		&mut self,
		name: &str,
		args: &[Value],
		state: &mut State,
	) -> Result<ReturnValue, Error>;

	/// Get the value from a global with the given `name`.
	///
	/// Returns `Some(_)` if the global could be found.
	fn get_global_val(&self, name: &str) -> Option<Value>;

	/// Get export
	fn get_export(&self, name: &str) -> Option<ExternVal>;
}

#[cfg(test)]
mod tests {
	use super::{MemoryError, Region};

	#[test]
	fn validate_region_passes_for_valid_region() {
		// empty
		let region = Region { offset: 23, capacity: 500, length: 0 };
		region.validate().unwrap();

		// half full
		let region = Region { offset: 23, capacity: 500, length: 250 };
		region.validate().unwrap();

		// full
		let region = Region { offset: 23, capacity: 500, length: 500 };
		region.validate().unwrap();

		// at end of linear memory (1)
		let region = Region { offset: u32::MAX, capacity: 0, length: 0 };
		region.validate().unwrap();

		// at end of linear memory (2)
		let region = Region { offset: 1, capacity: u32::MAX - 1, length: 0 };
		region.validate().unwrap();
	}

	#[test]
	fn validate_region_fails_for_zero_offset() {
		let region = Region { offset: 0, capacity: 500, length: 250 };
		let result = region.validate();
		match result.unwrap_err() {
			MemoryError::RegionIsNull => {},
			e => panic!("Got unexpected error: {:?}", e),
		}
	}

	#[test]
	fn validate_region_fails_for_length_exceeding_capacity() {
		let region = Region { offset: 23, capacity: 500, length: 501 };
		let result = region.validate();
		match result.unwrap_err() {
			MemoryError::RegionInconsistent => {},
			e => panic!("Got unexpected error: {:?}", e),
		}
	}

	#[test]
	fn validate_region_fails_when_exceeding_address_space() {
		let region = Region { offset: 23, capacity: u32::MAX, length: 501 };
		let result = region.validate();
		match result.unwrap_err() {
			MemoryError::RegionInconsistent => {},
			e => panic!("Got unexpected error: {:?}", e),
		}

		let region = Region { offset: u32::MAX, capacity: 1, length: 0 };
		let result = region.validate();
		match result.unwrap_err() {
			MemoryError::RegionInconsistent => {},
			e => panic!("Got unexpected error: {:?}", e),
		}
	}
}
