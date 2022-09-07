use std::{pin::Pin, time::Duration};

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

use crate::error::Error;
#[cfg(feature = "testing")]
use ibc::applications::transfer::msgs::transfer::MsgTransfer;
use ibc::{
	applications::transfer::PrefixedCoin,
	core::{
		ics02_client::{client_type::ClientType, header::AnyHeader},
		ics04_channel::channel::{ChannelEnd, Order::Unordered},
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	events::IbcEvent,
	signer::Signer,
	timestamp::Timestamp,
	Height,
};
use ibc_proto::ibc::core::channel::v1::QueryChannelsResponse;
use ibc_rpc::PacketInfo;

pub mod error;

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
	) -> Result<(AnyHeader, Vec<IbcEvent>, UpdateType), Self::Error>
	where
		T: Chain,
		Self::Error: From<T::Error>;

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
	async fn channel_whitelist(&self) -> Result<Vec<(ChannelId, PortId)>, Self::Error>;

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

	/// Returns the client type of this chain.
	fn client_type(&self) -> ClientType;
}

/// Provides an interface that allows us run the hyperspace-testsuite
/// with [`Chain`] implementations.
#[cfg(feature = "testing")]
#[async_trait::async_trait]
pub trait TestProvider: Chain + Clone + 'static {
	/// Initiate an ibc transfer on chain.
	async fn send_transfer(&self, params: MsgTransfer<PrefixedCoin>) -> Result<(), Self::Error>;

	/// Return a stream that yields when new [`IbcEvents`] are parsed from a finality notification
	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent> + Send + Sync>>;

	/// Returns a stream that yields chain Block number and hash
	async fn subscribe_blocks(&self) -> Pin<Box<dyn Stream<Item = u64> + Send + Sync>>;

	/// Should return timestamp of chain at a given block height
	async fn timestamp_at(&self, block_number: u64) -> u64;
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

	/// Return a stream that yields when new [`IbcEvents`] are ready to be queried.
	async fn finality_notifications(
		&self,
	) -> Pin<Box<dyn Stream<Item = Self::FinalityEvent> + Send + Sync>>;

	/// This should be used to submit new messages [`Vec<Any>`] from a counterparty chain to this
	/// chain.
	async fn submit_ibc_messages(&self, messages: Vec<Any>) -> Result<(), Self::Error>;
}

/// Return undelivered packet sequences
/// Implementation should work both for ordered and unordered channels
pub async fn query_undelivered_sequences(
	source_height: Height,
	sink_height: Height,
	channel_id: ChannelId,
	port_id: PortId,
	source: &impl Chain,
	sink: &impl Chain,
) -> Result<Vec<u64>, anyhow::Error> {
	let channel_response = source.query_channel_end(source_height, channel_id, port_id.clone()).await?;
	let channel_end = ChannelEnd::try_from(
		channel_response
			.channel
			.ok_or_else(|| Error::Custom("ChannelEnd not could not be decoded".to_string()))?,
	)
	.map_err(|e| Error::Custom(e.to_string()))?;
	// First we fetch all packet commitments from source
	let seqs = source.query_packet_commitments(source_height, channel_id, port_id.clone()).await?;
	let counterparty_channel_id = channel_end
		.counterparty()
		.channel_id
		.ok_or_else(|| Error::Custom("Expected counterparty channel id".to_string()))?;
	let counterparty_port_id = channel_end.counterparty().port_id.clone();

	let undelivered_sequences = if channel_end.ordering == Unordered {
		sink.query_unreceived_packets(
			sink_height,
			counterparty_channel_id,
			counterparty_port_id.clone(),
			seqs,
		)
		.await?
	} else {
		let next_seq_recv = sink
			.query_next_sequence_recv(
				sink_height,
				&counterparty_port_id,
				&counterparty_channel_id,
			)
			.await?
			.next_sequence_receive;
		seqs.into_iter().filter(|seq| *seq > next_seq_recv).collect()
	};

	Ok(undelivered_sequences)
}

/// Return undelivered packet acknowledgements
pub async fn query_undelivered_acks(
	source_height: Height,
	sink_height: Height,
	channel_id: ChannelId,
	port_id: PortId,
	source: &impl Chain,
	sink: &impl Chain,
) -> Result<Vec<u64>, anyhow::Error> {
	let channel_response = source.query_channel_end(source_height, channel_id, port_id.clone()).await?;
	let channel_end = ChannelEnd::try_from(
		channel_response
			.channel
			.ok_or_else(|| Error::Custom("ChannelEnd not could not be decoded".to_string()))?,
	)
	.map_err(|e| Error::Custom(e.to_string()))?;
	// First we fetch all packet acknowledgements from source
	let seqs = source.query_packet_acknowledgements(source_height, channel_id, port_id.clone()).await?;
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
