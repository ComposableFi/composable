use parachain_inherent::ParachainInherentData;
use sc_cli::CliConfiguration;
use sc_consensus_manual_seal::consensus::timestamp::SlotTimestampProvider;
use sc_service::TFullBackend;
use sp_runtime::generic::Era;
use std::sync::Arc;
use substrate_simnode::{ChainInfo, FullClientFor, SignatureVerificationOverride, SimnodeCli};

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

/// [`SimnodeCli`] implementation
pub struct PicassoCli;

impl SimnodeCli for PicassoCli {
	type CliConfig = sc_cli::RunCmd;
	type SubstrateCli = node::cli::Cli;

	fn cli_config(cli: &Self::SubstrateCli) -> &Self::CliConfig {
		&cli.run.base
	}

	fn log_filters(cli_config: &Self::CliConfig) -> Result<String, sc_cli::Error> {
		cli_config.log_filters()
	}
}
/// ChainInfo implementation.
pub struct PicassoChainInfo;

impl ChainInfo for PicassoChainInfo {
	type Block = common::OpaqueBlock;
	type ExecutorDispatch = ExecutorDispatch;
	type Runtime = picasso_runtime::Runtime;
	type RuntimeApi = picasso_runtime::RuntimeApi;
	type SelectChain = sc_consensus::LongestChain<TFullBackend<Self::Block>, Self::Block>;
	type BlockImport = Arc<FullClientFor<Self>>;
	type SignedExtras = picasso_runtime::SignedExtra;
	type InherentDataProviders = (
		SlotTimestampProvider,
		sp_consensus_aura::inherents::InherentDataProvider,
		ParachainInherentData,
	);
	type Cli = PicassoCli;

	fn signed_extras(from: <Self::Runtime as system::Config>::AccountId) -> Self::SignedExtras {
		(
			system::CheckSpecVersion::<Self::Runtime>::new(),
			system::CheckTxVersion::<Self::Runtime>::new(),
			system::CheckGenesis::<Self::Runtime>::new(),
			system::CheckMortality::<Self::Runtime>::from(Era::Immortal),
			system::CheckNonce::<Self::Runtime>::from(
				system::Pallet::<Self::Runtime>::account_nonce(from),
			),
			system::CheckWeight::<Self::Runtime>::new(),
			transaction_payment::ChargeTransactionPayment::<Self::Runtime>::from(0),
		)
	}
}
