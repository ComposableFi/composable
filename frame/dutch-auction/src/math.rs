//! Price function for auction with price going to minimal possible value.
//! Linear, step-wise exponential, and continuous exponential, others, configured from MakerDao
//! https://github.com/makerdao/dss/blob/master/src/abaci.sol

use composable_support::math::safe::{SafeDiv, SafeMul};
use composable_traits::{
	defi::LiftedFixedBalance,
	time::{DurationSeconds, LinearDecrease, StairstepExponentialDecrease, TimeReleaseFunction},
};
use sp_runtime::{
	traits::{Saturating, Zero},
	ArithmeticError, FixedPointNumber,
};

pub trait AuctionTimeCurveModel {
	/// return current auction price
	fn price(
		&self,
		initial_price: LiftedFixedBalance,
		duration_since_start: DurationSeconds,
	) -> Result<LiftedFixedBalance, ArithmeticError>;
}

impl AuctionTimeCurveModel for TimeReleaseFunction {
	fn price(
		&self,
		initial_price: LiftedFixedBalance,
		duration_since_start: DurationSeconds,
	) -> Result<LiftedFixedBalance, sp_runtime::ArithmeticError> {
		match self {
			TimeReleaseFunction::LinearDecrease(x) => x.price(initial_price, duration_since_start),
			TimeReleaseFunction::StairstepExponentialDecrease(x) =>
				x.price(initial_price, duration_since_start),
		}
	}
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
					.ok_or(ArithmeticError::Underflow)?,
				)?
				.safe_div(
					// see https://github.com/paritytech/substrate/issues/10572
					&LiftedFixedBalance::checked_from_integer(self.total as u128)
						.ok_or(ArithmeticError::Overflow)?,
				)
		}
	}
}

/// returns: initial_price * (cut ^ duration_since_start)
/// if time step is 1, than can optimize away division if needed
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

	use composable_traits::{
		defi::LiftedFixedBalance,
		time::{DurationSeconds, LinearDecrease, StairstepExponentialDecrease, ONE_HOUR},
	};

	use sp_arithmetic::assert_eq_error_rate;
	use sp_runtime::{
		traits::{One, Zero},
		FixedPointNumber, Permill,
	};

	use crate::math::AuctionTimeCurveModel;

	#[test]
	pub fn test_linear_decrease() {
		let calc = LinearDecrease { total: ONE_HOUR };
		let delta = DurationSeconds::default();
		let initial_price = LiftedFixedBalance::saturating_from_integer(1000);
		let price = calc.price(initial_price, delta).unwrap();
		assert_eq!(price, initial_price);
		let delta = delta + 360;
		let price = calc.price(initial_price, delta).unwrap();
		assert_eq!(price, 900.into());
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
			cut: Permill::from_float(2.71_f64.powf(f64::ln(1.0 / 2.0) / half as f64)),
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

	use proptest::{prop_assert, strategy::Strategy, test_runner::TestRunner};

	#[test]
	pub fn proptest_half_each_second_vs_linear() {
		let mut runner = TestRunner::default();

		let time_max = 40; // making it larger makes overflow of comparison pow function, so price still works
		let initial_price = LiftedFixedBalance::saturating_from_integer(1_000_000);
		let calc_linear = LinearDecrease { total: time_max };
		let calc_divide_by_2 =
			StairstepExponentialDecrease { cut: Permill::from_rational(1_u32, 2_u32), step: 1 };

		// bases
		assert_eq!(
			calc_linear.price(initial_price, 1).unwrap(),
			initial_price - initial_price / LiftedFixedBalance::saturating_from_integer(time_max)
		);
		assert_eq!(calc_divide_by_2.price(initial_price, 1).unwrap(), initial_price / 2.into());

		// ends
		assert_eq!(
			calc_divide_by_2.price(initial_price, time_max).unwrap(),
			LiftedFixedBalance::zero()
		);
		assert_eq!(calc_linear.price(initial_price, time_max).unwrap(), LiftedFixedBalance::zero());

		runner
			.run(&(0..time_max).prop_map(|time| (time, time + 1)), |(time, time_next)| {
				let linear_1 = calc_linear.price(initial_price, time).unwrap();
				let linear_2 = calc_linear.price(initial_price, time_next).unwrap();
				prop_assert!(linear_2 < linear_1);
				let exp_1 = calc_divide_by_2.price(initial_price, time).unwrap();
				let exp_2 = calc_divide_by_2.price(initial_price, time_next).unwrap();
				prop_assert!(exp_2 <= exp_1);
				// prom property choses for cut to divide each iteration by 2
				let half_price = initial_price /
					LiftedFixedBalance::saturating_from_integer(2_u64.pow(time as u32));
				prop_assert!(half_price - exp_1 < LiftedFixedBalance::one());
				prop_assert!(LiftedFixedBalance::zero() <= half_price - exp_1);
				// from property of exp moving faster initially and than slowing down
				prop_assert!(exp_1 <= linear_1);

				Ok(())
			})
			.unwrap();
	}
}
