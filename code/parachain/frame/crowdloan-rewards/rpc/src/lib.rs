use codec::Codec;
use composable_support::rpc_helpers::SafeRpcWrapper;
use core::{fmt::Display, str::FromStr};
use crowdloan_rewards_runtime_api::CrowdloanRewardsRuntimeApi;
use frame_support::{pallet_prelude::MaybeSerializeDeserialize, Parameter};
use jsonrpsee::{
	core::{Error as RpcError, RpcResult},
	proc_macros::rpc,
	types::{error::CallError, ErrorObject},
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use sp_std::{marker::PhantomData, sync::Arc};

#[rpc(client, server)]
pub trait CrowdloanRewardsApi<BlockHash, AccountId, Balance>
where
	Balance: FromStr + Display,
{
	#[method(name = "crowdloanRewards_amountAvailableToClaimFor")]
	fn amount_available_to_claim_for(
		&self,
		account: AccountId,
		at: Option<BlockHash>,
	) -> RpcResult<SafeRpcWrapper<Balance>>;
}

/// A struct that implements the `CrowdloanRewardsApi`.
pub struct CrowdloanRewards<C, Block> {
	client: Arc<C>,
	_marker: PhantomData<Block>,
}

impl<C, M> CrowdloanRewards<C, M> {
	/// Create new `CrowdloanRewards` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block, AccountId, Balance>
	CrowdloanRewardsApiServer<<Block as BlockT>::Hash, AccountId, Balance>
	for CrowdloanRewards<C, (Block, AccountId, Balance)>
where
	Block: BlockT,
	AccountId: Send + Sync + Parameter + MaybeSerializeDeserialize + Ord + 'static,
	Balance: Send + Sync + 'static + Codec + FromStr + Display,
	C: Send + Sync + ProvideRuntimeApi<Block> + HeaderBackend<Block> + 'static,
	C::Api: CrowdloanRewardsRuntimeApi<Block, AccountId, Balance>,
{
	fn amount_available_to_claim_for(
		&self,
		remote_account: AccountId,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<SafeRpcWrapper<Balance>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| {
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash
		}));

		let runtime_api_result = api.amount_available_to_claim_for(&at, remote_account);
		// TODO(benluelo): Review what error message & code to use
		runtime_api_result.map_err(|e| {
			RpcError::Call(CallError::Custom(ErrorObject::owned(
				9876,
				"Something wrong",
				Some(format!("{:?}", e)),
			)))
		})
	}
}
