use crate::{currency::BalanceLike, defi::CurrencyPair, prelude::*};

use frame_support::{
	ensure,
	traits::{tokens::AssetId as AssetIdLike, Get},
	BoundedVec, CloneNoBound, EqNoBound, PartialEqNoBound, RuntimeDebug, RuntimeDebugNoBound,
};

use sp_runtime::{
	helpers_128bit::multiply_by_rational_with_rounding, traits::Zero, BoundedBTreeMap,
	DispatchError, Permill, Rational128,
};
use sp_std::collections::btree_map::BTreeMap;

pub type PoolId = Uint128;

/// Returns `ExecuteMsgResponse`
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema, QueryResponses))]
pub enum ExecuteMsg {
	/// Like Osmosis MsgJoinPool
	#[cfg_attr(feature = "std", returns(AddLiquidityResponse))]
	AddLiquidity { pool_id: PoolId, assets: Vec<Coin>, min_mint_amount: Uint128, keep_alive: bool },
	/// Like Osmosis MsgExitPool
	#[cfg_attr(feature = "std", returns(RemoveLiquidityResponse))]
	RemoveLiquidity { pool_id: PoolId, lp_amount: Uint128, min_receive: Vec<Coin> },
	/// Like Osmosis MsgSwapExactAmountOut
	#[cfg_attr(feature = "std", returns(BuyResponse))]
	Buy { pool_id: PoolId, in_asset_id: String, out_asset: Coin, keep_alive: bool },
	/// Like Osmosis MsgSwapExactAmountIn
	#[cfg_attr(feature = "std", returns(SwapResponse))]
	Swap { pool_id: PoolId, in_asset: Coin, min_receive: Coin, keep_alive: bool },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct AddLiquidityResponse {
	lp_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct RemoveLiquidityResponse {
	assets: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct BuyResponse {
	value: Coin,
	fee: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct SwapResponse {
	value: Coin,
	fee: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct AssetsResponse {
	assets: Vec<String>,
	fee: (Uint64, Uint64),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct LpTokenResponse {
	lp_token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct SwapResultResponse {
	value: Coin,
	fee: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct RedeemableAssetsForLpTokensResponse {
	assets: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct SimulateAddLiquidityResponse {
	amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct SimulateRemoveLiquidityResponse {
	pool_id: PoolId,
	amounts: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct SpotPriceResponse {
	base_asset: Coin,
	quote_asset_id: String,
	calculate_with_fees: bool,
}

/// Returns `QueryMsgResponse`
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema, QueryResponses))]
pub enum QueryMsg {
	/// total supply of any assets can be asked from bank as we share all tokens here
	#[cfg_attr(feature = "std", returns(AssetsResponse))]
	Assets { pool_id: PoolId },
	#[cfg_attr(feature = "std", returns(SpotPriceResponse))]
	SpotPrice { base_asset: Coin, quote_asset_id: String, calculate_with_fees: bool },
	#[cfg_attr(feature = "std", returns(LpTokenResponse))]
	LpToken { pool_id: PoolId },
	#[cfg_attr(feature = "std", returns(RedeemableAssetsForLpTokensResponse))]
	RedeemableAssetsForLpTokens { pool_id: PoolId, lp_amount: Uint128 },
	#[cfg_attr(feature = "std", returns(SimulateAddLiquidityResponse))]
	SimulateAddLiquidity { pool_id: PoolId, amounts: Vec<Coin> },
	#[cfg_attr(feature = "std", returns(SimulateRemoveLiquidityResponse))]
	SimulateRemoveLiquidity { pool_id: PoolId, lp_amount: Uint128, min_amount: Vec<Coin> },
}

/// Specifies and amount together with the asset ID of the amount.
#[derive(
	Encode,
	Decode,
	MaxEncodedLen,
	TypeInfo,
	Clone,
	PartialEq,
	Eq,
	Copy,
	RuntimeDebug,
	Serialize,
	Deserialize,
)]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct AssetAmount<AssetId, Balance> {
	pub asset_id: AssetId,
	pub amount: Balance,
}

impl<AssetId, Balance> AssetAmount<AssetId, Balance> {
	pub fn new(asset_id: AssetId, amount: Balance) -> Self {
		Self { asset_id, amount }
	}
}

/// The (expected or executed) result of a swap operation.
#[derive(
	Encode,
	Decode,
	MaxEncodedLen,
	TypeInfo,
	Clone,
	PartialEq,
	Eq,
	Copy,
	RuntimeDebug,
	Serialize,
	Deserialize,
)]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct SwapResult<AssetId, Balance> {
	pub value: AssetAmount<AssetId, Balance>,
	pub fee: AssetAmount<AssetId, Balance>,
}

impl<AssetId, Balance> SwapResult<AssetId, Balance> {
	pub fn new(
		value_asset_id: AssetId,
		value: Balance,
		fee_asset_id: AssetId,
		fee: Balance,
	) -> Self {
		Self {
			value: AssetAmount::new(value_asset_id, value),
			fee: AssetAmount::new(fee_asset_id, fee),
		}
	}
}

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

	/// Retrieves the pool assets and their weights.
	fn assets(pool_id: Self::PoolId) -> Result<BTreeMap<Self::AssetId, Permill>, DispatchError>;

	fn lp_token(pool_id: Self::PoolId) -> Result<Self::AssetId, DispatchError>;

	/// Returns the amount of base & quote asset redeemable for given amount of lp token.
	fn redeemable_assets_for_lp_tokens(
		pool_id: Self::PoolId,
		lp_amount: Self::Balance,
	) -> Result<BTreeMap<Self::AssetId, Self::Balance>, DispatchError>
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
		min_amounts: BTreeMap<Self::AssetId, Self::Balance>,
	) -> Result<BTreeMap<Self::AssetId, Self::Balance>, DispatchError>
	where
		Self::AssetId: sp_std::cmp::Ord;

	/// Get pure exchange value for given units of "in" given asset.
	/// `pool_id` the pool containing the `asset_id`.
	/// `base_asset` the amount of `asset_id` the user wants to swap.
	/// `quote_asset_id` the asset the user is interested in.
	fn spot_price(
		pool_id: Self::PoolId,
		base_asset: AssetAmount<Self::AssetId, Self::Balance>,
		quote_asset_id: Self::AssetId,
		calculate_with_fees: bool,
	) -> Result<SwapResult<Self::AssetId, Self::Balance>, DispatchError>;

	/// Deposit coins into the pool
	/// `amounts` - list of amounts of coins to deposit,
	/// `min_mint_amount` - minimum amount of LP tokens to mint from the deposit.
	fn add_liquidity(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		assets: BTreeMap<Self::AssetId, Self::Balance>,
		min_mint_amount: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::Balance, DispatchError>;

	/// Withdraw coins from the pool.
	/// Withdrawal amount are based on current deposit ratios.
	/// `amount` - quantity of LP tokens to burn in the withdrawal,
	/// `min_amounts` - minimum amounts of underlying coins to receive.
	fn remove_liquidity(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		lp_amount: Self::Balance,
		min_receive: BTreeMap<Self::AssetId, Self::Balance>,
	) -> Result<BTreeMap<Self::AssetId, Self::Balance>, DispatchError>;

	/// Buy given `amount` of given asset from the pool.
	/// In buy user does not know how much assets he/she has to exchange to get desired amount.
	fn do_buy(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		in_asset_id: Self::AssetId,
		out_asset: AssetAmount<Self::AssetId, Self::Balance>,
		keep_alive: bool,
	) -> Result<SwapResult<Self::AssetId, Self::Balance>, DispatchError>;

	/// Perform an exchange effectively trading the in_asset against the min_receive one.
	fn do_swap(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		in_asset: AssetAmount<Self::AssetId, Self::Balance>,
		min_receive: AssetAmount<Self::AssetId, Self::Balance>,
		keep_alive: bool,
	) -> Result<SwapResult<Self::AssetId, Self::Balance>, DispatchError>;
}

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

	/// Calculates the fee distribution
	///
	/// # Parameters
	/// * asset_id - The asset ID that the fee will be paid in
	/// * fee - The total fee taken from the transaction
	pub fn calculate_fees<AssetId: AssetIdLike, Balance: BalanceLike>(
		&self,
		asset_id: AssetId,
		fee: Balance,
	) -> Fee<AssetId, Balance> {
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

/// Most basic representation of an AMM pool possible with extensibility for future cases. Any AMM
/// implementation should embed this to inherit the basics.
#[derive(
	Encode,
	Decode,
	MaxEncodedLen,
	TypeInfo,
	CloneNoBound,
	Default,
	PartialEqNoBound,
	EqNoBound,
	RuntimeDebugNoBound,
)]
#[scale_info(skip_type_params(MaxAssets))]
pub struct BasicPoolInfo<
	AccountId: Clone + PartialEq + Debug,
	AssetId: Ord + Clone + Debug,
	MaxAssets: Get<u32>,
> {
	/// Owner of pool
	pub owner: AccountId,
	/// Swappable assets with their normalized(sum of weights = 1) weights
	pub assets_weights: BoundedBTreeMap<AssetId, Permill, MaxAssets>,
	/// AssetId of LP token
	pub lp_token: AssetId,
	/// Amount of the fee pool charges for the exchange
	pub fee_config: FeeConfig,
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

#[cfg(test)]
mod tests {
	use crate::dex::{Fee, FeeConfig};
	use sp_arithmetic::Permill;
	use std::ops::Mul;

	#[test]
	fn calculate_fee() {
		let total_fee: u128 = 10_000_000_000;
		let f = FeeConfig {
			fee_rate: Permill::from_percent(1),
			owner_fee_rate: Permill::from_percent(1),
			protocol_fee_rate: Permill::from_percent(1),
		};
		assert_eq!(
			f.calculate_fees(1, total_fee),
			Fee {
				fee: total_fee,
				lp_fee: 9_900_000_000,
				owner_fee: 99_000_000,
				protocol_fee: 1_000_000,
				asset_id: 1
			}
		);

		let f_default = FeeConfig::default_from(Permill::from_perthousand(3));
		assert_eq!(
			f_default.calculate_fees(1, total_fee),
			Fee {
				fee: 10_000_000_000,
				lp_fee: 8_000_000_000,
				owner_fee: 0,
				protocol_fee: 2_000_000_000,
				asset_id: 1
			}
		);

		let f2 = f.mul(Permill::from_percent(50));
		assert_eq!(
			f2.calculate_fees(1, total_fee),
			Fee {
				fee: 10_000_000_000,
				lp_fee: 9_900_000_000,
				owner_fee: 99_000_000,
				protocol_fee: 1_000_000,
				asset_id: 1
			}
		);
	}
}

#[derive(
	Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Default, PartialEq, Eq, Copy, RuntimeDebug,
)]
pub struct AssetDepositInfo<AssetId> {
	pub asset_id: AssetId,
	pub deposit_amount: u128,
	pub existing_balance: u128,
	pub asset_weight: Permill,
}

impl<AssetId> AssetDepositInfo<AssetId> {
	pub fn get_deposit_ratio(&self) -> Rational128 {
		Rational128::from(self.deposit_amount, self.existing_balance)
	}

	pub fn cmp_by_deposit_ratio(&self, other: Self) -> Ordering {
		self.get_deposit_ratio().cmp(&other.get_deposit_ratio())
	}
}

/// Normalizes a list of asset deposits to the smallest ratio of all of the contained assets.
pub fn normalize_asset_deposit_infos_to_min_ratio<AssetId: Debug + Copy>(
	// REVIEW(ben,connor): Maybe make this a BiBoundedVec? Would remove the need for the custom
	// error type as well.
	mut asset_deposit_infos: Vec<AssetDepositInfo<AssetId>>,
) -> Result<Vec<AssetDepositInfo<AssetId>>, AssetDepositNormalizationError> {
	// at least 2 assets are required to normalize
	ensure!(asset_deposit_infos.len() > 1, AssetDepositNormalizationError::NotEnoughAssets);

	let smallest_ratio = asset_deposit_infos
		.iter()
		.map(|adi| adi.get_deposit_ratio())
		.min()
		.expect("at least 2 items are present in the vec as per the check above; qed;");

	for asset_deposit_info in &mut asset_deposit_infos {
		debug_assert!(
			!asset_deposit_info.existing_balance.is_zero(),
			"balance for asset {:?} was zero when it should not have been; \
			this will result in a `DivideByZero` error in production code.",
			asset_deposit_info.asset_id
		);

		asset_deposit_info.deposit_amount = multiply_by_rational_with_rounding(
			asset_deposit_info.existing_balance,
			smallest_ratio.n(),
			smallest_ratio.d(),
			// amount out will be less than the maximum allowed, so round up
			sp_arithmetic::Rounding::Up,
		)
		.ok_or(AssetDepositNormalizationError::ArithmeticOverflow)?;
	}

	Ok(asset_deposit_infos)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetDepositNormalizationError {
	ArithmeticOverflow,
	NotEnoughAssets,
}

#[cfg(test)]
mod test_asset_deposit_normalization {
	use sp_runtime::AccountId32;

use super::*;

	fn generate_asset_deposit_infos<const N: usize>(
		adi_inputs: [(u128, u128); N],
	) -> Vec<AssetDepositInfo<u128>> {
		adi_inputs
			.into_iter()
			.enumerate()
			.map(|(id, (deposit, balance))| AssetDepositInfo {
				asset_id: id as u128,
				deposit_amount: deposit,
				existing_balance: balance,
				asset_weight: Permill::from_rational::<u32>(1, N as u32),
			})
			.collect()
	}

	#[test]
	fn no_assets_error() {
		use sp_runtime::traits::AccountIdConversion;
		let account_id : AccountId32 = frame_support::PalletId(*b"pal_pblo").into_account_truncating();
		todo!("{:?}", account_id.to_string());
		// assert_eq!(
		// 	normalize_asset_deposit_infos_to_min_ratio::<u128>(vec![]),
		// 	Err(AssetDepositNormalizationError::NotEnoughAssets)
		// );
	}

	#[test]
	fn only_one_asset_error() {
		assert_eq!(
			normalize_asset_deposit_infos_to_min_ratio::<u128>(generate_asset_deposit_infos([(
				100, 100
			)])),
			Err(AssetDepositNormalizationError::NotEnoughAssets)
		);
	}

	#[test]
	fn two_assets_works() {
		assert_eq!(
			normalize_asset_deposit_infos_to_min_ratio::<u128>(generate_asset_deposit_infos([
				(300, 100),
				(300, 200),
			])),
			Ok(generate_asset_deposit_infos([(150, 100), (300, 200)]))
		);
	}

	#[test]
	fn more_than_2_assets_works() {
		assert_eq!(
			normalize_asset_deposit_infos_to_min_ratio::<u128>(generate_asset_deposit_infos([
				(300, 100),
				(300, 200),
				(200, 100),
				(400, 600),
			])),
			Ok(generate_asset_deposit_infos([(67, 100), (134, 200), (67, 100), (400, 600)]))
		);
	}
}
