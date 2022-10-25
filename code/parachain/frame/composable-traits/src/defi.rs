//! Common codes and conventions for DeFi pallets

use codec::{Codec, Decode, Encode, FullCodec, MaxEncodedLen};
use frame_support::{pallet_prelude::MaybeSerializeDeserialize, Parameter};
use scale_info::TypeInfo;
use sp_arithmetic::Rounding;
use sp_runtime::{
	helpers_128bit::multiply_by_rational_with_rounding,
	traits::{CheckedAdd, CheckedMul, CheckedSub, Zero},
	ArithmeticError, DispatchError, FixedPointNumber, FixedPointOperand, FixedU128,
};
use sp_std::fmt::Debug;

use crate::currency::{AssetIdLike, BalanceLike, MathBalance};

/// I give `amount` into protocol, but want to some amount within `limit` back. Amount of what
/// depends on protocol and other higher context.
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Debug, Clone, PartialEq, Eq)]
pub struct Take<Balance> {
	/// amount of `base`
	pub amount: Balance,
	/// direction depends on referenced order type
	/// either minimal or maximal amount of `quote` for given `base`
	/// depending on engine configuration, `limit` can be hard or flexible (change with time)
	pub limit: LiftedFixedBalance,
}

impl<Balance: MathBalance> Take<Balance> {
	pub fn is_valid(&self) -> bool {
		self.amount > Balance::zero() && self.limit > Ratio::zero()
	}

	pub fn new(amount: Balance, limit: Ratio) -> Self {
		Self { amount, limit }
	}

	pub fn quote_limit_amount(&self) -> Result<Balance, ArithmeticError> {
		self.quote_amount(self.amount)
	}

	pub fn quote_amount(&self, amount: Balance) -> Result<Balance, ArithmeticError> {
		let result = multiply_by_rational_with_rounding(
			amount.into(),
			self.limit.into_inner(),
			Ratio::DIV,
			Rounding::Down,
		)
		.ok_or(ArithmeticError::Overflow)?;
		result.try_into().map_err(|_| ArithmeticError::Overflow)
	}
}

/// take `quote` currency and give `base` currency
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Debug, Clone, PartialEq, Eq)]
pub struct Sell<AssetId, Balance> {
	pub pair: CurrencyPair<AssetId>,
	pub take: Take<Balance>,
}

impl<AssetId: PartialEq, Balance: MathBalance> Sell<AssetId, Balance> {
	pub fn is_valid(&self) -> bool {
		self.take.is_valid()
	}
	pub fn new(
		base: AssetId,
		quote: AssetId,
		base_amount: Balance,
		minimal_base_unit_price_in_quote: Ratio,
	) -> Self {
		Self {
			take: Take { amount: base_amount, limit: minimal_base_unit_price_in_quote },
			pair: CurrencyPair { base, quote },
		}
	}
}

/// See [currency pair](https://www.investopedia.com/terms/c/currencypair.asp)
/// Pair with same `base` and `quote` is considered valid as it allows to have mixer, money
/// laundering like behavior.
/// Can be used with Oracles, DEXes.
/// Example, can do - give `base`, how much `quote` needed for unit.
/// Can be local `Copy` `AssetId` or remote XCM asset id pair.
#[repr(C)]
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Debug, Clone)]
pub struct CurrencyPair<AssetId> {
	/// See [Base Currency](https://www.investopedia.com/terms/b/basecurrency.asp).
	/// Also can be named `native`(to the market) currency.
	/// Usually less stable, can be used as collateral.
	pub base: AssetId,
	/// Counter currency.
	/// Also can be named `price` currency.
	/// Usually more stable, may be `borrowable` asset.
	pub quote: AssetId,
}

// TODO (vim): Equivalence holds even if base = quote and quote = base. Confusing!!!
impl<AssetId: PartialEq> PartialEq for CurrencyPair<AssetId> {
	fn eq(&self, other: &Self) -> bool {
		(self.base == other.base && self.quote == other.quote) ||
			(self.base == other.quote && self.quote == other.base)
	}
}

impl<AssetId: PartialEq> Eq for CurrencyPair<AssetId> {}

impl<AssetId: Ord> Ord for CurrencyPair<AssetId> {
	fn cmp(&self, other: &Self) -> core::cmp::Ordering {
		if self.eq(other) {
			return core::cmp::Ordering::Equal
		}
		let base_ordering = self.base.cmp(&other.base);
		if base_ordering.is_eq() {
			return self.quote.cmp(&other.quote)
		}
		base_ordering
	}
}

impl<AssetId: Ord> PartialOrd for CurrencyPair<AssetId> {
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl<AssetId: Copy + PartialEq> CurrencyPair<AssetId> {
	pub fn swap(&self) -> Self {
		Self { base: self.quote, quote: self.base }
	}

	pub fn contains(&self, asset_id: AssetId) -> bool {
		self.base == asset_id || self.quote == asset_id
	}
}

impl<AssetId> From<(AssetId, AssetId)> for CurrencyPair<AssetId> {
	fn from(other: (AssetId, AssetId)) -> Self {
		Self { base: other.0, quote: other.1 }
	}
}

/// `AssetId` is Copy, than consider pair to be Copy
impl<AssetId: Copy> Copy for CurrencyPair<AssetId> {}

impl<AssetId: PartialEq> CurrencyPair<AssetId> {
	pub fn new(base: AssetId, quote: AssetId) -> Self {
		Self { base, quote }
	}

	///```rust
	/// let pair = composable_traits::defi::CurrencyPair::<u128>::new(13, 42);
	/// let slice =  pair.as_slice();
	/// assert_eq!(slice[0], pair.base);
	/// assert_eq!(slice[1], pair.quote);
	/// ```
	/// ```rust
	/// # let pair = composable_traits::defi::CurrencyPair::<u128>::new(13, 42);
	/// # let slice =  pair.as_slice();
	/// // it is copy
	/// drop(pair);
	/// let _ = slice[0];
	/// ```
	pub fn as_slice(&self) -> &[AssetId] {
		#[allow(trivial_casts)]
		unsafe {
			// used often and we avoid clone here
			sp_std::slice::from_raw_parts(self as *const Self as *const AssetId, 2)
		}
	}

	pub fn reverse(&mut self) {
		sp_std::mem::swap(&mut self.quote, &mut self.base)
	}
}

impl<AssetId: PartialEq> AsRef<[AssetId]> for CurrencyPair<AssetId> {
	fn as_ref(&self) -> &[AssetId] {
		self.as_slice()
	}
}

#[cfg(test)]
mod test_currency_pair {
	use super::*;
	#[test]
	fn test_cmp_for_currency_pair() {
		let pair1: CurrencyPair<u8> = CurrencyPair::new(0_u8, 1_u8);
		let pair2: CurrencyPair<u8> = CurrencyPair::new(1_u8, 2_u8);
		let pair3: CurrencyPair<u8> = CurrencyPair::new(0_u8, 1_u8);
		let pair4: CurrencyPair<u8> = CurrencyPair::new(1_u8, 0_u8);
		assert_eq!(pair1.cmp(&pair2), core::cmp::Ordering::Less);
		assert_eq!(pair1.cmp(&pair4), core::cmp::Ordering::Equal);
		assert_eq!(pair1.cmp(&pair3), core::cmp::Ordering::Equal);
		assert_eq!(pair2.cmp(&pair1), core::cmp::Ordering::Greater);
		assert_eq!(pair2.cmp(&pair1.swap()), core::cmp::Ordering::Greater);
		assert_eq!(pair2.cmp(&pair3), core::cmp::Ordering::Greater);
		assert_eq!(pair2.cmp(&pair4), core::cmp::Ordering::Greater);
		assert_eq!(pair3.cmp(&pair1), core::cmp::Ordering::Equal);
		assert_eq!(pair3.cmp(&pair2), core::cmp::Ordering::Less);
		assert_eq!(pair3.cmp(&pair4), core::cmp::Ordering::Equal);
		assert_eq!(pair4.cmp(&pair1), core::cmp::Ordering::Equal);
		assert_eq!(pair4.cmp(&pair2), core::cmp::Ordering::Less);
		assert_eq!(pair4.cmp(&pair3), core::cmp::Ordering::Equal);
	}
}

/// type parameters for traits in pure defi area
pub trait DeFiEngine {
	/// The asset ID type
	type MayBeAssetId: AssetIdLike;
	/// The balance type of an account
	type Balance: BalanceLike;
	/// The user account identifier type for the runtime
	type AccountId;
}

/// take nothing
impl<Balance: Default> Default for Take<Balance> {
	fn default() -> Self {
		Self { amount: Default::default(), limit: Default::default() }
	}
}

impl<AssetId: Default> Default for CurrencyPair<AssetId> {
	fn default() -> Self {
		Self { base: Default::default(), quote: Default::default() }
	}
}

/// default sale is no sale and invalid sale
impl<AssetId: Default, Balance: Default> Default for Sell<AssetId, Balance> {
	fn default() -> Self {
		Self { pair: Default::default(), take: Default::default() }
	}
}

/// order is something that lives some some time until taken
pub trait OrderIdLike:
	FullCodec
	+ MaxEncodedLen
	+ Copy
	+ Eq
	+ PartialEq
	+ sp_std::fmt::Debug
	+ TypeInfo
	+ sp_std::hash::Hash
	+ Default
{
}
impl<
		T: FullCodec
			+ MaxEncodedLen
			+ Copy
			+ Eq
			+ PartialEq
			+ sp_std::fmt::Debug
			+ TypeInfo
			+ sp_std::hash::Hash
			+ Default,
	> OrderIdLike for T
{
}

pub trait SellEngine<Configuration>: DeFiEngine {
	type OrderId: OrderIdLike;
	/// sell base asset for price given or higher
	/// - `from_to` - account requesting sell
	fn ask(
		from_to: &Self::AccountId,
		order: Sell<Self::MayBeAssetId, Self::Balance>,
		configuration: Configuration,
	) -> Result<Self::OrderId, DispatchError>;
	/// take order. get not found error if order never existed or was removed.
	/// - `take.limit` - for `sell` order it is maximal value are you to pay for `base` in `quote`
	///   asset, for `buy`
	/// order it is minimal value you are eager to accept for `base`
	/// - `take.amount` - amount of
	/// `base` you are ready to exchange for this order
	fn take(
		from_to: &Self::AccountId,
		order_id: Self::OrderId,
		take: Take<Self::Balance>,
	) -> Result<(), DispatchError>;
}

pub trait DeFiComposableConfig: frame_system::Config {
	type MayBeAssetId: AssetIdLike + MaybeSerializeDeserialize + Default + MaxEncodedLen + Debug;

	type Balance: BalanceLike
		+ MathBalance
		+ Default
		+ Parameter
		+ Codec
		+ Copy
		+ Ord
		+ CheckedAdd
		+ CheckedSub
		+ CheckedMul
		+ CheckedSub
		+ From<u64> // at least 64 bit
		+ From<u128>
		+ Zero
		+ FixedPointOperand
		+ Into<u128>; // cannot do From<u128>, until LiftedFixedBalance integer part is larger than 128
			  // bit
}

// TODO: Use Validate for these types.

pub mod validate {
	use composable_support::validation::Validate;
	use sp_runtime::{traits::One, FixedU128};

	/// Ensures that the value is `>= 1`.
	pub struct OneOrMore;

	impl Validate<FixedU128, OneOrMore> for OneOrMore {
		fn validate(input: FixedU128) -> Result<FixedU128, &'static str> {
			if input >= FixedU128::one() {
				Ok(input)
			} else {
				Err("number was less than one")
			}
		}
	}

	/// Ensures that the value is `> 1`.
	pub struct MoreThanOne;

	impl Validate<FixedU128, MoreThanOne> for MoreThanOne {
		fn validate(input: FixedU128) -> Result<FixedU128, &'static str> {
			if input > FixedU128::one() {
				Ok(input)
			} else {
				Err("number was less than or equal to one")
			}
		}
	}

	#[cfg(test)]
	mod test_validate_fixed_u128 {
		use composable_support::validation::TryIntoValidated;
		use sp_runtime::{
			traits::{One, Zero},
			FixedPointNumber, FixedU128,
		};

		use super::*;

		#[test]
		fn test_validate_one_or_more() {
			let zero = FixedU128::zero();
			let less_than_one = FixedU128::checked_from_rational(1, 10).unwrap();
			let one = FixedU128::one();
			let more_than_one = FixedU128::checked_from_integer::<u128>(100_000).unwrap();

			assert!(zero.try_into_validated::<OneOrMore>().is_err());
			assert!(less_than_one.try_into_validated::<OneOrMore>().is_err());
			assert!(one.try_into_validated::<OneOrMore>().is_ok());
			assert!(more_than_one.try_into_validated::<OneOrMore>().is_ok());
		}

		#[test]
		fn test_validate_more_than_one() {
			let zero = FixedU128::zero();
			let less_than_one = FixedU128::checked_from_rational(1, 10).unwrap();
			let one = FixedU128::one();
			let more_than_one = FixedU128::checked_from_integer::<u128>(100_000).unwrap();

			assert!(zero.try_into_validated::<MoreThanOne>().is_err());
			assert!(less_than_one.try_into_validated::<MoreThanOne>().is_err());
			assert!(one.try_into_validated::<MoreThanOne>().is_err());
			assert!(more_than_one.try_into_validated::<MoreThanOne>().is_ok());
		}
	}
}

pub type OneOrMoreFixedU128 = FixedU128;

/// The fixed point number of suggested by substrate precision
/// Must be (1.0..MAX] because applied only to price normalized values
pub type MoreThanOneFixedU128 = FixedU128;
// Possible alternative to using type aliases.
// pub struct MoreThanOneFixedU128(Validated<FixedU128, MoreThanOne>);

/// Must be \[0..1\]
pub type ZeroToOneFixedU128 = FixedU128;

/// Number like of higher bits, so that amount and balance calculations are done it it with higher
/// precision via fixed point.
/// While this is 128 bit, cannot support u128 because 18 bits are for of mantissa (so maximal
/// integer is 110 bit). Can support u128 if lift upper to use FixedU256 analog.
pub type LiftedFixedBalance = FixedU128;

/// unitless ratio of one thing to other.
pub type Ratio = FixedU128;
pub type Rate = FixedU128;

#[cfg(test)]
mod tests {
	use super::{Ratio, Take};
	use sp_runtime::FixedPointNumber;

	#[test]
	fn take_ratio_half() {
		let price = 10;
		let amount = 100_u128;
		let take = Take::new(amount, Ratio::saturating_from_integer(price));
		let result = take.quote_amount(amount / 2).unwrap();
		assert_eq!(result, price * amount / 2);
	}

	#[test]
	fn take_ratio_half_amount_half_price() {
		let price_part = 50;
		let amount = 100_u128;
		let take = Take::new(amount, Ratio::saturating_from_rational(price_part, 100));
		let result = take.quote_amount(amount).unwrap();
		assert_eq!(result, price_part * amount / 100);
	}
}
