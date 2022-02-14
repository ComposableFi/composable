use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{traits::Get, BoundedVec, RuntimeDebug};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, Permill};
use sp_std::vec::Vec;

use crate::defi::CurrencyPair;

/// Implement AMM curve from "StableSwap - efficient mechanism for Stablecoin liquidity by Micheal
/// Egorov" Also blog at https://miguelmota.com/blog/understanding-stableswap-curve/ has very good explanation.
pub trait CurveAmm {
	/// The asset ID type
	type AssetId;
	/// The balance type of an account
	type Balance;
	/// The user account identifier type for the runtime
	type AccountId;
	/// Type that represents pool id
	type PoolId;

	fn pool_exists(pool_id: Self::PoolId) -> bool;

	fn currency_pair(pool_id: Self::PoolId) -> Result<CurrencyPair<Self::AssetId>, DispatchError>;

	/// Get pure exchange value for given units of given asset. (Note this does not include fees.)
	/// `pool_id` the pool containing the `asset_id`.
	/// `asset_id` the asset the user is interested in.
	/// `amount` the amount of `asset_id` the user want to obtain.
	/// Return the amount of quote asset if `asset_id` is base asset, otherwise the amount of base
	/// asset.
	fn get_exchange_value(
		pool_id: Self::PoolId,
		asset_id: Self::AssetId,
		amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError>;

	/// Buy given `amount` of given asset from the pool.
	/// In buy user does not know how much assets he/she has to exchange to get desired amount.
	fn buy(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		asset_id: Self::AssetId,
		amount: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::Balance, DispatchError>;

	/// Sell given `amount` of given asset to the pool.
	/// In sell user specifies `amount` of asset he/she wants to exchange to get other asset.
	fn sell(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		asset_id: Self::AssetId,
		amount: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::Balance, DispatchError>;

	/// Deposit coins into the pool
	/// `amounts` - list of amounts of coins to deposit,
	/// `min_mint_amount` - minimum amout of LP tokens to mint from the deposit.
	fn add_liquidity(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		base_amount: Self::Balance,
		quote_amount: Self::Balance,
		min_mint_amount: Self::Balance,
		keep_alive: bool,
	) -> Result<(), DispatchError>;

	/// Withdraw coins from the pool.
	/// Withdrawal amount are based on current deposit ratios.
	/// `amount` - quantity of LP tokens to burn in the withdrawal,
	/// `min_amounts` - minimum amounts of underlying coins to receive.
	fn remove_liquidity(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		lp_amount: Self::Balance,
		min_base_amount: Self::Balance,
		min_quote_amount: Self::Balance,
	) -> Result<(), DispatchError>;

	/// Perform an exchange.
	/// This operation is a buy order on the provided `pair`, effectively trading the quote asset
	/// against the base one. The pair can be swapped to execute a sell order.
	/// Implementor must check the pair.
	fn exchange(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		pair: CurrencyPair<Self::AssetId>,
		quote_amount: Self::Balance,
		min_receive: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::Balance, DispatchError>;
}

/// Pool type
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Default, PartialEq, Eq, RuntimeDebug)]
pub struct StableSwapPoolInfo<AccountId, AssetId> {
	/// Owner of pool
	pub owner: AccountId,
	/// Swappable assets
	pub pair: CurrencyPair<AssetId>,
	/// AssetId of LP token,
	pub lp_token: AssetId,
	/// Initial amplification coefficient
	pub amplification_coefficient: u16,
	/// Amount of the fee pool charges for the exchange, this goes to liquidity provider.
	pub fee: Permill,
	/// Amount of the fee pool charges for the exchange
	pub protocol_fee: Permill,
}

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
		slippage: sp_runtime::Perbill,
	) -> Result<Self::Balance, DispatchError>;
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Default, PartialEq, RuntimeDebug)]
pub struct ConstantProductPoolInfo<AccountId, AssetId> {
	/// Owner of pool
	pub owner: AccountId,
	/// Swappable assets
	pub pair: CurrencyPair<AssetId>,
	/// AssetId of LP token,
	pub lp_token: AssetId,
	/// Amount of the fee pool charges for the exchange
	pub fee: Permill,
	/// Amount of the fee pool charges for the exchange
	pub owner_fee: Permill,
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum DexRouteNode<PoolId> {
	Curve(PoolId),
	Uniswap(PoolId),
}

/// Describes route for DEX.
/// `Direct` gives vector of pool_id to use as router.
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum DexRoute<PoolId, MaxHops: Get<u32>> {
	Direct(BoundedVec<DexRouteNode<PoolId>, MaxHops>),
}

pub trait DexRouter<AccountId, AssetId, PoolId, Balance, MaxHops> {
	/// If route is `None` then delete existing entry for `asset_pair`
	/// If route is `Some` and no entry exist for `asset_pair` then add new entry
	/// else update existing entry.
	fn update_route(
		who: &AccountId,
		asset_pair: CurrencyPair<AssetId>,
		route: Option<BoundedVec<DexRouteNode<PoolId>, MaxHops>>,
	) -> Result<(), DispatchError>;
	/// If route exist return `Some(Vec<PoolId>)`, else `None`.
	fn get_route(asset_pair: CurrencyPair<AssetId>) -> Option<Vec<DexRouteNode<PoolId>>>;
	/// Exchange `dx` of `base` asset of `asset_pair` with associated route.
	fn exchange(
		who: &AccountId,
		asset_pair: CurrencyPair<AssetId>,
		dx: Balance,
	) -> Result<Balance, DispatchError>;
	/// Sell `amount` of `base` asset of asset_pair with associated route.
	fn sell(
		who: &AccountId,
		asset_pair: CurrencyPair<AssetId>,
		amount: Balance,
	) -> Result<Balance, DispatchError>;
	/// Buy `amount` of `quote` asset of asset_pair with associated route.
	fn buy(
		who: &AccountId,
		asset_pair: CurrencyPair<AssetId>,
		amount: Balance,
	) -> Result<Balance, DispatchError>;
}

pub struct ConversionError;

/// Similar to `sp_runtime::traits::Convert` but in case if type `A` can't be converted to
/// `B` it returns Error.
pub trait SafeConvert<A, B> {
	fn convert(a: A) -> Result<B, ConversionError>;
}
