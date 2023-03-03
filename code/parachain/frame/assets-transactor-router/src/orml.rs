//! Implementations of common trait definitions from [orml](https://docs.rs/orml-traits).

use crate::{route, route_asset_type, Config, Pallet};
use composable_traits::assets::{AssetType, AssetTypeInspect};
use frame_support::{
	dispatch::DispatchResult,
	traits::{
		Currency, ExistenceRequirement, Get, LockableCurrency, ReservableCurrency, WithdrawReasons,
	},
};
use orml_traits::{MultiCurrency, MultiLockableCurrency, MultiReservableCurrency};
use sp_runtime::{traits::CheckedSub, ArithmeticError, DispatchError};

impl<T: Config> MultiCurrency<T::AccountId> for Pallet<T> {
	type CurrencyId = T::AssetId;
	type Balance = T::Balance;

	route! {
		fn minimum_balance(currency_id: Self::CurrencyId) -> Self::Balance;
	}

	route! {
		fn total_issuance(currency_id: Self::CurrencyId) -> Self::Balance;
	}

	route! {
		fn total_balance(currency_id: Self::CurrencyId, who: &T::AccountId) -> Self::Balance;
	}

	route! {
		fn free_balance(currency_id: Self::CurrencyId, who: &T::AccountId) -> Self::Balance;
	}

	fn ensure_can_withdraw(
		currency_id: Self::CurrencyId,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		if currency_id == T::NativeAssetId::get() {
			let new_balance = <<T as Config>::NativeTransactor>::free_balance(who)
				.checked_sub(&amount)
				.ok_or(DispatchError::Arithmetic(ArithmeticError::Underflow))?;
			<<T as Config>::NativeTransactor>::ensure_can_withdraw(
				who,
				amount,
				WithdrawReasons::all(),
				new_balance,
			)
		} else {
			route_asset_type! {
				ensure_can_withdraw(currency_id, who, amount)
			}
		}
	}

	fn transfer(
		currency_id: Self::CurrencyId,
		from: &T::AccountId,
		to: &T::AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		if currency_id == T::NativeAssetId::get() {
			<<T as Config>::NativeTransactor>::transfer(
				from,
				to,
				amount,
				ExistenceRequirement::AllowDeath,
			)
		} else {
			route_asset_type! {
				transfer(currency_id, from, to, amount)
			}
		}
	}

	fn deposit(
		currency_id: Self::CurrencyId,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		if currency_id == T::NativeAssetId::get() {
			// Drop the imbalance, causing the total issuance to increase, in accordance with the
			// MultiCurrency trait.
			<<T as Config>::NativeTransactor>::deposit_creating(who, amount);
			Ok(())
		} else {
			route_asset_type! {
				deposit(currency_id, who, amount)
			}
		}
	}

	fn withdraw(
		currency_id: Self::CurrencyId,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		if currency_id == T::NativeAssetId::get() {
			// Drop the imbalance, causing the total issuance to decrease, in accordance with the
			// MultiCurrency trait.
			<<T as Config>::NativeTransactor>::withdraw(
				who,
				amount,
				WithdrawReasons::all(),
				ExistenceRequirement::AllowDeath,
			)
			.map(|_| ())
		} else {
			route_asset_type! {
				withdraw(currency_id, who, amount)
			}
		}
	}

	route! {
		fn can_slash(currency_id: Self::CurrencyId, who: &T::AccountId, amount: Self::Balance) -> bool;
	}

	fn slash(
		currency_id: Self::CurrencyId,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> Self::Balance {
		if currency_id == T::NativeAssetId::get() {
			// Drop the imbalance, causing the total issuance to decrease, in accordance with the
			// MultiCurrency trait.
			<<T as Config>::NativeTransactor>::slash(who, amount).1
		} else {
			route_asset_type! {
				slash(currency_id, who, amount)
			}
		}
	}
}

impl<T: Config> MultiLockableCurrency<T::AccountId> for Pallet<T> {
	type Moment = T::BlockNumber;

	fn set_lock(
		lock_id: orml_traits::LockIdentifier,
		currency_id: Self::CurrencyId,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		if currency_id == T::NativeAssetId::get() {
			<<T as Config>::NativeTransactor>::set_lock(
				lock_id,
				who,
				amount,
				WithdrawReasons::all(),
			);
			Ok(())
		} else {
			match <T::AssetsRegistry as AssetTypeInspect>::inspect(&currency_id) {
				AssetType::Foreign =>
					<<T as Config>::ForeignTransactor>::set_lock(lock_id, currency_id, who, amount),
				AssetType::Local =>
					<<T as Config>::LocalTransactor>::set_lock(lock_id, currency_id, who, amount),
			}
		}
	}

	fn extend_lock(
		lock_id: orml_traits::LockIdentifier,
		currency_id: Self::CurrencyId,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		if currency_id == T::NativeAssetId::get() {
			<<T as Config>::NativeTransactor>::extend_lock(
				lock_id,
				who,
				amount,
				WithdrawReasons::all(),
			);
			Ok(())
		} else {
			match <T::AssetsRegistry as AssetTypeInspect>::inspect(&currency_id) {
				AssetType::Foreign => <<T as Config>::ForeignTransactor>::extend_lock(
					lock_id,
					currency_id,
					who,
					amount,
				),
				AssetType::Local =>
					<<T as Config>::LocalTransactor>::extend_lock(lock_id, currency_id, who, amount),
			}
		}
	}

	fn remove_lock(
		lock_id: orml_traits::LockIdentifier,
		currency_id: Self::CurrencyId,
		who: &T::AccountId,
	) -> DispatchResult {
		if currency_id == T::NativeAssetId::get() {
			<<T as Config>::NativeTransactor>::remove_lock(lock_id, who);
			Ok(())
		} else {
			match <T::AssetsRegistry as AssetTypeInspect>::inspect(&currency_id) {
				AssetType::Foreign =>
					<<T as Config>::ForeignTransactor>::remove_lock(lock_id, currency_id, who),
				AssetType::Local =>
					<<T as Config>::LocalTransactor>::remove_lock(lock_id, currency_id, who),
			}
		}
	}
}

impl<T: Config> MultiReservableCurrency<T::AccountId> for Pallet<T> {
	route! {
		fn can_reserve(
			currency_id: Self::CurrencyId,
			who: &T::AccountId,
			amount: Self::Balance
		) -> bool;
	}

	fn slash_reserved(
		currency_id: Self::CurrencyId,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> Self::Balance {
		if currency_id == T::NativeAssetId::get() {
			// Drop the negative imbalance, causing the total issuance to be reduced, in accordance
			// with `MultiReservableCurrency`.
			<<T as Config>::NativeTransactor>::slash_reserved(who, amount).1
		} else {
			route_asset_type! {
				slash_reserved(currency_id, who, amount)
			}
		}
	}

	route! {
		fn reserved_balance(currency_id: Self::CurrencyId, who: &T::AccountId) -> Self::Balance;
	}

	route! {
		fn reserve(
			currency_id: Self::CurrencyId,
			who: &T::AccountId,
			amount: Self::Balance
		) -> sp_runtime::DispatchResult;
	}

	route! {
		fn unreserve(
			currency_id: Self::CurrencyId,
			who: &T::AccountId,
			amount: Self::Balance
		) -> Self::Balance;
	}

	route! {
		fn repatriate_reserved(
			currency_id: Self::CurrencyId,
			slashed: &T::AccountId,
			beneficiary: &T::AccountId,
			amount: Self::Balance,
			status: orml_traits::BalanceStatus
		) -> core::result::Result<Self::Balance, DispatchError>;
	}
}
