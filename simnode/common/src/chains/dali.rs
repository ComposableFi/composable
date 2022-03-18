use crate::cli::ComposableCli;
use parachain_inherent::ParachainInherentData;
use sc_consensus_manual_seal::consensus::timestamp::SlotTimestampProvider;
use sc_service::TFullBackend;
use sp_runtime::generic::Era;
use std::sync::Arc;
use substrate_simnode::{FullClientFor, RpcHandlerArgs, SignatureVerificationOverride};

/// A unit struct which implements `NativeExecutionDispatch` feeding in the
/// hard-coded runtime.
pub struct ExecutorDispatch;

impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
	type ExtendHostFunctions =
		(frame_benchmarking::benchmarking::HostFunctions, SignatureVerificationOverride);

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		dali_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		dali_runtime::native_version()
	}
}

/// ChainInfo implementation.
pub struct ChainInfo;

impl substrate_simnode::ChainInfo for ChainInfo {
	type Block = common::OpaqueBlock;
	type ExecutorDispatch = ExecutorDispatch;
	type Runtime = dali_runtime::Runtime;
	type RuntimeApi = dali_runtime::RuntimeApi;
	type SelectChain = sc_consensus::LongestChain<TFullBackend<Self::Block>, Self::Block>;
	type BlockImport = Arc<FullClientFor<Self>>;
	type InherentDataProviders = (
		SlotTimestampProvider,
		sp_consensus_aura::inherents::InherentDataProvider,
		ParachainInherentData,
	);
	type SignedExtras = dali_runtime::SignedExtra;
	type Cli = ComposableCli;

	fn create_rpc_io_handler<SC>(
		deps: RpcHandlerArgs<Self, SC>,
	) -> jsonrpc_core::MetaIoHandler<sc_rpc::Metadata> {
		let full_deps = node::rpc::FullDeps {
			client: deps.client,
			pool: deps.pool,
			deny_unsafe: deps.deny_unsafe,
		};
		node::rpc::create::<_, _, Self::Block>(full_deps)
	}

	fn signed_extras(from: <Self::Runtime as system::Config>::AccountId) -> Self::SignedExtras {
		let nonce = system::Pallet::<Self::Runtime>::account_nonce(from);
		(
			system::CheckNonZeroSender::<Self::Runtime>::new(),
			system::CheckSpecVersion::<Self::Runtime>::new(),
			system::CheckTxVersion::<Self::Runtime>::new(),
			system::CheckGenesis::<Self::Runtime>::new(),
			system::CheckEra::<Self::Runtime>::from(Era::Immortal),
			system::CheckNonce::<Self::Runtime>::from(nonce),
			system::CheckWeight::<Self::Runtime>::new(),
			transaction_payment::ChargeTransactionPayment::<Self::Runtime>::from(0),
		)
	}
}
