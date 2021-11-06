use codec::{Decode, Encode};
use jsonrpc_core_client::{transports::ws, RpcChannel, RpcError};
use polkadot_core_primitives::{AccountId, AccountIndex, BlockNumber, Hash, Header};
use rococo::{SessionKeys, SignedBlock, VERSION};
use sc_rpc::{author::AuthorClient, chain::ChainClient};
use serde::Deserialize;
use sp_core::{sr25519, Pair};
use sp_rpc::{list::ListOrValue, number::NumberOrHex};
use sp_runtime::{generic::Era, traits::IdentifyAccount, MultiSigner};
use std::str::FromStr;
use structopt::StructOpt;
use substrate_frame_rpc_system::SystemApiClient;
use subxt::{ClientBuilder, PairSigner};

mod dali;
use dali::api;

type RococoChain = ChainClient<BlockNumber, Hash, Header, SignedBlock>;

/// The `insert` command
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
	api: api::RuntimeApi<api::DefaultConfig>,
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
	let main = Main::from_args();
	let state = State::new().await;

	match main {
		Main::RotateKeys => rotate_keys(&state).await?,
		Main::UpgradeRuntime { path } => {
			let wasm = std::fs::read(path).unwrap();
			upgrade_runtime(wasm, &state).await?
		}
	};

	Ok(())
}

async fn rotate_keys(state: &State) -> Result<(), RpcError> {
	let url = url::Url::from_str(&state.env.rpc_node).unwrap();
	let rpc_channel = ws::connect::<RpcChannel>(&url).await.unwrap();
	let dali_author: AuthorClient<common::Hash, common::Hash> = rpc_channel.clone().into();

	// first rotate, our keys.
	let bytes = dali_author.rotate_keys().await.unwrap();
	let keys = SessionKeys::decode(&mut &bytes[..]).unwrap();
	// assert that our keys have been rotated.
	assert!(dali_author.has_session_keys(keys.clone().encode().into()).await.unwrap());

	// now to set our session keys on cha cha cha
	let call = rococo::Call::Session(session::Call::set_keys { keys: keys.clone(), proof: vec![] });

	let url = url::Url::from_str("wss://fullnode-relay.chachacha.centrifuge.io").unwrap();
	let rpc_channel = ws::connect::<RpcChannel>(&url).await.unwrap();
	let chachacha_chain_client: RococoChain = rpc_channel.clone().into();
	let chachacha_system_client: SystemApiClient<Hash, AccountId, AccountIndex> =
		rpc_channel.clone().into();

	// get genesis hash
	let genesis_hash = match chachacha_chain_client
		.block_hash(Some(ListOrValue::Value(NumberOrHex::Number(0))))
		.await
	{
		Ok(ListOrValue::Value(Some(hash))) => hash,
		_ => unreachable!("genesis hash should exist"),
	};

	let account_id = MultiSigner::from(state.signer.public()).into_account();
	let account_index = chachacha_system_client.nonce(account_id.clone()).await.unwrap();

	let extra = (
		system::CheckSpecVersion::<rococo::Runtime>::new(),
		system::CheckTxVersion::<rococo::Runtime>::new(),
		system::CheckGenesis::<rococo::Runtime>::new(),
		system::CheckMortality::<rococo::Runtime>::from(Era::Immortal),
		system::CheckNonce::<rococo::Runtime>::from(account_index),
		system::CheckWeight::<rococo::Runtime>::new(),
		transaction_payment::ChargeTransactionPayment::<rococo::Runtime>::from(0),
	);

	let additional =
		(VERSION.spec_version, VERSION.transaction_version, genesis_hash, genesis_hash, (), (), ());

	let payload = rococo::SignedPayload::from_raw(call, extra, additional);
	let signature = payload.using_encoded(|payload| state.signer.sign(payload));
	let (call, extra, _) = payload.deconstruct();
	let extrinsic = rococo::UncheckedExtrinsic::new_signed(
		call,
		account_id.clone().into(),
		signature.into(),
		extra,
	);

	let chachacha_author: AuthorClient<Hash, Hash> = rpc_channel.clone().into();
	// TODO: use subxt
	chachacha_author.submit_extrinsic(extrinsic.encode().into()).await.unwrap();

	println!("Confirm your keys on PolkadotJs:\n{:#?}", keys);

	Ok(())
}

async fn upgrade_runtime(wasm: Vec<u8>, state: &State) -> Result<(), subxt::Error> {
	let code_hash = sp_io::hashing::blake2_256(&wasm);
	let signer = PairSigner::new(state.signer.clone());
	let result = state
		.api
		.tx()
		.parachain_system()
		.authorize_upgrade(code_hash.into())
		.sign_and_submit_then_watch(&signer)
		.await?;

	if let None = result.find_event::<api::parachain_system::events::UpgradeAuthorized>()? {
		return Err(subxt::Error::Other("Failed to authorize upgrade".into()));
	}

	let result = state
		.api
		.tx()
		.parachain_system()
		.enact_authorized_upgrade(wasm)
		.sign_and_submit_then_watch(&signer)
		.await?;

	if let None = result.find_event::<api::parachain_system::events::ValidationFunctionStored>()? {
		return Err(subxt::Error::Other("Failed to enact upgrade".into()));
	}

	Ok(())
}
