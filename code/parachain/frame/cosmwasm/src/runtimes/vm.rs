use super::abstraction::{CanonicalCosmwasmAccount, CosmwasmAccount, Gas};
use crate::{runtimes::abstraction::GasOutcome, types::*, weights::WeightInfo, Config, Pallet};
use alloc::{borrow::ToOwned, string::String};
use core::marker::{Send, Sync};
use cosmwasm_vm::{
	cosmwasm_std::{CodeInfoResponse, Coin, ContractInfoResponse, Empty, Env, MessageInfo},
	executor::ExecutorError,
	has::Has,
	memory::{MemoryReadError, MemoryWriteError},
	system::{CosmwasmCodeId, CosmwasmContractMeta, SystemError},
	transaction::Transactional,
	vm::{VMBase, VmGas},
};
use cosmwasm_vm_wasmi::{
	OwnedWasmiVM, WasmiContext, WasmiInput, WasmiModule, WasmiOutput, WasmiVMError,
};
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};
use wasmi::{core::HostError, Instance, Memory};

/// Different type of contract runtimes. A contract might either be dynamically loaded or statically
/// invoked (precompiled).
pub enum ContractBackend {
	/// A dynamically loaded CosmWasmbased contract. This code has previously been uploaded by a
	/// user.
	CosmWasm {
		/// The wasmi module instantiated for the CosmWasm contract.
		executing_module: Option<WasmiModule>,
	},
	/// A substrate pallet, which is a precompiled contract that is included in the runtime.
	Pallet,
}

impl WasmiContext for ContractBackend {
	fn executing_module(&self) -> Option<WasmiModule> {
		match self {
			ContractBackend::CosmWasm { executing_module } => executing_module.clone(),
			ContractBackend::Pallet => None,
		}
	}

	fn set_wasmi_context(&mut self, instance: Instance, memory: Memory) {
		*self =
			ContractBackend::CosmWasm { executing_module: Some(WasmiModule { instance, memory }) };
	}
}

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
	Ibc(String),
}

impl<T: Config + core::marker::Send + core::marker::Sync + 'static> HostError
	for CosmwasmVMError<T>
{
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

/// Initial storage mutability.
pub enum InitialStorageMutability {
	/// The storage is currently readonly, any operation trying to mutate it will result in a
	/// [`CosmwasmVMError::ReadOnlyViolation`]
	ReadOnly,
	/// Mutable operations on the storage are currently allowed.
	ReadWrite,
}

/// VM shared cache
#[derive(Clone)]
pub struct CosmwasmVMCache {
	/// Code cache, a mapping from an identifier to it's code.
	pub code: BTreeMap<CosmwasmCodeId, Vec<u8>>,
}

/// VM shared state
#[derive(Clone)]
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
	pub iterators: BTreeMap<u32, Vec<u8>>,
	/// Actual contract runtime
	pub contract_runtime: ContractBackend,
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

impl<'a, T: Config> WasmiContext for CosmwasmVM<'a, T> {
	fn executing_module(&self) -> Option<WasmiModule> {
		self.contract_runtime.executing_module()
	}

	fn set_wasmi_context(&mut self, instance: Instance, memory: Memory) {
		self.contract_runtime.set_wasmi_context(instance, memory)
	}
}

impl<'a, T: Config> CosmwasmVM<'a, T> {
	pub fn charge_raw(&mut self, gas: u64) -> Result<(), <Self as VMBase>::Error> {
		match self.shared.gas.charge(gas) {
			GasOutcome::Halt => Err(CosmwasmVMError::OutOfGas),
			GasOutcome::Continue => Ok(()),
		}
	}
}

impl<'a, T: Config + Send + Sync> VMBase for CosmwasmVM<'a, T> {
	type Input<'x> = WasmiInput<OwnedWasmiVM<Self>>;
	type Output<'x> = WasmiOutput<OwnedWasmiVM<Self>>;
	type QueryCustom = Empty;
	type MessageCustom = Empty;
	type ContractMeta = CosmwasmContractMeta<CosmwasmAccount<T>>;
	type Address = CosmwasmAccount<T>;
	type CanonicalAddress = CanonicalCosmwasmAccount<T>;
	type StorageKey = Vec<u8>;
	type StorageValue = Vec<u8>;
	type Error = CosmwasmVMError<T>;

	fn running_contract_meta(&mut self) -> Result<Self::ContractMeta, Self::Error> {
		log::debug!(target: "runtime::contracts", "contract_meta");
		Ok(Pallet::<T>::do_running_contract_meta(self))
	}

	fn db_scan(
		&mut self,
		_start: Option<Self::StorageKey>,
		_end: Option<Self::StorageKey>,
		_order: cosmwasm_vm::cosmwasm_std::Order,
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

	fn contract_meta(&mut self, address: Self::Address) -> Result<Self::ContractMeta, Self::Error> {
		log::debug!(target: "runtime::contracts", "code_id");
		Pallet::<T>::do_contract_meta(address.into_inner())
	}

	fn continue_query(
		&mut self,
		address: Self::Address,
		message: &[u8],
	) -> Result<cosmwasm_vm::executor::QueryResult, Self::Error> {
		log::debug!(target: "runtime::contracts", "continue_query");
		Pallet::<T>::do_continue_query(self, address.into_inner(), message)
	}

	fn continue_execute(
		&mut self,
		address: Self::Address,
		funds: Vec<Coin>,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_vm::cosmwasm_std::Event),
	) -> Result<Option<cosmwasm_vm::cosmwasm_std::Binary>, Self::Error> {
		log::debug!(target: "runtime::contracts", "continue_execute");
		Pallet::<T>::do_continue_execute(self, address.into_inner(), funds, message, event_handler)
	}

	fn continue_reply(
		&mut self,
		message: cosmwasm_vm::cosmwasm_std::Reply,
		event_handler: &mut dyn FnMut(cosmwasm_vm::cosmwasm_std::Event),
	) -> Result<Option<cosmwasm_vm::cosmwasm_std::Binary>, Self::Error> {
		log::debug!(target: "runtime::contracts", "continue_reply");
		Pallet::<T>::do_continue_reply(self, message, event_handler)
	}

	fn continue_instantiate(
		&mut self,
		contract_meta: Self::ContractMeta,
		funds: Vec<Coin>,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_vm::cosmwasm_std::Event),
	) -> Result<(Self::Address, Option<cosmwasm_vm::cosmwasm_std::Binary>), Self::Error> {
		log::debug!(target: "runtime::contracts", "continue_instantiate");
		self.continue_instantiate2(contract_meta, funds, b"salt", message, event_handler)
	}

	fn continue_instantiate2(
		&mut self,
		contract_meta: Self::ContractMeta,
		funds: Vec<Coin>,
		salt: &[u8],
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_vm::cosmwasm_std::Event),
	) -> Result<(Self::Address, Option<cosmwasm_vm::cosmwasm_std::Binary>), Self::Error> {
		log::debug!(target: "runtime::contracts", "continue_instantiate2");
		Pallet::<T>::do_continue_instantiate(
			self,
			contract_meta,
			funds,
			salt,
			message,
			event_handler,
		)
		.map(|r| (self.contract_address.clone(), r))
	}

	fn continue_migrate(
		&mut self,
		address: Self::Address,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_vm::cosmwasm_std::Event),
	) -> Result<Option<cosmwasm_vm::cosmwasm_std::Binary>, Self::Error> {
		log::debug!(target: "runtime::contracts", "continue_migrate");
		Pallet::<T>::do_continue_migrate(self, address.into_inner(), message, event_handler)
	}

	fn query_custom(
		&mut self,
		_: Self::QueryCustom,
	) -> Result<
		cosmwasm_vm::cosmwasm_std::SystemResult<cosmwasm_vm::executor::CosmwasmQueryResult>,
		Self::Error,
	> {
		log::debug!(target: "runtime::contracts", "query_custom");
		Err(CosmwasmVMError::Unsupported)
	}

	fn message_custom(
		&mut self,
		_: Self::MessageCustom,
		_: &mut dyn FnMut(cosmwasm_vm::cosmwasm_std::Event),
	) -> Result<Option<cosmwasm_vm::cosmwasm_std::Binary>, Self::Error> {
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
		Ok(Coin { denom, amount: amount.into() })
	}

	fn all_balance(&mut self, account: &Self::Address) -> Result<Vec<Coin>, Self::Error> {
		log::debug!(target: "runtime::contracts", "all balance: {}", Into::<String>::into(account.clone()));
		//  TODO(hussein-aitlahcen): support iterating over all tokens???
		Err(CosmwasmVMError::Unsupported)
	}

	fn supply(&mut self, denom: String) -> Result<Coin, Self::Error> {
		let amount = Pallet::<T>::do_supply(denom.clone())?;
		Ok(Coin { denom, amount: amount.into() })
	}

	fn query_contract_info(
		&mut self,
		address: Self::Address,
	) -> Result<ContractInfoResponse, Self::Error> {
		log::debug!(target: "runtime::contracts", "query_contract_info");
		Pallet::<T>::do_query_contract_info(self, address.into_inner())
	}

	fn query_code_info(
		&mut self,
		code_id: CosmwasmCodeId,
	) -> Result<CodeInfoResponse, Self::Error> {
		log::debug!(target: "runtime::contracts", "query_code_info");
		Pallet::<T>::do_query_code_info(code_id)
	}

	fn debug(&mut self, message: Vec<u8>) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "[CONTRACT-LOG] {}", String::from_utf8_lossy(&message));
		Ok(())
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

	fn addr_validate(&mut self, input: &str) -> Result<Result<(), Self::Error>, Self::Error> {
		log::debug!(target: "runtime::contracts", "addr_validate");
		match Pallet::<T>::do_addr_validate(input.to_owned()) {
			Ok(_) => Ok(Ok(())),
			Err(e) => Ok(Err(e)),
		}
	}

	fn addr_canonicalize(
		&mut self,
		input: &str,
	) -> Result<Result<Self::CanonicalAddress, Self::Error>, Self::Error> {
		log::debug!(target: "runtime::contracts", "addr_canonicalize");
		let account = match Pallet::<T>::do_addr_canonicalize(input.to_owned()) {
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

	fn abort(&mut self, message: String) -> Result<(), Self::Error> {
		log::debug!(target: "runtime::contracts", "db_abort");
		Err(CosmwasmVMError::Aborted(message))
	}

	fn charge(&mut self, value: VmGas) -> Result<(), Self::Error> {
		let gas_to_charge = match value {
			VmGas::Instrumentation { metered } => metered as u64,
			VmGas::DbRead => T::WeightInfo::db_read().ref_time(),
			VmGas::DbWrite => T::WeightInfo::db_write().ref_time(),
			VmGas::DbRemove => T::WeightInfo::db_remove().ref_time(),
			VmGas::DbScan => T::WeightInfo::db_scan().ref_time(),
			VmGas::DbNext => T::WeightInfo::db_next().ref_time(),
			VmGas::Balance => T::WeightInfo::balance().ref_time(),
			VmGas::Secp256k1Verify => T::WeightInfo::secp256k1_verify().ref_time(),
			VmGas::Secp256k1RecoverPubkey => T::WeightInfo::secp256k1_recover_pubkey().ref_time(),
			VmGas::Ed25519Verify => T::WeightInfo::ed25519_verify().ref_time(),
			VmGas::Ed25519BatchVerify => T::WeightInfo::ed25519_batch_verify().ref_time(),
			VmGas::AddrValidate => T::WeightInfo::addr_validate().ref_time(),
			VmGas::AddrCanonicalize => T::WeightInfo::addr_canonicalize().ref_time(),
			VmGas::AddrHumanize => T::WeightInfo::addr_humanize().ref_time(),
			VmGas::GetContractMeta => T::WeightInfo::contract_meta().ref_time(),
			VmGas::SetContractMeta => T::WeightInfo::set_contract_meta().ref_time(),
			VmGas::Transfer { nb_of_coins } => T::WeightInfo::transfer(nb_of_coins).ref_time(),
			VmGas::ContinueExecute { nb_of_coins } =>
				T::WeightInfo::continue_execute(nb_of_coins).ref_time(),
			VmGas::ContinueInstantiate { nb_of_coins } =>
				T::WeightInfo::continue_instantiate(nb_of_coins).ref_time(),
			VmGas::ContinueMigrate => T::WeightInfo::continue_migrate().ref_time(),
			VmGas::ContinueQuery => T::WeightInfo::continue_query().ref_time(),
			VmGas::ContinueReply => T::WeightInfo::continue_reply().ref_time(),
			VmGas::QueryRaw => T::WeightInfo::query_raw().ref_time(),
			VmGas::QueryContractInfo => T::WeightInfo::query_contract_info().ref_time(),
			VmGas::QueryCodeInfo => T::WeightInfo::query_code_info().ref_time(),
			_ => 1_u64,
			// NOTE: **Operations that require no charge**: Debug,
			// NOTE: **Unsupported operations**:
			// 		   QueryCustom, MessageCustom, Burn, AllBalance
			// TODO(aeryz): Implement when centauri is ready: IbcTransfer, IbcSendPacket,
			//				IbcCloseChannel
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

	fn secp256k1_verify(
		&mut self,
		message_hash: &[u8],
		signature: &[u8],
		public_key: &[u8],
	) -> Result<bool, Self::Error> {
		log::debug!(target: "runtime::contracts", "secp256k1_verify");
		Ok(Pallet::<T>::do_secp256k1_verify(message_hash, signature, public_key))
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

	fn ed25519_verify(
		&mut self,
		message: &[u8],
		signature: &[u8],
		public_key: &[u8],
	) -> Result<bool, Self::Error> {
		log::debug!(target: "runtime::contracts", "ed25519_verify");
		Ok(Pallet::<T>::do_ed25519_verify(message, signature, public_key))
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

	fn ibc_transfer(
		&mut self,
		channel_id: String,
		to_address: String,
		amount: Coin,
		timeout: cosmwasm_vm::cosmwasm_std::IbcTimeout,
	) -> Result<(), Self::Error> {
		Pallet::<T>::do_ibc_transfer(self, channel_id, to_address, amount, timeout)
	}

	fn ibc_send_packet(
		&mut self,
		channel_id: String,
		data: cosmwasm_vm::cosmwasm_std::Binary,
		timeout: cosmwasm_vm::cosmwasm_std::IbcTimeout,
	) -> Result<(), Self::Error> {
		Pallet::<T>::do_ibc_send_packet(self, channel_id, data, timeout)
	}

	fn ibc_close_channel(&mut self, channel_id: String) -> Result<(), Self::Error> {
		Pallet::<T>::do_ibc_close_channel(self, channel_id)
	}

	fn transfer_from(
		&mut self,
		from: &Self::Address,
		to: &Self::Address,
		funds: &[Coin],
	) -> Result<(), Self::Error> {
		Pallet::<T>::do_transfer(from.as_ref(), to.as_ref(), funds, false)?;
		Ok(())
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
