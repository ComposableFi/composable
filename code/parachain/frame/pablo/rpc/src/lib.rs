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
use sp_std::{cmp::Ord, collections::btree_map::BTreeMap, sync::Arc};

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

	#[method(name = "pablo_simulateAddLiquidity")]
	fn simulate_add_liquidity(
		&self,
		who: SafeRpcWrapper<AccountId>,
		pool_id: SafeRpcWrapper<PoolId>,
		amounts: BTreeMap<SafeRpcWrapper<AssetId>, SafeRpcWrapper<Balance>>,
		at: Option<BlockHash>,
	) -> RpcResult<SafeRpcWrapper<Balance>>;

	#[method(name = "pablo_simulateRemoveLiquidity")]
	fn simulate_remove_liquidity(
		&self,
		who: SafeRpcWrapper<AccountId>,
		pool_id: SafeRpcWrapper<PoolId>,
		lp_amount: SafeRpcWrapper<Balance>,
		min_expected_amounts: BTreeMap<SafeRpcWrapper<AssetId>, SafeRpcWrapper<Balance>>,
		at: Option<BlockHash>,
	) -> RpcResult<BTreeMap<SafeRpcWrapper<AssetId>, SafeRpcWrapper<Balance>>>;
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

	fn simulate_add_liquidity(
		&self,
		who: SafeRpcWrapper<AccountId>,
		pool_id: SafeRpcWrapper<PoolId>,
		amounts: BTreeMap<SafeRpcWrapper<AssetId>, SafeRpcWrapper<Balance>>,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<SafeRpcWrapper<Balance>> {
		let api = self.client.runtime_api();

		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		// calling ../../runtime-api
		let runtime_api_result = api.simulate_add_liquidity(&at, who, pool_id, amounts);
		runtime_api_result.map_err(|e| {
			RpcError::Call(CallError::Custom(ErrorObject::owned(
				9876,
				"Something wrong",
				Some(format!("{:?}", e)),
			)))
		})
	}

	fn simulate_remove_liquidity(
		&self,
		who: SafeRpcWrapper<AccountId>,
		pool_id: SafeRpcWrapper<PoolId>,
		lp_amount: SafeRpcWrapper<Balance>,
		min_expected_amounts: BTreeMap<SafeRpcWrapper<AssetId>, SafeRpcWrapper<Balance>>,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<BTreeMap<SafeRpcWrapper<AssetId>, SafeRpcWrapper<Balance>>> {
		let api = self.client.runtime_api();

		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		// calling ../../runtime-api
		let runtime_api_result =
			api.simulate_remove_liquidity(&at, who, pool_id, lp_amount, min_expected_amounts);
		runtime_api_result.map_err(|e| {
			RpcError::Call(CallError::Custom(ErrorObject::owned(
				9876,
				"Something wrong",
				Some(format!("{:?}", e)),
			)))
		})
	}
}
