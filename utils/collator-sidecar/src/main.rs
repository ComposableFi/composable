use jsonrpsee::{
	core::client::ClientT,
	rpc_params,
	ws_client::{WsClient, WsClientBuilder},
};
use serde::Deserialize;
use std::sync::Arc;
use structopt::StructOpt;
use tide::{log, Request};

#[derive(Debug, Deserialize, StructOpt, Clone)]
struct Main {
	#[structopt(long)]
	ws_port: String,
	#[structopt(long)]
	port: String,
}

struct State {
	client: WsClient,
}

#[tokio::main]
async fn main() -> tide::Result<()> {
	env_logger::init();
	let args = Main::from_args();

	let url = format!("ws://127.0.0.1:{}", args.ws_port);
	let client = WsClientBuilder::default()
		.max_request_body_size(u32::MAX)
		.build(url)
		.await
		.unwrap();

	let mut app = tide::with_state(Arc::new(State { client }));
	app.at("/").post(log_handler);
	app.listen(format!("0.0.0.0:{}", args.port)).await?;

	Ok(())
}

async fn log_handler(mut req: Request<Arc<State>>) -> tide::Result {
	let targets = req.body_string().await?;
	log::info!("got new targets: {}", targets);

	let result = if !targets.is_empty() {
		req.state()
			.client
			.request::<()>("system_addLogFilter", rpc_params!(targets))
			.await
	} else {
		req.state().client.request::<()>("system_resetLogFilter", None).await
	};
	log::info!("result: {:?}", result);

	if let Err(e) = result {
		return Ok(format!("Error: {:?}", e).into())
	}

	Ok("".into())
}
