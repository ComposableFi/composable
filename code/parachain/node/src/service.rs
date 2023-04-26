// std
use std::{sync::Arc, time::Duration};
// Cumulus Imports
use common::OpaqueBlock;
use cumulus_client_cli::CollatorOptions;
use cumulus_client_consensus_aura::{AuraConsensus, BuildAuraConsensusParams, SlotProportion};
use cumulus_client_consensus_common::ParachainBlockImport;
use cumulus_client_network::BlockAnnounceValidator;
use cumulus_client_service::{
	prepare_node_config, start_collator, start_full_node, StartCollatorParams, StartFullNodeParams,
};
use cumulus_primitives_core::ParaId;
use cumulus_relay_chain_inprocess_interface::build_inprocess_relay_chain;
use cumulus_relay_chain_interface::{RelayChainInterface, RelayChainResult};
use cumulus_relay_chain_minimal_node::build_minimal_relay_chain_node;
use polkadot_service::CollatorPair;
use sc_consensus::ImportQueue;
use sc_network_common::service::NetworkBlock;
// Substrate Imports
use crate::{
	chain_spec,
	client::{Client, FullBackend, FullClient},
	rpc,
	runtime::{
		assets::ExtendWithAssetsApi, cosmwasm::ExtendWithCosmwasmApi,
		crowdloan_rewards::ExtendWithCrowdloanRewardsApi, ibc::ExtendWithIbcApi,
		lending::ExtendWithLendingApi, pablo::ExtendWithPabloApi,
		staking_rewards::ExtendWithStakingRewardsApi, BaseHostRuntimeApis,
	},
};
use sc_client_api::{BlockBackend, StateBackendFor};
use sc_executor::NativeExecutionDispatch;
use sc_service::{Configuration, PartialComponents, TaskManager};
use sc_telemetry::{Telemetry, TelemetryHandle, TelemetryWorker, TelemetryWorkerHandle};
use sp_api::{ConstructRuntimeApi, StateBackend};
#[cfg(feature = "ocw")]
use sp_core::crypto::KeyTypeId;
use sp_runtime::traits::BlakeTwo256;
use sp_trie::PrefixedMemoryDB;

pub struct PicassoExecutor;

impl sc_executor::NativeExecutionDispatch for PicassoExecutor {
	type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		picasso_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		picasso_runtime::version::native_version()
	}
}

pub struct ComposableExecutor;

impl sc_executor::NativeExecutionDispatch for ComposableExecutor {
	type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		composable_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		composable_runtime::version::native_version()
	}
}

pub enum Executor {
	Picasso(PicassoExecutor),

	Composable(ComposableExecutor),
}

/// Starts a `ServiceBuilder` for a full service.
///
/// Use this macro if you don't actually need the full service, but just the builder in order to
/// be able to perform chain operations.
#[allow(clippy::type_complexity)]
pub fn new_chain_ops(
	config: &Configuration,
) -> Result<
	(
		Arc<Client>,
		Arc<FullBackend>,
		sc_consensus::BasicQueue<OpaqueBlock, PrefixedMemoryDB<BlakeTwo256>>,
		TaskManager,
	),
	sc_service::Error,
> {
	let components = match config.chain_spec.id() {
		chain if chain.contains("composable") => {
			let components = new_partial::<composable_runtime::RuntimeApi, ComposableExecutor>(
				config,
				Some(chain_spec::composable::DALEK_END_BLOCK),
			)?;
			(
				Arc::new(Client::from(components.client)),
				components.backend,
				components.import_queue,
				components.task_manager,
			)
		},
		chain if chain.contains("picasso") => {
			let components = new_partial::<picasso_runtime::RuntimeApi, PicassoExecutor>(
				config,
				Some(chain_spec::picasso::DALEK_END_BLOCK),
			)?;
			(
				Arc::new(Client::from(components.client)),
				components.backend,
				components.import_queue,
				components.task_manager,
			)
		},
		chain => panic!("Unknown chain: {}", chain),
	};

	Ok(components)
}

#[allow(clippy::type_complexity)]
pub fn new_partial<RuntimeApi, Executor>(
	config: &Configuration,
	// The block number until ed25519-dalek should be used for signature verification.
	dalek_end_block: Option<u32>,
) -> Result<
	PartialComponents<
		FullClient<RuntimeApi, Executor>,
		FullBackend,
		(),
		sc_consensus::DefaultImportQueue<OpaqueBlock, FullClient<RuntimeApi, Executor>>,
		sc_transaction_pool::FullPool<OpaqueBlock, FullClient<RuntimeApi, Executor>>,
		(
			Option<Telemetry>,
			Option<TelemetryWorkerHandle>,
			ParachainBlockImport<OpaqueBlock, Arc<FullClient<RuntimeApi, Executor>>, FullBackend>,
		),
	>,
	sc_service::Error,
>
where
	RuntimeApi:
		ConstructRuntimeApi<OpaqueBlock, FullClient<RuntimeApi, Executor>> + Send + Sync + 'static,
	RuntimeApi::RuntimeApi: BaseHostRuntimeApis<
		StateBackend = sc_client_api::StateBackendFor<FullBackend, OpaqueBlock>,
	>,
	sc_client_api::StateBackendFor<FullBackend, OpaqueBlock>: sp_api::StateBackend<BlakeTwo256>,
	Executor: sc_executor::NativeExecutionDispatch + 'static,
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

	let executor = sc_executor::NativeElseWasmExecutor::<Executor>::new(
		config.wasm_method,
		config.default_heap_pages,
		config.max_runtime_instances,
		config.runtime_cache_size,
	);

	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<OpaqueBlock, RuntimeApi, _>(
			config,
			telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
			executor,
		)?;
	let client = Arc::new(client);

	use sc_client_api::ExecutorProvider;
	client.execution_extensions().set_extensions_factory(
		sc_client_api::execution_extensions::ExtensionBeforeBlock::<
			crate::client::Block,
			sp_io::UseDalekExt,
		>::new(dalek_end_block.unwrap_or(0)),
	);
	let telemetry_worker_handle = telemetry.as_ref().map(|(worker, _)| worker.handle());

	let telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager.spawn_handle().spawn("telemetry", None, worker.run());
		telemetry
	});

	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_essential_handle(),
		client.clone(),
	);

	let (import_queue, parachain_block_import) = parachain_build_import_queue(
		backend.clone(),
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
		other: (telemetry, telemetry_worker_handle, parachain_block_import),
	};

	Ok(params)
}

/// Start the right parachain subsystem for the right chainspec.
pub async fn start_node(
	config: Configuration,
	polkadot_config: Configuration,
	collator_options: CollatorOptions,
	id: ParaId,
	sealing: Option<crate::cli::Sealing>,
) -> sc_service::error::Result<TaskManager> {
	let task_manager = match config.chain_spec.id() {
		chain if chain.contains("composable") =>
			crate::service::start_node_impl::<composable_runtime::RuntimeApi, ComposableExecutor>(
				config,
				polkadot_config,
				collator_options,
				id,
				Some(chain_spec::composable::DALEK_END_BLOCK),
				sealing,
			)
			.await?,
		chain if chain.contains("picasso") =>
			crate::service::start_node_impl::<picasso_runtime::RuntimeApi, PicassoExecutor>(
				config,
				polkadot_config,
				collator_options,
				id,
				Some(chain_spec::picasso::DALEK_END_BLOCK),
				sealing,
			)
			.await?,
		_ => panic!("Unknown chain_id: {}", config.chain_spec.id()),
	};

	Ok(task_manager)
}

/// Start a node with the given parachain `Configuration` and relay chain `Configuration`.
///
/// This is the actual implementation that is abstract over the executor and the runtime api.
#[sc_tracing::logging::prefix_logs_with("Parachain")]
async fn start_node_impl<RuntimeApi, Executor>(
	parachain_config: Configuration,
	polkadot_config: Configuration,
	collator_options: CollatorOptions,
	id: ParaId,
	// The block number until ed25519-dalek should be used for signature verification.
	dalek_end_block: Option<u32>,
	sealing: Option<crate::cli::Sealing>,
) -> sc_service::error::Result<TaskManager>
where
	RuntimeApi:
		ConstructRuntimeApi<OpaqueBlock, FullClient<RuntimeApi, Executor>> + Send + Sync + 'static,
	RuntimeApi::RuntimeApi: BaseHostRuntimeApis<StateBackend = StateBackendFor<FullBackend, OpaqueBlock>>
		+ ExtendWithStakingRewardsApi<RuntimeApi, Executor>
		+ ExtendWithAssetsApi<RuntimeApi, Executor>
		+ ExtendWithCrowdloanRewardsApi<RuntimeApi, Executor>
		+ ExtendWithPabloApi<RuntimeApi, Executor>
		+ ExtendWithLendingApi<RuntimeApi, Executor>
		+ ExtendWithCosmwasmApi<RuntimeApi, Executor>
		+ ExtendWithIbcApi<RuntimeApi, Executor>,
	StateBackendFor<FullBackend, OpaqueBlock>: StateBackend<BlakeTwo256>,
	Executor: NativeExecutionDispatch + 'static,
{
	let mut parachain_config = prepare_node_config(parachain_config);

	let params = new_partial::<RuntimeApi, Executor>(&parachain_config, dalek_end_block)?;

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

	let (mut telemetry, telemetry_worker_handle, parachain_block_import) = params.other;

	let client = params.client.clone();
	let backend = params.backend.clone();
	let mut task_manager = params.task_manager;

	let (relay_chain_interface, collator_key) = build_relay_chain_interface(
		polkadot_config,
		&parachain_config,
		telemetry_worker_handle,
		&mut task_manager,
		collator_options.clone(),
	)
	.await
	.map_err(|e| sc_service::Error::Application(Box::new(e) as Box<_>))?;

	let block_announce_validator = BlockAnnounceValidator::new(relay_chain_interface.clone(), id);

	let force_authoring = parachain_config.force_authoring;
	let validator = parachain_config.role.is_authority();
	let prometheus_registry = parachain_config.prometheus_registry().cloned();
	let transaction_pool = params.transaction_pool.clone();
	let import_queue_service = params.import_queue.service();

	let select_chain = sc_consensus::LongestChain::new(backend.clone());
	let (_grandpa_block_import, grandpa_link) = sc_finality_grandpa::block_import(
		client.clone(),
		&(client.clone() as Arc<_>),
		select_chain.clone(),
		telemetry.as_ref().map(|x| x.handle()),
	)?;

	let grandpa_protocol_name = sc_finality_grandpa::protocol_standard_name(
		&client.block_hash(0)?.expect("Genesis block exists; qed"),
		&parachain_config.chain_spec,
	);

	let warp_sync: Option<Arc<dyn sc_network::config::WarpSyncProvider<OpaqueBlock>>> =
		if sealing.is_some() {
			None
		} else {
			parachain_config
				.network
				.extra_sets
				.push(sc_finality_grandpa::grandpa_peers_set_config(grandpa_protocol_name.clone()));
			Some(Arc::new(sc_finality_grandpa::warp_proof::NetworkProvider::new(
				backend.clone(),
				grandpa_link.shared_authority_set().clone(),
				Vec::default(),
			)))
		};

	let (network, system_rpc_tx, tx_handler_controller, start_network) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &parachain_config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue: params.import_queue,
			warp_sync,
			block_announce_validator_builder: Some(Box::new(|_| {
				Box::new(block_announce_validator)
			})),
		})?;

	if parachain_config.offchain_worker.enabled {
		sc_service::build_offchain_workers(
			&parachain_config,
			task_manager.spawn_handle(),
			client.clone(),
			network.clone(),
		);
	}

	let rpc_builder = {
		let client = client.clone();
		let transaction_pool = transaction_pool.clone();
		let chain_props = parachain_config.chain_spec.properties();
		Box::new(move |deny_unsafe, _| {
			let deps = rpc::FullDeps {
				client: client.clone(),
				pool: transaction_pool.clone(),
				deny_unsafe,
				chain_props: chain_props.clone(),
			};

			Ok(rpc::create(deps).expect("RPC failed to initialize"))
		})
	};

	sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		rpc_builder,
		client: client.clone(),
		transaction_pool: transaction_pool.clone(),
		task_manager: &mut task_manager,
		config: parachain_config,
		keystore: params.keystore_container.sync_keystore(),
		backend: backend.clone(),
		network: network.clone(),
		tx_handler_controller,
		system_rpc_tx,
		telemetry: telemetry.as_mut(),
	})?;

	let announce_block = {
		let network = network.clone();
		Arc::new(move |hash, data| network.announce_block(hash, data))
	};

	let relay_chain_slot_duration = Duration::from_secs(6);

	if validator {
		let keystore = params.keystore_container.sync_keystore();
		let slot_duration = cumulus_client_consensus_aura::slot_duration(&*client)?;

		let proposer_factory = sc_basic_authorship::ProposerFactory::with_proof_recording(
			task_manager.spawn_handle(),
			client.clone(),
			transaction_pool,
			prometheus_registry.as_ref(),
			telemetry.as_ref().map(|t| t.handle()),
		);

		let backoff_authoring_blocks =
			Some(sc_consensus_slots::BackoffAuthoringOnFinalizedHeadLagging::default());

		let relay_chain_interface_inherent = relay_chain_interface.clone();
		let parachain_consensus =
			AuraConsensus::build::<sp_consensus_aura::sr25519::AuthorityPair, _, _, _, _, _, _>(
				BuildAuraConsensusParams {
					proposer_factory,
					create_inherent_data_providers: move |_, (relay_parent, validation_data)| {
						let relay_chain_interface_inherent = relay_chain_interface_inherent.clone();
						async move {
							let parachain_inherent = cumulus_primitives_parachain_inherent::ParachainInherentData::create_at(
							relay_parent,
							&relay_chain_interface_inherent,
							&validation_data,
							id,
						)
						.await;
							let time = sp_timestamp::InherentDataProvider::from_system_time();

							let slot =
							sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
								*time,
								slot_duration,
							);

							let parachain_inherent = parachain_inherent.ok_or_else(|| {
								Box::<dyn std::error::Error + Send + Sync>::from(
									"Failed to create parachain inherent",
								)
							})?;
							Ok((slot, time, parachain_inherent))
						}
					},
					block_import: parachain_block_import,
					para_client: client.clone(),
					backoff_authoring_blocks,
					sync_oracle: network,
					keystore,
					force_authoring,
					slot_duration,
					telemetry: telemetry.as_ref().map(|t| t.handle()),
					// We got around 500ms for proposing
					block_proposal_slot_portion: SlotProportion::new(1_f32 / 24_f32),
					// And a maximum of 750ms if slots are skipped
					max_block_proposal_slot_portion: Some(SlotProportion::new(1_f32 / 16_f32)),
				},
			);

		let spawner = task_manager.spawn_handle();

		let params = StartCollatorParams {
			para_id: id,
			relay_chain_interface: relay_chain_interface.clone(),
			block_status: client.clone(),
			announce_block,
			client: client.clone(),
			task_manager: &mut task_manager,
			spawner,
			parachain_consensus,
			import_queue: import_queue_service,
			collator_key: collator_key.expect("Command line arguments do not allow this. qed"),
			relay_chain_slot_duration,
		};

		start_collator(params).await?;
	} else {
		let params = StartFullNodeParams {
			client: client.clone(),
			announce_block,
			task_manager: &mut task_manager,
			para_id: id,
			relay_chain_interface,
			relay_chain_slot_duration,
			import_queue: import_queue_service,
		};

		start_full_node(params)?;
	}

	start_network.start_network();

	Ok(task_manager)
}

/// Build the import queue for the the parachain runtime.
#[allow(clippy::type_complexity)]
pub fn parachain_build_import_queue<RuntimeApi, Executor>(
	backend: Arc<FullBackend>,
	client: Arc<FullClient<RuntimeApi, Executor>>,
	config: &Configuration,
	telemetry: Option<TelemetryHandle>,
	task_manager: &TaskManager,
) -> Result<
	(
		sc_consensus::DefaultImportQueue<OpaqueBlock, FullClient<RuntimeApi, Executor>>,
		ParachainBlockImport<OpaqueBlock, Arc<FullClient<RuntimeApi, Executor>>, FullBackend>,
	),
	sc_service::Error,
>
where
	RuntimeApi:
		ConstructRuntimeApi<OpaqueBlock, FullClient<RuntimeApi, Executor>> + Send + Sync + 'static,
	RuntimeApi::RuntimeApi: BaseHostRuntimeApis<
		StateBackend = sc_client_api::StateBackendFor<FullBackend, OpaqueBlock>,
	>,
	Executor: NativeExecutionDispatch + 'static,
{
	let block_import = ParachainBlockImport::new(client.clone(), backend);
	let slot_duration = sc_consensus_aura::slot_duration(&*client)?;
	cumulus_client_consensus_aura::import_queue::<
		sp_consensus_aura::sr25519::AuthorityPair,
		_,
		_,
		_,
		_,
		_,
	>(cumulus_client_consensus_aura::ImportQueueParams {
		block_import: block_import.clone(),
		client,
		create_inherent_data_providers: move |_, _| async move {
			let time = sp_timestamp::InherentDataProvider::from_system_time();

			let slot =
				sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
					*time,
					slot_duration,
				);

			Ok((slot, time))
		},
		registry: config.prometheus_registry(),
		spawner: &task_manager.spawn_essential_handle(),
		telemetry,
	})
	.map(|shared| (shared, block_import))
	.map_err(Into::into)
}

async fn build_relay_chain_interface(
	polkadot_config: Configuration,
	parachain_config: &Configuration,
	telemetry_worker_handle: Option<TelemetryWorkerHandle>,
	task_manager: &mut TaskManager,
	collator_options: CollatorOptions,
) -> RelayChainResult<(Arc<(dyn RelayChainInterface + 'static)>, Option<CollatorPair>)> {
	if !collator_options.relay_chain_rpc_urls.is_empty() {
		build_minimal_relay_chain_node(
			polkadot_config,
			task_manager,
			collator_options.relay_chain_rpc_urls,
		)
		.await
	} else {
		build_inprocess_relay_chain(
			polkadot_config,
			parachain_config,
			telemetry_worker_handle,
			task_manager,
			None,
		)
	}
}
