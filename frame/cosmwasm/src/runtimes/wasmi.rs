use super::abstraction::{CosmwasmAccount, Gas};
use crate::{AccountIdOf, Config, ContractInfoOf, Pallet, runtimes::abstraction::GasOutcome};
use alloc::string::String;
use core::{fmt::Display, marker::PhantomData, num::NonZeroU32};
use cosmwasm_minimal_std::{Coin, Empty, Env, MessageInfo};
use cosmwasm_vm::{
	executor::ExecutorError,
	has::Has,
	memory::{
		MemoryReadError, MemoryWriteError, Pointable, ReadWriteMemory, ReadableMemory,
		WritableMemory,
	},
	system::{CosmwasmContractMeta, SystemError},
	transaction::Transactional,
	vm::{VMBase, VmErrorOf},
};
use cosmwasm_vm_wasmi::{
	WasmiHostFunction, WasmiHostFunctionIndex, WasmiInput, WasmiModule, WasmiModuleExecutor,
	WasmiOutput, WasmiVM, WasmiVMError,
};
use parity_wasm::elements::{self, External, Internal, MemoryType, Module, Type, ValueType};
use sp_runtime::traits::Convert;
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};
use wasmi::CanResume;
use wasmi_validation::{validate_module, PlainValidator};

#[derive(Debug)]
pub enum CosmwasmVMError<Custom> {
	Interpreter(wasmi::Error),
	VirtualMachine(WasmiVMError),
	Pallet(Custom),
	AccountConversionFailure,
	Aborted(String),
  OutOfGas,
}

impl<T: Config> From<crate::Error<T>> for CosmwasmVMError<crate::Error<T>> {
	fn from(e: crate::Error<T>) -> Self {
		Self::Pallet(e)
	}
}

impl<T> From<wasmi::Error> for CosmwasmVMError<T> {
	fn from(e: wasmi::Error) -> Self {
		Self::Interpreter(e)
	}
}

impl<T> From<WasmiVMError> for CosmwasmVMError<T> {
	fn from(e: WasmiVMError) -> Self {
		Self::VirtualMachine(e)
	}
}

impl<T> From<SystemError> for CosmwasmVMError<T> {
	fn from(e: SystemError) -> Self {
		Self::VirtualMachine(e.into())
	}
}

impl<T> From<ExecutorError> for CosmwasmVMError<T> {
	fn from(e: ExecutorError) -> Self {
		Self::VirtualMachine(e.into())
	}
}

impl<T> From<MemoryReadError> for CosmwasmVMError<T> {
	fn from(e: MemoryReadError) -> Self {
		Self::VirtualMachine(e.into())
	}
}

impl<T> From<MemoryWriteError> for CosmwasmVMError<T> {
	fn from(e: MemoryWriteError) -> Self {
		Self::VirtualMachine(e.into())
	}
}

impl<T> CanResume for CosmwasmVMError<T> {
	fn can_resume(&self) -> bool {
		false
	}
}

pub struct CosmwasmVM<T: Config> {
	pub host_functions: BTreeMap<WasmiHostFunctionIndex, WasmiHostFunction<Self>>,
	pub executing_module: WasmiModule,
	pub cosmwasm_env: Env,
	pub cosmwasm_message_info: MessageInfo,
	pub contract_address: CosmwasmAccount<T>,
	pub contract_info: ContractInfoOf<T>,
  pub gas: Gas,
	pub _marker: PhantomData<T>,
}

impl<T: Config> Has<Env> for CosmwasmVM<T> {
	fn get(&self) -> Env {
		self.cosmwasm_env.clone()
	}
}
impl<T: Config> Has<MessageInfo> for CosmwasmVM<T> {
	fn get(&self) -> MessageInfo {
		self.cosmwasm_message_info.clone()
	}
}

impl<T: Config> WasmiModuleExecutor for CosmwasmVM<T> {
	fn executing_module(&self) -> WasmiModule {
		self.executing_module.clone()
	}
}

impl<T: Config> Has<BTreeMap<WasmiHostFunctionIndex, WasmiHostFunction<Self>>> for CosmwasmVM<T> {
	fn get(&self) -> BTreeMap<WasmiHostFunctionIndex, WasmiHostFunction<Self>> {
		self.host_functions.clone()
	}
}

impl<T: Config> Pointable for CosmwasmVM<T> {
	type Pointer = u32;
}

impl<T: Config> ReadableMemory for CosmwasmVM<T> {
	type Error = VmErrorOf<Self>;
	fn read(&self, offset: Self::Pointer, buffer: &mut [u8]) -> Result<(), Self::Error> {
		self.executing_module
			.memory
			.get_into(offset, buffer)
			.map_err(|_| WasmiVMError::LowLevelMemoryReadError.into())
	}
}

impl<T: Config> WritableMemory for CosmwasmVM<T> {
	type Error = VmErrorOf<Self>;
	fn write(&self, offset: Self::Pointer, buffer: &[u8]) -> Result<(), Self::Error> {
		self.executing_module
			.memory
			.set(offset, buffer)
			.map_err(|_| WasmiVMError::LowLevelMemoryWriteError.into())
	}
}

impl<T: Config> ReadWriteMemory for CosmwasmVM<T> {}

impl<T: Config> CosmwasmVM<T> {
	fn load_subvm<R>(
		&mut self,
		address: <Self as VMBase>::Address,
		funds: Vec<Coin>,
		f: impl FnOnce(&mut WasmiVM<CosmwasmVM<T>>) -> R,
	) -> Result<R, VmErrorOf<Self>> {
		todo!()
	}
}

impl<T: Config> VMBase for CosmwasmVM<T> {
	type Input<'x> = WasmiInput<'x, WasmiVM<Self>>;
	type Output<'x> = WasmiOutput<'x, WasmiVM<Self>>;
	type QueryCustom = Empty;
	type MessageCustom = Empty;
	type CodeId = CosmwasmContractMeta;
	type Address = CosmwasmAccount<T>;
	type StorageKey = Vec<u8>;
	type StorageValue = Vec<u8>;
	type Error = CosmwasmVMError<crate::Error<T>>;

	fn new_contract(&mut self, code_id: Self::CodeId) -> Result<Self::Address, Self::Error> {
		log::debug!(target: "runtime::contracts", "new_contract");
		todo!()
	}

	fn set_code_id(
		&mut self,
		address: Self::Address,
		new_code_id: Self::CodeId,
	) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "set_code_id");
		todo!()
	}

	fn code_id(&mut self, address: Self::Address) -> Result<Self::CodeId, Self::Error> {
		log::debug!(target: "runtime::contracts", "code_id");
		todo!()
	}

	fn query_continuation(
		&mut self,
		address: Self::Address,
		message: &[u8],
	) -> Result<cosmwasm_minimal_std::QueryResult, Self::Error> {
		log::debug!(target: "runtime::contracts", "query_continuation");
		todo!()
	}

	fn continue_execute(
		&mut self,
		address: Self::Address,
		funds: Vec<Coin>,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_minimal_std::Event),
	) -> Result<Option<cosmwasm_minimal_std::Binary>, Self::Error> {
		log::debug!(target: "runtime::contracts", "continue_execute");
		todo!()
	}

	fn continue_instantiate(
		&mut self,
		address: Self::Address,
		funds: Vec<Coin>,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_minimal_std::Event),
	) -> Result<Option<cosmwasm_minimal_std::Binary>, Self::Error> {
		log::debug!(target: "runtime::contracts", "continue_instantiate");
		todo!()
	}

	fn continue_migrate(
		&mut self,
		address: Self::Address,
		funds: Vec<Coin>,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_minimal_std::Event),
	) -> Result<Option<cosmwasm_minimal_std::Binary>, Self::Error> {
		log::debug!(target: "runtime::contracts", "continue_migrate");
		todo!()
	}

	fn query_custom(
		&mut self,
		query: Self::QueryCustom,
	) -> Result<
		cosmwasm_minimal_std::SystemResult<cosmwasm_minimal_std::CosmwasmQueryResult>,
		Self::Error,
	> {
		log::debug!(target: "runtime::contracts", "query_custom");
		todo!()
	}

	fn message_custom(
		&mut self,
		message: Self::MessageCustom,
		event_handler: &mut dyn FnMut(cosmwasm_minimal_std::Event),
	) -> Result<Option<cosmwasm_minimal_std::Binary>, Self::Error> {
		log::debug!(target: "runtime::contracts", "message_custom");
		todo!()
	}

	fn query_raw(
		&mut self,
		address: Self::Address,
		key: Self::StorageKey,
	) -> Result<Option<Self::StorageValue>, Self::Error> {
		log::debug!(target: "runtime::contracts", "query_raw");
		todo!()
	}

	fn transfer(&mut self, to: &Self::Address, funds: &[Coin]) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "transfer: {:#?}", funds);
		Ok(())
	}

	fn burn(&mut self, funds: &[Coin]) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "burn: {:#?}", funds);
		Ok(())
	}

	fn balance(&mut self, account: &Self::Address, denom: String) -> Result<Coin, Self::Error> {
		log::debug!(target: "runtime::contracts", "balance: {} => {:#?}", Into::<String>::into(account.clone()), denom);
		todo!()
	}

	fn all_balance(&mut self, account: &Self::Address) -> Result<Vec<Coin>, Self::Error> {
		log::debug!(target: "runtime::contracts", "all balance: {}", Into::<String>::into(account.clone()));
		todo!()
	}

	fn query_info(
		&mut self,
		address: Self::Address,
	) -> Result<cosmwasm_minimal_std::ContractInfoResponse, Self::Error> {
		log::debug!(target: "runtime::contracts", "query_info");
		todo!()
	}

	fn db_read(
		&mut self,
		key: Self::StorageKey,
	) -> Result<Option<Self::StorageValue>, Self::Error> {
		log::debug!(target: "runtime::contracts", "db_read");
		let value = Pallet::<T>::do_db_read(&self.contract_info.trie_id, key)?;
		Ok(value)
	}

	fn db_write(
		&mut self,
		key: Self::StorageKey,
		value: Self::StorageValue,
	) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "db_write");
		Pallet::<T>::do_db_write(&self.contract_info.trie_id, key, value)?;
		Ok(())
	}

	fn db_remove(&mut self, key: Self::StorageKey) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "db_remove");
		Pallet::<T>::do_db_remove(&self.contract_info.trie_id, key)?;
		Ok(())
	}

	fn abort(&mut self, message: String) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "db_abort");
		Err(CosmwasmVMError::Aborted(message))
	}

	fn charge(&mut self, value: cosmwasm_vm::vm::VmGas) -> Result<(), Self::Error> {
		// TODO: REALLY IMPORTANT
		Ok(())
	}

	fn gas_checkpoint_push(
		&mut self,
		checkpoint: cosmwasm_vm::vm::VmGasCheckpoint,
	) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "gas_checkpoint_push");
    match self.gas.push(checkpoint) {
        GasOutcome::Continue => Ok(()),
        GasOutcome::Halt => Err(CosmwasmVMError::OutOfGas),
    }
	}

	fn gas_checkpoint_pop(&mut self) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "gas_checkpoint_pop");
    self.gas.pop();
		Ok(())
	}

	fn gas_ensure_available(&mut self) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "gas_ensure_available");
		Ok(())
	}
}

impl<T: Config> Transactional for CosmwasmVM<T> {
	type Error = CosmwasmVMError<crate::Error<T>>;
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
						});
					}
				},
				None if *requirement == ExportRequirement::Mandatory => {
					return Err(ValidationError::MissingMandatoryExport(name))
				},
				None => {
					// Not mandatory
				},
			}
		}
		Ok(self)
	}
	pub fn validate_imports(
		self,
		host_module: &str,
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
				return Err(ValidationError::ImportIsBanned(m, f));
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
				return Err(ValidationError::MustDeclareOneTable);
			}
			if let Some(table_type) = table_section.entries().first() {
				if table_type.limits().initial() > limit {
					return Err(ValidationError::TableExceedLimit);
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
						return Err(ValidationError::BrTableExceedLimit);
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
				return Err(ValidationError::GlobalsExceedLimit);
			}
		}
		Ok(self)
	}
	pub fn validate_no_floating_types(self) -> Result<Self, ValidationError> {
		let CodeValidation(module) = self;
		if let Some(global_section) = module.global_section() {
			for global in global_section.entries() {
				match global.global_type().content_type() {
					ValueType::F32 | ValueType::F64 => {
						return Err(ValidationError::GlobalFloatingPoint)
					},
					_ => {},
				}
			}
		}
		if let Some(code_section) = module.code_section() {
			for func_body in code_section.bodies() {
				for local in func_body.locals() {
					match local.value_type() {
						ValueType::F32 | ValueType::F64 => {
							return Err(ValidationError::LocalFloatingPoint)
						},
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
								ValueType::F32 | ValueType::F64 => {
									return Err(ValidationError::ParamFloatingPoint)
								},
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
					return Err(ValidationError::FunctionParameterExceedLimit);
				}
			}
		}
		Ok(self)
	}
}
