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
	use sp_runtime::{ArithmeticError, FixedPointNumber, FixedPointOperand, FixedU128, Percent, Permill, Perquintill, offchain::Duration, traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, One,
			Saturating, Zero,
		}};

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
		// it will take 10 steps to half the price
		// sum  10 times the `(e^ln(1/2)/10)` = will equal `e^ln(1/2)` = `1/2`
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

	use  proptest::{self::*, strategy::Strategy, test_runner::TestRunner};

	#[test]
	pub fn proptest_half_each_second_vs_linear() {
		let mut runner = TestRunner::default();

		let time = 1000;
		let initial_price = LiftedFixedBalance::saturating_from_integer(1_000_000);
		let calc_linear = LinearDecrease { total:time};
		let calc_divide_by_2 = StairstepExponentialDecrease {
			cut: Permill::from_rational(1, 2),
			step: 1,
		};

		runner.run((0..1000u32).prop_map(|time|(time, time + 1)), |(time, time_next)| {

			let linear_1 = calc_linear.price(initial_price, time).unwrap();
			let linear_2 = calc_linear.price(initial_price, time_next).unwrap();
			prop_assert!(linear_2 <= linear_1);
			let exp_1 = calc_divide_by_2.price(initial_price, time).unwrap();
			let exp_2 = calc_divide_by_2.price(initial_price, time_next).unwrap();
			prop_assert!(exp_2 <= exp_1);
			// prom property choses for cut to divide each iteration by 2
			let mut half_price = initial_price / LiftedFixedBalance::from(2^time);
			prop_assert_eq!(exp1, half_price);
			// from proprety of exp moving fastr initially and than slowing down
			prop_assert!(exp_2 <= linear_1);

		}).unwrap();
	}
}
