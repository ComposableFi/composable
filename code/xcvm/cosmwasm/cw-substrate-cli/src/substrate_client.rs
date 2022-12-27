use crate::{
	error::Error,
	types::api::{
		self,
		cosmwasm::events::{AdminUpdated, Emitted, Executed, Instantiated, Migrated, Uploaded},
		runtime_types::{
			pallet_cosmwasm::pallet::CodeIdentifier,
			primitives::currency::CurrencyId,
			sp_runtime::bounded::{bounded_btree_map::BoundedBTreeMap, bounded_vec::BoundedVec},
		},
	},
};
use clap::{ArgGroup, Args, Subcommand};
use cosmwasm_orchestrate::fetcher::{CosmosApi, CosmosFetcher, FileFetcher};
use sp_keyring::sr25519::Keyring;
use std::{collections::BTreeMap, fmt::Display, fs, io::Read, path::PathBuf, str::FromStr};
use subxt::{
	blocks::ExtrinsicEvents,
	ext::{
		codec::Encode,
		sp_core::{crypto::AccountId32, ed25519, sr25519, Pair},
		sp_runtime::{MultiSignature, MultiSigner},
	},
	tx::PairSigner,
	OnlineClient, SubstrateConfig,
};

/// Interact with a substrate-based chain.
#[derive(Args, Debug)]
pub struct SubstrateCommand {
	/// Name of the development account that will be used as signer. (eg. alice)
	// NOTE(aeryz): This conflicts with `scheme` because it can only be `sr25519`.
	#[arg(short, long, value_parser = parse_keyring, conflicts_with_all = &["seed", "mnemonic", "scheme"])]
	name: Option<Keyring>,

	/// Secret seed of the signer
	#[arg(short, long, conflicts_with_all = &["name", "mnemonic"])]
	seed: Option<Vec<u8>>,

	/// Mnemonic of the signer
	#[arg(short, long, conflicts_with_all = &["name", "seed"])]
	mnemonic: Option<String>,

	/// Signature scheme. (eg. sr25519, ed25519)
	#[arg(long, default_value_t = KeyScheme::Sr25519)]
	scheme: KeyScheme,

	/// Password for the mnemonic
	#[arg(short, long)]
	password: Option<String>,

	/// Websocket endpoint of the substrate chain
	#[arg(short = 'c', long, default_value_t = String::from("ws://127.0.0.1:9944"))]
	chain_endpoint: String,

	#[command(subcommand)]
	command: CosmwasmCommand,
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

#[derive(Debug, Subcommand)]
pub enum CosmwasmCommand {
	/// Upload a CosmWasm contract
	#[clap(group(
        ArgGroup::new("wasm_source")
        .required(true)
        .args(&["file_path", "url", "cosmos_rpc"]),
    ))]
	Upload {
		/// Path to local CosmWasm contract binary
		#[arg(short = 'f', long, conflicts_with_all = &["cosmos_rpc", "url"])]
		file_path: Option<PathBuf>,
		/// Url to fetch the contract from. The contract binary will be fetched by
		/// sending a GET request to this `url`.
		#[arg(short = 'u', long, conflicts_with_all = &["file_path", "cosmos_rpc"])]
		url: Option<String>,
		/// Rpc endpoint of a running Cosmos chain.
		#[arg(long, requires = "cosmos-fetch", conflicts_with_all = &["file_path", "url"])]
		cosmos_rpc: Option<String>,
		/// Contract address of the contract binary that will be fetched and uploaded
		#[arg(long, group = "cosmos-fetch")]
		contract: Option<String>,
		/// Code ID of the contract that will be fetched and uploaded
		#[arg(long, group = "cosmos-fetch")]
		code_id: Option<u64>,
	},

	/// Instantiate a CosmWasm contract
	Instantiate {
		/// Code ID of the code that will be used to instantiate the contract
		#[arg(short, long)]
		code_id: u64,
		/// Additional data to be used in contract address derivation in case you want to
		/// instantiate the same contract with the same message and label multiple times
		#[arg(short, long)]
		salt: String,
		/// Contract's admin
		#[arg(short, long)]
		admin: Option<AccountId32>,
		/// Human-readable label of the contract
		#[arg(short, long)]
		label: String,
		/// Funds to be moved prior to execution. Format is "ASSET-1:AMOUNT-1,ASSET-2:AMOUNT-2"
		#[arg(short, long, value_parser = parse_funds)]
		funds: Option<BTreeMap<u128, u128>>,
		/// Gas limit
		#[arg(short, long)]
		gas: u64,
		/// Instantiate message
		#[arg(short, long)]
		message: String,
	},

	/// Execute a CosmWasm contract
	Execute {
		/// Contract to be executed
		#[arg(short, long)]
		contract: AccountId32,
		/// Funds to be moved prior to execution. Format is "ASSET-1:AMOUNT-1,ASSET-2:AMOUNT-2"
		#[arg(short, long, value_parser = parse_funds)]
		funds: Option<BTreeMap<u128, u128>>,
		#[arg(short, long)]
		/// Gas limit
		gas: u64,
		/// Execute message
		#[arg(short, long)]
		message: String,
	},

	/// Migrate a CosmWasm contract
	Migrate {
		/// Contract to be migrated
		#[arg(short, long)]
		contract: AccountId32,
		/// The new code ID to use
		#[arg(short, long)]
		new_code_id: u64,
		/// Gas limit
		#[arg(short, long)]
		gas: u64,
		/// Migrate message
		#[arg(short, long)]
		message: String,
	},

	/// Update admin of a CosmWasm contract
	#[clap(group(
        ArgGroup::new("admin")
        .required(true)
        .args(&["new_admin", "no_admin"]),
    ))]
	UpdateAdmin {
		/// Contract to be updated
		#[arg(short, long)]
		contract: AccountId32,
		/// New admin of the contract
		#[arg(short = 'a', long, conflicts_with = "no_admin")]
		new_admin: Option<AccountId32>,
		/// Erase the admin of the contract (migrates are not possible after this point)
		#[arg(long, conflicts_with = "new_admin")]
		// NOTE: This argument won't be used programmatically. It exists so that
		// users don't accidentally delete the admin because they forget to set
		// `new_admin`. If this flag is on, we ensure `new_admin` is `None` and
		// hence the admin will be deleted.
		no_admin: bool,
		/// Gas limit
		#[arg(short, long)]
		gas: u64,
	},
}

impl SubstrateCommand {
	pub async fn run(self) -> Result<(), Error> {
		match self.scheme {
			KeyScheme::Sr25519 => self.dispatch_command::<sr25519::Pair>().await,
			KeyScheme::Ed25519 => self.dispatch_command::<ed25519::Pair>().await,
		}
	}

	async fn dispatch_command<P: Pair>(self) -> Result<(), Error>
	where
		P::Seed: TryFrom<Vec<u8>>,
		MultiSignature: From<<P as Pair>::Signature>,
		MultiSigner: From<<P as Pair>::Public>,
	{
		let pair = self.get_signer_pair::<P>()?;

		match self.command {
			CosmwasmCommand::Upload { file_path, url, cosmos_rpc: chain, contract, code_id } => {
				let Some(pair) = pair else {
                    return Err(Error::OperationNeedsToBeSigned);
                };

				let contract_code = match (file_path, url, chain) {
					(Some(file_path), _, _) => {
						let mut f = fs::File::open(&file_path).expect("Could not read file");
						let metadata = fs::metadata(&file_path).expect("Could not read metadata");
						let mut buffer = vec![0; metadata.len() as usize];
						f.read_exact(&mut buffer).expect("Buffer overflow during file read");
						buffer
					},
					(_, Some(url), _) => FileFetcher::from_url(url).await.unwrap(),
					(_, _, Some(cosmos_url)) =>
						if let Some(contract) = contract {
							CosmosFetcher::from_contract_addr(&cosmos_url, &contract).await.unwrap()
						} else if let Some(code_id) = code_id {
							CosmosFetcher::from_code_id(&cosmos_url, code_id).await.unwrap()
						} else {
							panic!("impossible");
						},
					_ => todo!(),
				};

				let events = do_signed_transaction(
					self.chain_endpoint,
					pair,
					api::tx().cosmwasm().upload(BoundedVec(contract_code)),
				)
				.await?;
				let uploaded = find_and_cast_events::<Uploaded>(&events, true)?;

				println!("[ + ] Contract uploaded.");
				println!("\t- Code Hash: {}", uploaded[0].code_hash);
				println!("\t- Code ID: {}", uploaded[0].code_id);

				print_cosmwasm_events(&events)?;

				Ok(())
			},
			CosmwasmCommand::Instantiate { code_id, salt, admin, label, funds, gas, message } => {
				let Some(pair) = pair else {
                    return Err(Error::OperationNeedsToBeSigned);
                };

				let events = do_signed_transaction(
					self.chain_endpoint,
					pair,
					api::tx().cosmwasm().instantiate(
						CodeIdentifier::CodeId(code_id),
						BoundedVec(salt.into()),
						admin,
						BoundedVec(label.into()),
						BoundedBTreeMap(
							funds
								.unwrap_or_default()
								.into_iter()
								.map(|(asset, amount)| (CurrencyId(asset), (amount, true)))
								.collect(),
						),
						gas,
						BoundedVec(message.into()),
					),
				)
				.await?;

				let event = find_and_cast_events::<Instantiated>(&events, true)?;

				println!("[ + ] Contract instantiated.");
				println!("\t- Contract address: {}", event[0].contract);
				print_cosmwasm_events(&events)?;

				Ok(())
			},
			CosmwasmCommand::Execute { contract, funds, gas, message } => {
				let Some(pair) = pair else {
                    return Err(Error::OperationNeedsToBeSigned);
                };

				let events = do_signed_transaction(
					self.chain_endpoint,
					pair,
					api::tx().cosmwasm().execute(
						contract,
						BoundedBTreeMap(
							funds
								.unwrap_or_default()
								.into_iter()
								.map(|(asset, amount)| (CurrencyId(asset), (amount, true)))
								.collect(),
						),
						gas,
						BoundedVec(message.into()),
					),
				)
				.await?;

				let _ = find_and_cast_events::<Executed>(&events, true)?;

				println!("[ + ] Contract executed.");
				print_cosmwasm_events(&events)?;

				Ok(())
			},
			CosmwasmCommand::Migrate { contract, new_code_id, gas, message } => {
				let Some(pair) = pair else {
					return Err(Error::OperationNeedsToBeSigned);
				};

				let events = do_signed_transaction(
					self.chain_endpoint,
					pair,
					api::tx().cosmwasm().migrate(
						contract,
						CodeIdentifier::CodeId(new_code_id),
						gas,
						BoundedVec(message.into()),
					),
				)
				.await?;
				let _ = find_and_cast_events::<Migrated>(&events, true)?;
				println!("[ + ] Contract migrated.");
				print_cosmwasm_events(&events)?;

				Ok(())
			},
			CosmwasmCommand::UpdateAdmin { contract, new_admin, gas, .. } => {
				let Some(pair) = pair else {
					return Err(Error::OperationNeedsToBeSigned);
				};

				let events = do_signed_transaction(
					self.chain_endpoint,
					pair,
					api::tx().cosmwasm().update_admin(contract, new_admin, gas),
				)
				.await?;
				let _ = find_and_cast_events::<AdminUpdated>(&events, true)?;
				println!("[ + ] Contract's admin is updated.");

				Ok(())
			},
		}
	}

	fn get_signer_pair<P: Pair>(&self) -> Result<Option<P>, Error>
	where
		P::Seed: TryFrom<Vec<u8>>,
	{
		let pair = if let Some(name) = self.name.as_ref() {
			P::from_string(&format!("//{}", name), None).map_err(|_| Error::InvalidSeed)?
		} else if let Some(mnemonic) = self.mnemonic.as_ref() {
			let (pair, _) = P::from_phrase(mnemonic, self.password.as_deref())
				.map_err(|_| Error::InvalidPhrase)?;
			pair
		} else if let Some(seed) = self.seed.as_ref() {
			let seed: P::Seed = seed.clone().try_into().map_err(|_| Error::InvalidSeed)?;
			P::from_seed(&seed)
		} else {
			return Ok(None)
		};

		Ok(Some(pair))
	}
}

async fn do_signed_transaction<CallData: Encode, P: Pair>(
	endpoint: String,
	signer: P,
	tx: subxt::tx::StaticTxPayload<CallData>,
) -> Result<ExtrinsicEvents<SubstrateConfig>, Error>
where
	MultiSignature: From<<P as Pair>::Signature>,
	MultiSigner: From<<P as Pair>::Public>,
{
	let signer = PairSigner::new(signer);
	let api = OnlineClient::<SubstrateConfig>::from_url(endpoint)
		.await
		.map_err(Error::Substrate)?;
	let events = api
		.tx()
		.sign_and_submit_then_watch_default(&tx, &signer)
		.await
		.map_err(Error::Substrate)?
		.wait_for_in_block()
		.await
		.map_err(Error::Substrate)?
		.wait_for_success()
		.await
		.map_err(Error::Substrate)?;
	Ok(events)
}

fn find_and_cast_events<E: subxt::events::StaticEvent>(
	events: &ExtrinsicEvents<SubstrateConfig>,
	is_mandatory: bool,
) -> Result<Vec<E>, Error> {
	let mut desired_events = Vec::new();
	for event in events.iter() {
		let event = event.map_err(Error::Substrate)?;
		if let Some(event) = event.as_event::<E>().map_err(|_| Error::Codec)? {
			desired_events.push(event)
		}
	}

	if is_mandatory && desired_events.is_empty() {
		Err(Error::ExpectedEventNotEmitted)
	} else {
		Ok(desired_events)
	}
}

fn print_cosmwasm_events(events: &ExtrinsicEvents<SubstrateConfig>) -> Result<(), Error> {
	let events = find_and_cast_events::<Emitted>(events, false)?;
	for event in events {
		println!("- Event: {}", String::from_utf8_lossy(&event.ty));
		println!("\t- Contract: {}", event.contract);
		println!("\t- Attributes:");
		for (key, value) in event.attributes {
			println!(
				"\t\t- {}: {}",
				String::from_utf8_lossy(&key),
				String::from_utf8_lossy(&value)
			);
		}
	}
	Ok(())
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

pub fn parse_keyring(s: &str) -> Result<Keyring, String> {
	Keyring::from_str(s).map_err(|_| Error::InvalidAddress.to_string())
}
