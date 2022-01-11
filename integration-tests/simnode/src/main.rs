#![deny(unused_extern_crates, missing_docs)]

//! Basic example of end to end runtime tests.
mod chain;
mod cli;
mod events;
mod tests;

pub use chain::*;
use sc_cli::{CliConfiguration, SubstrateCli};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let cli = node::cli::Cli::from_args();
	let chain_id = cli.run.base.chain_id(false)?;

	match &*chain_id {
		"picasso" => picasso::run()?,
		"dali-chachacha" => dali::run()?,
		_ => panic!("Unsupported chai: {}", chain_id),
	};

	Ok(())
}
