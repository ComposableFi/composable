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

//! Environment definition of the wasm smart-contract runtime.

use crate::{
	exec::{ExecError, ExecResult, Ext, StorageKey, TopicOf},
	gas::{ChargedAmount, Token},
	schedule::HostFnWeights,
	wasm::{
		cosmwasm::types::{DeserializeLimit, ExecuteResult, QueryResult},
		env_def::ConvertibleToWasm,
	},
	BalanceOf, CodeHash, Config, Error, SENTINEL,
};
use bitflags::bitflags;
use codec::{Decode, DecodeAll, Encode, MaxEncodedLen};
use frame_support::{dispatch::DispatchError, ensure, weights::Weight};
use pallet_contracts_primitives::{ExecReturnValue, ReturnFlags};
use sp_core::{crypto::UncheckedFrom, Bytes};
use sp_io::hashing::{blake2_128, blake2_256, keccak_256, sha2_256};
use sp_runtime::{
	traits::{Bounded, Zero},
	ArithmeticError,
};
use sp_sandbox::{
	default_executor::{
		DefinedHostFunctions, EnvironmentDefinitionBuilder, GuestExternals, Memory,
	},
	SandboxMemory,
};
use sp_std::{prelude::*, vec};
use wasm_instrument::parity_wasm::elements::ValueType;
use wasmi::{
	ExternVal, FuncInstance, Module, ModuleInstance, ModuleRef, NopExternals, RuntimeValue, Trap,
};

use super::cosmwasm::types::{InstantiateResult, MessageInfo};

/// Every error that can be returned to a contract when it calls any of the host functions.
///
/// # Note
///
/// This enum can be extended in the future: New codes can be added but existing codes
/// will not be changed or removed. This means that any contract **must not** exhaustively
/// match return codes. Instead, contracts should prepare for unknown variants and deal with
/// those errors gracefuly in order to be forward compatible.
#[repr(u32)]
pub enum ReturnCode {
	/// API call successful.
	Success = 0,
	/// The called function trapped and has its state changes reverted.
	/// In this case no output buffer is returned.
	CalleeTrapped = 1,
	/// The called function ran to completion but decided to revert its state.
	/// An output buffer is returned when one was supplied.
	CalleeReverted = 2,
	/// The passed key does not exist in storage.
	KeyNotFound = 3,
	/// Deprecated and no longer returned: There is only the minimum balance.
	_BelowSubsistenceThreshold = 4,
	/// See [`Error::TransferFailed`].
	TransferFailed = 5,
	/// Deprecated and no longer returned: Endowment is no longer required.
	_EndowmentTooLow = 6,
	/// No code could be found at the supplied code hash.
	CodeNotFound = 7,
	/// The contract that was called is no contract (a plain account).
	NotCallable = 8,
	/// The call to `seal_debug_message` had no effect because debug message
	/// recording was disabled.
	LoggingDisabled = 9,
	/// The call dispatched by `seal_call_runtime` was executed but returned an error.
	#[cfg(feature = "unstable-interface")]
	CallRuntimeReturnedError = 10,
	/// ECDSA pubkey recovery failed (most probably wrong recovery id or signature), or
	/// ECDSA compressed pubkey conversion into Ethereum address failed (most probably
	/// wrong pubkey provided).
	#[cfg(feature = "unstable-interface")]
	EcdsaRecoverFailed = 11,
}

impl ConvertibleToWasm for ReturnCode {
	type NativeType = Self;
	const VALUE_TYPE: ValueType = ValueType::I32;
	fn to_typed_value(self) -> sp_sandbox::Value {
		sp_sandbox::Value::I32(self as i32)
	}
	fn from_typed_value(_: sp_sandbox::Value) -> Option<Self> {
		debug_assert!(false, "We will never receive a ReturnCode but only send it to wasm.");
		None
	}
}

impl From<ExecReturnValue> for ReturnCode {
	fn from(from: ExecReturnValue) -> Self {
		if from.flags.contains(ReturnFlags::REVERT) {
			Self::CalleeReverted
		} else {
			Self::Success
		}
	}
}

/// The data passed through when a contract uses `seal_return`.
pub struct ReturnData {
	/// The flags as passed through by the contract. They are still unchecked and
	/// will later be parsed into a `ReturnFlags` bitflags struct.
	flags: u32,
	/// The output buffer passed by the contract as return data.
	data: Vec<u8>,
}

/// Enumerates all possible reasons why a trap was generated.
///
/// This is either used to supply the caller with more information about why an error
/// occurred (the SupervisorError variant).
/// The other case is where the trap does not constitute an error but rather was invoked
/// as a quick way to terminate the application (all other variants).
pub enum TrapReason {
	/// The supervisor trapped the contract because of an error condition occurred during
	/// execution in privileged code.
	SupervisorError(DispatchError),
	/// Signals that trap was generated in response to call `seal_return` host function.
	Return(ReturnData),
	/// Signals that a trap was generated in response to a successful call to the
	/// `seal_terminate` host function.
	Termination,
}

impl<T: Into<DispatchError>> From<T> for TrapReason {
	fn from(from: T) -> Self {
		Self::SupervisorError(from.into())
	}
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
#[derive(Copy, Clone)]
pub enum RuntimeCosts {
	/// Charge the gas meter with the cost of a metering block. The charged costs are
	/// the supplied cost of the block plus the overhead of the metering itself.
	MeteringBlock(u32),
	/// Weight charged for copying data from the sandbox.
	CopyFromContract(u32),
	/// Weight charged for copying data to the sandbox.
	CopyToContract(u32),
	/// Weight of calling `seal_caller`.
	Caller,
	/// Weight of calling `seal_is_contract`.
	IsContract,
	/// Weight of calling `seal_code_hash`.
	#[cfg(feature = "unstable-interface")]
	CodeHash,
	/// Weight of calling `seal_own_code_hash`.
	#[cfg(feature = "unstable-interface")]
	OwnCodeHash,
	/// Weight of calling `seal_caller_is_origin`.
	CallerIsOrigin,
	/// Weight of calling `seal_address`.
	Address,
	/// Weight of calling `seal_gas_left`.
	GasLeft,
	/// Weight of calling `seal_balance`.
	Balance,
	/// Weight of calling `seal_value_transferred`.
	ValueTransferred,
	/// Weight of calling `seal_minimum_balance`.
	MinimumBalance,
	/// Weight of calling `seal_block_number`.
	BlockNumber,
	/// Weight of calling `seal_now`.
	Now,
	/// Weight of calling `seal_weight_to_fee`.
	WeightToFee,
	/// Weight of calling `seal_input` without the weight of copying the input.
	InputBase,
	/// Weight of calling `seal_return` for the given output size.
	Return(u32),
	/// Weight of calling `seal_terminate`.
	Terminate,
	/// Weight of calling `seal_random`. It includes the weight for copying the subject.
	Random,
	/// Weight of calling `seal_deposit_event` with the given number of topics and event size.
	DepositEvent { num_topic: u32, len: u32 },
	/// Weight of calling `seal_debug_message`.
	DebugMessage,
	/// Weight of calling `seal_set_storage` for the given storage item sizes.
	SetStorage { old_bytes: u32, new_bytes: u32 },
	/// Weight of calling `seal_clear_storage` per cleared byte.
	ClearStorage(u32),
	/// Weight of calling `seal_contains_storage` per byte of the checked item.
	#[cfg(feature = "unstable-interface")]
	ContainsStorage(u32),
	/// Weight of calling `seal_get_storage` with the specified size in storage.
	GetStorage(u32),
	/// Weight of calling `seal_take_storage` for the given size.
	#[cfg(feature = "unstable-interface")]
	TakeStorage(u32),
	/// Weight of calling `seal_transfer`.
	Transfer,
	/// Base weight of calling `seal_call`.
	CallBase,
	/// Weight of calling `seal_delegate_call` for the given input size.
	DelegateCallBase,
	/// Weight of the transfer performed during a call.
	CallSurchargeTransfer,
	/// Weight per byte that is cloned by supplying the `CLONE_INPUT` flag.
	CallInputCloned(u32),
	/// Weight of calling `seal_instantiate` for the given input length and salt.
	InstantiateBase { input_data_len: u32, salt_len: u32 },
	/// Weight of the transfer performed during an instantiate.
	InstantiateSurchargeTransfer,
	/// Weight of calling `seal_hash_sha_256` for the given input size.
	HashSha256(u32),
	/// Weight of calling `seal_hash_keccak_256` for the given input size.
	HashKeccak256(u32),
	/// Weight of calling `seal_hash_blake2_256` for the given input size.
	HashBlake256(u32),
	/// Weight of calling `seal_hash_blake2_128` for the given input size.
	HashBlake128(u32),
	/// Weight of calling `seal_ecdsa_recover`.
	#[cfg(feature = "unstable-interface")]
	EcdsaRecovery,
	/// Weight charged by a chain extension through `seal_call_chain_extension`.
	ChainExtension(u64),
	/// Weight charged for calling into the runtime.
	#[cfg(feature = "unstable-interface")]
	CallRuntime(Weight),
	/// Weight of calling `seal_set_code_hash`
	#[cfg(feature = "unstable-interface")]
	SetCodeHash,
	/// Weight of calling `ecdsa_to_eth_address`
	#[cfg(feature = "unstable-interface")]
	EcdsaToEthAddress,
}

impl RuntimeCosts {
	fn token<T>(&self, s: &HostFnWeights<T>) -> RuntimeToken
	where
		T: Config,
		T::AccountId: UncheckedFrom<T::Hash> + AsRef<[u8]>,
	{
		use self::RuntimeCosts::*;
		let weight = match *self {
			MeteringBlock(amount) => s.gas.saturating_add(amount.into()),
			CopyFromContract(len) => s.return_per_byte.saturating_mul(len.into()),
			CopyToContract(len) => s.input_per_byte.saturating_mul(len.into()),
			Caller => s.caller,
			IsContract => s.is_contract,
			#[cfg(feature = "unstable-interface")]
			Hash => s.code_hash,
			#[cfg(feature = "unstable-interface")]
			OwnCodeHash => s.own_code_hash,
			CallerIsOrigin => s.caller_is_origin,
			Address => s.address,
			GasLeft => s.gas_left,
			Balance => s.balance,
			ValueTransferred => s.value_transferred,
			MinimumBalance => s.minimum_balance,
			BlockNumber => s.block_number,
			Now => s.now,
			WeightToFee => s.weight_to_fee,
			InputBase => s.input,
			Return(len) => s.r#return.saturating_add(s.return_per_byte.saturating_mul(len.into())),
			Terminate => s.terminate,
			Random => s.random,
			DepositEvent { num_topic, len } => s
				.deposit_event
				.saturating_add(s.deposit_event_per_topic.saturating_mul(num_topic.into()))
				.saturating_add(s.deposit_event_per_byte.saturating_mul(len.into())),
			DebugMessage => s.debug_message,
			SetStorage { new_bytes, old_bytes } => s
				.set_storage
				.saturating_add(s.set_storage_per_new_byte.saturating_mul(new_bytes.into()))
				.saturating_add(s.set_storage_per_old_byte.saturating_mul(old_bytes.into())),
			ClearStorage(len) => s
				.clear_storage
				.saturating_add(s.clear_storage_per_byte.saturating_mul(len.into())),
			#[cfg(feature = "unstable-interface")]
			ContainsStorage(len) => s
				.contains_storage
				.saturating_add(s.contains_storage_per_byte.saturating_mul(len.into())),
			GetStorage(len) =>
				s.get_storage.saturating_add(s.get_storage_per_byte.saturating_mul(len.into())),
			#[cfg(feature = "unstable-interface")]
			TakeStorage(len) => s
				.take_storage
				.saturating_add(s.take_storage_per_byte.saturating_mul(len.into())),
			Transfer => s.transfer,
			CallBase => s.call,
			DelegateCallBase => s.delegate_call,
			CallSurchargeTransfer => s.call_transfer_surcharge,
			CallInputCloned(len) => s.call_per_cloned_byte.saturating_mul(len.into()),
			InstantiateBase { input_data_len, salt_len } => s
				.instantiate
				.saturating_add(s.return_per_byte.saturating_mul(input_data_len.into()))
				.saturating_add(s.instantiate_per_salt_byte.saturating_mul(salt_len.into())),
			InstantiateSurchargeTransfer => s.instantiate_transfer_surcharge,
			HashSha256(len) => s
				.hash_sha2_256
				.saturating_add(s.hash_sha2_256_per_byte.saturating_mul(len.into())),
			HashKeccak256(len) => s
				.hash_keccak_256
				.saturating_add(s.hash_keccak_256_per_byte.saturating_mul(len.into())),
			HashBlake256(len) => s
				.hash_blake2_256
				.saturating_add(s.hash_blake2_256_per_byte.saturating_mul(len.into())),
			HashBlake128(len) => s
				.hash_blake2_128
				.saturating_add(s.hash_blake2_128_per_byte.saturating_mul(len.into())),
			#[cfg(feature = "unstable-interface")]
			EcdsaRecovery => s.ecdsa_recover,
			ChainExtension(amount) => amount,

			#[cfg(feature = "unstable-interface")]
			CallRuntime(weight) => weight,
			#[cfg(feature = "unstable-interface")]
			SetCodeHash => s.set_code_hash,
			#[cfg(feature = "unstable-interface")]
			EcdsaToEthAddress => s.ecdsa_to_eth_address,
		};
		RuntimeToken {
			#[cfg(test)]
			_created_from: *self,
			weight,
		}
	}
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
#[derive(Copy, Clone)]
struct RuntimeToken {
	#[cfg(test)]
	_created_from: RuntimeCosts,
	weight: Weight,
}

impl<T> Token<T> for RuntimeToken
where
	T: Config,
	T::AccountId: UncheckedFrom<T::Hash> + AsRef<[u8]>,
{
	fn weight(&self) -> Weight {
		self.weight
	}
}

bitflags! {
	/// Flags used to change the behaviour of `seal_call` and `seal_delegate_call`.
	pub struct CallFlags: u32 {
		/// Forward the input of current function to the callee.
		///
		/// Supplied input pointers are ignored when set.
		///
		/// # Note
		///
		/// A forwarding call will consume the current contracts input. Any attempt to
		/// access the input after this call returns will lead to [`Error::InputForwarded`].
		/// It does not matter if this is due to calling `seal_input` or trying another
		/// forwarding call. Consider using [`Self::CLONE_INPUT`] in order to preserve
		/// the input.
		const FORWARD_INPUT = 0b0000_0001;
		/// Identical to [`Self::FORWARD_INPUT`] but without consuming the input.
		///
		/// This adds some additional weight costs to the call.
		///
		/// # Note
		///
		/// This implies [`Self::FORWARD_INPUT`] and takes precedence when both are set.
		const CLONE_INPUT = 0b0000_0010;
		/// Do not return from the call but rather return the result of the callee to the
		/// callers caller.
		///
		/// # Note
		///
		/// This makes the current contract completely transparent to its caller by replacing
		/// this contracts potential output by the callee ones. Any code after `seal_call`
		/// can be safely considered unreachable.
		const TAIL_CALL = 0b0000_0100;
		/// Allow the callee to reenter into the current contract.
		///
		/// Without this flag any reentrancy into the current contract that originates from
		/// the callee (or any of its callees) is denied. This includes the first callee:
		/// You cannot call into yourself with this flag set.
		///
		/// # Note
		///
		/// For `seal_delegate_call` should be always unset, otherwise
		/// [`Error::InvalidCallFlags`] is returned.
		const ALLOW_REENTRY = 0b0000_1000;
	}
}

pub const INSTANTIATE_FUNCTION: &str = "instantiate";
pub const EXECUTE_FUNCTION: &str = "execute";
pub const QUERY_FUNCTION: &str = "query";
pub const ALLOCATE_FUNCTION: &str = "allocate";
pub const DEALLOCATE_FUNCTION: &str = "deallocate";

/// Can only be used for one call.
pub struct Runtime<'a, E: Ext + 'a> {
	ext: &'a mut E,
	memory: sp_sandbox::default_executor::Memory,
	instance: ModuleRef,
	defined_host_functions: DefinedHostFunctions<Self>,
	trap_reason: Option<TrapReason>,
}

impl<'a, E: Ext + 'a> Runtime<'a, E> {
	/// Get a mutable reference to the memory region of this contract.
	pub fn memory(&mut self) -> &mut sp_sandbox::default_executor::Memory {
		&mut self.memory
	}

	/// Invoke an exported function from the loaded module.
	pub fn invoke(
		&mut self,
		function: &str,
		args: &[RuntimeValue],
	) -> Result<Option<RuntimeValue>, DispatchError> {
		match self.instance.export_by_name(function) {
			Some(ExternVal::Func(func_instance)) => {
				let cloned = self.defined_host_functions.clone();
				let mut externals = GuestExternals { state: self, defined_host_functions: &cloned };
				FuncInstance::invoke(&func_instance, args, &mut externals)
					.map_err(|_| DispatchError::Other("Failed to invoke function"))
			},
			_ => Err(DispatchError::Other("Failed to find exported function")),
		}
	}

	/// Call the allocate function of the loaded cosmwasm contract.
	pub fn allocate<T: TryInto<i32>>(&mut self, len: T) -> Result<u32, DispatchError> {
		match self.invoke(
			ALLOCATE_FUNCTION,
			&[RuntimeValue::I32(
				len.try_into()
					.map_err(|_| DispatchError::Arithmetic(ArithmeticError::Overflow))?,
			)],
		) {
			Ok(Some(RuntimeValue::I32(ptr))) => Ok(ptr as u32),
			e => {
				log::debug!(target: "runtime::contracts", "Allocate failed {:?}", e);
				Err(DispatchError::Other("allocate failed"))
			},
		}
	}

	/// Call the deallocate function of the loaded cosmwasm contract.
	pub fn deallocate<T: TryInto<i32>>(&mut self, ptr: T) -> Result<(), DispatchError> {
		match self.invoke(
			DEALLOCATE_FUNCTION,
			&[RuntimeValue::I32(
				ptr.try_into()
					.map_err(|_| DispatchError::Arithmetic(ArithmeticError::Overflow))?,
			)],
		) {
			Ok(None) => Ok(()),
			e => {
				log::debug!(target: "runtime::contracts", "Deallocate failed {:?}", e);
				Err(DispatchError::Other("deallocate failed"))
			},
		}
	}

	/// Load a raw payload into a contract memory.
	pub fn passthrough_in(&mut self, data: &[u8]) -> Result<u32, DispatchError> {
		let ptr = self.allocate(data.len())?;
		self.memory()
			.write_region(ptr, &data)
			.map_err(|_| DispatchError::Other("could not write region"))?;
		Ok(ptr)
	}

	/// JSON marshaling of an arbitrary type into the contract memory.
	pub fn marshall_in<T>(&mut self, x: &T) -> Result<u32, DispatchError>
	where
		T: serde::ser::Serialize + ?Sized,
	{
		let serialized =
			serde_json::to_vec(x).map_err(|_| DispatchError::Other("couldn't serialize"))?;
		self.passthrough_in(&serialized)
	}

	/// JSON marshaling of an arbitrary type from the contract memory.
	pub fn marshall_out<T>(&mut self, ptr: u32) -> Result<T, DispatchError>
	where
		T: serde::de::DeserializeOwned + DeserializeLimit + ?Sized,
	{
		log::debug!(target: "runtime::contracts", "Marshall out");
		let value = self
			.memory()
			.read_region(ptr, T::deserialize_limit())
			.map_err(|_| DispatchError::Other("could not read region"))?;
		serde_json::from_slice(&value).map_err(|_| DispatchError::Other("couldn't deserialize"))
	}
}

impl<'a, E> Runtime<'a, E>
where
	E: Ext + 'a,
	<E::T as frame_system::Config>::AccountId:
		UncheckedFrom<<E::T as frame_system::Config>::Hash> + AsRef<[u8]>,
{
	pub fn new(
		ext: &'a mut E,
		code: &[u8],
		env_def_builder: &EnvironmentDefinitionBuilder<Self>,
	) -> Result<Self, DispatchError> {
		log::debug!(target: "runtime::contracts", "loading code from buffer");

		let module = Module::from_buffer(code).map_err(|_| DispatchError::Other(""))?;
		let not_started_instance =
			ModuleInstance::new(&module, env_def_builder).map_err(|_| DispatchError::Other(""))?;

		log::debug!(target: "runtime::contracts", "starting instance with no externals");
		let defined_host_functions = env_def_builder.defined_host_functions.clone();
		let instance = {
			let instance = not_started_instance
				.run_start(&mut NopExternals)
				.map_err(|_| DispatchError::Other(""))?;
			instance
		};

		let memory = match instance.export_by_name("memory") {
			Some(ExternVal::Memory(memory)) => {
				log::debug!(target: "runtime::contracts", "set internal memory");
				Ok(Memory::new(memory))
			},
			_ => Err(DispatchError::Other(
				"could not find memory export, must be impossible as checked on upload",
			)),
		}?;

		Ok(Runtime {
			ext,
			memory,
			instance,
			defined_host_functions,
			trap_reason: None,
		})
	}

	pub fn do_instantiate(
		&mut self,
		env: super::cosmwasm::types::Env,
		info: MessageInfo,
		message: &[u8],
	) -> Result<InstantiateResult, DispatchError> {
		let parameters =
			vec![self.marshall_in(&env)?, self.marshall_in(&info)?, self.passthrough_in(message)?]
				.into_iter()
				.map(|v| RuntimeValue::I32(v as i32))
				.collect::<Vec<_>>();
		match self.invoke(INSTANTIATE_FUNCTION, &parameters) {
			Ok(Some(RuntimeValue::I32(response_ptr))) => {
				let response = self.marshall_out::<InstantiateResult>(response_ptr as u32);
				self.deallocate(response_ptr)?;
				log::debug!(target: "runtime::contracts", "Instantiate done {:?}", response);
				response
			},
			e => {
				log::debug!(target: "runtime::contracts", "Instantiate failed {:?}", e);
				Err(DispatchError::Other("could not instantiate"))
			},
		}
	}

	pub fn do_execute(
		&mut self,
		env: super::cosmwasm::types::Env,
		info: MessageInfo,
		message: &[u8],
	) -> Result<ExecuteResult, DispatchError> {
		let parameters =
			vec![self.marshall_in(&env)?, self.marshall_in(&info)?, self.passthrough_in(message)?]
				.into_iter()
				.map(|v| RuntimeValue::I32(v as i32))
				.collect::<Vec<_>>();
		match self.invoke(EXECUTE_FUNCTION, &parameters) {
			Ok(Some(RuntimeValue::I32(response_ptr))) => {
				let response = self.marshall_out::<ExecuteResult>(response_ptr as u32);
				self.deallocate(response_ptr)?;
				log::debug!(target: "runtime::contracts", "Execute done {:?}", response);
				response
			},
			e => {
				log::debug!(target: "runtime::contracts", "Execute failed {:?}", e);
				Err(DispatchError::Other("could not execute"))
			},
		}
	}

	pub fn do_query(
		&mut self,
		env: super::cosmwasm::types::Env,
		message: &[u8],
	) -> Result<QueryResult, DispatchError> {
		let parameters = vec![self.marshall_in(&env)?, self.passthrough_in(message)?]
			.into_iter()
			.map(|v| RuntimeValue::I32(v as i32))
			.collect::<Vec<_>>();
		match self.invoke(QUERY_FUNCTION, &parameters) {
			Ok(Some(RuntimeValue::I32(response_ptr))) => {
				let response = self.marshall_out::<QueryResult>(response_ptr as u32);
				self.deallocate(response_ptr)?;
				log::debug!(target: "runtime::contracts", "Query done {:?}", response);
				response
			},
			e => {
				log::debug!(target: "runtime::contracts", "Query failed {:?}", e);
				Err(DispatchError::Other("could not execute"))
			},
		}
	}

	/// Get a mutable reference to the inner `Ext`.
	///
	/// This is mainly for the chain extension to have access to the environment the
	/// contract is executing in.
	pub fn ext(&mut self) -> &mut E {
		self.ext
	}

	/// Store the reason for a host function triggered trap.
	///
	/// This is called by the `define_env` macro in order to store any error returned by
	/// the host functions defined through the said macro. It should **not** be called
	/// manually.
	pub fn set_trap_reason(&mut self, reason: TrapReason) {
		self.trap_reason = Some(reason);
	}

	/// Charge the gas meter with the specified token.
	///
	/// Returns `Err(HostError)` if there is not enough gas.
	pub fn charge_gas(&mut self, costs: RuntimeCosts) -> Result<ChargedAmount, DispatchError> {
		let token = costs.token(&self.ext.schedule().host_fn_weights);
		self.ext.gas_meter().charge(token)
	}

	/// Adjust a previously charged amount down to its actual amount.
	///
	/// This is when a maximum a priori amount was charged and then should be partially
	/// refunded to match the actual amount.
	pub fn adjust_gas(&mut self, charged: ChargedAmount, actual_costs: RuntimeCosts) {
		let token = actual_costs.token(&self.ext.schedule().host_fn_weights);
		self.ext.gas_meter().adjust_gas(charged, token);
	}

	/// Read designated chunk from the sandbox memory.
	///
	/// Returns `Err` if one of the following conditions occurs:
	///
	/// - requested buffer is not within the bounds of the sandbox memory.
	pub fn read_sandbox_memory(&self, ptr: u32, len: u32) -> Result<Vec<u8>, DispatchError> {
		ensure!(len <= self.ext.schedule().limits.max_memory_size(), Error::<E::T>::OutOfBounds);
		let mut buf = vec![0u8; len as usize];
		self.memory
			.get(ptr, buf.as_mut_slice())
			.map_err(|_| Error::<E::T>::OutOfBounds)?;
		Ok(buf)
	}

	/// Read designated chunk from the sandbox memory into the supplied buffer.
	///
	/// Returns `Err` if one of the following conditions occurs:
	///
	/// - requested buffer is not within the bounds of the sandbox memory.
	pub fn read_sandbox_memory_into_buf(
		&self,
		ptr: u32,
		buf: &mut [u8],
	) -> Result<(), DispatchError> {
		self.memory.get(ptr, buf).map_err(|_| Error::<E::T>::OutOfBounds.into())
	}

	/// Reads and decodes a type with a size fixed at compile time from contract memory.
	///
	/// # Note
	///
	/// The weight of reading a fixed value is included in the overall weight of any
	/// contract callable function.
	pub fn read_sandbox_memory_as<D: Decode + MaxEncodedLen>(
		&self,
		ptr: u32,
	) -> Result<D, DispatchError> {
		let buf = self.read_sandbox_memory(ptr, D::max_encoded_len() as u32)?;
		let decoded = D::decode_all(&mut &buf[..])
			.map_err(|_| DispatchError::from(Error::<E::T>::DecodingFailed))?;
		Ok(decoded)
	}

	/// Read designated chunk from the sandbox memory and attempt to decode into the specified type.
	///
	/// Returns `Err` if one of the following conditions occurs:
	///
	/// - requested buffer is not within the bounds of the sandbox memory.
	/// - the buffer contents cannot be decoded as the required type.
	///
	/// # Note
	///
	/// There must be an extra benchmark for determining the influence of `len` with
	/// regard to the overall weight.
	pub fn read_sandbox_memory_as_unbounded<D: Decode>(
		&self,
		ptr: u32,
		len: u32,
	) -> Result<D, DispatchError> {
		let buf = self.read_sandbox_memory(ptr, len)?;
		let decoded = D::decode_all(&mut &buf[..])
			.map_err(|_| DispatchError::from(Error::<E::T>::DecodingFailed))?;
		Ok(decoded)
	}

	/// Write the given buffer and its length to the designated locations in sandbox memory and
	/// charge gas according to the token returned by `create_token`.
	//
	/// `out_ptr` is the location in sandbox memory where `buf` should be written to.
	/// `out_len_ptr` is an in-out location in sandbox memory. It is read to determine the
	/// length of the buffer located at `out_ptr`. If that buffer is large enough the actual
	/// `buf.len()` is written to this location.
	///
	/// If `out_ptr` is set to the sentinel value of `SENTINEL` and `allow_skip` is true the
	/// operation is skipped and `Ok` is returned. This is supposed to help callers to make copying
	/// output optional. For example to skip copying back the output buffer of an `seal_call`
	/// when the caller is not interested in the result.
	///
	/// `create_token` can optionally instruct this function to charge the gas meter with the token
	/// it returns. `create_token` receives the variable amount of bytes that are about to be copied
	/// by this function.
	///
	/// In addition to the error conditions of `write_sandbox_memory` this functions returns
	/// `Err` if the size of the buffer located at `out_ptr` is too small to fit `buf`.
	pub fn write_sandbox_output(
		&mut self,
		out_ptr: u32,
		out_len_ptr: u32,
		buf: &[u8],
		allow_skip: bool,
		create_token: impl FnOnce(u32) -> Option<RuntimeCosts>,
	) -> Result<(), DispatchError> {
		if allow_skip && out_ptr == SENTINEL {
			return Ok(())
		}

		let buf_len = buf.len() as u32;
		let len: u32 = self.read_sandbox_memory_as(out_len_ptr)?;

		if len < buf_len {
			Err(Error::<E::T>::OutputBufferTooSmall)?
		}

		if let Some(costs) = create_token(buf_len) {
			self.charge_gas(costs)?;
		}

		self.memory
			.set(out_ptr, buf)
			.and_then(|_| self.memory.set(out_len_ptr, &buf_len.encode()))
			.map_err(|_| Error::<E::T>::OutOfBounds)?;

		Ok(())
	}

	/// Write the given buffer to the designated location in the sandbox memory.
	///
	/// Returns `Err` if one of the following conditions occurs:
	///
	/// - designated area is not within the bounds of the sandbox memory.
	fn write_sandbox_memory(&mut self, ptr: u32, buf: &[u8]) -> Result<(), DispatchError> {
		self.memory.set(ptr, buf).map_err(|_| Error::<E::T>::OutOfBounds.into())
	}

	/// Computes the given hash function on the supplied input.
	///
	/// Reads from the sandboxed input buffer into an intermediate buffer.
	/// Returns the result directly to the output buffer of the sandboxed memory.
	///
	/// It is the callers responsibility to provide an output buffer that
	/// is large enough to hold the expected amount of bytes returned by the
	/// chosen hash function.
	///
	/// # Note
	///
	/// The `input` and `output` buffers may overlap.
	fn compute_hash_on_intermediate_buffer<F, R>(
		&mut self,
		hash_fn: F,
		input_ptr: u32,
		input_len: u32,
		output_ptr: u32,
	) -> Result<(), DispatchError>
	where
		F: FnOnce(&[u8]) -> R,
		R: AsRef<[u8]>,
	{
		// Copy input into supervisor memory.
		let input = self.read_sandbox_memory(input_ptr, input_len)?;
		// Compute the hash on the input buffer using the given hash function.
		let hash = hash_fn(&input);
		// Write the resulting hash back into the sandboxed output buffer.
		self.write_sandbox_memory(output_ptr, hash.as_ref())?;
		Ok(())
	}
}

/// A kibi (kilo binary)
const KI: u32 = 1024;
/// A mibi (mega binary)
const MI: u32 = 1024 * 1024;
/// Max key length for db_write/db_read/db_remove/db_scan (when VM reads the key argument from Wasm
/// memory)
const MAX_LENGTH_DB_KEY: u32 = 64 * KI;
/// Max value length for db_write (when VM reads the value argument from Wasm memory)
const MAX_LENGTH_DB_VALUE: u32 = 128 * KI;
/// Typically 20 (Cosmos SDK, Ethereum), 32 (Nano, Substrate) or 54 (MockApi)
const MAX_LENGTH_CANONICAL_ADDRESS: u32 = 64;
/// The max length of human address inputs (in bytes).
/// The maximum allowed size for [bech32](https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki#bech32)
/// is 90 characters and we're adding some safety margin around that for other formats.
const MAX_LENGTH_HUMAN_ADDRESS: u32 = 256;
const MAX_LENGTH_QUERY_CHAIN_REQUEST: u32 = 64 * KI;
/// Length of a serialized Ed25519  signature
const MAX_LENGTH_ED25519_SIGNATURE: u32 = 64;
/// Max length of a Ed25519 message in bytes.
/// This is an arbitrary value, for performance / memory contraints. If you need to verify larger
/// messages, let us know.
const MAX_LENGTH_ED25519_MESSAGE: u32 = 128 * 1024;
/// Max number of batch Ed25519 messages / signatures / public_keys.
/// This is an arbitrary value, for performance / memory contraints. If you need to batch-verify a
/// larger number of signatures, let us know.
const MAX_COUNT_ED25519_BATCH: u32 = 256;

/// Max length for a debug message
const MAX_LENGTH_DEBUG: u32 = 2 * MI;

// This is the API exposed to contracts.
//
// # Note
//
// Any input that leads to a out of bound error (reading or writing) or failing to decode
// data passed to the supervisor will lead to a trap. This is not documented explicitly
// for every function.
define_env!(Env, <E: Ext>,
	// Account for used gas. Traps if gas used is greater than gas limit.
	//
	// NOTE: This is a implementation defined call and is NOT a part of the public API.
	// This call is supposed to be called only by instrumentation injected code.
	//
	// - amount: How much gas is used.
	[seal0] gas(ctx, amount: u32) => {
		ctx.charge_gas(RuntimeCosts::MeteringBlock(amount))?;
		Ok(())
	},

	// Computes the SHA2 256-bit hash on the given input buffer.
	//
	// Returns the result directly into the given output buffer.
	//
	// # Note
	//
	// - The `input` and `output` buffer may overlap.
	// - The output buffer is expected to hold at least 32 bytes (256 bits).
	// - It is the callers responsibility to provide an output buffer that
	//   is large enough to hold the expected amount of bytes returned by the
	//   chosen hash function.
	//
	// # Parameters
	//
	// - `input_ptr`: the pointer into the linear memory where the input
	//                data is placed.
	// - `input_len`: the length of the input data in bytes.
	// - `output_ptr`: the pointer into the linear memory where the output
	//                 data is placed. The function will write the result
	//                 directly into this buffer.
	[seal0] seal_hash_sha2_256(ctx, input_ptr: u32, input_len: u32, output_ptr: u32) => {
		ctx.charge_gas(RuntimeCosts::HashSha256(input_len))?;
		Ok(ctx.compute_hash_on_intermediate_buffer(sha2_256, input_ptr, input_len, output_ptr)?)
	},

	// Computes the KECCAK 256-bit hash on the given input buffer.
	//
	// Returns the result directly into the given output buffer.
	//
	// # Note
	//
	// - The `input` and `output` buffer may overlap.
	// - The output buffer is expected to hold at least 32 bytes (256 bits).
	// - It is the callers responsibility to provide an output buffer that
	//   is large enough to hold the expected amount of bytes returned by the
	//   chosen hash function.
	//
	// # Parameters
	//
	// - `input_ptr`: the pointer into the linear memory where the input
	//                data is placed.
	// - `input_len`: the length of the input data in bytes.
	// - `output_ptr`: the pointer into the linear memory where the output
	//                 data is placed. The function will write the result
	//                 directly into this buffer.
	[seal0] seal_hash_keccak_256(ctx, input_ptr: u32, input_len: u32, output_ptr: u32) => {
		ctx.charge_gas(RuntimeCosts::HashKeccak256(input_len))?;
		Ok(ctx.compute_hash_on_intermediate_buffer(keccak_256, input_ptr, input_len, output_ptr)?)
	},

	// Computes the BLAKE2 256-bit hash on the given input buffer.
	//
	// Returns the result directly into the given output buffer.
	//
	// # Note
	//
	// - The `input` and `output` buffer may overlap.
	// - The output buffer is expected to hold at least 32 bytes (256 bits).
	// - It is the callers responsibility to provide an output buffer that
	//   is large enough to hold the expected amount of bytes returned by the
	//   chosen hash function.
	//
	// # Parameters
	//
	// - `input_ptr`: the pointer into the linear memory where the input
	//                data is placed.
	// - `input_len`: the length of the input data in bytes.
	// - `output_ptr`: the pointer into the linear memory where the output
	//                 data is placed. The function will write the result
	//                 directly into this buffer.
	[seal0] seal_hash_blake2_256(ctx, input_ptr: u32, input_len: u32, output_ptr: u32) => {
		ctx.charge_gas(RuntimeCosts::HashBlake256(input_len))?;
		Ok(ctx.compute_hash_on_intermediate_buffer(blake2_256, input_ptr, input_len, output_ptr)?)
	},

	// Computes the BLAKE2 128-bit hash on the given input buffer.
	//
	// Returns the result directly into the given output buffer.
	//
	// # Note
	//
	// - The `input` and `output` buffer may overlap.
	// - The output buffer is expected to hold at least 16 bytes (128 bits).
	// - It is the callers responsibility to provide an output buffer that
	//   is large enough to hold the expected amount of bytes returned by the
	//   chosen hash function.
	//
	// # Parameters
	//
	// - `input_ptr`: the pointer into the linear memory where the input
	//                data is placed.
	// - `input_len`: the length of the input data in bytes.
	// - `output_ptr`: the pointer into the linear memory where the output
	//                 data is placed. The function will write the result
	//                 directly into this buffer.
	[seal0] seal_hash_blake2_128(ctx, input_ptr: u32, input_len: u32, output_ptr: u32) => {
		ctx.charge_gas(RuntimeCosts::HashBlake128(input_len))?;
		Ok(ctx.compute_hash_on_intermediate_buffer(blake2_128, input_ptr, input_len, output_ptr)?)
	},

	// ============ COSMWASM ============
	[env] db_read(ctx, key_ptr: u32) -> u32 => {
		log::debug!(target: "runtime::contracts", "DbRead");
		let charged = ctx.charge_gas(RuntimeCosts::GetStorage(ctx.ext.max_value_size()))?;
		let key = keccak_256(&ctx.memory().read_region(key_ptr, MAX_LENGTH_DB_KEY as _)?);
		if let Some(value) = ctx.ext.get_storage(&key) {
			ctx.adjust_gas(charged, RuntimeCosts::GetStorage(value.len() as u32));
		  let value_ptr = ctx.allocate(value.len())?;
		  ctx.memory().write_region(value_ptr, &value)?;
			Ok(value_ptr)
		} else {
			ctx.adjust_gas(charged, RuntimeCosts::GetStorage(0));
			Ok(0)
		}
	},

	[env] db_write(ctx, key_ptr: u32, value_ptr: u32) => {
		log::debug!(target: "runtime::contracts", "DbWrite");
		let key = keccak_256(&ctx.memory().read_region(key_ptr, MAX_LENGTH_DB_KEY as _)?);
		let value = ctx.memory().read_region(value_ptr, MAX_LENGTH_DB_VALUE as _)?;
	  let value_len = value.len() as u32;
		let max_size = ctx.ext.max_value_size();
		let charged = ctx
			.charge_gas(RuntimeCosts::SetStorage { new_bytes: value_len, old_bytes: max_size })?;
		if value_len > max_size {
			Err(Error::<E::T>::ValueTooLarge)?;
		}
		let write_outcome = ctx.ext.set_storage(key, Some(value), false)?;
		ctx.adjust_gas(
			charged,
			RuntimeCosts::SetStorage { new_bytes: value_len, old_bytes: write_outcome.old_len() },
		);
		Ok(())
	},

	[env] db_remove(ctx, key_ptr: u32) => {
		log::debug!(target: "runtime::contracts", "DbRemove");
		let key = keccak_256(&ctx.memory().read_region(key_ptr, MAX_LENGTH_DB_KEY as _)?);
		let max_size = ctx.ext.max_value_size();
		let charged = ctx
			.charge_gas(RuntimeCosts::SetStorage { new_bytes: 0, old_bytes: max_size })?;
		let write_outcome = ctx.ext.set_storage(key, None, false)?;
		ctx.adjust_gas(
			charged,
			RuntimeCosts::SetStorage { new_bytes: 0, old_bytes: write_outcome.old_len() },
		);
		Ok(())
	},

	[env] db_scan(ctx, _statr_ptr: u32, _end_ptr: u32, _order: i32) -> u32 => {
		log::debug!(target: "runtime::contracts", "DbScan");
		Ok(0)
	},

	[env] db_next(ctx, _iterator_id: u32) -> u32 => {
		log::debug!(target: "runtime::contracts", "DbNext");
		Ok(0)
	},

	[env] addr_validate(ctx, _source_ptr: u32) -> u32 => {
		log::debug!(target: "runtime::contracts", "AddrValidate");
		Ok(0)
	},
	[env] addr_canonicalize(ctx, _source_ptr: u32, _destination_ptr: u32) -> u32 => {
		log::debug!(target: "runtime::contracts", "AddrCanonicalize");
		Ok(0)
	},
	[env] addr_humanize(ctx, _source_ptr: u32, _destination_ptr: u32) -> u32 => {
		log::debug!(target: "runtime::contracts", "AddrHumanize");
		Ok(0)
	},

	[env] secp256k1_verify(ctx, _message_hash_ptr: u32, _signature_ptr: u32, _public_key_ptr: u32) -> u32 => {
		log::debug!(target: "runtime::contracts", "Verify1");
		Ok(0)
	},
	[env] secp256k1_recover_pubkey(ctx, _message_hash_ptr: u32, _signature_ptr: u32, _recovery_param: u32) -> u64 => {
		log::debug!(target: "runtime::contracts", "Pubkey");
		Ok(0)
	},
	[env] ed25519_verify(ctx, _message_ptr: u32, _signature_ptr: u32, _public_key_ptr: u32) -> u32 => {
		log::debug!(target: "runtime::contracts", "Verify");
		Ok(0)
	},
	[env] ed25519_batch_verify(ctx, _messages_ptr: u32, _signatures_ptr: u32, _public_keys_ptr: u32) -> u32 => {
		log::debug!(target: "runtime::contracts", "BatchVerify");
		Ok(0)
	},

	[env] debug(ctx, source_ptr: u32) => {
		log::debug!(target: "runtime::contracts", "Debug");
		ctx.charge_gas(RuntimeCosts::DebugMessage)?;
		if ctx.ext.append_debug_buffer("") {
			let data = ctx.memory().read_region(source_ptr, MAX_LENGTH_DEBUG as _)?;
			let msg = core::str::from_utf8(&data)
				.map_err(|_| <Error<E::T>>::DebugMessageInvalidUTF8)?;
		  log::debug!(target: "runtime::contracts", "Debug message: {}", msg);
			ctx.ext.append_debug_buffer(msg);
		}
		Ok(())
	},

	[env] query_chain(ctx, _request: u32) -> u32 => {
		log::debug!(target: "runtime::contracts", "QueryChain");
		Ok(0)
	},
);
