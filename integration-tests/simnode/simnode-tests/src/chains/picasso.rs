use crate::tests;
use common::DAYS;
use simnode_common::chains::picasso::ChainInfo;
use std::error::Error;
use substrate_simnode::Node;
use support::storage;

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

	assert_eq!(raw_key_value, Some(2087));

	Ok(())
}
