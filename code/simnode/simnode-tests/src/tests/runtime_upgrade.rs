use polkadot_primitives::v2::UpgradeGoAhead;
use sc_client_api::{CallExecutor, ExecutorProvider};
use sc_executor::NativeElseWasmExecutor;
use sc_service::TFullCallExecutor;
use simnode_apis::CreateTransactionApi;
use simnode_common::{events::AllRuntimeEvents, match_event};
use sp_api::ConstructRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT, Header},
	AccountId32, OpaqueExtrinsic,
};
use std::error::Error;
use substrate_simnode::{ChainInfo, FullClientFor, Node};

// generic tests for runtime upgrades
pub(crate) async fn parachain_runtime_upgrades<T>(
	node: &Node<T>,
	code: Vec<u8>,
) -> Result<(), Box<dyn Error>>
where
	T: ChainInfo,
	<T as ChainInfo>::Runtime:
		system::Config<AccountId = AccountId32> + sudo::Config + parachain_info::Config,
	<T::Runtime as system::Config>::RuntimeEvent: Into<AllRuntimeEvents> + Clone,
	<T::RuntimeApi as ConstructRuntimeApi<T::Block, FullClientFor<T>>>::RuntimeApi:
		CreateTransactionApi<
			T::Block,
			<T::Runtime as system::Config>::AccountId,
			<T::Runtime as system::Config>::RuntimeCall,
		>,
	<<T as ChainInfo>::Runtime as system::Config>::AccountId: codec::Codec,
	<<T as ChainInfo>::Runtime as system::Config>::RuntimeCall: codec::Codec,
	<TFullCallExecutor<T::Block, NativeElseWasmExecutor<T::ExecutorDispatch>> as CallExecutor<
		T::Block,
	>>::Error: std::fmt::Debug,
	<T::Runtime as system::Config>::RuntimeCall:
		From<system::Call<T::Runtime>> + From<sudo::Call<T::Runtime>>,
	<T::Runtime as sudo::Config>::Call: From<system::Call<T::Runtime>>,
	<<T::Block as BlockT>::Header as Header>::Number: num_traits::cast::AsPrimitive<u32>,
	<<T as ChainInfo>::Block as BlockT>::Extrinsic: From<OpaqueExtrinsic>,
{
	let sudo = node.with_state(None, sudo::Pallet::<T::Runtime>::key).unwrap();

	let old_runtime_version = node
		.client()
		.executor()
		.runtime_version(&BlockId::Hash(node.client().info().best_hash))?
		.spec_version;

	println!("\nold_runtime_version: {}\n", old_runtime_version);

	let call = sudo::Call::sudo_unchecked_weight {
		// Former Dali runtime had the spec_name of Picasso. Changing it to Dali requires
		// `without_checks`.
		call: Box::new(system::Call::set_code_without_checks { code }.into()),
		weight: 0,
	};
	node.submit_extrinsic(call, sudo).await?;
	node.seal_blocks(1).await;

	// give upgrade signal in the sproofed parachain inherents
	node.give_upgrade_signal(UpgradeGoAhead::GoAhead);
	node.seal_blocks(1).await;

	// assert that the runtime has been updated by looking at events
	let has_event = node.events(None).into_iter().any(|event| {
		match_event!(
			event.event.into(),
			ParachainSystem,
			parachain_system::Event::ValidationFunctionApplied { .. }
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
