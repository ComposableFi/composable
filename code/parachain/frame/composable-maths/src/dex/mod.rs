use sp_runtime::{traits::CheckedAdd, PerThing};

pub mod constant_product;
pub mod price;
#[cfg(test)]
pub mod tests;

pub trait WeightMath<'t>: PerThing + 't {
	/// Sums all weights in an array. If weights total greater than `Perthing::one()`, returns
	/// `None`.
	fn sum_weights(weights: impl IntoIterator<Item = &'t Self>) -> Option<Self>;

	/// Returns `true` if all weights in array are non-zero.
	fn non_zero_weights(weights: impl IntoIterator<Item = &'t Self>) -> bool;
}

impl<'t, T: PerThing + CheckedAdd + 't> WeightMath<'t> for T {
	fn sum_weights(weights: impl IntoIterator<Item = &'t Self>) -> Option<Self> {
		weights
			.into_iter()
			.try_fold(Self::zero(), |total_weight, weight_n| total_weight.checked_add(weight_n))
	}

	fn non_zero_weights(weights: impl IntoIterator<Item = &'t Self>) -> bool {
		weights.into_iter().all(|weight| !weight.is_zero())
	}
}
