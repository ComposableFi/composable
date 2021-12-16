//! Common codes for defi pallets

use codec::{Codec, Decode, Encode, FullCodec};
use frame_support::{pallet_prelude::MaybeSerializeDeserialize, Parameter};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{CheckedAdd, CheckedMul, CheckedSub, Zero},
	DispatchError, FixedPointOperand,
};

use crate::{
	currency::{AssetIdLike, BalanceLike},
	math::LiftedFixedBalance,
};

#[derive(Encode, Decode, TypeInfo)]
pub struct Take<Balance> {
	/// amount of `base`
	pub amount: Balance,
	/// direction depends on referenced order type
	/// either minimal or maximal amount of `quote` for given unit of `base`
	pub limit: Balance,
}

/// take `quote` currency and give `base` currency
#[derive(Encode, Decode, TypeInfo)]
pub struct Sell<AssetId, Balance> {
	pub pair: CurrencyPair<AssetId>,
	pub take: Take<Balance>,	
}

/// given `base`, how much `quote` needed for unit
/// see [currency pair](https://www.investopedia.com/terms/c/currencypair.asp)
#[derive(Encode, Decode, TypeInfo)]
pub struct CurrencyPair<AssetId> {
	/// See [Base Currency](https://www.investopedia.com/terms/b/basecurrency.asp)
	pub base: AssetId,
	/// counter currency
	pub quote: AssetId,
}

/// type parameters for traits in pure defi area
pub trait DeFiEngine {
	/// The asset ID type
	type AssetId: AssetIdLike;
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

impl<AssetId, Balance> Sell<AssetId, Balance> {
	pub fn new(base: AssetId, quote: AssetId, quote_amount: Balance) -> Self {
		Self { limit: quote_amount, pair: CurrencyPair { base, quote } }
	}
}

/// default sale is no sale
impl<AssetId: Default, Balance: Default> Default for Sell<AssetId, Balance> {
	fn default() -> Self {
		Self { pair: Default::default(), limit: Default::default() }
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
		order: Sell<Self::AssetId, Self::Balance>,
		base_amount: Self::Balance,
		configuration: Configuration,
	) -> Result<Self::OrderId, DispatchError>;
	/// take order. get not found error if order never existed or was removed.
	/// - `take.limit` - for `sell` order it is maximal value are you to pay for `base` in `quote` asset, for `buy`
	/// order it is minimal value you are eager to accept for `base` 
	/// - `take.amount` - amount of
	/// `base` you are ready to exchange for this order
	fn take(
		from_to: &Self::AccountId,
		order: Self::OrderId,
		take: Take<Self::Balance>,
	) -> Result<(), DispatchError>;
}

pub trait DeFiComposableConfig: frame_system::Config {
	// what.
	type AssetId: AssetIdLike + MaybeSerializeDeserialize + From<u128> + Default;

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
