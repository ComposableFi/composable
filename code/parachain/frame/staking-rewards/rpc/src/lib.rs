use codec::Codec;
use composable_support::rpc_helpers::SafeRpcWrapper;
use composable_traits::dex::PriceAggregate;
use core::{fmt::Display, str::FromStr};
use jsonrpsee::{
	core::{Error as RpcError, RpcResult},
	proc_macros::rpc,
	types::{error::CallError, ErrorObject},
};
use pallet_staking_rewards_runtime_api::ClaimableAmountError;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use sp_std::{cmp::Ord, collections::btree_map::BTreeMap, sync::Arc};
use staking_rewards_runtime_api::StakingRewardsRuntimeApi;

#[rpc(client, server)]
pub trait StakingRewardsApi<BlockHash, AssetId, FinancialNftInstanceId, Balance>
where
	AssetId: FromStr + Display + Ord,
	FinancialNftInstanceId: FromStr + Display,
	Balance: FromStr + Display,
{
	#[method(name = "stakingRewards_getClaimableAmount")]
	fn get_claimable_amount(
		&self,
		fnft_collection_id: SafeRpcWrapper<AssetId>,
		fnft_instance_id: SafeRpcWrapper<FinancialNftInstanceId>,
		at: Option<BlockHash>,
	) -> RpcResult<BTreeMap<AssetId, Option<Balance>>>;
}

pub struct StakingRewards<C, Block> {
	client: Arc<C>,
	_marker: sp_std::marker::PhantomData<Block>,
}

impl<C, M> StakingRewards<C, M> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block, AssetId, FinancialNftInstanceId, Balance>
	StakingRewardsApiServer<<Block as BlockT>::Hash, AssetId, FinancialNftInstanceId, Balance>
	for StakingRewards<C, (Block, AssetId, FinancialNftInstanceId, Balance)>
where
	Block: BlockT,
	AssetId: Send + Sync + 'static + Codec + FromStr + Display + Ord,
	FinancialNftInstanceId: Send + Sync + 'static + Codec + FromStr + Display,
	Balance: Send + Sync + 'static + Codec + FromStr + Display,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: StakingRewardsRuntimeApi<Block, AssetId, FinancialNftInstanceId, Balance>,
{
	fn get_claimable_amount(
		&self,
		fnft_collection_id: SafeRpcWrapper<AssetId>,
		fnft_instance_id: SafeRpcWrapper<FinancialNftInstanceId>,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Result<BTreeMap<AssetId, Option<Balance>>, ClaimableAmountError>> {
		let api = self.client.runtime_api();

		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		// calling ../../runtime-api
		let runtime_api_result =
			api.get_claimable_amount(&at, fnft_collection_id, fnft_instance_id);
		runtime_api_result.map_err(|e| {
			RpcError::Call(CallError::Custom(ErrorObject::owned(
				9876,
				"Something wrong",
				Some(format!("{:?}", e)),
			)))
		})
	}
}
