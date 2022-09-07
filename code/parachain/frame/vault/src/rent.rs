use crate::{BalanceOf, BlockNumberOf, Config};
use composable_traits::vault::Deposit;
use frame_support::pallet_prelude::*;
use sp_runtime::{traits::Saturating, SaturatedConversion};

#[derive(Copy, Clone, Encode, Decode, Debug, PartialEq, Eq)]
pub enum Verdict<T: Config> {
	Exempt,
	Charge { remaining: BalanceOf<T>, payable: BalanceOf<T> },
	Evict,
}

pub fn deposit_from_balance<T: Config>(amount: T::Balance) -> Deposit<T::Balance, T::BlockNumber> {
	if amount > T::ExistentialDeposit::get() {
		Deposit::Existential
	} else {
		Deposit::Rent { amount, at: <frame_system::Pallet<T>>::block_number() }
	}
}

pub fn evaluate_deletion<T: Config>(
	current_block: BlockNumberOf<T>,
	deposit: Deposit<BalanceOf<T>, BlockNumberOf<T>>,
) -> bool {
	match deposit {
		Deposit::Existential => false,
		Deposit::Rent { at, .. } => current_block.saturating_sub(at) >= T::TombstoneDuration::get(),
	}
}

pub fn evaluate_eviction<T: Config>(
	current_block: BlockNumberOf<T>,
	deposit: Deposit<BalanceOf<T>, BlockNumberOf<T>>,
) -> Verdict<T> {
	match deposit {
		Deposit::Existential => Verdict::Exempt,
		Deposit::Rent { amount, at } => {
			// Rent was already paid this block.
			if current_block <= at {
				Verdict::Exempt
			} else {
				let num_blocks = current_block.saturating_sub(at).saturated_into::<u32>().into();
				let rent_due = T::RentPerBlock::get().saturating_mul(num_blocks);
				let should_evict = rent_due >= amount;
				if should_evict {
					return Verdict::Evict
				}
				Verdict::Charge { remaining: amount.saturating_sub(rent_due), payable: rent_due }
			}
		},
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mocks::tests::{ExtBuilder, Test};

	#[test]
	fn test_existential() {
		ExtBuilder::default().build().execute_with(|| {
			assert_eq!(evaluate_eviction::<Test>(0, Deposit::Existential), Verdict::Exempt)
		})
	}

	#[test]
	fn test_charge_exempt() {
		ExtBuilder::default().build().execute_with(|| {
			assert_eq!(
				evaluate_eviction::<Test>(2, Deposit::Rent { amount: 10, at: 2 }),
				Verdict::Exempt
			)
		})
	}

	#[test]
	fn test_charge_simple() {
		ExtBuilder::default().build().execute_with(|| {
			assert_eq!(
				evaluate_eviction::<Test>(5, Deposit::Rent { amount: 10, at: 0 }),
				Verdict::Charge { remaining: 5, payable: 5 }
			)
		})
	}

	#[test]
	fn test_charge_evict() {
		ExtBuilder::default().build().execute_with(|| {
			assert_eq!(
				evaluate_eviction::<Test>(11, Deposit::Rent { amount: 10, at: 0 }),
				Verdict::Evict
			)
		})
	}
}
