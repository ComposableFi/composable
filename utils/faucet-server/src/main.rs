use codec::Encode;
use hmac::{Hmac, Mac, NewMac};
use jsonrpc_core_client::{transports::ws, RpcChannel, RpcError};
use sc_rpc::{author::AuthorClient, chain::ChainClient};
use sha2::Sha256;
use sp_core::{crypto::Ss58Codec, sr25519, Pair};
use sp_rpc::{list::ListOrValue, number::NumberOrHex};
use sp_runtime::{generic, generic::Era, traits::IdentifyAccount, MultiSigner};
use std::{str::FromStr, sync::Arc};
use structopt::StructOpt;
use substrate_frame_rpc_system::SystemApiClient;
use tide::{prelude::*, Error, Request};

#[derive(Debug, Deserialize, StructOpt, Clone)]
struct Main {
	#[structopt(long)]
	port: String,
}

#[derive(Debug, Deserialize, Clone)]
struct SlackWebhook {
	// token: String,
	// team_id: String,
	// team_domain: String,
	// enterprise_id: String,
	// enterprise_name: String,
	// channel_id: String,
	// channel_name: String,
	// command: String,
	// response_url: String,
	// trigger_id: String,
	// api_app_id: String,
	user_id: String,
	user_name: String,
	text: String,
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
	env: Env,
}

#[derive(Deserialize, Debug)]
struct Env {
	slack_signing_key: String,
	root_key: String,
	rpc_node: String,
}

#[tokio::main]
async fn main() -> tide::Result<()> {
	env_logger::init();
	dotenv::dotenv().expect("couldn't load env vars");
	let args = Main::from_args();

	let state = init().await;
	let mut app = tide::with_state(state);
	app.at("/").post(faucet_handler);
	app.listen(format!("0.0.0.0:{}", args.port)).await?;

	Ok(())
}

async fn init() -> Arc<State> {
	let env = envy::from_env::<Env>().expect("Missing env vars");
	let url = url::Url::from_str(&env.rpc_node).unwrap();

	// create the signer
	let signer = sr25519::Pair::from_string(&env.root_key, None).unwrap();
	// connection to make rpc requests over
	let channel = ws::connect::<RpcChannel>(&url).await.unwrap();
	let (dali_chain, dali_system, dali_author) =
		(channel.clone().into(), channel.clone().into(), channel.into());
	Arc::new(State { dali_author, dali_chain, dali_system, signer, env })
}

async fn faucet_handler(mut req: Request<Arc<State>>) -> tide::Result {
	type HmacSha256 = Hmac<Sha256>;

	// Verify signature from slack
	let body_string = req.body_string().await?;
	let timestamp = req
		.header("X-Slack-Request-Timestamp")
		.ok_or_else(|| {
			Error::from_str(400, "No `X-Slack-Request-Timestamp` in headers".to_string())
		})?
		.as_str();
	// strip out "v0="
	let signature = &req
		.header("X-Slack-Signature")
		.ok_or_else(|| Error::from_str(400, "No `X-Slack-Signature` in headers".to_string()))?
		.as_str()[3..];

	// Signing key from slack.
	let mut mac = HmacSha256::new_from_slice(req.state().env.slack_signing_key.as_bytes())
		.expect("HMAC can take key of any size");
	let preimage = format!("v0:{}:{}", timestamp, body_string);
	mac.update(preimage.as_bytes());
	mac.verify(&hex::decode(signature)?)
		.map_err(|_| Error::from_str(400, "Invalid Signature".to_string()))?;
	// message has been verified.

	let SlackWebhook { user_id, text, user_name, .. } = serde_urlencoded::from_str(&body_string)?;

	let address = match common::AccountId::from_string(&text) {
		Ok(a) => a,
		Err(e) => return Ok(format!("Error: {:?}", e).into()),
	};

	match enrich(address.into(), req.state()).await {
		Err(e) => return Ok(format!("Error: {:?}", e).into()),
		Ok(()) => {},
	};

	log::info!("Sent {} 1k Dali", user_name);

	Ok(format!("Sent <@{}> 1,000 Dalis", user_id).into())
}

async fn enrich(address: picasso::Address, state: &State) -> Result<(), RpcError> {
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

	let call = picasso::Call::Balances(balances::Call::transfer {
		dest: address,
		// 1k dali
		value: 1_000_000_000_000_000,
	});

	let payload = picasso::SignedPayload::from_raw(call, extra, additional);
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
