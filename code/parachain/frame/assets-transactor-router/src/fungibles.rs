//! Implementations of the various `fungible::*` traits for the pallet.
//!
//! All of these implementations route to the NativeTransactor.

use composable_traits::assets::{
	BiBoundedAssetName, BiBoundedAssetSymbol, InspectRegistryMetadata, MutateRegistryMetadata,
};
use frame_support::{
	pallet_prelude::*,
	traits::tokens::{
		fungible::{
			Inspect as NativeInspect, InspectHold as NativeInspectHold, Mutate as NativeMutate,
			MutateHold as NativeMutateHold, Transfer as NativeTransfer,
			Unbalanced as NativeUnbalanced,
		},
		fungibles::{self, Inspect, InspectHold, Mutate, MutateHold, Transfer, Unbalanced},
		DepositConsequence, WithdrawConsequence,
	},
};
use sp_std::vec::Vec;

use crate::{route, Config, Pallet};

impl<T: Config> fungibles::metadata::Inspect<T::AccountId> for Pallet<T> {
	fn name(asset: Self::AssetId) -> Vec<u8> {
		<T::AssetsRegistry as InspectRegistryMetadata>::asset_name(&asset).unwrap_or_default()
	}

	fn symbol(asset: Self::AssetId) -> Vec<u8> {
		<T::AssetsRegistry as InspectRegistryMetadata>::symbol(&asset).unwrap_or_default()
	}

	fn decimals(asset: Self::AssetId) -> u8 {
		<T::AssetsRegistry as InspectRegistryMetadata>::decimals(&asset).unwrap_or_default()
	}
}

impl<T: Config> fungibles::metadata::Mutate<T::AccountId> for Pallet<T> {
	fn set(
		asset: Self::AssetId,
		_from: &T::AccountId,
		name: Vec<u8>,
		symbol: Vec<u8>,
		decimals: u8,
	) -> DispatchResult {
		let name = BiBoundedAssetName::from_vec(name).ok();
		let symbol = BiBoundedAssetSymbol::from_vec(symbol).ok();

		<T::AssetsRegistry as MutateRegistryMetadata>::set_metadata(
			&asset,
			name,
			symbol,
			Some(decimals),
		)
	}
}

impl<T: Config> fungibles::InspectMetadata<T::AccountId> for Pallet<T> {
	fn name(asset: &Self::AssetId) -> Vec<u8> {
		<Self as fungibles::metadata::Inspect<T::AccountId>>::name(*asset)
	}

	fn symbol(asset: &Self::AssetId) -> Vec<u8> {
		<Self as fungibles::metadata::Inspect<T::AccountId>>::symbol(*asset)
	}

	fn decimals(asset: &Self::AssetId) -> u8 {
		<Self as fungibles::metadata::Inspect<T::AccountId>>::decimals(*asset)
	}
}

impl<T: Config> Unbalanced<T::AccountId> for Pallet<T> {
	route! {
		fn set_balance(
			asset: Self::AssetId,
			who: &T::AccountId,
			amount: Self::Balance
		) -> DispatchResult;
	}

	route! {
		fn set_total_issuance(asset: Self::AssetId, amount: Self::Balance);
	}

	route! {
		fn decrease_balance(
			asset: Self::AssetId,
			who: &T::AccountId,
			amount: Self::Balance
		) -> Result<Self::Balance, DispatchError>;
	}

	route! {
		fn decrease_balance_at_most(
			asset: Self::AssetId,
			who: &T::AccountId,
			amount: Self::Balance
		) -> Self::Balance;
	}

	route! {
		fn increase_balance(
			asset: Self::AssetId,
			who: &T::AccountId,
			amount: Self::Balance
		) -> Result<Self::Balance, DispatchError>;
	}

	route! {
		fn increase_balance_at_most(
			asset: Self::AssetId,
			who: &T::AccountId,
			amount: Self::Balance
		) -> Self::Balance;
	}
}

impl<T: Config> Transfer<T::AccountId> for Pallet<T> {
	route! {
		fn transfer(
			asset: Self::AssetId,
			source: &T::AccountId,
			dest: &T::AccountId,
			amount: Self::Balance,
			keep_alive: bool
		) -> Result<Self::Balance, DispatchError>;
	}
}

impl<T: Config> MutateHold<T::AccountId> for Pallet<T> {
	route! {
		fn hold(asset: Self::AssetId, who: &T::AccountId, amount: Self::Balance) -> DispatchResult;
	}

	route! {
		fn release(
			asset: Self::AssetId,
			who: &T::AccountId,
			amount: Self::Balance,
			best_effort: bool
		) -> Result<Self::Balance, DispatchError>;
	}

	route! {
		fn transfer_held(
			asset: Self::AssetId,
			source: &T::AccountId,
			dest: &T::AccountId,
			amount: Self::Balance,
			best_effort: bool,
			on_hold: bool
		) -> Result<Self::Balance, DispatchError>;
	}
}

impl<T: Config> Mutate<T::AccountId> for Pallet<T> {
	route! {
		fn mint_into(
			asset: Self::AssetId,
			who: &T::AccountId,
			amount: Self::Balance
		) -> DispatchResult;
	}

	route! {
		fn burn_from(
			asset: Self::AssetId,
			who: &T::AccountId,
			amount: Self::Balance
		) -> Result<Self::Balance, DispatchError>;
	}

	route! {
		fn slash(
			asset: Self::AssetId,
			who: &T::AccountId,
			amount: Self::Balance
		) -> Result<Self::Balance, DispatchError>;
	}

	route! {
		fn teleport(
			asset: Self::AssetId,
			source: &T::AccountId,
			dest: &T::AccountId,
			amount: Self::Balance
		) -> Result<Self::Balance, DispatchError>;
	}
}

impl<T: Config> Inspect<T::AccountId> for Pallet<T> {
	type AssetId = T::AssetId;
	type Balance = T::Balance;

	route! {
		fn total_issuance(asset: Self::AssetId) -> Self::Balance;
	}

	route! {
		fn minimum_balance(asset: Self::AssetId) -> Self::Balance;
	}

	route! {
		fn balance(asset: Self::AssetId, who: &T::AccountId) -> Self::Balance;
	}

	route! {
		fn reducible_balance(
			asset: Self::AssetId,
			who: &T::AccountId,
			keep_alive: bool
		) -> Self::Balance;
	}

	route! {
		fn can_deposit(
			asset: Self::AssetId,
			who: &T::AccountId,
			amount: Self::Balance,
			mint: bool
		) -> DepositConsequence;
	}

	route! {
		fn can_withdraw(
			asset: Self::AssetId,
			who: &T::AccountId,
			amount: Self::Balance
		) -> WithdrawConsequence<Self::Balance>;
	}
}

impl<T: Config> InspectHold<T::AccountId> for Pallet<T> {
	route! {
		fn balance_on_hold(asset: Self::AssetId, who: &T::AccountId) -> Self::Balance;
	}

	route! {
		fn can_hold(asset: Self::AssetId, who: &T::AccountId, amount: Self::Balance) -> bool;
	}
}
