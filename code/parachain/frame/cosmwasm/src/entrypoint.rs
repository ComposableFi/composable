use crate::{
	runtimes::wasmi::{CosmwasmVM, CosmwasmVMError, CosmwasmVMShared},
	AccountIdOf, CodeHashToId, CodeIdToInfo, Config, ContractInfoOf, ContractLabelOf,
	ContractMessageOf, ContractToInfo, CurrentNonce, EntryPoint, Error, Event, FundsOf,
	InstrumentedCode, Pallet, PristineCode,
};
use composable_support::abstractions::utils::increment::Increment;
use core::marker::PhantomData;
use cosmwasm_minimal_std::Coin;
use cosmwasm_vm::{
	executor::{ExecuteInput, InstantiateInput, MigrateInput},
	system::{cosmwasm_system_entrypoint, cosmwasm_system_run, CosmwasmCallVM, CosmwasmCodeId},
	vm::VMBase,
};
use cosmwasm_vm_wasmi::WasmiVM;
use frame_support::{
	ensure,
	traits::{Get, ReservableCurrency},
};
use sp_runtime::{traits::Hash, SaturatedConversion};

pub struct EntryPointCaller<S: CallerState> {
	state: S,
}

pub struct Dispatchable<I, O, T: Config> {
	sender: AccountIdOf<T>,
	contract: AccountIdOf<T>,
	contract_info: ContractInfoOf<T>,
	entry_point: EntryPoint,
	output: O,
	marker: PhantomData<I>,
}

pub trait CallerState {}

impl CallerState for MigrateInput {}

impl CallerState for InstantiateInput {}

impl CallerState for ExecuteInput {}

impl<I, O, T: Config> CallerState for Dispatchable<I, O, T> {}

impl EntryPointCaller<InstantiateInput> {
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

pub type VmErrorOf<T> = <T as VMBase>::Error;

impl<I, O, T> EntryPointCaller<Dispatchable<I, O, T>>
where
	T: Config,
{
	pub(crate) fn call(
		self,
		shared: &mut CosmwasmVMShared,
		funds: FundsOf<T>,
		message: ContractMessageOf<T>,
	) -> Result<O, CosmwasmVMError<T>>
	where
		for<'x> WasmiVM<CosmwasmVM<'x, T>>: CosmwasmCallVM<I>,
		for<'x> VmErrorOf<WasmiVM<CosmwasmVM<'x, T>>>: Into<CosmwasmVMError<T>>,
	{
		Pallet::<T>::do_extrinsic_dispatch(
			shared,
			self.state.entry_point,
			self.state.sender,
			self.state.contract,
			self.state.contract_info,
			funds,
			|vm| cosmwasm_system_entrypoint::<I, _>(vm, &message).map_err(Into::into),
		)?;
		Ok(self.state.output)
	}

	pub(crate) fn continue_run(
		self,
		shared: &mut CosmwasmVMShared,
		funds: Vec<Coin>,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_minimal_std::Event),
	) -> Result<Option<cosmwasm_minimal_std::Binary>, CosmwasmVMError<T>>
	where
		for<'x> WasmiVM<CosmwasmVM<'x, T>>: CosmwasmCallVM<I>,
		for<'x> VmErrorOf<WasmiVM<CosmwasmVM<'x, T>>>: Into<CosmwasmVMError<T>>,
	{
		Pallet::<T>::cosmwasm_call(
			shared,
			self.state.sender,
			self.state.contract,
			self.state.contract_info,
			funds,
			|vm| cosmwasm_system_run::<I, _>(vm, message, event_handler).map_err(Into::into),
		)
	}
}

impl EntryPointCaller<ExecuteInput> {
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

impl EntryPointCaller<MigrateInput> {
	pub(crate) fn setup<T: Config>(
		migrator: AccountIdOf<T>,
		contract: AccountIdOf<T>,
		new_code_id: CosmwasmCodeId,
	) -> Result<EntryPointCaller<Dispatchable<MigrateInput, (), T>>, Error<T>> {
		CodeIdToInfo::<T>::try_mutate_exists(new_code_id, |entry| -> Result<(), Error<T>> {
			let code_info = entry.as_mut().ok_or(Error::<T>::CodeNotFound)?;
			code_info.refcount =
				code_info.refcount.checked_add(1).ok_or(Error::<T>::RefcountOverflow)?;
			Ok(())
		})?;

		let (contract_info, code_id) = ContractToInfo::<T>::try_mutate(
			&contract,
			|entry| -> Result<(ContractInfoOf<T>, u64), Error<T>> {
				let info = entry.as_mut().ok_or(Error::<T>::ContractNotFound)?;
				ensure!(info.admin.as_ref() == Some(&migrator), Error::<T>::NotAuthorized);
				let old_code_id = info.code_id;
				info.code_id = new_code_id;
				Ok((info.clone(), old_code_id))
			},
		)?;

		CodeIdToInfo::<T>::try_mutate_exists(code_id, |entry| -> Result<(), Error<T>> {
			let code_info = entry.as_mut().ok_or(Error::<T>::CodeNotFound)?;
			code_info.refcount =
				code_info.refcount.checked_sub(1).ok_or(Error::<T>::RefcountOverflow)?;
			if code_info.refcount == 0 {
				// Code is unused after this point, so it can be removed
				*entry = None;
				let code =
					PristineCode::<T>::try_get(code_id).map_err(|_| Error::<T>::CodeNotFound)?;
				let deposit = code.len().saturating_mul(T::CodeStorageByteDeposit::get() as _);
				let _ = T::NativeAsset::unreserve(&migrator, deposit.saturated_into());
				let code_hash = T::Hashing::hash(&code);
				PristineCode::<T>::remove(code_id);
				InstrumentedCode::<T>::remove(code_id);
				CodeHashToId::<T>::remove(code_hash);
			}
			Ok(())
		})?;

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
