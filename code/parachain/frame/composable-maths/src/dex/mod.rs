use sp_runtime::{traits::CheckedAdd, PerThing};

pub mod constant_product;
pub mod price;
#[cfg(test)]
pub mod tests;

pub trait WeightMath: PerThing {
	/// Sums all weights in an array. If weights total greater than `Perthing::one()`, returns
	/// `None`.
	fn sum_weights(weights: &[Self]) -> Option<Self>;

	/// Returns `true` if all weights in array are non-zero.
	fn non_zero_weights(weights: &[Self]) -> bool;
}

impl<T: PerThing + CheckedAdd> WeightMath for T {
	fn sum_weights(weights: &[Self]) -> Option<Self> {
		weights
			.iter()
			.try_fold(Self::zero(), |total_weight, weight_n| total_weight.checked_add(weight_n))
	}

	fn non_zero_weights(weights: &[Self]) -> bool {
		for weight in weights {
			if weight.is_zero() {
				return false
			}
		}

		true
	}
}
