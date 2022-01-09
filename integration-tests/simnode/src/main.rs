#![deny(unused_extern_crates, missing_docs)]

//! Basic example of end to end runtime tests.
mod chain_info;
mod node;

pub use chain_info::*;
use codec::Encode;
use picasso_runtime::{Event, Runtime};
use sc_client_api::{call_executor::ExecutorProvider, CallExecutor};
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, AccountId32};
use std::{error::Error, str::FromStr};
use support::storage::{self, generator::StorageValue};

fn main() -> Result<(), Box<dyn Error>> {
	node::run(|node| async move {
		let sudo = AccountId32::from_str("5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL")?;

		// fetch the storage key
		let key = parachain_info::ParachainId::<Runtime>::storage_value_final_key().to_vec();

		let raw_key_value: Option<u32> = node.with_state(|| storage::unhashed::get(&key[..]));

		assert_eq!(raw_key_value, Some(2000));
		let new_para_id: u32 = 2140;

		let value = new_para_id.encode();

		let call = sudo::Call::sudo_unchecked_weight {
			call: Box::new(system::Call::set_storage { items: vec![(key.clone(), value)] }.into()),
			weight: 0,
		};
		node.submit_extrinsic(call, Some(sudo.clone())).await?;
		node.seal_blocks(1).await;
		let raw_key_value: Option<u32> = node.with_state(|| storage::unhashed::get(&key[..]));

		assert_eq!(raw_key_value, Some(new_para_id));

		//
		// node.submit_extrinsic(
		// 	system::Call::remark { remark: b"Hello World".to_vec() },
		// 	Some(sudo.clone()),
		// )
		// .await?;
		// node.seal_blocks(1).await;
		//
		// let old_runtime_version = node
		// 	.client()
		// 	.executor()
		// 	.runtime_version(&BlockId::Hash(node.client().info().best_hash))?
		// 	.spec_version;
		// println!("\nold_runtime_version: {}\n", old_runtime_version);
		//
		// let code = picasso_runtime::WASM_BINARY
		// 	.ok_or("Polkadot development wasm not available")?
		// 	.to_vec();
		//
		// let call = sudo::Call::sudo_unchecked_weight {
		// 	call: Box::new(system::Call::set_code { code }.into()),
		// 	weight: 0,
		// };
		// node.submit_extrinsic(call, Some(sudo)).await?;
		// node.seal_blocks(2).await;
		// // assert that the runtime has been updated by looking at events
		// let events = node.events().into_iter().filter(|event| {
		// 	matches!(
		// 		event.event,
		// 		Event::ParachainSystem(parachain_system::Event::ValidationFunctionApplied(_))
		// 	)
		// });
		// // make sure event was emitted
		// assert_eq!(
		// 	events.count(),
		// 	1,
		// 	"system::Event::CodeUpdate not found in events: {:#?}",
		// 	node.events()
		// );
		// let new_runtime_version = node
		// 	.client()
		// 	.executor()
		// 	.runtime_version(&BlockId::Hash(node.client().info().best_hash))?
		// 	.spec_version;
		// println!("\nnew_runtime_version: {}\n", new_runtime_version);
		//
		// // just confirming
		// assert!(
		// 	new_runtime_version > old_runtime_version,
		// 	"Invariant, spec_version of new runtime: {} not greater than spec_version of old runtime:
		// {}", 	new_runtime_version,
		// 	old_runtime_version,
		// );
		//
		// // try to author 10 blocks, if it doesn't panic, all good.
		// node.seal_blocks(10).await;
		Ok(())
	})
}
