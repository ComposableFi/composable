mod args;
mod error;
mod new_cmd;
mod substrate;

use args::{Args, Command};
use clap::Parser;

#[tokio::main]
async fn main() {
	let args = Args::parse();

	let result = match args.main_command {
		Command::Substrate(substrate_command) => substrate_command.run().await,
		Command::New(new_command) => new_command.run(),
	};

	if let Err(e) = result {
		eprintln!("{}", e);
	}
}
