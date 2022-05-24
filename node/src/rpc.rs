//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use frame_benchmarking::frame_support::CloneNoBound;
use polkadot_service::{ConstructRuntimeApi, NativeExecutionDispatch};
use sc_client_api::StateBackendFor;
use std::sync::Arc;

use common::OpaqueBlock;
pub use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool::FullPool;
use sp_api::{ProvideRuntimeApi, StateBackend};
use sp_blockchain::{HeaderBackend, HeaderMetadata};
use sp_runtime::traits::BlakeTwo256;

use crate::{
	client::{FullBackend, FullClient},
	runtime::{
		assets::ExtendWithAssetsApi, crowdloan_rewards::ExtendWithCrowdloanRewardsApi,
		lending::ExtendWithLendingApi, pablo::ExtendWithPabloApi, BaseHostRuntimeApis,
	},
};

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
	RuntimeApi:
		ConstructRuntimeApi<OpaqueBlock, FullClient<RuntimeApi, Executor>> + Send + Sync + 'static,
	RuntimeApi::RuntimeApi:
		BaseHostRuntimeApis<StateBackend = StateBackendFor<FullBackend, OpaqueBlock>>,
	StateBackendFor<FullBackend, OpaqueBlock>: StateBackend<BlakeTwo256>,
	Executor: NativeExecutionDispatch + 'static,
	FullClient<RuntimeApi, Executor>: ProvideRuntimeApi<OpaqueBlock>
		+ HeaderBackend<OpaqueBlock>
		+ HeaderMetadata<OpaqueBlock, Error = sp_blockchain::Error>
		+ 'static
		+ Send
		+ Sync
		+ Sized,
	<FullClient<RuntimeApi, Executor> as ProvideRuntimeApi<OpaqueBlock>>::Api:
		BaseHostRuntimeApis<StateBackend = StateBackendFor<FullBackend, OpaqueBlock>>
			+ ExtendWithAssetsApi<RuntimeApi, Executor>
			+ ExtendWithCrowdloanRewardsApi<RuntimeApi, Executor>
			+ ExtendWithPabloApi<RuntimeApi, Executor>
			+ ExtendWithLendingApi<RuntimeApi, Executor>,
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
