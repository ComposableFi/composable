use crate::{error::Error, substrate};
use clap::{Args, Parser, Subcommand};
use subxt::utils::AccountId32;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CosmosCommand {
	#[command(subcommand)]
	pub subcommand: CosmosSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum CosmosSubcommand {
	Substrate(substrate::Command),
}

#[derive(Args, Debug)]
pub struct QueryCommand {
	#[command(subcommand)]
	pub subcommands: QuerySubcommands,
}

#[derive(Debug, Subcommand)]
pub enum QuerySubcommands {
	/// Query a CosmWasm contract
	Query(crate::substrate::cosmwasm::Query),

	/// Dry-run an instantiate call
	#[group(skip)]
	Instantiate {
		/// Caller of the instantiate call
		#[arg(long)]
		sender: AccountId32,
		#[command(flatten)]
		instantiate: WasmInstantiate,
	},
}

#[derive(Args, Debug)]
pub struct TxCommand {
	#[command(subcommand)]
	pub subcommands: TxSubcommands,
}

#[derive(Debug, Subcommand)]
pub enum TxSubcommands {
	/// Upload a CosmWasm contract
	Store(crate::substrate::cosmwasm::Upload),

	/// Instantiate a CosmWasm contract
	Instantiate(WasmInstantiate),

	/// Execute a CosmWasm contract
	Execute(crate::substrate::cosmwasm::Execute),

	/// Migrate a CosmWasm contract
	Migrate(crate::substrate::cosmwasm::Migrate),

	/// Update admin of a CosmWasm contract
	UpdateAdmin(crate::substrate::cosmwasm::UpdateAdmin),
}

#[derive(Args, Clone, Debug)]
pub struct WasmInstantiate {
	/// Code ID of the code that will be used to instantiate the contract
	#[arg()]
	pub code_id_int64: u64,

	/// Instantiate message
	#[arg()]
	pub json_encoded_init_args: String,

	/// Additional data to be used in contract address derivation in case you want to
	/// instantiate the same contract with the same message and label multiple times
	#[arg(short, long)]
	pub salt: String,
	/// Contract's admin
	#[arg(short, long)]
	pub admin: Option<AccountId32>,
	/// Human-readable label of the contract
	#[arg(short, long)]
	pub label: String,
	/// Funds to be moved prior to execution. Format is "ASSET-1:AMOUNT-1,ASSET-2:AMOUNT-2"
	#[arg(long, value_parser = parse_funds)]
	pub amount: Option<Vec<(u128, u128)>>,
	/// Gas limit
	#[arg(short, long)]
	pub gas: u64,
}

pub fn parse_funds(funds_str: &str) -> Result<Option<Vec<(u128, u128)>>, String> {
	let mut funds = Vec::new();
	for asset in funds_str.split(',') {
		let asset: Vec<&str> = asset.split(':').collect();
		if asset.len() != 2 {
			return Err(Error::InvalidFundsFormat.to_string())
		}
		funds.push((
			asset[0].parse().map_err(|_| Error::InvalidFundsFormat.to_string())?,
			asset[1].parse().map_err(|_| Error::InvalidFundsFormat.to_string())?,
		));
	}
	Ok(Some(funds))
}
