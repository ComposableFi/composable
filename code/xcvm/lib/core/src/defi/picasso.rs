use crate::prelude::*;

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
	pub lp_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct RemoveLiquidityResponse {
	pub assets: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct BuyResponse {
	pub value: Coin,
	pub fee: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct SwapResponse {
	pub value: Coin,
	pub fee: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct AssetsResponse {
	pub assets: Vec<(String, (Uint64, Uint64))>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct LpTokenResponse {
	pub lp_token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct SwapResultResponse {
	pub value: Coin,
	pub fee: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct RedeemableAssetsForLpTokensResponse {
	pub assets: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct SimulateAddLiquidityResponse {
	pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct SimulateRemoveLiquidityResponse {
	pub amounts: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct SpotPriceResponse {
	pub value: Coin,
	pub fee: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema, QueryResponses))]
pub enum QueryMsg {
	/// total supply of any assets can be asked from Bank as we share all tokens here
	#[cfg_attr(feature = "std", returns(AssetsResponse))]
	Assets { pool_id: PoolId },
	#[cfg_attr(feature = "std", returns(SpotPriceResponse))]
	SpotPrice {
		pool_id: PoolId,
		base_asset: Coin,
		quote_asset_id: String,
		calculate_with_fees: bool,
	},
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