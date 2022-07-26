use super::abstraction::{CosmwasmAccount, Gas};
use crate::{runtimes::abstraction::GasOutcome, Config, ContractInfoOf, Pallet};
use alloc::string::String;
use core::marker::PhantomData;
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
	WasmiHostFunction, WasmiHostFunctionIndex, WasmiInput, WasmiModule, WasmiModuleExecutor,
	WasmiOutput, WasmiVM, WasmiVMError,
};
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
	ReadlyViolation,
	OutOfGas,
	Unsupported,
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

pub struct CosmwasmVMCache {
	pub code: BTreeMap<CosmwasmCodeId, Vec<u8>>,
}

pub struct CosmwasmVMShared {
	pub readonly: u32,
	pub depth: u32,
	pub gas: Gas,
	pub cache: CosmwasmVMCache,
}

impl CosmwasmVMShared {
	pub fn readonly(&self) -> bool {
		self.readonly == 0
	}
	pub fn push_readonly(&mut self) {
		self.readonly += 1;
	}
	pub fn pop_readonly(&mut self) {
		self.readonly -= 1;
	}
}

pub struct CosmwasmVM<'a, T: Config> {
	pub host_functions: BTreeMap<WasmiHostFunctionIndex, WasmiHostFunction<Self>>,
	pub executing_module: WasmiModule,
	pub cosmwasm_env: Env,
	pub cosmwasm_message_info: MessageInfo,
	pub contract_address: CosmwasmAccount<T>,
	pub contract_info: ContractInfoOf<T>,
	pub shared: &'a mut CosmwasmVMShared,
	pub _marker: PhantomData<T>,
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
}

impl<'a, T: Config> Has<BTreeMap<WasmiHostFunctionIndex, WasmiHostFunction<Self>>>
	for CosmwasmVM<'a, T>
{
	fn get(&self) -> BTreeMap<WasmiHostFunctionIndex, WasmiHostFunction<Self>> {
		self.host_functions.clone()
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
	type CodeId = CosmwasmContractMeta<CosmwasmAccount<T>>;
	type Address = CosmwasmAccount<T>;
	type StorageKey = Vec<u8>;
	type StorageValue = Vec<u8>;
	type Error = CosmwasmVMError<T>;

	fn set_code_id(
		&mut self,
		address: Self::Address,
		CosmwasmContractMeta { code_id: new_code_id, admin, label }: Self::CodeId,
	) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "set_code_id");
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

	fn code_id(&mut self, address: Self::Address) -> Result<Self::CodeId, Self::Error> {
		log::debug!(target: "runtime::contracts", "code_id");
		let info = Pallet::<T>::contract_info(address.as_ref())?;
		Ok(CosmwasmContractMeta {
			code_id: info.code_id,
			admin: info.admin.map(CosmwasmAccount::new),
			label: String::from_utf8_lossy(&info.label.into_inner()).into(),
		})
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
		CosmwasmContractMeta { code_id, admin, label }: Self::CodeId,
		funds: Vec<Coin>,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_minimal_std::Event),
	) -> Result<Option<cosmwasm_minimal_std::Binary>, Self::Error> {
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
		//  TODO: support iterating over all tokens???
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
			admin: info.admin.clone().map(|admin| CosmwasmAccount::<T>::new(admin).into()),
			pinned,
			// TODO: IBC
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
		if self.shared.readonly() {
			Err(CosmwasmVMError::ReadlyViolation)
		} else {
			Pallet::<T>::do_db_write(self, &key, &value)?;
			Ok(())
		}
	}

	fn db_remove(&mut self, key: Self::StorageKey) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "db_remove");
		if self.shared.readonly() {
			Err(CosmwasmVMError::ReadlyViolation)
		} else {
			Pallet::<T>::do_db_remove(self, &key);
			Ok(())
		}
	}

	fn abort(&mut self, message: String) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "db_abort");
		Err(CosmwasmVMError::Aborted(message))
	}

	fn charge(&mut self, value: VmGas) -> Result<(), Self::Error> {
		let gas_to_charge = match value {
			VmGas::Instrumentation { metered } => metered as u64,
			// TODO: gas for each operations
			_ => 1u64,
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
			.filter(|entry| match *entry.external() {
				External::Function(_) => true,
				_ => false,
			})
			.count();
		for (requirement, name, signature) in expected_exports {
			match export_entries.iter().find(|e| &e.field() == name) {
				Some(export) => {
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
						.ok_or_else(|| ValidationError::ExportDoesNotExists(name))?
						.type_ref();
					let Type::Function(ref func_ty) = types
						.get(func_ty_idx as usize)
						.ok_or_else(|| ValidationError::ExportWithoutSignature(name))?;
					if signature != &func_ty.params() {
						return Err(ValidationError::ExportWithWrongSignature {
							export_name: name,
							expected_signature: signature.to_vec(),
							actual_signature: func_ty.params().to_vec(),
						})
					}
				},
				None if *requirement == ExportRequirement::Mandatory =>
					return Err(ValidationError::MissingMandatoryExport(name)),
				None => {
					// Not mandatory
				},
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
			let Type::Function(_) = types
				.get(*type_idx as usize)
				.ok_or_else(|| ValidationError::ImportWithoutSignature)?;
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
