use crate::{currency::BalanceLike, defi::CurrencyPair};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	traits::{tokens::AssetId as AssetIdLike, Get},
	BoundedVec, RuntimeDebug,
};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{DispatchError, Permill};
use sp_std::{collections::btree_map::BTreeMap, ops::Mul, vec::Vec};

/// Trait for automated market maker.
pub trait Amm {
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

	fn lp_token(pool_id: Self::PoolId) -> Result<Self::AssetId, DispatchError>;

	/// Returns the amount of base & quote asset redeemable for given amount of lp token.
	fn redeemable_assets_for_lp_tokens(
		pool_id: Self::PoolId,
		lp_amount: Self::Balance,
		min_expected_amounts: BTreeMap<Self::AssetId, Self::Balance>,
	) -> Result<RedeemableAssets<Self::AssetId, Self::Balance>, DispatchError>
	where
		Self::AssetId: sp_std::cmp::Ord;

	/// Simulate add_liquidity computations, on success returns the amount of LP tokens
	/// that would be received by adding the given amounts of base and quote.
	fn simulate_add_liquidity(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		amounts: BTreeMap<Self::AssetId, Self::Balance>,
	) -> Result<Self::Balance, DispatchError>
	where
		Self::AssetId: sp_std::cmp::Ord;

	/// Simulate remove_liquidity computations, on success returns the amount of base/quote assets
	/// that would be received by removing the given amounts of lp tokens.
	fn simulate_remove_liquidity(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		lp_amount: Self::Balance,
		min_expected_amounts: BTreeMap<Self::AssetId, Self::Balance>,
	) -> Result<RemoveLiquiditySimulationResult<Self::AssetId, Self::Balance>, DispatchError>
	where
		Self::AssetId: sp_std::cmp::Ord;

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
		min_receive: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::Balance, DispatchError>;

	/// Sell given `amount` of given asset to the pool.
	/// In sell user specifies `amount` of asset he/she wants to exchange to get other asset.
	fn sell(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		asset_id: Self::AssetId,
		amount: Self::Balance,
		min_receive: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::Balance, DispatchError>;

	/// Deposit coins into the pool
	/// `amounts` - list of amounts of coins to deposit,
	/// `min_mint_amount` - minimum amount of LP tokens to mint from the deposit.
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

// TODO: Perhaps we need a way to not have a max reward for a pool.
pub const MAX_REWARDS: u128 = 100_000_000_000_000_000_000_000_u128;
pub const REWARD_PERCENTAGE: u32 = 10;

/// Pool Fees
#[derive(
	Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Default, PartialEq, Eq, Copy, RuntimeDebug,
)]
pub struct Fee<AssetId, Balance> {
	// total fee
	pub fee: Balance,
	/// Amount of the fee pool charges for the exchange, this goes to liquidity providers.
	pub lp_fee: Balance,
	/// Amount of the fee that goes out to the owner of the pool
	pub owner_fee: Balance,
	/// Amount of the protocol fees(for PBLO holders) out of owner_fees.
	pub protocol_fee: Balance,
	/// assetId of the fees
	pub asset_id: AssetId,
}

impl<AssetId: AssetIdLike, Balance: BalanceLike> Fee<AssetId, Balance> {
	pub fn zero(asset_id: AssetId) -> Self {
		Fee {
			fee: Balance::zero(),
			lp_fee: Balance::zero(),
			owner_fee: Balance::zero(),
			protocol_fee: Balance::zero(),
			asset_id,
		}
	}
}

/// Pool Fee Config
#[derive(
	Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Default, PartialEq, Eq, Copy, RuntimeDebug,
)]
pub struct FeeConfig {
	/// Amount of the fee pool charges for the exchange, this goes to liquidity provider.
	pub fee_rate: Permill,
	/// Amount of the fee that goes out to the owner of the pool
	pub owner_fee_rate: Permill,
	/// Amount of the protocol fees(for PBLO holders) out of owner_fees.
	pub protocol_fee_rate: Permill,
}

impl FeeConfig {
	pub fn zero() -> Self {
		FeeConfig {
			fee_rate: Permill::zero(),
			owner_fee_rate: Permill::zero(),
			protocol_fee_rate: Permill::zero(),
		}
	}

	pub fn default_from(trading_fee: Permill) -> Self {
		FeeConfig {
			fee_rate: trading_fee,
			owner_fee_rate: Permill::from_percent(20),
			protocol_fee_rate: Permill::from_percent(100),
		}
	}

	pub fn calculate_fees<AssetId: AssetIdLike, Balance: BalanceLike>(
		&self,
		asset_id: AssetId,
		amount: Balance,
	) -> Fee<AssetId, Balance> {
		let fee: Balance = self.fee_rate.mul_floor(amount);
		let owner_fee: Balance = self.owner_fee_rate.mul_floor(fee);
		let protocol_fee: Balance = self.protocol_fee_rate.mul_floor(owner_fee);
		Fee {
			fee,
			// safe as the values are calculated as per million
			lp_fee: fee - owner_fee,
			owner_fee: owner_fee - protocol_fee,
			protocol_fee,
			asset_id,
		}
	}
}

impl Mul<Permill> for FeeConfig {
	type Output = Self;

	fn mul(self, rhs: Permill) -> Self::Output {
		FeeConfig {
			fee_rate: self.fee_rate.mul(rhs),
			owner_fee_rate: self.owner_fee_rate,
			protocol_fee_rate: self.protocol_fee_rate,
		}
	}
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

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Default, PartialEq, Eq, RuntimeDebug)]
pub struct ConstantProductPoolInfo<AccountId, AssetId> {
	/// Owner of pool
	pub owner: AccountId,
	/// Swappable assets
	pub pair: CurrencyPair<AssetId>,
	/// AssetId of LP token
	pub lp_token: AssetId,
	/// Amount of the fee pool charges for the exchange
	pub fee_config: FeeConfig,
	/// The weight of the base asset. Must hold `1 = base_weight + quote_weight`
	pub base_weight: Permill,
	/// The weight of the quote asset. Must hold `1 = base_weight + quote_weight`
	pub quote_weight: Permill,
}

/// Describes route for DEX.
/// `Direct` gives vector of pool_id to use as router.
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum DexRoute<PoolId, MaxHops: Get<u32>> {
	Direct(BoundedVec<PoolId, MaxHops>),
}

pub trait DexRouter<AssetId, PoolId, Balance, MaxHops> {
	/// If route is `None` then delete existing entry for `asset_pair`
	/// If route is `Some` and no entry exist for `asset_pair` then add new entry
	/// else update existing entry.
	fn update_route(
		asset_pair: CurrencyPair<AssetId>,
		route: Option<BoundedVec<PoolId, MaxHops>>,
	) -> Result<(), DispatchError>;
	/// If route exist return `Some((Vec<PoolId>, bool))`, else `None`.
	/// boolean in pair indicates if route needs to be used in reversed direction.
	fn get_route(asset_pair: CurrencyPair<AssetId>) -> Option<(Vec<PoolId>, bool)>;
}

/// Aggregated prices for a given base/quote currency pair in a pool.
#[derive(RuntimeDebug, Encode, Decode, Default, Clone, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PriceAggregate<PoolId, AssetId, Balance> {
	pub pool_id: PoolId,
	pub base_asset_id: AssetId,
	pub quote_asset_id: AssetId,
	pub spot_price: Balance, // prices based on any other stat such as TWAP goes here..
}

/// RedeemableAssets for given amount of lp tokens.
#[derive(RuntimeDebug, Encode, Decode, Default, Clone, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct RedeemableAssets<AssetId, Balance>
where
	AssetId: Ord,
{
	pub assets: BTreeMap<AssetId, Balance>,
}

/// RemoveLiquiditySimulationResult for given amount of lp tokens.
#[derive(RuntimeDebug, Encode, Decode, Default, Clone, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct RemoveLiquiditySimulationResult<AssetId, Balance>
where
	AssetId: Ord,
{
	pub assets: BTreeMap<AssetId, Balance>,
}

#[cfg(test)]
mod tests {
	use crate::dex::{Fee, FeeConfig};
	use sp_arithmetic::Permill;
	use std::ops::Mul;

	#[test]
	fn calculate_fee() {
		const UNIT: u128 = 1_000_000_000_000_u128;
		let amount = 1_000_000_u128 * UNIT;
		let f = FeeConfig {
			fee_rate: Permill::from_percent(1),
			owner_fee_rate: Permill::from_percent(1),
			protocol_fee_rate: Permill::from_percent(1),
		};
		assert_eq!(
			f.calculate_fees(1, amount),
			Fee {
				fee: 10000 * UNIT,
				lp_fee: 9900 * UNIT,
				owner_fee: 99 * UNIT,
				protocol_fee: 1 * UNIT,
				asset_id: 1
			}
		);

		let f_default = FeeConfig::default_from(Permill::from_perthousand(3));
		assert_eq!(
			f_default.calculate_fees(1, amount),
			Fee {
				fee: 3000 * UNIT,
				lp_fee: 2400 * UNIT,
				owner_fee: 0,
				protocol_fee: 600 * UNIT,
				asset_id: 1
			}
		);

		let f2 = f.mul(Permill::from_percent(50));
		assert_eq!(
			f2.calculate_fees(1, amount),
			Fee {
				fee: 5000000000000000,
				lp_fee: 4950000000000000,
				owner_fee: 49500000000000,
				protocol_fee: 500000000000,
				asset_id: 1
			}
		);
	}
}
