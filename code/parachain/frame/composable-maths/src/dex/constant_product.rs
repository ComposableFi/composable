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
	ArithmeticError, DispatchError, PerThing,
};

/// From https://balancer.fi/whitepaper.pdf, equation (2)
/// Compute the spot price of an asset pair.
/// - `wi` the weight on the quote asset
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

	// TODO (vim): This validation must be done at the pallet level then here as there could be more
	//  assets  and weights in the pool than what's here.
	ensure!(weight_sum == expected_weight_sum, ArithmeticError::Overflow);

	let base_unit = Decimal::from_u128(base_unit).ok_or(ArithmeticError::Overflow)?;
	let bi = Decimal::from_u128(bi).ok_or(ArithmeticError::Overflow)?;
	let bo = Decimal::from_u128(bo).ok_or(ArithmeticError::Overflow)?;
	let full_perthing =
		Decimal::from_u32(T::one().deconstruct().into()).ok_or(ArithmeticError::Overflow)?;
	let wi_numerator = Decimal::from_u32(wi).ok_or(ArithmeticError::Overflow)?;
	let wi = wi_numerator.safe_div(&full_perthing)?;
	let wo_numerator = Decimal::from_u32(wo).ok_or(ArithmeticError::Overflow)?;
	let wo = wo_numerator.safe_div(&full_perthing)?;
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
	// TODO (vim): This validation must be done at the pallet level then here as there could be more
	//  assets  and weights in the pool than what's here.
	ensure!(weight_sum == expected_weight_sum, ArithmeticError::Overflow);

	let ai = Decimal::from_u128(ai).ok_or(ArithmeticError::Overflow)?;
	let bi = Decimal::from_u128(bi).ok_or(ArithmeticError::Overflow)?;
	let bo = Decimal::from_u128(bo).ok_or(ArithmeticError::Overflow)?;
	let weight_numerator = Decimal::from_u32(wi).ok_or(ArithmeticError::Overflow)?;
	let weight_denominator = Decimal::from_u32(wo).ok_or(ArithmeticError::Overflow)?;
	let weight_power = weight_numerator.safe_div(&weight_denominator)?;
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

	// TODO (vim): This validation must be done at the pallet level then here as there could be more
	//  assets  and weights in the pool than what's here.
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
	/* TODO (vim): Possible attack vector exists: From https://uniswap.org/whitepaper.pdf
		The formula ensures that a liquidity pool share will never be worth less than
	the geometric mean of the reserves in that pool. However, it is possible for the value of
	a liquidity pool share to grow over time, either by accumulating trading fees or through
	“donations” to the liquidity pool. In theory, this could result in a situation where the value
	of the minimum quantity of liquidity pool shares (1e-18 pool shares) is worth so much that
	it becomes infeasible for small liquidity providers to provide any liquidity
	To mitigate this, Uniswap v2 burns the first 1e-15 (0.000000000000001) pool shares that
		are minted (1000 times the minimum quantity of pool shares), sending them to the zero
		address instead of to the minter. This should be a negligible cost for almost any token
		pair.11 But it dramatically increases the cost of the above attack. In order to raise the
		value of a liquidity pool share to $100, the attacker would need to donate $100,000 to the
		pool, which would be permanently locked up as liquidity.
			 */
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

#[derive(Debug, Eq, PartialEq)]
pub enum ConstantProductAmmError {
	ArithmeticError(ArithmeticError),
	CannotTakeMoreThanAvailable,
	InvalidTokensList,
}

impl From<ArithmeticError> for ConstantProductAmmError {
	fn from(error: ArithmeticError) -> Self {
		ConstantProductAmmError::ArithmeticError(error)
	}
}

impl From<ConstantProductAmmError> for DispatchError {
	fn from(error: ConstantProductAmmError) -> Self {
		match error {
			ConstantProductAmmError::ArithmeticError(error) => DispatchError::from(error),
			ConstantProductAmmError::CannotTakeMoreThanAvailable =>
				DispatchError::from(
					"`a_out` must not be greater than `b_o` (can't take out more than what's available)!"
				),
			ConstantProductAmmError::InvalidTokensList => DispatchError::from("Must provide non-empty tokens list!"),
		}
	}
}

/// Computes the decimal value of a a `PerThing` by taking the deconstructed parts of a `PerThing`
/// and dividing them by `PerThing::one()`.
///
/// # Example
/// ```rust,ignore
/// let per_thing = PerMill::from_percent(10);
/// assert_eq!(decimal_from_per_thing(per_thing), Decimal::new(10, 2));
/// ```
fn decimal_from_per_thing<T: PerThing>(per_thing: T) -> Result<Decimal, ArithmeticError> {
	let numerator =
		Decimal::from_u128(per_thing.deconstruct().into()).ok_or(ArithmeticError::Overflow)?;
	let denominator =
		Decimal::from_u128(T::one().deconstruct().into()).ok_or(ArithmeticError::Overflow)?;

	numerator.safe_div(&denominator)
}

/// Computes the LP to mint on first deposit.
///
/// If `Ok`, returns a tuple containing `(lp_to_mint, fee)`.
///
/// Fees are currently always 0. Fees are normally charged to avoid fee-less swaps by adding and
/// removing liquidity. With the initial deposit, these chances are so low that it is safe to
/// process without a fee.
///
/// Paramaters:
/// * `lp_token_details` - Vec of tuples containing `(token_deposit, token_balance, token_weight)`
/// * `f` - Fee
///
/// https://github.com/ComposableFi/composable/blob/main/rfcs/0008-pablo-lbp-cpp-restructure.md#42-liquidity-provider-token-lpt-math-updates
/// Equation 6
fn compute_first_deposit_lp_<T: PerThing>(
	pool_assets: Vec<(u128, u128, T)>,
	_f: T,
) -> Result<(u128, u128), ConstantProductAmmError> {
	let k: u128 = pool_assets.len().try_into().map_err(|_| ArithmeticError::Overflow)?;
	let product = pool_assets
		.iter()
		.try_fold(1, |product, (d_i, _b_i, _w_i)| product.safe_mul(d_i))?;

	// NOTE: Zero fees on first deposit
	Ok((k.safe_mul(&product)?, 0))
}

/// Computes the LP to mint on an existing deposit.
///
/// If `Ok`, returns a tuple containing `(lp_to_mint, fee)`.
///
/// Paramaters:
/// * `p_supply` -
/// * `d_k` - Deposit of token `k`
/// * `b_k` - Balance of token `k`
/// * `w_k` - Weight of token `k`
/// * `f` - Fee
///
/// https://github.com/ComposableFi/composable/blob/main/rfcs/0008-pablo-lbp-cpp-restructure.md#42-liquidity-provider-token-lpt-math-updates
/// Equation 5
fn compute_deposit_lp_<T: PerThing>(
	p_supply: u128,
	d_k: u128,
	b_k: u128,
	w_k: T,
	f: T,
) -> Result<(u128, u128), ConstantProductAmmError> {
	let p_supply = Decimal::from_u128(p_supply).ok_or(ArithmeticError::Overflow)?;
	let d_k = Decimal::from_u128(d_k).ok_or(ArithmeticError::Overflow)?;
	let b_k = Decimal::from_u128(b_k).ok_or(ArithmeticError::Overflow)?;
	let w_k = decimal_from_per_thing(w_k)?;

	let left_from_fee =
		if f.is_zero() { Decimal::ONE } else { decimal_from_per_thing(f.left_from_one())? };
	let d_k_left_from_fee = d_k.safe_mul(&left_from_fee)?;

	let base = d_k_left_from_fee.safe_add(&b_k)?.safe_div(&b_k)?;
	let power = base.checked_powd(w_k).ok_or(ArithmeticError::Overflow)?;
	let ratio = power.safe_sub(&Decimal::ONE)?;

	let issued = p_supply.safe_mul(&ratio)?.to_u128().ok_or(ArithmeticError::Overflow)?;
	let fee = d_k.safe_sub(&d_k_left_from_fee)?.to_u128().ok_or(ArithmeticError::Overflow)?;

	Ok((issued, fee))
}
