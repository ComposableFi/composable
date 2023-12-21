use std::path::PathBuf;

use crate::{error::Error, substrate};
use clap::{ArgGroup, Args, Parser, Subcommand};
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
	Wasm(WasmRpcQuery),
}

#[derive(Args, Debug)]
pub struct TxCommand {
	#[command(subcommand)]
	pub subcommands: TxSubcommands,

	#[arg(long)]
	pub dry_run: Option<bool>,
}

#[derive(Debug, Subcommand)]
pub enum TxSubcommands {
	/// Upload a CosmWasm contract
	Store(StoreCommand),

	/// Instantiate a CosmWasm contract
	Instantiate2(WasmInstantiate2),

	/// Execute a CosmWasm contract
	Execute(Execute),

	/// Migrate a CosmWasm contract
	Migrate(Migrate),

	/// Update admin of a CosmWasm contract
	UpdateAdmin(UpdateAdmin),
}

#[derive(Args, Debug)]
pub struct StoreCommand {
	/// Path to local CosmWasm contract binary
	#[arg()]
	pub wasm_file: PathBuf,
}

#[derive(Args, Clone, Debug)]
pub struct WasmInstantiate {
	/// Code ID of the code that will be used to instantiate the contract
	#[arg()]
	pub code_id_int64: u64,

	/// Instantiate message
	#[arg()]
	pub json_encoded_init_args: String,

	/// Gas limit
	#[arg(short, long)]
	pub gas: u64,

	/// Contract's admin
	#[arg(short, long)]
	pub admin: Option<AccountId32>,
	/// Human-readable label of the contract
	#[arg(short, long)]
	pub label: String,
	/// Funds to be moved prior to execution. Format is "ASSET-1:AMOUNT-1,ASSET-2:AMOUNT-2"
	#[arg(short, long, value_parser = parse_funds, default_value = "")]
	pub funds: ::std::vec::Vec<(u128, u128)>,
}

#[derive(Args, Clone, Debug)]
pub struct WasmInstantiate2 {
	#[command(flatten)]
	pub instantiate: WasmInstantiate,

	/// Additional data to be used in contract address derivation in case you want to
	/// instantiate the same contract with the same message and label multiple times
	#[arg()]
	pub salt: String,
}

pub fn parse_funds(funds_str: &str) -> Result<Vec<(u128, u128)>, String> {
	let mut funds = Vec::new();
	if funds_str.is_empty() {
		return Ok(funds)
	}
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
	Ok(funds)
}

#[derive(Args, Debug)]
pub struct Execute {
	/// Contract to be executed
	#[arg(short, long)]
	pub contract: AccountId32,
	/// Funds to be moved prior to execution. Format is "ASSET-1:AMOUNT-1,ASSET-2:AMOUNT-2"
	#[arg(short, long, value_parser = parse_funds, default_value = "")]
	pub funds: ::std::vec::Vec<(u128, u128)>,
	/// Execute message
	#[arg(short, long)]
	pub message: String,

	/// Gas limit
	#[arg(short, long)]
	pub gas: u64,
}

#[derive(Args, Debug)]
pub struct Migrate {
	/// Contract to be migrated
	#[arg(short, long)]
	pub contract: AccountId32,
	/// The new code ID to use
	#[arg(short, long)]
	pub new_code_id: u64,
	/// Migrate message
	#[arg(short, long)]
	pub message: String,

	/// Gas limit
	#[arg(short, long)]
	pub gas: u64,
}

#[derive(Args, Debug)]
#[clap(group(
    ArgGroup::new("admin")
    .required(true)
    .args(&["new_admin", "no_admin"]),
))]
pub struct UpdateAdmin {
	/// Contract to be updated
	#[arg(short, long)]
	pub contract: AccountId32,
	/// New admin of the contract
	#[arg(short = 'a', long, conflicts_with = "no_admin")]
	pub new_admin: Option<AccountId32>,
	/// Erase the admin of the contract (migrates are not possible after this point)
	// NOTE: This argument won't be used programmatically. It exists so that
	// users don't accidentally delete the admin because they forget to set
	// `new_admin`. If this flag is on, we ensure `new_admin` is `None` and
	// hence the admin will be deleted.
	#[arg(long, conflicts_with = "new_admin")]
	pub no_admin: bool,

	/// Gas limit
	#[arg(short, long)]
	pub gas: u64,
}

#[derive(Args, Debug)]
pub struct WasmRpcQuery {
	/// Contract to be queried
	#[arg(short, long)]
	pub contract: AccountId32,
	/// Gas limit
	#[arg(short, long)]
	pub gas: u64,
	/// Query request
	#[arg(short, long)]
	pub query: String,
}
