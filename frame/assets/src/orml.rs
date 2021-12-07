//! Implementations of common trait definitions from [orml](https://docs.rs/orml-traits).

use crate::{Config, Pallet};
use frame_support::{
	dispatch::DispatchResult,
	pallet_prelude::MaybeSerializeDeserialize,
	traits::{
		Currency, ExistenceRequirement, Get, LockableCurrency, ReservableCurrency, WithdrawReasons,
	},
};
use num_traits::CheckedSub;
use orml_traits::{MultiCurrency, MultiLockableCurrency, MultiReservableCurrency};
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, Saturating},
	ArithmeticError, DispatchError,
};

impl<T: Config, AccountId> MultiCurrency<AccountId> for Pallet<T>
where
	<T as Config>::Balance: Saturating + AtLeast32BitUnsigned + num_traits::Saturating,
	<T as Config>::AssetId: MaybeSerializeDeserialize,
	<T as Config>::NativeCurrency: Currency<AccountId, Balance = T::Balance>,
	<T as Config>::MultiCurrency:
		MultiCurrency<AccountId, Balance = T::Balance, CurrencyId = T::AssetId>,
{
	type CurrencyId = T::AssetId;

	type Balance = T::Balance;

	fn minimum_balance(currency_id: Self::CurrencyId) -> Self::Balance {
		if currency_id == T::NativeAssetId::get() {
			return <<T as Config>::NativeCurrency>::minimum_balance()
		}
		<<T as Config>::MultiCurrency>::minimum_balance(currency_id)
	}

	fn total_issuance(currency_id: Self::CurrencyId) -> Self::Balance {
		if currency_id == T::NativeAssetId::get() {
			return <<T as Config>::NativeCurrency>::total_issuance()
		}
		<<T as Config>::MultiCurrency>::total_issuance(currency_id)
	}

	fn total_balance(currency_id: Self::CurrencyId, who: &AccountId) -> Self::Balance {
		if currency_id == T::NativeAssetId::get() {
			return <<T as Config>::NativeCurrency>::total_balance(who)
		}
		<<T as Config>::MultiCurrency>::total_balance(currency_id, who)
	}

	fn free_balance(currency_id: Self::CurrencyId, who: &AccountId) -> Self::Balance {
		if currency_id == T::NativeAssetId::get() {
			return <<T as Config>::NativeCurrency>::free_balance(who)
		}
		<<T as Config>::MultiCurrency>::free_balance(currency_id, who)
	}

	fn ensure_can_withdraw(
		currency_id: Self::CurrencyId,
		who: &AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		if currency_id == T::NativeAssetId::get() {
			let new_balace = <<T as Config>::NativeCurrency>::free_balance(who)
				.checked_sub(&amount)
				.ok_or(DispatchError::Arithmetic(ArithmeticError::Underflow))?;
			return <<T as Config>::NativeCurrency>::ensure_can_withdraw(
				who,
				amount,
				WithdrawReasons::all(),
				new_balace,
			)
		}
		<<T as Config>::MultiCurrency>::ensure_can_withdraw(currency_id, who, amount)
	}

	fn transfer(
		currency_id: Self::CurrencyId,
		from: &AccountId,
		to: &AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		if currency_id == T::NativeAssetId::get() {
			return <<T as Config>::NativeCurrency>::transfer(
				from,
				to,
				amount,
				ExistenceRequirement::AllowDeath,
			)
		}
		<<T as Config>::MultiCurrency>::transfer(currency_id, from, to, amount)
	}

	fn deposit(
		currency_id: Self::CurrencyId,
		who: &AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		if currency_id == T::NativeAssetId::get() {
			// Drop the imbalance, causing the total issuance to increase, in accordance with the
			// MultiCurrency trait.
			<<T as Config>::NativeCurrency>::deposit_creating(who, amount);
			return Ok(())
		}
		<<T as Config>::MultiCurrency>::deposit(currency_id, who, amount)
	}

	fn withdraw(
		currency_id: Self::CurrencyId,
		who: &AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		if currency_id == T::NativeAssetId::get() {
			// Drop the imbalance, causing the total issuance to decrease, in accordance with the
			// MultiCurrency trait.
			return <<T as Config>::NativeCurrency>::withdraw(
				who,
				amount,
				WithdrawReasons::all(),
				ExistenceRequirement::AllowDeath,
			)
			.map(|_| ())
		}
		<<T as Config>::MultiCurrency>::withdraw(currency_id, who, amount)
	}

	fn can_slash(currency_id: Self::CurrencyId, who: &AccountId, amount: Self::Balance) -> bool {
		if currency_id == T::NativeAssetId::get() {
			return <<T as Config>::NativeCurrency>::can_slash(who, amount)
		}
		<<T as Config>::MultiCurrency>::can_slash(currency_id, who, amount)
	}

	fn slash(
		currency_id: Self::CurrencyId,
		who: &AccountId,
		amount: Self::Balance,
	) -> Self::Balance {
		if currency_id == T::NativeAssetId::get() {
			// Drop the imbalance, causing the total issuance to decrease, in accordance with the
			// MultiCurrency trait.
			return <<T as Config>::NativeCurrency>::slash(who, amount).1
		}
		<<T as Config>::MultiCurrency>::slash(currency_id, who, amount)
	}
}

impl<T: Config, AccountId> MultiLockableCurrency<AccountId> for Pallet<T>
where
	<T as Config>::NativeCurrency: LockableCurrency<AccountId, Balance = T::Balance>
		+ Currency<AccountId, Balance = T::Balance>,
	<T as Config>::MultiCurrency:
		MultiLockableCurrency<AccountId, Balance = T::Balance, CurrencyId = T::AssetId>,
	<T as Config>::Balance: Saturating + num_traits::Saturating,
	<T as Config>::AssetId: MaybeSerializeDeserialize,
{
	type Moment = T::BlockNumber;

	fn set_lock(
		lock_id: orml_traits::LockIdentifier,
		currency_id: Self::CurrencyId,
		who: &AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		if currency_id == T::NativeAssetId::get() {
			<<T as Config>::NativeCurrency>::set_lock(lock_id, who, amount, WithdrawReasons::all());
			return Ok(())
		}
		<<T as Config>::MultiCurrency>::set_lock(lock_id, currency_id, who, amount)
	}

	fn extend_lock(
		lock_id: orml_traits::LockIdentifier,
		currency_id: Self::CurrencyId,
		who: &AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		if currency_id == T::NativeAssetId::get() {
			<<T as Config>::NativeCurrency>::extend_lock(
				lock_id,
				who,
				amount,
				WithdrawReasons::all(),
			);
			return Ok(())
		}
		<<T as Config>::MultiCurrency>::extend_lock(lock_id, currency_id, who, amount)
	}

	fn remove_lock(
		lock_id: orml_traits::LockIdentifier,
		currency_id: Self::CurrencyId,
		who: &AccountId,
	) -> DispatchResult {
		if currency_id == T::NativeAssetId::get() {
			<<T as Config>::NativeCurrency>::remove_lock(lock_id, who);
			return Ok(())
		}
		<<T as Config>::MultiCurrency>::remove_lock(lock_id, currency_id, who)
	}
}

impl<T: Config, AccountId> MultiReservableCurrency<AccountId> for Pallet<T>
where
	<T as Config>::NativeCurrency: ReservableCurrency<AccountId, Balance = T::Balance>
		+ Currency<AccountId, Balance = T::Balance>,
	<T as Config>::MultiCurrency:
		MultiReservableCurrency<AccountId, Balance = T::Balance, CurrencyId = T::AssetId>,
	<T as Config>::Balance: Saturating + num_traits::Saturating,
	<T as Config>::AssetId: MaybeSerializeDeserialize,
{
	fn can_reserve(currency_id: Self::CurrencyId, who: &AccountId, amount: Self::Balance) -> bool {
		if currency_id == T::NativeAssetId::get() {
			return <<T as Config>::NativeCurrency>::can_reserve(who, amount)
		}
		<<T as Config>::MultiCurrency>::can_reserve(currency_id, who, amount)
	}

	fn slash_reserved(
		currency_id: Self::CurrencyId,
		who: &AccountId,
		amount: Self::Balance,
	) -> Self::Balance {
		if currency_id == T::NativeAssetId::get() {
			// Drop the negative imbalance, causing the total issuance to be reduced, in accordance
			// with `MultiReservableCurrency`.
			return <<T as Config>::NativeCurrency>::slash_reserved(who, amount).1
		}
		<<T as Config>::MultiCurrency>::slash_reserved(currency_id, who, amount)
	}

	fn reserved_balance(currency_id: Self::CurrencyId, who: &AccountId) -> Self::Balance {
		if currency_id == T::NativeAssetId::get() {
			return <<T as Config>::NativeCurrency>::reserved_balance(who)
		}
		<<T as Config>::MultiCurrency>::reserved_balance(currency_id, who)
	}

	fn reserve(
		currency_id: Self::CurrencyId,
		who: &AccountId,
		amount: Self::Balance,
	) -> sp_runtime::DispatchResult {
		if currency_id == T::NativeAssetId::get() {
			return <<T as Config>::NativeCurrency>::reserve(who, amount)
		}
		<<T as Config>::MultiCurrency>::reserve(currency_id, who, amount)
	}

	fn unreserve(
		currency_id: Self::CurrencyId,
		who: &AccountId,
		amount: Self::Balance,
	) -> Self::Balance {
		if currency_id == T::NativeAssetId::get() {
			return <<T as Config>::NativeCurrency>::unreserve(who, amount)
		}
		<<T as Config>::MultiCurrency>::unreserve(currency_id, who, amount)
	}

	fn repatriate_reserved(
		currency_id: Self::CurrencyId,
		slashed: &AccountId,
		beneficiary: &AccountId,
		amount: Self::Balance,
		status: orml_traits::BalanceStatus,
	) -> core::result::Result<Self::Balance, DispatchError> {
		if currency_id == T::NativeAssetId::get() {
			return <<T as Config>::NativeCurrency>::repatriate_reserved(
				slashed,
				beneficiary,
				amount,
				status,
			)
		}
		<<T as Config>::MultiCurrency>::repatriate_reserved(
			currency_id,
			slashed,
			beneficiary,
			amount,
			status,
		)
	}
}
