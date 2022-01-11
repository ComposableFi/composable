use crate::{events::AllRuntimeEvents, match_event};
use polkadot_primitives::v1::UpgradeGoAhead;
use sc_client_api::{CallExecutor, ExecutorProvider};
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

// generic tests for runtime upgrades
pub(crate) async fn parachain_runtime_upgrades<T>(
	node: &Node<T>,
	code: Vec<u8>,
) -> Result<(), Box<dyn Error>>
where
	T: ChainInfo,
	<T as ChainInfo>::Runtime:
		system::Config<AccountId = AccountId32> + sudo::Config + parachain_info::Config,
	<T::Runtime as system::Config>::Event: Into<AllRuntimeEvents> + Clone,
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
	let sudo = node.with_state(None, sudo::Pallet::<T::Runtime>::key);

	let old_runtime_version = node
		.client()
		.executor()
		.runtime_version(&BlockId::Hash(node.client().info().best_hash))?
		.spec_version;

	println!("\nold_runtime_version: {}\n", old_runtime_version);

	let call = sudo::Call::sudo_unchecked_weight {
		call: Box::new(system::Call::set_code_without_checks { code }.into()),
		weight: 0,
	};
	node.submit_extrinsic(call, Some(sudo)).await?;
	node.seal_blocks(1).await;

	// give upgrade signal in the sproofed parachain inherents
	node.give_upgrade_signal(UpgradeGoAhead::GoAhead);
	node.seal_blocks(1).await;

	// assert that the runtime has been updated by looking at events
	let has_event = node.events(None).into_iter().any(|event| {
		match_event!(
			event.event.into(),
			ParachainSystem,
			parachain_system::Event::ValidationFunctionApplied(_)
		)
	});
	// make sure event was emitted
	assert!(has_event, "system::Event::CodeUpdate not found in events: {:#?}", node.events(None));

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
