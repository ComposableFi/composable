use grandpa_client_primitives::parachain_header_storage_key;
use grandpa_prover::{GrandpaProver};
use std::time::Duration;
use jsonrpsee::{async_client::Client, tracing::log, ws_client::WsClientBuilder};
use subxt::{config::Header, rpc::types::StorageChangeSet, Config, OnlineClient};
use std::sync::Arc;
use hyperspace_core::substrate::DefaultConfig as PolkadotConfig;


#[tokio::main]
async fn main() {
    let para_storage_key = parachain_header_storage_key(2019);
    println!("Hello, world!");

    let relay = std::env::var("RELAY_HOST").unwrap_or_else(|_| "rpc.polkadot.io".to_string());
	let para = std::env::var("PARA_HOST").unwrap_or_else(|_| "rpc.polkadot.io".to_string());

	let relay_ws_url = format!("wss://{relay}:443");
	let para_ws_url = format!("wss://{para}:443");

    let relay_ws_client = Arc::new(WsClientBuilder::default().build(relay_ws_url).await.unwrap());
    let relay_client = OnlineClient::<PolkadotConfig>::from_rpc_client(relay_ws_client.clone()).await.unwrap();
    let para_ws_client = Arc::new(WsClientBuilder::default().build(para_ws_url).await.unwrap());
    let para_client = OnlineClient::<PolkadotConfig>::from_rpc_client(para_ws_client.clone()).await.unwrap();

    // let prover = GrandpaProver::<PolkadotConfig>::new(
	// 	&relay_ws_url,
	// 	&para_ws_url,
	// 	2000,
	// 	Duration::from_millis(100),
	// )
	// .await
	// .unwrap();
	let keys = vec![para_storage_key.as_ref()];

	let state_proof: Vec<Vec<u8>> = relay_client
						.rpc()
						.read_proof(keys.iter().map(AsRef::as_ref), None)
						.await.unwrap()
						.proof
						.into_iter()
						.map(|p| p.0)
						.collect();
	println!("state_proof: {:?}", state_proof);
	assert!(state_proof.len() > 0);
}