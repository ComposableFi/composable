use codec::Codec;
use composable_support::rpc_helpers::SafeRpcWrapper;
use composable_traits::dex::{PriceAggregate, RedeemableAssets, RemoveLiquidityDryrunResult};
use core::{fmt::Display, str::FromStr};
use jsonrpsee::{
	core::{Error as RpcError, RpcResult},
	proc_macros::rpc,
	types::{error::CallError, ErrorObject},
};
use pablo_runtime_api::PabloRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use sp_std::{cmp::Ord, sync::Arc};

#[rpc(client, server)]
pub trait PabloApi<BlockHash, AccountId, PoolId, AssetId, Balance>
where
	AccountId: FromStr + Display,
	PoolId: FromStr + Display,
	AssetId: FromStr + Display + Ord,
	Balance: FromStr + Display,
{
	#[method(name = "pablo_pricesFor")]
	fn prices_for(
		&self,
		pool_id: SafeRpcWrapper<PoolId>,
		base_asset_id: SafeRpcWrapper<AssetId>,
		quote_asset_id: SafeRpcWrapper<AssetId>,
		amount: SafeRpcWrapper<Balance>,
		at: Option<BlockHash>,
	) -> RpcResult<
		PriceAggregate<SafeRpcWrapper<PoolId>, SafeRpcWrapper<AssetId>, SafeRpcWrapper<Balance>>,
	>;

	#[method(name = "pablo_expectedLpTokensGivenLiquidity")]
	fn expected_lp_tokens_given_liquidity(
		&self,
		pool_id: SafeRpcWrapper<PoolId>,
		base_asset_amount: SafeRpcWrapper<Balance>,
		quote_asset_amount: SafeRpcWrapper<Balance>,
		at: Option<BlockHash>,
	) -> RpcResult<SafeRpcWrapper<Balance>>;

	#[method(name = "pablo_redeemableAssetForGivenLpTokens")]
	fn redeemable_assets_for_given_lp_tokens(
		&self,
		pool_id: SafeRpcWrapper<PoolId>,
		lp_amount: SafeRpcWrapper<Balance>,
		min_base_amount: SafeRpcWrapper<Balance>,
		min_quote_amount: SafeRpcWrapper<Balance>,
		at: Option<BlockHash>,
	) -> RpcResult<RedeemableAssets<SafeRpcWrapper<AssetId>, SafeRpcWrapper<Balance>>>;

	#[method(name = "pablo_remove_liquidity_dryrun")]
	fn remove_liquidity_dryrun(
		&self,
		who: SafeRpcWrapper<AccountId>,
		pool_id: SafeRpcWrapper<PoolId>,
		lp_amount: SafeRpcWrapper<Balance>,
		min_base_amount: SafeRpcWrapper<Balance>,
		min_quote_amount: SafeRpcWrapper<Balance>,
		at: Option<BlockHash>,
	) -> RpcResult<RemoveLiquidityDryrunResult<SafeRpcWrapper<AssetId>, SafeRpcWrapper<Balance>>>;
}

pub struct Pablo<C, Block> {
	client: Arc<C>,
	_marker: sp_std::marker::PhantomData<Block>,
}

impl<C, M> Pablo<C, M> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block, AccountId, PoolId, AssetId, Balance>
	PabloApiServer<<Block as BlockT>::Hash, AccountId, PoolId, AssetId, Balance>
	for Pablo<C, (Block, AccountId, PoolId, AssetId, Balance)>
where
	Block: BlockT,
	AccountId: Send + Sync + 'static + Codec + FromStr + Display,
	PoolId: Send + Sync + 'static + Codec + FromStr + Display,
	AssetId: Send + Sync + 'static + Codec + FromStr + Display + Ord,
	Balance: Send + Sync + 'static + Codec + FromStr + Display,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: PabloRuntimeApi<Block, AccountId, PoolId, AssetId, Balance>,
{
	fn prices_for(
		&self,
		pool_id: SafeRpcWrapper<PoolId>,
		base_asset_id: SafeRpcWrapper<AssetId>,
		quote_asset_id: SafeRpcWrapper<AssetId>,
		amount: SafeRpcWrapper<Balance>,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<
		PriceAggregate<SafeRpcWrapper<PoolId>, SafeRpcWrapper<AssetId>, SafeRpcWrapper<Balance>>,
	> {
		let api = self.client.runtime_api();

		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		// calling ../../runtime-api
		let runtime_api_result =
			api.prices_for(&at, pool_id.0, base_asset_id.0, quote_asset_id.0, amount.0);
		runtime_api_result.map_err(|e| {
			RpcError::Call(CallError::Custom(ErrorObject::owned(
				9876,
				"Something wrong",
				Some(format!("{:?}", e)),
			)))
		})
	}

	fn expected_lp_tokens_given_liquidity(
		&self,
		pool_id: SafeRpcWrapper<PoolId>,
		base_asset_amount: SafeRpcWrapper<Balance>,
		quote_asset_amount: SafeRpcWrapper<Balance>,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<SafeRpcWrapper<Balance>> {
		let api = self.client.runtime_api();

		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		// calling ../../runtime-api
		let runtime_api_result = api.expected_lp_tokens_given_liquidity(
			&at,
			pool_id,
			base_asset_amount,
			quote_asset_amount,
		);
		runtime_api_result.map_err(|e| {
			RpcError::Call(CallError::Custom(ErrorObject::owned(
				9876,
				"Something wrong",
				Some(format!("{:?}", e)),
			)))
		})
	}

	fn redeemable_assets_for_given_lp_tokens(
		&self,
		pool_id: SafeRpcWrapper<PoolId>,
		lp_amount: SafeRpcWrapper<Balance>,
		min_base_amount: SafeRpcWrapper<Balance>,
		min_quote_amount: SafeRpcWrapper<Balance>,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<RedeemableAssets<SafeRpcWrapper<AssetId>, SafeRpcWrapper<Balance>>> {
		let api = self.client.runtime_api();

		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		// calling ../../runtime-api
		let runtime_api_result = api.redeemable_assets_for_given_lp_tokens(
			&at,
			pool_id,
			lp_amount,
			min_base_amount,
			min_quote_amount,
		);
		runtime_api_result.map_err(|e| {
			RpcError::Call(CallError::Custom(ErrorObject::owned(
				9876,
				"Something wrong",
				Some(format!("{:?}", e)),
			)))
		})
	}

	fn remove_liquidity_dryrun(
		&self,
		who: SafeRpcWrapper<AccountId>,
		pool_id: SafeRpcWrapper<PoolId>,
		lp_amount: SafeRpcWrapper<Balance>,
		min_base_amount: SafeRpcWrapper<Balance>,
		min_quote_amount: SafeRpcWrapper<Balance>,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<RemoveLiquidityDryrunResult<SafeRpcWrapper<AssetId>, SafeRpcWrapper<Balance>>> {
		let api = self.client.runtime_api();

		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		// calling ../../runtime-api
		let runtime_api_result = api.remove_liquidity_dryrun(
			&at,
			who,
			pool_id,
			lp_amount,
			min_base_amount,
			min_quote_amount,
		);
		runtime_api_result.map_err(|e| {
			RpcError::Call(CallError::Custom(ErrorObject::owned(
				9876,
				"Something wrong",
				Some(format!("{:?}", e)),
			)))
		})
	}
}
