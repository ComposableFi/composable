use std::pin::Pin;

use codec::Codec;
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
use sp_keystore::SyncCryptoStorePtr;
use sp_runtime::KeyTypeId;

pub mod cosmos;
pub mod parachain;
use ibc::{
	core::{
		ics02_client::{
			client_consensus::AnyConsensusState, client_state::AnyClientState, header::AnyHeader,
		},
		ics04_channel::packet::{Packet, Sequence},
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	events::IbcEvent,
	signer::Signer,
	Height,
};

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
	/// IbcEvent type;
	type IbcEvent;

	/// Finality event type, passed on to [`Chain::query_latest_ibc_events`]
	type FinalityEvent;

	/// Error type, just needs to implement standard error trait.
	type Error: std::error::Error + From<String> + Send + Sync + 'static;

	/// Given the finality event, covert it to an [`AnyHeader`]
	async fn client_update_header<T>(
		&mut self,
		finality_event: Self::FinalityEvent,
		counterparty: &T,
	) -> Result<(AnyHeader, AnyClientState, UpdateType), Self::Error>
	where
		T: Chain,
		Self::Error: From<T::Error>;

	/// Query latest ibc events
	async fn query_latest_ibc_events(
		&mut self,
		update_client: &AnyHeader,
		client_state: &AnyClientState,
	) -> Result<Vec<IbcEvent>, Self::Error>;

	/// Fetch consensus state of chain
	async fn host_consensus_state(&self, at: Height) -> Result<AnyConsensusState, Self::Error>;

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

	/// Cache packets that have been sent from this chain
	fn cache_send_packet_seq(&mut self, packet: Packet);

	/// Remove packets with given sequences from cache
	fn remove_packets(&mut self, seqs: Vec<Sequence>);

	/// Get packet cache
	fn cached_packets(&self) -> &Vec<Packet>;

	/// Return the chain connection prefix
	fn connection_prefix(&self) -> CommitmentPrefix;

	/// Apply connecton prefix to path and encode
	fn apply_prefix(&self, path: String) -> Vec<u8>;

	/// Return the cached consensus height for the given client height
	async fn consensus_height(&self, client_height: Height) -> Option<Height>;

	/// Return the host chain's light client id on counterparty chain
	fn client_id(&self) -> ClientId;

	/// Return latest finalized height
	async fn latest_height(&self) -> Result<Height, Self::Error>;

	/// Return a stream that yields when new [`IbcEvents`] are parsed from a finality notification
	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = Self::IbcEvent> + Send + Sync>>;

	/// Check if the client on counterparty chain has been updated since it was created
	fn client_update_status(&self) -> bool;

	/// Set client update status
	fn set_client_update_status(&mut self, status: bool);
}

/// Provides an interface for managing key management for signing.
pub trait KeyProvider {
	type Public: Clone;
	type Signature: Codec;
	/// Should return the relayer's account id on the host chain as a string in the expected format
	/// Could be a hexadecimal, bech32 or ss58 string, any format the chain supports
	fn account_id(&self) -> Signer;
	/// Public key for the relayer
	fn public_key(&self) -> Self::Public;
	/// Return a reference to the keystore
	fn key_store(&self) -> SyncCryptoStorePtr;
	/// Key type id for key store access
	fn key_type_id(&self) -> KeyTypeId;
}

/// Provides an interface for the chain to the relayer core for submitting IbcEvents as well as
/// finality notifications
#[async_trait::async_trait]
pub trait Chain: IbcProvider + KeyProvider + Send + Sync {
	/// Return a stream that yields when new [`IbcEvents`] are ready to be queried.
	async fn finality_notifications(
		&self,
	) -> Pin<Box<dyn Stream<Item = Self::FinalityEvent> + Send + Sync>>;

	/// This should be used to submit new messages [`Vec<Any>`] from a counterparty chain to this
	/// chain. This should only return when the events have been submitted and finalized.
	async fn submit_ibc_messages(&self, messages: Vec<Any>) -> Result<(), Self::Error>;
}
