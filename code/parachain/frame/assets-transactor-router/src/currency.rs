use frame_support::traits::tokens::currency::{Currency, LockableCurrency, ReservableCurrency};

use crate::{Config, Pallet};

impl<T: Config> Currency<T::AccountId> for Pallet<T> {
	type Balance = <T::NativeTransactor as Currency<T::AccountId>>::Balance;
	type PositiveImbalance = <T::NativeTransactor as Currency<T::AccountId>>::PositiveImbalance;
	type NegativeImbalance = <T::NativeTransactor as Currency<T::AccountId>>::NegativeImbalance;

	fn total_balance(who: &T::AccountId) -> Self::Balance {
		T::NativeTransactor::total_balance(who)
	}

	fn can_slash(who: &T::AccountId, value: Self::Balance) -> bool {
		T::NativeTransactor::can_slash(who, value)
	}

	fn total_issuance() -> Self::Balance {
		T::NativeTransactor::total_issuance()
	}

	fn minimum_balance() -> Self::Balance {
		T::NativeTransactor::minimum_balance()
	}

	fn burn(amount: Self::Balance) -> Self::PositiveImbalance {
		T::NativeTransactor::burn(amount)
	}

	fn issue(amount: Self::Balance) -> Self::NegativeImbalance {
		T::NativeTransactor::issue(amount)
	}

	fn free_balance(who: &T::AccountId) -> Self::Balance {
		T::NativeTransactor::free_balance(who)
	}

	fn ensure_can_withdraw(
		who: &T::AccountId,
		amount: Self::Balance,
		reasons: frame_support::traits::WithdrawReasons,
		new_balance: Self::Balance,
	) -> frame_support::pallet_prelude::DispatchResult {
		T::NativeTransactor::ensure_can_withdraw(who, amount, reasons, new_balance)
	}

	fn transfer(
		source: &T::AccountId,
		dest: &T::AccountId,
		value: Self::Balance,
		existence_requirement: frame_support::traits::ExistenceRequirement,
	) -> frame_support::pallet_prelude::DispatchResult {
		T::NativeTransactor::transfer(source, dest, value, existence_requirement)
	}

	fn slash(who: &T::AccountId, value: Self::Balance) -> (Self::NegativeImbalance, Self::Balance) {
		T::NativeTransactor::slash(who, value)
	}

	fn deposit_into_existing(
		who: &T::AccountId,
		value: Self::Balance,
	) -> Result<Self::PositiveImbalance, sp_runtime::DispatchError> {
		T::NativeTransactor::deposit_into_existing(who, value)
	}

	fn deposit_creating(who: &T::AccountId, value: Self::Balance) -> Self::PositiveImbalance {
		T::NativeTransactor::deposit_creating(who, value)
	}

	fn withdraw(
		who: &T::AccountId,
		value: Self::Balance,
		reasons: frame_support::traits::WithdrawReasons,
		liveness: frame_support::traits::ExistenceRequirement,
	) -> Result<Self::NegativeImbalance, sp_runtime::DispatchError> {
		T::NativeTransactor::withdraw(who, value, reasons, liveness)
	}

	fn make_free_balance_be(
		who: &T::AccountId,
		balance: Self::Balance,
	) -> frame_support::traits::SignedImbalance<Self::Balance, Self::PositiveImbalance> {
		T::NativeTransactor::make_free_balance_be(who, balance)
	}
}

impl<T: Config> LockableCurrency<T::AccountId> for Pallet<T> {
	type Moment = <T::NativeTransactor as LockableCurrency<T::AccountId>>::Moment;
	type MaxLocks = <T::NativeTransactor as LockableCurrency<T::AccountId>>::MaxLocks;

	fn set_lock(
		id: orml_traits::LockIdentifier,
		who: &T::AccountId,
		amount: Self::Balance,
		reasons: frame_support::traits::WithdrawReasons,
	) {
		T::NativeTransactor::set_lock(id, who, amount, reasons)
	}

	fn extend_lock(
		id: orml_traits::LockIdentifier,
		who: &T::AccountId,
		amount: Self::Balance,
		reasons: frame_support::traits::WithdrawReasons,
	) {
		T::NativeTransactor::extend_lock(id, who, amount, reasons)
	}

	fn remove_lock(id: orml_traits::LockIdentifier, who: &T::AccountId) {
		T::NativeTransactor::remove_lock(id, who)
	}
}

impl<T: Config> ReservableCurrency<T::AccountId> for Pallet<T> {
	fn can_reserve(who: &T::AccountId, value: Self::Balance) -> bool {
		T::NativeTransactor::can_reserve(who, value)
	}

	fn slash_reserved(
		who: &T::AccountId,
		value: Self::Balance,
	) -> (Self::NegativeImbalance, Self::Balance) {
		T::NativeTransactor::slash_reserved(who, value)
	}

	fn reserved_balance(who: &T::AccountId) -> Self::Balance {
		T::NativeTransactor::reserved_balance(who)
	}

	fn reserve(
		who: &T::AccountId,
		value: Self::Balance,
	) -> frame_support::pallet_prelude::DispatchResult {
		T::NativeTransactor::reserve(who, value)
	}

	fn unreserve(who: &T::AccountId, value: Self::Balance) -> Self::Balance {
		T::NativeTransactor::unreserve(who, value)
	}

	fn repatriate_reserved(
		slashed: &T::AccountId,
		beneficiary: &T::AccountId,
		value: Self::Balance,
		status: orml_traits::BalanceStatus,
	) -> Result<Self::Balance, sp_runtime::DispatchError> {
		T::NativeTransactor::repatriate_reserved(slashed, beneficiary, value, status)
	}
}
