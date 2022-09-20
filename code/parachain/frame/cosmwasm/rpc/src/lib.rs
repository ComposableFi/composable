use codec::Codec;
use composable_support::rpc_helpers::SafeRpcWrapper;
use core::{fmt::Display, str::FromStr};
use cosmwasm_runtime_api::CosmwasmRuntimeApi;
use jsonrpsee::{
	core::{Error as RpcError, RpcResult},
	proc_macros::rpc,
	types::{error::CallError, ErrorObject},
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use sp_std::sync::Arc;

#[rpc(client, server)]
pub trait CosmwasmApi<BlockHash, AccountId, QueryRequest, Binary>
where
	AccountId: FromStr + Display,
{
	#[method(name = "cosmwasm_query")]
	fn query(
		&self,
		contract: SafeRpcWrapper<AccountId>,
		gas: SafeRpcWrapper<u64>,
		query_request: QueryRequest,
		at: Option<BlockHash>,
	) -> RpcResult<Binary>;
}

pub struct Cosmwasm<C, Block> {
	client: Arc<C>,
	_marker: sp_std::marker::PhantomData<Block>,
}

impl<C, M> Cosmwasm<C, M> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block, AccountId, QueryRequest, Binary>
	CosmwasmApiServer<<Block as BlockT>::Hash, AccountId, QueryRequest, Binary>
	for Cosmwasm<C, (Block, AccountId, QueryRequest, Binary)>
where
	Block: BlockT,
	AccountId: Send + Sync + 'static + Codec + FromStr + Display,
	QueryRequest: Send + Sync + 'static + Codec,
	Binary: Send + Sync + 'static + Codec,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: CosmwasmRuntimeApi<Block, AccountId, QueryRequest, Binary>,
{
	fn query(
		&self,
		contract: SafeRpcWrapper<AccountId>,
		gas: SafeRpcWrapper<u64>,
		query_request: QueryRequest,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Binary> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));
		let runtime_api_result = api.query(&at, contract.0, gas.0, query_request);
		runtime_api_result.map_err(|e| {
			RpcError::Call(CallError::Custom(ErrorObject::owned(
				9876,
				"Something wrong",
				Some(format!("{:?}", e)),
			)))
		})
	}
}
