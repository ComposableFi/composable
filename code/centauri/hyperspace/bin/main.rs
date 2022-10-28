use anyhow::Result;
use clap::Parser;
use hyperspace::logging;
use metrics::{data::Metrics, handler::MetricsHandler, init_prometheus};
use primitives::Chain;
use prometheus::Registry;
use std::{path::PathBuf, str::FromStr, time::Duration};

mod chain;

use chain::Config;
use ibc::core::{ics04_channel::channel::Order, ics24_host::identifier::PortId};
use primitives::{
	utils::{create_channel, create_clients, create_connection},
	IbcProvider,
};

#[derive(Debug, Parser)]
pub struct Cli {
	#[structopt(subcommand)]
	pub subcommand: Subcommand,
}

/// Possible subcommands of the main binary.
#[derive(Debug, Parser)]
pub enum Subcommand {
	#[clap(name = "relay", about = "Start relaying messages between two chains")]
	Relay(Cmd),
	#[clap(name = "create-clients", about = "Creates light clients on both chains")]
	CreateClients(Cmd),
	#[clap(name = "create-connection", about = "Creates a connection between both chains")]
	CreateConnection(Cmd),
	#[clap(name = "create-channel", about = "Creates a channel on the specified port")]
	CreateChannel(Cmd),
}

#[derive(Debug, Clone, Parser)]
pub struct Cmd {
	/// Relayer config path.
	#[clap(long)]
	config: String,
	/// Port id for channel creation
	#[clap(long)]
	port_id: Option<String>,
	/// Connection delay period in seconds
	#[clap(long)]
	#[clap(long)]
	delay_period: Option<u32>,
	/// Channel order
	#[clap(long)]
	order: Option<String>,
	/// Channel version
	#[clap(long)]
	version: Option<String>,
}

impl Cmd {
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
		let metrics_a = Metrics::register(any_chain_a.name(), &registry)?;
		let metrics_b = Metrics::register(any_chain_b.name(), &registry)?;
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

	pub async fn create_clients(&self) -> Result<()> {
		let path: PathBuf = self.config.parse()?;
		let file_content = tokio::fs::read_to_string(path).await?;
		let config: Config = toml::from_str(&file_content)?;
		let any_chain_a = config.chain_a.into_client().await?;
		let any_chain_b = config.chain_b.into_client().await?;

		let (client_id_a_on_b, client_id_b_on_a) =
			create_clients(&any_chain_a, &any_chain_b).await?;
		log::info!(
			"ClientId for Chain {} on Chain {}: {}",
			any_chain_b.name(),
			any_chain_a.name(),
			client_id_b_on_a
		);
		log::info!(
			"ClientId for Chain {} on Chain {}: {}",
			any_chain_a.name(),
			any_chain_b.name(),
			client_id_a_on_b
		);
		Ok(())
	}

	pub async fn create_connection(&self) -> Result<()> {
		let delay = self
			.delay_period
			.expect("delay_period should be provided when creating a connection");
		let delay = Duration::from_secs(delay.into());
		let path: PathBuf = self.config.parse()?;
		let file_content = tokio::fs::read_to_string(path).await?;
		let config: Config = toml::from_str(&file_content)?;
		let any_chain_a = config.chain_a.into_client().await?;
		let any_chain_b = config.chain_b.into_client().await?;

		let any_chain_a_clone = any_chain_a.clone();
		let any_chain_b_clone = any_chain_b.clone();
		let handle = tokio::task::spawn(async move {
			hyperspace::relay(any_chain_a_clone, any_chain_b_clone, None, None)
				.await
				.unwrap();
		});

		let (connection_id_a, connection_id_b) =
			create_connection(&any_chain_a, &any_chain_b, delay).await?;
		log::info!("ConnectionId on Chain {}: {}", any_chain_a.name(), connection_id_a);
		log::info!("ConnectionId on Chain {}: {}", any_chain_b.name(), connection_id_b);
		handle.abort();
		Ok(())
	}

	pub async fn create_channel(&self) -> Result<()> {
		let port_id = PortId::from_str(
			self.port_id
				.as_ref()
				.expect("port_id must be specified when creating a channel")
				.as_str(),
		)
		.expect("Port id was invalid");
		let version = self
			.version
			.as_ref()
			.expect("version must be specified when creating a channel")
			.clone();
		let order = self.order.as_ref().expect("order must be specified when creating a channel, expected one of 'ordered' or 'unordered'").as_str();
		let path: PathBuf = self.config.parse()?;
		let file_content = tokio::fs::read_to_string(path).await?;
		let config: Config = toml::from_str(&file_content)?;
		let any_chain_a = config.chain_a.into_client().await?;
		let any_chain_b = config.chain_b.into_client().await?;

		let any_chain_a_clone = any_chain_a.clone();
		let any_chain_b_clone = any_chain_b.clone();
		let handle = tokio::task::spawn(async move {
			hyperspace::relay(any_chain_a_clone, any_chain_b_clone, None, None)
				.await
				.unwrap();
		});

		let order = Order::from_str(order).expect("Expected one of 'ordered' or 'unordered'");
		let (channel_id_a, channel_id_b) = create_channel(
			&any_chain_a,
			&any_chain_b,
			any_chain_a.connection_id(),
			port_id,
			version,
			order,
		)
		.await?;
		log::info!("ChannelId on Chain {}: {}", any_chain_a.name(), channel_id_a);
		log::info!("ChannelId on Chain {}: {}", any_chain_b.name(), channel_id_b);
		handle.abort();
		Ok(())
	}
}

#[tokio::main]
async fn main() -> Result<()> {
	logging::setup_logging();
	let cli = Cli::parse();

	match &cli.subcommand {
		Subcommand::Relay(cmd) => cmd.run().await,
		Subcommand::CreateClients(cmd) => cmd.create_clients().await,
		Subcommand::CreateConnection(cmd) => cmd.create_connection().await,
		Subcommand::CreateChannel(cmd) => cmd.create_channel().await,
	}
}
