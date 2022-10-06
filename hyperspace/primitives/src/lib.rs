use std::pin::Pin;

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

use ibc::{
	core::{
		ics02_client::client_state::ClientType,
		ics04_channel::packet::Packet,
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	events::IbcEvent,
	signer::Signer,
	Height,
};

pub mod error;
pub mod mock;

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

	/// Query packets
	async fn query_packets(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seqs: Vec<u64>,
	) -> Result<Vec<Packet>, Self::Error>;

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

	/// Return latest finalized height
	async fn latest_height(&self) -> Result<Height, Self::Error>;

	/// Return a proof for the host consensus state at the given height to be included in the
	/// consensus state proof.
	async fn query_host_consensus_state_proof(
		&self,
		height: Height,
	) -> Result<Option<Vec<u8>>, Self::Error>;

	/// Return the chain connection prefix
	fn connection_prefix(&self) -> CommitmentPrefix;

	/// Return the host chain's light client id on counterparty chain
	fn client_id(&self) -> ClientId;

	/// Returns the client type of this chain.
	fn client_type(&self) -> ClientType;

	/// Return a stream that yields when new [`IbcEvents`] are parsed from a finality notification
	/// Only used in tests.
	#[cfg(feature = "testing")]
	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent> + Send + Sync>>;
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
