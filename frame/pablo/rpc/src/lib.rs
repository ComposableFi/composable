use codec::Codec;
use pablo_runtime_api::PabloRuntimeApi;
use composable_support::rpc_helpers::{SafeRpcWrapper, SafeRpcWrapperType};
use composable_traits::dex::PriceAggregate;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result as RpcResult};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use sp_std::sync::Arc;

#[rpc]
pub trait PabloApi<BlockHash, PoolId, AssetId, Balance>
where
	PoolId: Codec,
	AssetId: Codec,
	Balance: Codec,
{
	#[rpc(name = "pablo_pricesFor")]
	fn prices_for(
		&self,
		pool_id: PoolId,
		base_asset_id: AssetId,
		quote_asset_id: AssetId,
		amount: Balance,
		at: Option<BlockHash>,
	) -> RpcResult<PriceAggregate<PoolId, AssetId, Balance>>;
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
	PabloApi<<Block as BlockT>::Hash, PoolId, AssetId, Balance>
	for Pablo<C, (Block, PoolId, AssetId, Balance)>
where
	Block: BlockT,
	PoolId: Send + Sync + 'static + Codec,
	AssetId: Send + Sync + 'static + Codec,
	Balance: Send + Sync + 'static + Codec,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: PabloRuntimeApi<Block, PoolId, AssetId, Balance>,
{
	fn prices_for(
		&self,
		pool_id: PoolId,
		base_asset_id: AssetId,
		quote_asset_id: AssetId,
		amount: Balance,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<PriceAggregate<PoolId, AssetId, Balance>> {
		let api = self.client.runtime_api();

		let at = BlockId::hash(at.unwrap_or_else(|| {
			self.client.info().best_hash
		}));

		let runtime_api_result = api.prices_for(&at, pool_id, base_asset_id, quote_asset_id, amount);
		runtime_api_result.map_err(|e| {
			RpcError {
				code: ErrorCode::ServerError(9876), // No real reason for this value
				message: "Something wrong".into(),
				data: Some(format!("{:?}", e).into()),
			}
		})
	}
}
