#![deny(unused_extern_crates, missing_docs)]

//! Basic example of end to end runtime tests.
mod chain_info;

pub use chain_info::*;
use common::DAYS;
use picasso_runtime::{Event, Runtime};
use polkadot_primitives::v1::UpgradeGoAhead;
use sc_client_api::{call_executor::ExecutorProvider, CallExecutor};
use sc_executor::NativeElseWasmExecutor;
use sc_service::TFullCallExecutor;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::{BlockId, UncheckedExtrinsic},
	traits::{Block as BlockT, Header},
	AccountId32, MultiAddress, MultiSignature,
};
use std::error::Error;
use substrate_simnode::{ChainInfo, Node};
use support::storage;

fn main() -> Result<(), Box<dyn Error>> {
	substrate_simnode::parachain_node::<PicassoChainInfo, _, _>(|node| async move {
		// test runtime upgrades
		_parachain_runtime_upgrades(&node).await?;
		// test the storage override tx
		_parachain_info_storage_override_test(&node).await?;

		// try to create blocks for a month, if it doesn't panic, all good.
		node.seal_blocks((30 * DAYS) as usize).await;

		Ok(())
	})
}

// generic tests for runtime upgrades
async fn _parachain_runtime_upgrades<T>(node: &Node<T>) -> Result<(), Box<dyn Error>>
where
	T: ChainInfo,
	<T as ChainInfo>::Runtime: system::Config<AccountId = AccountId32, Event = Event>
		+ sudo::Config
		+ parachain_info::Config,
	<TFullCallExecutor<T::Block, NativeElseWasmExecutor<T::ExecutorDispatch>> as CallExecutor<
		T::Block,
	>>::Error: std::fmt::Debug,
	<T::Block as BlockT>::Extrinsic: From<
		UncheckedExtrinsic<
			MultiAddress<
				<T::Runtime as system::Config>::AccountId,
				<T::Runtime as system::Config>::Index,
			>,
			<T::Runtime as system::Config>::Call,
			MultiSignature,
			T::SignedExtras,
		>,
	>,
	<T::Runtime as system::Config>::Call:
		From<system::Call<T::Runtime>> + From<sudo::Call<T::Runtime>>,
	<T::Runtime as sudo::Config>::Call: From<system::Call<T::Runtime>>,
	<<T::Block as BlockT>::Header as Header>::Number: num_traits::cast::AsPrimitive<u32>,
{
	let sudo = node.with_state(None, sudo::Pallet::<Runtime>::key);

	// test code-substitute for picasso, by authoring blocks past the launch period
	node.seal_blocks(10).await;

	let old_runtime_version = node
		.client()
		.executor()
		.runtime_version(&BlockId::Hash(node.client().info().best_hash))?
		.spec_version;

	println!("\nold_runtime_version: {}\n", old_runtime_version);

	let code = picasso_runtime::WASM_BINARY
		.ok_or("Polkadot development wasm not available")?
		.to_vec();

	let call = sudo::Call::sudo_unchecked_weight {
		call: Box::new(system::Call::set_code { code }.into()),
		weight: 0,
	};
	node.submit_extrinsic(call, Some(sudo)).await?;
	node.seal_blocks(1).await;

	// give upgrade signal in the sproofed parachain inherents
	node.give_upgrade_signal(UpgradeGoAhead::GoAhead);
	node.seal_blocks(1).await;

	// assert that the runtime has been updated by looking at events
	let events = node.events(None).into_iter().filter(|event| {
		matches!(
			event.event,
			Event::ParachainSystem(parachain_system::Event::ValidationFunctionApplied(_))
		)
	});
	// make sure event was emitted
	assert_eq!(
		events.count(),
		1,
		"system::Event::CodeUpdate not found in events: {:#?}",
		node.events(None)
	);

	let new_runtime_version = node
		.client()
		.executor()
		.runtime_version(&BlockId::Hash(node.client().info().best_hash))?
		.spec_version;
	println!("\nnew_runtime_version: {}\n", new_runtime_version);

	// just confirming
	assert!(
		new_runtime_version > old_runtime_version,
		"Invariant, spec_version of new runtime: {} not greater than spec_version of old runtime:
		{}",
		new_runtime_version,
		old_runtime_version,
	);

	Ok(())
}

async fn _parachain_info_storage_override_test(
	node: &Node<PicassoChainInfo>,
) -> Result<(), Box<dyn Error>> {
	// sudo account on-chain
	let sudo = node.with_state(None, sudo::Pallet::<Runtime>::key);

	// gotten from
	// hex::encode(&parachain_info::ParachainId::<Runtime>::storage_value_final_key().to_vec());
	let key = hex::decode("0d715f2646c8f85767b5d2764bb2782604a74d81251e398fd8a0a4d55023bb3f")?;

	let raw_key_value: Option<u32> = node.with_state(None, || storage::unhashed::get(&key[..]));

	assert_eq!(raw_key_value, Some(2104));
	let new_para_id: u32 = 2087;

	// gotten from hex::encode(new_para_id.encode())
	let value = hex::decode("27080000")?;

	let call = sudo::Call::sudo_unchecked_weight {
		call: Box::new(system::Call::set_storage { items: vec![(key.clone(), value)] }.into()),
		weight: 0,
	};
	node.submit_extrinsic(call, Some(sudo.clone())).await?;
	node.seal_blocks(1).await;
	let raw_key_value: Option<u32> = node.with_state(None, || storage::unhashed::get(&key[..]));

	assert_eq!(raw_key_value, Some(new_para_id));

	Ok(())
}
