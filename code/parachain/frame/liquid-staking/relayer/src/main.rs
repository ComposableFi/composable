use grandpa_client_primitives::parachain_header_storage_key;
use grandpa_prover::{GrandpaProver};
use hyperspace_core::substrate::composable::parachain_subxt::api::pallet_liquid_staking::calls::types::SetStakingLedger;
use sp_core::storage::StorageKey;
use subxt::SubstrateConfig;
use subxt::dynamic::Value;
use subxt::ext::scale_value::Composite;
use subxt::tx::Payload;
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
use crate::parachain_subxt::api::runtime_types::pallet_liquid_staking::types::{StakingLedger, UnlockChunk};
use futures_util::stream::StreamExt;




#[tokio::main]
async fn main() {
    
    let sovereign_account_id = "13YMK2ecbyxtm4cmFs31PqzWmQ7gWVboJSmXbcA56DB94xB9";
    let sovereign_account_id_index_0 = "12x6QU4c9eRPxJMATFsRNFiZTMK5QgZkdZFFeu2QDKn4TR82";
    let sovereign_account_id_index_1 = "1461Z7Bm1bwQpz1PuYMQ8phj9bRpxNU7ZYsb7aXQRAUuNecG";
    let sovereign_account_id_index_2 = "15ySsNFkAhswdn9hSKkzoK7LhmJrj8bgyUZQAiM7Df9JpBUH";
    let sovereign_account_id_index_3 = "15s3DuzMeftBH7YdHykwPDUd2DBxdNbiyqgDfZDA3i5eRwUW";
    let sovereign_account_id_index_4 = "12uNvUSK39SDbHbqWuMhFdw2hHySrkbenVrHdS678fkj9BBb";
    let sovereign_account_id_index_5 = "14tDkT3U93Pc1wLrHEjfYuhPPnFpMwDr7o8phPCTwTRj5wfE";

    let para_storage_key = parachain_header_storage_key(2019);

    let p0 = "5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc456d08aea5b028f73699523ae21709a815640ec97748f5b5da9a2298e830e8971df7908861e1710b957fe06f0703bca7d";
    let p1 = "5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc491af1d8906a21795a98d84506b4216828886ca7474c66c027a9dc5d73901481568d551d01f13d0eb3bd36dd20ed2f13e";
    let p2 = "5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4bc407118eef848de28bad224cc60c8fedbfdc1d87d5f65f94602d8e31d7426de7b7d0bb4a8e9223b72bb39231737377c";
    let p3 = "5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc43802b39b0fa95ce7e37486987631cef3d71aafa64aa8d5d9164071a14106cdba71fe4b8fc08de39a7606c8d16f5b20f8";
    let p4 = "5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4b8ac2e6f27924e06455d8e5c75b6a775542eca539ca4e92c4b4e0496bef934595597645e21eb29b5ce94bc38b7e45181";
    let p5 = "5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4232647f67d6923193db6aedafe0d5ea3abc522231610ec74f6391785db08c68f3c07c1bb45e2193b413f66926dec5072";

    let s0 = StorageKey(hex::decode(p0).expect("Failed to decode hex string p0"));
    let s1 = StorageKey(hex::decode(p1).expect("Failed to decode hex string p1"));
    let s2 = StorageKey(hex::decode(p2).expect("Failed to decode hex string p2"));
    let s3 = StorageKey(hex::decode(p3).expect("Failed to decode hex string p3"));
    let s4 = StorageKey(hex::decode(p4).expect("Failed to decode hex string p4"));
    let s5 = StorageKey(hex::decode(p5).expect("Failed to decode hex string p5"));

    let tuple = vec![
        (sovereign_account_id_index_0, s0, 0), 
        (sovereign_account_id_index_1, s1, 1), 
        (sovereign_account_id_index_2, s2, 2), 
        (sovereign_account_id_index_3, s3, 3), 
        (sovereign_account_id_index_4, s4, 4), 
        (sovereign_account_id_index_5, s5, 5)
    ];
    let relay = std::env::var("RELAY_HOST").unwrap_or_else(|_| "rpc.polkadot.io".to_string());
	let para = std::env::var("PARA_HOST").unwrap_or_else(|_| "rpc.polkadot.io".to_string());

	let relay_ws_url = format!("wss://{relay}:443");
	let relay_ws_url = format!("ws://127.0.0.1:8001");
	let para_ws_url = format!("ws://127.0.0.1:8000");

    let relay_ws_client = Arc::new(WsClientBuilder::default().build(relay_ws_url).await.unwrap());
    let relay_client = OnlineClient::<PolkadotConfig>::from_rpc_client(relay_ws_client.clone()).await.unwrap();
    let para_ws_client = Arc::new(WsClientBuilder::default().build(para_ws_url).await.unwrap());
    let para_client = OnlineClient::<ComposableConfig>::from_rpc_client(para_ws_client.clone()).await.unwrap();

    
    while true{

        for i in &tuple{

            let keys = vec![i.1.as_ref()];

            let state_proof: Vec<Vec<u8>> = vec![]; //TODO uncomment when rpc method is available
                            // relay_client
                            // .rpc()
                            // .read_proof(keys.iter().map(AsRef::as_ref), None)
                            // .await.unwrap()
                            // .proof
                            // .into_iter()
                            // .map(|p| p.0)
                            // .collect();
            // assert!(state_proof.len() > 0);

            let block_hash =
                        relay_client.rpc().block_hash(None).await.unwrap().unwrap();
            println!("block_hash: {:?}", block_hash);

            use subxt::utils::AccountId32;
            let account_id = AccountId32::from_str(i.0).unwrap();
            let staking = relaychain::api::storage().staking().ledger(account_id);

            let Some(ledger) = relay_client
                .storage()
                .at(block_hash)
                .fetch(&staking)
                .await.unwrap() else {
                    println!("ledger not found");
                    continue;
                };
                


            let sl = hyperspace_core::substrate::composable::relaychain::api::runtime_types::pallet_staking::StakingLedger::try_from(ledger).unwrap();
            println!("sl: {:?}", sl);

            
            let mut unlocking = vec![];
            for chunk in sl.unlocking.0.iter(){
                let e = UnlockChunk { value: chunk.value, era: chunk.era, };
                unlocking.push(e);
            }

            let mut claimed_rewards = vec![];

            for claimed_reward in sl.claimed_rewards.0.iter(){
                let e = claimed_reward.clone();
                claimed_rewards.push(e);
            }

            let xxx = StakingLedger::<AccountId32, u128> {
                stash: AccountId32::from_str(i.0).unwrap(),
                total: sl.total,
                active: sl.active,
                unlocking: unlocking,
                claimed_rewards: claimed_rewards
            };
            let tx_set_staking_ledger = parachain_subxt::api::tx().pallet_liquid_staking().set_staking_ledger(i.2, xxx, state_proof);
            let tx_value = parachain_subxt::api::tx().pallet_liquid_staking().initiate_exchange_rate();

            let api = OnlineClient::<subxt::SubstrateConfig>::from_url("ws://127.0.0.1:8000").await.unwrap();

            use subxt::ext::sp_core::Pair;
            //test wallet for lsd testing 5DPqUqEfnp3buHaqiVnPt8ryykJEQRgdqAjbscnrZG2qDADa
            let key = sp_keyring::sr25519::sr25519::Pair::from_string(&"private sentence hip meadow place say issue winner express edge royal aerobic", None).expect("secret");
            let signer: PairSigner<SubstrateConfig, sp_keyring::sr25519::sr25519::Pair> = PairSigner::new(key.clone());

            sign_and_submit_staking_ledger_update(&api, tx_set_staking_ledger, signer).await;
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }

        //sleep 5 hours before next submit of ledger proof to lsd pallet on composable
        tokio::time::sleep(std::time::Duration::from_secs(60 * 50 * 5)).await;
    }

    todo!();
}

async fn sign_and_submit_staking_ledger_update(api: &OnlineClient<SubstrateConfig>, p: Payload<SetStakingLedger>, s: PairSigner<SubstrateConfig, sp_keyring::sr25519::sr25519::Pair>){
    let mut i = 10;
    while i > 0 {
        let signed =
            api.tx().sign_and_submit_then_watch(&p, &s, <_>::default()).await;
        println!("signed: {:?}", signed);
        i -= 1;
        match signed {
            Ok(_) => {
                i = 0;
            },
            Err(e) => {
                println!("Error: {:?}", e);
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            },
        } 
    }
}


// let event = relay_client.events();
    // let stream = relay_client.blocks().subscribe_finalized().await.unwrap()
    // .filter_map(|block| async {
    //     let block = block.ok().unwrap();
    //     let hash = block.hash();
    //     let events = event.at(hash).await.ok().unwrap();
    //     Some(events)
    // });