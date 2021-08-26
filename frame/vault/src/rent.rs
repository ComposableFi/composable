use crate::{BalanceOf, BlockNumberOf, Config};
use composable_traits::vault::Deposit;
use frame_support::pallet_prelude::*;
use sp_runtime::{traits::Saturating, SaturatedConversion};

#[derive(Copy, Clone, Encode, Decode, Debug, PartialEq)]
pub enum Verdict<T: Config> {
	Exempt,
	Charge { remaining: BalanceOf<T>, payable: BalanceOf<T> },
	Evict { reward: BalanceOf<T> },
}

pub fn evaluate_eviction<T: Config>(
	deposit: Deposit<BalanceOf<T>, BlockNumberOf<T>>,
) -> Verdict<T> {
	match deposit {
		Deposit::Existential => Verdict::Exempt,
		Deposit::Rent { amount, at } => {
			let current_block = <frame_system::Pallet<T>>::block_number();
			// Rent was already paid this block.
			if current_block <= at {
				Verdict::Exempt
			} else {
				let num_blocks = current_block.saturating_sub(at).saturated_into::<u32>().into();
				let rent_due = T::RentPerBlock::get().saturating_mul(num_blocks);
				let should_evict = rent_due >= amount;
				if should_evict {
					return Verdict::Evict { reward: amount }
				}
				Verdict::Charge { remaining: amount.saturating_sub(rent_due), payable: rent_due }
			}
		},
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mocks::{ExtBuilder, Test};

	#[test]
	fn test_existential() {
		ExtBuilder::default().build().execute_with(|| {
			assert_eq!(evaluate_eviction::<Test>(Deposit::Existential), Verdict::Exempt)
		})
	}

	#[test]
	fn test_charge_exempt() {
		ExtBuilder::default().build().execute_with(|| {
			<frame_system::Pallet<Test>>::set_block_number(2);
			assert_eq!(
				evaluate_eviction::<Test>(Deposit::Rent { amount: 10, at: 2 }),
				Verdict::Exempt
			)
		})
	}

	#[test]
	fn test_charge_simple() {
		ExtBuilder::default().build().execute_with(|| {
			<frame_system::Pallet<Test>>::set_block_number(5);
			assert_eq!(
				evaluate_eviction::<Test>(Deposit::Rent { amount: 10, at: 0 }),
				Verdict::Charge { remaining: 5, payable: 5 }
			)
		})
	}

	#[test]
	fn test_charge_evict() {
		ExtBuilder::default().build().execute_with(|| {
			<frame_system::Pallet<Test>>::set_block_number(11);
			assert_eq!(
				evaluate_eviction::<Test>(Deposit::Rent { amount: 10, at: 0 }),
				Verdict::Evict { reward: 10 }
			)
		})
	}
}
