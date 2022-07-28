use composable_support::math::safe::{
	safe_multiply_by_rational, SafeAdd, SafeDiv, SafeMul, SafeSub,
};
use frame_support::ensure;
use rust_decimal::{
	prelude::{FromPrimitive, ToPrimitive},
	Decimal, MathematicalOps,
};
use sp_runtime::{
	traits::{IntegerSquareRoot, One, Zero},
	ArithmeticError, PerThing,
};

/// From https://balancer.fi/whitepaper.pdf, equation (2)
/// Compute the spot price of an asset pair.
/// - `wi` the weight of the quote asset
/// - `wo` the weight of the base asset
/// - `bi` the pool quote balance
/// - `bo` the pool base balance
/// - `base_unit` the unit normalized to the base asset decimal
pub fn compute_spot_price<T: PerThing>(
	wi: T,
	wo: T,
	bi: u128,
	bo: u128,
	base_unit: u128,
) -> Result<u128, ArithmeticError>
where
	T::Inner: Into<u32>,
{
	let wi: u32 = wi.deconstruct().into();
	let wo: u32 = wo.deconstruct().into();
	let weight_sum = wi.safe_add(&wo)?;
	let expected_weight_sum: u32 = T::one().deconstruct().into();
	ensure!(weight_sum == expected_weight_sum, ArithmeticError::Overflow);

	let base_unit = Decimal::from_u128(base_unit).ok_or(ArithmeticError::Overflow)?;
	let bi = Decimal::from_u128(bi).ok_or(ArithmeticError::Overflow)?;
	let bo = Decimal::from_u128(bo).ok_or(ArithmeticError::Overflow)?;
	let full_perthing =
		Decimal::from_u32(T::one().deconstruct().into()).ok_or(ArithmeticError::Overflow)?;
	let wi_numer = Decimal::from_u32(wi).ok_or(ArithmeticError::Overflow)?;
	let wi = wi_numer.safe_div(&full_perthing)?;
	let wo_numer = Decimal::from_u32(wo).ok_or(ArithmeticError::Overflow)?;
	let wo = wo_numer.safe_div(&full_perthing)?;
	let bi_div_wi = bi.safe_div(&wi)?;
	let bo_div_wo = bo.safe_div(&wo)?;
	let spot_price = bi_div_wi.safe_div(&bo_div_wo)?;
	spot_price.safe_mul(&base_unit)?.to_u128().ok_or(ArithmeticError::Overflow)
}

/// From https://balancer.fi/whitepaper.pdf, equation (15)
/// Compute the amount of base asset (out) given the quote asset (in).
/// - `wi` the weight on the quote asset
/// - `wo` the weight of the base asset
/// - `bi` the pool quote balance
/// - `bo` the pool base balance
/// - `ai` the quote amount to trade
pub fn compute_out_given_in<T: PerThing>(
	wi: T,
	wo: T,
	bi: u128,
	bo: u128,
	ai: u128,
) -> Result<u128, ArithmeticError>
where
	T::Inner: Into<u32>,
{
	let wi: u32 = wi.deconstruct().into();
	let wo: u32 = wo.deconstruct().into();
	let weight_sum = wi.safe_add(&wo)?;
	let expected_weight_sum: u32 = T::one().deconstruct().into();
	ensure!(weight_sum == expected_weight_sum, ArithmeticError::Overflow);

	let ai = Decimal::from_u128(ai).ok_or(ArithmeticError::Overflow)?;
	let bi = Decimal::from_u128(bi).ok_or(ArithmeticError::Overflow)?;
	let bo = Decimal::from_u128(bo).ok_or(ArithmeticError::Overflow)?;
	let weight_numer = Decimal::from_u32(wi).ok_or(ArithmeticError::Overflow)?;
	let weight_denom = Decimal::from_u32(wo).ok_or(ArithmeticError::Overflow)?;
	let weight_power = weight_numer.safe_div(&weight_denom)?;
	let bi_div_bi_plus_ai = bi.safe_div(&bi.safe_add(&ai)?)?;
	let term_to_weight_power =
		bi_div_bi_plus_ai.checked_powd(weight_power).ok_or(ArithmeticError::Overflow)?;
	let one_minus_term = Decimal::one().safe_sub(&term_to_weight_power)?;
	let ao = bo.safe_mul(&one_minus_term)?.to_u128().ok_or(ArithmeticError::Overflow)?;
	Ok(ao)
}

/// From https://balancer.fi/whitepaper.pdf, equation (20)
/// Compute the amount of quote asset (in) given the expected amount of base asset (out).
/// - `wi` the weight on the quote asset
/// - `wo` the weight of the base asset
/// - `bi` the pool quote balance
/// - `bo` the pool base balance
/// - `ai` the quote amount to trade
pub fn compute_in_given_out<T: PerThing>(
	wi: T,
	wo: T,
	bi: u128,
	bo: u128,
	ao: u128,
) -> Result<u128, ArithmeticError>
where
	T::Inner: Into<u32>,
{
	let wi: u32 = wi.deconstruct().into();
	let wo: u32 = wo.deconstruct().into();
	let weight_sum = wi.safe_add(&wo)?;
	let expected_weight_sum: u32 = T::one().deconstruct().into();
	ensure!(weight_sum == expected_weight_sum, ArithmeticError::Overflow);

	let ao = Decimal::from_u128(ao).ok_or(ArithmeticError::Overflow)?;
	let bi = Decimal::from_u128(bi).ok_or(ArithmeticError::Overflow)?;
	let bo = Decimal::from_u128(bo).ok_or(ArithmeticError::Overflow)?;
	let weight_numer = Decimal::from_u32(wo).ok_or(ArithmeticError::Overflow)?;
	let weight_denom = Decimal::from_u32(wi).ok_or(ArithmeticError::Overflow)?;
	let weight_power = weight_numer.safe_div(&weight_denom)?;
	let bo_div_bo_minus_ao = bo.safe_div(&bo.safe_sub(&ao)?)?;
	let term_to_weight_power =
		bo_div_bo_minus_ao.checked_powd(weight_power).ok_or(ArithmeticError::Overflow)?;
	let term_minus_one = term_to_weight_power.safe_sub(&Decimal::one())?;
	let ai = bi.safe_mul(&term_minus_one)?.to_u128().ok_or(ArithmeticError::Overflow)?;
	Ok(ai)
}

/// https://uniswap.org/whitepaper.pdf, equation (13)
/// Compute the initial share of an LP provider.
/// - `base_amount` the base asset amount deposited.
/// - `quote_amount` the quote asset amount deposited.
#[inline(always)]
pub fn compute_first_deposit_lp(
	base_amount: u128,
	quote_amount: u128,
) -> Result<u128, ArithmeticError> {
	base_amount
		.integer_sqrt_checked()
		.ok_or(ArithmeticError::Overflow)?
		.safe_mul(&quote_amount.integer_sqrt_checked().ok_or(ArithmeticError::Overflow)?)
}

/// https://uniswap.org/whitepaper.pdf, equation (12)
/// Compute the share of an LP provider for an existing, non-empty pool.
/// - `lp_total_issuance` the total LP already issued to other LP providers.
/// - `base_amount` the base amount provided by the current LP provider.
/// - `pool_base_aum` the pool base asset under management.
/// - `pool_quote_aum` the pool quote asset under management.
#[inline(always)]
pub fn compute_deposit_lp(
	lp_total_issuance: u128,
	base_amount: u128,
	quote_amount: u128,
	pool_base_aum: u128,
	pool_quote_aum: u128,
) -> Result<(u128, u128), ArithmeticError> {
	let first_deposit = lp_total_issuance.is_zero();
	if first_deposit {
		let lp_to_mint = compute_first_deposit_lp(base_amount, quote_amount)?;
		Ok((quote_amount, lp_to_mint))
	} else {
		let overwritten_quote_amount =
			safe_multiply_by_rational(pool_quote_aum, base_amount, pool_base_aum)?;
		let lp_to_mint = safe_multiply_by_rational(lp_total_issuance, base_amount, pool_base_aum)?;
		Ok((overwritten_quote_amount, lp_to_mint))
	}
}

/// Compute the share of an LP provider for an existing non-empty pool in the case with a single
/// asset.
///
/// From https://balancer.fi/whitepaper.pdf, equation (25):
///
/// - `amount` the amount deposited;
/// - `balance` the pool balance;
/// - `weight` the weight of asset;
/// - `lp_supply` the total LP already issued to other LP providers.
#[inline(always)]
pub fn compute_deposit_lp_single_asset<T: PerThing>(
	amount: u128,
	balance: u128,
	weight: T,
	lp_supply: u128,
) -> Result<u128, ArithmeticError>
where
	T::Inner: Into<u32>,
{
	let amount = Decimal::from_u128(amount).ok_or(ArithmeticError::Overflow)?;
	let balance = Decimal::from_u128(balance).ok_or(ArithmeticError::Overflow)?;
	let weight = Decimal::from_u32(weight.deconstruct().into()).ok_or(ArithmeticError::Overflow)?;
	let full_perthing =
		Decimal::from_u32(T::one().deconstruct().into()).ok_or(ArithmeticError::Overflow)?;
	let weight = weight.safe_div(&full_perthing)?;
	let lp_supply = Decimal::from_u128(lp_supply).ok_or(ArithmeticError::Overflow)?;

	let amount_div_balance = amount.safe_div(&balance)?;
	let value_ratio = Decimal::one()
		.safe_add(&amount_div_balance)?
		.checked_powd(weight)
		.ok_or(ArithmeticError::Overflow)?;
	lp_supply
		.safe_mul(&value_ratio.safe_sub(&Decimal::one())?)?
		.to_u128()
		.ok_or(ArithmeticError::Overflow)
}
