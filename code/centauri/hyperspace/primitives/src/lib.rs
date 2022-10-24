#![allow(clippy::all)]

use std::{pin::Pin, str::FromStr, time::Duration};

use futures::Stream;
use ibc_proto::{
	google::protobuf::Any,
	ibc::core::{
		channel::v1::{
			QueryChannelResponse, QueryNextSequenceReceiveResponse,
			QueryPacketAcknowledgementResponse, QueryPacketCommitmentResponse,
			QueryPacketReceiptResponse,
		},
		client::v1::{QueryClientStateResponse, QueryConsensusStateResponse},
		connection::v1::QueryConnectionResponse,
	},
};
use subxt::ext::sp_core;

use crate::error::Error;
#[cfg(feature = "testing")]
use ibc::applications::transfer::msgs::transfer::MsgTransfer;
use ibc::{
	applications::transfer::PrefixedCoin,
	core::{
		ics02_client::{
			client_consensus::ConsensusState as ConsensusStateT,
			client_state::{ClientState as ClientStateT, ClientType},
		},
		ics04_channel::{
			channel::{ChannelEnd, Order},
			context::calculate_block_delay,
			packet::Packet,
		},
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	events::IbcEvent,
	signer::Signer,
	timestamp::Timestamp,
	Height,
};
use ibc_proto::ibc::core::{
	channel::v1::QueryChannelsResponse, connection::v1::IdentifiedConnection,
};
use ibc_rpc::PacketInfo;
use pallet_ibc::light_clients::{AnyClientState, AnyConsensusState};

pub mod error;
pub mod mock;
pub mod utils;

pub enum UpdateMessage {
	Single(Any),
	Batch(Vec<Any>),
}

pub enum UpdateType {
	// contains an authority set change.
	Mandatory,
	// doesn't contain an authority set change
	Optional,
}

impl UpdateType {
	pub fn is_optional(&self) -> bool {
		match self {
			UpdateType::Mandatory => false,
			UpdateType::Optional => true,
		}
	}
}

pub fn apply_prefix(mut commitment_prefix: Vec<u8>, path: String) -> Vec<u8> {
	let path = path.as_bytes().to_vec();
	commitment_prefix.extend_from_slice(&path);
	commitment_prefix
}

/// Provides an interface for accessing new events and Ibc data on the chain which must be
/// relayed to the counterparty chain.
#[async_trait::async_trait]
pub trait IbcProvider {
	/// Finality event type, passed on to [`Chain::query_latest_ibc_events`]
	type FinalityEvent;

	/// Error type, just needs to implement standard error trait.
	type Error: std::error::Error + From<String> + Send + Sync + 'static;

	/// Query the latest ibc events finalized by the recent finality event. Use the counterparty
	/// [`Chain`] to query the on-chain [`ClientState`] so you can scan for new events in between
	/// the client state and the new finality event.
	async fn query_latest_ibc_events<T>(
		&mut self,
		finality_event: Self::FinalityEvent,
		counterparty: &T,
	) -> Result<(Any, Vec<IbcEvent>, UpdateType), anyhow::Error>
	where
		T: Chain;

	/// Return a stream that yields when new [`IbcEvents`] are parsed from a finality notification
	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent>>>;

	/// Query client consensus state with proof
	/// return the consensus height for the client along with the response
	async fn query_client_consensus(
		&self,
		at: Height,
		client_id: ClientId,
		consensus_height: Height,
	) -> Result<QueryConsensusStateResponse, Self::Error>;

	/// Query client state with proof
	async fn query_client_state(
		&self,
		at: Height,
		client_id: ClientId,
	) -> Result<QueryClientStateResponse, Self::Error>;

	/// Query connection end with proof
	async fn query_connection_end(
		&self,
		at: Height,
		connection_id: ConnectionId,
	) -> Result<QueryConnectionResponse, Self::Error>;

	/// Query channel end with proof
	async fn query_channel_end(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<QueryChannelResponse, Self::Error>;

	/// Query proof for provided key path
	async fn query_proof(&self, at: Height, keys: Vec<Vec<u8>>) -> Result<Vec<u8>, Self::Error>;

	/// Query packet commitment with proof
	async fn query_packet_commitment(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse, Self::Error>;

	/// Query packet acknowledgement commitment with proof
	async fn query_packet_acknowledgement(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse, Self::Error>;

	/// Query next sequence to be received
	async fn query_next_sequence_recv(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<QueryNextSequenceReceiveResponse, Self::Error>;

	/// Query packet receipt
	async fn query_packet_receipt(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketReceiptResponse, Self::Error>;

	/// Return latest finalized height and timestamp
	async fn latest_height_and_timestamp(&self) -> Result<(Height, Timestamp), Self::Error>;

	async fn query_packet_commitments(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error>;

	async fn query_packet_acknowledgements(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error>;

	/// Given a list of counterparty packet commitments, the querier checks if the packet
	/// has already been received by checking if a receipt exists on this
	/// chain for the packet sequence. All packets that haven't been received yet
	/// are returned in the response
	/// Usage: To use this method correctly, first query all packet commitments on
	/// the sending chain using the query_packet_commitments method.
	/// and send the request to this Query/UnreceivedPackets on the **receiving**
	/// chain. This method should then return the list of packet sequences that
	/// are yet to be received on the receiving chain.
	/// NOTE: WORKS ONLY FOR UNORDERED CHANNELS
	async fn query_unreceived_packets(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error>;

	/// Given a list of packet acknowledgements sequences from the sink chain
	/// return a list of acknowledgement sequences that have not been received on the source chain
	async fn query_unreceived_acknowledgements(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error>;

	/// Channel whitelist
	fn channel_whitelist(&self) -> Vec<(ChannelId, PortId)>;

	/// Query all channels for a connection
	async fn query_connection_channels(
		&self,
		at: Height,
		connection_id: &ConnectionId,
	) -> Result<QueryChannelsResponse, Self::Error>;

	/// Query send packets
	async fn query_send_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error>;

	/// Query recieved packets
	async fn query_recv_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error>;

	/// Return the expected block time for this chain
	fn expected_block_time(&self) -> Duration;

	/// Query the time and height at which this client was updated on this chain for the given
	/// client height
	async fn query_client_update_time_and_height(
		&self,
		client_id: ClientId,
		client_height: Height,
	) -> Result<(Height, Timestamp), Self::Error>;

	/// Return a proof for the host consensus state at the given height to be included in the
	/// consensus state proof.
	async fn query_host_consensus_state_proof(
		&self,
		height: Height,
	) -> Result<Option<Vec<u8>>, Self::Error>;

	/// Should return the list of ibc denoms available to this account to spend.
	async fn query_ibc_balance(&self) -> Result<Vec<PrefixedCoin>, Self::Error>;

	/// Return the chain connection prefix
	fn connection_prefix(&self) -> CommitmentPrefix;

	/// Return the host chain's light client id on counterparty chain
	fn client_id(&self) -> ClientId;

	/// Return the connection id on this chain
	fn connection_id(&self) -> ConnectionId;

	/// Returns the client type of this chain.
	fn client_type(&self) -> ClientType;

	/// Should return timestamp in nanoseconds of chain at a given block height
	async fn query_timestamp_at(&self, block_number: u64) -> Result<u64, Self::Error>;

	/// Should return a list of all clients on the chain
	async fn query_clients(&self) -> Result<Vec<ClientId>, Self::Error>;

	/// Should return a list of all clients on the chain
	async fn query_channels(&self) -> Result<Vec<(ChannelId, PortId)>, Self::Error>;

	/// Query all connection states for associated client
	async fn query_connection_using_client(
		&self,
		height: u32,
		client_id: String,
	) -> Result<Vec<IdentifiedConnection>, Self::Error>;

	/// Returns a boolean value that determines if the light client should receive a mandatory
	/// update
	fn is_update_required(
		&self,
		latest_height: u64,
		latest_client_height_on_counterparty: u64,
	) -> bool;

	/// This should return a subjectively chosen client and consensus state for this chain.
	async fn initialize_client_state(
		&self,
	) -> Result<(AnyClientState, AnyConsensusState), Self::Error>;

	/// Should find client id that was created in this transaction
	async fn query_client_id_from_tx_hash(
		&self,
		tx_hash: sp_core::H256,
		block_hash: Option<sp_core::H256>,
	) -> Result<ClientId, Self::Error>;
}

/// Provides an interface that allows us run the hyperspace-testsuite
/// with [`Chain`] implementations.
#[cfg(feature = "testing")]
#[async_trait::async_trait]
pub trait TestProvider: Chain + Clone + 'static {
	/// Initiate an ibc transfer on chain.
	async fn send_transfer(&self, params: MsgTransfer<PrefixedCoin>) -> Result<(), Self::Error>;

	/// Initiate a ping on chain
	async fn send_ping(
		&self,
		channel_id: ChannelId,
		timeout: pallet_ibc::Timeout,
	) -> Result<(), Self::Error>;

	/// Returns a stream that yields chain Block number and hash
	async fn subscribe_blocks(&self) -> Pin<Box<dyn Stream<Item = u64> + Send + Sync>>;

	/// Set the channel whitelist for the relayer task.
	fn set_channel_whitelist(&mut self, channel_whitelist: Vec<(ChannelId, PortId)>);
}

/// Provides an interface for managing key management for signing.
pub trait KeyProvider {
	/// Should return the relayer's account id on the host chain as a string in the expected format
	/// Could be a hexadecimal, bech32 or ss58 string, any format the chain supports
	fn account_id(&self) -> Signer;
}

/// Provides an interface for the chain to the relayer core for submitting IbcEvents as well as
/// finality notifications
#[async_trait::async_trait]
pub trait Chain: IbcProvider + KeyProvider + Send + Sync {
	/// Name of this chain, used in logs.
	fn name(&self) -> &str;

	/// Should return a nuerical value for the max weight of transactions allowed in a block.
	fn block_max_weight(&self) -> u64;

	/// Should return an estimate of the weight of a batch of messages.
	async fn estimate_weight(&self, msg: Vec<Any>) -> Result<u64, Self::Error>;

	/// Return a stream that yields when new [`IbcEvents`] are ready to be queried.
	async fn finality_notifications(
		&self,
	) -> Pin<Box<dyn Stream<Item = Self::FinalityEvent> + Send + Sync>>;

	/// This should be used to submit new messages [`Vec<Any>`] from a counterparty chain to this
	/// chain.
	/// Should return a tuple of transaction hash and optionally block hash where the transaction
	/// was executed
	async fn submit(
		&self,
		messages: Vec<Any>,
	) -> Result<(sp_core::H256, Option<sp_core::H256>), Self::Error>;
}

/// Returns undelivered packet sequences that have been sent out from
/// the `source` chain to the `sink` chain
/// works for both ordered and unordered channels
pub async fn query_undelivered_sequences(
	source_height: Height,
	sink_height: Height,
	channel_id: ChannelId,
	port_id: PortId,
	source: &impl Chain,
	sink: &impl Chain,
) -> Result<Vec<u64>, anyhow::Error> {
	let channel_response =
		source.query_channel_end(source_height, channel_id, port_id.clone()).await?;
	let channel_end = ChannelEnd::try_from(
		channel_response
			.channel
			.ok_or_else(|| Error::Custom("ChannelEnd not could not be decoded".to_string()))?,
	)
	.map_err(|e| Error::Custom(e.to_string()))?;
	// First we fetch all packet commitments from source
	let seqs = source
		.query_packet_commitments(source_height, channel_id, port_id.clone())
		.await?;
	let counterparty_channel_id = channel_end
		.counterparty()
		.channel_id
		.ok_or_else(|| Error::Custom("Expected counterparty channel id".to_string()))?;
	let counterparty_port_id = channel_end.counterparty().port_id.clone();

	let undelivered_sequences = if channel_end.ordering == Order::Unordered {
		sink.query_unreceived_packets(
			sink_height,
			counterparty_channel_id,
			counterparty_port_id.clone(),
			seqs,
		)
		.await?
	} else {
		let next_seq_recv = sink
			.query_next_sequence_recv(sink_height, &counterparty_port_id, &counterparty_channel_id)
			.await?
			.next_sequence_receive;
		seqs.into_iter().filter(|seq| *seq > next_seq_recv).collect()
	};

	Ok(undelivered_sequences)
}

/// Queries the `source` chain for packet acknowledgements that have not been seen by the `sink`
/// chain.
pub async fn query_undelivered_acks(
	source_height: Height,
	sink_height: Height,
	channel_id: ChannelId,
	port_id: PortId,
	source: &impl Chain,
	sink: &impl Chain,
) -> Result<Vec<u64>, anyhow::Error> {
	let channel_response =
		source.query_channel_end(source_height, channel_id, port_id.clone()).await?;
	let channel_end = ChannelEnd::try_from(
		channel_response
			.channel
			.ok_or_else(|| Error::Custom("ChannelEnd not could not be decoded".to_string()))?,
	)
	.map_err(|e| Error::Custom(e.to_string()))?;
	// First we fetch all packet acknowledgements from source
	let seqs = source
		.query_packet_acknowledgements(source_height, channel_id, port_id.clone())
		.await?;
	let counterparty_channel_id = channel_end
		.counterparty()
		.channel_id
		.ok_or_else(|| Error::Custom("Expected counterparty channel id".to_string()))?;
	let counterparty_port_id = channel_end.counterparty().port_id.clone();

	let undelivered_acks = sink
		.query_unreceived_acknowledgements(
			sink_height,
			counterparty_channel_id,
			counterparty_port_id.clone(),
			seqs,
		)
		.await?;

	Ok(undelivered_acks)
}

pub fn packet_info_to_packet(packet_info: &PacketInfo) -> Packet {
	Packet {
		sequence: packet_info.sequence.into(),
		source_port: PortId::from_str(&packet_info.source_port).expect("Port should be valid"),
		source_channel: ChannelId::from_str(&packet_info.source_channel)
			.expect("Channel should be valid"),
		destination_port: PortId::from_str(&packet_info.destination_port)
			.expect("Port should be valid"),
		destination_channel: ChannelId::from_str(&packet_info.destination_channel)
			.expect("Channel should be valid"),
		data: packet_info.data.clone(),
		timeout_height: packet_info.timeout_height.clone().into(),
		timeout_timestamp: Timestamp::from_nanoseconds(packet_info.timeout_timestamp)
			.expect("Timestamp should be valid"),
	}
}

/// Should return the first client height with a latest_height and consensus state timestamp that
/// is equal to or greater than the values provided
pub async fn find_suitable_proof_height_for_client(
	chain: &impl Chain,
	at: Height,
	client_id: ClientId,
	start_height: Height,
	timestamp_to_match: Option<Timestamp>,
	latest_client_height: Height,
) -> Option<Height> {
	// If searching for existence of just a height we use a pure linear search because there's no
	// valid comparison to be made and there might be missing values  for some heights
	if timestamp_to_match.is_none() {
		for height in start_height.revision_height..=latest_client_height.revision_height {
			let temp_height = Height::new(start_height.revision_number, height);
			let consensus_state =
				chain.query_client_consensus(at, client_id.clone(), temp_height).await.ok();
			if consensus_state.is_none() {
				continue
			}
			return Some(temp_height)
		}
	} else {
		let timestamp_to_match = timestamp_to_match.unwrap();
		let mut start = start_height.revision_height;
		let mut end = latest_client_height.revision_height;
		let mut last_known_valid_height = None;
		if start > end {
			return None
		}
		while end - start > 1 {
			let mid = (end + start) / 2;
			let temp_height = Height::new(start_height.revision_number, mid);
			let consensus_state =
				chain.query_client_consensus(at, client_id.clone(), temp_height).await.ok();
			if consensus_state.is_none() {
				start += 1;
				continue
			}

			let consensus_state =
				AnyConsensusState::try_from(consensus_state.unwrap().consensus_state?).ok()?;
			if consensus_state.timestamp().nanoseconds() < timestamp_to_match.nanoseconds() {
				start = mid + 1;
				continue
			} else {
				last_known_valid_height = Some(temp_height);
				end = mid;
			}
		}
		let start_height = Height::new(start_height.revision_number, start);

		let consensus_state =
			chain.query_client_consensus(at, client_id.clone(), start_height).await.ok();
		if let Some(consensus_state) = consensus_state {
			let consensus_state =
				AnyConsensusState::try_from(consensus_state.consensus_state?).ok()?;
			if consensus_state.timestamp().nanoseconds() >= timestamp_to_match.nanoseconds() {
				return Some(start_height)
			}
		}

		return last_known_valid_height
	}
	None
}

pub async fn query_maximum_height_for_timeout_proofs(
	source: &impl Chain,
	sink: &impl Chain,
) -> Option<u64> {
	let mut min_timeout_height = None;
	let (source_height, ..) = source.latest_height_and_timestamp().await.ok()?;
	let (sink_height, ..) = sink.latest_height_and_timestamp().await.ok()?;
	for (channel, port_id) in source.channel_whitelist() {
		let undelivered_sequences = query_undelivered_sequences(
			source_height,
			sink_height,
			channel,
			port_id.clone(),
			source,
			sink,
		)
		.await
		.ok()?;
		let send_packets =
			source.query_send_packets(channel, port_id, undelivered_sequences).await.ok()?;
		for send_packet in send_packets {
			let sink_client_state = source
				.query_client_state(
					Height::new(source_height.revision_number, send_packet.height),
					sink.client_id(),
				)
				.await
				.ok()?;
			let sink_client_state =
				AnyClientState::try_from(sink_client_state.client_state?).ok()?;
			let height = sink_client_state.latest_height();
			let timestamp_at_creation =
				sink.query_timestamp_at(height.revision_height).await.ok()?;
			let period = send_packet.timeout_timestamp.saturating_sub(timestamp_at_creation);
			if period == 0 {
				min_timeout_height =
					min_timeout_height.max(Some(send_packet.timeout_height.revision_height));
				continue
			}
			let period = Duration::from_nanos(period);
			let approx_height =
				calculate_block_delay(period, sink.expected_block_time()).saturating_add(1);
			let timeout_height = if send_packet.timeout_height.revision_height < approx_height {
				send_packet.timeout_height.revision_height
			} else {
				approx_height
			};

			min_timeout_height = min_timeout_height.max(Some(timeout_height))
		}
	}
	min_timeout_height
}
