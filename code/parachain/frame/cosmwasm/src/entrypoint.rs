use crate::{
	runtimes::wasmi::{CosmwasmVMError, CosmwasmVMShared},
	AccountIdOf, CodeHashToId, CodeIdToInfo, Config, ContractInfoOf, ContractLabelOf,
	ContractMessageOf, ContractToInfo, CurrentNonce, EntryPoint, Error, Event, FundsOf,
	InstrumentedCode, Pallet, PristineCode,
};
use composable_support::abstractions::utils::increment::Increment;
use cosmwasm_vm::{
	executor::{InstantiateInput, MigrateInput},
	system::{cosmwasm_system_entrypoint, CosmwasmCodeId},
};
use frame_support::{
	ensure,
	traits::{Get, ReservableCurrency},
};
use sp_runtime::{traits::Hash, SaturatedConversion};

pub struct EntryPointCaller<S: CallerState> {
	state: S,
}

pub struct MigrateCall<T: Config> {
	contract_info: ContractInfoOf<T>,
	migrator: AccountIdOf<T>,
	contract: AccountIdOf<T>,
}

pub trait CallerState {}

impl CallerState for MigrateInput {}
impl<T: Config> CallerState for MigrateCall<T> {}

impl CallerState for InstantiateInput {}
impl<T: Config> CallerState for InstantiateCall<T> {}

pub struct InstantiateCall<T: Config> {
	instantiator: AccountIdOf<T>,
	contract: AccountIdOf<T>,
	contract_info: ContractInfoOf<T>,
}

impl EntryPointCaller<InstantiateInput> {
	pub(crate) fn setup<T: Config>(
		instantiator: AccountIdOf<T>,
		code_id: CosmwasmCodeId,
		salt: &[u8],
		admin: Option<AccountIdOf<T>>,
		label: ContractLabelOf<T>,
		message: &[u8],
	) -> Result<EntryPointCaller<InstantiateCall<T>>, Error<T>> {
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
		Ok(EntryPointCaller { state: InstantiateCall { instantiator, contract, contract_info } })
	}
}

impl<T> EntryPointCaller<InstantiateCall<T>>
where
	T: Config,
{
	pub(crate) fn call(
		self,
		shared: &mut CosmwasmVMShared,
		funds: FundsOf<T>,
		message: ContractMessageOf<T>,
	) -> Result<AccountIdOf<T>, CosmwasmVMError<T>> {
		let contract = self.state.contract;
		Pallet::<T>::do_extrinsic_dispatch(
			shared,
			EntryPoint::Instantiate,
			self.state.instantiator,
			contract.clone(),
			self.state.contract_info,
			funds,
			|vm| cosmwasm_system_entrypoint::<InstantiateInput, _>(vm, &message),
		)?;
		Ok(contract)
	}
}

impl<T> EntryPointCaller<MigrateCall<T>>
where
	T: Config,
{
	pub(crate) fn call(
		self,
		shared: &mut CosmwasmVMShared,
		message: ContractMessageOf<T>,
	) -> Result<(), CosmwasmVMError<T>> {
		Pallet::<T>::do_extrinsic_dispatch(
			shared,
			EntryPoint::Migrate,
			self.state.migrator,
			self.state.contract,
			self.state.contract_info,
			Default::default(),
			|vm| cosmwasm_system_entrypoint::<MigrateInput, _>(vm, &message),
		)
	}
}

impl EntryPointCaller<MigrateInput> {
	/// Setup for a contract migration.
	/// This is called prior to calling the `migrate` export of the contract.
	pub(crate) fn setup<T: Config>(
		migrator: AccountIdOf<T>,
		contract: AccountIdOf<T>,
		new_code_id: CosmwasmCodeId,
	) -> Result<EntryPointCaller<MigrateCall<T>>, Error<T>> {
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

		Ok(EntryPointCaller { state: MigrateCall { contract_info, contract, migrator } })
	}
}
