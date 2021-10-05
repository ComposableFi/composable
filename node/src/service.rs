// std
use std::sync::Arc;

// Local Runtime Types
use picasso_runtime::{self as parachain_runtime, RuntimeApi};

// Cumulus Imports
use cumulus_client_consensus_aura::{
	build_aura_consensus, BuildAuraConsensusParams, SlotProportion,
};
use cumulus_client_consensus_common::ParachainConsensus;
use cumulus_client_network::build_block_announce_validator;
use cumulus_client_service::{
	prepare_node_config, start_collator, start_full_node, StartCollatorParams, StartFullNodeParams,
};
use cumulus_primitives_core::ParaId;

// Substrate Imports
use sc_client_api::ExecutorProvider;
use sc_executor::native_executor_instance;
pub use sc_executor::NativeExecutor;
use sc_network::NetworkService;
use sc_service::{Configuration, PartialComponents, Role, TFullBackend, TFullClient, TaskManager};
use sc_telemetry::{Telemetry, TelemetryHandle, TelemetryWorker, TelemetryWorkerHandle};
use sp_api::ConstructRuntimeApi;
use sp_consensus::SlotData;
#[cfg(feature = "ocw")]
use sp_core::crypto::KeyTypeId;
use sp_keystore::SyncCryptoStorePtr;
use sp_runtime::traits::BlakeTwo256;
use substrate_prometheus_endpoint::Registry;

// Runtime type overrides
type BlockNumber = u32;
type Header = sp_runtime::generic::Header<BlockNumber, sp_runtime::traits::BlakeTwo256>;
pub type Block = sp_runtime::generic::Block<Header, sp_runtime::OpaqueExtrinsic>;
type Hash = sp_core::H256;

// Native executor instance.
native_executor_instance!(
	pub ParachainRuntimeExecutor,
	parachain_runtime::api::dispatch,
	parachain_runtime::native_version,
	frame_benchmarking::benchmarking::HostFunctions,
);

/// Starts a `ServiceBuilder` for a full service.
///
/// Use this macro if you don't actually need the full service, but just the builder in order to
/// be able to perform chain operations.
#[allow(clippy::type_complexity)]
pub fn new_partial<RuntimeApi, Executor, BIQ>(
	config: &Configuration,
	build_import_queue: BIQ,
) -> Result<
	PartialComponents<
		TFullClient<Block, RuntimeApi, Executor>,
		TFullBackend<Block>,
		(),
		sc_consensus::DefaultImportQueue<Block, TFullClient<Block, RuntimeApi, Executor>>,
		sc_transaction_pool::FullPool<Block, TFullClient<Block, RuntimeApi, Executor>>,
		(Option<Telemetry>, Option<TelemetryWorkerHandle>),
	>,
	sc_service::Error,
>
where
	RuntimeApi: ConstructRuntimeApi<Block, TFullClient<Block, RuntimeApi, Executor>>
		+ Send
		+ Sync
		+ 'static,
	RuntimeApi::RuntimeApi: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
		+ sp_api::Metadata<Block>
		+ sp_session::SessionKeys<Block>
		+ sp_api::ApiExt<
			Block,
			StateBackend = sc_client_api::StateBackendFor<TFullBackend<Block>, Block>,
		> + sp_offchain::OffchainWorkerApi<Block>
		+ sp_block_builder::BlockBuilder<Block>,
	sc_client_api::StateBackendFor<TFullBackend<Block>, Block>: sp_api::StateBackend<BlakeTwo256>,
	Executor: sc_executor::NativeExecutionDispatch + 'static,
	BIQ: FnOnce(
		Arc<TFullClient<Block, RuntimeApi, Executor>>,
		&Configuration,
		Option<TelemetryHandle>,
		&TaskManager,
	) -> Result<
		sc_consensus::DefaultImportQueue<Block, TFullClient<Block, RuntimeApi, Executor>>,
		sc_service::Error,
	>,
{
	let telemetry = config
		.telemetry_endpoints
		.clone()
		.filter(|x| !x.is_empty())
		.map(|endpoints| -> Result<_, sc_telemetry::Error> {
			let worker = TelemetryWorker::new(16)?;
			let telemetry = worker.handle().new_telemetry(endpoints);
			Ok((worker, telemetry))
		})
		.transpose()?;

	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, Executor>(
			config,
			telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
		)?;
	let client = Arc::new(client);

	let telemetry_worker_handle = telemetry.as_ref().map(|(worker, _)| worker.handle());

	let telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager.spawn_handle().spawn("telemetry", worker.run());
		telemetry
	});

	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_essential_handle(),
		client.clone(),
	);

	let import_queue = build_import_queue(
		client.clone(),
		config,
		telemetry.as_ref().map(|telemetry| telemetry.handle()),
		&task_manager,
	)?;

	let params = PartialComponents {
		backend,
		client,
		import_queue,
		keystore_container,
		task_manager,
		transaction_pool,
		select_chain: (),
		other: (telemetry, telemetry_worker_handle),
	};

	Ok(params)
}

/// Start a node with the given parachain `Configuration` and relay chain `Configuration`.
///
/// This is the actual implementation that is abstract over the executor and the runtime api.
#[sc_tracing::logging::prefix_logs_with("Parachain")]
async fn start_node_impl<RuntimeApi, Executor, RB, BIQ, BIC>(
	parachain_config: Configuration,
	polkadot_config: Configuration,
	id: ParaId,
	rpc_ext_builder: RB,
	build_import_queue: BIQ,
	build_consensus: BIC,
) -> sc_service::error::Result<(TaskManager, Arc<TFullClient<Block, RuntimeApi, Executor>>)>
where
	RuntimeApi: ConstructRuntimeApi<Block, TFullClient<Block, RuntimeApi, Executor>>
		+ Send
		+ Sync
		+ 'static,
	RuntimeApi::RuntimeApi: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
		+ sp_api::Metadata<Block>
		+ sp_session::SessionKeys<Block>
		+ sp_api::ApiExt<
			Block,
			StateBackend = sc_client_api::StateBackendFor<TFullBackend<Block>, Block>,
		> + sp_offchain::OffchainWorkerApi<Block>
		+ sp_block_builder::BlockBuilder<Block>
		+ cumulus_primitives_core::CollectCollationInfo<Block>,
	sc_client_api::StateBackendFor<TFullBackend<Block>, Block>: sp_api::StateBackend<BlakeTwo256>,
	Executor: sc_executor::NativeExecutionDispatch + 'static,
	RB: Fn(
			Arc<TFullClient<Block, RuntimeApi, Executor>>,
		) -> jsonrpc_core::IoHandler<sc_rpc::Metadata>
		+ Send
		+ 'static,
	BIQ: FnOnce(
		Arc<TFullClient<Block, RuntimeApi, Executor>>,
		&Configuration,
		Option<TelemetryHandle>,
		&TaskManager,
	) -> Result<
		sc_consensus::DefaultImportQueue<Block, TFullClient<Block, RuntimeApi, Executor>>,
		sc_service::Error,
	>,
	BIC: FnOnce(
		Arc<TFullClient<Block, RuntimeApi, Executor>>,
		Option<&Registry>,
		Option<TelemetryHandle>,
		&TaskManager,
		&polkadot_service::NewFull<polkadot_service::Client>,
		Arc<sc_transaction_pool::FullPool<Block, TFullClient<Block, RuntimeApi, Executor>>>,
		Arc<NetworkService<Block, Hash>>,
		SyncCryptoStorePtr,
		bool,
	) -> Result<Box<dyn ParachainConsensus<Block>>, sc_service::Error>,
{
	if matches!(parachain_config.role, Role::Light) {
		return Err("Light client not supported!".into())
	}

	let parachain_config = prepare_node_config(parachain_config);

	let params = new_partial::<RuntimeApi, Executor, BIQ>(&parachain_config, build_import_queue)?;

	#[cfg(feature = "ocw")]
	{
		let keystore = params.keystore_container.sync_keystore();
		if parachain_config.offchain_worker.enabled {
			// Initialize seed for signing transaction using off-chain workers. This is a
			// convenience so learners can see the transactions submitted simply running the node.
			// Typically these keys should be inserted with RPC calls to `author_insertKey`.
			// TODO(Jesse): remove in prod
			{
				sp_keystore::SyncCryptoStore::sr25519_generate_new(
					&*keystore,
					KeyTypeId(*b"orac"),
					Some("//Alice"),
				)
				.expect("Creating key with account Alice should succeed.");
			}
		}
	}

	let (mut telemetry, telemetry_worker_handle) = params.other;

	let relay_chain_full_node =
		cumulus_client_service::build_polkadot_full_node(polkadot_config, telemetry_worker_handle)
			.map_err(|e| match e {
				polkadot_service::Error::Sub(x) => x,
				s => format!("{}", s).into(),
			})?;

	let client = params.client.clone();
	let backend = params.backend.clone();
	let block_announce_validator = build_block_announce_validator(
		relay_chain_full_node.client.clone(),
		id,
		Box::new(relay_chain_full_node.network.clone()),
		relay_chain_full_node.backend.clone(),
	);

	let force_authoring = parachain_config.force_authoring;
	let validator = parachain_config.role.is_authority();
	let prometheus_registry = parachain_config.prometheus_registry().cloned();
	let transaction_pool = params.transaction_pool.clone();
	let mut task_manager = params.task_manager;
	let import_queue = cumulus_client_service::SharedImportQueue::new(params.import_queue);
	let (network, system_rpc_tx, start_network) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &parachain_config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue: import_queue.clone(),
			on_demand: None,
			warp_sync: None,
			block_announce_validator_builder: Some(Box::new(|_| block_announce_validator)),
		})?;

	if parachain_config.offchain_worker.enabled {
		sc_service::build_offchain_workers(
			&parachain_config,
			task_manager.spawn_handle(),
			client.clone(),
			network.clone(),
		);
	}

	let rpc_client = client.clone();
	let rpc_extensions_builder = Box::new(move |_, _| Ok(rpc_ext_builder(rpc_client.clone())));

	sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		on_demand: None,
		remote_blockchain: None,
		rpc_extensions_builder,
		client: client.clone(),
		transaction_pool: transaction_pool.clone(),
		task_manager: &mut task_manager,
		config: parachain_config,
		keystore: params.keystore_container.sync_keystore(),
		backend: backend.clone(),
		network: network.clone(),
		system_rpc_tx,
		telemetry: telemetry.as_mut(),
	})?;

	let announce_block = {
		let network = network.clone();
		Arc::new(move |hash, data| network.announce_block(hash, data))
	};

	if validator {
		let parachain_consensus = build_consensus(
			client.clone(),
			prometheus_registry.as_ref(),
			telemetry.as_ref().map(|t| t.handle()),
			&task_manager,
			&relay_chain_full_node,
			transaction_pool,
			network,
			params.keystore_container.sync_keystore(),
			force_authoring,
		)?;

		let spawner = task_manager.spawn_handle();

		let params = StartCollatorParams {
			para_id: id,
			block_status: client.clone(),
			announce_block,
			client: client.clone(),
			task_manager: &mut task_manager,
			relay_chain_full_node,
			spawner,
			parachain_consensus,
			import_queue,
		};

		start_collator(params).await?;
	} else {
		let params = StartFullNodeParams {
			client: client.clone(),
			announce_block,
			task_manager: &mut task_manager,
			para_id: id,
			relay_chain_full_node,
		};

		start_full_node(params)?;
	}

	start_network.start_network();

	Ok((task_manager, client))
}

/// Build the import queue for the the parachain runtime.
pub fn parachain_build_import_queue(
	client: Arc<TFullClient<Block, RuntimeApi, ParachainRuntimeExecutor>>,
	config: &Configuration,
	telemetry: Option<TelemetryHandle>,
	task_manager: &TaskManager,
) -> Result<
	sc_consensus::DefaultImportQueue<
		Block,
		TFullClient<Block, RuntimeApi, ParachainRuntimeExecutor>,
	>,
	sc_service::Error,
> {
	let slot_duration = cumulus_client_consensus_aura::slot_duration(&*client)?;

	cumulus_client_consensus_aura::import_queue::<
		sp_consensus_aura::sr25519::AuthorityPair,
		_,
		_,
		_,
		_,
		_,
		_,
	>(cumulus_client_consensus_aura::ImportQueueParams {
		block_import: client.clone(),
		client: client.clone(),
		create_inherent_data_providers: move |_, _| async move {
			let time = sp_timestamp::InherentDataProvider::from_system_time();

			let slot =
				sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_duration(
					*time,
					slot_duration.slot_duration(),
				);

			Ok((time, slot))
		},
		registry: config.prometheus_registry(),
		can_author_with: sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone()),
		spawner: &task_manager.spawn_essential_handle(),
		telemetry,
	})
	.map_err(Into::into)
}

/// Start a normal parachain node.
pub async fn start_node(
	parachain_config: Configuration,
	polkadot_config: Configuration,
	id: ParaId,
) -> sc_service::error::Result<(
	TaskManager,
	Arc<TFullClient<Block, RuntimeApi, ParachainRuntimeExecutor>>,
)> {
	start_node_impl::<RuntimeApi, ParachainRuntimeExecutor, _, _, _>(
		parachain_config,
		polkadot_config,
		id,
		|_| Default::default(),
		parachain_build_import_queue,
		|client,
		 prometheus_registry,
		 telemetry,
		 task_manager,
		 relay_chain_node,
		 transaction_pool,
		 sync_oracle,
		 keystore,
		 force_authoring| {
			let slot_duration = cumulus_client_consensus_aura::slot_duration(&*client)?;

			let proposer_factory = sc_basic_authorship::ProposerFactory::with_proof_recording(
				task_manager.spawn_handle(),
				client.clone(),
				transaction_pool,
				prometheus_registry,
				telemetry.clone(),
			);

			let relay_chain_backend = relay_chain_node.backend.clone();
			let relay_chain_client = relay_chain_node.client.clone();
			Ok(build_aura_consensus::<
				sp_consensus_aura::sr25519::AuthorityPair,
				_,
				_,
				_,
				_,
				_,
				_,
				_,
				_,
				_,
			>(BuildAuraConsensusParams {
				proposer_factory,
				create_inherent_data_providers: move |_, (relay_parent, validation_data)| {
					let parachain_inherent =
					cumulus_primitives_parachain_inherent::ParachainInherentData::create_at_with_client(
						relay_parent,
						&relay_chain_client,
						&*relay_chain_backend,
						&validation_data,
						id,
					);
					async move {
						let time = sp_timestamp::InherentDataProvider::from_system_time();

						let slot =
						sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_duration(
							*time,
							slot_duration.slot_duration(),
						);

						let parachain_inherent = parachain_inherent.ok_or_else(|| {
							Box::<dyn std::error::Error + Send + Sync>::from(
								"Failed to create parachain inherent",
							)
						})?;
						Ok((time, slot, parachain_inherent))
					}
				},
				block_import: client.clone(),
				relay_chain_client: relay_chain_node.client.clone(),
				relay_chain_backend: relay_chain_node.backend.clone(),
				para_client: client,
				backoff_authoring_blocks: Option::<()>::None,
				sync_oracle,
				keystore,
				force_authoring,
				slot_duration,
				// We got around 500ms for proposing
				block_proposal_slot_portion: SlotProportion::new(1f32 / 24f32),
				// And a maximum of 750ms if slots are skipped
				max_block_proposal_slot_portion: Some(SlotProportion::new(1f32 / 16f32)),
				telemetry,
			}))
		},
	)
	.await
}


#[cfg(test)]
mod tests {
	use cumulus_primitives_parachain_inherent::ParachainInherentData;

	use std::collections::BTreeMap;
	use codec::{Encode, Decode};
	/// An identifier for an inherent.
	pub type InherentIdentifier = [u8; 8];

	/// Inherent data to include in a block.
	#[derive(Clone, Default, Encode, Decode, Debug)]
	pub struct InherentData {
		/// All inherent data encoded with parity-scale-codec and an identifier.
		data: BTreeMap<InherentIdentifier, Vec<u8>>
	}
	#[derive(Debug, Encode, Decode, Clone, Copy, Default)]
	pub struct Timestamp(u64);

	/// Unit type wrapper that represents a slot.
	#[derive(Debug, Encode, Decode, Clone, Copy, Default)]
	pub struct Slot(u64);

	#[test]
	fn can_decode() {
		let bytes = hex::decode("0c61757261736c6f7420c0fe3910000000007379736931333337411fdd02a86f7b979d7c5f580c51ba32b9d4602e6dd4d0acf86d6d2b9584eeac1253300908fc62c126433a2f3168df03163e684798cd21bca7e1916cf1109c6a19b7c44f932b9467f9c29a534b2e3de2bbe314303839140532c139fbbf5e189225f4c6c81f08066175726120b8fe39100000000005617572610101067f9d2863c17c2c8e7350e4980acddd6870a944c7376165fe8baa7159aaec4eb65724d3987de11640c1a2fdcbebf91c161d8befce83aca89b62ff4e9d69b285100000001e66ca3d391412795c2033e0f81b695261f036c9b3063e5312a2e3506201a14f000050002c61037ede3d8a54d27e44a9d5ce189618f22db4b49d95320d9021994c850f25b8e385d902000030000080000008000000000010000000100005000000050000000100000001000000000050000004000000286bee0000000004000000040000000500000000000000000000000000000000000000000000000000000000000000000000000800000000200000040000000400000000001000b00400000000000000000000140000000400000004000000000000000000060000006400000002000000c8000000020000001900000000000000020000000200000015018001108068327ba20f2bb2c9883b311eef9e517649a2690ccf34a940c2d2746eb588f4d5802b8c49da282d256ecf815858b3789344e2871404c1ed75db7bc1be6db38ccf011501800804805470c18f95e20dde431cb114cfeaf1ff0630f4448bd13b49012e2ab20de640a5805ec24b3f27644659695973cb6ee4f49f4ba3cff385c2ef32902518cb9e203b22f49e207f03cfdce586301014700e2c25930041585f078d434d6125b40443fe11fd292d13a410000009084c5f0ec2d17a76153ff51817f12d9cfc3c7f040001059eb6f36e027abb2091cfb5110ab5087fe96d685f06155b3cd9a8c9e5e9a23fd5dc13a5ed20c0fe391000000000685f08316cbf8fa0da822a20ac1c55bf1be320020000000000000080be8dc563400b7ab48d2faec9f49559e98e89f1b3cf4f598682a4d23ca9dfa63d8024c29e3bde8fa133999d0356894b580ffd65a451a19f623d1e740cc9736ea31180610ab1ad39b250f2b1612ea821e0e096df25f72b1e238ab73cc6e81a7f47da12585f078d434d6125b40443fe11fd292d13a4100300000080f9fdaf4c9d37389247a7caa7a50b9862259375c2a4d732ddfbca2c2ba010960c806c6be0ff8e9188d35ee8c7af0cc6120e451775aa3e0bf6b5058b7dc271de01df80fbfda55793b3c6931dcfd90fd1a9f341e9474dbb9e237e6ba7d416f94e23c583685f090e2fbf2d792cb324bffa9427fe1f0e200b00000010000000c90780fffe804ee7f50a66031c6df8f8a06287a701421740df183980d5148ec94faeaf4b7cd8809494822980291369f298d40bdfbdb8969e419e93770635e8000898ae75750e528002dc4ab17b79ace3fc1db71a7fd5549fe136921d88f7279e202052f533f893d280f063fd5b9ed2664522f3366d7da5511949551985ffbbc327a2fd67bdaec3174e80eac002b8ab9ae1f4ab153e224472dbe0ba9e9de1eb3c9255e3f2688377cdd37980c0ca6f6ca9e23c938906f5bdf614d39376c64515fb9fba32465fc743201774c78003c579a0f00e99ccdd67d8d2958a979241f1fc0e5b75b10a3f4b83fd0bc332c480e33eae1ec0bcd6a9b28df074e40a91eccb5ddc95cda5ee73d4b1419cd8927f7d806be76d1637a1f2393ff5d100fef04708bb0966e38c19e38a64afa733c7c76e0b80c3894174722f5b9a4f0a6917fe2963a5df453445a3d63bb36e5a97b1856c009780de0cf483ff62d0ea4fdd7260e286ac40cd54ca35c4e679e7a3d9967e61a044d680c1f122fb24ff49e818adeb15c843df929c984954f8d9b167fd98e27c095323b580b414027bc1ef681393b6e4be8b30d7a7ea1be2030a4cb507bf5128c7ea5dad4e80abe0339eaa5b4e3529c30b48702220a22be8c03c72f74a895c3af27644a1e02a80290ab351e33f61467eda2e0a592c448fde1bf0ed1df41c7339c1def9fc1b08659901802102800e7fecc1d0c346dc93afc52063186a19d45cc3b5a29485e952096b2b0cf40eb2806385f74454cea937cea991614dd8bb7a2347b23dda1d6d8a8818ae2198784afb80e5fd99bb591f335862da6ff4839079b46d09ed44dd10acc1b127b7c0c3a682c0a102801115806083fccc5ad9489e8fffd469466a4206a743d8dbbc8fb301059daa1d198c862880989783ca2ecf0db9def32437c785585684864536c85c7a058824436e0777efa180e8d41073c6e9d101bf1b8c0aea0eb5b14f3b0fc5facde3def7dab500743e95d78046c5eb83ee48d52be6d46c251b1863432bd93b459592054a60292779e034d1028072db863c38f84bbb990b24912015867a827be1145afa78c5f6aa6c865759cc97a9019d0da05ca59913bc38a8630590f2627c014180455e1dadeed4b05290f6363644c4ec06e8f62ebc926fed7cb8ddd4070ec9831e585f078d434d6125b40443fe11fd292d13a410000009087c7700e67da63472835bb0b737093a19ad4c63f5a4efb16ffa83d0070000040025019ef78c98723ddc9073523ef3beefda0c0005585f078d434d6125b40443fe11fd292d13a410000009087c77095dac46c07a40d91506e7637ec4ba5763f5a4efb16ffa83d007000004001501804001805499eba4624ae09044e3b359063ed76107a6e7026690a7645c5bb9daac23af32806c909f3db3c757cbb50a2d7ddf337772495acacefd3e2e30d7b76fde051c72a9000074696d73746170302065b4424f7c010000").unwrap();
		// let bytes = hex::decode("0c61757261736c6f742021ff3910000000007379736931333337411c8901000000000000000000000000000000000000000000000000000000000000000000d69f41ed6e6842f6f41cc93b7910cf7d38e4ec51054c83c45a2402d8f0315d4a03170a2e7597b7b7e3d84c05391d139a62b157e78786d8c082f29dcf4c111314000b000000b36479c503a430081eece29df48dff22b5160438f938e16877903d3dba939140000050002c1501804001805499eba4624ae09044e3b359063ed76107a6e7026690a7645c5bb9daac23af32806c909f3db3c757cbb50a2d7ddf337772495acacefd3e2e30d7b76fde051c72a9a102801115806083fccc5ad9489e8fffd469466a4206a743d8dbbc8fb301059daa1d198c862880989783ca2ecf0db9def32437c785585684864536c85c7a058824436e0777efa180e8d41073c6e9d101bf1b8c0aea0eb5b14f3b0fc5facde3def7dab500743e95d78046c5eb83ee48d52be6d46c251b1863432bd93b459592054a60292779e034d10280e1b88bc526fbfdb1045cc0251243b6f8c88637e114690b00908ad1dae476b7a099018021028089f6ceb40681377d3f4064290b82b338e1de6d4b68a936cc6efaa4009367a267806385f74454cea937cea991614dd8bb7a2347b23dda1d6d8a8818ae2198784afb80e5fd99bb591f335862da6ff4839079b46d09ed44dd10acc1b127b7c0c3a682c0947d0da05ca59913bc38a8630590f2627c878d434d6125b40443fe11fd292d13a41000000908c90780fffe804ee7f50a66031c6df8f8a06287a701421740df183980d5148ec94faeaf4b7cd880a57880d5a786e454cab93eb417ea2dee47e19f6dc77c4f728383dc03e8b3f0c080816c00728e53aa3984c4c334248c76b6269638a9c9b9d757d68b705949afd26c805a3f150bcd67868c42415375321796cb4a0a0a21a8084ab7d4ee8520bf4203a880b1cf7731e4dbfedcfd1b70f4b333e2eff99c513e8c0283e83985f6c60b966f518021d9ea6c29c32919685191fe0c478ddd346f447435f76dcb6bc1ed94830978aa8091369f47083bc585194534c6ce713cd882bb260005845157ac8e10af90d68bd180e33eae1ec0bcd6a9b28df074e40a91eccb5ddc95cda5ee73d4b1419cd8927f7d807681d7a3476f4d910fcae77f5e6170354f62bad0bc2cb9b86435789946bfb13880c3894174722f5b9a4f0a6917fe2963a5df453445a3d63bb36e5a97b1856c0097807ad0220248ec08e47fc3030860327d9f7fe73ac493cabb55b85a2a3d386dbca98066f4800caa52897808dd2e53e8b883a1eb608e696e92394620bebaea84c89aa2802751276f10cbb0d647616b8bcd06e10746e730089ad8467ce3aa93abdee1350780abe0339eaa5b4e3529c30b48702220a22be8c03c72f74a895c3af27644a1e02a803a94cb8d7fa60d6ffaca4d5bf8076aa4c880b0e8db5cf2df154619859a8611cf61037ede3d8a54d27e44a9d5ce189618f22db4b49d95320d9021994c850f25b8e385d902000030000080000008000000000010000000100005000000050000000100000001000000000050000004000000286bee0000000004000000040000000500000000000000000000000000000000000000000000000000000000000000000000000800000000200000040000000400000000001000b00400000000000000000000140000000400000004000000000000000000060000006400000002000000c800000002000000190000000000000002000000020000001501800804809b87354ff387ce8f6749f53708ddd211521c1629ac8136f10bb6e941d2f4da2b80249f6b327c3f3cc51f2df859c2187d67b617ec248c09078d3cc49df0798cb2f1150180011080b819bec52997b5f3a2e5b13851de8a4f0b96f05bada8680d539665db74fa5ff5802b8c49da282d256ecf815858b3789344e2871404c1ed75db7bc1be6db38ccf01947ef78c98723ddc9073523ef3beefda0c878d434d6125b40443fe11fd292d13a4100000090801059eb6f36e027abb2091cfb5110ab5087fe96d685f06155b3cd9a8c9e5e9a23fd5dc13a5ed2021ff391000000000685f08316cbf8fa0da822a20ac1c55bf1be320010000000000000080be8dc563400b7ab48d2faec9f49559e98e89f1b3cf4f598682a4d23ca9dfa63d80d34ab8f518291bf89bbee5ee725902bf482580145711b690d5156969504b9b6880aa5f536733aa2b3a9166b10b42b81719d141dd6c035c682b79ee994ff80e3a78585f078d434d6125b40443fe11fd292d13a4100300000080f9fdaf4c9d37389247a7caa7a50b9862259375c2a4d732ddfbca2c2ba010960c8047950f640289f7e7b869df5b9cba92476b3b551c1978fbb5ce35d3853aed257780fbfda55793b3c6931dcfd90fd1a9f341e9474dbb9e237e6ba7d416f94e23c583685f090e2fbf2d792cb324bffa9427fe1f0e20000000000b000000f49e207f03cfdce586301014700e2c25930041585f078d434d6125b40443fe11fd292d13a410000009084c5f0ec2d17a76153ff51817f12d9cfc3c7f0400000074696d737461703020ca954b4f7c010000").unwrap();

		let inherent = InherentData::decode(&mut &bytes[..]).unwrap();

		for (key, value) in inherent.data.into_iter() {
			match &key {
				b"timstap0" => {
					println!("Timestamp: {:?}", Timestamp::decode(&mut &value[..]))
				}
				b"auraslot" => {
					println!("Slot: {:?}", Slot::decode(&mut &value[..]))
				}
				b"sysi1337" => {
					println!("ParachainInherentData: {:?}", ParachainInherentData::decode(&mut &value[..]).unwrap().validation_data)
				}
				_ => unreachable!()
			}
		}
	}
}
