use composable_support::math::safe::{SafeAdd, SafeDiv, SafeMul, SafeSub};
use frame_support::ensure;
use rust_decimal::{
	prelude::{FromPrimitive, ToPrimitive},
	Decimal, MathematicalOps, RoundingStrategy,
};
use sp_runtime::{traits::Zero, ArithmeticError, DispatchError, PerThing};

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
pub fn compute_out_given_in<T: PerThing>(
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
pub fn compute_in_given_out<T: PerThing>(
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
	let fee = a_sent.safe_mul(&fee)?.round_up().safe_to_u128()?;

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
pub fn compute_first_deposit_lp<T: PerThing>(
	// REVIEW(benluelo): Make this a named struct instead of a tuple?
	mut pool_assets: impl ExactSizeIterator<Item = (u128, T)>,
	_f: T,
) -> ConstantProductAmmResult<ConstantProductAmmValueFeePair> {
	let k: u128 = pool_assets.len().try_into().map_err(|_| ArithmeticError::Overflow)?;
	ensure!(!k.is_zero(), ConstantProductAmmError::InvalidTokensList);

	let product = pool_assets.try_fold::<_, _, Result<_, ArithmeticError>>(
		Decimal::from(1),
		|product, (d_i, w_i)| {
			let d_i = Decimal::safe_from_fixed_point(d_i)?;
			let w_i = Decimal::safe_from_per_thing(w_i)?;
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
pub fn compute_deposit_lp<T: PerThing>(
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

/// Computes the required deposit to receive a minimum amount of LPT
///
/// NOTE: When considering accounting for fees - the result of this function may produce a result
/// slightly lower than the minimum LP to mint.
///
/// # Parameters
/// * `p_supply` - Existing supply of LP tokens
/// * `p_issued_min` - Minimum LPT a user wants from their investment
/// * `b_k` - Balance of token `k`
/// * `f` - Fee
pub fn compute_deposit_for_min_lp<T: PerThing>(
	p_supply: u128,
	p_issued_min: u128,
	b_k: u128,
	f: T,
) -> ConstantProductAmmResult<ConstantProductAmmValueFeePair> {
	let left_from_fee = f.left_from_one();
	let nominator = p_issued_min.safe_mul(&b_k)?;
	let denominator = left_from_fee.mul_ceil(p_supply);

	let value = nominator.safe_div(&denominator)?;
	let fee = value.safe_sub(&left_from_fee.mul_ceil(value))?;

	Ok(ConstantProductAmmValueFeePair { value, fee })
}
