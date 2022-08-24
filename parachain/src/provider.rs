use std::pin::Pin;

use super::polkadot;
use beefy_light_client_primitives::{ClientState, NodesUtils};
use codec::{Decode, Encode};
use futures::{Stream, TryStreamExt};
use ibc::{
	clients::ics11_beefy::header::BeefyHeader,
	core::{
		ics02_client::{
			client_consensus::AnyConsensusState,
			client_state::AnyClientState,
			header::{AnyHeader, Header as IbcHeaderT},
		},
		ics04_channel::packet::{Packet, Sequence},
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
	generic::Header,
	traits::{BlakeTwo256, Header as HeaderT, IdentifyAccount, Verify},
	MultiSigner,
};
use subxt::{rpc::RpcError, Config};

use super::{error::Error, ParachainClient};
use ibc_proto::ibc::core::channel::v1::Packet as RawPacket;
use ibc_rpc::{BlockNumberOrHash, IbcApiClient};
use primitives::{Chain, IbcProvider, KeyProvider, UpdateType};
use sp_core::H256;
use tokio_stream::wrappers::BroadcastStream;

pub type FinalityEvent = Result<String, RpcError>;

#[async_trait::async_trait]
impl<T: Config + Send + Sync> IbcProvider for ParachainClient<T>
where
	u32: From<<<T as Config>::Header as HeaderT>::Number>,
	u32: From<<T as Config>::BlockNumber>,
	Self: KeyProvider,
	<T::Signature as Verify>::Signer:
		From<<Self as KeyProvider>::Public> + IdentifyAccount<AccountId = T::AccountId>,
	MultiSigner: From<<Self as KeyProvider>::Public>,
	<T as subxt::Config>::Address: From<<T as subxt::Config>::AccountId>,
	<T as subxt::Config>::Signature: From<<Self as KeyProvider>::Signature>,
{
	type IbcEvent = Result<Vec<IbcEvent>, String>;
	type FinalityEvent = FinalityEvent;
	type Error = Error;

	async fn client_update_header<C>(
		&mut self,
		finality_event: Self::FinalityEvent,
		counterparty: &C,
	) -> Result<(AnyHeader, AnyClientState, UpdateType), Self::Error>
	where
		C: Chain,
		Self::Error: From<C::Error>,
	{
		let client_id = self.client_id();
		let latest_height = counterparty.latest_height().await?;
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
		let commitment = finality_event?;
		let recv_commitment = hex::decode(&commitment[2..])?;
		let signed_commitment: beefy_primitives::SignedCommitment<
			u32,
			beefy_primitives::crypto::Signature,
		> = codec::Decode::decode(&mut &*recv_commitment)?;

		if signed_commitment.commitment.validator_set_id < beefy_client_state.current_authorities.id
		{
			log::info!(
				"Commitment: {:#?}\nClientState: {:#?}",
				signed_commitment.commitment,
				beefy_client_state
			);
			// If validator set id of signed commitment is less than current validator set id we
			// have Then commitment is outdated and we skip it.
			println!(
				"Skipping outdated commitment \n Received signed commitmment with validator_set_id: {:?}\n Current authority set id: {:?}\n Next authority set id: {:?}\n",
				signed_commitment.commitment.validator_set_id, beefy_client_state.current_authorities.id, beefy_client_state.next_authorities.id
			);
			return Err(Error::HeaderConstruction(
				"Received an outdated beefy commitment".to_string(),
			));
		}

		// check if validator set has changed.
		// If client on counterparty has never been updated since it was created we want to update
		// it
		let update_type = match signed_commitment.commitment.validator_set_id
			== beefy_client_state.next_authorities.id
		{
			true => UpdateType::Mandatory,
			false => UpdateType::Optional,
		};

		let (parachain_headers, batch_proof) = self
			.fetch_finalized_parachain_headers_at(
				signed_commitment.commitment.block_number,
				&beefy_client_state,
			)
			.await?;
		let mmr_update =
			self.fetch_mmr_update_proof_for(signed_commitment, &beefy_client_state).await?;
		let mmr_size = NodesUtils::new(batch_proof.leaf_count).size();
		let beefy_header = BeefyHeader {
			parachain_headers,
			mmr_proofs: batch_proof.items.into_iter().map(|item| item.encode()).collect(),
			mmr_size,
			mmr_update_proof: Some(mmr_update),
		};
		let header = beefy_header.wrap_any();
		Ok((header, client_state, update_type))
	}

	async fn query_latest_ibc_events(
		&mut self,
		header: &AnyHeader,
		client_state: &AnyClientState,
	) -> Result<Vec<IbcEvent>, Self::Error> {
		let beefy_header = match header {
			AnyHeader::Beefy(header) => header,
			_ => unreachable!(),
		};

		// Get finalized parachain block numbers, but only those higher than the latest para height
		// recorded in the on chain client state, because in some cases a parachain block that was
		// already finalized in a former beefy block might still be part of the parachain headers in
		// a later beefy block, discovered this from previous logs
		let finalized_block_numbers = beefy_header
			.parachain_headers
			.iter()
			.filter_map(|header| {
				if (client_state.latest_height().revision_height as u32)
					< header.parachain_header.number.into()
				{
					Some(header.parachain_header.number)
				} else {
					None
				}
			})
			.collect::<Vec<_>>();
		log::info!(
			"Fetching events from parachain ParaId({}) for blocks {}..{}",
			self.para_id,
			finalized_block_numbers[0],
			finalized_block_numbers.last().unwrap()
		);

		let finalized_block_numbers =
			finalized_block_numbers.into_iter().map(BlockNumberOrHash::Number).collect();
		let events = self.query_events_at(finalized_block_numbers).await?;
		if self.sender.send(events.clone()).is_err() {
			log::error!("Failed to push ibc events to stream, no active receiver found");
		}
		Ok(events)
	}

	async fn host_consensus_state(&self, height: Height) -> Result<AnyConsensusState, Self::Error> {
		let consensus_state_response = IbcApiClient::<u32, H256>::query_consensus_state(
			&*self.para_client.rpc().client,
			height.revision_height as u32,
		)
		.await?;
		let consensus_state = consensus_state_response.consensus_state.ok_or_else(|| {
			Error::Custom("[host_consensus_state] Rpc returned a None value".to_string())
		})?;

		let consensus_state = AnyConsensusState::try_from(consensus_state)?;
		Ok(consensus_state)
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

	async fn query_packets(
		&self,
		_at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seqs: Vec<u64>,
	) -> Result<Vec<Packet>, Self::Error> {
		let packets: Vec<RawPacket> = IbcApiClient::<u32, H256>::query_packets(
			&*self.para_client.rpc().client,
			channel_id.to_string(),
			port_id.to_string(),
			seqs.clone(),
		)
		.await
		.map_err(|e| Error::QueryPackets {
			channel_id: channel_id.to_string(),
			port_id: port_id.to_string(),
			sequences: seqs,
			err: e.to_string(),
		})?;

		let packets = packets
			.into_iter()
			.map(|raw_packet| raw_packet.try_into())
			.collect::<Result<Vec<Packet>, _>>()?;
		Ok(packets)
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

	fn cache_send_packet_seq(&mut self, packet: Packet) {
		self.packet_cache.push(packet);
	}

	fn remove_packets(&mut self, seqs: Vec<Sequence>) {
		self.packet_cache = self
			.packet_cache
			.iter()
			.filter_map(
				|packet| if !seqs.contains(&packet.sequence) { Some(packet.clone()) } else { None },
			)
			.collect();
	}

	fn cached_packets(&self) -> &Vec<Packet> {
		&self.packet_cache
	}

	fn connection_prefix(&self) -> CommitmentPrefix {
		CommitmentPrefix::try_from(self.commitment_prefix.clone()).expect("Should not fail")
	}

	fn apply_prefix(&self, path: String) -> Vec<u8> {
		let mut key_path = vec![self.commitment_prefix.clone()];
		let path = vec![path.as_bytes().to_vec()];
		key_path.extend_from_slice(&path);
		key_path.into_iter().flatten().collect()
	}

	async fn consensus_height(&self, client_height: Height) -> Option<Height> {
		let beefy_height = client_height.revision_height as u32;
		println!("[consensus height]: client_height {:?}", client_height);

		let subxt_block_number: subxt::BlockNumber = beefy_height.into();
		let block_hash = self.relay_client.rpc().block_hash(Some(subxt_block_number)).await.ok()?;

		let api = self
			.relay_client
			.clone()
			.to_runtime_api::<polkadot::api::RuntimeApi<T, subxt::PolkadotExtrinsicParams<_>>>();

		let para_id =
			polkadot::api::runtime_types::polkadot_parachain::primitives::Id(self.para_id);

		let head_data: polkadot::api::runtime_types::polkadot_parachain::primitives::HeadData =
			api.storage().paras().heads(&para_id, block_hash).await.ok().flatten()?;
		let decoded_header = Header::<u32, BlakeTwo256>::decode(&mut &*head_data.0).ok()?;
		let height: u32 = (*decoded_header.number()).into();
		Some(Height::new(self.para_id.into(), height.into()))
	}

	fn client_id(&self) -> ClientId {
		self.client_id()
	}

	async fn latest_height(&self) -> Result<Height, Self::Error> {
		let finalized_header = self
			.para_client
			.rpc()
			.header(None)
			.await?
			.ok_or_else(|| Error::Custom("Latest height query returned None".to_string()))?;
		let latest_height = *finalized_header.number();
		Ok(Height::new(self.para_id.into(), latest_height.into()))
	}

	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = Self::IbcEvent> + Send + Sync>> {
		let stream = BroadcastStream::new(self.sender.subscribe());
		Box::pin(Box::new(stream.map_err(|err| err.to_string())))
	}

	fn client_update_status(&self) -> bool {
		*self.client_update_status.lock().unwrap()
	}

	fn set_client_update_status(&mut self, status: bool) {
		let mut temp_status = self.client_update_status.lock().unwrap();
		*temp_status = status;
	}
}
