pub use chains::*;
use sc_cli::{CliConfiguration, SubstrateCli};
use std::error::Error;

mod chains;
mod tests;

fn main() -> Result<(), Box<dyn Error>> {
	let cli = node::cli::Cli::from_args();
	let chain_id = cli.run.base.chain_id(false)?;

	match &*chain_id {
		chain if chain.contains("picasso") => picasso::run()?,
		chain if chain.contains("dali") => dali::run()?,
		chain if chain.contains("composable") => composable::run()?,
		_ => panic!("Unsupported chain_id: {}", chain_id),
	};

	Ok(())
}
