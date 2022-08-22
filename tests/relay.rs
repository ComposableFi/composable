use futures::StreamExt;
use hyperspace::chain::parachain::{ParachainClient, ParachainClientConfig};
use hyperspace::logging;
use ibc::core::ics24_host::identifier::ClientId;
use sp_keystore::{testing::KeyStore, SyncCryptoStore, SyncCryptoStorePtr};
use sp_runtime::{KeyTypeId, MultiSigner};
use subxt::DefaultConfig;
mod common;

use common::Args;
use std::sync::Arc;
use std::str::FromStr;

#[tokio::test]
async fn main() {
	logging::setup_logging();
	let args = Args::default();
	let alice = sp_keyring::AccountKeyring::Alice;
	let alice_pub_key = MultiSigner::Sr25519(alice.public());

	let key_store: SyncCryptoStorePtr = Arc::new(KeyStore::new());
	let key_type_id = KeyTypeId::from(0u32);

	SyncCryptoStore::insert_unknown(&*key_store, key_type_id, "//Alice", &alice.public().0)
		.unwrap();

	// Create client configurations
	let config_a = ParachainClientConfig {
		para_id: args.para_id_a,
		parachain_rpc_url: args.chain_a,
		relay_chain_rpc_url: args.relay_chain.clone(),
		client_id: None,
		commitment_prefix: args.connection_prefix_b.as_bytes().to_vec(),
		public_key: alice_pub_key.clone(),
		key_store: key_store.clone(),
		key_type_id,
	};

	let config_b = ParachainClientConfig {
		para_id: args.para_id_b,
		parachain_rpc_url: args.chain_b,
		relay_chain_rpc_url: args.relay_chain,
		client_id: None,
		commitment_prefix: args.connection_prefix_b.as_bytes().to_vec(),
		public_key: alice_pub_key,
		key_store,
		key_type_id,
	};

	let mut client_a = ParachainClient::<DefaultConfig>::new(config_a).await.unwrap();
	let mut client_b = ParachainClient::<DefaultConfig>::new(config_b).await.unwrap();

	// Wait until for parachains to start producing blocks
	let block_subscription = client_a.para_client.rpc().subscribe_blocks().await.unwrap();
	println!("Waiting for  block production from parachains");
	let _ = block_subscription.take(2).collect::<Vec<_>>().await;
	println!("Parachains have started block production");

	let client_id = ClientId::from_str("11-beefy-0").expect("Should have a valid client id");

	client_a.set_client_id(client_id.clone());

	client_b.set_client_id(client_id);

	// Start relayer loop

	hyperspace::relay(client_a, client_b).await.unwrap();
}
