#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Proof {
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	/// Protobuf encoded `ibc::Height`
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct IdentifiedChannel {
	pub channel_id: String,
	pub port_id: String,
	/// Protobuf encoded `ibc::core::ics04_channel::connection::ChannelEnd`
	pub channel_end: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct IdentifiedConnection {
	pub connection_id: String,
	/// Protobuf encoded `ibc::core::ics03_connection::connection::ConnectionEnd`
	pub connection_end: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct IdentifiedClientState {
	pub client_id: String,
	pub client_type: String,
	/// Protobuf encoded client state for this client type as defined in `ibc-rs`
	pub client_state: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct IdentifiedConsensusState {
	pub client_id: String,
	pub client_type: String,
	/// Protobuf encoded consensus state for this client type as defined in `ibc-rs`
	pub consensus_state: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryClientStateResponse {
	pub client_state: IdentifiedClientState,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	/// Protobuf encoded `ibc::Height`
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryClientStatesResponse {
	pub client_states: Vec<IdentifiedClientState>,
	/// Protobuf encoded `ibc::Height`
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryConsensusStateResponse {
	pub consensus_state: IdentifiedConsensusState,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	/// Protobuf encoded `ibc::Height`
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryConnectionResponse {
	/// Protobuf encoded `ibc::core::ics03_connection::connection::ConnectionEnd`
	pub connection: Vec<u8>,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	/// Protobuf encoded `ibc::Height`
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryChannelResponse {
	/// Protobuf encoded `ibc::core::ics04_channel::connection::ChannelEnd`
	pub channel: Vec<u8>,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	/// Protobuf encoded `ibc::Height`
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryChannelsResponse {
	pub channels: Vec<IdentifiedChannel>,
	/// Protobuf encoded `ibc::Height`
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryConnectionsResponse {
	pub connections: Vec<IdentifiedConnection>,
	/// Protobuf encoded `ibc::Height`
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryNextSequenceReceiveResponse {
	pub sequence: u64,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	/// Protobuf encoded `ibc::Height`
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryPacketCommitmentResponse {
	pub commitment: Vec<u8>,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	/// Protobuf encoded `ibc::Height`
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PacketState {
	pub port_id: String,
	pub channel_id: String,
	pub sequence: u64,
	pub data: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryPacketCommitmentsResponse {
	pub commitments: Vec<PacketState>,
	/// Protobuf encoded `ibc::Height`
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryPacketAcknowledgementResponse {
	pub ack: Vec<u8>,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	/// Protobuf encoded `ibc::Height`
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryPacketAcknowledgementsResponse {
	pub acks: Vec<PacketState>,
	/// Protobuf encoded `ibc::Height`
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryPacketReceiptResponse {
	pub receipt: Vec<u8>,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	/// Protobuf encoded `ibc::Height`
	pub height: Vec<u8>,
}

// Temporary structs
#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Coin {
	pub amt: u128,
	pub denom: String,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryDenomTraceResponse {
	pub trace: String,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryDenomTracesResponse {
	pub trace: Vec<String>,
}

#[derive(codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ConnectionHandshakeProof {
	/// Protobuf encoded client state
	pub client_state: IdentifiedClientState,
	/// Trie proof for connection state, client state and consensus state
	pub proof: Vec<Vec<u8>>,
	pub height: Vec<u8>,
}
