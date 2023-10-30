use grandpa_client_primitives::parachain_header_storage_key;
use grandpa_prover::{GrandpaProver};
use subxt::SubstrateConfig;
use subxt::dynamic::Value;
use subxt::ext::scale_value::Composite;
use std::io::Bytes;
use std::str::FromStr;
use std::time::Duration;
use jsonrpsee::{async_client::Client, tracing::log, ws_client::WsClientBuilder};
use subxt::{config::Header, rpc::types::StorageChangeSet, Config, OnlineClient};
use std::sync::Arc;
use hyperspace_core::substrate::DefaultConfig as PolkadotConfig;
use hyperspace_core::substrate::ComposableConfig;
use hyperspace_core::substrate::composable::relaychain;
use hyperspace_core::substrate::composable::parachain_subxt;
// use subxt_signer::sr25519::dev::{self};
use sp_keyring::AccountKeyring;
use subxt::tx::PairSigner;
use subxt::utils::AccountId32;
use sp_core::Pair;




#[tokio::main]
async fn main() {
    
    let sovereign_account_id = "13YMK2ecbyxtm4cmFs31PqzWmQ7gWVboJSmXbcA56DB94xB9";
    let sovereign_account_id_index_0 = "12x6QU4c9eRPxJMATFsRNFiZTMK5QgZkdZFFeu2QDKn4TR82";

    let para_storage_key = parachain_header_storage_key(2019);
    println!("Hello, world!");

    //0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc456d08aea5b028f73699523ae21709a815640ec97748f5b5da9a2298e830e8971df7908861e1710b957fe06f0703bca7d
    //convert to bytes this 0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc456d08aea5b028f73699523ae21709a815640ec97748f5b5da9a2298e830e8971df7908861e1710b957fe06f0703bca7d
    let hex_string = "5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc456d08aea5b028f73699523ae21709a815640ec97748f5b5da9a2298e830e8971df7908861e1710b957fe06f0703bca7d";

    let bytes = hex::decode(hex_string).expect("Failed to decode hex string");

    let relay = std::env::var("RELAY_HOST").unwrap_or_else(|_| "rpc.polkadot.io".to_string());
	let para = std::env::var("PARA_HOST").unwrap_or_else(|_| "rpc.polkadot.io".to_string());

	let relay_ws_url = format!("wss://{relay}:443");
	let relay_ws_url = format!("ws://127.0.0.1:8001");
	let para_ws_url = format!("ws://127.0.0.1:8000");

    let relay_ws_client = Arc::new(WsClientBuilder::default().build(relay_ws_url).await.unwrap());
    let relay_client = OnlineClient::<PolkadotConfig>::from_rpc_client(relay_ws_client.clone()).await.unwrap();
    let para_ws_client = Arc::new(WsClientBuilder::default().build(para_ws_url).await.unwrap());
    let para_client = OnlineClient::<ComposableConfig>::from_rpc_client(para_ws_client.clone()).await.unwrap();

	let keys = vec![para_storage_key.as_ref()];

	let state_proof: Vec<Vec<u8>> = vec![]; //TODO uncomment when rpc method is available
                        // relay_client
						// .rpc()
						// .read_proof(keys.iter().map(AsRef::as_ref), None)
						// .await.unwrap()
						// .proof
						// .into_iter()
						// .map(|p| p.0)
						// .collect();
	println!("state_proof: {:?}", state_proof);
	// assert!(state_proof.len() > 0);

    let block_hash =
				relay_client.rpc().block_hash(None).await.unwrap().unwrap();
    println!("block_hash: {:?}", block_hash);

    let timestamp_addr = relaychain::api::storage().timestamp().now();
    let unix_timestamp_millis = relay_client
        .storage()
        .at(block_hash)
        .fetch(&timestamp_addr)
        .await.unwrap()
        .expect("Timestamp should exist");
    let timestamp_nanos = Duration::from_millis(unix_timestamp_millis).as_nanos() as u64;
    println!("timestamp_nanos: {:?}", timestamp_nanos);

    println!("timestamp_nanos: {:?}", timestamp_nanos);
    use subxt::utils::AccountId32;
    let account_id = AccountId32::from_str(sovereign_account_id_index_0).unwrap();
    println!("{}", bytes.len()); // Add a newline at the end for readability
    let staking = relaychain::api::storage().staking().ledger(account_id);

    let ledger = relay_client
        .storage()
        .at(block_hash)
        .fetch(&staking)
        .await.unwrap()
        .expect("Ledger should exist");


    let sl = hyperspace_core::substrate::composable::relaychain::api::runtime_types::pallet_staking::StakingLedger::try_from(ledger).unwrap();
    println!("sl: {:?}", sl);

    // type t = hyperspace_core::substrate::composable::relaychain::api::runtime_types::pallet_liquid_staking::types::StakingLedger<::subxt::utils::AccountId32, ::core::primitive::u128>;
    // StakingLedger::<AccountId32, u128>::try_from(ledger).unwrap();
    use crate::parachain_subxt::api::runtime_types::pallet_liquid_staking::types::StakingLedger;
    let xxx = StakingLedger::<AccountId32, u128> {
        stash: AccountId32::from_str(sovereign_account_id_index_0).unwrap(),
        total: sl.total,
        active: sl.active,
        unlocking: vec![],  //TODO
        claimed_rewards: vec![], //TODO
    };
    let tx_set_staking_ledger = parachain_subxt::api::tx().pallet_liquid_staking().set_staking_ledger(0, xxx, state_proof);
    let tx_value = parachain_subxt::api::tx().pallet_liquid_staking().initiate_exchange_rate();

    let api = OnlineClient::<subxt::SubstrateConfig>::from_url("ws://127.0.0.1:8000").await.unwrap();
    let v : Vec::<Value<()>> = vec![];
    // let tx_value = subxt::dynamic::tx("PalletLiquidStaking", "initiate_exchange_rate", v);
    // let signer = PairSigner::new(AccountKeyring::Alice.pair());

    use subxt::ext::sp_core::Pair;
    //test wallet for lsd testing 5DPqUqEfnp3buHaqiVnPt8ryykJEQRgdqAjbscnrZG2qDADa
    let key = sp_keyring::sr25519::sr25519::Pair::from_string(&"private sentence hip meadow place say issue winner express edge royal aerobic", None).expect("secret");
    let signer: PairSigner<SubstrateConfig, sp_keyring::sr25519::sr25519::Pair> = PairSigner::new(key.clone());

    let mut i = 10;
    // while i > 0 {
    //     let signed =
    //         api.tx().sign_and_submit_then_watch(&tx_value, &signer, <_>::default()).await;
    //     println!("signed: {:?}", signed);
    //     i -= 1;
    //     match signed {
    //         Ok(progress) => {
    //             i = 0;
    //         },
    //         Err(e) => {
    //             println!("Error: {:?}", e);
    //             tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    //         },
    //     } 
    // }

    let mut i = 10;
    while i > 0 {
        let signed =
            api.tx().sign_and_submit_then_watch(&tx_set_staking_ledger, &signer, <_>::default()).await;
        println!("signed: {:?}", signed);
        i -= 1;
        match signed {
            Ok(progress) => {
                i = 0;
            },
            Err(e) => {
                println!("Error: {:?}", e);
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            },
        } 
    }
    


    todo!();
}