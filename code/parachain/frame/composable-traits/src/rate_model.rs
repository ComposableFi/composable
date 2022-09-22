// Copyright 2021 Composable Developer.
// This file is part of Composable Finance.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::ops::Neg;

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::{cmp::Ordering, convert::TryInto};

use sp_runtime::{
	traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, One, Saturating, Zero},
	ArithmeticError, FixedI128, FixedPointNumber, FixedU128, RuntimeDebug,
};

use composable_support::math::safe::{LiftedFixedBalance, SafeArithmetic};
use sp_arithmetic::per_things::Percent;

use crate::loans::{DurationSeconds, ONE_HOUR};

/// The fixed point number from 0..to max.
/// Unlike `Ratio` it can be more than 1.
/// And unlike `NormalizedCollateralFactor`, it can be less than one.
pub type Rate = FixedU128;

/// The fixed point number of suggested by substrate precision
/// Must be (1.0.. because applied only to price normalized values
pub type NormalizedCollateralFactor = FixedU128;

/// Must be [0..1]
/// TODO: implement Ratio as wrapper over FixedU128
pub type Ratio = FixedU128;

/// current notion of year will take away 1/365 from lenders and give away to borrowers (as does no
/// accounts to length of year)
pub const SECONDS_PER_YEAR: DurationSeconds = 365 * 24 * ONE_HOUR;

pub trait InterestRate {
	fn get_borrow_rate(&mut self, utilization: Percent) -> Option<Rate>;
}

/// Parallel interest rate model
#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, TypeInfo)]
pub enum InterestRateModel {
	Jump(JumpModel),
	Curve(CurveModel),
	DynamicPIDController(DynamicPIDControllerModel),
	DoubleExponent(DoubleExponentModel),
}

impl Default for InterestRateModel {
	// unwrap is used with known parameters, and unit tested right below.
	#[allow(clippy::disallowed_methods)]
	fn default() -> Self {
		Self::new_jump_model(
			Rate::saturating_from_rational(2, 100),
			Rate::saturating_from_rational(10, 100),
			Rate::saturating_from_rational(32, 100),
			Percent::from_percent(80),
		)
		.unwrap()
	}
}

#[test]
fn test_interest_rate_model_default() {
	InterestRateModel::default();
}

impl InterestRateModel {
	pub fn new_jump_model(
		base_rate: Rate,
		jump_rate: Rate,
		full_rate: Rate,
		target_utilization: Percent,
	) -> Option<Self> {
		JumpModel::new_model(base_rate, jump_rate, full_rate, target_utilization).map(Self::Jump)
	}

	pub fn new_curve_model(base_rate: Rate) -> Option<Self> {
		CurveModel::new_model(base_rate).map(Self::Curve)
	}

	pub fn new_dynamic_pid_model(
		proportional_parameter: FixedI128,
		integral_parameter: FixedI128,
		derivative_parameter: FixedI128,
		previous_error_value: FixedI128,
		previous_integral_term: FixedI128,
		previous_interest_rate: FixedU128,
		optimal_utilization_ratio: FixedU128,
	) -> Option<Self> {
		DynamicPIDControllerModel::new_model(
			proportional_parameter,
			integral_parameter,
			derivative_parameter,
			previous_error_value,
			previous_integral_term,
			previous_interest_rate,
			optimal_utilization_ratio,
		)
		.map(Self::DynamicPIDController)
	}

	pub fn new_double_exponent_model(coefficients: [u8; 16]) -> Option<Self> {
		DoubleExponentModel::new_model(coefficients).map(Self::DoubleExponent)
	}

	/// Calculates the current supply interest rate
	pub fn get_supply_rate(borrow_rate: Rate, util: Ratio, reserve_factor: Ratio) -> Rate {
		// ((1 - reserve_factor) * borrow_rate) * utilization
		let one_minus_reserve_factor = Ratio::one().saturating_sub(reserve_factor);
		let rate_to_pool = borrow_rate.saturating_mul(one_minus_reserve_factor);

		rate_to_pool.saturating_mul(util)
	}
}

impl InterestRate for InterestRateModel {
	/// Calculates the current borrow interest rate
	fn get_borrow_rate(&mut self, utilization: Percent) -> Option<Rate> {
		match self {
			Self::Jump(jump) => jump.get_borrow_rate(utilization),
			Self::Curve(curve) => curve.get_borrow_rate(utilization),
			Self::DynamicPIDController(dynamic_pid_model) =>
				dynamic_pid_model.get_borrow_rate(utilization),
			Self::DoubleExponent(double_exponents_model) =>
				double_exponents_model.get_borrow_rate(utilization),
		}
	}
}

/// The jump interest rate model
#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, Default, TypeInfo)]
pub struct JumpModel {
	/// The base interest rate when utilization rate is 0
	pub base_rate: Rate,
	/// The interest rate on jump utilization point
	pub jump_rate: Rate,
	/// The max interest rate when utilization rate is 100%
	pub full_rate: Rate,
	/// The utilization point at which the jump_rate is applied
	/// For target_utilization, we should have used sp_runtime::Perquintill, but since Balance is
	/// until can't be created from u128.
	pub target_utilization: Percent,
}

impl JumpModel {
	pub const MAX_BASE_RATE: Ratio = Ratio::from_inner(100_000_000_000_000_000); // 10%
	pub const MAX_JUMP_RATE: Ratio = Ratio::from_inner(300_000_000_000_000_000); // 30%
	pub const MAX_FULL_RATE: Ratio = Ratio::from_inner(500_000_000_000_000_000); // 50%

	/// Create a new rate model
	pub fn new_model(
		base_rate: Ratio,
		jump_rate: Ratio,
		full_rate: Ratio,
		target_utilization: Percent,
	) -> Option<JumpModel> {
		let model = Self { base_rate, jump_rate, full_rate, target_utilization };

		if model.base_rate <= Self::MAX_BASE_RATE &&
			model.jump_rate <= Self::MAX_JUMP_RATE &&
			model.full_rate <= Self::MAX_FULL_RATE &&
			model.base_rate <= model.jump_rate &&
			model.jump_rate <= model.full_rate
		{
			Some(model)
		} else {
			None
		}
	}
}

impl InterestRate for JumpModel {
	/// Calculates the borrow interest rate of jump model
	fn get_borrow_rate(&mut self, utilization: Percent) -> Option<Rate> {
		match utilization.cmp(&self.target_utilization) {
			Ordering::Less => {
				// utilization * (jump_rate - base_rate) / target_utilization + base_rate
				Some(
					self.jump_rate
						.checked_sub(&self.base_rate)?
						.saturating_mul(utilization.into())
						.checked_div(&self.target_utilization.into())?
						.checked_add(&self.base_rate)?,
				)
			},
			Ordering::Equal => Some(self.jump_rate),
			Ordering::Greater => {
				//  (utilization - target_utilization)*(full_rate - jump_rate) / ( 1 -
				// target_utilization) + jump_rate
				let excess_utilization = utilization.saturating_sub(self.target_utilization);
				let available = Percent::one().saturating_sub(self.target_utilization);
				Some(
					self.full_rate
						.checked_sub(&self.jump_rate)?
						.saturating_mul(excess_utilization.into())
						.checked_div(&available.into())?
						.checked_add(&self.jump_rate)?,
				)
			},
		}
	}
}

/// The curve interest rate model
#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, Default, TypeInfo)]
pub struct CurveModel {
	base_rate: Rate,
}

impl CurveModel {
	pub const MAX_BASE_RATE: Rate = Rate::from_inner(Rate::DIV / 100 * 10); // 10%

	/// Create a new curve model
	pub fn new_model(base_rate: Rate) -> Option<CurveModel> {
		let model = Self { base_rate };
		if model.base_rate <= Self::MAX_BASE_RATE {
			Some(model)
		} else {
			None
		}
	}
}

impl InterestRate for CurveModel {
	/// Calculates the borrow interest rate of curve model
	fn get_borrow_rate(&mut self, utilization: Percent) -> Option<Rate> {
		const NINE: usize = 9;
		let utilization: Rate = utilization.saturating_pow(NINE).into();
		utilization.checked_add(&self.base_rate)
	}
}

/// The dynamic interest rate curve based on control theory
/// https://www.delphidigital.io/reports/dynamic-interest-rate-model-based-on-control-theory/
/// PID Controller (proportional-integral-derivative controller)
/// Error term is calculated as `et = uo - ut`.
/// Proportional term is calculated as `pt = kp * et`.
/// Integral term is calculated as `it = it_1 + ki * et`, here `it_1` is previous_integral_term.
/// Derivative term is calculated as `dt = kd * (et - et_1)`. here `et_1` is previous_error_value.
/// Control value is calculated as `ut = pt + it + dt`.
/// New Interest rate is calculated as `ir = ir_t_1 + ut` here ir_t_1 is previous_interest_rate.
///
/// To know how `kp`, `ki` and `kd` are derived please check paper at above URL.
#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, Default, TypeInfo)]
pub struct DynamicPIDControllerModel {
	/// proportional_parameter
	kp: FixedI128,
	/// integral_parameter
	ki: FixedI128,
	/// derivative_parameter
	kd: FixedI128,
	/// previous error value
	et_1: FixedI128,
	/// previous integral term
	it_1: FixedI128,
	/// previous interest rate
	ir_t_1: FixedU128,
	/// optimal utilization_ratio
	uo: FixedU128,
}

impl DynamicPIDControllerModel {
	pub fn get_output_utilization_ratio(
		&mut self,
		utilization_ratio: FixedU128,
	) -> Result<Rate, ArithmeticError> {
		// compute error term `et = uo - ut`
		let et: i128 = self.uo.into_inner().try_into().unwrap_or(0_i128) -
			utilization_ratio.into_inner().try_into().unwrap_or(0_i128);
		let et: FixedI128 = FixedI128::from_inner(et);
		// compute proportional term `pt = kp * et`
		let pt = self.kp.checked_mul(&et).ok_or(ArithmeticError::Overflow)?;
		//compute integral term `it = it_1 + ki * et`
		let it = self
			.it_1
			.checked_add(&self.ki.checked_mul(&et).ok_or(ArithmeticError::Overflow)?)
			.ok_or(ArithmeticError::Overflow)?;
		self.it_1 = it;
		// compute derivative term `dt = kd * (et - et_1)`
		let dt = self.kd.checked_mul(&(et - self.et_1)).ok_or(ArithmeticError::Overflow)?;
		self.et_1 = et;

		// compute u(t), control value `ut = pt + it + dt`
		let ut = pt + it + dt;
		// update interest_rate `ir = ir_t_1 + ut`
		if ut.is_negative() {
			let ut = ut.neg();
			self.ir_t_1 = self.ir_t_1.saturating_sub(FixedU128::from_inner(
				ut.into_inner().try_into().unwrap_or(0_u128),
			));
		} else {
			self.ir_t_1 = self.ir_t_1.saturating_add(FixedU128::from_inner(
				ut.into_inner().try_into().unwrap_or(0_u128),
			));
		}
		Ok(self.ir_t_1)
	}

	pub fn new_model(
		proportional_parameter: FixedI128,
		integral_parameter: FixedI128,
		derivative_parameter: FixedI128,
		previous_error_value: FixedI128,
		previous_integral_term: FixedI128,
		previous_interest_rate: FixedU128,
		optimal_utilization_ratio: FixedU128,
	) -> Option<DynamicPIDControllerModel> {
		Some(DynamicPIDControllerModel {
			kp: proportional_parameter,
			ki: integral_parameter,
			kd: derivative_parameter,
			et_1: previous_error_value,
			it_1: previous_integral_term,
			ir_t_1: previous_interest_rate,
			uo: optimal_utilization_ratio,
		})
	}
}

impl InterestRate for DynamicPIDControllerModel {
	fn get_borrow_rate(&mut self, utilization: Percent) -> Option<Rate> {
		// const NINE: usize = 9;
		let utilization: Rate = utilization.into();
		if let Ok(interest_rate) = Self::get_output_utilization_ratio(self, utilization) {
			return Some(interest_rate)
		}
		None
	}
}

const EXPECTED_COEFFICIENTS_SUM: u16 = 100;

/// The double exponent interest rate model
/// Interest based on a polynomial of the utilization of the market.
/// Interest = C_0 + C_1 * U^(2^0) + C_2 * U^(2^1) + C_3 * U^(2^2) ...
/// C_0, C_1, C_2, ... coefficients are passed as packed u128.
/// Each coefficient is of 8 byte. C_0 is at LSB and C_15 at MSB.
/// For reference check https://github.com/dydxprotocol/solo/blob/master/contracts/external/interestsetters/DoubleExponentInterestSetter.sol
/// https://help.dydx.exchange/en/articles/2924246-how-do-interest-rates-work
#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, Default, TypeInfo)]
pub struct DoubleExponentModel {
	coefficients: [u8; 16],
}

impl DoubleExponentModel {
	/// Create a double exponent model
	pub fn new_model(coefficients: [u8; 16]) -> Option<Self> {
		let sum_of_coefficients = coefficients.iter().fold(0_u16, |acc, &c| acc + c as u16);
		if sum_of_coefficients == EXPECTED_COEFFICIENTS_SUM {
			return Some(DoubleExponentModel { coefficients })
		}
		None
	}
}

impl InterestRate for DoubleExponentModel {
	fn get_borrow_rate(&mut self, utilization: Percent) -> Option<Rate> {
		let polynomial_0: FixedU128 = FixedU128::one();
		let result_0: Rate = Rate::zero();
		let res =
			self.coefficients
				.iter()
				.try_fold((result_0, polynomial_0), |(res, poly), coeff| {
					let coefficient = FixedU128::from_inner(u128::from(*coeff));
					let result = res.checked_add(&coefficient.checked_mul(&poly)?)?;
					let polynomial = poly.checked_mul(&utilization.into())?;
					Some((result, polynomial))
				});
		res.map(|(r, _p)| r.checked_div(&FixedU128::from_inner(EXPECTED_COEFFICIENTS_SUM.into())))
			.flatten()
	}
}

pub fn accrued_interest(
	borrow_rate: Rate,
	amount: u128,
	delta_time: DurationSeconds,
) -> Option<u128> {
	borrow_rate
		.checked_mul_int(amount)?
		.checked_mul(delta_time.into())?
		.checked_div(SECONDS_PER_YEAR.into())
}

pub fn increment_index(
	borrow_rate: Rate,
	index: Rate,
	delta_time: DurationSeconds,
) -> Result<Rate, ArithmeticError> {
	borrow_rate
		.safe_mul(&index)?
		.safe_mul(&FixedU128::saturating_from_integer(delta_time))?
		.safe_div(&FixedU128::saturating_from_integer(SECONDS_PER_YEAR))
}

pub fn increment_borrow_rate(
	borrow_rate: Rate,
	delta_time: DurationSeconds,
) -> Result<Rate, ArithmeticError> {
	borrow_rate
		.safe_mul(&FixedU128::saturating_from_integer(delta_time))?
		.safe_div(&FixedU128::saturating_from_integer(SECONDS_PER_YEAR))
}

#[cfg(test)]
mod tests {
	use super::*;
	use proptest::{prop_assert, strategy::Strategy, test_runner::TestRunner};
	use sp_runtime::FixedU128;

	// Test jump model
	#[test]
	fn init_jump_model_works() {
		let base_rate = Rate::saturating_from_rational(2, 100);
		let jump_rate = Rate::saturating_from_rational(10, 100);
		let full_rate = Rate::saturating_from_rational(32, 100);
		let target_utilization = Percent::from_percent(80);
		assert_eq!(
			JumpModel::new_model(base_rate, jump_rate, full_rate, target_utilization).unwrap(),
			JumpModel {
				base_rate: Rate::from_inner(20_000_000_000_000_000),
				jump_rate: Rate::from_inner(100_000_000_000_000_000),
				full_rate: Rate::from_inner(320_000_000_000_000_000),
				target_utilization: Percent::from_percent(80),
			}
		);
	}

	#[test]
	fn get_borrow_rate_works() {
		// init
		let base_rate = Rate::saturating_from_rational(2, 100);
		let jump_rate = Rate::saturating_from_rational(10, 100);
		let full_rate = Rate::saturating_from_rational(32, 100);
		let target_utilization = Percent::from_percent(80);
		let mut jump_model =
			JumpModel::new_model(base_rate, jump_rate, full_rate, target_utilization).unwrap();
		// normal rate
		let mut cash: u128 = 500;
		let borrows: u128 = 1000;
		let utilization = Percent::from_rational(borrows, cash + borrows);
		let borrow_rate = jump_model.get_borrow_rate(utilization).unwrap();
		assert_eq!(
			borrow_rate,
			jump_model.jump_rate.saturating_mul(utilization.into()) + jump_model.base_rate,
		);

		// jump rate
		cash = 100;
		let utilization = Percent::from_rational(borrows, cash + borrows);
		let borrow_rate = jump_model.get_borrow_rate(utilization).unwrap();
		let normal_rate =
			jump_model.jump_rate.saturating_mul(target_utilization.into()) + jump_model.base_rate;
		let excess_util = utilization.saturating_sub(target_utilization);
		assert_eq!(
			borrow_rate,
			(jump_model.full_rate - jump_model.jump_rate).saturating_mul(excess_util.into()) /
				FixedU128::saturating_from_rational(20, 100) +
				normal_rate,
		);
	}

	// Test curve model
	// TODO: Add test cases for curve model

	#[test]
	fn get_supply_rate_works() {
		let borrow_rate = Rate::saturating_from_rational(2, 100);
		let util = Ratio::saturating_from_rational(50, 100);
		let reserve_factor = Ratio::zero();
		let supply_rate = InterestRateModel::get_supply_rate(borrow_rate, util, reserve_factor);
		assert_eq!(
			supply_rate,
			borrow_rate.saturating_mul(Ratio::one().saturating_sub(reserve_factor) * util),
		);
	}

	#[test]
	fn curve_model_correctly_calculates_borrow_rate() {
		let mut model = CurveModel::new_model(Rate::saturating_from_rational(2, 100)).unwrap();
		assert_eq!(
			model.get_borrow_rate(Percent::from_percent(80)).unwrap(),
			// curve model has arbitrary power parameters leading to changes in precision of high
			// power
			Rate::from_inner(140000000000000000)
		);
	}

	#[derive(Debug, Clone)]
	struct JumpModelStrategy {
		pub base_rate: Ratio,
		pub jump_percentage: Ratio,
		pub full_percentage: Ratio,
		pub target_utilization: Percent,
	}

	fn valid_jump_model() -> impl Strategy<Value = JumpModelStrategy> {
		(
			(1..=10_u32).prop_map(|x| Ratio::saturating_from_rational(x, 100)),
			(11..=30_u32).prop_map(|x| Ratio::saturating_from_rational(x, 100)),
			(31..=50).prop_map(|x| Ratio::saturating_from_rational(x, 100)),
			(0..=100_u8).prop_map(Percent::from_percent),
		)
			.prop_filter("Jump rate model", |(base, jump, full, _)| {
				// tried high order strategy - failed as it tries to combine collections with not
				// collection alternative to define arbitrary and proptest attributes with filtering
				// overall cardinality is small, so should work well
				// here we have one liner, not sure why in code we have many lines....
				base <= jump &&
					jump <= full && base <= &JumpModel::MAX_BASE_RATE &&
					jump <= &JumpModel::MAX_JUMP_RATE &&
					full <= &JumpModel::MAX_FULL_RATE
			})
			.prop_map(|(base_rate, jump_percentage, full_percentage, target_utilization)| {
				JumpModelStrategy {
					base_rate,
					full_percentage,
					jump_percentage,
					target_utilization,
				}
			})
	}

	#[test]
	fn test_empty_drained_market() {
		let mut jump_model = JumpModel::new_model(
			FixedU128::from_float(0.01),
			FixedU128::from_float(0.11),
			FixedU128::from_float(0.31),
			Percent::zero(),
		)
		.unwrap();
		let borrow_rate = jump_model
			.get_borrow_rate(Percent::zero())
			.expect("borrow rate must be defined");

		assert_eq!(borrow_rate, jump_model.jump_rate);
	}

	#[test]
	fn test_slope() {
		let mut jump_model = JumpModel::new_model(
			FixedU128::from_float(0.01),
			FixedU128::from_float(0.11),
			FixedU128::from_float(0.31),
			Percent::from_percent(80),
		)
		.unwrap();

		let x1 = 70;
		let x2 = 75;
		let y1 = jump_model.get_borrow_rate(Percent::from_percent(x1)).unwrap();
		let y2 = jump_model.get_borrow_rate(Percent::from_percent(x2)).unwrap();
		let s1 = (y2 - y1) /
			(FixedU128::saturating_from_integer(x2) - FixedU128::saturating_from_integer(x1));

		let x1 = 81;
		let x2 = 86;
		let y1 = jump_model.get_borrow_rate(Percent::from_percent(x1)).unwrap();
		let y2 = jump_model.get_borrow_rate(Percent::from_percent(x2)).unwrap();
		let s2 = (y2 - y1) /
			(FixedU128::saturating_from_integer(x2) - FixedU128::saturating_from_integer(x1));

		assert!(s1 < s2, "slope after target is growing faster")
	}

	#[test]
	fn proptest_jump_model() {
		let mut runner = TestRunner::default();
		runner
			.run(&(valid_jump_model(), 0..=100_u8), |(strategy, utilization)| {
				let base_rate = strategy.base_rate;
				let jump_rate = strategy.jump_percentage;
				let full_rate = strategy.full_percentage;
				let target_utilization = strategy.target_utilization;
				let mut jump_model =
					JumpModel::new_model(base_rate, jump_rate, full_rate, target_utilization)
						.unwrap();

				let utilization = Percent::from_percent(utilization);
				let borrow_rate =
					jump_model.get_borrow_rate(utilization).expect("borrow rate must be defined");
				prop_assert!(borrow_rate > Rate::zero());
				Ok(())
			})
			.unwrap();
	}

	#[test]
	fn proptest_jump_model_rate() {
		let base_rate = Rate::saturating_from_rational(2, 100);
		let jump_rate = Rate::saturating_from_rational(10, 100);
		let full_rate = Rate::saturating_from_rational(32, 100);
		let strategy = (0..=100_u8, 1..=99_u8)
			.prop_map(|(optimal, utilization)| (optimal, utilization, utilization + 1));

		let mut runner = TestRunner::default();
		runner
			.run(&strategy, |(optimal, previous, next)| {
				let utilization_1 = Percent::from_percent(previous);
				let utilization_2 = Percent::from_percent(next);
				let optimal = Percent::from_percent(optimal);
				let mut model = JumpModel::new_model(base_rate, jump_rate, full_rate, optimal)
					.expect("model should be defined");
				let rate_1 = model.get_borrow_rate(utilization_1);
				let rate_2 = model.get_borrow_rate(utilization_2);
				if optimal < Percent::from_percent(100) {
					prop_assert!(rate_1 < rate_2);
				}
				Ok(())
			})
			.unwrap();
	}

	#[cfg(feature = "visualization")]
	fn jump_model_plotter() {
		use plotters::prelude::*;
		let base_rate = Rate::saturating_from_rational(2, 100);
		let jump_rate = Rate::saturating_from_rational(10, 100);
		let full_rate = Rate::saturating_from_rational(32, 100);
		let optimal = Percent::from_percent(80);
		let mut model = JumpModel::new_model(base_rate, jump_rate, full_rate, optimal).unwrap();

		let area = BitMapBackend::new("./jump_model.png", (1024, 768)).into_drawing_area();
		area.fill(&WHITE).unwrap();

		let mut chart = ChartBuilder::on(&area)
			.set_label_area_size(LabelAreaPosition::Left, 40)
			.set_label_area_size(LabelAreaPosition::Bottom, 40)
			.build_cartesian_2d(0.0..100.0, 0.0..100.0)
			.unwrap();
		chart.configure_mesh().draw().unwrap();
		chart
			.draw_series(LineSeries::new(
				(0..=100).map(|x| {
					let utilization = Percent::from_percent(x);
					let rate = model.get_borrow_rate(utilization).unwrap();
					(x as f64, rate.to_float() * 100.0)
				}),
				&RED,
			))
			.unwrap();
	}
	#[cfg(feature = "visualization")]
	fn dynamic_pid_model_plotter() {
		use plotters::prelude::*;
		let kp = FixedI128::saturating_from_rational(600, 100);
		let ki = FixedI128::saturating_from_rational(200, 100);
		let kd = FixedI128::saturating_from_rational(1275, 100);
		let et_1 = FixedI128::from_inner(0i128);
		let it_1 = FixedI128::from_inner(0i128);
		let ir_t_1 = FixedU128::saturating_from_rational(500, 100);
		let uo = FixedU128::saturating_from_rational(80, 100);
		let mut model =
			DynamicPIDControllerModel::new_model(kp, ki, kd, et_1, it_1, ir_t_1, uo).unwrap();

		let area = BitMapBackend::new("./dynamic_pid_model.png", (1024, 768)).into_drawing_area();
		area.fill(&WHITE).unwrap();

		let mut chart = ChartBuilder::on(&area)
			.set_label_area_size(LabelAreaPosition::Left, 40)
			.set_label_area_size(LabelAreaPosition::Bottom, 40)
			.build_cartesian_2d(0.0..200.0, 0.0..5000.0)
			.unwrap();
		chart.configure_mesh().draw().unwrap();
		chart
			.draw_series(LineSeries::new(
				[
					50, 55, 51, 57, 60, 66, 66, 66, 66, 77, 78, 50, 78, 88, 88, 90, 78, 79, 74, 74,
					80, 80, 62, 59, 58, 59, 58, 60, 61, 62, 62, 62, 63, 80, 85, 99, 80, 81, 82, 60,
					60, 40, 30, 31, 32, 40, 50, 51, 51, 40, 50, 60, 66, 69, 60, 80, 70, 70, 77, 70,
					60, 56, 52, 50, 45, 44, 40, 30, 10, 30, 40, 50, 60, 70, 71, 71, 71, 70, 80, 80,
					90, 91, 90, 91, 90, 91, 90, 91, 90, 91, 90, 91, 90, 91, 92, 90, 90, 90, 90, 90,
					90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 80, 80, 70, 71, 70, 71, 70,
					71, 70, 71, 70, 68, 67, 66, 65, 64, 63, 62, 61, 50, 50, 40, 30,
				]
				.iter()
				.enumerate()
				.map(|(i, x)| {
					let utilization = Percent::from_percent(*x);
					let rate = model.get_borrow_rate(utilization).unwrap();
					(i as f64, rate.to_float() * 100.0)
				}),
				&RED,
			))
			.unwrap();
	}

	#[cfg(feature = "visualization")]
	fn double_exponents_model_plotter() {
		use plotters::prelude::*;
		let coefficients: [u8; 16] = [60, 20, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
		let mut model = DoubleExponentModel::new_model(coefficients).unwrap();
		let area =
			BitMapBackend::new("./double_exponents_model.png", (1024, 768)).into_drawing_area();
		area.fill(&WHITE).unwrap();

		let mut chart = ChartBuilder::on(&area)
			.set_label_area_size(LabelAreaPosition::Left, 40)
			.set_label_area_size(LabelAreaPosition::Bottom, 40)
			.build_cartesian_2d(0.0..100.0, 0.0..100.0)
			.unwrap();
		chart.configure_mesh().draw().unwrap();
		chart
			.draw_series(LineSeries::new(
				(60..=100).map(|x| {
					let utilization = Percent::from_percent(x);
					let rate = model.get_borrow_rate(utilization).unwrap();
					(x as f64, rate.to_float() * 100.0)
				}),
				&RED,
			))
			.unwrap();
	}
}
