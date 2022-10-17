use frame_support::pallet_prelude::*;
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, One, Saturating, Zero};

pub trait Decayer<Balance, BlockNumber> {
	/// Decay the `amount` proportionally to the time elapsed `current_block - last_decay_block`
	/// Returns `None` if an input value is invalid
	fn checked_decay(
		&self,
		amount: Balance,
		current_block: BlockNumber,
		last_decay_block: BlockNumber,
	) -> Option<Balance>;

	/// Determine how many blocks are required to pass until the `amount` fully recover from this
	/// decayer. Returns `None` if the recovery period cannot be computed.
	fn full_recovery_period(&self, amount: Balance) -> Option<BlockNumber>;
}

/// Recommend type for storing the decay function of a penalty.
#[derive(Decode, Encode, MaxEncodedLen, TypeInfo, Debug, PartialEq, Eq, Clone)]
pub enum BudgetPenaltyDecayer<Balance, BlockNumber> {
	/// Linear variant of the decay function, which decreases every block.
	Linear(LinearDecay<Balance, BlockNumber>),
}

impl<Balance, BlockNumber> BudgetPenaltyDecayer<Balance, BlockNumber> {
	#[allow(dead_code)]
	pub fn linear(n: Balance) -> BudgetPenaltyDecayer<Balance, BlockNumber> {
		BudgetPenaltyDecayer::Linear(LinearDecay { factor: n, _marker: PhantomData })
	}
}

impl<Balance, BlockNumber> Decayer<Balance, BlockNumber>
	for BudgetPenaltyDecayer<Balance, BlockNumber>
where
	BlockNumber: CheckedSub + Saturating + Into<Balance> + TryFrom<Balance> + One + CheckedAdd,
	Balance: CheckedMul + CheckedDiv + Saturating + Zero,
{
	fn checked_decay(
		&self,
		amount: Balance,
		current: BlockNumber,
		last: BlockNumber,
	) -> Option<Balance> {
		match self {
			BudgetPenaltyDecayer::Linear(lin) => lin.checked_decay(amount, current, last),
		}
	}

	fn full_recovery_period(&self, amount: Balance) -> Option<BlockNumber> {
		match self {
			BudgetPenaltyDecayer::Linear(lin) => lin.full_recovery_period(amount),
		}
	}
}

#[derive(Decode, Encode, MaxEncodedLen, TypeInfo, Default, Debug, PartialEq, Eq, Clone)]
pub struct LinearDecay<Balance, BlockNumber> {
	/// Factor by which we decay every block.
	factor: Balance,
	_marker: core::marker::PhantomData<BlockNumber>,
}

impl<Balance, BlockNumber> Decayer<Balance, BlockNumber> for LinearDecay<Balance, BlockNumber>
where
	BlockNumber: CheckedSub + Saturating + Into<Balance> + TryFrom<Balance> + One + CheckedAdd,
	Balance: CheckedMul + CheckedDiv + Saturating + Zero,
{
	fn checked_decay(
		&self,
		amount: Balance,
		current: BlockNumber,
		last: BlockNumber,
	) -> Option<Balance> {
		let diff = current.saturating_sub(last);
		let reduction = diff.into().checked_mul(&self.factor)?;
		Some(amount.saturating_sub(reduction))
	}

	fn full_recovery_period(&self, amount: Balance) -> Option<BlockNumber> {
		let full_period = amount.checked_div(&self.factor)?;
		let block_full_period: BlockNumber = TryFrom::<Balance>::try_from(full_period).ok()?;
		let block_full_period_plus_one: BlockNumber = block_full_period.checked_add(&One::one())?;
		Some(block_full_period_plus_one)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_linear_decrease() {
		let mut penalty = 1000;
		let prev = penalty;
		let penalty_decayer = BudgetPenaltyDecayer::linear(10);

		(0..=100).for_each(|x| {
			penalty = penalty_decayer.checked_decay(penalty, x, x - 1).unwrap();
			assert!(prev > penalty);
		});
	}
}
