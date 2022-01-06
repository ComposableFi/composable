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
	pub fn linear(n: Number) -> BudgetDecay<Number> {
		BudgetDecay::Linear(LinearDecay { factor: n })
	}
}

impl<Balance, BlockNumber> Decayable<Balance, BlockNumber> for BudgetDecay<Balance>
where
	BlockNumber: CheckedSub + Into<Balance>,
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
	BlockNumber: CheckedSub + Into<Balance>,
	Balance: CheckedMul + Saturating + Zero,
{
	fn checked_decay(
		&self,
		amount: Balance,
		last: BlockNumber,
		current: BlockNumber,
	) -> Option<Balance> {
		let diff = current.checked_sub(&last)?;
		let reduction = diff.into().checked_mul(&self.factor)?;
		Some(amount.saturating_sub(reduction))
	}
}
