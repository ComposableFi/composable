pub mod cosmwasm;
pub mod rpc;
pub mod subxt_api;
pub mod tx;
pub mod types;

use crate::{
	args::{QueryCommand, TxCommand},
	error::Error,
};
use anyhow::anyhow;
use clap::{Args, Subcommand};
use sp_keyring::sr25519::Keyring;
use std::{fmt::Display, str::FromStr};
use subxt::ext::{
	sp_core::{ed25519, sr25519, Pair},
	sp_runtime::{MultiSignature, MultiSigner},
};

use self::{rpc::QueryCommandRunner, tx::CommandRunner};

/// Interact with the CosmWasm contracts on a substrate-based chain.
#[derive(Args, Debug)]
pub struct Command {
	/// Name of the development account that will be used as signer. (eg. alice)
	#[arg(short, long, value_parser = parse_keyring, conflicts_with_all = &["seed", "mnemonic", "scheme"])]
	from: Option<Keyring>,

	/// Secret seed of the signer
	#[arg(short, long, conflicts_with_all = &["from", "mnemonic"])]
	seed: Option<String>,

	/// Mnemonic of the signer
	#[arg(short, long, conflicts_with_all = &["from", "seed"])]
	mnemonic: Option<String>,

	/// Signature scheme. (eg. sr25519, ed25519)
	#[arg(long, default_value_t = KeyScheme::Sr25519)]
	scheme: KeyScheme,

	/// Password for the mnemonic
	#[arg(short, long)]
	password: Option<String>,

	/// Websocket endpoint of the substrate chain
	#[arg(long, default_value_t = String::from("ws://127.0.0.1:9944"))]
	node: String,

	#[arg(long, default_value_t = OutputType::Text)]
	output: OutputType,

	#[command(subcommand)]
	subcommand: Subcommands,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
	Tx(TxCommand),
	Rpc(QueryCommand),
}

#[derive(Debug, Copy, Clone)]
pub enum KeyScheme {
	Sr25519,
	Ed25519,
}

impl FromStr for KeyScheme {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, String> {
		let scheme = match s {
			"sr25519" => KeyScheme::Sr25519,
			"ed25519" => KeyScheme::Ed25519,
			_ => return Err("unknown scheme".into()),
		};
		Ok(scheme)
	}
}

impl Display for KeyScheme {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let scheme = match self {
			KeyScheme::Sr25519 => "sr25519",
			KeyScheme::Ed25519 => "ed25519",
		};
		write!(f, "{scheme}")
	}
}

#[derive(Debug, Copy, Clone)]
pub enum OutputType {
	Text,
	Json,
}

impl FromStr for OutputType {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, String> {
		let output_type = match s {
			"text" => OutputType::Text,
			"json" => OutputType::Json,
			_ => return Err("unknown output type".into()),
		};
		Ok(output_type)
	}
}

impl Display for OutputType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let output_type = match self {
			OutputType::Text => "text",
			OutputType::Json => "json",
		};
		write!(f, "{output_type}")
	}
}

pub struct CosmosCommandRunner;

impl CosmosCommandRunner {
	pub async fn run(command: Command) -> anyhow::Result<()> {
		match command.scheme {
			KeyScheme::Sr25519 => Self::dispatch_command::<sr25519::Pair>(command).await,
			KeyScheme::Ed25519 => Self::dispatch_command::<ed25519::Pair>(command).await,
		}
	}

	async fn dispatch_command<P: Pair>(command: Command) -> anyhow::Result<()>
	where
		P::Seed: TryFrom<Vec<u8>>,
		MultiSignature: From<<P as Pair>::Signature>,
		MultiSigner: From<<P as Pair>::Public>,
		subxt::utils::MultiSignature: From<<P as sp_core::Pair>::Signature>,
	{
		match command.subcommand {
			Subcommands::Rpc(subcommand) =>
				QueryCommandRunner::run(subcommand, command.node, command.output).await,
			Subcommands::Tx(subcommand) => {
				let Some(pair) = get_signer_pair::<P>(command.from, command.mnemonic, command.seed, command.password)? else {
                    return Err(anyhow!("{}", Error::OperationNeedsToBeSigned));
                };
				CommandRunner::run(subcommand, pair, command.node, command.output).await
			},
		}
	}
}

fn get_signer_pair<P: Pair>(
	name: Option<Keyring>,
	mnemonic: Option<String>,
	seed: Option<String>,
	password: Option<String>,
) -> anyhow::Result<Option<P>>
where
	P::Seed: TryFrom<Vec<u8>>,
{
	let pair = if let Some(name) = name {
		P::from_string(&format!("//{}", name), None)
			.map_err(|_| anyhow!("{}", Error::InvalidSeed))?
	} else if let Some(mnemonic) = mnemonic {
		let (pair, _) = P::from_phrase(&mnemonic, password.as_deref())
			.map_err(|_| anyhow!("{}", Error::InvalidPhrase))?;
		pair
	} else if let Some(seed) = seed {
		P::from_string(&seed, None).map_err(|_| anyhow!("{}", Error::InvalidSeed))?
	} else {
		return Ok(None)
	};

	Ok(Some(pair))
}

pub fn parse_keyring(s: &str) -> Result<Keyring, String> {
	Keyring::from_str(s).map_err(|_| Error::InvalidAddress.to_string())
}
