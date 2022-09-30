use std::{str::FromStr, sync::Arc, time::Duration};

pub mod chain;
pub mod error;
pub mod key_provider;
pub(crate) mod parachain;
pub(crate) mod polkadot;
pub mod provider;
pub mod signer;
pub mod utils;

pub mod light_client_protocol;
#[cfg(feature = "testing")]
pub mod test_provider;

use codec::Decode;
use error::Error;
use ibc_rpc::IbcApiClient;
use serde::Deserialize;

use beefy_light_client_primitives::{ClientState, MmrUpdateProof};
use beefy_prover::{helpers::unsafe_cast_to_jsonrpsee_client, ClientWrapper};

use common::AccountId;
use ibc::{
	core::ics24_host::identifier::{ChannelId, ClientId, PortId},
	events::IbcEvent,
};
use ibc_proto::ibc::core::client::v1::IdentifiedClientState;
use ics11_beefy::{
	client_message::ParachainHeader, client_state::ClientState as BeefyClientState,
	consensus_state::ConsensusState as BeefyConsensusState,
};
use pallet_ibc::{
	light_clients::{AnyClientState, AnyConsensusState, HostFunctionsManager},
	TransferParams,
};
use pallet_mmr_primitives::BatchProof;
use signer::ExtrinsicSigner;
use sp_core::{ecdsa, ed25519, sr25519, Bytes, Pair, H256};
use sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr};
use sp_runtime::{
	generic::Era,
	traits::{IdentifyAccount, Verify},
	KeyTypeId, MultiSignature,
};
use ss58_registry::Ss58AddressFormat;
use subxt::{
	ext::sp_runtime::{traits::Header as HeaderT, MultiSigner},
	rpc::rpc_params,
	tx::{
		AssetTip, BaseExtrinsicParamsBuilder, ExtrinsicParams, SubstrateExtrinsicParamsBuilder,
		TxPayload,
	},
};
use tokio::sync::broadcast::{self, Sender};

use crate::{parachain::api, utils::fetch_max_extrinsic_weight};
use primitives::KeyProvider;

use crate::light_client_protocol::LightClientProtocol;
use grandpa_light_client_primitives::ParachainHeadersWithFinalityProof;
use grandpa_prover::GrandpaProver;
use ibc::timestamp::Timestamp;
use ics10_grandpa::{
	client_state::ClientState as GrandpaClientState,
	consensus_state::ConsensusState as GrandpaConsensusState,
};
use sp_keystore::testing::KeyStore;
use sp_runtime::traits::{One, Zero};

#[derive(Clone)]
/// Implements the [`crate::Chain`] trait for parachains.
/// This is responsible for:
/// 1. Tracking a parachain light client on a counter-party chain, advancing this light
/// client state  as new finality proofs are observed.
/// 2. Submiting new IBC messages to this parachain.
pub struct ParachainClient<T: subxt::Config> {
	/// Chain name
	pub name: String,
	/// Relay chain rpc client
	pub relay_client: subxt::OnlineClient<T>,
	/// Parachain rpc client
	pub para_client: subxt::OnlineClient<T>,
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
	/// ibc event stream sender
	pub sender: Sender<IbcEvent>,
	/// the maximum extrinsic weight allowed by this client
	pub max_extrinsic_weight: u64,
	/// Channels cleared for packet relay
	pub channel_whitelist: Vec<(ChannelId, PortId)>,
	/// Light client protocol
	pub light_client_protocol: LightClientProtocol,
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
	/// Path to a keystore file
	pub private_key: String,
	/// used for encoding relayer address.
	pub ss58_version: u8,
	/// Channels cleared for packet relay
	pub channel_whitelist: Vec<(ChannelId, PortId)>,
	/// Light client protocol
	pub light_client_protocol: LightClientProtocol,
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
	<T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams:
		From<BaseExtrinsicParamsBuilder<T, AssetTip>>,
{
	pub async fn new(config: ParachainClientConfig) -> Result<Self, Error> {
		let para_client = subxt::OnlineClient::from_url(&config.parachain_rpc_url).await?;

		let relay_client = subxt::OnlineClient::from_url(&config.relay_chain_rpc_url).await?;

		let (sender, _) = broadcast::channel(32);
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
			sender,
			max_extrinsic_weight,
			ss58_version: Ss58AddressFormat::from(config.ss58_version),
			channel_whitelist: config.channel_whitelist,
			light_client_protocol: config.light_client_protocol,
		})
	}

	pub async fn query_finalized_parachain_headers_between(
		&self,
		latest_finalized_hash: T::Hash,
		previous_finalized_hash: T::Hash,
	) -> Result<Vec<T::Header>, Error>
	where
		T::BlockNumber: From<u32>,
		T: subxt::Config,
		T::BlockNumber: Ord + Zero,
		u32: From<T::BlockNumber>,
	{
		let prover = GrandpaProver {
			relay_client: self.relay_client.clone(),
			para_client: self.para_client.clone(),
			para_id: self.para_id,
		};

		prover
			.query_finalized_parachain_headers_between(
				latest_finalized_hash,
				previous_finalized_hash,
			)
			.await
			.map_err(|e| {
				Error::from(format!(
					"[query_finalized_parachain_headers_between] Failed due to {:?}",
					e
				))
			})
	}

	pub async fn query_finalized_parachain_headers_at(
		&self,
		commitment_block_number: u32,
		client_state: &ClientState,
	) -> Result<Vec<T::Header>, Error>
	where
		u32: From<T::BlockNumber>,
		T::BlockNumber: From<u32>,
	{
		let client_wrapper = ClientWrapper {
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

	pub async fn query_grandpa_finalized_parachain_headers_with_proof(
		&self,
		latest_finalized_hash: T::Hash,
		previous_finalized_hash: T::Hash,
		headers: Vec<T::BlockNumber>,
	) -> Result<ParachainHeadersWithFinalityProof<T::Header>, Error>
	where
		T::BlockNumber: Ord + sp_runtime::traits::Zero,
		T::Header: HeaderT,
		<T::Header as HeaderT>::Hash: From<T::Hash>,
		T::BlockNumber: One,
	{
		let prover = GrandpaProver {
			relay_client: self.relay_client.clone(),
			para_client: self.para_client.clone(),
			para_id: self.para_id,
		};

		let result = prover
			.query_finalized_parachain_headers_with_proof(
				latest_finalized_hash,
				previous_finalized_hash,
				headers,
			)
			.await
			.map_err(|e| {
				Error::from(format!(
					"[query_finalized_parachain_headers_with_proof] Failed due to {:?}",
					e
				))
			})?;
		result.ok_or_else(|| {
			Error::from(
				"[query_finalized_parachain_headers_with_proof] Failed due to empty finality proof"
					.to_string(),
			)
		})
	}

	pub async fn query_beefy_finalized_parachain_headers_with_proof(
		&self,
		commitment_block_number: u32,
		client_state: &ClientState,
		headers: Vec<T::BlockNumber>,
	) -> Result<(Vec<ParachainHeader>, BatchProof<H256>), Error>
	where
		T::BlockNumber: Ord + sp_runtime::traits::Zero,
	{
		let client_wrapper = ClientWrapper {
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

	pub async fn fetch_mmr_update_proof_for(
		&self,
		signed_commitment: beefy_primitives::SignedCommitment<
			u32,
			beefy_primitives::crypto::Signature,
		>,
		client_state: &ClientState,
	) -> Result<MmrUpdateProof, Error> {
		let client_wrapper = ClientWrapper {
			relay_client: self.relay_client.clone(),
			para_client: self.para_client.clone(),
			beefy_activation_block: client_state.beefy_activation_block,
			para_id: self.para_id,
		};

		let mmr_update = client_wrapper
			.fetch_mmr_update_proof_for(signed_commitment)
			.await
			.map_err(|e| {
				Error::from(format!("[fetch_mmr_update_proof_for] Failed due to {:?}", e))
			})?;
		Ok(mmr_update)
	}

	pub fn client_id(&self) -> ClientId {
		self.client_id.as_ref().expect("Client Id should be defined").clone()
	}

	pub fn set_client_id(&mut self, client_id: ClientId) {
		self.client_id = Some(client_id)
	}

	pub fn set_channel_whitelist(&mut self, channel_whitelist: Vec<(ChannelId, PortId)>) {
		self.channel_whitelist = channel_whitelist;
	}

	/// Construct a beefy client state to be submitted to the counterparty chain
	pub async fn construct_beefy_client_state(
		&self,
		beefy_activation_block: u32,
	) -> Result<(AnyClientState, AnyConsensusState), Error>
	where
		Self: KeyProvider,
		<T::Signature as Verify>::Signer:
			From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
		MultiSigner: From<MultiSigner>,
		<T as subxt::Config>::Address: From<<T as subxt::Config>::AccountId>,
		u32: From<<T as subxt::Config>::BlockNumber>,
	{
		use ibc::core::ics24_host::identifier::ChainId;
		let api = self.relay_client.storage();
		let para_client_api = self.para_client.storage();
		let client_wrapper = ClientWrapper {
			relay_client: self.relay_client.clone(),
			para_client: self.para_client.clone(),
			beefy_activation_block,
			para_id: self.para_id,
		};
		loop {
			let beefy_state = client_wrapper
				.construct_beefy_client_state(beefy_activation_block)
				.await
				.map_err(|e| {
					Error::from(format!("[construct_beefy_client_state] Failed due to {:?}", e))
				})?;

			let subxt_block_number: subxt::rpc::BlockNumber =
				beefy_state.latest_beefy_height.into();
			let block_hash = self.relay_client.rpc().block_hash(Some(subxt_block_number)).await?;
			let heads_addr = polkadot::api::storage().paras().heads(
				&polkadot::api::runtime_types::polkadot_parachain::primitives::Id(self.para_id),
			);
			let head_data = api.fetch(&heads_addr, block_hash).await?.ok_or_else(|| {
				Error::Custom(format!(
					"Couldn't find header for ParaId({}) at relay block {:?}",
					self.para_id, block_hash
				))
			})?;
			let decoded_para_head = sp_runtime::generic::Header::<
				u32,
				sp_runtime::traits::BlakeTwo256,
			>::decode(&mut &*head_data.0)?;
			let block_number = decoded_para_head.number;
			let client_state = BeefyClientState::<HostFunctionsManager> {
				chain_id: ChainId::new("relay-chain".to_string(), 0),
				relay_chain: Default::default(),
				mmr_root_hash: beefy_state.mmr_root_hash,
				latest_beefy_height: beefy_state.latest_beefy_height,
				frozen_height: None,
				beefy_activation_block: beefy_state.beefy_activation_block,
				latest_para_height: block_number,
				para_id: self.para_id,
				authority: beefy_state.current_authorities,
				next_authority_set: beefy_state.next_authorities,
				_phantom: Default::default(),
			};
			// we can't use the genesis block to construct the initial state.
			if block_number == 0 {
				continue
			}
			let subxt_block_number: subxt::rpc::BlockNumber = block_number.into();
			let block_hash =
				self.para_client.rpc().block_hash(Some(subxt_block_number)).await.unwrap();
			let timestamp_addr = parachain::api::storage().timestamp().now();
			let unix_timestamp_millis = para_client_api
				.fetch(&timestamp_addr, block_hash)
				.await?
				.expect("Timestamp should exist");
			let timestamp_nanos = Duration::from_millis(unix_timestamp_millis).as_nanos() as u64;

			let consensus_state = AnyConsensusState::Beefy(BeefyConsensusState {
				timestamp: Timestamp::from_nanoseconds(timestamp_nanos)
					.unwrap()
					.into_tm_time()
					.unwrap(),
				root: decoded_para_head.state_root.as_bytes().to_vec().into(),
			});

			return Ok((AnyClientState::Beefy(client_state), consensus_state))
		}
	}

	pub async fn construct_grandpa_client_state(
		&self,
	) -> Result<(AnyClientState, AnyConsensusState), Error>
	where
		Self: KeyProvider,
		<T::Signature as Verify>::Signer:
			From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
		MultiSigner: From<MultiSigner>,
		<T as subxt::Config>::Address: From<<T as subxt::Config>::AccountId>,
		u32: From<<T as subxt::Config>::BlockNumber>,
	{
		let api = self.relay_client.storage();
		let para_client_api = self.para_client.storage();
		loop {
			let current_set_id_addr = polkadot::api::storage().grandpa().current_set_id();
			let current_set_id = api
				.fetch(&current_set_id_addr, None)
				.await?
				.expect("Failed to fetch current set id");

			let current_authorities = {
				let bytes = self
					.relay_client
					.rpc()
					.request::<String>(
						"state_call",
						rpc_params!("GrandpaApi_grandpa_authorities", "0x"),
					)
					.await
					.map(|res| hex::decode(&res[2..]))
					.expect("Failed to fetch authorities")
					.expect("Failed to hex decode authorities");

				sp_finality_grandpa::AuthorityList::decode(&mut &bytes[..])
					.expect("Failed to scale decode authorities")
			};

			let latest_relay_hash = self
				.relay_client
				.rpc()
				.finalized_head()
				.await
				.expect("Failed to fetch finalized header");

			let heads_addr = polkadot::api::storage().paras().heads(
				&polkadot::api::runtime_types::polkadot_parachain::primitives::Id(self.para_id),
			);
			let head_data =
				api.fetch(&heads_addr, Some(latest_relay_hash)).await?.ok_or_else(|| {
					Error::Custom(format!(
						"Couldn't find header for ParaId({}) at relay block {:?}",
						self.para_id, latest_relay_hash
					))
				})?;
			let decoded_para_head = sp_runtime::generic::Header::<
				u32,
				sp_runtime::traits::BlakeTwo256,
			>::decode(&mut &*head_data.0)?;
			let block_number = decoded_para_head.number;
			// we can't use the genesis block to construct the initial state.
			if block_number == 0 {
				continue
			}

			let mut client_state = GrandpaClientState::<HostFunctionsManager>::default();

			client_state.relay_chain = Default::default();
			client_state.current_authorities = current_authorities;
			client_state.current_set_id = current_set_id;
			client_state.latest_relay_hash = latest_relay_hash.into();
			client_state.frozen_height = None;
			client_state.latest_para_height = block_number;
			client_state.para_id = self.para_id;

			let subxt_block_number: subxt::rpc::BlockNumber = block_number.into();
			let block_hash =
				self.para_client.rpc().block_hash(Some(subxt_block_number)).await.unwrap();
			let timestamp_addr = parachain::api::storage().timestamp().now();
			let unix_timestamp_millis = para_client_api
				.fetch(&timestamp_addr, block_hash)
				.await?
				.expect("Timestamp should exist");
			let timestamp_nanos = Duration::from_millis(unix_timestamp_millis).as_nanos() as u64;

			let consensus_state = AnyConsensusState::Grandpa(GrandpaConsensusState {
				timestamp: Timestamp::from_nanoseconds(timestamp_nanos)
					.unwrap()
					.into_tm_time()
					.unwrap(),
				root: decoded_para_head.state_root.as_bytes().to_vec().into(),
			});

			return Ok((AnyClientState::Grandpa(client_state), consensus_state))
		}
	}

	pub async fn submit_create_client_msg(&self, msg: pallet_ibc::Any) -> Result<ClientId, Error> {
		let call = api::tx().ibc().deliver(vec![api::runtime_types::pallet_ibc::Any {
			type_url: msg.type_url,
			value: msg.value,
		}]);
		let (ext_hash, block_hash) = self.submit_call(call, true).await?;

		// Query newly created client Id
		let para_client = unsafe { unsafe_cast_to_jsonrpsee_client(&self.para_client) };
		let identified_client_state: IdentifiedClientState =
			IbcApiClient::<u32, sp_core::H256>::query_newly_created_client(
				&*para_client,
				block_hash.unwrap().into(),
				ext_hash.into(),
			)
			.await?;

		let client_id = ClientId::from_str(&identified_client_state.client_id)
			.expect("Should have a valid client id");
		Ok(client_id)
	}

	pub async fn transfer_tokens(
		&self,
		params: TransferParams<AccountId>,
		asset_id: u128,
		amount: u128,
	) -> Result<(), Error> {
		use pallet_ibc::{MultiAddress, Timeout};
		let params = api::runtime_types::pallet_ibc::TransferParams {
			to: match params.to {
				MultiAddress::Id(id) => {
					let id: [u8; 32] = id.into();
					api::runtime_types::pallet_ibc::MultiAddress::Id(id.into())
				},
				MultiAddress::Raw(raw) => api::runtime_types::pallet_ibc::MultiAddress::Raw(raw),
			},

			source_channel: params.source_channel,

			timeout: match params.timeout {
				Timeout::Offset { timestamp, height } =>
					api::runtime_types::pallet_ibc::Timeout::Offset { timestamp, height },
				Timeout::Absolute { timestamp, height } =>
					api::runtime_types::pallet_ibc::Timeout::Absolute { timestamp, height },
			},
		};
		// Submit extrinsic to parachain node
		let call = api::tx().ibc().transfer(
			params,
			api::runtime_types::primitives::currency::CurrencyId(asset_id),
			amount.into(),
		);

		self.submit_call(call, true).await?;

		Ok(())
	}

	pub async fn submit_call<C: TxPayload>(
		&self,
		call: C,
		wait_for_in_block: bool,
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
				return Err(Error::Custom("Failed to submit extrinsic after 5 tries".to_string()))
			}

			let tx_params = <SubstrateExtrinsicParamsBuilder<T>>::new()
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
			if res.is_ok() {
				break res.unwrap()
			}
			count += 1;
		};

		if wait_for_in_block {
			let ext_hash = progress.extrinsic_hash();
			let tx_in_block = progress.wait_for_in_block().await?;
			return Ok((ext_hash, Some(tx_in_block.block_hash())))
		}

		Ok((progress.extrinsic_hash(), None))
	}

	pub async fn submit_sudo_call(
		&self,
		call: api::runtime_types::dali_runtime::Call,
	) -> Result<(), Error> {
		let signer = ExtrinsicSigner::<T, Self>::new(
			self.key_store.clone(),
			self.key_type_id.clone(),
			self.public_key.clone(),
		);

		let ext = api::tx().sudo().sudo(call);
		// Submit extrinsic to parachain node

		let tx_params = SubstrateExtrinsicParamsBuilder::new()
			.tip(AssetTip::new(100_000))
			.era(Era::Immortal, self.para_client.genesis_hash());

		let _progress = self
			.para_client
			.tx()
			.sign_and_submit_then_watch(&ext, &signer, tx_params.into())
			.await?
			.wait_for_in_block()
			.await?;

		Ok(())
	}

	pub async fn set_pallet_params(
		&self,
		receive_enabled: bool,
		send_enabled: bool,
	) -> Result<(), Error> {
		let params = api::runtime_types::pallet_ibc::PalletParams { receive_enabled, send_enabled };

		let call = api::runtime_types::dali_runtime::Call::Ibc(
			api::runtime_types::pallet_ibc::pallet::Call::set_params { params },
		);

		self.submit_sudo_call(call).await?;

		Ok(())
	}
}
