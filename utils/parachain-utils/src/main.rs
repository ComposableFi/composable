use codec::{Decode, Encode};
use jsonrpc_core_client::{transports::ws, RpcChannel, RpcError};
use sc_rpc::author::AuthorClient;
use serde::Deserialize;
use sp_core::{sr25519, Pair, H256};
use sp_runtime::{traits::IdentifyAccount, MultiSigner};
use std::str::FromStr;
use structopt::StructOpt;
use subxt::{ClientBuilder, PairSigner};
use subxt_clients::{chachacha, picasso};

/// The command options
#[derive(Debug, StructOpt, Clone)]
pub enum Main {
	RotateKeys,
	UpgradeRuntime {
		/// path to wasm file
		#[structopt(long)]
		path: String,
	},
}

#[derive(Deserialize, Debug)]
struct Env {
	/// Root key used to sign transactions
	root_key: String,
	/// Url to dali rpc node
	rpc_node: String,
}

struct State {
	/// Subxt api
	api: picasso::api::RuntimeApi<picasso::api::DefaultConfig>,
	/// Pair signer
	signer: sr25519::Pair,
	/// Env variables
	env: Env,
}

impl State {
	async fn new() -> Self {
		let env = envy::from_env::<Env>().unwrap();
		// create the signer
		let signer = sr25519::Pair::from_string(&env.root_key, None).unwrap();

		let api = ClientBuilder::new()
			.set_url(&env.rpc_node)
			.build()
			.await
			.unwrap()
			.to_runtime_api();

		State { api, signer, env }
	}
}

#[derive(derive_more::From, Debug)]
enum Error {
	Subxt(subxt::Error),
	Rpc(RpcError),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
	env_logger::init();

	let main = Main::from_args();
	let state = State::new().await;

	match main {
		Main::RotateKeys => rotate_keys(&state).await?,
		Main::UpgradeRuntime { path } => {
			let wasm = std::fs::read(path).unwrap();
			upgrade_runtime(wasm, &state).await?
		},
	};

	Ok(())
}

async fn rotate_keys(state: &State) -> Result<(), Error> {
	let url = url::Url::from_str(&state.env.rpc_node).unwrap();
	let rpc_channel = ws::connect::<RpcChannel>(&url).await?;
	let dali_author: AuthorClient<common::Hash, common::Hash> = rpc_channel.clone().into();

	// first rotate, our keys.
	let bytes = dali_author.rotate_keys().await?.to_vec();
	use chachacha::api::runtime_types::rococo_runtime::SessionKeys;
	// assert that our keys have been rotated.
	assert!(dali_author.has_session_keys(bytes.clone().into()).await?);

	// now to set our session keys on cha cha cha
	let api = ClientBuilder::new()
		.set_url("wss://fullnode-relay.chachacha.centrifuge.io")
		.build()
		.await?
		.to_runtime_api::<chachacha::api::RuntimeApi<chachacha::api::DefaultConfig>>();

	let signer = PairSigner::new(state.signer.clone());
	let account = MultiSigner::from(state.signer.public()).into_account();

	let _ = api
		.tx()
		.session()
		.set_keys(SessionKeys::decode(&mut &bytes[..]).unwrap(), vec![])
		.sign_and_submit_then_watch(&signer)
		.await?;

	// check storage for the new keys
	let key_bytes = api
		.storage()
		.session()
		.next_keys(account, None)
		.await?
		.ok_or_else(|| subxt::Error::Other("Failed to set keys!".into()))?
		.encode();

	// should match
	assert_eq!(bytes, key_bytes);

	Ok(())
}

async fn upgrade_runtime(code: Vec<u8>, state: &State) -> Result<(), subxt::Error> {
	use crate::picasso::api::runtime_types::{
		cumulus_pallet_parachain_system::pallet::Call as ParachainSystemCall, picasso_runtime::Call,
	};
	let code_hash: H256 = sp_io::hashing::blake2_256(&code).into();
	let signer = PairSigner::new(state.signer.clone());
	let call = Call::ParachainSystem(ParachainSystemCall::authorize_upgrade { code_hash });
	let result = state
		.api
		.tx()
		.sudo()
		.sudo_unchecked_weight(call, 0)
		.sign_and_submit_then_watch(&signer)
		.await?;

	if result
		.find_event::<picasso::api::parachain_system::events::UpgradeAuthorized>()?
		.is_none()
	{
		return Err(subxt::Error::Other("Failed to authorize upgrade".into()))
	}

	let call = Call::ParachainSystem(ParachainSystemCall::enact_authorized_upgrade { code });
	let result = state
		.api
		.tx()
		.sudo()
		.sudo_unchecked_weight(call, 0)
		.sign_and_submit_then_watch(&signer)
		.await?;

	if result
		.find_event::<picasso::api::parachain_system::events::ValidationFunctionStored>()?
		.is_none()
	{
		return Err(subxt::Error::Other("Failed to enact upgrade".into()))
	}

	log::info!("Runtime upgrade proposed, extrinsic hash: {}", result.extrinsic);

	Ok(())
}
