use hyperspace_core::substrate::DefaultConfig as PolkadotConfig;
use jsonrpsee::ws_client::WsClientBuilder;
use sp_core::storage::StorageKey;
use std::{str::FromStr, sync::Arc};
use subxt::{
	tx::{PairSigner, Payload},
	OnlineClient, SubstrateConfig,
};

cfg_if::cfg_if! {
	if #[cfg(feature = "composable")] {
		use hyperspace_core::substrate::ComposableConfig as LsdConfig;
		use hyperspace_core::substrate::{
			composable::{
				parachain_subxt,
				parachain_subxt::api::pallet_liquid_staking::calls::types::SetStakingLedger, relaychain,
				parachain_subxt::api::runtime_types::pallet_liquid_staking::types::{
					StakingLedger, UnlockChunk,
				}
			},
		};
	} else {
		use hyperspace_core::substrate::PicassoKusamaConfig as LsdConfig;
		use hyperspace_core::substrate::{
			picasso_kusama::{
				parachain_subxt,
				parachain_subxt::api::pallet_liquid_staking::calls::types::SetStakingLedger, relaychain,
				parachain_subxt::api::runtime_types::pallet_liquid_staking::types::{
					StakingLedger, UnlockChunk,
				}
			},
		};
	}
}

mod config;

pub struct SovereignSubAccount {
	pub address: String,
	pub storage_key: StorageKey,
	pub derivative_index: u16,
}

#[tokio::main]
async fn main() {
	/*
		SEED="some twelve words" RELAY_HOST=ws://127.0.0.1:8000 PARA_HOST=ws://127.0.0.1:8001 SLEEP_TIME_MIN=60 cargo run -p lsd-relayer --features composable
	*/

	let sub_accounts = config::get_config();

	let seed_phrase = std::env::var("SEED").unwrap_or_else(|_| {
		"private sentence hip meadow place say issue winner express edge royal aerobic".to_string()
	});

	let sleep_time_min = std::env::var("SLEEP_TIME_MIN").unwrap_or_else(|_| "60".to_string());
	let sleep_time_min = sleep_time_min.parse::<u64>().unwrap_or_else(|_| 60);

	let relay = std::env::var("RELAY_HOST")
		.unwrap_or_else(|_| "wss://polkadot-rpc-tn.dwellir.com:443".to_string());
	let para = std::env::var("PARA_HOST")
		.unwrap_or_else(|_| "wss://composable-rpc.dwellir.com:443".to_string());

	println!("Configuration:");
	println!("relay: {}", relay);
	println!("para: {}", para);
	println!("sleep_time_min: {}", sleep_time_min);

	let relay_ws_url = relay.as_str();
	let para_ws_url = para.as_str();

	let relay_ws_client = Arc::new(WsClientBuilder::default().build(relay_ws_url).await.unwrap());
	let relay_client = OnlineClient::<PolkadotConfig>::from_rpc_client(relay_ws_client.clone())
		.await
		.unwrap();
	let para_api = OnlineClient::<subxt::SubstrateConfig>::from_url(para_ws_url).await.unwrap();

	loop {
		for i in &sub_accounts {
			let keys = vec![i.storage_key.as_ref()];

			let block_hash_para = para_api.rpc().block_hash(None).await.unwrap().unwrap();
			let validation_data =
				parachain_subxt::api::storage().pallet_liquid_staking().validation_data();
			let Some(validation_data) = para_api.storage().at(block_hash_para).fetch(&validation_data).await.expect("PalletLiquidStaking pallet should be available in the runtime")
			else {
				println!("validation data: {} not found", i.address);
				continue;
			};

			use crate::parachain_subxt::api::runtime_types::polkadot_primitives::v4::PersistedValidationData;

			let validation_data = PersistedValidationData::try_from(validation_data)
				.expect("Failed to decode PersistedValidationData");
			// println!("validation_data: {:?}", validation_data);

			let block_hash = relay_client
				.rpc()
				.block_hash(Some(validation_data.relay_parent_number.into()))
				.await
				.unwrap()
				.unwrap();
			println!("block_hash: {:?}", block_hash);

			let state_proof: Vec<Vec<u8>> = relay_client
				.rpc()
				.read_proof(keys.iter().map(AsRef::as_ref), None)
				.await
				.unwrap()
				.proof
				.into_iter()
				.map(|p| p.0)
				.collect();
			assert!(state_proof.len() > 0);

			use subxt::utils::AccountId32;
			let account_id =
				AccountId32::from_str(&i.address).expect("Failed to decode AccountId32");
			let staking = relaychain::api::storage().staking().ledger(account_id);

			let Some(ledger) = relay_client.storage().at(block_hash).fetch(&staking).await.expect("Staking pallet should be available in the runtime")
			else {
				println!("ledger: {} not found", i.address);
				continue;
			};

			let relaychain_staking_ledger =
				relaychain::api::runtime_types::pallet_staking::StakingLedger::try_from(ledger)
					.expect("Failed to decode StakingLedger");

			let mut unlocking = vec![];
			for chunk in relaychain_staking_ledger.unlocking.0.iter() {
				let e = UnlockChunk { value: chunk.value, era: chunk.era };
				unlocking.push(e);
			}

			let mut claimed_rewards = vec![];

			for claimed_reward in relaychain_staking_ledger.claimed_rewards.0.iter() {
				let e = claimed_reward.clone();
				claimed_rewards.push(e);
			}

			let para_input_staking_ledger = StakingLedger::<AccountId32, u128> {
				stash: AccountId32::from_str(&i.address).expect("Failed to decode AccountId32"),
				total: relaychain_staking_ledger.total,
				active: relaychain_staking_ledger.active,
				unlocking,
				claimed_rewards,
			};
			let tx_set_staking_ledger = parachain_subxt::api::tx()
				.pallet_liquid_staking()
				.set_staking_ledger(i.derivative_index, para_input_staking_ledger, state_proof);

			use subxt::ext::sp_core::Pair;
			//test wallet for lsd testing 5DPqUqEfnp3buHaqiVnPt8ryykJEQRgdqAjbscnrZG2qDADa
			let key = sp_keyring::sr25519::sr25519::Pair::from_string(&seed_phrase, None)
				.expect("secret");
			let signer: PairSigner<SubstrateConfig, sp_keyring::sr25519::sr25519::Pair> =
				PairSigner::new(key.clone());

			sign_and_submit_staking_ledger_update(&para_api, tx_set_staking_ledger, signer).await;
			tokio::time::sleep(std::time::Duration::from_secs(10)).await;
		}

		//sleep N hours before next submit of ledger proof to lsd pallet on composable
		tokio::time::sleep(std::time::Duration::from_secs(60 * sleep_time_min)).await;
	}
}

async fn sign_and_submit_staking_ledger_update(
	api: &OnlineClient<SubstrateConfig>,
	p: Payload<SetStakingLedger>,
	s: PairSigner<SubstrateConfig, sp_keyring::sr25519::sr25519::Pair>,
) {
	let mut i = 10;
	while i > 0 {
		let signed = api.tx().sign_and_submit_then_watch(&p, &s, <_>::default()).await;
		// println!("signed: {:?}", signed);
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
