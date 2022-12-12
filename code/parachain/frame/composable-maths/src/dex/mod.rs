use core::{cmp::Ordering, ops::Sub};

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

/// Checks if `a` and `b` are within an acceptable level of computation error.
///
/// Currently this checks within `1/100_000`.
///
/// # Example
///
/// ```rust
/// # use sp_runtime::Permill;
/// # use composable_maths::dex::per_thing_acceptable_computation_error;
/// let a = Permill::from_parts(100_000);
/// let b = Permill::from_parts(100_010);
/// let c = Permill::from_parts(99_990);
///
/// assert!(per_thing_acceptable_computation_error(a, b));
/// assert!(per_thing_acceptable_computation_error(a, c));
/// ```
///
/// ```rust
/// # use sp_runtime::Permill;
/// # use composable_maths::dex::per_thing_acceptable_computation_error;
/// let a = Permill::from_parts(100_000);
/// let b = Permill::from_parts(100_020);
/// let c = Permill::from_parts(99_980);
///
/// assert!(!per_thing_acceptable_computation_error(a, b));
/// assert!(!per_thing_acceptable_computation_error(a, c));
/// ```
// TODO(benluelo): Make `c` a param?
pub fn per_thing_acceptable_computation_error<T>(a: T, b: T) -> bool
where
	T: PerThing + Sub<Output = T>,
{
	let c = match a.cmp(&b) {
		Ordering::Less => b - a,
		Ordering::Equal => return true,
		Ordering::Greater => a - b,
	};

	let epsilon = T::from_rational::<u128>(1, 100_000);

	c <= epsilon
}

#[cfg(test)]
mod per_thing_computation_error {
	use sp_runtime::{Perbill, Permill};

	use super::per_thing_acceptable_computation_error;

	#[test]
	fn exact_same() {
		let a = Perbill::from_rational::<u32>(1, 100_000);

		assert!(per_thing_acceptable_computation_error(a, a))
	}

	#[test]
	fn difference_too_large() {
		let a = Permill::from_rational::<u32>(1, 100_000);
		let b = Permill::from_rational::<u32>(3, 100_000);

		assert!(!per_thing_acceptable_computation_error(a, b))
	}

	#[test]
	fn difference_equals_epsilon() {
		let a = Permill::from_parts(499_990);

		let b = Permill::from_rational::<u32>(50, 100);

		// 50% - 49.999% = 0.001%
		assert!(per_thing_acceptable_computation_error(a, b))
	}

	#[test]
	fn acceptable_difference() {
		// 11ppm - 10ppm = 1ppm
		let a = Permill::from_parts(11);
		let b = Permill::from_parts(10);

		// 1 ppm < 0.001%
		assert!(per_thing_acceptable_computation_error(a, b))
	}
}
