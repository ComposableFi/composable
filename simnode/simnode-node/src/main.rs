use sc_cli::{CliConfiguration, SubstrateCli};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let cli = node::cli::Cli::from_args();
	let chain_id = cli.run.base.chain_id(false)?;

	match &*chain_id {
		chain if chain.contains("picasso") =>
			substrate_simnode::parachain_node::<common::chains::picasso::ChainInfo, _, _>(
				|node| async move {
					node.seal_blocks(10).await;
					node.until_shutdown().await;
					Ok(())
				},
			)?,
		chain if chain.contains("dali") =>
			substrate_simnode::parachain_node::<common::chains::dali::ChainInfo, _, _>(
				|node| async move {
					node.seal_blocks(10).await;
					node.until_shutdown().await;
					Ok(())
				},
			)?,
		chain if chain.contains("composable") =>
			substrate_simnode::parachain_node::<common::chains::composable::ChainInfo, _, _>(
				|node| async move {
					node.seal_blocks(10).await;
					node.until_shutdown().await;
					Ok(())
				},
			)?,
		_ => panic!("Unsupported chain_id: {}", chain_id),
	};

	Ok(())
}
