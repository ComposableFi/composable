extern crate clap;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate thiserror;

use anyhow::Result;
use clap::Parser;
use hyperspace::logging;
use metrics::{data::Metrics, handler::MetricsHandler, init_prometheus};
use prometheus::Registry;
use std::path::PathBuf;

mod chain;

#[derive(Deserialize)]
pub struct Config {
	chain_a: AnyConfig,
	chain_b: AnyConfig,
	pub core: CoreConfig,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AnyConfig {
	#[cfg(feature = "parachain")]
	Parachain(parachain::ParachainClientConfig),
}

#[derive(Deserialize)]
pub struct CoreConfig {
	pub prometheus_endpoint: String,
}

#[derive(Debug, Parser)]
pub struct Cli {
	#[structopt(subcommand)]
	pub subcommand: Subcommand,
}

/// Possible subcommands of the main binary.
#[derive(Debug, Parser)]
pub enum Subcommand {
	Relay(RelayCmd),
}

/// The `relay` command
#[derive(Debug, Clone, Parser)]
#[clap(name = "relay", about = "Start relaying messages between two chains")]
pub struct RelayCmd {
	/// Relayer config path.
	#[clap(long)]
	config: String,
}

impl RelayCmd {
	/// Run the command
	pub async fn run(&self) -> Result<()> {
		let path: PathBuf = self.config.parse()?;
		let file_content = tokio::fs::read_to_string(path).await?;
		let config: Config = toml::from_str(&file_content)?;
		let any_chain_a = config.chain_a.into_client().await?;
		let any_chain_b = config.chain_b.into_client().await?;

		let registry =
			Registry::new_custom(None, None).expect("this can only fail if the prefix is empty");
		let addr = config.core.prometheus_endpoint.parse().unwrap();
		let metrics_a = Metrics::register(any_config_a.name(), &registry)?;
		let metrics_b = Metrics::register(any_config_b.name(), &registry)?;
		let mut metrics_handler_a = MetricsHandler::new(registry.clone(), metrics_a);
		let mut metrics_handler_b = MetricsHandler::new(registry.clone(), metrics_b);
		metrics_handler_a.link_with_counterparty(&mut metrics_handler_b);
		tokio::spawn(init_prometheus(addr, registry.clone()));

		hyperspace::relay(
			any_chain_a,
			any_chain_b,
			Some(metrics_handler_a),
			Some(metrics_handler_b),
		)
		.await
	}
}

#[tokio::main]
async fn main() -> Result<()> {
	logging::setup_logging();
	let cli = Cli::parse();

	match &cli.subcommand {
		Subcommand::Relay(cmd) => cmd.run().await,
	}
}
