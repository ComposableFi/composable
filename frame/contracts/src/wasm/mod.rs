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

//! This module provides a means for executing contracts
//! represented in wasm.

#[macro_use]
mod env_def;
mod code_cache;
pub mod cosmwasm;
mod prepare;
mod runtime;

#[cfg(feature = "runtime-benchmarks")]
pub use crate::wasm::code_cache::reinstrument;
pub use crate::wasm::runtime::{CallFlags, ReturnCode, Runtime, RuntimeCosts};
use crate::{
	exec::{Executable, ExecuteFunction, Ext},
	gas::GasMeter,
	wasm::{
		cosmwasm::types::{
			CosmwasmExecutionResult, CosmwasmQueryResult, CosmwasmReplyResult, Env, ExecuteResult,
			InstantiateResult, MessageInfo, QueryResult, ReplyResult,
		},
		env_def::FunctionImplProvider,
	},
	AccountIdOf, BalanceOf, CodeHash, CodeHashToId, CodeStorage, Config, Error, Schedule,
};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::dispatch::{DispatchError, DispatchResult};
use sp_core::crypto::UncheckedFrom;
use sp_sandbox::SandboxEnvironmentBuilder;
use sp_std::prelude::*;

/// A prepared wasm module ready for execution.
///
/// # Note
///
/// This data structure is mostly immutable once created and stored. The exceptions that
/// can be changed by calling a contract are `instruction_weights_version` and `code`.
/// `instruction_weights_version` and `code` change when a contract with an outdated instrumentation
/// is called. Therefore one must be careful when holding any in-memory representation of this
/// type while calling into a contract as those fields can get out of date.
#[derive(Clone, Encode, Decode, scale_info::TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct PrefabWasmModule<T: Config> {
	/// Version of the instruction weights with which the code was instrumented.
	#[codec(compact)]
	instruction_weights_version: u32,
	/// Initial memory size of a contract's sandbox.
	#[codec(compact)]
	initial: u32,
	/// The maximum memory size of a contract's sandbox.
	#[codec(compact)]
	maximum: u32,
	/// Code instrumented with the latest schedule.
	code: Vec<u8>,
	/// The uninstrumented, pristine version of the code.
	///
	/// It is not stored because the pristine code has its own storage item. The value
	/// is only `Some` when this module was created from an `original_code` and `None` if
	/// it was loaded from storage.
	#[codec(skip)]
	original_code: Option<Vec<u8>>,
	/// The code hash of the stored code which is defined as the hash over the `original_code`.
	///
	/// As the map key there is no need to store the hash in the value, too. It is set manually
	/// when loading the module from storage.
	#[codec(skip)]
	code_hash: CodeHash<T>,
	// This isn't needed for contract execution and does not get loaded from storage by default.
	// It is `Some` if and only if this struct was generated from code.
	#[codec(skip)]
	owner_info: Option<OwnerInfo<T>>,
}

/// Information that belongs to a [`PrefabWasmModule`] but is stored separately.
///
/// It is stored in a separate storage entry to avoid loading the code when not necessary.
#[derive(Clone, Encode, Decode, scale_info::TypeInfo, MaxEncodedLen)]
#[codec(mel_bound())]
#[scale_info(skip_type_params(T))]
pub struct OwnerInfo<T: Config> {
	/// The account that has deployed the contract and hence is allowed to remove it.
	owner: AccountIdOf<T>,
	/// The amount of balance that was deposited by the owner in order to deploy it.
	#[codec(compact)]
	deposit: BalanceOf<T>,
	/// The number of contracts that use this as their code.
	#[codec(compact)]
	refcount: u64,
}

impl<T: Config> PrefabWasmModule<T>
where
	T::AccountId: UncheckedFrom<T::Hash> + AsRef<[u8]>,
{
	/// Create the module by checking and instrumenting `original_code`.
	///
	/// This does **not** store the module. For this one need to either call [`Self::store`]
	/// or [`<Self as Executable>::execute`].
	pub fn from_code(
		original_code: Vec<u8>,
		schedule: &Schedule<T>,
		owner: AccountIdOf<T>,
	) -> Result<Self, &'static str> {
		prepare::prepare_contract(original_code, schedule, owner)
	}

	/// Store the code without instantiating it.
	///
	/// Otherwise the code is stored when [`<Self as Executable>::execute`] is called.
	pub fn store(self, id: u64) -> DispatchResult {
		code_cache::store(self, id, false)
	}

	/// Remove the code from storage and refund the deposit to its owner.
	///
	/// Applies all necessary checks before removing the code.
	pub fn remove(origin: &T::AccountId, code_hash: CodeHash<T>) -> DispatchResult {
		code_cache::try_remove::<T>(origin, code_hash)
	}

	/// Returns whether there is a deposit to be payed for this module.
	///
	/// Returns `0` if the module is already in storage and hence no deposit will
	/// be charged when storing it.
	pub fn open_deposit(&self) -> BalanceOf<T> {
		if <CodeStorage<T>>::contains_key(&self.code_hash) {
			0u32.into()
		} else {
			// Only already in-storage contracts have their `owner_info` set to `None`.
			// Therefore it is correct to return `0` in this case.
			self.owner_info.as_ref().map(|i| i.deposit).unwrap_or_default()
		}
	}

	/// Create and store the module without checking nor instrumenting the passed code.
	///
	/// # Note
	///
	/// This is useful for benchmarking where we don't want instrumentation to skew
	/// our results. This also does not collect any deposit from the `owner`.
	#[cfg(feature = "runtime-benchmarks")]
	pub fn store_code_unchecked(
		id: u64,
		original_code: Vec<u8>,
		schedule: &Schedule<T>,
		owner: T::AccountId,
	) -> DispatchResult {
		let executable = prepare::benchmarking::prepare_contract(original_code, schedule, owner)
			.map_err::<DispatchError, _>(Into::into)?;
		code_cache::store(executable, id, false)
	}
}

impl<T: Config> Executable<T> for PrefabWasmModule<T>
where
	T::AccountId: UncheckedFrom<T::Hash> + AsRef<[u8]>,
{
	fn from_storage(
		code_hash: CodeHash<T>,
		schedule: &Schedule<T>,
		gas_meter: &mut GasMeter<T>,
	) -> Result<Self, DispatchError> {
		code_cache::load(code_hash, schedule, gas_meter)
	}

	fn from_storage_with_id(
		code_id: u64,
		schedule: &Schedule<T>,
		gas_meter: &mut GasMeter<T>,
	) -> Result<Self, DispatchError> {
		code_cache::load_with_id(code_id, schedule, gas_meter)
	}

	fn add_user(code_hash: CodeHash<T>) -> Result<(), DispatchError> {
		code_cache::increment_refcount::<T>(code_hash)
	}

	fn remove_user(code_hash: CodeHash<T>) {
		code_cache::decrement_refcount::<T>(code_hash)
	}

	fn execute<E: Ext<T = T>>(
		self,
		ext: &mut E,
		function: &ExecuteFunction,
		env: Env,
		info: MessageInfo,
		input_data: Vec<u8>,
	) -> Result<CosmwasmExecutionResult, DispatchError> {
		// We store before executing so that the code hash is available in the constructor.
		let code = self.code.clone();
		let hash = self.code_hash.clone();
		if let &ExecuteFunction::Instantiate = function {
			code_cache::store(
				self,
				CodeHashToId::<T>::get(hash).ok_or(Error::<T>::ContractNotFound)?,
				true,
			)?;
		}
		let mut imports = sp_sandbox::default_executor::EnvironmentDefinitionBuilder::new();
		runtime::Env::impls(&mut |module, name, func_ptr| {
			imports.add_host_func(module, name, func_ptr);
		});

		let mut runtime = Runtime::new(ext, &code, &imports)?;

		log::debug!(target: "runtime::contracts", "Executing function {:?}", function);

		match function {
			ExecuteFunction::Instantiate => {
				let InstantiateResult(response) = runtime.do_instantiate(env, info, &input_data)?;
				Ok(response)
			},
			ExecuteFunction::Call => {
				let ExecuteResult(response) = runtime.do_execute(env, info, &input_data)?;
				Ok(response)
			},
		}
	}

	fn query<E: Ext<T = T>>(
		self,
		ext: &mut E,
		env: Env,
		input_data: Vec<u8>,
	) -> Result<CosmwasmQueryResult, DispatchError> {
		// We store before executing so that the code hash is available in the constructor.
		let code = self.code.clone();
		let mut imports = sp_sandbox::default_executor::EnvironmentDefinitionBuilder::new();
		runtime::Env::impls(&mut |module, name, func_ptr| {
			imports.add_host_func(module, name, func_ptr);
		});

		let mut runtime = Runtime::new(ext, &code, &imports)?;

		let QueryResult(response) = runtime.do_query(env, &input_data)?;

		Ok(response)
	}

	fn reply<E: Ext<T = T>>(
		self,
		ext: &mut E,
		env: Env,
		input_data: Vec<u8>,
	) -> Result<CosmwasmReplyResult, DispatchError> {
		// We store before executing so that the code hash is available in the constructor.
		let code = self.code;
		let mut imports = sp_sandbox::default_executor::EnvironmentDefinitionBuilder::new();
		runtime::Env::impls(&mut |module, name, func_ptr| {
			imports.add_host_func(module, name, func_ptr);
		});

		let mut runtime = Runtime::new(ext, &code, &imports)?;

		log::debug!(target: "runtime::contracts", "Executing reply");

		let ReplyResult(response) = runtime.do_reply(env, &input_data)?;

		Ok(response)
	}

	fn code_hash(&self) -> &CodeHash<T> {
		&self.code_hash
	}

	fn code_len(&self) -> u32 {
		self.code.len() as u32
	}
}
