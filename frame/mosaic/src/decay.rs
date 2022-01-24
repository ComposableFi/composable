use frame_support::pallet_prelude::*;
use num_traits::{CheckedMul, CheckedSub, Saturating, Zero};

pub trait Decayer<Balance, BlockNumber> {
	fn checked_decay(
		&self,
		amount: Balance,
		last: BlockNumber,
		current: BlockNumber,
	) -> Option<Balance>;
}

/// Recommend type for storing the decay function of a penalty.
#[derive(Decode, Encode, TypeInfo, Debug, PartialEq, Clone)]
pub enum BudgetPenaltyDecayer<Number> {
	/// Linear variant of the decay function, which decreases every block.
	Linear(LinearDecay<Number>),
}

impl<Number> BudgetPenaltyDecayer<Number> {
	#[allow(dead_code)]
	pub fn linear(n: Number) -> BudgetPenaltyDecayer<Number> {
		BudgetPenaltyDecayer::Linear(LinearDecay { factor: n })
	}
}

impl<Balance, BlockNumber> Decayer<Balance, BlockNumber> for BudgetPenaltyDecayer<Balance>
where
	BlockNumber: CheckedSub + Saturating + Into<Balance>,
	Balance: CheckedMul + Saturating + Zero,
{
	fn checked_decay(
		&self,
		amount: Balance,
		last: BlockNumber,
		current: BlockNumber,
	) -> Option<Balance> {
		match self {
			BudgetPenaltyDecayer::Linear(lin) => lin.checked_decay(amount, last, current),
		}
	}
}

#[derive(Decode, Encode, TypeInfo, Default, Debug, PartialEq, Clone)]
pub struct LinearDecay<Number> {
	/// Factor by which we decay every block.
	factor: Number,
}

impl<Balance, BlockNumber> Decayer<Balance, BlockNumber> for LinearDecay<Balance>
where
	BlockNumber: CheckedSub + Saturating + Into<Balance>,
	Balance: CheckedMul + Saturating + Zero,
{
	fn checked_decay(
		&self,
		amount: Balance,
		last: BlockNumber,
		current: BlockNumber,
	) -> Option<Balance> {
		let diff = current.saturating_sub(last);
		let reduction = diff.into().checked_mul(&self.factor)?;
		Some(amount.saturating_sub(reduction))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_linear_decrease() {
		let mut penalty = 1000;
		let prev = penalty.clone();
		let penalty_decayer = BudgetPenaltyDecayer::linear(10);

		(0..=100).for_each(|x| {
			penalty = penalty_decayer.checked_decay(penalty, x - 1, x).unwrap();
			println!("{} {}", prev, penalty);
			assert!(prev > penalty);
		});
	}
}
