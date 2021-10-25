use codec::{Decode, Encode};
use jsonrpc_core_client::{
	transports::{http, ws},
	RpcError,
};
use polkadot_core_primitives::{AccountId, AccountIndex, BlockNumber, Hash, Header};
use rococo::{Call, SessionKeys, SignedBlock, VERSION};
use sc_rpc::{author::AuthorClient, chain::ChainClient};
use sp_core::{sr25519, Pair};
use sp_rpc::{list::ListOrValue, number::NumberOrHex};
use sp_runtime::{generic, generic::Era, traits::IdentifyAccount, MultiSigner};
use std::str::FromStr;
use structopt::StructOpt;
use substrate_frame_rpc_system::SystemApiClient;

type RococoChain = ChainClient<BlockNumber, Hash, Header, SignedBlock>;
type DaliChain = ChainClient<
	common::BlockNumber,
	common::Hash,
	picasso::Header,
	generic::SignedBlock<picasso::Block>,
>;

/// The `insert` command
#[derive(Debug, StructOpt, Clone)]
pub enum Main {
	RotateKeys {
		/// The secret key URI.
		/// If the value is a file, the file content is used as URI.
		/// If not given, you will be prompted for the URI.
		#[structopt(long)]
		key: String,

		/// port number.
		#[structopt(long)]
		port: Option<String>,
	},
	UpgradeRuntime {
		/// The secret key URI.
		/// If the value is a file, the file content is used as URI.
		/// If not given, you will be prompted for the URI.
		#[structopt(long)]
		key: String,
		/// path to wasm file
		path: String,
	},
}

#[tokio::main]
async fn main() -> Result<(), RpcError> {
	let main = Main::from_args();

	match main {
		Main::RotateKeys { key, port } => rotate_keys(port, key).await?,
		Main::UpgradeRuntime { key, path } => {
			let wasm = std::fs::read(path).unwrap();
			upgrade_runtime(wasm, key).await?
		},
	};

	Ok(())
}

async fn rotate_keys(port: Option<String>, key: String) -> Result<(), RpcError> {
	let signer = sr25519::Pair::from_string(&key, None).unwrap();
	let account_id = MultiSigner::from(signer.public()).into_account();

	let uri = format!("http://localhost:{}", port.unwrap_or("9933".into()));
	let author = http::connect::<AuthorClient<Hash, Hash>>(&uri).await.unwrap();
	// first rotate, our keys.
	let bytes = author.rotate_keys().await.unwrap();
	let keys = SessionKeys::decode(&mut &bytes[..]).unwrap();
	// assert that our keys have been rotated.
	assert!(author.has_session_keys(keys.clone().encode().into()).await.unwrap());

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

	let additional = (
		VERSION.spec_version,
		VERSION.transaction_version,
		genesis_hash.clone(),
		genesis_hash.clone(),
		(),
		(),
		(),
	);

	let payload = rococo::SignedPayload::from_raw(call, extra, additional);
	let signature = payload.using_encoded(|payload| signer.sign(payload));
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

async fn upgrade_runtime(wasm: Vec<u8>, key: String) -> Result<(), RpcError> {
	let signer = sr25519::Pair::from_string(&key, None).unwrap();
	let account_id = MultiSigner::from(signer.public()).into_account();

	let url = url::Url::from_str("wss://dali-chachacha-rpc.composable.finance").unwrap();

	let dali_chain_client = ws::connect::<DaliChain>(&url).await.unwrap();
	let dali_system_client =
		ws::connect::<SystemApiClient<common::Hash, common::AccountId, common::AccountIndex>>(&url)
			.await?;

	// get genesis hash
	let genesis_hash = match dali_chain_client
		.block_hash(Some(ListOrValue::Value(NumberOrHex::Number(0))))
		.await
	{
		Ok(ListOrValue::Value(Some(hash))) => hash,
		_ => unreachable!("genesis hash should exist"),
	};

	let account_index = dali_system_client.nonce(account_id.clone()).await?;

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
		genesis_hash.clone(),
		genesis_hash.clone(),
		(),
		(),
		(),
	);

	let call = scheduler::Call::schedule_after {
		after: 600,
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
	let signature = payload.using_encoded(|payload| signer.sign(payload));
	let (call, extra, _) = payload.deconstruct();
	let extrinsic = picasso::UncheckedExtrinsic::new_signed(
		call,
		account_id.clone().into(),
		signature.into(),
		extra,
	);

	let dali_author = ws::connect::<AuthorClient<common::Hash, common::Hash>>(&url).await?;
	// send off the extrinsic
	dali_author.submit_extrinsic(extrinsic.encode().into()).await?;

	Ok(())
}
