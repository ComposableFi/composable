use crate::{cli::ComposableCli, tests};
use common::DAYS;
use parachain_inherent::ParachainInherentData;
use sc_consensus_manual_seal::consensus::timestamp::SlotTimestampProvider;
use sc_service::TFullBackend;
use std::{error::Error, sync::Arc};
use substrate_simnode::{FullClientFor, Node, SignatureVerificationOverride};
use support::storage;

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
}

/// run all integration tests
pub fn run() -> Result<(), Box<dyn Error>> {
	substrate_simnode::parachain_node::<ChainInfo, _, _>(|node| async move {
		// test the storage override tx
		_parachain_info_storage_override_test(&node).await?;

		// test code-substitute for picasso, by authoring blocks past the launch period
		// test runtime upgrades
		let code = picasso_runtime::WASM_BINARY.ok_or("Picasso wasm not available")?.to_vec();
		tests::runtime_upgrade::parachain_runtime_upgrades(&node, code).await?;

		// try to create blocks for a month, if it doesn't panic, all good.
		node.seal_blocks((30 * DAYS) as usize).await;

		Ok(())
	})
}

async fn _parachain_info_storage_override_test(
	node: &Node<ChainInfo>,
) -> Result<(), Box<dyn Error>> {
	// sudo account on-chain
	let _sudo = node.with_state(None, sudo::Pallet::<picasso_runtime::Runtime>::key).unwrap();

	// gotten from
	// hex::encode(&parachain_info::ParachainId::<Runtime>::storage_value_final_key().to_vec());
	let key = hex::decode("0d715f2646c8f85767b5d2764bb2782604a74d81251e398fd8a0a4d55023bb3f")?;

	let raw_key_value: Option<u32> = node.with_state(None, || storage::unhashed::get(&key[..]));

	assert_eq!(raw_key_value, Some(2104));

	Ok(())
}
