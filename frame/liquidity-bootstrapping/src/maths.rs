use composable_traits::math::SafeArithmetic;
use rust_decimal::{
	prelude::{FromPrimitive, ToPrimitive},
	Decimal, MathematicalOps,
};
use sp_runtime::{traits::One, ArithmeticError, DispatchError, PerThing};

/// From https://balancer.fi/whitepaper.pdf, equation (15)
/// Compute the amount of base asset (out) given the quote asset (in).
/// - `wi` the weight on the quote asset
/// - `wo` the weight of the base asset
/// - `bi` the pool quote balance
/// - `bo` the pool base balance
/// - `ai` the quote amount to trade
#[inline(always)]
pub(crate) fn compute_out_given_in<T: PerThing>(
	wi: T,
	wo: T,
	bi: u128,
	bo: u128,
	ai: u128,
) -> Result<u128, DispatchError>
where
	T::Inner: Into<u32>,
{
	let ai = Decimal::from_u128(ai).ok_or(ArithmeticError::Overflow)?;
	let bi = Decimal::from_u128(bi).ok_or(ArithmeticError::Overflow)?;
	let bo = Decimal::from_u128(bo).ok_or(ArithmeticError::Overflow)?;
	let weight_numer =
		Decimal::from_u32(wi.deconstruct().into()).ok_or(ArithmeticError::Overflow)?;
	let weight_denom =
		Decimal::from_u32(wo.deconstruct().into()).ok_or(ArithmeticError::Overflow)?;
	let weight_power = weight_numer.safe_div(&weight_denom)?;
	let bi_div_bi_plus_ai = bi.safe_div(&bi.safe_add(&ai)?)?;
	let term_to_weight_power =
		bi_div_bi_plus_ai.checked_powd(weight_power).ok_or(ArithmeticError::Overflow)?;
	let one_minus_term = Decimal::one().safe_sub(&term_to_weight_power)?;
	let ao = bo.safe_mul(&one_minus_term)?.to_u128().ok_or(ArithmeticError::Overflow)?;
	Ok(ao)
}

/// From https://balancer.fi/whitepaper.pdf, equation (2)
/// Compute the spot price of an asset pair.
/// - `wi` the weight on the quote asset
/// - `wo` the weight of the base asset
/// - `bi` the pool quote balance
/// - `bo` the pool base balance
/// - `base_unit` the unit normalized to the base asset decimal
#[inline(always)]
pub(crate) fn compute_spot_price<T: PerThing>(wi: T, wo: T, bi: u128, bo: u128, base_unit: u128) -> Result<u128, DispatchError>
where
	T::Inner: Into<u32>,
{
	let base_unit = Decimal::from_u128(base_unit).ok_or(ArithmeticError::Overflow)?;
	let bi = Decimal::from_u128(bi).ok_or(ArithmeticError::Overflow)?;
	let bo = Decimal::from_u128(bo).ok_or(ArithmeticError::Overflow)?;
	let full_perthing =
		Decimal::from_u32(T::one().deconstruct().into()).ok_or(ArithmeticError::Overflow)?;
	let wi_numer = Decimal::from_u32(wi.deconstruct().into()).ok_or(ArithmeticError::Overflow)?;
	let wi = wi_numer.safe_div(&full_perthing)?;
	let wo_numer = Decimal::from_u32(wo.deconstruct().into()).ok_or(ArithmeticError::Overflow)?;
	let wo = wo_numer.safe_div(&full_perthing)?;
	let bi_div_wi = bi.safe_div(&wi)?;
	let bo_div_wo = bo.safe_div(&wo)?;
	let spot_price = bi_div_wi.safe_div(&bo_div_wo)?;
	Ok(spot_price.safe_mul(&base_unit)?.to_u128().ok_or(ArithmeticError::Overflow)?)
}
