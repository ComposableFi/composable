use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use sp_core::{crypto::Ss58Codec, sr25519, Pair};
use std::sync::Arc;
use structopt::StructOpt;
use subxt::{ClientBuilder, PairSigner};
use subxt_clients::picasso::api;
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
	api: api::RuntimeApi<api::DefaultConfig>,
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

	// create the signer
	let signer = sr25519::Pair::from_string(&env.root_key, None).unwrap();

	let api = ClientBuilder::new()
		.set_url(&env.rpc_node)
		.build()
		.await
		.unwrap()
		.to_runtime_api();

	Arc::new(State { api, signer, env })
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

async fn enrich(address: common::Address, state: &State) -> Result<(), subxt::Error> {
	let signer = PairSigner::new(state.signer.clone());
	let result = state
		.api
		.tx()
		.balances()
		// 1k Dali
		.transfer(address, 1_000_000_000_000_000)
		.sign_and_submit_then_watch(&signer)
		.await?;

	if result.find_event::<api::balances::events::Transfer>()?.is_none() {
		return Err(subxt::Error::Other("Transfer failed".into()))
	}

	Ok(())
}
