use codec::Codec;
use composable_support::rpc_helpers::SafeRpcWrapper;
use composable_traits::defi::Rate;
use core::{fmt::Display, str::FromStr};
use jsonrpsee::{
	core::{Error as RpcError, RpcResult},
	proc_macros::rpc,
	types::{error::CallError, ErrorObject},
};
use lending_runtime_api::LendingRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use sp_std::sync::Arc;

#[rpc(client, server)]
pub trait LendingApi<BlockHash, MarketId>
where
	MarketId: FromStr + Display,
{
	#[method(name = "lending_currentInterestRate")]
	fn current_interest_rate(
		&self,
		market_id: SafeRpcWrapper<MarketId>,
		at: Option<BlockHash>,
	) -> RpcResult<SafeRpcWrapper<Rate>>;
}

pub struct Lending<C, Block> {
	client: Arc<C>,
	_marker: sp_std::marker::PhantomData<Block>,
}

impl<C, M> Lending<C, M> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block, MarketId> LendingApiServer<<Block as BlockT>::Hash, MarketId>
	for Lending<C, (Block, MarketId)>
where
	Block: BlockT,
	MarketId: Send + Sync + 'static + Codec + FromStr + Display,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: LendingRuntimeApi<Block, MarketId>,
{
	fn current_interest_rate(
		&self,
		market_id: SafeRpcWrapper<MarketId>,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<SafeRpcWrapper<Rate>> {
		let api = self.client.runtime_api();

		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		// calling ../../runtime-api
		let runtime_api_result = api.current_interest_rate(&at, market_id.0);
		runtime_api_result.map_err(|e| {
			RpcError::Call(CallError::Custom(ErrorObject::owned(
				9876,
				"Something wrong",
				Some(format!("{:?}", e)),
			)))
		})
	}
}
