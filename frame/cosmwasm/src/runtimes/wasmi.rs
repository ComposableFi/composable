use crate::{AccountIdOf, Config};
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
use sp_runtime::traits::Convert;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::vec::Vec;
use wasmi::CanResume;

#[derive(Debug)]
pub enum CosmwasmVMError<Custom> {
	Interpreter(wasmi::Error),
	VirtualMachine(WasmiVMError),
	Pallet(Custom),
	AccountConversionFailure,
	Aborted(String),
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

#[derive(Clone, Debug)]
pub struct CosmwasmAccount<T, U>(PhantomData<T>, U);

impl<T, U> CosmwasmAccount<T, U> {
	pub fn new(x: U) -> Self {
		CosmwasmAccount(PhantomData, x)
	}
}

impl<T: Config> Into<String> for CosmwasmAccount<T, <T as frame_system::Config>::AccountId> {
	fn into(self) -> String {
		T::AccountToAddr::convert(self.1)
	}
}

impl<T: Config> TryFrom<String> for CosmwasmAccount<T, <T as frame_system::Config>::AccountId> {
	type Error = VmErrorOf<CosmwasmVM<T>>;
	fn try_from(value: String) -> Result<Self, Self::Error> {
		T::AccountToAddr::convert(value)
			.map(CosmwasmAccount::new)
			.map_err(|()| CosmwasmVMError::AccountConversionFailure)
	}
}

pub struct CosmwasmVM<T: Config> {
	pub host_functions: BTreeMap<WasmiHostFunctionIndex, WasmiHostFunction<Self>>,
	pub executing_module: WasmiModule,
	pub env: Env,
	pub info: MessageInfo,
	pub _marker: PhantomData<T>,
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
	type Address = CosmwasmAccount<T, AccountIdOf<T>>;
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
		Ok(None)
	}

	fn db_write(
		&mut self,
		key: Self::StorageKey,
		value: Self::StorageValue,
	) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "db_write");
		Ok(())
	}

	fn db_remove(&mut self, key: Self::StorageKey) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "db_remove");
		Ok(())
	}

	fn abort(&mut self, message: String) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "db_abort");
		Err(CosmwasmVMError::Aborted(message))
	}

	fn charge(&mut self, value: cosmwasm_vm::vm::VmGas) -> Result<(), Self::Error> {
		Ok(())
	}

	fn gas_checkpoint_push(
		&mut self,
		checkpoint: cosmwasm_vm::vm::VmGasCheckpoint,
	) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "gas_checkpoint_push");
		Ok(())
	}

	fn gas_checkpoint_pop(&mut self) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "gas_checkpoint_pop");
		Ok(())
	}

	fn gas_ensure_available(&mut self) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "gas_ensure_available");
		Ok(())
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
