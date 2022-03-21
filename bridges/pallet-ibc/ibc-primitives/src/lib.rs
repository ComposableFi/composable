#![cfg_attr(not(feature = "std"), no_std)]

use scale_info::prelude::vec::Vec;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Proof {
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct IdentifiedChannel {
	pub channel_id: Vec<u8>,
	pub port_id: Vec<u8>,
	/// Protobuf encoded `ibc::core::ics04_channel::connection::ChannelEnd`
	pub channel_end: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct IdentifiedConnection {
	pub connection_id: Vec<u8>,
	/// Protobuf encoded `ibc::core::ics03_connection::connection::ConnectionEnd`
	pub connection_end: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryClientStateResponse {
	pub client_state: Vec<u8>,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryClientStatesResponse {
	pub client_states: Vec<Vec<u8>>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryConsensusStateResponse {
	pub consensus_state: Vec<u8>,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryConnectionResponse {
	/// Protobuf encoded `ibc::core::ics03_connection::connection::ConnectionEnd`
	pub connection: Vec<u8>,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryChannelResponse {
	/// Protobuf encoded `ibc::core::ics04_channel::connection::ChannelEnd`
	pub channel: Vec<u8>,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryChannelsResponse {
	pub channels: Vec<IdentifiedChannel>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryConnectionsResponse {
	pub connections: Vec<IdentifiedConnection>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryNextSequenceReceiveResponse {
	pub sequence: u64,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryPacketCommitmentResponse {
	pub commitment: Vec<u8>,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PacketState {
	pub port_id: Vec<u8>,
	pub channel_id: Vec<u8>,
	pub sequence: u64,
	pub data: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryPacketCommitmentsResponse {
	pub commitments: Vec<PacketState>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryPacketAcknowledgementResponse {
	pub ack: Vec<u8>,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryPacketAcknowledgementsResponse {
	pub acks: Vec<PacketState>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryPacketReceiptResponse {
	pub receipt: Vec<u8>,
	/// Trie proof
	pub proof: Vec<Vec<u8>>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryDenomTraceResponse {
	pub trace: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryDenomTracesResponse {
	pub trace: Vec<Vec<u8>>,
}

#[derive(codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ConnectionHandshakeProof {
	/// Protobuf encoded client state
	pub client_state: Vec<u8>,
	/// Trie proof for connection state, client state and consensus state
	pub proof: Vec<Vec<u8>>,
	pub height: u64,
}
