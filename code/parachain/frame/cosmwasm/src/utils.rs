use alloc::{string::String, vec::Vec};
use cosmwasm_vm::{cosmwasm_std::Coin, system::CosmwasmContractMeta};
use sp_core::{crypto::UncheckedFrom, storage::ChildInfo};
use sp_runtime::traits::{Convert, Hash};

use crate::{
	runtimes::{
		abstraction::{CanonicalCosmwasmAccount, CosmwasmAccount, VMPallet},
		vm::CosmwasmVMError,
	},
	types::{AccountIdOf, AssetIdOf, BalanceOf, CodeHashOf, ContractInfoOf, ContractTrieIdOf},
	Config, ContractToInfo, Error, Pallet,
};

// TODO(cor): move these out of the `impl` as they do not refer to `self` or `Self`.
impl<T: Config> Pallet<T> {
	/// Deterministic contract address computation, similar to https://eips.ethereum.org/EIPS/eip-1014.
	pub(crate) fn derive_contract_address(
		instantiator: &AccountIdOf<T>,
		salt: &[u8],
		code_hash: CodeHashOf<T>,
		message: &[u8],
	) -> AccountIdOf<T> {
		let data: Vec<_> = instantiator
			.as_ref()
			.iter()
			.chain(salt)
			.chain(code_hash.as_ref())
			.chain(T::Hashing::hash(message).as_ref())
			.cloned()
			.collect();
		UncheckedFrom::unchecked_from(T::Hashing::hash(&data))
	}

	/// Deterministic contract trie id generation.
	pub(crate) fn derive_contract_trie_id(
		contract: &AccountIdOf<T>,
		nonce: u64,
	) -> ContractTrieIdOf<T> {
		let data: Vec<_> = contract.as_ref().iter().chain(&nonce.to_le_bytes()).cloned().collect();
		T::Hashing::hash(&data).as_ref().to_vec().try_into().expect(
			"hashing len implementation must always be <= defined max contract trie id size; QED;",
		)
	}

	/// Handy wrapper to update contract info.
	pub(crate) fn set_contract_info(contract: &AccountIdOf<T>, info: ContractInfoOf<T>) {
		ContractToInfo::<T>::insert(contract, info)
	}

	/// Handy wrapper to return contract info.
	pub(crate) fn contract_info(contract: &AccountIdOf<T>) -> Result<ContractInfoOf<T>, Error<T>> {
		ContractToInfo::<T>::get(contract).ok_or(Error::<T>::ContractNotFound)
	}

	/// Try to convert from a CosmWasm address to a native AccountId.
	pub(crate) fn cosmwasm_addr_to_account(
		cosmwasm_addr: String,
	) -> Result<AccountIdOf<T>, <T as VMPallet>::VmError> {
		T::AccountToAddr::convert(cosmwasm_addr)
			.map_err(|()| CosmwasmVMError::AccountConversionFailure)
	}

	/// Convert from a native ahttps://app.clickup.com/20465559/v/l/6-210281072-1ccount to a CosmWasm address.
	pub(crate) fn account_to_cosmwasm_addr(account: AccountIdOf<T>) -> String {
		T::AccountToAddr::convert(account)
	}

	/// Convert a native asset and amount into a CosmWasm [`Coin`].
	pub(crate) fn native_asset_to_cosmwasm_asset(
		asset: AssetIdOf<T>,
		amount: BalanceOf<T>,
	) -> Coin {
		let denom = T::AssetToDenom::convert(asset);
		Coin { denom, amount: amount.into().into() }
	}

	/// Try to convert from a CosmWasm denom to a native [`AssetIdOf<T>`].
	pub(crate) fn cosmwasm_asset_to_native_asset(denom: String) -> Result<AssetIdOf<T>, Error<T>> {
		T::AssetToDenom::convert(denom).map_err(|_| Error::<T>::UnknownDenom)
	}

	/// Build a [`ChildInfo`] out of a contract trie id.
	pub(crate) fn contract_child_trie(trie_id: &[u8]) -> ChildInfo {
		ChildInfo::new_default(trie_id)
	}

	pub(crate) fn do_contract_meta(
		address: AccountIdOf<T>,
	) -> Result<CosmwasmContractMeta<CosmwasmAccount<T>>, CosmwasmVMError<T>> {
		let info = Pallet::<T>::contract_info(&address)?;
		Ok(CosmwasmContractMeta {
			code_id: info.code_id,
			admin: info.admin.clone().map(CosmwasmAccount::new),
			label: String::from_utf8_lossy(&info.label).into(),
		})
	}

	/// Validate a string address
	pub(crate) fn do_addr_validate(address: String) -> Result<AccountIdOf<T>, CosmwasmVMError<T>> {
		Pallet::<T>::cosmwasm_addr_to_account(address)
	}

	/// Canonicalize a human readable address
	pub(crate) fn do_addr_canonicalize(
		address: String,
	) -> Result<AccountIdOf<T>, CosmwasmVMError<T>> {
		Pallet::<T>::cosmwasm_addr_to_account(address)
	}

	/// Humanize a canonical address
	pub(crate) fn do_addr_humanize(address: &CanonicalCosmwasmAccount<T>) -> CosmwasmAccount<T> {
		address.0.clone()
	}
}
