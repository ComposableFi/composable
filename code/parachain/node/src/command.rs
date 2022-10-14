// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
#[cfg(feature = "composable")]
use crate::service::ComposableExecutor;
#[cfg(feature = "dali")]
use crate::service::DaliExecutor;

use crate::{
	chain_spec,
	cli::{Cli, RelayChainCli, Subcommand},
	service::{new_chain_ops, new_partial, PicassoExecutor},
};
use cumulus_primitives_core::ParaId;
use frame_benchmarking_cli::BenchmarkCmd;
use log::info;
use picasso_runtime::Block;
use sc_cli::{
	ChainSpec, CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams,
	NetworkParams, Result, RuntimeVersion, SharedParams, SubstrateCli,
};
use sc_service::config::{BasePath, PrometheusConfig};
use sp_runtime::traits::AccountIdConversion;
use std::net::SocketAddr;

fn load_spec(id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
	Ok(match id {
		// Must define the default chain here because `export-genesis-state` command
		// does not support `--chain` and `--parachain-id` arguments simultaneously.
		#[cfg(feature = "dali")]
		id if id.contains("dali") => Box::new(chain_spec::dali_dev(id)),
		"picasso-dev" => Box::new(chain_spec::picasso_dev()),
		#[cfg(feature = "composable")]
		"composable-dev" => Box::new(chain_spec::composable_dev()),
		// Dali (Rococo Relay)
		#[cfg(feature = "dali")]
		"dali-rococo" => Box::new(chain_spec::dali_rococo()),
		// Picasso (Kusama Relay)
		"picasso" => Box::new(chain_spec::picasso()),
		// Composable (Westend Relay)
		#[cfg(feature = "composable")]
		"composable-westend" => Box::new(chain_spec::composable_westend()),
		// Composable (Polkadot Relay)
		#[cfg(feature = "composable")]
		"" | "composable" => Box::new(chain_spec::composable()),
		path => Box::new(chain_spec::picasso::ChainSpec::from_json_file(
			std::path::PathBuf::from(path),
		)?),
	})
}

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Composable".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/ComposableFi/composable/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2020
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		load_spec(id)
	}

	fn native_runtime_version(spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		match spec.id() {
			// composable-chains
			#[cfg(feature = "composable")]
			chain if chain.contains("composable") => &composable_runtime::VERSION,
			// dali chains
			#[cfg(feature = "dali")]
			chain if chain.contains("dali") => &dali_runtime::VERSION,
			// picasso chains
			chain if chain.contains("picasso") => &picasso_runtime::VERSION,
			_ => panic!("Unknown chain_id: {}", spec.id()),
		}
	}
}

impl SubstrateCli for RelayChainCli {
	fn impl_name() -> String {
		"Parachain Collator Template".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		"Parachain Collator Template\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relaychain node.\n\n\
		parachain-collator [parachain-args] -- [relaychain-args]"
			.into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/substrate-developer-hub/substrate-parachain-template/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2017
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		polkadot_cli::Cli::from_iter([RelayChainCli::executable_name()].iter()).load_spec(id)
	}

	fn native_runtime_version(chain_spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		polkadot_cli::Cli::native_runtime_version(chain_spec)
	}
}

macro_rules! construct_async_run {
	(|$components:ident, $cli:ident, $cmd:ident, $config:ident| $( $code:tt )* ) => {{
		let runner = $cli.create_runner($cmd)?;
		runner.async_run(|$config| {
			let $components = new_chain_ops(
				&$config,
			)?;
			let task_manager = $components.3;
			{ $( $code )* }.map(|v| (v, task_manager))
		})
	}}
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.0, components.2))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.0, config.database))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.0, config.chain_spec))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.0, components.2))
			})
		},
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			runner.sync_run(|config| {
				let polkadot_cli = RelayChainCli::new(
					&config,
					[RelayChainCli::executable_name()].iter().chain(cli.relaychain_args.iter()),
				);

				let polkadot_config = SubstrateCli::create_configuration(
					&polkadot_cli,
					&polkadot_cli,
					config.tokio_handle.clone(),
				)
				.map_err(|err| format!("Relay chain argument error: {}", err))?;

				cmd.run(config, polkadot_config)
			})
		},
		Some(Subcommand::Revert(cmd)) => construct_async_run!(|components, cli, cmd, config| {
			Ok(cmd.run(components.0, components.1, None))
		}),
		Some(Subcommand::ExportGenesisState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|_config| {
				let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
				let state_version = Cli::native_runtime_version(&spec).state_version();
				cmd.run::<Block>(&*spec, state_version)
			})
		},
		Some(Subcommand::ExportGenesisWasm(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|_config| {
				let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
				cmd.run(&*spec)
			})
		},
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			match cmd {
				BenchmarkCmd::Pallet(cmd) =>
					if cfg!(feature = "runtime-benchmarks") {
						runner.sync_run(|config| match config.chain_spec.id() {
							id if id.contains("picasso") =>
								cmd.run::<Block, PicassoExecutor>(config),
							#[cfg(feature = "dali")]
							id if id.contains("dali") => cmd.run::<Block, DaliExecutor>(config),
							#[cfg(feature = "composable")]
							id if id.contains("composable") => cmd.run::<Block, ComposableExecutor>(config),
							id => panic!("Unknown Chain: {}", id),
						})
					} else {
						Err("Benchmarking wasn't enabled when building the node. \
						     You can enable it with `--features runtime-benchmarks`."
							.into())
					},
				BenchmarkCmd::Block(cmd) =>
					runner.sync_run(|config| match config.chain_spec.id() {
						id if id.contains("picasso") => {
							let partials = new_partial::<
								picasso_runtime::RuntimeApi,
								PicassoExecutor,
							>(&config)?;
							cmd.run(partials.client)
						},
						#[cfg(feature = "dali")]
						id if id.contains("dali") => {
							let partials =
								new_partial::<dali_runtime::RuntimeApi, DaliExecutor>(&config)?;
							cmd.run(partials.client)
						},
						#[cfg(feature = "composable")]
						id if id.contains("composable") => {
							let partials = new_partial::<
								composable_runtime::RuntimeApi,
								ComposableExecutor,
							>(&config)?;
							cmd.run(partials.client)
						},
						id => panic!("Unknown Chain: {}", id),
					}),
				BenchmarkCmd::Storage(cmd) =>
					runner.sync_run(|config| match config.chain_spec.id() {
						id if id.contains("picasso") => {
							let partials = new_partial::<
								picasso_runtime::RuntimeApi,
								PicassoExecutor,
							>(&config)?;
							let db = partials.backend.expose_db();
							let storage = partials.backend.expose_storage();
							cmd.run(config, partials.client, db, storage)
						},
						#[cfg(feature = "dali")]
						id if id.contains("dali") => {
							let partials =
								new_partial::<dali_runtime::RuntimeApi, DaliExecutor>(&config)?;
							let db = partials.backend.expose_db();
							let storage = partials.backend.expose_storage();
							cmd.run(config, partials.client, db, storage)
						},
						#[cfg(feature = "composable")]
						id if id.contains("composable") => {
							let partials = new_partial::<
								composable_runtime::RuntimeApi,
								ComposableExecutor,
							>(&config)?;
							let db = partials.backend.expose_db();
							let storage = partials.backend.expose_storage();
							cmd.run(config, partials.client, db, storage)
						},
						id => panic!("Unknown Chain: {}", id),
					}),

				BenchmarkCmd::Overhead(_) |
				BenchmarkCmd::Extrinsic(_) |
				BenchmarkCmd::Machine(_) => Err("Unsupported benchmarking command".into()),
			}
		},
		None => {
			let runner = cli.create_runner(&cli.run.normalize())?;
			let collator_options = cli.run.collator_options();

			runner.run_node_until_exit(|config| async move {
				let _ = &cli;
				let para_id = chain_spec::Extensions::try_get(&*config.chain_spec)
					.map(|e| e.para_id)
					.ok_or("Could not find parachain extension in chain-spec.")?;

				let polkadot_cli = RelayChainCli::new(
					&config,
					[RelayChainCli::executable_name()].iter().chain(cli.relaychain_args.iter()),
				);

				let id = ParaId::from(para_id);

				let parachain_account =
					AccountIdConversion::<polkadot_primitives::v2::AccountId>::into_account_truncating(&id);

				let tokio_handle = config.tokio_handle.clone();
				let polkadot_config =
					SubstrateCli::create_configuration(&polkadot_cli, &polkadot_cli, tokio_handle)
						.map_err(|err| format!("Relay chain argument error: {}", err))?;

				info!("Parachain id: {:?}", id);
				info!("Parachain Account: {}", parachain_account);
				info!("Is collating: {}", if config.role.is_authority() { "yes" } else { "no" });

				Ok(crate::service::start_node(config, polkadot_config, collator_options, id)
					.await?)
			})
		},
	}
}

impl DefaultConfigurationValues for RelayChainCli {
	fn p2p_listen_port() -> u16 {
		30334
	}

	fn rpc_ws_listen_port() -> u16 {
		9945
	}

	fn rpc_http_listen_port() -> u16 {
		9934
	}

	fn prometheus_listen_port() -> u16 {
		9616
	}
}

impl CliConfiguration<Self> for RelayChainCli {
	fn shared_params(&self) -> &SharedParams {
		self.base.base.shared_params()
	}

	fn import_params(&self) -> Option<&ImportParams> {
		self.base.base.import_params()
	}

	fn network_params(&self) -> Option<&NetworkParams> {
		self.base.base.network_params()
	}

	fn keystore_params(&self) -> Option<&KeystoreParams> {
		self.base.base.keystore_params()
	}

	fn base_path(&self) -> Result<Option<BasePath>> {
		Ok(self
			.shared_params()
			.base_path()
			.or_else(|| self.base_path.clone().map(Into::into)))
	}

	fn rpc_http(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
		self.base.base.rpc_http(default_listen_port)
	}

	fn rpc_ipc(&self) -> Result<Option<String>> {
		self.base.base.rpc_ipc()
	}

	fn rpc_ws(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
		self.base.base.rpc_ws(default_listen_port)
	}

	fn prometheus_config(
		&self,
		default_listen_port: u16,
		chain_spec: &Box<dyn ChainSpec>,
	) -> Result<Option<PrometheusConfig>> {
		self.base.base.prometheus_config(default_listen_port, chain_spec)
	}

	fn init<F>(
		&self,
		_support_url: &String,
		_impl_version: &String,
		_logger_hook: F,
		_config: &sc_service::Configuration,
	) -> Result<()>
	where
		F: FnOnce(&mut sc_cli::LoggerBuilder, &sc_service::Configuration),
	{
		unreachable!("PolkadotCli is never initialized; qed");
	}

	fn chain_id(&self, is_dev: bool) -> Result<String> {
		let chain_id = self.base.base.chain_id(is_dev)?;

		Ok(if chain_id.is_empty() { self.chain_id.clone().unwrap_or_default() } else { chain_id })
	}

	fn role(&self, is_dev: bool) -> Result<sc_service::Role> {
		self.base.base.role(is_dev)
	}

	fn transaction_pool(&self, x: bool) -> Result<sc_service::config::TransactionPoolOptions> {
		self.base.base.transaction_pool(x)
	}

	fn state_cache_child_ratio(&self) -> Result<Option<usize>> {
		self.base.base.state_cache_child_ratio()
	}

	fn rpc_methods(&self) -> Result<sc_service::config::RpcMethods> {
		self.base.base.rpc_methods()
	}

	fn rpc_ws_max_connections(&self) -> Result<Option<usize>> {
		self.base.base.rpc_ws_max_connections()
	}

	fn rpc_cors(&self, is_dev: bool) -> Result<Option<Vec<String>>> {
		self.base.base.rpc_cors(is_dev)
	}

	fn default_heap_pages(&self) -> Result<Option<u64>> {
		self.base.base.default_heap_pages()
	}

	fn force_authoring(&self) -> Result<bool> {
		self.base.base.force_authoring()
	}

	fn disable_grandpa(&self) -> Result<bool> {
		self.base.base.disable_grandpa()
	}

	fn max_runtime_instances(&self) -> Result<Option<usize>> {
		self.base.base.max_runtime_instances()
	}

	fn announce_block(&self) -> Result<bool> {
		self.base.base.announce_block()
	}

	fn telemetry_endpoints(
		&self,
		chain_spec: &Box<dyn ChainSpec>,
	) -> Result<Option<sc_telemetry::TelemetryEndpoints>> {
		self.base.base.telemetry_endpoints(chain_spec)
	}
}
