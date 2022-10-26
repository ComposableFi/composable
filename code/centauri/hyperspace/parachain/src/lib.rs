#![allow(clippy::all)]

use std::{str::FromStr, sync::Arc};

pub mod chain;
pub mod error;
pub mod key_provider;
pub mod parachain;
pub mod polkadot;
pub mod provider;
pub mod signer;
pub mod utils;

pub mod finality_protocol;
#[cfg(any(test, feature = "testing"))]
pub mod test_provider;

use error::Error;
use serde::Deserialize;

use beefy_light_client_primitives::{ClientState, MmrUpdateProof};
use beefy_prover::Prover;
use ibc::core::ics24_host::identifier::{ChannelId, ClientId, PortId};
use ics11_beefy::client_message::ParachainHeader;
use pallet_mmr_primitives::BatchProof;
use sp_core::{ecdsa, ed25519, sr25519, Bytes, Pair, H256};
use sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr};
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	KeyTypeId, MultiSignature,
};
use ss58_registry::Ss58AddressFormat;
use subxt::{
	ext::sp_runtime::{generic::Era, traits::Header as HeaderT, MultiSigner},
	tx::{AssetTip, BaseExtrinsicParamsBuilder, ExtrinsicParams},
	PolkadotConfig,
};

use crate::utils::{fetch_max_extrinsic_weight, unsafe_cast_to_jsonrpsee_client};
use primitives::KeyProvider;

use crate::{finality_protocol::FinalityProtocol, signer::ExtrinsicSigner};
use grandpa_prover::GrandpaProver;
use ics10_grandpa::client_state::ClientState as GrandpaClientState;
use jsonrpsee_ws_client::WsClientBuilder;
use sp_keyring::AccountKeyring;
use sp_keystore::testing::KeyStore;
use sp_runtime::traits::One;
use subxt::tx::{
	PairSigner, PlainTip, PolkadotExtrinsicParamsBuilder, SubstrateExtrinsicParamsBuilder,
	TxPayload,
};

/// Implements the [`crate::Chain`] trait for parachains.
/// This is responsible for:
/// 1. Tracking a parachain light client on a counter-party chain, advancing this light
/// client state  as new finality proofs are observed.
/// 2. Submiting new IBC messages to this parachain.
#[derive(Clone)]
pub struct ParachainClient<T: subxt::Config> {
	/// Chain name
	pub name: String,
	/// Relay chain rpc client
	pub relay_client: subxt::OnlineClient<PolkadotConfig>,
	/// Parachain rpc client
	pub para_client: subxt::OnlineClient<T>,
	/// Relay chain ws client
	pub relay_ws_client: Arc<jsonrpsee_ws_client::WsClient>,
	/// Parachain ws client
	pub para_ws_client: Arc<jsonrpsee_ws_client::WsClient>,
	/// Parachain Id
	pub para_id: u32,
	/// Light client id on counterparty chain
	pub client_id: Option<ClientId>,
	/// Commitment prefix
	pub commitment_prefix: Vec<u8>,
	/// Public key for relayer on chain
	pub public_key: MultiSigner,
	/// Reference to keystore
	pub key_store: SyncCryptoStorePtr,
	/// Key type Id
	pub key_type_id: KeyTypeId,
	/// used for encoding relayer address.
	pub ss58_version: Ss58AddressFormat,
	/// the maximum extrinsic weight allowed by this client
	pub max_extrinsic_weight: u64,
	/// Channels cleared for packet relay
	pub channel_whitelist: Vec<(ChannelId, PortId)>,
	/// Finality protocol
	pub finality_protocol: FinalityProtocol,
}

enum KeyType {
	Sr25519,
	Ed25519,
	Ecdsa,
}

impl KeyType {
	pub fn to_key_type_id(&self) -> KeyTypeId {
		match self {
			KeyType::Sr25519 => KeyTypeId(sr25519::CRYPTO_ID.0),
			KeyType::Ed25519 => KeyTypeId(ed25519::CRYPTO_ID.0),
			KeyType::Ecdsa => KeyTypeId(ecdsa::CRYPTO_ID.0),
		}
	}
}

impl FromStr for KeyType {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"sr25519" => Ok(KeyType::Sr25519),
			"ed25519" => Ok(KeyType::Ed25519),
			"ecdsa" => Ok(KeyType::Ecdsa),
			_ => Err(Error::Custom("Invalid key type".to_string())),
		}
	}
}

/// config options for [`ParachainClient`]
#[derive(Debug, Deserialize)]
pub struct ParachainClientConfig {
	/// Chain name
	pub name: String,
	/// Parachain Id
	pub para_id: u32,
	/// rpc url for parachain
	pub parachain_rpc_url: String,
	/// rpc url for relay chain
	pub relay_chain_rpc_url: String,
	/// Light client id on counterparty chain
	pub client_id: Option<ClientId>,
	/// Commitment prefix
	pub commitment_prefix: Bytes,
	/// Raw private key for signing transactions
	pub private_key: String,
	/// used for encoding relayer address.
	pub ss58_version: u8,
	/// Channels cleared for packet relay
	pub channel_whitelist: Vec<(ChannelId, PortId)>,
	/// Finality protocol
	pub finality_protocol: FinalityProtocol,
	/// Digital signature scheme
	pub key_type: String,
}

impl<T: subxt::Config + Send + Sync> ParachainClient<T>
where
	u32: From<<<T as subxt::Config>::Header as HeaderT>::Number>,
	Self: KeyProvider,
	<T::Signature as Verify>::Signer: From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	MultiSigner: From<MultiSigner>,
	<T as subxt::Config>::Address: From<<T as subxt::Config>::AccountId>,
	T::Signature: From<MultiSignature>,
	H256: From<T::Hash>,
	T::BlockNumber: From<u32> + Ord + sp_runtime::traits::Zero + One,
	<T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams:
		From<BaseExtrinsicParamsBuilder<T, AssetTip>>,
{
	/// Initializes a [`ParachainClient`] given a [`ParachainConfig`]
	pub async fn new(config: ParachainClientConfig) -> Result<Self, Error> {
		let relay_ws_client = Arc::new(
			WsClientBuilder::default()
				.build(&config.relay_chain_rpc_url)
				.await
				.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?,
		);
		let para_ws_client = Arc::new(
			WsClientBuilder::default()
				.build(&config.parachain_rpc_url)
				.await
				.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?,
		);

		let para_client = subxt::OnlineClient::from_rpc_client(unsafe {
			unsafe_cast_to_jsonrpsee_client(&para_ws_client)
		})
		.await?;

		let relay_client = subxt::OnlineClient::from_rpc_client(unsafe {
			unsafe_cast_to_jsonrpsee_client(&relay_ws_client)
		})
		.await?;

		let max_extrinsic_weight = fetch_max_extrinsic_weight(&para_client).await?;

		let key_store: SyncCryptoStorePtr = Arc::new(KeyStore::new());
		let key_type = KeyType::from_str(&config.key_type)?;
		let key_type_id = key_type.to_key_type_id();

		let public_key: MultiSigner = match key_type {
			KeyType::Sr25519 => sr25519::Pair::from_string_with_seed(&config.private_key, None)
				.map_err(|_| Error::Custom("invalid key".to_owned()))?
				.0
				.public()
				.into(),
			KeyType::Ed25519 => ed25519::Pair::from_string_with_seed(&config.private_key, None)
				.map_err(|_| Error::Custom("invalid key".to_owned()))?
				.0
				.public()
				.into(),
			KeyType::Ecdsa => ecdsa::Pair::from_string_with_seed(&config.private_key, None)
				.map_err(|_| Error::Custom("invalid key".to_owned()))?
				.0
				.public()
				.into(),
		};

		SyncCryptoStore::insert_unknown(
			&*key_store,
			key_type_id,
			&*config.private_key,
			public_key.as_ref(),
		)
		.unwrap();

		Ok(Self {
			name: config.name,
			para_client,
			relay_client,
			para_id: config.para_id,
			client_id: config.client_id,
			commitment_prefix: config.commitment_prefix.0,
			public_key,
			key_store,
			key_type_id,
			max_extrinsic_weight,
			para_ws_client,
			relay_ws_client,
			ss58_version: Ss58AddressFormat::from(config.ss58_version),
			channel_whitelist: config.channel_whitelist,
			finality_protocol: config.finality_protocol,
		})
	}

	/// Returns a grandpa proving client.
	pub fn grandpa_prover(&self) -> GrandpaProver<T> {
		let relay_ws_client = unsafe { unsafe_cast_to_jsonrpsee_client(&self.relay_ws_client) };
		let para_ws_client = unsafe { unsafe_cast_to_jsonrpsee_client(&self.para_ws_client) };
		GrandpaProver {
			relay_client: self.relay_client.clone(),
			relay_ws_client,
			para_client: self.para_client.clone(),
			para_ws_client,
			para_id: self.para_id,
		}
	}

	/// Queries parachain headers that have been finalized by BEEFY in between the given relay chain
	/// heights
	pub async fn query_beefy_finalized_parachain_headers_between(
		&self,
		commitment_block_number: u32,
		client_state: &ClientState,
	) -> Result<Vec<T::Header>, Error>
	where
		u32: From<T::BlockNumber>,
		T::BlockNumber: From<u32>,
	{
		let client_wrapper = Prover {
			relay_client: self.relay_client.clone(),
			para_client: self.para_client.clone(),
			beefy_activation_block: client_state.beefy_activation_block,
			para_id: self.para_id,
		};

		let headers = client_wrapper
			.query_finalized_parachain_headers_at(
				commitment_block_number,
				client_state.latest_beefy_height,
			)
			.await
			.map_err(|e| {
				Error::from(format!("[fetch_finalized_parachain_headers_at] Failed due to {:?}", e))
			})?;

		Ok(headers)
	}

	/// Construct the [`ParachainHeadersWithFinalityProof`] for parachain headers with the given
	/// numbers using the BEEFY finality proof with the given relay chain heights.
	pub async fn query_beefy_finalized_parachain_headers_with_proof(
		&self,
		commitment_block_number: u32,
		client_state: &ClientState,
		headers: Vec<T::BlockNumber>,
	) -> Result<(Vec<ParachainHeader>, BatchProof<H256>), Error>
	where
		T::BlockNumber: Ord + sp_runtime::traits::Zero,
	{
		let client_wrapper = Prover {
			relay_client: self.relay_client.clone(),
			para_client: self.para_client.clone(),
			beefy_activation_block: client_state.beefy_activation_block,
			para_id: self.para_id,
		};

		let (parachain_headers, batch_proof) = client_wrapper
			.query_finalized_parachain_headers_with_proof(
				commitment_block_number,
				client_state.latest_beefy_height,
				headers,
			)
			.await
			.map_err(|e| {
				Error::from(format!("[fetch_finalized_parachain_headers_at] Failed due to {:?}", e))
			})?;

		let parachain_headers = parachain_headers
			.into_iter()
			.map(|para_header| {
				Ok(ParachainHeader {
					parachain_header: codec::Decode::decode(&mut &*para_header.parachain_header)?,
					partial_mmr_leaf: para_header.partial_mmr_leaf,
					parachain_heads_proof: para_header.parachain_heads_proof,
					heads_leaf_index: para_header.heads_leaf_index,
					heads_total_count: para_header.heads_total_count,
					extrinsic_proof: para_header.extrinsic_proof,
					timestamp_extrinsic: para_header.timestamp_extrinsic,
				})
			})
			.collect::<Result<Vec<_>, codec::Error>>()?;

		Ok((parachain_headers, batch_proof))
	}

	/// Queries for the BEEFY mmr update proof for the given signed commitment height.
	pub async fn query_beefy_mmr_update_proof(
		&self,
		signed_commitment: beefy_primitives::SignedCommitment<
			u32,
			beefy_primitives::crypto::Signature,
		>,
		client_state: &ClientState,
	) -> Result<MmrUpdateProof, Error> {
		let prover = Prover {
			relay_client: self.relay_client.clone(),
			para_client: self.para_client.clone(),
			beefy_activation_block: client_state.beefy_activation_block,
			para_id: self.para_id,
		};

		let mmr_update =
			prover.fetch_mmr_update_proof_for(signed_commitment).await.map_err(|e| {
				Error::from(format!("[fetch_mmr_update_proof_for] Failed due to {:?}", e))
			})?;
		Ok(mmr_update)
	}

	/// Submits the given transaction to the parachain node, waits for it to be included in a block
	/// and asserts that it was successfully dispatched on-chain.
	///
	/// We retry sending the transaction up to 5 times in the case where the transaction pool might
	/// reject the transaction because of conflicting nonces.
	pub async fn submit_call<C: TxPayload>(
		&self,
		call: C,
	) -> Result<(T::Hash, Option<T::Hash>), Error> {
		let signer = ExtrinsicSigner::<T, Self>::new(
			self.key_store.clone(),
			self.key_type_id.clone(),
			self.public_key.clone(),
		);

		// Submit extrinsic to parachain node
		let tip = 100_000u128;
		// Try extrinsic submission five times in case of failures
		let mut count = 0;
		let progress = loop {
			if count == 5 {
				Err(Error::Custom("Failed to submit extrinsic after 5 tries".to_string()))?
			}

			let tx_params = <SubstrateExtrinsicParamsBuilder<T>>::new()
				// todo: tx should be mortal
				.era(Era::Immortal, self.para_client.genesis_hash());

			let res = self
				.para_client
				.tx()
				.sign_and_submit_then_watch(
					&call,
					&signer,
					tx_params.tip(AssetTip::new(count * tip)).into(),
				)
				.await;
			match res {
				Ok(progress) => break progress,
				Err(e) => {
					log::warn!("Failed to submit extrinsic: {:?}. Retrying...", e);
					count += 1;
				},
			}
		};

		let tx_in_block = progress.wait_for_in_block().await?;
		tx_in_block.wait_for_success().await?;
		Ok((tx_in_block.extrinsic_hash(), Some(tx_in_block.block_hash())))
	}

	pub async fn submit_call_relaychain<C: TxPayload>(
		&self,
		call: C,
	) -> Result<(H256, Option<H256>), Error> {
		let signer = PairSigner::new(AccountKeyring::Alice.pair());

		// Submit extrinsic to relaychain node
		let tip = 100_000u128;
		// Try extrinsic submission five times in case of failures
		let mut count = 0;
		let progress = loop {
			if count == 5 {
				Err(Error::Custom("Failed to submit extrinsic after 5 tries".to_string()))?
			}

			let tx_params = PolkadotExtrinsicParamsBuilder::new()
				.era(Era::Immortal, self.relay_client.genesis_hash());

			let res = self
				.relay_client
				.tx()
				.sign_and_submit_then_watch(
					&call,
					&signer,
					tx_params.tip(PlainTip::new(count * tip)),
				)
				.await;
			match res {
				Ok(progress) => break progress,
				Err(e) => {
					log::warn!("Failed to submit extrinsic: {:?}. Retrying...", e);
					count += 1;
				},
			}
		};

		let tx_in_block = progress.wait_for_in_block().await?;
		tx_in_block.wait_for_success().await?;
		Ok((tx_in_block.extrinsic_hash(), Some(tx_in_block.block_hash())))
	}

	pub fn client_id(&self) -> ClientId {
		self.client_id.as_ref().expect("Client Id should be defined").clone()
	}
}
