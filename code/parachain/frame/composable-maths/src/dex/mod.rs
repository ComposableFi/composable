use sp_runtime::{traits::CheckedAdd, PerThing};

pub mod constant_product;
pub mod price;
#[cfg(test)]
pub mod tests;

pub trait PoolWeightMathExt<'t, T: PerThing + CheckedAdd + 't> {
	/// Sums all weights in an array. If weights total greater than `Perthing::one()`, returns
	/// `None`.
	fn sum_weights(self) -> Option<T>;

	/// Returns `true` if all weights in array are non-zero.
	fn non_zero_weights(self) -> bool;
}

impl<'t, Iter, T> PoolWeightMathExt<'t, T> for Iter
where
	Iter: Iterator<Item = &'t T>,
	T: PerThing + CheckedAdd + 't,
{
	fn sum_weights(mut self) -> Option<T> {
		self.try_fold(T::zero(), |total_weight, weight_n| total_weight.checked_add(weight_n))
	}

	fn non_zero_weights(self) -> bool {
		self.into_iter().all(|weight| !weight.is_zero())
	}
}
