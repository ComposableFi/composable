use assets_runtime_api::AssetsRuntimeApi;
use codec::Codec;
use composable_support::rpc_helpers::{SafeRpcWrapper, SafeRpcWrapperType};
use jsonrpc_core::{Error as RpcError, ErrorCode, Result as RpcResult};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use sp_std::sync::Arc;

#[rpc]
pub trait AssetsApi<BlockHash, AssetId, AccountId, Balance>
where
	AssetId: SafeRpcWrapperType,
	Balance: SafeRpcWrapperType,
{
	#[rpc(name = "assets_balanceOf")]
	fn balance_of(
		&self,
		currency: SafeRpcWrapper<AssetId>,
		account: AccountId,
		at: Option<BlockHash>,
	) -> RpcResult<SafeRpcWrapper<Balance>>;
}

pub struct Assets<C, Block> {
	client: Arc<C>,
	_marker: sp_std::marker::PhantomData<Block>,
}

impl<C, M> Assets<C, M> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block, AssetId, AccountId, Balance>
	AssetsApi<<Block as BlockT>::Hash, AssetId, AccountId, Balance>
	for Assets<C, (Block, AssetId, AccountId, Balance)>
where
	Block: BlockT,
	AssetId: Codec + Send + Sync + 'static + SafeRpcWrapperType,
	AccountId: Codec + Send + Sync + 'static,
	Balance: Send + Sync + 'static + SafeRpcWrapperType,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: AssetsRuntimeApi<Block, AssetId, AccountId, Balance>,
{
	fn balance_of(
		&self,
		asset_id: SafeRpcWrapper<AssetId>,
		account_id: AccountId,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<SafeRpcWrapper<Balance>> {
		let api = self.client.runtime_api();

		let at = BlockId::hash(at.unwrap_or_else(|| {
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash
		}));

		let runtime_api_result = api.balance_of(&at, asset_id, account_id);
		// TODO(benluelo): Review what error message & code to use
		runtime_api_result.map_err(|e| {
			RpcError {
				code: ErrorCode::ServerError(9876), // No real reason for this value
				message: "Something wrong".into(),
				data: Some(format!("{:?}", e).into()),
			}
		})
	}
}
