use sp_runtime::{traits::CheckedAdd, PerThing};

pub mod constant_product;
pub mod price;
#[cfg(test)]
pub mod tests;

pub trait PoolWeightMathExt<'t>: 't {
	/// Sums all weights in an array. If weights total greater than `Perthing::one()`, returns
	/// `None`.
	fn sum_weights<Weight: 't + PerThing + CheckedAdd>(
		weights: impl IntoIterator<Item = &'t Weight>,
	) -> Option<Weight>;

	/// Returns `true` if all weights in array are non-zero.
	fn non_zero_weights<Weight: 't + PerThing>(
		weights: impl IntoIterator<Item = &'t Weight>,
	) -> bool;
}

impl<'t, T: PerThing + CheckedAdd + 't> PoolWeightMathExt<'t> for T {
	fn sum_weights<Weight: 't + PerThing + CheckedAdd>(
		weights: impl IntoIterator<Item = &'t Weight>,
	) -> Option<Weight> {
		weights
			.into_iter()
			.try_fold(Weight::zero(), |total_weight, weight_n| total_weight.checked_add(weight_n))
	}

	fn non_zero_weights<Weight: 't + PerThing>(
		weights: impl IntoIterator<Item = &'t Weight>,
	) -> bool {
		weights.into_iter().all(|weight| !weight.is_zero())
	}
}
