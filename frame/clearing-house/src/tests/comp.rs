use composable_maths::labs::numbers::TryIntoDecimal;
use sp_runtime::{traits::Zero, FixedI128, FixedPointNumber};

/// Returns whether `a` isn't larger than `b` and at most a tiny amount smaller
pub fn approx_eq_lower<T: TryIntoDecimal<FixedI128>>(a: T, b: T) -> bool {
	let a_dec: FixedI128 = a.try_into_decimal().unwrap();
	let b_dec: FixedI128 = b.try_into_decimal().unwrap();
	let diff = b_dec - a_dec;
	FixedI128::zero() <= diff && diff < FixedI128::saturating_from_rational(1, 10_i32.pow(8))
}
