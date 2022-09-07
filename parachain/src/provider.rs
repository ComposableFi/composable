use beefy_light_client_primitives::{ClientState, NodesUtils};
use codec::{Decode, Encode};
use itertools::Itertools;
use std::{fmt::Display, str::FromStr, time::Duration};

use ibc::{
	applications::transfer::{Amount, PrefixedCoin, PrefixedDenom},
	clients::ics11_beefy::header::{BeefyHeader, ParachainHeadersWithProof},
	core::{
		ics02_client::{
			client_state::AnyClientState,
			client_type::ClientType,
			header::{AnyHeader, Header as IbcHeaderT},
		},
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	events::IbcEvent,
	Height,
};

use ibc_proto::ibc::core::{
	channel::v1::{
		QueryChannelResponse, QueryNextSequenceReceiveResponse, QueryPacketAcknowledgementResponse,
		QueryPacketCommitmentResponse, QueryPacketReceiptResponse,
	},
	client::v1::{QueryClientStateResponse, QueryConsensusStateResponse},
	connection::v1::QueryConnectionResponse,
};
use sp_runtime::{
	traits::{Header as HeaderT, IdentifyAccount, Verify},
	MultiSignature, MultiSigner,
};
use subxt::Config;

use super::{error::Error, ParachainClient};
use ibc_rpc::{BlockNumberOrHash, IbcApiClient, PacketInfo};
use primitives::{Chain, IbcProvider, KeyProvider, UpdateType};
use sp_core::H256;

use crate::{parachain, parachain::api::runtime_types::primitives::currency::CurrencyId};
use beefy_prover::helpers::fetch_timestamp_extrinsic_with_proof;
use ibc::{core::ics02_client::client_consensus::AnyConsensusState, timestamp::Timestamp};
use ibc_proto::ibc::core::channel::v1::QueryChannelsResponse;

/// Finality event for parachains
pub type FinalityEvent =
	beefy_primitives::SignedCommitment<u32, beefy_primitives::crypto::Signature>;

#[async_trait::async_trait]
impl<T: Config + Send + Sync> IbcProvider for ParachainClient<T>
where
	u32: From<<<T as Config>::Header as HeaderT>::Number>,
	u32: From<<T as Config>::BlockNumber>,
	Self: KeyProvider,
	<T::Signature as Verify>::Signer: From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	MultiSigner: From<MultiSigner>,
	<T as subxt::Config>::Address: From<<T as subxt::Config>::AccountId>,
	T::Signature: From<MultiSignature>,
	T::BlockNumber: From<u32> + Display + Ord + sp_runtime::traits::Zero,
{
	type FinalityEvent = FinalityEvent;
	type Error = Error;

	async fn query_latest_ibc_events<C>(
		&mut self,
		signed_commitment: Self::FinalityEvent,
		counterparty: &C,
	) -> Result<(AnyHeader, Vec<IbcEvent>, UpdateType), Self::Error>
	where
		C: Chain,
		Self::Error: From<C::Error>,
	{
		let client_id = self.client_id();
		let latest_height = counterparty.latest_height_and_timestamp().await?.0;
		let response = counterparty.query_client_state(latest_height, client_id).await?;
		let client_state = response.client_state.ok_or_else(|| {
			From::from("Received an empty client state from counterparty".to_string())
		})?;
		let client_state = AnyClientState::try_from(client_state)
			.map_err(|_| From::from("Failed to decode client state".to_string()))?;
		let beefy_client_state = match &client_state {
			AnyClientState::Beefy(client_state) => ClientState {
				latest_beefy_height: client_state.latest_beefy_height,
				mmr_root_hash: client_state.mmr_root_hash,
				current_authorities: client_state.authority.clone(),
				next_authorities: client_state.next_authority_set.clone(),
				beefy_activation_block: client_state.beefy_activation_block,
			},
			c => Err(Error::ClientStateRehydration(format!(
				"Expected AnyClientState::Beefy found: {:?}",
				c
			)))?,
		};

		if signed_commitment.commitment.validator_set_id < beefy_client_state.current_authorities.id
		{
			log::info!(
				"Commitment: {:#?}\nClientState: {:#?}",
				signed_commitment.commitment,
				beefy_client_state
			);
			// If validator set id of signed commitment is less than current validator set id we
			// have Then commitment is outdated and we skip it.
			log::warn!(
				"Skipping outdated commitment \n Received signed commitmment with validator_set_id: {:?}\n Current authority set id: {:?}\n Next authority set id: {:?}\n",
				signed_commitment.commitment.validator_set_id, beefy_client_state.current_authorities.id, beefy_client_state.next_authorities.id
			);
			Err(Error::HeaderConstruction("Received an outdated beefy commitment".to_string()))?
		}

		// if validator set has changed this is a mandatory update
		// let update_type = match signed_commitment.commitment.validator_set_id ==
		// 	beefy_client_state.next_authorities.id
		// {
		// 	true => UpdateType::Mandatory,
		// 	false => UpdateType::Optional,
		// };

		let update_type = UpdateType::Mandatory;

		// fetch the new parachain headers that have been finalized
		let headers = self
			.query_finalized_parachain_headers_at(
				signed_commitment.commitment.block_number,
				&beefy_client_state,
			)
			.await?;

		log::info!(
			"Fetching events from {} for blocks {}..{}",
			self.name(),
			headers[0].number(),
			headers.last().unwrap().number()
		);

		// Get finalized parachain block numbers, but only those higher than the latest para
		// height recorded in the on-chain client state, because in some cases a parachain
		// block that was already finalized in a former beefy block might still be part of
		// the parachain headers in a later beefy block, discovered this from previous logs
		let finalized_block_numbers = headers
			.into_iter()
			.filter_map(|header| {
				if (client_state.latest_height().revision_height as u32) <
					u32::from(*header.number())
				{
					Some(header)
				} else {
					None
				}
			})
			.map(|h| BlockNumberOrHash::Number(From::from(*h.number())))
			.collect();

		// block_number => events
		let events = IbcApiClient::<u32, H256>::query_events(
			&*self.para_client.rpc().client,
			finalized_block_numbers,
		)
		.await?;
		// header number is serialized to string
		let headers_with_events = events
			.keys()
			.map(|num| str::parse::<u32>(&*num))
			.map_ok(T::BlockNumber::from)
			.collect::<Result<Vec<_>, _>>()?;
		let events: Vec<IbcEvent> = events.into_values().flatten().collect();

		// only query proofs for headers that actually have events
		let headers_with_proof = if !events.is_empty() {
			let (headers, batch_proof) = self
				.query_finalized_parachain_headers_with_proof(
					signed_commitment.commitment.block_number,
					&beefy_client_state,
					headers_with_events,
				)
				.await?;
			let mmr_size = NodesUtils::new(batch_proof.leaf_count).size();

			Some(ParachainHeadersWithProof {
				headers,
				mmr_size,
				mmr_proofs: batch_proof.items.into_iter().map(|item| item.encode()).collect(),
			})
		} else {
			None
		};

		let mmr_update =
			self.fetch_mmr_update_proof_for(signed_commitment, &beefy_client_state).await?;
		let beefy_header = BeefyHeader { mmr_update_proof: Some(mmr_update), headers_with_proof };

		for event in events.iter() {
			if self.sender.send(event.clone()).is_err() {
				log::trace!("Failed to push {event:?} to stream, no active receiver found");
				break
			}
		}

		Ok((beefy_header.wrap_any(), events, update_type))
	}

	async fn query_client_consensus(
		&self,
		at: Height,
		client_id: ClientId,
		consensus_height: Height,
	) -> Result<QueryConsensusStateResponse, Self::Error> {
		let res = IbcApiClient::<u32, H256>::query_client_consensus_state(
			&*self.para_client.rpc().client,
			Some(at.revision_height as u32),
			client_id.to_string(),
			consensus_height.revision_height,
			consensus_height.revision_number,
			false,
		)
		.await?;
		Ok(res)
	}

	async fn query_client_state(
		&self,
		at: Height,
		client_id: ClientId,
	) -> Result<QueryClientStateResponse, Self::Error> {
		let response = IbcApiClient::<u32, H256>::query_client_state(
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			client_id.to_string(),
		)
		.await?;
		Ok(response)
	}

	async fn query_connection_end(
		&self,
		at: Height,
		connection_id: ConnectionId,
	) -> Result<QueryConnectionResponse, Self::Error> {
		let response = IbcApiClient::<u32, H256>::query_connection(
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			connection_id.to_string(),
		)
		.await?;
		Ok(response)
	}

	async fn query_channel_end(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<QueryChannelResponse, Self::Error> {
		let response = IbcApiClient::<u32, H256>::query_channel(
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
		)
		.await?;
		Ok(response)
	}

	async fn query_proof(&self, at: Height, keys: Vec<Vec<u8>>) -> Result<Vec<u8>, Self::Error> {
		let proof = IbcApiClient::<u32, H256>::query_proof(
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			keys,
		)
		.await?;

		Ok(proof.proof)
	}

	async fn query_packet_commitment(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse, Self::Error> {
		let res = IbcApiClient::<u32, H256>::query_packet_commitment(
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
			seq,
		)
		.await?;
		Ok(res)
	}

	async fn query_packet_acknowledgement(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse, Self::Error> {
		let res = IbcApiClient::<u32, H256>::query_packet_acknowledgement(
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
			seq,
		)
		.await?;
		Ok(res)
	}

	async fn query_next_sequence_recv(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<QueryNextSequenceReceiveResponse, Self::Error> {
		let res = IbcApiClient::<u32, H256>::query_next_seq_recv(
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
		)
		.await?;
		Ok(res)
	}

	async fn query_packet_receipt(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketReceiptResponse, Self::Error> {
		let res = IbcApiClient::<u32, H256>::query_packet_receipt(
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
			seq,
		)
		.await?;
		Ok(res)
	}

	async fn query_packet_commitments(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error> {
		let res = IbcApiClient::<u32, H256>::query_packet_commitments(
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
		)
		.await?;
		Ok(res.commitments.into_iter().map(|packet_state| packet_state.sequence).collect())
	}

	async fn query_packet_acknowledgements(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error> {
		let res = IbcApiClient::<u32, H256>::query_packet_acknowledgements(
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
		)
		.await?;
		Ok(res
			.acknowledgements
			.into_iter()
			.map(|packet_state| packet_state.sequence)
			.collect())
	}

	async fn query_unreceived_acknowledgements(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		let res = IbcApiClient::<u32, H256>::query_unreceived_acknowledgements(
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
			seqs,
		)
		.await?;
		Ok(res)
	}

	async fn query_unreceived_packets(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		let res = IbcApiClient::<u32, H256>::query_unreceived_packets(
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
			seqs,
		)
		.await?;
		Ok(res)
	}

	async fn channel_whitelist(&self) -> Result<Vec<(ChannelId, PortId)>, Self::Error> {
		// Use all available channel on chain for now, better strategy later
		let response =
			IbcApiClient::<u32, H256>::query_channels(&*self.para_client.rpc().client).await?;
		response
			.channels
			.into_iter()
			.map(|identified_chan| {
				Ok((
					ChannelId::from_str(&identified_chan.channel_id)
						.expect("Failed to convert invalid string to channel id"),
					PortId::from_str(&identified_chan.port_id)
						.expect("Failed to convert invalid string to port id"),
				))
			})
			.collect::<Result<Vec<_>, _>>()
	}

	async fn query_connection_channels(
		&self,
		at: Height,
		connection_id: &ConnectionId,
	) -> Result<QueryChannelsResponse, Self::Error> {
		let response = IbcApiClient::<u32, H256>::query_connection_channels(
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			connection_id.to_string(),
		)
		.await?;
		Ok(response)
	}

	async fn latest_height_and_timestamp(&self) -> Result<(Height, Timestamp), Self::Error> {
		let finalized_header = self
			.para_client
			.rpc()
			.header(None)
			.await?
			.ok_or_else(|| Error::Custom("Latest height query returned None".to_string()))?;
		let latest_height = *finalized_header.number();
		let height = Height::new(self.para_id.into(), latest_height.into());

		let api = self
			.para_client
			.clone()
			.to_runtime_api::<parachain::api::RuntimeApi<T, subxt::PolkadotExtrinsicParams<_>>>();
		let block_hash = finalized_header.hash();
		let unix_timestamp_millis = api.storage().timestamp().now(Some(block_hash)).await?;
		let timestamp_nanos = Duration::from_millis(unix_timestamp_millis).as_nanos() as u64;

		Ok((height, Timestamp::from_nanoseconds(timestamp_nanos)?))
	}

	async fn query_recv_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error> {
		let response = IbcApiClient::<u32, H256>::query_recv_packets(
			&*self.para_client.rpc().client,
			channel_id.to_string(),
			port_id.to_string(),
			seqs,
		)
		.await?;
		Ok(response)
	}

	async fn query_send_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error> {
		let response = IbcApiClient::<u32, H256>::query_send_packets(
			&*self.para_client.rpc().client,
			channel_id.to_string(),
			port_id.to_string(),
			seqs,
		)
		.await?;
		Ok(response)
	}

	fn expected_block_time(&self) -> Duration {
		// Parachains have an expected block time of 12 seconds
		Duration::from_secs(12)
	}

	async fn query_client_update_time_and_height(
		&self,
		client_id: ClientId,
		client_height: Height,
	) -> Result<(Height, Timestamp), Self::Error> {
		let response = IbcApiClient::<u32, H256>::query_client_update_time_and_height(
			&*self.para_client.rpc().client,
			client_id.to_string(),
			client_height.revision_number,
			client_height.revision_height,
		)
		.await?;
		Ok((
			response.height.into(),
			Timestamp::from_nanoseconds(response.timestamp)
				.map_err(|_| Error::Custom("Received invalid timestamp".to_string()))?,
		))
	}

	async fn query_host_consensus_state_proof(
		&self,
		height: Height,
	) -> Result<Option<Vec<u8>>, Self::Error> {
		let hash = self.para_client.rpc().block_hash(Some(height.revision_height.into())).await?;
		let header = self
			.para_client
			.rpc()
			.header(hash)
			.await?
			.ok_or_else(|| Error::Custom("Latest height query returned None".to_string()))?;
		let extrinsic_with_proof =
			fetch_timestamp_extrinsic_with_proof(&self.para_client, Some(header.hash()))
				.await
				.map_err(Error::BeefyProver)?;

		// lol should probably export this from pallet-ibc
		#[derive(Encode, Decode)]
		struct HostConsensusProof {
			header: Vec<u8>,
			extrinsic: Vec<u8>,
			extrinsic_proof: Vec<Vec<u8>>,
		}

		let host_consensus_proof = HostConsensusProof {
			header: header.encode(),
			extrinsic: extrinsic_with_proof.ext,
			extrinsic_proof: extrinsic_with_proof.proof,
		};
		Ok(Some(host_consensus_proof.encode()))
	}

	async fn query_ibc_balance(&self) -> Result<Vec<PrefixedCoin>, Self::Error> {
		let api = self
			.para_client
			.clone()
			.to_runtime_api::<parachain::api::RuntimeApi<T, subxt::PolkadotExtrinsicParams<_>>>();

		let account = self.public_key.clone().into_account();
		let balance = api.storage().tokens().accounts(&account, &CurrencyId(1), None).await?;

		dbg!(&balance);

		Ok(vec![PrefixedCoin {
			denom: PrefixedDenom::from_str("PICA")?,
			amount: Amount::from_str(&format!("{}", balance.free))?,
		}])
	}

	fn connection_prefix(&self) -> CommitmentPrefix {
		CommitmentPrefix::try_from(self.commitment_prefix.clone()).expect("Should not fail")
	}

	fn client_id(&self) -> ClientId {
		self.client_id()
	}

	fn client_type(&self) -> ClientType {
		ClientType::Beefy
	}
}
