//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use frame_benchmarking::frame_support::CloneNoBound;
use polkadot_service::{ConstructRuntimeApi, NativeExecutionDispatch};
use sc_client_api::StateBackendFor;
use std::sync::Arc;

use assets_rpc::{Assets, AssetsApi};
use common::{AccountId, AccountIndex, Balance, PoolId};
use composable_traits::assets::Asset;
use crowdloan_rewards_rpc::{CrowdloanRewards, CrowdloanRewardsApi};
use pablo_rpc::{Pablo, PabloApi};
pub use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_runtime::traits::Block;

/// Full client dependencies.
#[derive(CloneNoBound)]
pub struct FullDeps<Client, Pool> {
	/// The client instance to use.
	pub client: Arc<Client>,
	/// Transaction pool instance.
	pub pool: Arc<Pool>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
}

/// Instantiate all full RPC extensions.
pub fn create<RuntimeApi, Executor>(
	deps: FullDeps<
		FullClient<RuntimeApi, Executor>,
		FullPool<OpaqueBlock, FullClient<RuntimeApi, Executor>>,
	>,
) -> jsonrpc_core::MetaIoHandler<sc_rpc::Metadata>
where
	B: Block,
	C: ProvideRuntimeApi<B>,
	C: HeaderBackend<B> + HeaderMetadata<B, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<B, AccountId, AccountIndex>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<B, Balance>,
	C::Api: assets_runtime_api::AssetsRuntimeApi<B, CurrencyId, AccountId, Balance>,
	C::Api: crowdloan_rewards_runtime_api::CrowdloanRewardsRuntimeApi<B, AccountId, Balance>,
	C::Api: pablo_runtime_api::PabloRuntimeApi<B, PoolId, CurrencyId, Balance>,
	C::Api: BlockBuilder<B>,
	P: TransactionPool + 'static,
{
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
	use substrate_frame_rpc_system::{FullSystem, SystemApi};

	let mut io = jsonrpc_core::MetaIoHandler::default();

	io.extend_with(SystemApi::to_delegate(FullSystem::new(
		deps.client.clone(),
		deps.pool.clone(),
		deps.deny_unsafe,
	)));

	io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(
		deps.client.clone(),
	)));

	<FullClient<RuntimeApi, Executor> as ProvideRuntimeApi<OpaqueBlock>>::Api::extend_with_assets_api(
		&mut io,
		deps.clone(),
	);

	<FullClient<RuntimeApi, Executor> as ProvideRuntimeApi<OpaqueBlock>>::Api::extend_with_crowdloan_rewards_api(
		&mut io,
		deps.clone(),
	);

	<FullClient<RuntimeApi, Executor> as ProvideRuntimeApi<OpaqueBlock>>::Api::extend_with_pablo_api(
		&mut io,
		deps.clone(),
	);

	<FullClient<RuntimeApi, Executor> as ProvideRuntimeApi<OpaqueBlock>>::Api::extend_with_lending_api(
		&mut io,
		deps,
	);

	// Extend this RPC with a custom API by using the following syntax.
	// `YourRpcStruct` should have a reference to a client, which is needed
	// to call into the runtime.
	// `io.extend_with(YourRpcTrait::to_delegate(YourRpcStruct::new(ReferenceToClient, ...)));`

	io
}
