//! Price function for auction with price going to minimal possible value.
//! Linear, step-wise exponential, and continuous exponential, others, configured from MakerDao
//! https://github.com/makerdao/dss/blob/master/src/abaci.sol

use composable_traits::{
	auction::{LinearDecrease, StairstepExponentialDecrease},
	loans::DurationSeconds,
	math::{LiftedFixedBalance, SafeArithmetic},
};

use sp_runtime::{
	traits::{
		AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, One,
		Saturating, Zero,
	},
	ArithmeticError, FixedPointNumber, FixedPointOperand, FixedU128, Percent, Permill, Perquintill,
};

pub trait AuctionTimeCurveModel {
	/// return current auction price
	fn price(
		&self,
		initial_price: LiftedFixedBalance,
		duration_since_start: DurationSeconds,
	) -> Result<LiftedFixedBalance, ArithmeticError>;
}

/// Price calculation when price is decreased linearly in proportion to time:
/// Returns y = initial_price * ((tau - duration_since_start) / tau)
impl AuctionTimeCurveModel for LinearDecrease {
	fn price(
		&self,
		initial_price: LiftedFixedBalance,
		duration_since_start: DurationSeconds,
	) -> Result<LiftedFixedBalance, ArithmeticError> {
		if duration_since_start >= self.total {
			Ok(LiftedFixedBalance::zero())
		} else {
			// here we violate unit of measure to have best math
			initial_price
				.safe_mul(
					&LiftedFixedBalance::checked_from_integer(
						self.total.saturating_sub(duration_since_start) as u128,
					)
					.unwrap(),
				)?
				.safe_div(&LiftedFixedBalance::checked_from_integer(self.total as u128).unwrap())
		}
	}
}

/// returns: top * (cut ^ dur)
impl AuctionTimeCurveModel for StairstepExponentialDecrease {
	fn price(
		&self,
		initial_price: LiftedFixedBalance,
		duration_since_start: DurationSeconds,
	) -> Result<LiftedFixedBalance, ArithmeticError> {
		let multiplier =
			self.cut.saturating_pow(duration_since_start.safe_div(&self.step)? as usize);
		initial_price.safe_mul(&multiplier.into())
	}
}

#[cfg(test)]
mod tests {
	use std::convert::TryInto;

	use composable_traits::{
		auction::{LinearDecrease, StairstepExponentialDecrease},
		loans::{DurationSeconds, ONE_HOUR},
		math::LiftedFixedBalance,
	};

	use sp_arithmetic::assert_eq_error_rate;
	use sp_runtime::{
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, One,
			Saturating, Zero,
		},
		ArithmeticError, FixedPointNumber, FixedPointOperand, FixedU128, Percent, Permill,
		Perquintill,
	};

	use crate::price_function::AuctionTimeCurveModel;

	#[test]
	pub fn test_linear_decrease() {
		let calc = LinearDecrease { total: ONE_HOUR };
		let delta = DurationSeconds::default();
		let initial_price = LiftedFixedBalance::saturating_from_integer(1000);
		let price = calc.price(initial_price, delta).unwrap();
		assert_eq!(price, initial_price);
		let delta = delta + 360;
		let price = calc.price(initial_price, delta).unwrap();
		assert_eq!(price, (1000 - 100 * 1).into());
		let delta = delta + 360 * 8;
		let price = calc.price(initial_price, delta).unwrap();
		assert_eq!(price, (1000 - 100 * 9).into());
		let delta = delta + 360;
		let price = calc.price(initial_price, delta).unwrap();
		assert_eq!(price, 0.into());
	}

	#[test]
	pub fn test_continuous_exp_decrease() {
		// it will take 100 steps to half the price (sum 100 e^ln(1/2)/100) = e^ln(1/2) = 1/2
		let half = 10;

		let calc = StairstepExponentialDecrease {
			cut: Permill::from_float(2.71f64.powf(f64::ln(1.0 / 2.0) / half as f64)),
			step: 1,
		};

		let initial_price = 4000.0;
		let mut expected_price = initial_price;

		for i in 0..=5 {
			let price: f64 = calc
				.price(LiftedFixedBalance::from_float(initial_price), i * half)
				.unwrap()
				.to_float();
			// Permill seems not good enough for long auction with many steps, but we are fast few
			// step
			assert_eq_error_rate!(price, expected_price, initial_price / 500.0);
			expected_price /= 2.0;
		}
	}

	#[test]
	pub fn proptest() {
		// it decreases, so x < x - 1
		// exponteica decreases

	}
}
