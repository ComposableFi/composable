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

    //0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc456d08aea5b028f73699523ae21709a815640ec97748f5b5da9a2298e830e8971df7908861e1710b957fe06f0703bca7d
    //convert to bytes this 0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc456d08aea5b028f73699523ae21709a815640ec97748f5b5da9a2298e830e8971df7908861e1710b957fe06f0703bca7d
    let hex_string = "5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc456d08aea5b028f73699523ae21709a815640ec97748f5b5da9a2298e830e8971df7908861e1710b957fe06f0703bca7d";

    let bytes = hex::decode(hex_string).expect("Failed to decode hex string");

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