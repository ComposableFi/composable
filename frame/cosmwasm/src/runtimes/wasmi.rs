use super::abstraction::{CanonicalCosmwasmAccount, CosmwasmAccount, Gas};
use crate::{runtimes::abstraction::GasOutcome, Config, ContractInfoOf, Pallet};
use alloc::string::String;
use cosmwasm_minimal_std::{Coin, ContractInfoResponse, Empty, Env, MessageInfo};
use cosmwasm_vm::{
	executor::{cosmwasm_call, ExecutorError, InstantiateInput, MigrateInput, QueryInput},
	has::Has,
	memory::{
		MemoryReadError, MemoryWriteError, Pointable, ReadWriteMemory, ReadableMemory,
		WritableMemory,
	},
	system::{cosmwasm_system_run, CosmwasmCodeId, CosmwasmContractMeta, SystemError},
	transaction::Transactional,
	vm::{VMBase, VmErrorOf, VmGas},
};
use cosmwasm_vm_wasmi::{
	WasmiHostFunction, WasmiHostFunctionIndex, WasmiHostModule, WasmiInput, WasmiModule,
	WasmiModuleExecutor, WasmiModuleName, WasmiOutput, WasmiVM, WasmiVMError,
};
use frame_support::storage::ChildTriePrefixIterator;
use parity_wasm::elements::{self, External, Internal, Module, Type, ValueType};
use sp_core::{ecdsa, ed25519};
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};
use wasmi::CanResume;
use wasmi_validation::{validate_module, PlainValidator};

const SUBSTRATE_ECDSA_SIGNATURE_LEN: usize = 65;

#[derive(Debug)]
pub enum CosmwasmVMError<T: Config> {
	Interpreter(wasmi::Error),
	VirtualMachine(WasmiVMError),
	Pallet(crate::Error<T>),
	AccountConversionFailure,
	Aborted(String),
	ReadOnlyViolation,
	OutOfGas,
	Unsupported,
}

impl<T: Config> core::fmt::Display for CosmwasmVMError<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl<T: Config> From<crate::Error<T>> for CosmwasmVMError<T> {
	fn from(e: crate::Error<T>) -> Self {
		Self::Pallet(e)
	}
}

impl<T: Config> From<wasmi::Error> for CosmwasmVMError<T> {
	fn from(e: wasmi::Error) -> Self {
		Self::Interpreter(e)
	}
}

impl<T: Config> From<WasmiVMError> for CosmwasmVMError<T> {
	fn from(e: WasmiVMError) -> Self {
		Self::VirtualMachine(e)
	}
}

impl<T: Config> From<SystemError> for CosmwasmVMError<T> {
	fn from(e: SystemError) -> Self {
		Self::VirtualMachine(e.into())
	}
}

impl<T: Config> From<ExecutorError> for CosmwasmVMError<T> {
	fn from(e: ExecutorError) -> Self {
		Self::VirtualMachine(e.into())
	}
}

impl<T: Config> From<MemoryReadError> for CosmwasmVMError<T> {
	fn from(e: MemoryReadError) -> Self {
		Self::VirtualMachine(e.into())
	}
}

impl<T: Config> From<MemoryWriteError> for CosmwasmVMError<T> {
	fn from(e: MemoryWriteError) -> Self {
		Self::VirtualMachine(e.into())
	}
}

impl<T: Config> CanResume for CosmwasmVMError<T> {
	fn can_resume(&self) -> bool {
		false
	}
}

/// Initial storage mutability.
pub enum InitialStorageMutability {
	/// The storage is currently readonly, any operation trying to mutate it will result in a
	/// [`CosmwasmVMError::ReadOnlyViolation`]
	ReadOnly,
	/// Mutable operations on the storage are currently allowed.
	ReadWrite,
}

/// VM shared cache
pub struct CosmwasmVMCache {
	/// Code cache, a mapping from a CosmWasm contract code id to the baking code.
	pub code: BTreeMap<CosmwasmCodeId, Vec<u8>>,
}

/// VM shared state
pub struct CosmwasmVMShared {
	/// Readonly depth, used to determine whether the storage is currently readonly or not.
	/// Whenever a contract call `query`, we increment this counter before the call and decrement
	/// after the call. A value of 0 mean that the contract is able to mutate the storage.
	/// A value > 0 mean that the storage is readonly.
	pub storage_readonly_depth: u32,
	/// VM depth, i.e. how many contracts has been loaded and are currently running.
	pub depth: u32,
	/// Shared Gas metering.
	pub gas: Gas,
	/// Shared cache.
	pub cache: CosmwasmVMCache,
}

impl CosmwasmVMShared {
	/// Whether the storage is currently readonly.
	pub fn storage_is_readonly(&self) -> bool {
		self.storage_readonly_depth > 0
	}
	/// Increase storage readonly depth.
	/// Hapenning when a contract call the querier.
	pub fn push_readonly(&mut self) {
		self.storage_readonly_depth += 1;
	}
	/// Decrease storage readonly depth.
	/// Hapenning when a querier exit.
	pub fn pop_readonly(&mut self) {
		self.storage_readonly_depth -= 1;
	}
}

/// Cosmwasm VM instance data.
/// This structure hold the state for a single contract.
/// Note that all [`CosmwasmVM`] share the same [`CosmWasmVMShared`] structure.
pub struct CosmwasmVM<'a, T: Config> {
	/// NOTE(hussein-aitlahcen): would be nice to move the host functions to the shared structure.
	/// but we can't do it, otherwise we introduce a dependency do the lifetime `'a` here. This
	/// lifetime is used by the host function and when reusing the shared structure for sub-calls,
	/// the lifetime would be different (lifetime of children live longer than the initial one,
	/// hence we'll face a compilation issue). This could be solved with HKT or unsafe host
	/// functions (raw pointer without lifetime). Host functions by index.
	pub host_functions_by_index:
		BTreeMap<WasmiHostFunctionIndex, WasmiHostFunction<CosmwasmVM<'a, T>>>,
	/// Host functions by (module, name).
	pub host_functions_by_name: BTreeMap<WasmiModuleName, WasmiHostModule<CosmwasmVM<'a, T>>>,

	/// Currently executing Wasmi module. A.k.a. contract instance from Wasmi perspective.
	pub executing_module: WasmiModule,
	/// CosmWasm [`Env`] for the executing contract.
	pub cosmwasm_env: Env,
	/// CosmWasm [`MessageInfo`] for the executing contract.
	pub cosmwasm_message_info: MessageInfo,
	/// Executing contract address.
	pub contract_address: CosmwasmAccount<T>,
	/// Executing contract metadatas.
	pub contract_info: ContractInfoOf<T>,
	/// State shared accross all contracts within a single transaction.
	pub shared: &'a mut CosmwasmVMShared,
	/// Iterator id's to corresponding keys. Keys are used to get the next key.
	pub iterators: BTreeMap<u32, ChildTriePrefixIterator<(Vec<u8>, Vec<u8>)>>,
}

impl<'a, T: Config> Has<Env> for CosmwasmVM<'a, T> {
	fn get(&self) -> Env {
		self.cosmwasm_env.clone()
	}
}

impl<'a, T: Config> Has<MessageInfo> for CosmwasmVM<'a, T> {
	fn get(&self) -> MessageInfo {
		self.cosmwasm_message_info.clone()
	}
}

impl<'a, T: Config> WasmiModuleExecutor for CosmwasmVM<'a, T> {
	fn executing_module(&self) -> WasmiModule {
		self.executing_module.clone()
	}
	fn host_function(
		&self,
		index: WasmiHostFunctionIndex,
	) -> Option<&WasmiHostFunction<CosmwasmVM<'a, T>>> {
		self.host_functions_by_index.get(&index)
	}
}

impl<'a, T: Config> Pointable for CosmwasmVM<'a, T> {
	type Pointer = u32;
}

impl<'a, T: Config> ReadableMemory for CosmwasmVM<'a, T> {
	type Error = VmErrorOf<Self>;
	fn read(&self, offset: Self::Pointer, buffer: &mut [u8]) -> Result<(), Self::Error> {
		self.executing_module
			.memory
			.get_into(offset, buffer)
			.map_err(|_| WasmiVMError::LowLevelMemoryReadError.into())
	}
}

impl<'a, T: Config> WritableMemory for CosmwasmVM<'a, T> {
	type Error = VmErrorOf<Self>;
	fn write(&self, offset: Self::Pointer, buffer: &[u8]) -> Result<(), Self::Error> {
		self.executing_module
			.memory
			.set(offset, buffer)
			.map_err(|_| WasmiVMError::LowLevelMemoryWriteError.into())
	}
}

impl<'a, T: Config> ReadWriteMemory for CosmwasmVM<'a, T> {}

impl<'a, T: Config> CosmwasmVM<'a, T> {
	pub fn charge_raw(&mut self, gas: u64) -> Result<(), <Self as VMBase>::Error> {
		match self.shared.gas.charge(gas) {
			GasOutcome::Halt => Err(CosmwasmVMError::OutOfGas),
			GasOutcome::Continue => Ok(()),
		}
	}
}

impl<'a, T: Config> VMBase for CosmwasmVM<'a, T> {
	type Input<'x> = WasmiInput<'x, WasmiVM<Self>>;
	type Output<'x> = WasmiOutput<'x, WasmiVM<Self>>;
	type QueryCustom = Empty;
	type MessageCustom = Empty;
	type ContractMeta = CosmwasmContractMeta<CosmwasmAccount<T>>;
	type CanonicalAddress = CanonicalCosmwasmAccount<T>;
	type Address = CosmwasmAccount<T>;
	type StorageKey = Vec<u8>;
	type StorageValue = Vec<u8>;
	type Error = CosmwasmVMError<T>;

	fn set_contract_meta(
		&mut self,
		address: Self::Address,
		CosmwasmContractMeta { code_id: new_code_id, admin, label }: Self::ContractMeta,
	) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "set_contract_meta");
		let contract = address.into_inner();
		let mut info = Pallet::<T>::contract_info(&contract)?;
		info.code_id = new_code_id;
		info.admin = admin.map(|admin| admin.into_inner());
		info.label = label
			.as_bytes()
			.to_vec()
			.try_into()
			.map_err(|_| crate::Error::<T>::LabelTooBig)?;
		Pallet::<T>::set_contract_info(&contract, info);
		Ok(())
	}

	fn running_contract_meta(&mut self) -> Result<Self::ContractMeta, Self::Error> {
		log::debug!(target: "runtime::contracts", "contract_meta");
		Ok(CosmwasmContractMeta {
			code_id: self.contract_info.code_id,
			admin: self.contract_info.admin.clone().map(CosmwasmAccount::new),
			label: String::from_utf8_lossy(&self.contract_info.label).into(),
		})
	}

	fn contract_meta(&mut self, address: Self::Address) -> Result<Self::ContractMeta, Self::Error> {
		log::debug!(target: "runtime::contracts", "code_id");
		let info = Pallet::<T>::contract_info(address.as_ref())?;
		Ok(CosmwasmContractMeta {
			code_id: info.code_id,
			admin: info.admin.map(CosmwasmAccount::new),
			label: String::from_utf8_lossy(&info.label.into_inner()).into(),
		})
	}

	fn debug(&mut self, message: Vec<u8>) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "[CONTRACT-LOG] {}", String::from_utf8_lossy(&message));
		Ok(())
	}

	fn addr_validate(&mut self, input: &str) -> Result<Result<(), Self::Error>, Self::Error> {
		match Pallet::<T>::cosmwasm_addr_to_account(input.into()) {
			Ok(_) => Ok(Ok(())),
			Err(e) => Ok(Err(e)),
		}
	}

	fn addr_canonicalize(
		&mut self,
		input: &str,
	) -> Result<Result<Self::CanonicalAddress, Self::Error>, Self::Error> {
		let account = match Pallet::<T>::cosmwasm_addr_to_account(input.into()) {
			Ok(account) => account,
			Err(e) => return Ok(Err(e)),
		};

		Ok(Ok(CosmwasmAccount::new(account).into()))
	}

	fn addr_humanize(
		&mut self,
		addr: &Self::CanonicalAddress,
	) -> Result<Result<Self::Address, Self::Error>, Self::Error> {
		Ok(Ok(addr.0.clone()))
	}

	fn secp256k1_recover_pubkey(
		&mut self,
		message_hash: &[u8],
		signature: &[u8],
		recovery_param: u8,
	) -> Result<Result<Vec<u8>, ()>, Self::Error> {
		// `recovery_param` must be 0 or 1. Other values are not supported from CosmWasm.
		if recovery_param > 2 {
			return Ok(Err(()))
		}

		if signature.len() != SUBSTRATE_ECDSA_SIGNATURE_LEN - 1 {
			return Ok(Err(()))
		}

		// Try into a [u8; 32]
		let message_hash = match message_hash.try_into() {
			Ok(message_hash) => message_hash,
			Err(_) => return Ok(Err(())),
		};

		let signature = {
			// Since we fill `signature_inner` with `recovery_param`, when 64 bytes are written
			// the final byte will be the `recovery_param`.
			let mut signature_inner = [recovery_param; SUBSTRATE_ECDSA_SIGNATURE_LEN];
			signature_inner[..SUBSTRATE_ECDSA_SIGNATURE_LEN - 1].copy_from_slice(signature);
			signature_inner
		};

		// We used `compressed` function here because the api states that this function
		// needs to return a public key that can be used in `secp256k1_verify` which
		// takes a compressed public key.
		Ok(sp_io::crypto::secp256k1_ecdsa_recover_compressed(&signature, &message_hash)
			.map(|val| val.into())
			.map_err(|_| ()))
	}

	fn secp256k1_verify(
		&mut self,
		message_hash: &[u8],
		signature: &[u8],
		public_key: &[u8],
	) -> Result<bool, Self::Error> {
		if signature.len() != SUBSTRATE_ECDSA_SIGNATURE_LEN {
			return Ok(false)
		}

		// Try into a [u8; 32]
		let message_hash = match message_hash.try_into() {
			Ok(message_hash) => message_hash,
			Err(_) => return Ok(false),
		};

		// We are expecting 64 bytes long public keys but the substrate function use an
		// additional byte for recovery id. So we insert a dummy byte.
		let signature = {
			let mut signature_inner = [0_u8; SUBSTRATE_ECDSA_SIGNATURE_LEN];
			signature_inner[..SUBSTRATE_ECDSA_SIGNATURE_LEN - 1].copy_from_slice(signature);
			ecdsa::Signature(signature_inner)
		};

		let public_key = match ecdsa::Public::try_from(public_key) {
			Ok(public_key) => public_key,
			Err(_) => return Ok(false),
		};

		Ok(sp_io::crypto::ecdsa_verify_prehashed(&signature, &message_hash, &public_key))
	}

	fn ed25519_batch_verify(
		&mut self,
		messages: &[&[u8]],
		signatures: &[&[u8]],
		public_keys: &[&[u8]],
	) -> Result<bool, Self::Error> {
		if messages.len() != signatures.len() || signatures.len() != public_keys.len() {
			return Ok(false)
		}

		for ((message, signature), public_key) in
			messages.iter().zip(signatures.iter()).zip(public_keys.iter())
		{
			if !(self.ed25519_verify(message, signature, public_key)?) {
				return Ok(false)
			}
		}

		Ok(true)
	}

	fn ed25519_verify(
		&mut self,
		message: &[u8],
		signature: &[u8],
		public_key: &[u8],
	) -> Result<bool, Self::Error> {
		let signature: ed25519::Signature = match signature.try_into() {
			Ok(signature) => signature,
			Err(_) => return Ok(false),
		};

		let public_key: ed25519::Public = match public_key.try_into() {
			Ok(public_key) => public_key,
			Err(_) => return Ok(false),
		};

		Ok(sp_io::crypto::ed25519_verify(&signature, message, &public_key))
	}

	fn query_continuation(
		&mut self,
		address: Self::Address,
		message: &[u8],
	) -> Result<cosmwasm_minimal_std::QueryResult, Self::Error> {
		log::debug!(target: "runtime::contracts", "query_continuation");
		let sender = self.contract_address.clone().into_inner();
		let contract = address.into_inner();
		let info = Pallet::<T>::contract_info(&contract)?;
		self.shared.push_readonly();
		let result = Pallet::<T>::cosmwasm_call(
			self.shared,
			sender,
			contract,
			info,
			Default::default(),
			|vm| cosmwasm_call::<QueryInput, WasmiVM<CosmwasmVM<T>>>(vm, message),
		);
		self.shared.pop_readonly();
		result
	}

	fn continue_execute(
		&mut self,
		address: Self::Address,
		funds: Vec<Coin>,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_minimal_std::Event),
	) -> Result<Option<cosmwasm_minimal_std::Binary>, Self::Error> {
		log::debug!(target: "runtime::contracts", "continue_execute");
		let sender = self.contract_address.clone().into_inner();
		let contract = address.into_inner();
		let info = Pallet::<T>::contract_info(&contract)?;
		Pallet::<T>::cosmwasm_call(self.shared, sender, contract, info, funds, |vm| {
			cosmwasm_system_run::<InstantiateInput, _>(vm, message, event_handler)
		})
	}

	fn continue_instantiate(
		&mut self,
		CosmwasmContractMeta { code_id, admin, label }: Self::ContractMeta,
		funds: Vec<Coin>,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_minimal_std::Event),
	) -> Result<(Self::Address, Option<cosmwasm_minimal_std::Binary>), Self::Error> {
		log::debug!(target: "runtime::contracts", "continue_instantiate");
		let nonce = Pallet::<T>::next_contract_nonce(self.contract_address.as_ref())?;
		let (contract, info) = Pallet::<T>::do_instantiate_phase1(
			self.contract_address.clone().into_inner(),
			code_id,
			&nonce.to_le_bytes(),
			admin.map(|admin| admin.into_inner()),
			label
				.as_bytes()
				.to_vec()
				.try_into()
				.map_err(|_| crate::Error::<T>::LabelTooBig)?,
		)?;
		Pallet::<T>::cosmwasm_call(
			self.shared,
			self.contract_address.clone().into_inner(),
			contract,
			info,
			funds,
			|vm| cosmwasm_system_run::<InstantiateInput, _>(vm, message, event_handler),
		)
		.map(|r| (self.contract_address.clone(), r))
	}

	fn continue_migrate(
		&mut self,
		address: Self::Address,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_minimal_std::Event),
	) -> Result<Option<cosmwasm_minimal_std::Binary>, Self::Error> {
		log::debug!(target: "runtime::contracts", "continue_migrate");
		let sender = self.contract_address.clone().into_inner();
		let contract = address.into_inner();
		let info = Pallet::<T>::contract_info(&contract)?;
		Pallet::<T>::cosmwasm_call(
			self.shared,
			sender,
			contract,
			info,
			// Can't move funds in migration.
			Default::default(),
			|vm| cosmwasm_system_run::<MigrateInput, _>(vm, message, event_handler),
		)
	}

	fn query_custom(
		&mut self,
		_: Self::QueryCustom,
	) -> Result<
		cosmwasm_minimal_std::SystemResult<cosmwasm_minimal_std::CosmwasmQueryResult>,
		Self::Error,
	> {
		log::debug!(target: "runtime::contracts", "query_custom");
		Err(CosmwasmVMError::Unsupported)
	}

	fn message_custom(
		&mut self,
		_: Self::MessageCustom,
		_: &mut dyn FnMut(cosmwasm_minimal_std::Event),
	) -> Result<Option<cosmwasm_minimal_std::Binary>, Self::Error> {
		log::debug!(target: "runtime::contracts", "message_custom");
		Err(CosmwasmVMError::Unsupported)
	}

	fn query_raw(
		&mut self,
		address: Self::Address,
		key: Self::StorageKey,
	) -> Result<Option<Self::StorageValue>, Self::Error> {
		log::debug!(target: "runtime::contracts", "query_raw");
		let info = Pallet::<T>::contract_info(address.as_ref())?;
		Pallet::<T>::do_db_read_other_contract(self, &info.trie_id, &key)
	}

	fn transfer(&mut self, to: &Self::Address, funds: &[Coin]) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "transfer: {:#?}", funds);
		let from = self.contract_address.as_ref();
		Pallet::<T>::do_transfer(from, to.as_ref(), funds, false)?;
		Ok(())
	}

	fn burn(&mut self, funds: &[Coin]) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "burn: {:#?}", funds);
		// TODO: assets registry check etc...
		Err(CosmwasmVMError::Unsupported)
	}

	fn balance(&mut self, account: &Self::Address, denom: String) -> Result<Coin, Self::Error> {
		log::debug!(target: "runtime::contracts", "balance: {} => {:#?}", Into::<String>::into(account.clone()), denom);
		let amount = Pallet::<T>::do_balance(account.as_ref(), denom.clone())?;
		Ok(Coin { denom, amount })
	}

	fn all_balance(&mut self, account: &Self::Address) -> Result<Vec<Coin>, Self::Error> {
		log::debug!(target: "runtime::contracts", "all balance: {}", Into::<String>::into(account.clone()));
		//  TODO(hussein-aitlahcen): support iterating over all tokens???
		Err(CosmwasmVMError::Unsupported)
	}

	fn query_info(&mut self, address: Self::Address) -> Result<ContractInfoResponse, Self::Error> {
		log::debug!(target: "runtime::contracts", "query_info");
		// TODO: cache or at least check if its current contract and use `self.contract_info`
		let info = Pallet::<T>::contract_info(address.as_ref())?;
		let code_id = info.code_id;
		let pinned = self.shared.cache.code.contains_key(&code_id);
		Ok(ContractInfoResponse {
			code_id,
			creator: CosmwasmAccount::<T>::new(info.instantiator.clone()).into(),
			admin: info.admin.map(|admin| CosmwasmAccount::<T>::new(admin).into()),
			pinned,
			// TODO(hussein-aitlahcen): IBC
			ibc_port: None,
		})
	}

	fn db_read(
		&mut self,
		key: Self::StorageKey,
	) -> Result<Option<Self::StorageValue>, Self::Error> {
		log::debug!(target: "runtime::contracts", "db_read");
		Pallet::<T>::do_db_read(self, &key)
	}

	fn db_write(
		&mut self,
		key: Self::StorageKey,
		value: Self::StorageValue,
	) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "db_write");
		if self.shared.storage_is_readonly() {
			Err(CosmwasmVMError::ReadOnlyViolation)
		} else {
			Pallet::<T>::do_db_write(self, &key, &value)?;
			Ok(())
		}
	}

	fn db_remove(&mut self, key: Self::StorageKey) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "db_remove");
		if self.shared.storage_is_readonly() {
			Err(CosmwasmVMError::ReadOnlyViolation)
		} else {
			Pallet::<T>::do_db_remove(self, &key);
			Ok(())
		}
	}

	fn db_scan(
		&mut self,
		_start: Option<Self::StorageKey>,
		_end: Option<Self::StorageKey>,
		_order: cosmwasm_minimal_std::Order,
	) -> Result<u32, Self::Error> {
		log::debug!(target: "runtime::contracts", "db_scan");
		Pallet::<T>::do_db_scan(self)
	}

	fn db_next(
		&mut self,
		iterator_id: u32,
	) -> Result<(Self::StorageKey, Self::StorageValue), Self::Error> {
		log::debug!(target: "runtime::contracts", "db_next");
		match Pallet::<T>::do_db_next(self, iterator_id)? {
			Some(kv_pair) => Ok(kv_pair),
			None => Ok((Vec::new(), Vec::new())),
		}
	}

	fn abort(&mut self, message: String) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "db_abort");
		Err(CosmwasmVMError::Aborted(message))
	}

	fn charge(&mut self, value: VmGas) -> Result<(), Self::Error> {
		let gas_to_charge = match value {
			VmGas::Instrumentation { metered } => metered as u64,
			// TODO(hussein-aitlahcen): benchmarking required to compute _base_ gas for each
			// operations.
			_ => 1_u64,
		};
		self.charge_raw(gas_to_charge)
	}

	fn gas_checkpoint_push(
		&mut self,
		checkpoint: cosmwasm_vm::vm::VmGasCheckpoint,
	) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "gas_checkpoint_push");
		match self.shared.gas.push(checkpoint) {
			GasOutcome::Continue => Ok(()),
			GasOutcome::Halt => Err(CosmwasmVMError::OutOfGas),
		}
	}

	fn gas_checkpoint_pop(&mut self) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "gas_checkpoint_pop");
		self.shared.gas.pop();
		Ok(())
	}

	fn gas_ensure_available(&mut self) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "gas_ensure_available");
		if self.shared.gas.remaining() > 0 {
			Ok(())
		} else {
			Err(CosmwasmVMError::OutOfGas)
		}
	}
}

impl<'a, T: Config> Transactional for CosmwasmVM<'a, T> {
	type Error = CosmwasmVMError<T>;
	fn transaction_begin(&mut self) -> Result<(), Self::Error> {
		sp_io::storage::start_transaction();
		Ok(())
	}
	fn transaction_commit(&mut self) -> Result<(), Self::Error> {
		sp_io::storage::commit_transaction();
		Ok(())
	}
	fn transaction_rollback(&mut self) -> Result<(), Self::Error> {
		sp_io::storage::rollback_transaction();
		Ok(())
	}
}

#[derive(Debug)]
pub enum ValidationError {
	Validation(wasmi_validation::Error),
	ExportMustBeAFunction(&'static str),
	EntryPointPointToImport(&'static str),
	ExportDoesNotExists(&'static str),
	ExportWithoutSignature(&'static str),
	ExportWithWrongSignature {
		export_name: &'static str,
		expected_signature: Vec<ValueType>,
		actual_signature: Vec<ValueType>,
	},
	MissingMandatoryExport(&'static str),
	CannotImportTable,
	CannotImportGlobal,
	CannotImportMemory,
	ImportWithoutSignature,
	ImportIsBanned(&'static str, &'static str),
	MustDeclareOneInternalMemory,
	MustDeclareOneTable,
	TableExceedLimit,
	BrTableExceedLimit,
	GlobalsExceedLimit,
	GlobalFloatingPoint,
	LocalFloatingPoint,
	ParamFloatingPoint,
	FunctionParameterExceedLimit,
}

#[derive(PartialEq, Eq)]
pub enum ExportRequirement {
	Mandatory,
	Optional,
}

pub struct CodeValidation<'a>(&'a Module);
impl<'a> CodeValidation<'a> {
	pub fn new(module: &'a Module) -> Self {
		CodeValidation(module)
	}
	pub fn validate_base(self) -> Result<Self, ValidationError> {
		validate_module::<PlainValidator>(self.0, ()).map_err(ValidationError::Validation)?;
		Ok(self)
	}
	pub fn validate_exports(
		self,
		expected_exports: &[(ExportRequirement, &'static str, &'static [ValueType])],
	) -> Result<Self, ValidationError> {
		let CodeValidation(module) = self;
		let types = module.type_section().map(|ts| ts.types()).unwrap_or(&[]);
		let export_entries = module.export_section().map(|is| is.entries()).unwrap_or(&[]);
		let func_entries = module.function_section().map(|fs| fs.entries()).unwrap_or(&[]);
		let fn_space_offset = module
			.import_section()
			.map(|is| is.entries())
			.unwrap_or(&[])
			.iter()
			.filter(|entry| matches!(*entry.external(), External::Function(_)))
			.count();
		for (requirement, name, signature) in expected_exports {
			match (requirement, export_entries.iter().find(|e| &e.field() == name)) {
				(_, Some(export)) => {
					let fn_idx = match export.internal() {
						Internal::Function(ref fn_idx) => Ok(*fn_idx),
						_ => Err(ValidationError::ExportMustBeAFunction(name)),
					}?;
					let fn_idx = match fn_idx.checked_sub(fn_space_offset as u32) {
						Some(fn_idx) => Ok(fn_idx),
						None => Err(ValidationError::EntryPointPointToImport(name)),
					}?;
					let func_ty_idx = func_entries
						.get(fn_idx as usize)
						.ok_or(ValidationError::ExportDoesNotExists(name))?
						.type_ref();
					let Type::Function(ref func_ty) = types
						.get(func_ty_idx as usize)
						.ok_or(ValidationError::ExportWithoutSignature(name))?;
					if signature != &func_ty.params() {
						return Err(ValidationError::ExportWithWrongSignature {
							export_name: name,
							expected_signature: signature.to_vec(),
							actual_signature: func_ty.params().to_vec(),
						})
					}
				},
				(ExportRequirement::Mandatory, None) =>
					return Err(ValidationError::MissingMandatoryExport(name)),
				(ExportRequirement::Optional, None) => {},
			}
		}
		Ok(self)
	}
	pub fn validate_imports(
		self,
		import_banlist: &[(&'static str, &'static str)],
	) -> Result<Self, ValidationError> {
		let CodeValidation(module) = self;
		let types = module.type_section().map(|ts| ts.types()).unwrap_or(&[]);
		let import_entries = module.import_section().map(|is| is.entries()).unwrap_or(&[]);
		for import in import_entries {
			let type_idx = match import.external() {
				External::Table(_) => Err(ValidationError::CannotImportTable),
				External::Global(_) => Err(ValidationError::CannotImportGlobal),
				External::Memory(_) => Err(ValidationError::CannotImportMemory),
				External::Function(ref type_idx) => Ok(type_idx),
			}?;
			let import_name = import.field();
			let import_module = import.module();
			let Type::Function(_) =
				types.get(*type_idx as usize).ok_or(ValidationError::ImportWithoutSignature)?;
			if let Some((m, f)) =
				import_banlist.iter().find(|(m, f)| m == &import_module && f == &import_name)
			{
				return Err(ValidationError::ImportIsBanned(m, f))
			}
		}
		Ok(self)
	}
	pub fn validate_memory_limit(self) -> Result<Self, ValidationError> {
		let CodeValidation(module) = self;
		if module.memory_section().map_or(false, |ms| ms.entries().len() != 1) {
			Err(ValidationError::MustDeclareOneInternalMemory)
		} else {
			Ok(self)
		}
	}
	pub fn validate_table_size_limit(self, limit: u32) -> Result<Self, ValidationError> {
		let CodeValidation(module) = self;
		if let Some(table_section) = module.table_section() {
			if table_section.entries().len() > 1 {
				return Err(ValidationError::MustDeclareOneTable)
			}
			if let Some(table_type) = table_section.entries().first() {
				if table_type.limits().initial() > limit {
					return Err(ValidationError::TableExceedLimit)
				}
			}
		}
		Ok(self)
	}
	pub fn validate_br_table_size_limit(self, limit: u32) -> Result<Self, ValidationError> {
		let CodeValidation(module) = self;
		if let Some(code_section) = module.code_section() {
			for instr in code_section.bodies().iter().flat_map(|body| body.code().elements()) {
				use self::elements::Instruction::BrTable;
				if let BrTable(table) = instr {
					if table.table.len() > limit as usize {
						return Err(ValidationError::BrTableExceedLimit)
					}
				}
			}
		};
		Ok(self)
	}
	pub fn validate_global_variable_limit(self, limit: u32) -> Result<Self, ValidationError> {
		let CodeValidation(module) = self;
		if let Some(global_section) = module.global_section() {
			if global_section.entries().len() > limit as usize {
				return Err(ValidationError::GlobalsExceedLimit)
			}
		}
		Ok(self)
	}
	pub fn validate_no_floating_types(self) -> Result<Self, ValidationError> {
		let CodeValidation(module) = self;
		if let Some(global_section) = module.global_section() {
			for global in global_section.entries() {
				match global.global_type().content_type() {
					ValueType::F32 | ValueType::F64 =>
						return Err(ValidationError::GlobalFloatingPoint),
					_ => {},
				}
			}
		}
		if let Some(code_section) = module.code_section() {
			for func_body in code_section.bodies() {
				for local in func_body.locals() {
					match local.value_type() {
						ValueType::F32 | ValueType::F64 =>
							return Err(ValidationError::LocalFloatingPoint),
						_ => {},
					}
				}
			}
		}
		if let Some(type_section) = module.type_section() {
			for wasm_type in type_section.types() {
				match wasm_type {
					Type::Function(func_type) => {
						let return_type = func_type.results().get(0);
						for value_type in func_type.params().iter().chain(return_type) {
							match value_type {
								ValueType::F32 | ValueType::F64 =>
									return Err(ValidationError::ParamFloatingPoint),
								_ => {},
							}
						}
					},
				}
			}
		}
		Ok(self)
	}
	pub fn validate_parameter_limit(self, limit: u32) -> Result<Self, ValidationError> {
		let CodeValidation(module) = self;
		if let Some(type_section) = module.type_section() {
			for Type::Function(func) in type_section.types() {
				if func.params().len() > limit as usize {
					return Err(ValidationError::FunctionParameterExceedLimit)
				}
			}
		}
		Ok(self)
	}
}
