//! Common codes and conventions for DeFi pallets

use codec::{Codec, Decode, Encode, FullCodec};
use frame_support::{pallet_prelude::MaybeSerializeDeserialize, Parameter};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{CheckedAdd, CheckedMul, CheckedSub, Zero},
	ArithmeticError, DispatchError, FixedPointOperand, FixedU128,
};

use crate::{
	currency::{AssetIdLike, BalanceLike},
	defi::{LiftedFixedBalance},
	math::{SafeArithmetic},
};

#[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq)]
pub struct Take<Balance> {
	/// amount of `base`
	pub amount: Balance,
	/// direction depends on referenced order type
	/// either minimal or maximal amount of `quote` for given `base`
	/// depending on engine configuration, `limit` can be hard or flexible (change with time)
	pub limit: LiftedFixedBalance,
}

impl<Balance: PartialOrd + Zero + SafeArithmetic> Take<Balance> {
	pub fn is_valid(&self) -> bool {
		self.amount > Balance::zero() && self.limit > Ratio::zero()
	}
	pub fn new(amount: Balance, limit: Ratio) -> Self {
		Self { amount, limit }
	}

	pub fn quote_amount(&self) -> Result<Balance, ArithmeticError> {
		self.amount.safe_mul(&self.limit)
	}
}

/// take `quote` currency and give `base` currency
#[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq)]
pub struct Sell<AssetId, Balance> {
	pub pair: CurrencyPair<AssetId>,
	pub take: Take<Balance>,
}

impl<AssetId: PartialEq, Balance: PartialOrd + Zero + SafeArithmetic> Sell<AssetId, Balance> {
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

/// given `base`, how much `quote` needed for unit
/// see [currency pair](https://www.investopedia.com/terms/c/currencypair.asp)
/// Pair with same base and quote is considered valid as it allows to have mixer, money laundering
/// like behavior.
#[repr(C)]
#[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq)]
pub struct CurrencyPair<AssetId> {
	/// See [Base Currency](https://www.investopedia.com/terms/b/basecurrency.asp)
	pub base: AssetId,
	/// counter currency
	pub quote: AssetId,
}

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
	/// ```compile_fail
	/// # let pair = composable_traits::defi::CurrencyPair::<u128>::new(13, 42);
	/// # let slice =  pair.as_slice();
	/// drop(pair);
	/// let _ = slice[0];
	/// ```
	pub fn as_slice(&self) -> &[AssetId] {
		unsafe { sp_std::slice::from_raw_parts(self as *const Self as *const AssetId, 2) }
	}
}

impl<AssetId: PartialEq> AsRef<[AssetId]> for CurrencyPair<AssetId> {
	fn as_ref(&self) -> &[AssetId] {
		self.as_slice()
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
	FullCodec + Copy + Eq + PartialEq + sp_std::fmt::Debug + TypeInfo + sp_std::hash::Hash + Default
{
}
impl<
		T: FullCodec
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
	type MayBeAssetId: AssetIdLike + MaybeSerializeDeserialize + Default;

	type Balance: BalanceLike
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
		+ Zero
		+ FixedPointOperand
		+ Into<LiftedFixedBalance> // integer part not more than bits in this
		+ Into<u128>; // cannot do From<u128>, until LiftedFixedBalance integer part is larger than 128
			  // bit
}

/// The fixed point number from 0..to max 
pub type Rate = FixedU128;

/// The fixed point number of suggested by substrate precision
/// Must be (1.0.. because applied only to price normalized values
pub type MoreThanOneFixedU128 = FixedU128;

/// Must be [0..1]
pub type ZeroToOneFixedU128 = FixedU128;

/// Number like of higher bits, so that amount and balance calculations are done it it with higher
/// precision via fixed point.
/// While this is 128 bit, cannot support u128 because 18 bits are for of mantissa (so maximal
/// integer is 110 bit). Can support u128 if lift upper to use FixedU256 analog.
pub type LiftedFixedBalance = FixedU128;

/// unitless ratio of one thing to other. 
pub type Ratio = FixedU128;
