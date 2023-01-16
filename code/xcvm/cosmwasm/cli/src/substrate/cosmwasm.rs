use crate::error::Error;
use anyhow::anyhow;
use clap::{ArgGroup, Args};
use cosmwasm_orchestrate::fetcher::{CosmosApi, CosmosFetcher, FileFetcher};
use std::{collections::BTreeMap, fs, io::Read, path::PathBuf};
use subxt::ext::sp_core::crypto::AccountId32;

#[derive(Args, Debug)]
#[clap(group(
        ArgGroup::new("wasm_source")
        .required(true)
        .args(&["file_path", "url", "cosmos_rpc"]),
    ))]
pub struct Upload {
	/// Path to local CosmWasm contract binary
	#[arg(short = 'f', long, conflicts_with_all = &["cosmos_rpc", "url"])]
	pub file_path: Option<PathBuf>,
	/// Url to fetch the contract from. The contract binary will be fetched by
	/// sending a GET request to this `url`.
	#[arg(short = 'u', long, conflicts_with_all = &["file_path", "cosmos_rpc"])]
	pub url: Option<String>,
	/// Rpc endpoint of a running Cosmos chain.
	#[arg(long, requires = "cosmos-fetch", conflicts_with_all = &["file_path", "url"])]
	pub cosmos_rpc: Option<String>,
	/// Contract address of the contract binary that will be fetched and uploaded
	#[arg(long, group = "cosmos-fetch")]
	pub contract: Option<String>,
	/// Code ID of the contract that will be fetched and uploaded
	#[arg(long, group = "cosmos-fetch")]
	pub code_id: Option<u64>,
}

#[derive(Args, Clone, Debug)]
pub struct Instantiate {
	/// Code ID of the code that will be used to instantiate the contract
	#[arg(short, long)]
	pub code_id: u64,
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
	#[arg(short, long, value_parser = parse_funds)]
	pub funds: Option<BTreeMap<u128, u128>>,
	/// Gas limit
	#[arg(short, long)]
	pub gas: u64,
	/// Instantiate message
	#[arg(short, long)]
	pub message: String,
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
		let code = if let Some(file_path) = self.file_path.as_ref() {
			let mut f = fs::File::open(file_path)?;
			let metadata = fs::metadata(file_path)?;
			let mut buffer = vec![0u8; metadata.len() as usize];
			f.read_exact(&mut buffer)?;
			buffer
		} else if let Some(url) = self.url.as_ref() {
			FileFetcher::from_url(url).await.map_err(|e| anyhow!("{}", e))?
		} else if let Some(cosmos_url) = self.cosmos_rpc.as_ref() {
			if let Some(contract) = self.contract.as_ref() {
				CosmosFetcher::from_contract_addr(cosmos_url, contract)
					.await
					.map_err(|e| anyhow!("{}", e))?
			} else if let Some(code_id) = self.code_id.as_ref() {
				CosmosFetcher::from_code_id(cosmos_url, *code_id)
					.await
					.map_err(|e| anyhow!("{}", e))?
			} else {
				panic!("impossible")
			}
		} else {
			panic!("impossible")
		};
		Ok(code)
	}
}

pub fn parse_funds(funds_str: &str) -> Result<Option<BTreeMap<u128, u128>>, String> {
	let mut funds = BTreeMap::new();
	for asset in funds_str.split(',') {
		let asset: Vec<&str> = asset.split(':').collect();
		if asset.len() != 2 {
			return Err(Error::InvalidFundsFormat.to_string())
		}
		funds.insert(
			asset[0].parse().map_err(|_| Error::InvalidFundsFormat.to_string())?,
			asset[1].parse().map_err(|_| Error::InvalidFundsFormat.to_string())?,
		);
	}
	Ok(Some(funds))
}
