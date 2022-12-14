use composable_support::math::safe::{
	safe_multiply_by_rational, SafeAdd, SafeDiv, SafeMul, SafeSub,
};
use frame_support::ensure;
use rust_decimal::{
	prelude::{FromPrimitive, ToPrimitive},
	Decimal, MathematicalOps, RoundingStrategy,
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

/// Compute the amount of the output token given the amount of the input token.
///
/// If `Ok`, returns a `ConstantProductAmmValueFeePair` containing the `a_out` and the `fee`.
/// To get `a_out` without accounting for the fee, set `f = 0`.
/// Amount out, round down results.
///
/// **NOTE:** Weights must already be normalized.
///
/// # Parameters
/// * `w_i` - Weight of the input token
/// * `w_o` - Weight of the output token
/// * `b_i` - Balance of the input token
/// * `b_o` - Balance of the output token
/// * `a_sent` - Amount of the input token sent by the user
/// * `f` - Total swap fee
///
/// From https://github.com/ComposableFi/composable/blob/main/rfcs/0008-pablo-lbp-cpp-restructure.md#41-fee-math-updates,
/// equation (2)
pub fn compute_out_given_in_new<T: PerThing>(
	w_i: T,
	w_o: T,
	b_i: u128,
	b_o: u128,
	a_sent: u128,
	f: T,
) -> ConstantProductAmmResult<ConstantProductAmmValueFeePair> {
	let w_i = Decimal::safe_from_u128(w_i.deconstruct().into())?;
	let w_o = Decimal::safe_from_u128(w_o.deconstruct().into())?;
	let b_i = Decimal::safe_from_u128(b_i)?;
	let b_o = Decimal::safe_from_u128(b_o)?;
	let a_sent = Decimal::safe_from_u128(a_sent)?;

	let weight_ratio = w_i.safe_div(&w_o)?;
	// NOTE(connor): Use if to prevent pointless conversions if `f` is zero
	let left_from_fee =
		if f.is_zero() { Decimal::ONE } else { Decimal::safe_from_per_thing(f.left_from_one())? };
	let a_sent_fee_cut = a_sent.safe_mul(&left_from_fee)?;

	let base = b_i.safe_div(&b_i.safe_add(&a_sent_fee_cut)?)?;
	let power = base.checked_powd(weight_ratio).ok_or(ArithmeticError::Overflow)?;
	let ratio = Decimal::ONE.safe_sub(&power)?;

	let a_out = b_o.safe_mul(&ratio)?.round_down().safe_to_u128()?;
	let fee = a_sent.safe_sub(&a_sent_fee_cut)?.round_up().safe_to_u128()?;

	Ok(ConstantProductAmmValueFeePair { value: a_out, fee })
}

trait SafeDecimalConversions {
	/// Safely converts a `u128` to a decimal value.
	fn safe_from_u128(num: u128) -> Result<Self, ArithmeticError>
	where
		Self: Sized;

	/// Safely converts a decimal value to a `u128`.
	fn safe_to_u128(self) -> Result<u128, ArithmeticError>;

	/// Converts a `u128` fixed point number with 12 decimal places to decimal.
	///
	/// Conducts `number / 10^12`.
	fn safe_from_fixed_point(num: u128) -> Result<Self, ArithmeticError>
	where
		Self: Sized;

	/// Safely convert to a fixed-point `u128` with 12 decimals.
	fn safe_to_fixed_point(self) -> Result<u128, ArithmeticError>;

	/// Computes the decimal value of a a `PerThing` by taking the deconstructed parts of a
	/// `PerThing` and dividing them by `PerThing::one()`.
	///
	/// # Example
	/// ```rust,ignore
	/// let per_thing = PerMill::from_percent(10);
	/// assert_eq!(decimal_from_per_thing(per_thing), Decimal::new(10, 2));
	/// ```
	fn safe_from_per_thing<T: PerThing>(per_thing: T) -> Result<Self, ArithmeticError>
	where
		Self: Sized;
}

impl SafeDecimalConversions for Decimal {
	fn safe_from_u128(num: u128) -> Result<Self, ArithmeticError> {
		Decimal::from_u128(num).ok_or(ArithmeticError::Overflow)
	}

	fn safe_to_u128(self) -> Result<u128, ArithmeticError> {
		self.to_u128().ok_or(ArithmeticError::Overflow)
	}

	fn safe_from_fixed_point(fixed_point: u128) -> Result<Self, ArithmeticError> {
		let numerator = Decimal::safe_from_u128(fixed_point)?;
		let denominator = Decimal::from(10)
			.checked_powd(Decimal::from(12))
			.ok_or(ArithmeticError::Overflow)?;

		numerator.safe_div(&denominator)
	}

	fn safe_to_fixed_point(self) -> Result<u128, ArithmeticError> {
		let rhs = Decimal::from(10)
			.checked_powd(Decimal::from(12))
			.ok_or(ArithmeticError::Overflow)?;

		self.safe_mul(&rhs)?.to_u128().ok_or(ArithmeticError::Overflow)
	}

	fn safe_from_per_thing<T: PerThing>(per_thing: T) -> Result<Self, ArithmeticError> {
		let numerator = Decimal::safe_from_u128(per_thing.deconstruct().into())?;
		let denominator = Decimal::safe_from_u128(T::one().deconstruct().into())?;

		numerator.safe_div(&denominator)
	}
}

trait RoundingDecimal {
	/// Round a decimal value to the next whole number with a given `RoundingStrategy`
	fn round_to_whole_with_strategy(&self, rounding_strategy: RoundingStrategy) -> Self;
	/// Round a decimal value away from zero to the next whole number
	/// i.e. `-0.5 -> -1` and `0.5 -> 1`
	fn round_up(&self) -> Self;
	/// Round a decimal value to zero to the next whole number
	/// i.e. `-0.5 -> 0` and `0.5 -> 0`
	fn round_down(&self) -> Self;
}

impl RoundingDecimal for Decimal {
	fn round_to_whole_with_strategy(&self, rounding_strategy: RoundingStrategy) -> Self {
		self.round_dp_with_strategy(0, rounding_strategy)
	}

	fn round_up(&self) -> Self {
		self.round_to_whole_with_strategy(RoundingStrategy::AwayFromZero)
	}

	fn round_down(&self) -> Self {
		self.round_to_whole_with_strategy(RoundingStrategy::ToZero)
	}
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

/// Compute the amount of the input token given the amount of the output token.
///
/// If `Ok`, returns a `ConstantProductAmmValueFeePair` containing the `a_sent` and the `fee`.
/// To get `a_sent` without accounting for the fee, set `f = 0`.
/// Amount in, round up results.
///
/// Notes:
/// * Weights must already be normalized
/// * For an unbounded fee, `1 - f` must be greater than `b_i / 2^96`
/// * For a fee bounded between 0% - 1%, `b_i` must be less than or equal to
///   `1_960_897_022_228_042_355_440_212_770_816 / 25`
///
/// # Parameters
/// * `w_i` - Weight of the input token
/// * `w_o` - Weight of the output token
/// * `b_i` - Balance of the input token
/// * `b_o` - Balance of the output token
/// * `a_out` - Amount of the output token desired by the user
/// * `f` - Total swap fee
///
/// From https://github.com/ComposableFi/composable/blob/main/rfcs/0008-pablo-lbp-cpp-restructure.md#41-fee-math-updates,
/// equation (3)
pub fn compute_in_given_out_new<T: PerThing>(
	w_i: T,
	w_o: T,
	b_i: u128,
	b_o: u128,
	a_out: u128,
	f: T,
) -> ConstantProductAmmResult<ConstantProductAmmValueFeePair> {
	ensure!(a_out <= b_o, ConstantProductAmmError::CannotTakeMoreThanAvailable);
	let w_i = Decimal::safe_from_u128(w_i.deconstruct().into())?;
	let w_o = Decimal::safe_from_u128(w_o.deconstruct().into())?;
	let b_i = Decimal::safe_from_u128(b_i)?;
	let b_o = Decimal::safe_from_u128(b_o)?;
	let a_out = Decimal::safe_from_u128(a_out)?;

	let weight_ratio = w_o.safe_div(&w_i)?;
	// NOTE(connor): Use if to prevent pointless conversions if `f` is zero
	let left_from_fee =
		if f.is_zero() { Decimal::ONE } else { Decimal::safe_from_per_thing(f.left_from_one())? };
	let b_i_over_fee = b_i.safe_div(&left_from_fee)?;
	let fee = Decimal::ONE.safe_sub(&left_from_fee)?;

	let base = b_o.safe_div(&b_o.safe_sub(&a_out)?)?;
	let power = base.checked_powd(weight_ratio).ok_or(ArithmeticError::Overflow)?;
	let ratio = power.safe_sub(&Decimal::ONE)?;

	let a_sent = b_i_over_fee.safe_mul(&ratio)?.round_up();
	let fee = dbg!(dbg!(a_sent).safe_mul(&dbg!(fee)))?.round_up().safe_to_u128()?;

	Ok(ConstantProductAmmValueFeePair { value: a_sent.safe_to_u128()?, fee })
}

pub type ConstantProductAmmResult<T> = Result<T, ConstantProductAmmError>;

/// Many math functions for constant product return a some output value and a fee. This struct
/// contains both.
#[derive(Debug, Eq, PartialEq)]
pub struct ConstantProductAmmValueFeePair {
	pub value: u128,
	pub fee: u128,
}

/// Calculates `a_k` when redeeming
///
/// **NOTE**: May overflow when `w_k` is below 25%
///
/// # Parameters
/// * `p_supply` - Existing supply of LP
/// * `p_redeemed` - Redeemed LP tokens
/// * `b_k` - balance of token `k`
/// * `w_k` - weight of token `k`
///
/// From https://github.com/ComposableFi/composable/blob/main/rfcs/0008-pablo-lbp-cpp-restructure.md#42-liquidity-provider-token-lpt-math-updates
/// Equation 8
pub fn compute_redeemed_for_lp<T: PerThing>(
	p_supply: u128,
	p_redeemed: u128,
	b_k: u128,
	w_k: T,
) -> Result<u128, ConstantProductAmmError> {
	let p_supply = Decimal::safe_from_u128(p_supply)?;
	let p_redeemed = Decimal::safe_from_u128(p_redeemed)?;
	let b_k = Decimal::safe_from_u128(b_k)?;
	let w_k = Decimal::safe_from_per_thing(w_k)?;

	let weight_ratio = Decimal::ONE.safe_div(&w_k)?;
	let base = Decimal::ONE.safe_sub(&p_redeemed.safe_div(&p_supply)?)?;
	let power = base.checked_powd(weight_ratio).ok_or(ArithmeticError::Overflow)?;
	let ratio = Decimal::ONE.safe_sub(&power)?;

	let a_k = b_k.safe_mul(&ratio)?;

	Ok(a_k.safe_to_u128()?)
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
			ConstantProductAmmError::InvalidTokensList =>
				DispatchError::from("Must provide non-empty tokens list!"),
		}
	}
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

/// Computes the LP to mint on first deposit.
///
/// If `Ok`, returns a `ConstantProductAmmValueFeePair` containing the `lp_to_mint` and the `fee`.
///
/// Fees are currently always 0. Fees are normally charged to avoid fee-less swaps by adding and
/// removing liquidity. With the initial deposit, these chances are so low that it is safe to
/// process without a fee.
///
/// Balances must be in fixed point 12 decimal representation.
///
/// # Parameters
/// * `pool_assets` - Vec of tuples containing `(token_deposit, token_balance, token_weight)`
/// * `f` - Fee
///
/// https://github.com/ComposableFi/composable/blob/main/rfcs/0008-pablo-lbp-cpp-restructure.md#42-liquidity-provider-token-lpt-math-updates
/// Equation 6
pub fn compute_first_deposit_lp_<T: PerThing>(
	// REVIEW(benluelo): Make this a named struct instead of a tuple?
	pool_assets: &[(u128, T)],
	_f: T,
) -> ConstantProductAmmResult<ConstantProductAmmValueFeePair> {
	let k: u128 = pool_assets.len().try_into().map_err(|_| ArithmeticError::Overflow)?;
	ensure!(!k.is_zero(), ConstantProductAmmError::InvalidTokensList);

	let product = pool_assets.iter().try_fold::<_, _, Result<_, ArithmeticError>>(
		Decimal::from(1),
		|product, (d_i, w_i)| {
			let d_i = Decimal::safe_from_fixed_point(*d_i)?;
			let w_i = Decimal::safe_from_per_thing(*w_i)?;
			let pow = d_i.checked_powd(w_i).ok_or(ArithmeticError::Overflow)?;

			product.safe_mul(&pow)
		},
	)?;

	let k = Decimal::safe_from_u128(k)?;
	let product = k.safe_mul(&product)?;

	// NOTE: Zero fees on first deposit
	Ok(ConstantProductAmmValueFeePair { value: product.safe_to_fixed_point()?, fee: 0 })
}

/// Computes the LP to mint on an existing deposit.
///
/// If `Ok`, returns a `ConstantProductAmmValueFeePair` containing the `lp_to_mint` and the `fee`.
///
/// # Parameters
/// * `p_supply` - Existing supply of LP tokens
/// * `d_k` - Deposit of token `k`
/// * `b_k` - Balance of token `k`
/// * `w_k` - Weight of token `k`
/// * `f` - Fee
///
/// https://github.com/ComposableFi/composable/blob/main/rfcs/0008-pablo-lbp-cpp-restructure.md#42-liquidity-provider-token-lpt-math-updates
/// Equation 5
pub fn compute_deposit_lp_<T: PerThing>(
	p_supply: u128,
	d_k: u128,
	b_k: u128,
	w_k: T,
	f: T,
) -> ConstantProductAmmResult<ConstantProductAmmValueFeePair> {
	let p_supply = Decimal::safe_from_u128(p_supply)?;
	let d_k = Decimal::safe_from_u128(d_k)?;
	let b_k = Decimal::safe_from_u128(b_k)?;
	let w_k = Decimal::safe_from_per_thing(w_k)?;

	let left_from_fee =
		if f.is_zero() { Decimal::ONE } else { Decimal::safe_from_per_thing(f.left_from_one())? };
	let d_k_left_from_fee = d_k.safe_mul(&left_from_fee)?;

	let base = d_k_left_from_fee.safe_add(&b_k)?.safe_div(&b_k)?;
	let power = base.checked_powd(w_k).ok_or(ArithmeticError::Overflow)?;
	let ratio = power.safe_sub(&Decimal::ONE)?;

	let issued = p_supply.safe_mul(&ratio)?.safe_to_u128()?;
	let fee = d_k.safe_sub(&d_k_left_from_fee)?.round_up().safe_to_u128()?;

	Ok(ConstantProductAmmValueFeePair { value: issued, fee })
}
