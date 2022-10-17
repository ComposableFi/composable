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
	type SignedExtras = picasso_runtime::SignedExtra;
	type Cli = ComposableCli;
	fn create_rpc_io_handler<SC>(deps: RpcHandlerArgs<Self, SC>) -> jsonrpsee::RpcModule<()> {
		let full_deps = node::rpc::FullDeps {
			client: deps.client,
			pool: deps.pool,
			deny_unsafe: deps.deny_unsafe,
			chain_props: Default::default(),
		};
		node::rpc::create(full_deps).expect("Rpc to be initialized")
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
			asset_tx_payment::ChargeAssetTxPayment::<Self::Runtime>::from(0, None),
		)
	}
}
