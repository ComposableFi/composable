use crate::sproof::ParachainInherentSproof;
use crate::PicassoChainInfo;
use node::cli::Cli;
use sc_cli::{CliConfiguration, SubstrateCli};
use sc_consensus_manual_seal::consensus::aura::AuraConsensusDataProvider;
use sc_consensus_manual_seal::consensus::timestamp::SlotTimestampProvider;
use std::error::Error;
use std::future::Future;
use structopt::StructOpt;
use test_runner::{build_runtime, client_parts, ConfigOrChainSpec, Node};

/// Runs the test-runner as a binary.
pub fn run<F, Fut>(callback: F) -> Result<(), Box<dyn Error>>
where
	F: FnOnce(Node<PicassoChainInfo>) -> Fut,
	Fut: Future<Output = Result<(), Box<dyn Error>>>,
{
	let tokio_runtime = build_runtime()?;
	// parse cli args
	let cmd = <Cli as StructOpt>::from_args();
	// set up logging
	let filters = cmd.run.base.log_filters()?;
	let logger = sc_tracing::logging::LoggerBuilder::new(filters);
	logger.init()?;

	// set up the test-runner
	let config = cmd.create_configuration(&cmd.run.base, tokio_runtime.handle().clone())?;
	sc_cli::print_node_infos::<Cli>(&config);

	let (rpc, task_manager, client, pool, command_sink, backend) =
		client_parts::<PicassoChainInfo, _>(
			ConfigOrChainSpec::Config(config),
			|client, _sc, _keystore| {
				let cloned_client = client.clone();
				let create_inherent_data_providers = Box::new(move |_, _| {
					let client = cloned_client.clone();
					let mut parachain_sproof = ParachainInherentSproof::new(client.clone());
					async move {
						let timestamp = SlotTimestampProvider::aura(client.clone())
							.map_err(|err| format!("{:?}", err))?;

						let _aura = sp_consensus_aura::inherents::InherentDataProvider::new(
							timestamp.slot().into(),
						);

						let parachain_system = parachain_sproof.create_inherent(timestamp.slot());
						Ok((timestamp, _aura, parachain_system))
					}
				});
				let aura_provider = AuraConsensusDataProvider::new(client.clone());
				Ok((client, Some(Box::new(aura_provider)), create_inherent_data_providers))
			},
		)?;
	let node =
		Node::<PicassoChainInfo>::new(rpc, task_manager, client, pool, command_sink, backend);

	// hand off node.
	tokio_runtime.block_on(callback(node))?;

	Ok(())
}
