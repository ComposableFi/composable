use jsonrpc_core_client::transports::ws;
use sc_rpc::system::SystemClient;
use serde::Deserialize;
use std::str::FromStr;
use structopt::StructOpt;
use tide::Request;

#[derive(Debug, Deserialize, StructOpt, Clone)]
struct Main {
	#[structopt(long)]
	port: String,
}

#[derive(Clone)]
struct State {
	system: SystemClient<picasso::Block, common::BlockNumber>,
}

#[tokio::main]
async fn main() -> tide::Result<()> {
	env_logger::init();
	let args = Main::from_args();

	let url = url::Url::from_str(&format!("127.0.0.1:{}", args.port)).unwrap();
	let system = ws::connect(&url).await.expect("failed to connect to collator node");

	let mut app = tide::with_state(State { system });
	app.at("/").post(log_handler);
	app.listen(format!("0.0.0.0:{}", args.port)).await?;

	Ok(())
}

async fn log_handler(mut req: Request<State>) -> tide::Result {
	let targets = req.body_string().await?;

	let result = if targets.is_empty() {
		req.state().system.system_add_log_filter(targets).await
	} else {
		req.state().system.system_reset_log_filter().await
	};

	if let Err(e) = result {
		return Ok(format!("Error: {:?}", e).into())
	}

	Ok("".into())
}
