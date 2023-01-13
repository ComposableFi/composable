//! Implementations of common trait definitions from [orml](https://docs.rs/orml-traits).

use crate::{Config, Pallet};
use composable_traits::assets::{AssetType, AssetTypeInspect};
use frame_support::{
	dispatch::DispatchResult,
	traits::{
		Currency, ExistenceRequirement, Get, LockableCurrency, ReservableCurrency, WithdrawReasons,
	},
};
use orml_traits::{MultiCurrency, MultiLockableCurrency, MultiReservableCurrency};
use sp_runtime::{traits::CheckedSub, ArithmeticError, DispatchError};

macro_rules! route {
	(
		fn $fn:ident($asset:ident: $asset_ty:ty, $($arg:ident: $ty:ty),*) $(-> $ret:ty)?;
	) => {
		fn $fn($asset: $asset_ty, $($arg:$ty),*) $(-> $ret)? {
			if T::AssetId::from($asset.into()) == <T::NativeAssetId as frame_support::traits::Get<_>>::get() {
				<<T as Config>::NativeCurrency>::$fn($($arg),*)
			} else {
				match <T::AssetLookup as composable_traits::assets::AssetTypeInspect>::inspect(&$asset) {
					composable_traits::assets::AssetType::Foreign => {
						<<T as Config>::ForeignTransactor>::$fn($asset, $($arg),*)
					}
					composable_traits::assets::AssetType::Local => {
						<<T as Config>::LocalTransactor>::$fn($asset, $($arg),*)
					}
				}
			}
		}
	};
}

impl<T: Config> MultiCurrency<T::AccountId> for Pallet<T> {
	type CurrencyId = T::AssetId;
	type Balance = T::Balance;

	fn minimum_balance(currency_id: Self::CurrencyId) -> Self::Balance {
		if currency_id == T::NativeAssetId::get() {
			<<T as Config>::NativeCurrency>::minimum_balance()
		} else {
			match <T::AssetLookup as AssetTypeInspect>::inspect(&currency_id) {
				AssetType::Foreign =>
					<<T as Config>::ForeignTransactor>::minimum_balance(currency_id),
				AssetType::Local => <<T as Config>::LocalTransactor>::minimum_balance(currency_id),
			}
		}
	}

	fn total_issuance(currency_id: Self::CurrencyId) -> Self::Balance {
		if currency_id == T::NativeAssetId::get() {
			<<T as Config>::NativeCurrency>::total_issuance()
		} else {
			match <T::AssetLookup as AssetTypeInspect>::inspect(&currency_id) {
				AssetType::Foreign =>
					<<T as Config>::ForeignTransactor>::total_issuance(currency_id),
				AssetType::Local => <<T as Config>::LocalTransactor>::total_issuance(currency_id),
			}
		}
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
			let new_balance = <<T as Config>::NativeCurrency>::free_balance(who)
				.checked_sub(&amount)
				.ok_or(DispatchError::Arithmetic(ArithmeticError::Underflow))?;
			<<T as Config>::NativeCurrency>::ensure_can_withdraw(
				who,
				amount,
				WithdrawReasons::all(),
				new_balance,
			)
		} else {
			match <T::AssetLookup as AssetTypeInspect>::inspect(&currency_id) {
				AssetType::Foreign => <<T as Config>::ForeignTransactor>::ensure_can_withdraw(
					currency_id,
					who,
					amount,
				),
				AssetType::Local =>
					<<T as Config>::LocalTransactor>::ensure_can_withdraw(currency_id, who, amount),
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
			<<T as Config>::NativeCurrency>::transfer(
				from,
				to,
				amount,
				ExistenceRequirement::AllowDeath,
			)
		} else {
			match <T::AssetLookup as AssetTypeInspect>::inspect(&currency_id) {
				AssetType::Foreign =>
					<<T as Config>::ForeignTransactor>::transfer(currency_id, from, to, amount),
				AssetType::Local =>
					<<T as Config>::LocalTransactor>::transfer(currency_id, from, to, amount),
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
			<<T as Config>::NativeCurrency>::deposit_creating(who, amount);
			Ok(())
		} else {
			match <T::AssetLookup as AssetTypeInspect>::inspect(&currency_id) {
				AssetType::Foreign =>
					<<T as Config>::ForeignTransactor>::deposit(currency_id, who, amount),
				AssetType::Local =>
					<<T as Config>::LocalTransactor>::deposit(currency_id, who, amount),
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
			<<T as Config>::NativeCurrency>::withdraw(
				who,
				amount,
				WithdrawReasons::all(),
				ExistenceRequirement::AllowDeath,
			)
			.map(|_| ())
		} else {
			match <T::AssetLookup as AssetTypeInspect>::inspect(&currency_id) {
				AssetType::Foreign =>
					<<T as Config>::ForeignTransactor>::withdraw(currency_id, who, amount),
				AssetType::Local =>
					<<T as Config>::LocalTransactor>::withdraw(currency_id, who, amount),
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
			<<T as Config>::NativeCurrency>::slash(who, amount).1
		} else {
			match <T::AssetLookup as AssetTypeInspect>::inspect(&currency_id) {
				AssetType::Foreign =>
					<<T as Config>::ForeignTransactor>::slash(currency_id, who, amount),
				AssetType::Local =>
					<<T as Config>::LocalTransactor>::slash(currency_id, who, amount),
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
			<<T as Config>::NativeCurrency>::set_lock(lock_id, who, amount, WithdrawReasons::all());
			Ok(())
		} else {
			match <T::AssetLookup as AssetTypeInspect>::inspect(&currency_id) {
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
			<<T as Config>::NativeCurrency>::extend_lock(
				lock_id,
				who,
				amount,
				WithdrawReasons::all(),
			);
			Ok(())
		} else {
			match <T::AssetLookup as AssetTypeInspect>::inspect(&currency_id) {
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
			<<T as Config>::NativeCurrency>::remove_lock(lock_id, who);
			Ok(())
		} else {
			match <T::AssetLookup as AssetTypeInspect>::inspect(&currency_id) {
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
			<<T as Config>::NativeCurrency>::slash_reserved(who, amount).1
		} else {
			match <T::AssetLookup as AssetTypeInspect>::inspect(&currency_id) {
				AssetType::Foreign =>
					<<T as Config>::ForeignTransactor>::slash_reserved(currency_id, who, amount),
				AssetType::Local =>
					<<T as Config>::LocalTransactor>::slash_reserved(currency_id, who, amount),
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
