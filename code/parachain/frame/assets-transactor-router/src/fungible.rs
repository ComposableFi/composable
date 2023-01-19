use frame_support::{
	pallet_prelude::*,
	traits::tokens::{
		fungible::{Inspect, InspectHold, Mutate, MutateHold, Transfer, Unbalanced},
		DepositConsequence, WithdrawConsequence,
	},
};

use crate::{Config, Pallet};

impl<T: Config> MutateHold<T::AccountId> for Pallet<T> {
	fn hold(who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
		<<T as Config>::NativeCurrency>::hold(who, amount)
	}

	fn release(
		who: &T::AccountId,
		amount: Self::Balance,
		best_effort: bool,
	) -> Result<Self::Balance, DispatchError> {
		<<T as Config>::NativeCurrency>::release(who, amount, best_effort)
	}

	fn transfer_held(
		source: &T::AccountId,
		dest: &T::AccountId,
		amount: Self::Balance,
		best_effort: bool,
		on_held: bool,
	) -> Result<Self::Balance, DispatchError> {
		<<T as Config>::NativeCurrency>::transfer_held(source, dest, amount, best_effort, on_held)
	}
}

impl<T: Config> Mutate<T::AccountId> for Pallet<T> {
	fn mint_into(who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
		<<T as Config>::NativeCurrency>::mint_into(who, amount)
	}
	fn burn_from(
		who: &T::AccountId,
		amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError> {
		<<T as Config>::NativeCurrency>::burn_from(who, amount)
	}

	fn slash(who: &T::AccountId, amount: Self::Balance) -> Result<Self::Balance, DispatchError> {
		<<T as Config>::NativeCurrency>::slash(who, amount)
	}
	fn teleport(
		source: &T::AccountId,
		dest: &T::AccountId,
		amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError> {
		<<T as Config>::NativeCurrency>::teleport(source, dest, amount)
	}
}

impl<T: Config> Unbalanced<T::AccountId> for Pallet<T> {
	fn set_balance(who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
		<<T as Config>::NativeCurrency>::set_balance(who, amount)
	}

	fn set_total_issuance(amount: Self::Balance) {
		<<T as Config>::NativeCurrency>::set_total_issuance(amount)
	}

	fn decrease_balance(
		who: &T::AccountId,
		amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError> {
		<<T as Config>::NativeCurrency>::decrease_balance(who, amount)
	}

	fn decrease_balance_at_most(who: &T::AccountId, amount: Self::Balance) -> Self::Balance {
		<<T as Config>::NativeCurrency>::decrease_balance_at_most(who, amount)
	}

	fn increase_balance(
		who: &T::AccountId,
		amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError> {
		<<T as Config>::NativeCurrency>::increase_balance(who, amount)
	}

	fn increase_balance_at_most(who: &T::AccountId, amount: Self::Balance) -> Self::Balance {
		<<T as Config>::NativeCurrency>::increase_balance_at_most(who, amount)
	}
}

impl<T: Config> Transfer<T::AccountId> for Pallet<T> {
	fn transfer(
		source: &T::AccountId,
		dest: &T::AccountId,
		amount: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::Balance, DispatchError> {
		<<T as Config>::NativeCurrency>::transfer(source, dest, amount, keep_alive)
	}
}

impl<T: Config> Inspect<T::AccountId> for Pallet<T> {
	type Balance = T::Balance;

	fn total_issuance() -> Self::Balance {
		<<T as Config>::NativeCurrency>::total_issuance()
	}

	fn minimum_balance() -> Self::Balance {
		<<T as Config>::NativeCurrency>::minimum_balance()
	}

	fn balance(who: &T::AccountId) -> Self::Balance {
		<<T as Config>::NativeCurrency>::balance(who)
	}

	fn reducible_balance(who: &T::AccountId, keep_alive: bool) -> Self::Balance {
		<<T as Config>::NativeCurrency>::reducible_balance(who, keep_alive)
	}

	fn can_deposit(who: &T::AccountId, amount: Self::Balance, mint: bool) -> DepositConsequence {
		<<T as Config>::NativeCurrency>::can_deposit(who, amount, mint)
	}

	fn can_withdraw(
		who: &T::AccountId,
		amount: Self::Balance,
	) -> WithdrawConsequence<Self::Balance> {
		<<T as Config>::NativeCurrency>::can_withdraw(who, amount)
	}
}

impl<T: Config> InspectHold<T::AccountId> for Pallet<T>
where
	<T as Config>::NativeCurrency:
		Inspect<T::AccountId, Balance = T::Balance> + InspectHold<T::AccountId>,
{
	fn balance_on_hold(who: &T::AccountId) -> Self::Balance {
		<<T as Config>::NativeCurrency>::balance_on_hold(who)
	}

	fn can_hold(who: &T::AccountId, amount: Self::Balance) -> bool {
		<<T as Config>::NativeCurrency>::can_hold(who, amount)
	}
}
