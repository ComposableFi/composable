use crate::{AccountIdOf, Config};
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
use sp_std::collections::btree_map::BTreeMap;
use wasmi::CanResume;

#[derive(Debug)]
pub enum CosmwasmVMError {
	Interpreter(wasmi::Error),
	VirtualMachine(WasmiVMError),
}
impl From<wasmi::Error> for CosmwasmVMError {
	fn from(e: wasmi::Error) -> Self {
		Self::Interpreter(e)
	}
}
impl From<WasmiVMError> for CosmwasmVMError {
	fn from(e: WasmiVMError) -> Self {
		CosmwasmVMError::VirtualMachine(e)
	}
}
impl From<SystemError> for CosmwasmVMError {
	fn from(e: SystemError) -> Self {
		CosmwasmVMError::VirtualMachine(e.into())
	}
}
impl From<ExecutorError> for CosmwasmVMError {
	fn from(e: ExecutorError) -> Self {
		CosmwasmVMError::VirtualMachine(e.into())
	}
}
impl From<MemoryReadError> for CosmwasmVMError {
	fn from(e: MemoryReadError) -> Self {
		CosmwasmVMError::VirtualMachine(e.into())
	}
}
impl From<MemoryWriteError> for CosmwasmVMError {
	fn from(e: MemoryWriteError) -> Self {
		CosmwasmVMError::VirtualMachine(e.into())
	}
}
impl Display for CosmwasmVMError {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "{:?}", self)
	}
}
impl CanResume for CosmwasmVMError {
	fn can_resume(&self) -> bool {
		false
	}
}

pub struct CosmwasmVM<T: Config> {
	host_functions: BTreeMap<WasmiHostFunctionIndex, WasmiHostFunction<Self>>,
	executing_module: WasmiModule,
	env: Env,
	info: MessageInfo,
	_marker: PhantomData<T>,
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
	type Address = AccountIdOf<T>;
	type StorageKey = Vec<u8>;
	type StorageValue = Vec<u8>;
	type Error = CosmwasmVMError;

	fn new_contract(&mut self, code_id: Self::CodeId) -> Result<Self::Address, Self::Error> {
		todo!()
	}

	fn set_code_id(
		&mut self,
		address: Self::Address,
		new_code_id: Self::CodeId,
	) -> Result<(), Self::Error> {
		todo!()
	}

	fn code_id(&mut self, address: Self::Address) -> Result<Self::CodeId, Self::Error> {
		todo!()
	}

	fn query_continuation(
		&mut self,
		address: Self::Address,
		message: &[u8],
	) -> Result<cosmwasm_minimal_std::QueryResult, Self::Error> {
		todo!()
	}

	fn continue_execute(
		&mut self,
		address: Self::Address,
		funds: Vec<Coin>,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_minimal_std::Event),
	) -> Result<Option<cosmwasm_minimal_std::Binary>, Self::Error> {
		todo!()
	}

	fn continue_instantiate(
		&mut self,
		address: Self::Address,
		funds: Vec<Coin>,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_minimal_std::Event),
	) -> Result<Option<cosmwasm_minimal_std::Binary>, Self::Error> {
		todo!()
	}

	fn continue_migrate(
		&mut self,
		address: Self::Address,
		funds: Vec<Coin>,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_minimal_std::Event),
	) -> Result<Option<cosmwasm_minimal_std::Binary>, Self::Error> {
		todo!()
	}

	fn query_custom(
		&mut self,
		query: Self::QueryCustom,
	) -> Result<
		cosmwasm_minimal_std::SystemResult<cosmwasm_minimal_std::CosmwasmQueryResult>,
		Self::Error,
	> {
		todo!()
	}

	fn message_custom(
		&mut self,
		message: Self::MessageCustom,
		event_handler: &mut dyn FnMut(cosmwasm_minimal_std::Event),
	) -> Result<Option<cosmwasm_minimal_std::Binary>, Self::Error> {
		todo!()
	}

	fn query_raw(
		&mut self,
		address: Self::Address,
		key: Self::StorageKey,
	) -> Result<Option<Self::StorageValue>, Self::Error> {
		todo!()
	}

	fn transfer(&mut self, to: &Self::Address, funds: &[Coin]) -> Result<(), Self::Error> {
		todo!()
	}

	fn burn(&mut self, funds: &[Coin]) -> Result<(), Self::Error> {
		todo!()
	}

	fn balance(&mut self, account: &Self::Address, denom: String) -> Result<Coin, Self::Error> {
		todo!()
	}

	fn all_balance(&mut self, account: &Self::Address) -> Result<Vec<Coin>, Self::Error> {
		todo!()
	}

	fn query_info(
		&mut self,
		address: Self::Address,
	) -> Result<cosmwasm_minimal_std::ContractInfoResponse, Self::Error> {
		todo!()
	}

	fn db_read(
		&mut self,
		key: Self::StorageKey,
	) -> Result<Option<Self::StorageValue>, Self::Error> {
		todo!()
	}

	fn db_write(
		&mut self,
		key: Self::StorageKey,
		value: Self::StorageValue,
	) -> Result<(), Self::Error> {
		todo!()
	}

	fn db_remove(&mut self, key: Self::StorageKey) -> Result<(), Self::Error> {
		todo!()
	}

	fn abort(&mut self, message: String) -> Result<(), Self::Error> {
		todo!()
	}

	fn charge(&mut self, value: cosmwasm_vm::vm::VmGas) -> Result<(), Self::Error> {
		todo!()
	}

	fn gas_checkpoint_push(
		&mut self,
		checkpoint: cosmwasm_vm::vm::VmGasCheckpoint,
	) -> Result<(), Self::Error> {
		todo!()
	}

	fn gas_checkpoint_pop(&mut self) -> Result<(), Self::Error> {
		todo!()
	}

	fn gas_ensure_available(&mut self) -> Result<(), Self::Error> {
		todo!()
	}
}

impl<T: Config> Has<Env> for CosmwasmVM<T> {
	fn get(&self) -> Env {
		self.env.clone()
	}
}
impl<T: Config> Has<MessageInfo> for CosmwasmVM<T> {
	fn get(&self) -> MessageInfo {
		self.info.clone()
	}
}

impl<T: Config> Transactional for CosmwasmVM<T> {
	type Error = CosmwasmVMError;
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
