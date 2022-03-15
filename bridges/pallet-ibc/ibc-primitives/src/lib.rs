#![cfg_attr(not(feature = "std"), no_std)]
use ibc::core::{
	ics02_client::{client_consensus::AnyConsensusState, client_state::AnyClientState},
	ics03_connection::connection::ConnectionEnd,
	ics04_channel::{channel::ChannelEnd, packet::Sequence},
};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Proof {
	/// Trie proof
	proof: Vec<Vec<u8>>,
	/// Protobuf encoded `ibc::Height`
	height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryClientStateResponse {
	/// Protobuf encoded client state
	/// Should decode into the respective client state definition for this client
	pub client_state: Vec<u8>,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	/// Protobuf encoded `ibc::Height`
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryClientStatesResponse {
	/// Vector of Protobuf encoded client states
	/// Should decode into the respective client state definition for this client
	pub client_states: Vec<Vec<u8>>,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	/// Protobuf encoded `ibc::Height`
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryConsensusStateResponse {
	/// Protobuf encoded consensus state
	/// Should decode into the respective consensus state definition for this client
	pub consensus_state: Vec<u8>,
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
	/// Vector of Protobuf encoded `ibc::core::ics04_channel::connection::ChannelEnd`
	pub channels: Vec<Vec<u8>>,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	/// Protobuf encoded `ibc::Height`
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryConnectionsResponse {
	/// Vector of Protobuf encoded `ibc::core::ics03_connection::connection::ConnectionEnd`
	pub connections: Vec<Vec<u8>>,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	/// Protobuf encoded `ibc::Height`
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryNextSequenceReceiveResponse {
	/// Protobuf encoded `ibc::Sequence`
	pub sequence: Vec<u8>,
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
pub struct QueryPacketCommitmentsResponse {
	pub commitments: Vec<Vec<u8>>,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
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
	pub acks: Vec<Vec<u8>>,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
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
	pub client_state: Vec<u8>,
	/// Trie proof for connection state, client state and consensus state
	pub proof: Vec<Vec<u8>>,
	pub height: Vec<u8>,
}
