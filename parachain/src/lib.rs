use std::{
	collections::HashMap,
	str::FromStr,
	sync::{Arc, Mutex},
};

pub mod calls;
pub mod chain;
pub mod error;
pub mod key_provider;
pub(crate) mod polkadot;
pub mod provider;
pub mod signer;
pub mod utils;

use codec::{Codec, Decode};
use error::Error;

use calls::{ibc_transfer, sudo_call, DeliverPermissioned, RawAny, Sudo, Transfer, TransferParams};
use common::AccountId;
use ibc_rpc::{BlockNumberOrHash, IbcApiClient};
use signer::ExtrinsicSigner;

use beefy_light_client_primitives::{ClientState, MmrUpdateProof, PartialMmrLeaf};
use beefy_prover::{
	helpers::{fetch_timestamp_extrinsic_with_proof, TimeStampExtWithProof},
	ClientWrapper,
};
use ibc::{
	clients::ics11_beefy::{
		client_state::ClientState as BeefyClientState, consensus_state::ConsensusState,
		header::ParachainHeader,
	},
	core::{
		ics02_client::{
			client_consensus::{AnyConsensusState, ConsensusState as ConsensusStateT},
			client_state::{AnyClientState, ClientState as ClientStateT},
		},
		ics04_channel::packet::Packet,
		ics24_host::identifier::{ChainId, ClientId},
	},
	events::IbcEvent,
};
use ibc_proto::{google::protobuf::Any, ibc::core::client::v1::IdentifiedClientState};
use pallet_mmr_primitives::BatchProof;
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
};
use tokio::sync::broadcast::{self, Sender};

use primitives::KeyProvider;

#[derive(Clone)]
/// Implements the [`crate::Chain`] trait for parachains.
/// This is responsible for:
/// 1. Tracking a parachain light client on a counter-party chain, advancing this light
/// client state  as new finality proofs are observed.
/// 2. Submiting new IBC messages to this parachain.
pub struct ParachainClient<T: subxt::Config> {
	/// Relay chain rpc client
	pub relay_client: subxt::Client<T>,
	/// Parachain rpc client
	pub para_client: subxt::Client<T>,
	/// Parachain Id
	pub para_id: u32,
	/// Light client id on counterparty chain
	pub client_id: Option<ClientId>,
	/// Parachain's latest finalized height
	pub latest_para_height: Option<u32>,
	/// Commitment prefix
	pub commitment_prefix: Vec<u8>,
	/// Sent packet sequence cache
	pub packet_cache: Vec<Packet>,
	/// Public key for relayer on chain
	pub public_key: MultiSigner,
	/// Reference to keystore
	pub key_store: SyncCryptoStorePtr,
	/// Key type Id
	pub key_type_id: KeyTypeId,
	/// used for encoding relayer address.
	pub ss58_version: Ss58AddressFormat,
	/// ibc event stream sender
	pub sender: Sender<Vec<IbcEvent>>,
	/// Client update status
	pub client_update_status: Arc<Mutex<bool>>,
}

/// config options for [`ParachainClient`]
pub struct ParachainClientConfig {
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
}

impl<T: subxt::Config + Send + Sync> ParachainClient<T>
where
	u32: From<<<T as subxt::Config>::Header as HeaderT>::Number>,
	Self: KeyProvider,
	<T::Signature as Verify>::Signer: From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	MultiSigner: From<MultiSigner>,
	<T as subxt::Config>::Address: From<<T as subxt::Config>::AccountId>,
	<T as subxt::Config>::Signature: From<MultiSignature>,
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

		Ok(Self {
			para_client,
			relay_client,
			para_id: config.para_id,
			client_id: config.client_id,
			// The following should be initialized before main relayer loop starts.
			latest_para_height: None,
			commitment_prefix: config.commitment_prefix,
			packet_cache: vec![],
			public_key: config.public_key,
			key_store: config.key_store,
			key_type_id: config.key_type_id,
			sender,
			ss58_version: Ss58AddressFormat::from(config.ss58_version),
			client_update_status: Arc::new(Mutex::new(false)),
		})
	}

	pub async fn fetch_finalized_parachain_headers_at(
		&self,
		commitment_block_number: u32,
		client_state: &ClientState,
	) -> Result<(Vec<ParachainHeader>, BatchProof<H256>), Error> {
		let client_wrapper = ClientWrapper {
			relay_client: self.relay_client.clone(),
			para_client: self.para_client.clone(),
			beefy_activation_block: client_state.beefy_activation_block,
			para_id: self.para_id,
		};

		let (parachain_headers, batch_proof) = client_wrapper
			.fetch_finalized_parachain_headers_at(
				commitment_block_number,
				client_state.latest_beefy_height,
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

	pub fn latest_para_height(&self) -> u32 {
		*(self
			.latest_para_height
			.as_ref()
			.expect("Latest parachain height should be defined"))
	}

	pub fn set_latest_para_height(&mut self, height: u32) {
		self.latest_para_height = Some(height)
	}

	pub fn set_client_id(&mut self, client_id: ClientId) {
		self.client_id = Some(client_id)
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
	{
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

			let client_state = BeefyClientState {
				chain_id: ChainId::new("relay-chain".to_string(), 0),
				relay_chain: Default::default(),
				mmr_root_hash: beefy_state.mmr_root_hash,
				latest_beefy_height: beefy_state.latest_beefy_height,
				frozen_height: None,
				beefy_activation_block: beefy_state.beefy_activation_block,
				latest_para_height: 0,
				para_id: self.para_id,
				authority: beefy_state.current_authorities,
				next_authority_set: beefy_state.next_authorities,
			};

			// TODO: Move this code for consensus state construction to beefy-rs
			let api = client_wrapper
				.relay_client
				.clone()
				.to_runtime_api::<polkadot::api::RuntimeApi<T, subxt::PolkadotExtrinsicParams<_>>>(
				);
			let subxt_block_number: subxt::BlockNumber = client_state.latest_beefy_height.into();
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

			let consensus_state = ConsensusState::from_header(parachain_header).unwrap().wrap_any();

			return Ok((client_state.wrap_any(), consensus_state))
		}
	}

	pub async fn submit_create_client_msg(&self, msg: Any) -> Result<ClientId, Error> {
		let public_key = self.public_key.clone();
		let signer = ExtrinsicSigner::<T, Self>::new(
			self.key_store.clone(),
			self.key_type_id.clone(),
			public_key,
		);

		let ext = sudo_call::<T, subxt::PolkadotExtrinsicParams<T>, _>(
			&self.para_client,
			Sudo {
				client: &self.para_client,
				call: DeliverPermissioned {
					messages: vec![RawAny {
						type_url: msg.type_url.as_bytes().to_vec(),
						value: msg.value,
					}],
				},
			},
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
		let signer = ExtrinsicSigner::<T, Self>::new(
			self.key_store.clone(),
			self.key_type_id.clone(),
			self.public_key.clone(),
		);

		let ext = ibc_transfer::<T, subxt::PolkadotExtrinsicParams<T>>(
			&self.para_client,
			Transfer { params, asset_id, amount },
		);
		// Submit extrinsic to parachain node

		let tx_params = subxt::PolkadotExtrinsicParamsBuilder::new()
			.tip(PlainTip::new(100_000))
			.era(Era::Immortal, *self.para_client.genesis());

		let mut _progress = ext
			.sign_and_submit_then_watch(&signer, tx_params)
			.await?
			.wait_for_in_block()
			.await?;

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

	/// Get all ibc events deposited in finalized blocks
	pub async fn query_events_at(
		&self,
		block_numbers: Vec<BlockNumberOrHash<H256>>,
	) -> Result<Vec<IbcEvent>, Error> {
		let events: HashMap<String, Vec<IbcEvent>> =
			IbcApiClient::<u32, H256>::query_events(&*self.para_client.rpc().client, block_numbers)
				.await?;

		Ok(events.into_values().flatten().collect())
	}
}
