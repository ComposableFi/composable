use codec::{Decode, Encode};
use composable_support::{
	math::safe::{SafeAdd, SafeDiv, SafeMul},
	validation::Validate,
};
use scale_info::TypeInfo;
use sp_std::{cmp::Ordering, convert::TryInto};

use sp_runtime::{
	traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, One, Saturating, Zero},
	ArithmeticError, FixedI128, FixedPointNumber, FixedU128, RuntimeDebug,
};

use sp_arithmetic::per_things::Percent;

use crate::{
	defi::{LiftedFixedBalance, Rate, ZeroToOneFixedU128},
	time::{DurationSeconds, SECONDS_PER_YEAR_NAIVE},
};

/// utilization_ratio = total_borrows / (total_cash + total_borrows)
pub fn calculate_utilization_ratio(
	cash: LiftedFixedBalance,
	borrows: LiftedFixedBalance,
) -> Result<Percent, ArithmeticError> {
	if borrows.is_zero() {
		return Ok(Percent::zero())
	}

	let total = cash.safe_add(&borrows)?;

	Ok(Percent::from_rational(borrows.into_inner(), total.into_inner()))
}

pub trait InterestRate {
	// Mutable because of [`DynamicPIDControllerModel::get_output_utilization_ratio`].
	fn get_borrow_rate(&mut self, utilization: Percent) -> Option<Rate>;
}

// TODO: all these are MayBeModels after SCALE decode, need to map to Models after validation
/// Interest rate models
#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, TypeInfo)]
pub enum InterestRateModel {
	Jump(JumpModel),
	Curve(CurveModel),
	DynamicPIDController(DynamicPIDControllerModel),
	DoubleExponent(DoubleExponentModel),
}

impl Default for InterestRateModel {
	fn default() -> Self {
		Self::new_jump_model(
			Rate::saturating_from_rational(2, 100),
			Rate::saturating_from_rational(10, 100),
			Rate::saturating_from_rational(32, 100),
			Percent::from_percent(80),
		)
		.expect("default model is valid")
	}
}

impl InterestRateModel {
	pub fn new_jump_model(
		base_rate: Rate,
		jump_rate: Rate,
		full_rate: Rate,
		target_utilization: Percent,
	) -> Option<Self> {
		JumpModel::new(base_rate, jump_rate, full_rate, target_utilization).map(Self::Jump)
	}

	pub fn new_curve_model(base_rate: Rate) -> Option<Self> {
		CurveModel::new(base_rate).map(Self::Curve)
	}

	pub fn new_dynamic_pid_model(
		proportional_parameter: FixedI128,
		integral_parameter: FixedI128,
		derivative_parameter: FixedI128,
		initial_interest_rate: FixedU128,
		target_utilization: FixedU128,
	) -> Option<Self> {
		DynamicPIDControllerModel::new(
			proportional_parameter,
			integral_parameter,
			derivative_parameter,
			initial_interest_rate,
			target_utilization,
		)
		.map(Self::DynamicPIDController)
	}

	pub fn new_double_exponent_model(coefficients: [u8; 16]) -> Option<Self> {
		DoubleExponentModel::new(coefficients).map(Self::DoubleExponent)
	}

	/// Calculates the current supply interest rate
	pub fn get_supply_rate(
		borrow_rate: Rate,
		util: ZeroToOneFixedU128,
		reserve_factor: ZeroToOneFixedU128,
	) -> Rate {
		// ((1 - reserve_factor) * borrow_rate) * utilization
		let one_minus_reserve_factor = ZeroToOneFixedU128::one().saturating_sub(reserve_factor);
		let rate_to_pool = borrow_rate.saturating_mul(one_minus_reserve_factor);

		rate_to_pool.saturating_mul(util)
	}
}

pub struct InterestRateModelIsValid;
impl Validate<InterestRateModel, InterestRateModelIsValid> for InterestRateModelIsValid {
	fn validate(interest_rate_model: InterestRateModel) -> Result<InterestRateModel, &'static str> {
		const ERROR: &str = "interest rate model is not valid";
		match interest_rate_model {
			InterestRateModel::Jump(x) =>
				JumpModel::new(x.base_rate, x.jump_rate, x.full_rate, x.target_utilization)
					.ok_or(ERROR)
					.map(InterestRateModel::Jump),
			InterestRateModel::Curve(x) =>
				CurveModel::new(x.base_rate).ok_or(ERROR).map(InterestRateModel::Curve),
			InterestRateModel::DynamicPIDController(x) => DynamicPIDControllerModel::new(
				x.proportional_parameter,
				x.integral_parameter,
				x.derivative_parameter,
				x.previous_interest_rate,
				x.target_utilization,
			)
			.ok_or(ERROR)
			.map(InterestRateModel::DynamicPIDController),
			InterestRateModel::DoubleExponent(x) => DoubleExponentModel::new(x.coefficients)
				.ok_or(ERROR)
				.map(InterestRateModel::DoubleExponent),
		}
	}
}

// TODO: Use enum_dispatch crate
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
	pub const MAX_BASE_RATE: ZeroToOneFixedU128 =
		ZeroToOneFixedU128::from_inner(ZeroToOneFixedU128::DIV * 10 / 100);
	pub const MAX_JUMP_RATE: ZeroToOneFixedU128 =
		ZeroToOneFixedU128::from_inner(ZeroToOneFixedU128::DIV * 30 / 100);
	pub const MAX_FULL_RATE: ZeroToOneFixedU128 =
		ZeroToOneFixedU128::from_inner(ZeroToOneFixedU128::DIV * 50 / 100);

	/// Create a new rate model
	pub fn new(
		base_rate: ZeroToOneFixedU128,
		jump_rate: ZeroToOneFixedU128,
		full_rate: ZeroToOneFixedU128,
		target_utilization: Percent,
	) -> Option<JumpModel> {
		if base_rate <= Self::MAX_BASE_RATE &&
			jump_rate <= Self::MAX_JUMP_RATE &&
			full_rate <= Self::MAX_FULL_RATE &&
			base_rate <= jump_rate &&
			jump_rate <= full_rate
		{
			let model = Self { base_rate, jump_rate, full_rate, target_utilization };
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
	pub fn new(base_rate: Rate) -> Option<CurveModel> {
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
	/// `kp`
	proportional_parameter: FixedI128,
	/// `ki`
	integral_parameter: FixedI128,
	/// `kd`
	derivative_parameter: FixedI128,
	/// `et_1`
	previous_error_value: FixedI128,
	/// `it_1`
	previous_integral_term: FixedI128,
	/// `ir_t_1`
	previous_interest_rate: FixedU128,
	/// `uo`
	target_utilization: FixedU128,
}

impl DynamicPIDControllerModel {
	pub fn get_output_utilization_ratio(
		&mut self,
		utilization_ratio: FixedU128,
	) -> Result<Rate, ArithmeticError> {
		// compute error term `et = uo - ut`
		let et: i128 = self.target_utilization.into_inner().try_into().unwrap_or(0_i128) -
			utilization_ratio.into_inner().try_into().unwrap_or(0_i128);
		let et: FixedI128 = FixedI128::from_inner(et);
		// compute proportional term `pt = kp * et`
		let pt = self.proportional_parameter.checked_mul(&et).ok_or(ArithmeticError::Overflow)?;
		//compute integral term `it = it_1 + ki * et`
		let it = self
			.previous_integral_term
			.checked_add(
				&self.integral_parameter.checked_mul(&et).ok_or(ArithmeticError::Overflow)?,
			)
			.ok_or(ArithmeticError::Overflow)?;
		self.previous_integral_term = it;
		// compute derivative term `dt = kd * (et - et_1)`
		let dt = self
			.derivative_parameter
			.checked_mul(&(et - self.previous_error_value))
			.ok_or(ArithmeticError::Overflow)?;
		self.previous_error_value = et;

		// compute u(t), control value `ut = pt + it + dt`
		let ut = pt + it + dt;
		// update interest_rate `ir = ir_t_1 + ut`
		if ut.is_negative() {
			let ut = ut.neg();
			self.previous_interest_rate = self.previous_interest_rate.saturating_sub(
				FixedU128::from_inner(ut.into_inner().try_into().unwrap_or(0_u128)),
			);
		} else {
			self.previous_interest_rate = self.previous_interest_rate.saturating_add(
				FixedU128::from_inner(ut.into_inner().try_into().unwrap_or(0_u128)),
			);
		}

		Ok(self.previous_interest_rate)
	}

	pub fn new(
		proportional_parameter: FixedI128,
		integral_parameter: FixedI128,
		derivative_parameter: FixedI128,
		initial_interest_rate: FixedU128,
		target_utilization: ZeroToOneFixedU128,
	) -> Option<DynamicPIDControllerModel> {
		if target_utilization > ZeroToOneFixedU128::one() {
			None
		} else {
			Some(DynamicPIDControllerModel {
				proportional_parameter,
				integral_parameter,
				derivative_parameter,
				previous_error_value: <_>::zero(),
				previous_integral_term: <_>::zero(),
				previous_interest_rate: initial_interest_rate,
				target_utilization,
			})
		}
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
/// Interest = C_0 + C_1 * U^(2^2) + C_2 * U^(2^4) + C_3 * U^(2^8) ...
/// For reference check https://github.com/dydxprotocol/solo/blob/master/contracts/external/interestsetters/DoubleExponentInterestSetter.sol
/// https://web.archive.org/web/20210518033618/https://help.dydx.exchange/en/articles/2924246-how-do-interest-rates-work
#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, Default, TypeInfo)]
pub struct DoubleExponentModel {
	coefficients: [u8; 16],
}

impl DoubleExponentModel {
	/// Create a double exponent model
	pub fn new(coefficients: [u8; 16]) -> Option<Self> {
		let sum_of_coefficients = coefficients.iter().fold(0_u16, |acc, &c| acc + c as u16);
		if sum_of_coefficients == EXPECTED_COEFFICIENTS_SUM {
			return Some(DoubleExponentModel { coefficients })
		}
		None
	}
}

impl InterestRate for DoubleExponentModel {
	fn get_borrow_rate(&mut self, utilization: Percent) -> Option<Rate> {
		let polynomial: FixedU128 = utilization.into();
		let (result, _) = self.coefficients.iter().skip(1).fold(
			(FixedU128::saturating_from_integer(self.coefficients[0]), polynomial),
			|(rate, polynomial), element| {
				let polynomial = polynomial * polynomial;
				let rate = rate + FixedU128::saturating_from_integer(*element) * polynomial;
				(rate, polynomial)
			},
		);
		let maximal = FixedU128::saturating_from_integer(EXPECTED_COEFFICIENTS_SUM);
		Some(result / maximal)
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
		.checked_div(SECONDS_PER_YEAR_NAIVE.into())
}

/// compounding increment of borrow index
pub fn increment_index(
	borrow_rate: Rate,
	index: Rate,
	delta_time: DurationSeconds,
) -> Result<Rate, ArithmeticError> {
	// borrow_rate * index * delta_time / SECONDS_PER_YEAR_NAIVE + index
	borrow_rate
		.safe_mul(&index)?
		.safe_mul(&FixedU128::saturating_from_integer(delta_time))?
		.safe_div(&FixedU128::saturating_from_integer(SECONDS_PER_YEAR_NAIVE))?
		.safe_add(&index)
}

pub fn increment_borrow_rate(
	borrow_rate: Rate,
	delta_time: DurationSeconds,
) -> Result<Rate, ArithmeticError> {
	borrow_rate
		.safe_mul(&FixedU128::saturating_from_integer(delta_time))?
		.safe_div(&FixedU128::saturating_from_integer(SECONDS_PER_YEAR_NAIVE))
}
