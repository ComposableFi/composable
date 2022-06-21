use composable_maths::labs::numbers::IntoDecimal;
use sp_runtime::{traits::Zero, FixedI128, FixedPointNumber};

/// Returns whether `a` isn't larger than `b` and at most a tiny amount smaller
pub fn approx_eq_lower<T: IntoDecimal<FixedI128>>(a: T, b: T) -> bool {
	let a_dec: FixedI128 = a.into_decimal().unwrap();
	let b_dec: FixedI128 = b.into_decimal().unwrap();
	let diff = b_dec - a_dec;
	FixedI128::zero() <= diff && diff < FixedI128::saturating_from_rational(1, 10_i32.pow(8))
}
