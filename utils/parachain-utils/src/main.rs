use codec::{Decode, Encode};
use jsonrpc_core_client::{transports::ws, RpcChannel, RpcError};
use polkadot_core_primitives::{AccountId, AccountIndex, BlockNumber, Hash, Header};
use rococo::{Call, SessionKeys, SignedBlock, VERSION};
use sc_rpc::{author::AuthorClient, chain::ChainClient};
use serde::Deserialize;
use sp_core::{sr25519, Pair};
use sp_rpc::{list::ListOrValue, number::NumberOrHex};
use sp_runtime::{generic, generic::Era, traits::IdentifyAccount, MultiSigner};
use std::str::FromStr;
use structopt::StructOpt;
use substrate_frame_rpc_system::SystemApiClient;

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
	root_key: String,
	rpc_node: String,
}

struct State {
	dali_chain: ChainClient<
		common::BlockNumber,
		common::Hash,
		picasso::Header,
		generic::SignedBlock<picasso::Block>,
	>,
	dali_system: SystemApiClient<common::Hash, common::AccountId, common::AccountIndex>,
	dali_author: AuthorClient<common::Hash, common::Hash>,
	signer: sr25519::Pair,
}

impl State {
	async fn new() -> Self {
		let env = envy::from_env::<Env>().unwrap();

		let url = url::Url::from_str(&env.rpc_node).unwrap();

		// create the signer
		let signer = sr25519::Pair::from_string(&env.root_key, None).unwrap();
		// connection to make rpc requests over
		let channel = ws::connect::<RpcChannel>(&url).await.unwrap();
		let (dali_chain, dali_system, dali_author) =
			(channel.clone().into(), channel.clone().into(), channel.into());
		State { dali_author, dali_chain, dali_system, signer }
	}
}

#[tokio::main]
async fn main() -> Result<(), RpcError> {
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

async fn rotate_keys(state: &State) -> Result<(), RpcError> {
	let account_id = MultiSigner::from(state.signer.public()).into_account();

	// first rotate, our keys.
	let bytes = state.dali_author.rotate_keys().await.unwrap();
	let keys = SessionKeys::decode(&mut &bytes[..]).unwrap();
	// assert that our keys have been rotated.
	assert!(state.dali_author.has_session_keys(keys.clone().encode().into()).await.unwrap());

	// now to set our session keys on cha cha cha
	let call = Call::Session(session::Call::set_keys { keys: keys.clone(), proof: vec![] });

	let url = url::Url::from_str("wss://fullnode-relay.chachacha.centrifuge.io").unwrap();
	let chachacha_chain_client = ws::connect::<RococoChain>(&url).await.unwrap();
	let chachacha_system_client =
		ws::connect::<SystemApiClient<Hash, AccountId, AccountIndex>>(&url)
			.await
			.unwrap();

	// get genesis hash
	let genesis_hash = match chachacha_chain_client
		.block_hash(Some(ListOrValue::Value(NumberOrHex::Number(0))))
		.await
	{
		Ok(ListOrValue::Value(Some(hash))) => hash,
		_ => unreachable!("genesis hash should exist"),
	};

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

	let chachacha_author = ws::connect::<AuthorClient<Hash, Hash>>(&url).await.unwrap();
	// send off the extrinsic
	chachacha_author.submit_extrinsic(extrinsic.encode().into()).await.unwrap();

	println!("Confirm your keys on PolkadotJs:\n{:#?}", keys);

	Ok(())
}

async fn upgrade_runtime(wasm: Vec<u8>, state: &State) -> Result<(), RpcError> {
	let account_id = MultiSigner::from(state.signer.public()).into_account();

	// get genesis hash
	let genesis_hash = match state
		.dali_chain
		.block_hash(Some(ListOrValue::Value(NumberOrHex::Number(0))))
		.await
	{
		Ok(ListOrValue::Value(Some(hash))) => hash,
		_ => unreachable!("genesis hash should exist"),
	};

	let account_index = state.dali_system.nonce(account_id.clone()).await?;

	let extra = (
		system::CheckSpecVersion::<picasso::Runtime>::new(),
		system::CheckTxVersion::<picasso::Runtime>::new(),
		system::CheckGenesis::<picasso::Runtime>::new(),
		system::CheckMortality::<picasso::Runtime>::from(Era::Immortal),
		system::CheckNonce::<picasso::Runtime>::from(account_index),
		system::CheckWeight::<picasso::Runtime>::new(),
		transaction_payment::ChargeTransactionPayment::<picasso::Runtime>::from(0),
	);

	let additional = (
		picasso::VERSION.spec_version,
		picasso::VERSION.transaction_version,
		genesis_hash,
		genesis_hash,
		(),
		(),
		(),
	);

	let call = scheduler::Call::schedule_after {
		after: 5,
		maybe_periodic: None,
		priority: 0,
		call: Box::new(
			sudo::Call::sudo_unchecked_weight {
				call: Box::new(system::Call::set_code { code: wasm }.into()),
				weight: 0,
			}
			.into(),
		),
	};

	let payload = picasso::SignedPayload::from_raw(call.into(), extra, additional);
	let signature = payload.using_encoded(|payload| state.signer.sign(payload));
	let (call, extra, _) = payload.deconstruct();
	let extrinsic = picasso::UncheckedExtrinsic::new_signed(
		call,
		account_id.clone().into(),
		signature.into(),
		extra,
	);

	// send off the extrinsic
	state.dali_author.submit_extrinsic(extrinsic.encode().into()).await?;

	Ok(())
}
