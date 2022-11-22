use crate::{
	runtimes::wasmi::{CosmwasmVM, CosmwasmVMError, CosmwasmVMShared},
	AccountIdOf, CodeIdToInfo, Config, ContractInfoOf, ContractLabelOf, ContractMessageOf,
	ContractToInfo, CurrentNonce, EntryPoint, Error, Event, FundsOf, Pallet,
};
use alloc::{
	string::{String, ToString},
	vec::Vec,
};
use composable_support::abstractions::utils::increment::Increment;
use core::marker::PhantomData;
use cosmwasm_minimal_std::{Binary, Coin, Event as CosmwasmEvent};
#[cfg(feature = "ibc")]
use cosmwasm_vm::executor::ibc::*;
use cosmwasm_vm::{
	executor::{ExecuteInput, InstantiateInput, MigrateInput},
	system::{
		cosmwasm_system_entrypoint, cosmwasm_system_entrypoint_serialize, cosmwasm_system_run,
		CosmwasmCallVM, CosmwasmCodeId, StargateCosmwasmCallVM,
	},
	vm::VmErrorOf,
};
use cosmwasm_vm_wasmi::WasmiVM;
use frame_support::ensure;

/// State machine for entrypoint calls like `instantiate`, `migrate`, etc.
pub struct EntryPointCaller<S: CallerState> {
	state: S,
}

/// Generic ready-to-call state for all input types
pub struct Dispatchable<I, O, T: Config> {
	sender: AccountIdOf<T>,
	contract: AccountIdOf<T>,
	contract_info: ContractInfoOf<T>,
	entry_point: EntryPoint,
	/// Output of the dispatch call, () can be used in case it is irrelevant
	output: O,
	marker: PhantomData<I>,
}

pub trait CallerState {}

impl CallerState for MigrateInput {}

impl CallerState for InstantiateInput {}

impl CallerState for ExecuteInput {}

impl<I, O, T: Config> CallerState for Dispatchable<I, O, T> {}

/// Setup state for `instantiate` entrypoint.
impl EntryPointCaller<InstantiateInput> {
	/// Prepares for `instantiate` entrypoint call.
	///
	/// * `instantiator` - Address of the account that calls this entrypoint.
	pub(crate) fn setup<T: Config>(
		instantiator: AccountIdOf<T>,
		code_id: CosmwasmCodeId,
		salt: &[u8],
		admin: Option<AccountIdOf<T>>,
		label: ContractLabelOf<T>,
		message: &[u8],
	) -> Result<EntryPointCaller<Dispatchable<InstantiateInput, AccountIdOf<T>, T>>, Error<T>> {
		let code_hash = CodeIdToInfo::<T>::get(code_id)
			.ok_or(Error::<T>::CodeNotFound)?
			.pristine_code_hash;
		let contract =
			Pallet::<T>::derive_contract_address(&instantiator, salt, code_hash, message);
		// Make sure that contract address does not already exist
		ensure!(!ContractToInfo::<T>::contains_key(&contract), Error::<T>::ContractAlreadyExists);
		let nonce = CurrentNonce::<T>::increment().map_err(|_| Error::<T>::NonceOverflow)?;
		let trie_id = Pallet::<T>::derive_contract_trie_id(&contract, nonce);
		let contract_info = ContractInfoOf::<T> {
			instantiator: instantiator.clone(),
			code_id,
			trie_id,
			admin,
			label,
		};
		ContractToInfo::<T>::insert(&contract, &contract_info);
		CodeIdToInfo::<T>::try_mutate(code_id, |entry| -> Result<(), Error<T>> {
			let code_info = entry.as_mut().ok_or(Error::<T>::CodeNotFound)?;
			code_info.refcount =
				code_info.refcount.checked_add(1).ok_or(Error::<T>::RefcountOverflow)?;
			Ok(())
		})?;
		Pallet::<T>::deposit_event(Event::<T>::Instantiated {
			contract: contract.clone(),
			info: contract_info.clone(),
		});
		Ok(EntryPointCaller {
			state: Dispatchable {
				sender: instantiator,
				contract: contract.clone(),
				contract_info,
				entry_point: EntryPoint::Instantiate,
				output: contract,
				marker: PhantomData,
			},
		})
	}
}

impl EntryPointCaller<ExecuteInput> {
	/// Prepares for `execute` entrypoint call.
	///
	/// * `executor` - Address of the account that calls this entrypoint.
	/// * `contract` - Address of the contract to be called.
	pub(crate) fn setup<T: Config>(
		executor: AccountIdOf<T>,
		contract: AccountIdOf<T>,
	) -> Result<EntryPointCaller<Dispatchable<ExecuteInput, (), T>>, Error<T>> {
		let contract_info = Pallet::<T>::contract_info(&contract)?;
		Ok(EntryPointCaller {
			state: Dispatchable {
				entry_point: EntryPoint::Execute,
				sender: executor,
				contract,
				contract_info,
				output: (),
				marker: PhantomData,
			},
		})
	}
}

/// Setup state for `migrate` entrypoint.
impl EntryPointCaller<MigrateInput> {
	/// Prepares for `migrate` entrypoint call.
	///
	/// * `migrator` - Address of the account that calls this entrypoint.
	/// * `contract` - Address of the contract to be called.
	/// * `new_code_id` - New code id that the contract will point to (or use).
	pub(crate) fn setup<T: Config>(
		migrator: AccountIdOf<T>,
		contract: AccountIdOf<T>,
		new_code_id: CosmwasmCodeId,
	) -> Result<EntryPointCaller<Dispatchable<MigrateInput, (), T>>, Error<T>> {
		let mut contract_info = Pallet::<T>::contract_info(&contract)?;
		// If the contract is already migrated (which is the case for `continue_migrate`) don't try
		// to migrate again.
		if contract_info.code_id != new_code_id {
			Pallet::<T>::do_set_contract_meta(
				&contract,
				new_code_id,
				contract_info.admin.clone(),
				String::from_utf8_lossy(&contract_info.label).to_string(),
			)?;
			contract_info.code_id = new_code_id;
		}

		Pallet::<T>::deposit_event(Event::<T>::Migrated {
			contract: contract.clone(),
			to: new_code_id,
		});

		Ok(EntryPointCaller {
			state: Dispatchable {
				sender: migrator,
				contract,
				contract_info,
				entry_point: EntryPoint::Migrate,
				output: (),
				marker: PhantomData,
			},
		})
	}
}

/// Dispatch state for all `Input`s
impl<I, O, T> EntryPointCaller<Dispatchable<I, O, T>>
where
	T: Config,
{
	/// Start a cosmwasm transaction by calling an entrypoint.
	///
	/// * `shared` - Shared state of the Cosmwasm VM.
	/// * `funds` - Funds to be transferred before execution.
	/// * `message` - Message to be passed to the entrypoint.
	pub(crate) fn call(
		self,
		shared: &mut CosmwasmVMShared,
		funds: FundsOf<T>,
		message: ContractMessageOf<T>,
	) -> Result<O, CosmwasmVMError<T>>
	where
		for<'x> WasmiVM<CosmwasmVM<'x, T>>: CosmwasmCallVM<I> + StargateCosmwasmCallVM,
		for<'x> VmErrorOf<WasmiVM<CosmwasmVM<'x, T>>>: Into<CosmwasmVMError<T>>,
	{
		self.call_internal(shared, funds, |vm| {
			cosmwasm_system_entrypoint::<I, _>(vm, &message).map_err(Into::into)
		})
	}

	pub fn call_json<Message>(
		self,
		shared: &mut CosmwasmVMShared,
		funds: FundsOf<T>,
		message: Message,
	) -> Result<O, CosmwasmVMError<T>>
	where
		for<'x> WasmiVM<CosmwasmVM<'x, T>>: CosmwasmCallVM<I> + StargateCosmwasmCallVM,
		for<'x> VmErrorOf<WasmiVM<CosmwasmVM<'x, T>>>: Into<CosmwasmVMError<T>>,
		Message: serde::Serialize,
	{
		self.call_internal(shared, funds, |vm| {
			cosmwasm_system_entrypoint_serialize::<I, _, Message>(vm, &message).map_err(Into::into)
		})
	}

	fn call_internal<F>(
		self,
		shared: &mut CosmwasmVMShared,
		funds: FundsOf<T>,
		message: F,
	) -> Result<O, CosmwasmVMError<T>>
	where
		for<'x> WasmiVM<CosmwasmVM<'x, T>>: CosmwasmCallVM<I> + StargateCosmwasmCallVM,
		for<'x> VmErrorOf<WasmiVM<CosmwasmVM<'x, T>>>: Into<CosmwasmVMError<T>>,
		F: for<'x> FnOnce(
			&'x mut WasmiVM<CosmwasmVM<'x, T>>,
		) -> Result<(Option<Binary>, Vec<CosmwasmEvent>), CosmwasmVMError<T>>,
	{
		Pallet::<T>::do_extrinsic_dispatch(
			shared,
			self.state.entry_point,
			self.state.sender,
			self.state.contract,
			self.state.contract_info,
			funds,
			|vm| message(vm).map_err(Into::into),
		)?;
		Ok(self.state.output)
	}

	/// Continue the execution by running an entrypoint. This is used for running
	/// submessages.
	///
	/// * `shared` - Shared state of the Cosmwasm VM.
	/// * `funds` - Funds to be transferred before execution.
	/// * `message` - Message to be passed to the entrypoint.
	/// * `event_handler` - Event handler that is passed by the VM.
	pub(crate) fn continue_run(
		self,
		shared: &mut CosmwasmVMShared,
		funds: Vec<Coin>,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_minimal_std::Event),
	) -> Result<Option<cosmwasm_minimal_std::Binary>, CosmwasmVMError<T>>
	where
		for<'x> WasmiVM<CosmwasmVM<'x, T>>: CosmwasmCallVM<I> + StargateCosmwasmCallVM,
		for<'x> VmErrorOf<WasmiVM<CosmwasmVM<'x, T>>>: Into<CosmwasmVMError<T>>,
	{
		// Call `cosmwasm_call` to transfer funds and create the vm instance before
		// calling the callback.
		Pallet::<T>::cosmwasm_call(
			shared,
			self.state.sender,
			self.state.contract,
			self.state.contract_info,
			funds,
			// `cosmwasm_system_run` is called instead of `cosmwasm_system_entrypoint` here
			// because here, we want to continue running the transaction with the given
			// entrypoint, not create a new transaction.
			|vm| cosmwasm_system_run::<I, _>(vm, message, event_handler).map_err(Into::into),
		)
	}
}
