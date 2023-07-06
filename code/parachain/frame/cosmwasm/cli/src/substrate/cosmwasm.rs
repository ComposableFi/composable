use crate::{args::parse_funds, error::Error};
use anyhow::anyhow;
use clap::{ArgGroup, Args};
use std::{collections::BTreeMap, fs, io::Read, path::PathBuf};
use subxt::utils::AccountId32;

#[derive(Args, Debug)]
pub struct Upload {
	/// Path to local CosmWasm contract binary
	#[arg()]
	pub wasm_file: PathBuf,
}

#[derive(Args, Debug)]
pub struct Execute {
	/// Contract to be executed
	#[arg(short, long)]
	pub contract: AccountId32,
	/// Funds to be moved prior to execution. Format is "ASSET-1:AMOUNT-1,ASSET-2:AMOUNT-2"
	#[arg(short, long, value_parser = parse_funds)]
	pub funds: Option<BTreeMap<u128, u128>>,
	/// Gas limit
	#[arg(short, long)]
	pub gas: u64,
	/// Execute message
	#[arg(short, long)]
	pub message: String,
}

#[derive(Args, Debug)]
pub struct Migrate {
	/// Contract to be migrated
	#[arg(short, long)]
	pub contract: AccountId32,
	/// The new code ID to use
	#[arg(short, long)]
	pub new_code_id: u64,
	/// Gas limit
	#[arg(short, long)]
	pub gas: u64,
	/// Migrate message
	#[arg(short, long)]
	pub message: String,
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
pub struct Query {
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

impl Upload {
	pub async fn fetch_code(&self) -> anyhow::Result<Vec<u8>> {
		let mut file = fs::File::open(&self.wasm_file)?;
		let metadata = fs::metadata(&self.wasm_file)?;
		let mut buffer = vec![0u8; metadata.len() as usize];
		file.read_exact(&mut buffer)?;
		Ok(buffer)
	}
}
