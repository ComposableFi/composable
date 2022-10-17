use super::abstraction::{CanonicalCosmwasmAccount, CosmwasmAccount, Gas};
use crate::{
	runtimes::abstraction::GasOutcome, weights::WeightInfo, Config, ContractInfoOf, Pallet,
};
use alloc::string::String;
use cosmwasm_minimal_std::{Coin, ContractInfoResponse, Empty, Env, MessageInfo};
use cosmwasm_vm::{
	executor::ExecutorError,
	has::Has,
	memory::{
		MemoryReadError, MemoryWriteError, Pointable, ReadWriteMemory, ReadableMemory,
		WritableMemory,
	},
	system::{CosmwasmCodeId, CosmwasmContractMeta, SystemError},
	transaction::Transactional,
	vm::{VMBase, VmErrorOf, VmGas},
};
use cosmwasm_vm_wasmi::{
	WasmiHostFunction, WasmiHostFunctionIndex, WasmiHostModule, WasmiInput, WasmiModule,
	WasmiModuleExecutor, WasmiModuleName, WasmiOutput, WasmiVM, WasmiVMError,
};
use frame_support::storage::ChildTriePrefixIterator;
use parity_wasm::elements::{self, External, Internal, Module, Type, ValueType};
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};
use wasmi::CanResume;
use wasmi_validation::{validate_module, PlainValidator};

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
	Rpc(String),
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
	/// Happening when a contract call the querier.
	pub fn push_readonly(&mut self) {
		self.storage_readonly_depth += 1;
	}
	/// Decrease storage readonly depth.
	/// Happening when a querier exit.
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
	/// Executing contract metadata.
	pub contract_info: ContractInfoOf<T>,
	/// State shared across all contracts within a single transaction.
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
		Pallet::<T>::do_set_contract_meta(
			&address.into_inner(),
			new_code_id,
			admin.map(|admin| admin.into_inner()),
			label,
		)
		.map_err(Into::into)
	}

	fn running_contract_meta(&mut self) -> Result<Self::ContractMeta, Self::Error> {
		log::debug!(target: "runtime::contracts", "contract_meta");
		Ok(Pallet::<T>::do_running_contract_meta(self))
	}

	fn contract_meta(&mut self, address: Self::Address) -> Result<Self::ContractMeta, Self::Error> {
		log::debug!(target: "runtime::contracts", "code_id");
		Pallet::<T>::do_contract_meta(address.into_inner())
	}

	fn debug(&mut self, message: Vec<u8>) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "[CONTRACT-LOG] {}", String::from_utf8_lossy(&message));
		Ok(())
	}

	fn addr_validate(&mut self, input: &str) -> Result<Result<(), Self::Error>, Self::Error> {
		log::debug!(target: "runtime::contracts", "addr_validate");
		match Pallet::<T>::do_addr_validate(input.into()) {
			Ok(_) => Ok(Ok(())),
			Err(e) => Ok(Err(e)),
		}
	}

	fn addr_canonicalize(
		&mut self,
		input: &str,
	) -> Result<Result<Self::CanonicalAddress, Self::Error>, Self::Error> {
		log::debug!(target: "runtime::contracts", "addr_canonicalize");
		let account = match Pallet::<T>::do_addr_canonicalize(input.into()) {
			Ok(account) => account,
			Err(e) => return Ok(Err(e)),
		};

		Ok(Ok(CosmwasmAccount::new(account).into()))
	}

	fn addr_humanize(
		&mut self,
		addr: &Self::CanonicalAddress,
	) -> Result<Result<Self::Address, Self::Error>, Self::Error> {
		log::debug!(target: "runtime::contracts", "addr_humanize");
		Ok(Ok(Pallet::<T>::do_addr_humanize(addr)))
	}

	fn secp256k1_recover_pubkey(
		&mut self,
		message_hash: &[u8],
		signature: &[u8],
		recovery_param: u8,
	) -> Result<Result<Vec<u8>, ()>, Self::Error> {
		log::debug!(target: "runtime::contracts", "secp256k1_recover_pubkey");
		Ok(Pallet::<T>::do_secp256k1_recover_pubkey(message_hash, signature, recovery_param))
	}

	fn secp256k1_verify(
		&mut self,
		message_hash: &[u8],
		signature: &[u8],
		public_key: &[u8],
	) -> Result<bool, Self::Error> {
		log::debug!(target: "runtime::contracts", "secp256k1_verify");
		Ok(Pallet::<T>::do_secp256k1_verify(message_hash, signature, public_key))
	}

	fn ed25519_batch_verify(
		&mut self,
		messages: &[&[u8]],
		signatures: &[&[u8]],
		public_keys: &[&[u8]],
	) -> Result<bool, Self::Error> {
		log::debug!(target: "runtime::contracts", "ed25519_batch_verify");
		Ok(Pallet::<T>::do_ed25519_batch_verify(messages, signatures, public_keys))
	}

	fn ed25519_verify(
		&mut self,
		message: &[u8],
		signature: &[u8],
		public_key: &[u8],
	) -> Result<bool, Self::Error> {
		log::debug!(target: "runtime::contracts", "ed25519_verify");
		Ok(Pallet::<T>::do_ed25519_verify(message, signature, public_key))
	}

	fn query_continuation(
		&mut self,
		address: Self::Address,
		message: &[u8],
	) -> Result<cosmwasm_minimal_std::QueryResult, Self::Error> {
		log::debug!(target: "runtime::contracts", "query_continuation");
		Pallet::<T>::do_query_continuation(self, address.into_inner(), message)
	}

	fn continue_execute(
		&mut self,
		address: Self::Address,
		funds: Vec<Coin>,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_minimal_std::Event),
	) -> Result<Option<cosmwasm_minimal_std::Binary>, Self::Error> {
		log::debug!(target: "runtime::contracts", "continue_execute");
		Pallet::<T>::do_continue_execute(self, address.into_inner(), funds, message, event_handler)
	}

	fn continue_instantiate(
		&mut self,
		contract_meta: Self::ContractMeta,
		funds: Vec<Coin>,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_minimal_std::Event),
	) -> Result<(Self::Address, Option<cosmwasm_minimal_std::Binary>), Self::Error> {
		log::debug!(target: "runtime::contracts", "continue_instantiate");
		Pallet::<T>::do_continue_instantiate(self, contract_meta, funds, message, event_handler)
			.map(|r| (self.contract_address.clone(), r))
	}

	fn continue_migrate(
		&mut self,
		address: Self::Address,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_minimal_std::Event),
	) -> Result<Option<cosmwasm_minimal_std::Binary>, Self::Error> {
		log::debug!(target: "runtime::contracts", "continue_migrate");
		Pallet::<T>::do_continue_migrate(self, address.into_inner(), message, event_handler)
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
		Pallet::<T>::do_query_raw(self, address.into_inner(), &key)
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
		Pallet::<T>::do_query_info(self, address.into_inner())
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
			VmGas::DbRead => T::WeightInfo::db_read(),
			VmGas::DbWrite => T::WeightInfo::db_write(),
			VmGas::DbRemove => T::WeightInfo::db_remove(),
			VmGas::DbScan => T::WeightInfo::db_scan(),
			VmGas::DbNext => T::WeightInfo::db_next(),
			VmGas::Balance => T::WeightInfo::balance(),
			VmGas::Secp256k1Verify => T::WeightInfo::secp256k1_verify(),
			VmGas::Secp256k1RecoverPubkey => T::WeightInfo::secp256k1_recover_pubkey(),
			VmGas::Ed25519Verify => T::WeightInfo::ed25519_verify(),
			VmGas::Ed25519BatchVerify => T::WeightInfo::ed25519_batch_verify(),
			VmGas::AddrValidate => T::WeightInfo::addr_validate(),
			VmGas::AddrCanonicalize => T::WeightInfo::addr_canonicalize(),
			VmGas::AddrHumanize => T::WeightInfo::addr_humanize(),
			VmGas::GetContractMeta => T::WeightInfo::contract_meta(),
			VmGas::Transfer { nb_of_coins } => T::WeightInfo::transfer(nb_of_coins),
			VmGas::ContinueExecute { nb_of_coins } => T::WeightInfo::continue_execute(nb_of_coins),
			VmGas::ContinueInstantiate { nb_of_coins } =>
				T::WeightInfo::continue_instantiate(nb_of_coins),
			VmGas::ContinueMigrate => T::WeightInfo::continue_migrate(),
			VmGas::QueryContinuation => T::WeightInfo::query_continuation(),
			VmGas::QueryRaw => T::WeightInfo::query_raw(),
			VmGas::QueryInfo => T::WeightInfo::query_info(),
			// VmGas::Debug is not charged
			_ => 1_u64,
			/*
			-----------------
			Unsupported operations
			-----------------
			VmGas::QueryCustom => todo!(),
			VmGas::MessageCustom => todo!(),
			VmGas::Burn => todo!(),
			VmGas::AllBalance => todo!(),
			*/
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
