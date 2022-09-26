use std::{str::FromStr, time::Duration};

pub mod calls;
pub mod chain;
pub mod error;
pub mod key_provider;
pub(crate) mod parachain;
pub(crate) mod polkadot;
pub mod provider;
pub mod signer;
pub mod utils;

#[cfg(feature = "testing")]
pub mod test_provider;

use codec::{Codec, Decode};
use error::Error;

use calls::{sudo_call, Sudo, Transfer, TransferParams};
use common::AccountId;
use signer::ExtrinsicSigner;

#[cfg(feature = "beefy")]
use beefy_light_client_primitives::{ClientState, MmrUpdateProof, PartialMmrLeaf};
#[cfg(feature = "beefy")]
use beefy_prover::{
	helpers::{fetch_timestamp_extrinsic_with_proof, TimeStampExtWithProof},
	ClientWrapper,
};
use ibc::{
	core::ics24_host::identifier::{ChannelId, ClientId, PortId},
	events::IbcEvent,
};
use ibc_proto::ibc::core::client::v1::IdentifiedClientState;

#[cfg(feature = "beefy")]
use ics11_beefy::{
	client_message::ParachainHeader, client_state::ClientState as BeefyClientState,
	consensus_state::ConsensusState as BeefyConsensusState,
};
use pallet_ibc::light_clients::{AnyClientState, AnyConsensusState, HostFunctionsManager};
#[cfg(feature = "beefy")]
use pallet_mmr_primitives::BatchProof;
#[cfg(feature = "beefy")]
use sp_core::H256;
use sp_keystore::SyncCryptoStorePtr;
use sp_runtime::{
	generic::Era,
	traits::{IdentifyAccount, Verify},
	KeyTypeId, MultiSignature,
};
use ss58_registry::Ss58AddressFormat;
use subxt::{
	extrinsic::PlainTip,
	rpc::{rpc_params, ClientT},
	sp_runtime::{traits::Header as HeaderT, MultiSigner},
	PolkadotExtrinsicParams, SubmittableExtrinsic,
};
use tokio::sync::broadcast::{self, Sender};

use crate::{
	calls::{deliver, Deliver},
	utils::fetch_max_extrinsic_weight,
};
use primitives::KeyProvider;

use grandpa_light_client_primitives::ParachainHeadersWithFinalityProof;
use grandpa_prover::GrandpaProver;
use ibc::timestamp::Timestamp;
use ics10_grandpa::{
	client_state::ClientState as GrandpaClientState,
	consensus_state::ConsensusState as GrandpaConsensusState,
};
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
	pub relay_client: subxt::Client<T>,
	/// Parachain rpc client
	pub para_client: subxt::Client<T>,
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
}

/// config options for [`ParachainClient`]
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
	pub commitment_prefix: Vec<u8>,
	/// Relayer's Public on parachain key
	pub public_key: MultiSigner,
	/// Reference to keystore
	pub key_store: SyncCryptoStorePtr,
	/// used for encoding relayer address.
	pub ss58_version: u8,
	/// 4 byte Key type id
	pub key_type_id: KeyTypeId,
	/// Channels cleared for packet relay
	pub channel_whitelist: Vec<(ChannelId, PortId)>,
}

impl<T: subxt::Config + Send + Sync> ParachainClient<T>
where
	u32: From<<<T as subxt::Config>::Header as HeaderT>::Number>,
	Self: KeyProvider,
	<T::Signature as Verify>::Signer: From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	MultiSigner: From<MultiSigner>,
	<T as subxt::Config>::Address: From<<T as subxt::Config>::AccountId>,
	T::Signature: From<MultiSignature>,
{
	pub async fn new(config: ParachainClientConfig) -> Result<Self, Error> {
		let para_client = subxt::ClientBuilder::new()
			.set_url(&config.parachain_rpc_url)
			.build::<T>()
			.await?;

		let relay_client = subxt::ClientBuilder::new()
			.set_url(&config.relay_chain_rpc_url)
			.build::<T>()
			.await?;

		let (sender, _) = broadcast::channel(16);
		let max_extrinsic_weight = fetch_max_extrinsic_weight(&para_client).await?;
		Ok(Self {
			name: config.name,
			para_client,
			relay_client,
			para_id: config.para_id,
			client_id: config.client_id,
			commitment_prefix: config.commitment_prefix,
			public_key: config.public_key,
			key_store: config.key_store,
			key_type_id: config.key_type_id,
			sender,
			max_extrinsic_weight,
			ss58_version: Ss58AddressFormat::from(config.ss58_version),
			channel_whitelist: config.channel_whitelist,
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

	#[cfg(feature = "beefy")]
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

	#[cfg(feature = "beefy")]
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

	#[cfg(feature = "beefy")]
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

	#[cfg(feature = "beefy")]
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
		loop {
			let client_wrapper = ClientWrapper {
				relay_client: self.relay_client.clone(),
				para_client: self.para_client.clone(),
				beefy_activation_block,
				para_id: self.para_id,
			};

			let beefy_state = client_wrapper
				.construct_beefy_client_state(beefy_activation_block)
				.await
				.map_err(|e| {
					Error::from(format!("[construct_beefy_client_state] Failed due to {:?}", e))
				})?;

			// TODO: Move this code for consensus state construction to beefy-rs
			let api = client_wrapper
				.relay_client
				.clone()
				.to_runtime_api::<polkadot::api::RuntimeApi<T, subxt::PolkadotExtrinsicParams<_>>>(
				);
			let subxt_block_number: subxt::BlockNumber = beefy_state.latest_beefy_height.into();
			let block_hash =
				client_wrapper.relay_client.rpc().block_hash(Some(subxt_block_number)).await?;
			let head_data = api
				.storage()
				.paras()
				.heads(
					&polkadot::api::runtime_types::polkadot_parachain::primitives::Id(
						client_wrapper.para_id,
					),
					block_hash,
				)
				.await?
				.ok_or_else(|| {
					Error::Custom(format!(
						"Couldn't find header for ParaId({}) at relay block {:?}",
						client_wrapper.para_id, block_hash
					))
				})?;
			let decoded_para_head = frame_support::sp_runtime::generic::Header::<
				u32,
				frame_support::sp_runtime::traits::BlakeTwo256,
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
			let subxt_block_number: subxt::BlockNumber = block_number.into();
			let block_hash = client_wrapper
				.para_client
				.rpc()
				.block_hash(Some(subxt_block_number))
				.await
				.unwrap();

			let TimeStampExtWithProof { ext: timestamp_extrinsic, proof: extrinsic_proof } =
				fetch_timestamp_extrinsic_with_proof(&client_wrapper.para_client, block_hash)
					.await
					.unwrap();
			let parachain_header = ParachainHeader {
				parachain_header: decoded_para_head,
				partial_mmr_leaf: PartialMmrLeaf {
					version: Default::default(),
					parent_number_and_hash: Default::default(),
					beefy_next_authority_set: Default::default(),
				},
				parachain_heads_proof: vec![],
				heads_leaf_index: 0,
				heads_total_count: 0,
				extrinsic_proof,
				timestamp_extrinsic,
			};

			let consensus_state = AnyConsensusState::Beefy(
				BeefyConsensusState::from_header(parachain_header).unwrap(),
			);

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
		sp_core::H256: From<T::Hash>,
	{
		let api = self
			.relay_client
			.clone()
			.to_runtime_api::<polkadot::api::RuntimeApi<T, subxt::PolkadotExtrinsicParams<_>>>();
		let para_client_api = self
			.para_client
			.clone()
			.to_runtime_api::<polkadot::api::RuntimeApi<T, subxt::PolkadotExtrinsicParams<_>>>();
		loop {
			let current_set_id = api
				.storage()
				.grandpa()
				.current_set_id(None)
				.await
				.expect("Failed to fetch current set id");

			let current_authorities = {
				let bytes = self
					.relay_client
					.rpc()
					.client
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

			let head_data = api
				.storage()
				.paras()
				.heads(
					&polkadot::api::runtime_types::polkadot_parachain::primitives::Id(self.para_id),
					Some(latest_relay_hash),
				)
				.await?
				.ok_or_else(|| {
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
			let client_state = GrandpaClientState::<HostFunctionsManager> {
				relay_chain: Default::default(),
				current_authorities,
				current_set_id,
				latest_relay_hash: latest_relay_hash.into(),
				frozen_height: None,
				latest_para_height: block_number,
				para_id: self.para_id,
				..Default::default()
			};
			// we can't use the genesis block to construct the initial state.
			if block_number == 0 {
				continue
			}
			let subxt_block_number: subxt::BlockNumber = block_number.into();
			let block_hash =
				self.para_client.rpc().block_hash(Some(subxt_block_number)).await.unwrap();
			let unix_timestamp_millis =
				para_client_api.storage().timestamp().now(block_hash).await?;
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
		let public_key = self.public_key.clone();
		let signer = ExtrinsicSigner::<T, Self>::new(
			self.key_store.clone(),
			self.key_type_id.clone(),
			public_key,
		);

		let ext = deliver::<T, subxt::PolkadotExtrinsicParams<T>>(
			&self.para_client,
			Deliver { messages: vec![msg] },
		);
		// Submit extrinsic to parachain node

		let tx_params = subxt::PolkadotExtrinsicParamsBuilder::new()
			.tip(PlainTip::new(10_000))
			.era(Era::Immortal, *self.para_client.genesis());

		let progress = ext
			.sign_and_submit_then_watch(&signer, tx_params)
			.await?
			.wait_for_in_block()
			.await?;

		// Query newly created client Id
		let identified_client_state: IdentifiedClientState = self
			.para_client
			.rpc()
			.client
			.request(
				"ibc_queryNewlyCreatedClient",
				rpc_params!(progress.block_hash(), progress.extrinsic_hash()),
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
		let call = Transfer { params, asset_id, amount };
		// Submit extrinsic to parachain node

		self.submit_call(call, true).await?;

		Ok(())
	}

	pub async fn submit_call<C: Codec + Send + Sync + subxt::Call + Clone>(
		&self,
		call: C,
		wait_for_in_block: bool,
	) -> Result<(), Error> {
		use polkadot::api::runtime_types::sp_runtime::DispatchError;
		let signer = ExtrinsicSigner::<T, Self>::new(
			self.key_store.clone(),
			self.key_type_id.clone(),
			self.public_key.clone(),
		);

		let metadata = self.para_client.rpc().metadata().await?;
		// Check for pallet and call index existence in latest chain metadata to ensure our static
		// definitions are up to date
		let pallet = metadata
			.pallet(<C as subxt::Call>::PALLET)
			.map_err(|_| Error::PalletNotFound(<C as subxt::Call>::PALLET))?;
		pallet
			.call_index::<Deliver>()
			.map_err(|_| Error::CallNotFound(<C as subxt::Call>::FUNCTION))?;
		// Update the metadata held by the client
		let _ = self.para_client.metadata().try_write().and_then(|mut writer| {
			*writer = metadata;
			Some(writer)
		});

		// Submit extrinsic to parachain node
		let tip = 100_000u128;
		// Try extrinsic submission five times in case of failures
		let mut count = 0;
		let progress = loop {
			if count == 5 {
				return Err(Error::Custom("Failed to submit extrinsic after 5 tries".to_string()))
			}

			let ext =
				SubmittableExtrinsic::<T, PolkadotExtrinsicParams<T>, _, DispatchError, ()>::new(
					&self.para_client,
					call.clone(),
				);

			let tx_params = subxt::PolkadotExtrinsicParamsBuilder::new()
				.era(Era::Immortal, *self.para_client.genesis());

			let res = ext
				.sign_and_submit_then_watch(&signer, tx_params.tip(PlainTip::new(count * tip)))
				.await;
			if res.is_ok() {
				break res.unwrap()
			}
			count += 1;
		};

		if wait_for_in_block {
			progress.wait_for_in_block().await?;
		}

		Ok(())
	}

	pub async fn submit_sudo_call<C: Codec + Send + Sync + subxt::Call + Clone>(
		&self,
		call: C,
	) -> Result<(), Error> {
		let signer = ExtrinsicSigner::<T, Self>::new(
			self.key_store.clone(),
			self.key_type_id.clone(),
			self.public_key.clone(),
		);

		let ext = sudo_call::<T, subxt::PolkadotExtrinsicParams<T>, _>(
			&self.para_client,
			Sudo { call, client: &self.para_client },
		);
		// Submit extrinsic to parachain node

		let tx_params = subxt::PolkadotExtrinsicParamsBuilder::new()
			.tip(PlainTip::new(100_000))
			.era(Era::Immortal, *self.para_client.genesis());

		let _progress = ext
			.sign_and_submit_then_watch(&signer, tx_params)
			.await?
			.wait_for_in_block()
			.await?;

		Ok(())
	}
}
