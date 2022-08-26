use crate::error::Error;
use ibc::core::ics04_channel::packet::Packet;
use ibc::core::ics24_host::identifier::ClientId;
use near_crypto::{ED25519PublicKey, KeyType, Secp256K1PublicKey};
use near_indexer::{Indexer, InitConfigArgs};
use near_jsonrpc_client::methods::broadcast_tx_commit::{
	RpcBroadcastTxCommitRequest, RpcBroadcastTxCommitResponse,
};
use near_jsonrpc_client::JsonRpcClient;
use near_jsonrpc_primitives::types::query::{QueryResponseKind, RpcQueryRequest};
use near_primitives::transaction::{SignedTransaction, Transaction};
use near_primitives::types::{AccountId, Finality};
use sp_core::crypto::{CryptoTypeId, CryptoTypePublicPair, KeyTypeId};
use sp_keystore::SyncCryptoStorePtr;

mod chain;
mod error;
mod key_provider;
pub mod provider;

/// Implements the [`crate::Chain`] trait for NEAR.
pub struct Client {
	/// Near rpc client
	pub rpc_client: JsonRpcClient,
	/// Light client id on counterparty chain
	pub client_id: Option<ClientId>,
	/// Core contract id
	pub contract_id: AccountId,
	/// Near's latest finalized height
	pub latest_near_height: Option<u32>,
	/// Commitment prefix
	pub commitment_prefix: Vec<u8>,
	/// Sent packet sequence cache
	pub packet_cache: Vec<Packet>,
	/// Indexer that provides a stream of finalized messages from Near
	pub indexer: Indexer,
	/// Reference to keystore
	pub key_store: SyncCryptoStorePtr,
	/// 4 byte Key type id
	pub key_type_id: KeyTypeId,
	/// Signer's account id on Near
	pub signer: AccountId,
}

/// config options for [`Client`]
pub struct ClientConfig {
	/// rpc url for Near node
	pub rpc_url: String,
	/// Light client id on counterparty chain
	pub client_id: Option<ClientId>,
	/// Core contract id
	pub contract_id: AccountId,
	/// Commitment prefix
	pub commitment_prefix: Vec<u8>,
	/// Reference to keystore
	pub key_store: SyncCryptoStorePtr,
	/// 4 byte Key type id
	pub key_type_id: KeyTypeId,
	/// Signer's account id on Near
	pub signer: AccountId,
	/// Indexer configuration
	pub indexer_config: near_indexer::IndexerConfig,
}

impl Client {
	/// Initializes configurations for indexer with a default config.
	pub fn init_configs_default() -> anyhow::Result<()> {
		let home_dir = near_indexer::get_default_home();
		near_indexer::indexer_init_configs(
			&home_dir,
			InitConfigArgs {
				chain_id: None,
				account_id: None,
				test_seed: None,
				num_shards: 1,
				fast: false,
				genesis: None,
				download_genesis: false,
				download_genesis_url: None,
				download_config: false,
				download_config_url: None,
				boot_nodes: None,
				max_gas_burnt_view: None,
			},
		)
	}

	pub fn new(config: ClientConfig) -> Self {
		openssl_probe::init_ssl_cert_env_vars();
		let indexer = Indexer::new(config.indexer_config).expect("failed to crate indexer");
		let rpc_client = JsonRpcClient::connect(&config.rpc_url);

		Self {
			rpc_client,
			client_id: None,
			contract_id: config.contract_id,
			latest_near_height: None,
			commitment_prefix: vec![],
			packet_cache: vec![],
			indexer,
			key_store: config.key_store,
			signer: config.signer,
			key_type_id: config.key_type_id,
		}
	}

	pub async fn send_transaction(
		&self,
		prepopulated_unsigned_transaction: Transaction,
	) -> Result<RpcBroadcastTxCommitResponse, Error> {
		let public_key = self.public_key();
		let online_signer_access_key_response = self
			.rpc_client
			.call(RpcQueryRequest {
				block_reference: Finality::Final.into(),
				request: near_primitives::views::QueryRequest::ViewAccessKey {
					account_id: prepopulated_unsigned_transaction.signer_id.clone(),
					public_key: public_key.clone(),
				},
			})
			.await?;
		let current_nonce = if let QueryResponseKind::AccessKey(online_signer_access_key) =
			online_signer_access_key_response.kind
		{
			online_signer_access_key.nonce
		} else {
			unreachable!("we've requested the access key");
		};
		let unsigned_transaction = Transaction {
			public_key,
			block_hash: online_signer_access_key_response.block_hash,
			nonce: current_nonce + 1,
			..prepopulated_unsigned_transaction
		};
		let signature = self.sign(unsigned_transaction.get_hash_and_size().0.as_ref());
		let signed_transaction = SignedTransaction::new(signature, unsigned_transaction);
		// TODO: retry loop for sending transaction
		let transaction_info = self
			.rpc_client
			.call(RpcBroadcastTxCommitRequest { signed_transaction: signed_transaction.clone() })
			.await?;
		Ok(transaction_info)
	}

	pub fn client_id(&self) -> ClientId {
		self.client_id.as_ref().expect("Client Id should be defined").clone()
	}

	pub fn set_client_id(&mut self, client_id: ClientId) {
		self.client_id = Some(client_id)
	}

	/// Returns the first found public key in the keystore
	pub fn keystore_key(&self) -> CryptoTypePublicPair {
		use sp_keystore::SyncCryptoStore;
		let keys =
			SyncCryptoStore::keys(&*self.key_store, self.key_type_id).expect("failed to load key");
		keys.first().expect("no keys found").clone()
	}

	pub fn public_key(&self) -> near_crypto::PublicKey {
		let key = self.keystore_key();
		let near_key_type = sp_to_near_key_type(key.0);
		match near_key_type {
			KeyType::ED25519 => {
				let public_key = ED25519PublicKey::try_from(key.1.as_slice())
					.expect("invalid ed25519 public key");
				near_crypto::PublicKey::from(public_key)
			},
			KeyType::SECP256K1 => {
				let public_key = Secp256K1PublicKey::try_from(key.1.as_slice())
					.expect("invalid secp256k1 public key");
				near_crypto::PublicKey::from(public_key)
			},
		}
	}

	pub fn sign(&self, msg: &[u8]) -> near_crypto::Signature {
		use sp_keystore::SyncCryptoStore;
		let key = self.keystore_key();
		let near_key_type = sp_to_near_key_type(key.0);
		let sig_data = SyncCryptoStore::sign_with(&*self.key_store, self.key_type_id, &key, msg)
			.ok()
			.flatten()
			.expect("failed to sign");
		near_crypto::Signature::from_parts(near_key_type, &sig_data)
			.expect("failed to parse signature")
	}
}

fn sp_to_near_key_type(sp_key_type: CryptoTypeId) -> KeyType {
	match sp_key_type {
		sp_core::ed25519::CRYPTO_ID => KeyType::ED25519,
		sp_core::ecdsa::CRYPTO_ID => KeyType::SECP256K1,
		key_type => {
			panic!("unsupported key type: {:?}", key_type);
		},
	}
}
