use frame_support::pallet_prelude::*;
use num_traits::{CheckedMul, CheckedSub, Zero};

pub trait Decayable<Balance, BlockNumber> {
	fn checked_decay(
		&self,
		amount: Balance,
		last: BlockNumber,
		current: BlockNumber,
	) -> Option<Balance>;
}

#[derive(Decode, Encode, TypeInfo, Debug, PartialEq, Clone)]
pub enum BudgetDecay<Number> {
	Linear(LinearDecay<Number>),
}

impl<Balance, BlockNumber> Decayable<Balance, BlockNumber> for BudgetDecay<Balance>
where
	BlockNumber: CheckedSub + Into<Balance>,
	Balance: CheckedMul + CheckedSub + Zero,
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
	a: Number,
}

impl<Balance, BlockNumber> Decayable<Balance, BlockNumber> for LinearDecay<Balance>
where
	BlockNumber: CheckedSub + Into<Balance>,
	Balance: CheckedMul + CheckedSub + Zero,
{
	fn checked_decay(
		&self,
		amount: Balance,
		last: BlockNumber,
		current: BlockNumber,
	) -> Option<Balance> {
		let diff = current.checked_sub(&last)?;
		let factor = diff.into().checked_mul(&self.a)?;
		Some(amount.checked_sub(&factor).unwrap_or_else(|| Zero::zero()))
	}
}
