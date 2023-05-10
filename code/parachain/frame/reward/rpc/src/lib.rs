//! RPC interface for the Reward Module.

use codec::Codec;
use jsonrpsee::{
	core::{async_trait, Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorCode, ErrorObject},
};
use reward_rpc_runtime_api::BalanceWrapper;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	traits::{Block as BlockT, MaybeDisplay, MaybeFromStr},
	DispatchError,
};
use std::sync::Arc;

pub use reward_rpc_runtime_api::RewardApi as RewardRuntimeApi;

#[rpc(client, server)]
pub trait RewardApi<BlockHash, AccountId, CurrencyId, Balance, BlockNumber, UnsignedFixedPoint>
where
	Balance: Codec + MaybeDisplay + MaybeFromStr,
	AccountId: Codec,
	CurrencyId: Codec,
	BlockNumber: Codec,
	UnsignedFixedPoint: Codec,
{
	#[method(name = "reward_computeFarmingReward")]
	fn compute_farming_reward(
		&self,
		account_id: AccountId,
		pool_currency_id: CurrencyId,
		reward_currency_id: CurrencyId,
		at: Option<BlockHash>,
	) -> RpcResult<BalanceWrapper<Balance>>;

	#[method(name = "reward_estimateFarmingReward")]
	fn estimate_farming_reward(
		&self,
		account_id: AccountId,
		pool_currency_id: CurrencyId,
		reward_currency_id: CurrencyId,
		at: Option<BlockHash>,
	) -> RpcResult<BalanceWrapper<Balance>>;
}

fn internal_err<T: ToString>(message: T) -> JsonRpseeError {
	JsonRpseeError::Call(CallError::Custom(ErrorObject::owned(
		ErrorCode::InternalError.code(),
		message.to_string(),
		None::<()>,
	)))
}

/// A struct that implements the [`RewardApi`].
pub struct Reward<C, B> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<B>,
}

impl<C, B> Reward<C, B> {
	/// Create new `Reward` with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Reward { client, _marker: Default::default() }
	}
}

fn handle_response<T, E: std::fmt::Debug>(
	result: Result<Result<T, DispatchError>, E>,
	msg: String,
) -> RpcResult<T> {
	result
		.map_err(|err| internal_err(format!("Runtime error: {:?}: {:?}", msg, err)))?
		.map_err(|err| internal_err(format!("Execution error: {:?}: {:?}", msg, err)))
}

#[async_trait]
impl<C, Block, AccountId, CurrencyId, Balance, BlockNumber, UnsignedFixedPoint>
	RewardApiServer<
		<Block as BlockT>::Hash,
		AccountId,
		CurrencyId,
		Balance,
		BlockNumber,
		UnsignedFixedPoint,
	> for Reward<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api:
		RewardRuntimeApi<Block, AccountId, CurrencyId, Balance, BlockNumber, UnsignedFixedPoint>,
	AccountId: Codec,
	CurrencyId: Codec,
	Balance: Codec + MaybeDisplay + MaybeFromStr,
	BlockNumber: Codec,
	UnsignedFixedPoint: Codec,
{
	fn compute_farming_reward(
		&self,
		account_id: AccountId,
		pool_currency_id: CurrencyId,
		reward_currency_id: CurrencyId,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<BalanceWrapper<Balance>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);

		handle_response(
			api.compute_farming_reward(at, account_id, pool_currency_id, reward_currency_id),
			"Unable to compute the current reward".into(),
		)
	}

	fn estimate_farming_reward(
		&self,
		account_id: AccountId,
		pool_currency_id: CurrencyId,
		reward_currency_id: CurrencyId,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<BalanceWrapper<Balance>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);

		handle_response(
			api.estimate_farming_reward(at, account_id, pool_currency_id, reward_currency_id),
			"Unable to estimate the current reward".into(),
		)
	}
}
