use crate::{
	dispatchable_call::DispatchableCall,
	runtimes::{abstraction::CosmwasmAccount, vm::CosmwasmVMShared},
	types::*,
	CodeIdToInfo, Config, ContractToInfo, CurrentNonce, Error, Event, Pallet,
};

use composable_support::abstractions::utils::increment::Increment;
use core::marker::PhantomData;
use cosmwasm_vm::{
	executor::{ExecuteCall, InstantiateCall, MigrateCall, ReplyCall},
	system::CosmwasmCodeId,
};

use frame_support::ensure;
/// Prepares for `instantiate` entrypoint call.
///
/// * `instantiator` - Address of the account that calls this entrypoint.
pub(crate) fn setup_instantiate_call<T: Config>(
	instantiator: AccountIdOf<T>,
	code_id: CosmwasmCodeId,
	salt: &[u8],
	admin: Option<AccountIdOf<T>>,
	label: ContractLabelOf<T>,
	message: &[u8],
) -> Result<DispatchableCall<InstantiateCall, AccountIdOf<T>, T>, Error<T>> {
	let code_hash = CodeIdToInfo::<T>::get(code_id)
		.ok_or(Error::<T>::CodeNotFound)?
		.pristine_code_hash;
	let contract = Pallet::<T>::derive_contract_address(&instantiator, salt, &code_hash, message)?;
	// Make sure that contract address does not already exist
	ensure!(Pallet::<T>::contract_exists(&contract).is_err(), Error::<T>::ContractAlreadyExists);
	let nonce = CurrentNonce::<T>::increment().map_err(|_| Error::<T>::NonceOverflow)?;
	let trie_id = Pallet::<T>::derive_contract_trie_id(&contract, nonce);
	let contract_info =
		ContractInfoOf::<T> { instantiator: instantiator.clone(), code_id, trie_id, admin, label };
	ContractToInfo::<T>::insert(&contract, &contract_info);
	CodeIdToInfo::<T>::try_mutate(code_id, |entry| -> Result<(), Error<T>> {
		let code_info = entry.as_mut().ok_or(Error::<T>::CodeNotFound)?;
		code_info.refcount =
			code_info.refcount.checked_add(1).ok_or(Error::<T>::RefcountOverflow)?;
		Ok(())
	})?;
	Pallet::<T>::deposit_event(Event::<T>::Instantiated {
		contract: contract.clone(),
		info: contract_info,
	});
	Ok(DispatchableCall {
		sender: instantiator,
		contract: contract.clone(),
		entrypoint: EntryPoint::Instantiate,
		output: contract,
		marker: PhantomData,
	})
}

/// Prepares for `execute` entrypoint call.
///
/// * `executor` - Address of the account that calls this entrypoint.
/// * `contract` - Address of the contract to be called.
pub(crate) fn setup_execute_call<T: Config>(
	executor: AccountIdOf<T>,
	contract: AccountIdOf<T>,
) -> Result<DispatchableCall<ExecuteCall, (), T>, Error<T>> {
	Ok(DispatchableCall {
		entrypoint: EntryPoint::Execute,
		sender: executor,
		contract,
		output: (),
		marker: PhantomData,
	})
}

/// Prepares for `reply` entrypoint call.
///
/// * `executor` - Address of the account that calls this entrypoint.
/// * `contract` - Address of the contract to be called.
pub(crate) fn setup_reply_call<T: Config>(
	executor: AccountIdOf<T>,
	contract: AccountIdOf<T>,
) -> Result<DispatchableCall<ReplyCall, (), T>, Error<T>> {
	Ok(DispatchableCall {
		entrypoint: EntryPoint::Reply,
		sender: executor,
		contract,
		output: (),
		marker: PhantomData,
	})
}

/// Prepares for `migrate` entrypoint call.
///
/// * `migrator` - Address of the account that calls this entrypoint.
/// * `contract` - Address of the contract to be called.
/// * `new_code_id` - New code id that the contract will point to (or use).
pub(crate) fn setup_migrate_call<T: Config>(
	shared: &mut CosmwasmVMShared,
	migrator: AccountIdOf<T>,
	contract: AccountIdOf<T>,
	new_code_id: CosmwasmCodeId,
) -> Result<DispatchableCall<MigrateCall, (), T>, Error<T>> {
	let contract_info = Pallet::<T>::contract_info(&contract)?;
	// If the migrate already happened, no need to do that again.
	// This is the case for sub-message execution where `migrate` is
	// called by the VM.
	if contract_info.code_id != new_code_id {
		Pallet::<T>::sub_level_dispatch(
			shared,
			migrator.clone(),
			contract.clone(),
			Default::default(),
			|mut vm| {
				cosmwasm_vm::system::migrate(
					&mut vm,
					CosmwasmAccount::new(migrator.clone()),
					CosmwasmAccount::new(contract.clone()),
					new_code_id,
				)
			},
		)
		.map_err(|_| Error::<T>::NotAuthorized)?;
	}

	Ok(DispatchableCall {
		sender: migrator,
		contract,
		entrypoint: EntryPoint::Migrate,
		output: (),
		marker: PhantomData,
	})
}
