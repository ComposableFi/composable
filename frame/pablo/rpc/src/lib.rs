use codec::Codec;
use composable_support::rpc_helpers::SafeRpcWrapper;
use composable_traits::dex::PriceAggregate;
use core::{fmt::Display, str::FromStr};
use jsonrpc_core::{Error as RpcError, ErrorCode, Result as RpcResult};
use jsonrpc_derive::rpc;
use pablo_runtime_api::PabloRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use sp_std::sync::Arc;

#[rpc]
pub trait PabloApi<BlockHash, PoolId, AssetId, Balance>
where
	PoolId: FromStr + Display,
	AssetId: FromStr + Display,
	Balance: FromStr + Display,
{
	#[rpc(name = "pablo_pricesFor")]
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

	#[rpc(name = "pablo_expectedLpTokensGivenLiquidity")]
	fn expected_lp_tokens_given_liquidity(
		&self,
		pool_id: SafeRpcWrapper<PoolId>,
		base_asset_amount: SafeRpcWrapper<Balance>,
		quote_asset_amount: SafeRpcWrapper<Balance>,
		at: Option<BlockHash>,
	) -> RpcResult<SafeRpcWrapper<Balance>>;
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

impl<C, Block, PoolId, AssetId, Balance> PabloApi<<Block as BlockT>::Hash, PoolId, AssetId, Balance>
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
			RpcError {
				code: ErrorCode::ServerError(9876), // No real reason for this value
				message: "Something wrong".into(),
				data: Some(format!("{:?}", e).into()),
			}
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
			RpcError {
				code: ErrorCode::ServerError(9876), // No real reason for this value
				message: "Something wrong".into(),
				data: Some(format!("{:?}", e).into()),
			}
		})
	}
}
