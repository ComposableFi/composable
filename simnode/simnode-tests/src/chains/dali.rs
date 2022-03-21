use crate::tests;
use common::DAYS;
use simnode_common::chains::dali::ChainInfo;
use std::error::Error;

/// run all integration tests
pub fn run() -> Result<(), Box<dyn Error>> {
	substrate_simnode::parachain_node::<ChainInfo, _, _>(|node| async move {
		// test code-substitute for dali, by authoring blocks past the launch period
		node.seal_blocks(10).await;
		// test runtime upgrades
		let code = dali_runtime::WASM_BINARY.ok_or("Dali wasm not available")?.to_vec();
		tests::runtime_upgrade::parachain_runtime_upgrades(&node, code).await?;

		// try to create blocks for a month, if it doesn't panic, all good.
		node.seal_blocks((30 * DAYS) as usize).await;

		Ok(())
	})
}
