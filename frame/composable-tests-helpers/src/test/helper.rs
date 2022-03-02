use sp_runtime::{FixedPointNumber, FixedU128};

/// Default is percent
pub const DEFAULT_PRECISION: u128 = 1000;

/// Per mill
pub const DEFAULT_EPSILON: u128 = 1;

/// This function should be used in context of approximation.
/// It is extensively used in conjunction with proptest because of random input generation.
pub fn acceptable_computation_error(
	x: u128,
	y: u128,
	precision: u128,
	epsilon: u128,
) -> Result<(), FixedU128> {
	let delta = i128::abs(x as i128 - y as i128);
	if delta > 1 {
		let lower =
			FixedU128::saturating_from_rational(precision, precision.saturating_add(epsilon));
		let upper =
			FixedU128::saturating_from_rational(precision, precision.saturating_sub(epsilon));
		let q = FixedU128::checked_from_rational(x, y).expect("values too big; qed;");
		if lower <= q && q <= upper {
			Ok(())
		} else {
			Err(q)
		}
	} else {
		Ok(())
	}
}

pub fn default_acceptable_computation_error(x: u128, y: u128) -> Result<(), FixedU128> {
	acceptable_computation_error(x, y, DEFAULT_PRECISION, DEFAULT_EPSILON)
}
