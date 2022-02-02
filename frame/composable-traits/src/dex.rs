use codec::{Decode, Encode};
use frame_support::{traits::Get, BoundedVec, RuntimeDebug};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, FixedU128, Permill};
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
	/// Type that represents index type of token in the pool passed from the outside as an extrinsic
	/// argument.
	type PoolTokenIndex;

	/// Type that represents pool id
	type PoolId;

	/// Check pool with given id exists.
	fn pool_exists(pool_id: Self::PoolId) -> bool;

	/// Current number of pools (also ID for the next created pool)
	fn pool_count() -> Self::PoolId;

	/// Deposit coins into the pool
	/// `amounts` - list of amounts of coins to deposit,
	/// `min_mint_amount` - minimum amout of LP tokens to mint from the deposit.
	fn add_liquidity(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		amounts: Vec<Self::Balance>,
		min_mint_amount: Self::Balance,
	) -> Result<(), DispatchError>;

	/// Withdraw coins from the pool.
	/// Withdrawal amount are based on current deposit ratios.
	/// `amount` - quantity of LP tokens to burn in the withdrawal,
	/// `min_amounts` - minimum amounts of underlying coins to receive.
	fn remove_liquidity(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		amount: Self::Balance,
		min_amounts: Vec<Self::Balance>,
	) -> Result<(), DispatchError>;

	/// Perform an exchange between two coins.
	/// `i` - index value of the coin to send,
	/// `j` - index value of the coin to receive,
	/// `dx` - amount of `i` being exchanged,
	/// `min_dy` - minimum amount of `j` to receive.
	fn exchange(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		i: Self::PoolTokenIndex,
		j: Self::PoolTokenIndex,
		dx: Self::Balance,
		min_dy: Self::Balance,
	) -> Result<Self::Balance, DispatchError>;

	/// Withdraw admin fees
	fn withdraw_admin_fees(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		admin_fee_account: &Self::AccountId,
	) -> Result<(), DispatchError>;
}

/// Pool type
#[derive(Encode, Decode, TypeInfo, Clone, Default, PartialEq, Eq, RuntimeDebug)]
pub struct StableSwapPoolInfo<AccountId, AssetId> {
	/// Owner of pool
	pub owner: AccountId,
	/// AssetId of LP token,
	pub lp_token: AssetId,
	/// Initial amplification coefficient
	pub amplification_coefficient: FixedU128,
	/// Amount of the fee pool charges for the exchange
	pub fee: Permill,
	/// Amount of the admin fee pool charges for the exchange
	pub admin_fee: Permill,
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

#[derive(Encode, Decode, TypeInfo, Clone, Default, PartialEq, Eq, RuntimeDebug)]
pub struct ConstantProductPoolInfo<AccountId, AssetId> {
	/// Owner of pool
	pub owner: AccountId,
	/// AssetId of LP token,
	pub lp_token: AssetId,
	/// Amount of the fee pool charges for the exchange
	pub fee: Permill,
}

#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum DexRouteNode<PoolId> {
	Curve(PoolId),
	Uniswap(PoolId),
}

/// Describes route for DEX.
/// `Direct` gives vector of pool_id to use as router.
#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, RuntimeDebug)]
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
	/// Exchange `dx` of given `asset_pair` to get `dy`.
	fn exchange(
		who: &AccountId,
		asset_pair: CurrencyPair<AssetId>,
		dx: Balance,
	) -> Result<Balance, DispatchError>;
}
