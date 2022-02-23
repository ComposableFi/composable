use crate::cli::ComposableCli;
use parachain_inherent::ParachainInherentData;
use sc_consensus_manual_seal::consensus::timestamp::SlotTimestampProvider;
use sc_service::TFullBackend;
use std::sync::Arc;
use substrate_simnode::{FullClientFor, RpcHandlerArgs, SignatureVerificationOverride};

/// A unit struct which implements `NativeExecutionDispatch` feeding in the
/// hard-coded runtime.
pub struct ExecutorDispatch;

impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
	type ExtendHostFunctions =
		(frame_benchmarking::benchmarking::HostFunctions, SignatureVerificationOverride);

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		picasso_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		picasso_runtime::native_version()
	}
}

/// ChainInfo implementation.
pub struct ChainInfo;

impl substrate_simnode::ChainInfo for ChainInfo {
	type Block = common::OpaqueBlock;
	type ExecutorDispatch = ExecutorDispatch;
	type Runtime = picasso_runtime::Runtime;
	type RuntimeApi = picasso_runtime::RuntimeApi;
	type SelectChain = sc_consensus::LongestChain<TFullBackend<Self::Block>, Self::Block>;
	type BlockImport = Arc<FullClientFor<Self>>;
	type InherentDataProviders = (
		SlotTimestampProvider,
		sp_consensus_aura::inherents::InherentDataProvider,
		ParachainInherentData,
	);
	type Cli = ComposableCli;
	fn create_rpc_io_handler<SC>(
		deps: RpcHandlerArgs<Self, SC>,
	) -> jsonrpc_core::MetaIoHandler<sc_rpc::Metadata>
	where
		<<Self as substrate_simnode::ChainInfo>::RuntimeApi as sp_api::ConstructRuntimeApi<
			Self::Block,
			FullClientFor<Self>,
		>>::RuntimeApi: sp_api::Core<Self::Block>
			+ sp_transaction_pool::runtime_api::TaggedTransactionQueue<Self::Block>,
	{
		let full_deps = node::rpc::FullDeps {
			client: deps.client,
			pool: deps.pool,
			deny_unsafe: deps.deny_unsafe,
		};
		node::rpc::create::<_, _, Self::Block>(full_deps)
	}
}
