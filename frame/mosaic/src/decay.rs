use frame_support::pallet_prelude::*;
use num_traits::{CheckedMul, CheckedSub, Saturating, Zero};

pub trait Decayable<Balance, BlockNumber> {
	fn checked_decay(
		&self,
		amount: Balance,
		last: BlockNumber,
		current: BlockNumber,
	) -> Option<Balance>;
}

/// Recommend type for storing the decay function of a penalty.
#[derive(Decode, Encode, TypeInfo, Debug, PartialEq, Clone)]
pub enum BudgetDecay<Number> {
	/// Linear variant of the decay function, which decreases every block.
	Linear(LinearDecay<Number>),
}

impl<Number> BudgetDecay<Number> {
	#[allow(dead_code)]
	pub fn linear(n: Number) -> BudgetDecay<Number> {
		BudgetDecay::Linear(LinearDecay { factor: n })
	}
}

impl<Balance, BlockNumber> Decayable<Balance, BlockNumber> for BudgetDecay<Balance>
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
			BudgetDecay::Linear(lin) => lin.checked_decay(amount, last, current),
		}
	}
}

#[derive(Decode, Encode, TypeInfo, Default, Debug, PartialEq, Clone)]
pub struct LinearDecay<Number> {
	/// Factor by which we decay every block.
	factor: Number,
}

impl<Balance, BlockNumber> Decayable<Balance, BlockNumber> for LinearDecay<Balance>
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
		let decay = BudgetDecay::linear(10);

		(0..=100).for_each(|x| {
			penalty = decay.checked_decay(penalty, x - 1, x).unwrap();
			println!("{} {}", prev, penalty);
			assert!(prev > penalty);
		});
	}
}
