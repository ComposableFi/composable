use futures::StreamExt;
use hyperspace::logging;
use parachain::{ParachainClient, ParachainClientConfig};
use sp_core::crypto::KeyTypeId;
use sp_keystore::{testing::KeyStore, SyncCryptoStore, SyncCryptoStorePtr};
use sp_runtime::MultiSigner;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Args {
	pub chain_a: String,
	pub chain_b: String,
	pub relay_chain: String,
	pub para_id_a: u32,
	pub para_id_b: u32,
	pub connection_prefix_a: String,
	pub connection_prefix_b: String,
}

impl Default for Args {
	fn default() -> Self {
		Args {
			chain_a: "ws://127.0.0.1:9988".to_string(),
			chain_b: "ws://127.0.0.1:9188".to_string(),
			relay_chain: "ws://127.0.0.1:9944".to_string(),
			para_id_a: 2001,
			para_id_b: 2000,
			connection_prefix_a: "ibc/".to_string(),
			connection_prefix_b: "ibc/".to_string(),
		}
	}
}

#[derive(Debug, Clone)]
pub enum DefaultConfig {}

impl subxt::Config for DefaultConfig {
	type Index = u32;
	type BlockNumber = u32;
	type Hash = sp_core::H256;
	type Hashing = sp_runtime::traits::BlakeTwo256;
	type AccountId = sp_runtime::AccountId32;
	type Address = sp_runtime::MultiAddress<Self::AccountId, u32>;
	type Header = sp_runtime::generic::Header<Self::BlockNumber, sp_runtime::traits::BlakeTwo256>;
	type Signature = sp_runtime::MultiSignature;
	type Extrinsic = sp_runtime::OpaqueExtrinsic;
}

#[tokio::test]
async fn parachain_to_parachain_ibc_messaging_full_integration_test() {
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
		name: format!("ParaId({})", args.para_id_a),
		para_id: args.para_id_a,
		parachain_rpc_url: args.chain_a,
		relay_chain_rpc_url: args.relay_chain.clone(),
		client_id: None,
		commitment_prefix: args.connection_prefix_b.as_bytes().to_vec(),
		public_key: alice_pub_key.clone(),
		key_store: key_store.clone(),
		ss58_version: 49,
		key_type_id,
	};

	let config_b = ParachainClientConfig {
		name: format!("ParaId({})", args.para_id_b),
		para_id: args.para_id_b,
		parachain_rpc_url: args.chain_b,
		relay_chain_rpc_url: args.relay_chain,
		client_id: None,
		commitment_prefix: args.connection_prefix_b.as_bytes().to_vec(),
		public_key: alice_pub_key,
		key_store,
		ss58_version: 49,
		key_type_id,
	};

	let chain_a = ParachainClient::<DefaultConfig>::new(config_a).await.unwrap();
	let chain_b = ParachainClient::<DefaultConfig>::new(config_b).await.unwrap();

	// Wait until for parachains to start producing blocks
	log::info!("Waiting for  block production from parachains");
	let _ = chain_a
		.para_client
		.rpc()
		.subscribe_blocks()
		.await
		.unwrap()
		.take(2)
		.collect::<Vec<_>>()
		.await;
	log::info!("Parachains have started block production");

	hyperspace_testsuite::send_packet_and_assert_acknowledgment(&chain_a, &chain_b).await;
}
