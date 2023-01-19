use frame_support::traits::tokens::currency::{Currency, LockableCurrency, ReservableCurrency};

use crate::{Config, Pallet};

impl<T: Config> Currency<T::AccountId> for Pallet<T> {
	type Balance = <T::NativeCurrency as Currency<T::AccountId>>::Balance;
	type PositiveImbalance = <T::NativeCurrency as Currency<T::AccountId>>::PositiveImbalance;
	type NegativeImbalance = <T::NativeCurrency as Currency<T::AccountId>>::NegativeImbalance;

	fn total_balance(who: &T::AccountId) -> Self::Balance {
		T::NativeCurrency::total_balance(who)
	}

	fn can_slash(who: &T::AccountId, value: Self::Balance) -> bool {
		T::NativeCurrency::can_slash(who, value)
	}

	fn total_issuance() -> Self::Balance {
		T::NativeCurrency::total_issuance()
	}

	fn minimum_balance() -> Self::Balance {
		T::NativeCurrency::minimum_balance()
	}

	fn burn(amount: Self::Balance) -> Self::PositiveImbalance {
		T::NativeCurrency::burn(amount)
	}

	fn issue(amount: Self::Balance) -> Self::NegativeImbalance {
		T::NativeCurrency::issue(amount)
	}

	fn free_balance(who: &T::AccountId) -> Self::Balance {
		T::NativeCurrency::free_balance(who)
	}

	fn ensure_can_withdraw(
		who: &T::AccountId,
		amount: Self::Balance,
		reasons: frame_support::traits::WithdrawReasons,
		new_balance: Self::Balance,
	) -> frame_support::pallet_prelude::DispatchResult {
		T::NativeCurrency::ensure_can_withdraw(who, amount, reasons, new_balance)
	}

	fn transfer(
		source: &T::AccountId,
		dest: &T::AccountId,
		value: Self::Balance,
		existence_requirement: frame_support::traits::ExistenceRequirement,
	) -> frame_support::pallet_prelude::DispatchResult {
		T::NativeCurrency::transfer(source, dest, value, existence_requirement)
	}

	fn slash(who: &T::AccountId, value: Self::Balance) -> (Self::NegativeImbalance, Self::Balance) {
		T::NativeCurrency::slash(who, value)
	}

	fn deposit_into_existing(
		who: &T::AccountId,
		value: Self::Balance,
	) -> Result<Self::PositiveImbalance, sp_runtime::DispatchError> {
		T::NativeCurrency::deposit_into_existing(who, value)
	}

	fn deposit_creating(who: &T::AccountId, value: Self::Balance) -> Self::PositiveImbalance {
		T::NativeCurrency::deposit_creating(who, value)
	}

	fn withdraw(
		who: &T::AccountId,
		value: Self::Balance,
		reasons: frame_support::traits::WithdrawReasons,
		liveness: frame_support::traits::ExistenceRequirement,
	) -> Result<Self::NegativeImbalance, sp_runtime::DispatchError> {
		T::NativeCurrency::withdraw(who, value, reasons, liveness)
	}

	fn make_free_balance_be(
		who: &T::AccountId,
		balance: Self::Balance,
	) -> frame_support::traits::SignedImbalance<Self::Balance, Self::PositiveImbalance> {
		T::NativeCurrency::make_free_balance_be(who, balance)
	}
}

impl<T: Config> LockableCurrency<T::AccountId> for Pallet<T> {
	type Moment = <T::NativeCurrency as LockableCurrency<T::AccountId>>::Moment;
	type MaxLocks = <T::NativeCurrency as LockableCurrency<T::AccountId>>::MaxLocks;

	fn set_lock(
		id: orml_traits::LockIdentifier,
		who: &T::AccountId,
		amount: Self::Balance,
		reasons: frame_support::traits::WithdrawReasons,
	) {
		T::NativeCurrency::set_lock(id, who, amount, reasons)
	}

	fn extend_lock(
		id: orml_traits::LockIdentifier,
		who: &T::AccountId,
		amount: Self::Balance,
		reasons: frame_support::traits::WithdrawReasons,
	) {
		T::NativeCurrency::extend_lock(id, who, amount, reasons)
	}

	fn remove_lock(id: orml_traits::LockIdentifier, who: &T::AccountId) {
		T::NativeCurrency::remove_lock(id, who)
	}
}

impl<T: Config> ReservableCurrency<T::AccountId> for Pallet<T> {
	fn can_reserve(who: &T::AccountId, value: Self::Balance) -> bool {
		T::NativeCurrency::can_reserve(who, value)
	}

	fn slash_reserved(
		who: &T::AccountId,
		value: Self::Balance,
	) -> (Self::NegativeImbalance, Self::Balance) {
		T::NativeCurrency::slash_reserved(who, value)
	}

	fn reserved_balance(who: &T::AccountId) -> Self::Balance {
		T::NativeCurrency::reserved_balance(who)
	}

	fn reserve(
		who: &T::AccountId,
		value: Self::Balance,
	) -> frame_support::pallet_prelude::DispatchResult {
		T::NativeCurrency::reserve(who, value)
	}

	fn unreserve(who: &T::AccountId, value: Self::Balance) -> Self::Balance {
		T::NativeCurrency::unreserve(who, value)
	}

	fn repatriate_reserved(
		slashed: &T::AccountId,
		beneficiary: &T::AccountId,
		value: Self::Balance,
		status: orml_traits::BalanceStatus,
	) -> Result<Self::Balance, sp_runtime::DispatchError> {
		T::NativeCurrency::repatriate_reserved(slashed, beneficiary, value, status)
	}
}
