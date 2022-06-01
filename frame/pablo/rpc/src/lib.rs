use codec::Codec;
use composable_support::rpc_helpers::SafeRpcWrapper;
use composable_traits::dex::PriceAggregate;
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
use sp_std::sync::Arc;

#[rpc(client, server)]
pub trait PabloApi<BlockHash, PoolId, AssetId, Balance>
where
	PoolId: FromStr + Display,
	AssetId: FromStr + Display,
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
		at: Option<BlockHash>,
	) -> RpcResult<(SafeRpcWrapper<Balance>, SafeRpcWrapper<Balance>)>;
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

impl<C, Block, PoolId, AssetId, Balance>
	PabloApiServer<<Block as BlockT>::Hash, PoolId, AssetId, Balance>
	for Pablo<C, (Block, PoolId, AssetId, Balance)>
where
	Block: BlockT,
	PoolId: Send + Sync + 'static + Codec + FromStr + Display,
	AssetId: Send + Sync + 'static + Codec + FromStr + Display,
	Balance: Send + Sync + 'static + Codec + FromStr + Display,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: PabloRuntimeApi<Block, PoolId, AssetId, Balance>,
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
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<(SafeRpcWrapper<Balance>, SafeRpcWrapper<Balance>)> {
		let api = self.client.runtime_api();

		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		// calling ../../runtime-api
		let runtime_api_result = api.redeemable_assets_for_given_lp_tokens(&at, pool_id, lp_amount);
		runtime_api_result.map_err(|e| {
			RpcError::Call(CallError::Custom(ErrorObject::owned(
				9876,
				"Something wrong",
				Some(format!("{:?}", e)),
			)))
		})
	}
}
