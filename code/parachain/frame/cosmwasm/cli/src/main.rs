mod args;
mod error;
mod substrate;

use args::{CosmosCommand, CosmosSubcommand};
use clap::Parser;
use substrate::CosmosCommandRunner;

#[tokio::main]
async fn main() {
	let args = CosmosCommand::parse();

	let result = match args.subcommand {
		CosmosSubcommand::Substrate(command) => CosmosCommandRunner::run(command).await,
	};

	if let Err(e) = result {
		eprintln!("{}", e);
	}
}
