use sp_runtime::{FixedPointNumber, FixedU128};

pub const DEFAULT_ACCEPTABLE_DEVIATION: u128 = 1000;

/// Check that x/y ~ 1 up to a certain precision
pub fn acceptable_computation_error(x: u128, y: u128, precision: u128) -> Result<(), FixedU128> {
	let delta = i128::abs(x as i128 - y as i128);
	if delta > 1 {
		let epsilon: u128 = 1;
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
	acceptable_computation_error(x, y, DEFAULT_ACCEPTABLE_DEVIATION)
}
