#![allow(dead_code)]
#![allow(clippy::many_single_char_names)]
use frame_support::{
	codec::{Decode, Encode},
	sp_runtime::Perbill,
};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, Permill};

use sp_std::vec::Vec;

/// Describes a simple exchanges which does not allow advanced configurations such as slippage.
pub trait SimpleExchange {
	type AssetId;
	type Balance;
	type AccountId;
	type Error;

	/// Obtains the current price for a given asset, possibly routing through multiple markets.
	fn price(asset_id: Self::AssetId) -> Option<Self::Balance>;

	/// Exchange `amount` of `from` asset for `to` asset. The maximum price paid for the `to` asset
	/// is `SimpleExchange::price * (1 + slippage)`
	fn exchange(
		from: Self::AssetId,
		from_account: Self::AccountId,
		to: Self::AssetId,
		to_account: Self::AccountId,
		to_amount: Self::Balance,
		slippage: Perbill,
	) -> Result<Self::Balance, DispatchError>;
}

pub struct TakeResult<BALANCE> {
	pub amount: BALANCE,
	pub total_price: BALANCE,
}

/// see for examples:
/// - https://github.com/galacticcouncil/Basilisk-node/blob/master/pallets/exchange/src/lib.rs
/// - https://github.com/Polkadex-Substrate/polkadex-aura-node/blob/master/pallets/polkadex/src/lib.rs
/// expected that failed exchanges are notified by events.
pub trait Orderbook {
	type AssetId;
	type Balance;
	type AccountId;
	type OrderId;

	/// sell. exchanges specified amount of asset to other at specific price
	/// `source_price` price per unit
	/// `amm_slippage` set to zero to avoid AMM sell
	/// for remote auction we should  have sent some random to make sure we have idempotent request
	fn post(
		account_from: &Self::AccountId,
		asset: Self::AssetId,
		want: Self::AssetId,
		source_amount: Self::Balance,
		source_price: Self::Balance,
		amm_slippage: Permill,
	) -> Result<Self::OrderId, DispatchError>;

	/// sell. exchanges specified amount of asset to other at market price.
	fn market_sell(
		account: &Self::AccountId,
		asset: Self::AssetId,
		want: Self::AssetId,
		amount: Self::Balance,
		amm_slippage: Permill,
	) -> Result<Self::OrderId, DispatchError>;

	/// buy
	fn take(
		account: &Self::AccountId,
		orders: impl Iterator<Item = Self::OrderId>,
		up_to: Self::Balance,
	) -> Result<TakeResult<Self::Balance>, DispatchError>;

	fn is_order_executed(order_id: &Self::OrderId) -> bool;
}

/// Implement AMM curve from "StableSwap - efficient mechanism for Stablecoin liquidity by Micheal
/// Egorov" Also blog at https://miguelmota.com/blog/understanding-stableswap-curve/ has very good explanation.

pub trait CurveAmm {
	/// The asset ID type
	type AssetId;
	/// The balance type of an account
	type Balance;
	/// The user account identifier type for the runtime
	type AccountId;

	/// Current number of pools (also ID for the next created pool)
	fn pool_count() -> PoolId;

	/// Information about the pool with the specified `id`
	fn pool(id: PoolId) -> Option<PoolInfo<Self::AccountId, Self::AssetId, Self::Balance>>;

	/// Creates a pool, taking a creation fee from the caller
	fn create_pool(
		who: &Self::AccountId,
		assets: Vec<Self::AssetId>,
		amplification_coefficient: Self::Balance,
	) -> Result<PoolId, DispatchError>;
}

/// Type that represents index type of token in the pool passed from the outside as an extrinsic
/// argument.
pub type PoolTokenIndex = u32;

/// Type that represents pool id
pub type PoolId = u32;

/// Pool type
#[derive(Encode, Decode, TypeInfo, Clone, Default, PartialEq, Eq, Debug)]
pub struct PoolInfo<AccountId, AssetId, Balance> {
	/// Owner of pool
	pub owner: AccountId,
	/// LP multiasset
	pub pool_asset: AssetId,
	/// List of multiasset supported by the pool
	pub assets: Vec<AssetId>,
	/// Initial amplification coefficient
	pub amplification_coefficient: Balance,
	/// Current balances
	pub balances: Vec<Balance>,
}
