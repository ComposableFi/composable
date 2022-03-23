#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use ibc::{
	core::{
		ics04_channel::packet::{Packet, Sequence},
		ics05_port::capabilities::Capability,
		ics24_host::identifier::{ChannelId, PortId},
	},
	timestamp::Timestamp,
};
use scale_info::prelude::string::{String, ToString};
use sp_std::{str::FromStr, vec::Vec};

pub struct SendPacketData {
	pub data: Vec<u8>,
	pub timeout_height_offset: u64,
	/// This value should be represent nano seconds
	pub timeout_timestamp_offset: u64,
	pub capability: Capability,
	pub port_id: Vec<u8>,
	pub channel_id: Vec<u8>,
	pub dest_port_id: Vec<u8>,
	pub dest_channel_id: Vec<u8>,
}
#[derive(codec::Encode, codec::Decode, Clone)]
pub struct OffchainPacketType {
	pub sequence: u64,
	pub source_port: Vec<u8>,
	pub source_channel: Vec<u8>,
	pub destination_port: Vec<u8>,
	pub destination_channel: Vec<u8>,
	pub data: Vec<u8>,
	pub timeout_height: (u64, u64),
	pub timeout_timestamp: u64,
}

impl From<OffchainPacketType> for Packet {
	fn from(packet: OffchainPacketType) -> Self {
		Self {
			sequence: Sequence::from(packet.sequence),
			source_port: PortId::from_str(
				&String::from_utf8(packet.source_port).unwrap_or_default(),
			)
			.unwrap_or_default(),
			source_channel: ChannelId::from_str(
				&String::from_utf8(packet.source_channel).unwrap_or_default(),
			)
			.unwrap_or_default(),
			destination_port: PortId::from_str(
				&String::from_utf8(packet.destination_port).unwrap_or_default(),
			)
			.unwrap_or_default(),
			destination_channel: ChannelId::from_str(
				&String::from_utf8(packet.destination_channel).unwrap_or_default(),
			)
			.unwrap_or_default(),
			data: packet.data,
			timeout_height: ibc::Height::new(packet.timeout_height.0, packet.timeout_height.1),
			timeout_timestamp: Timestamp::from_nanoseconds(packet.timeout_timestamp)
				.unwrap_or_default(),
		}
	}
}

impl From<Packet> for OffchainPacketType {
	fn from(packet: Packet) -> Self {
		Self {
			sequence: packet.sequence.into(),
			source_port: packet.source_port.to_string().as_bytes().to_vec(),
			source_channel: packet.source_channel.to_string().as_bytes().to_vec(),
			destination_port: packet.destination_port.to_string().as_bytes().to_vec(),
			destination_channel: packet.destination_channel.to_string().as_bytes().to_vec(),
			data: packet.data,
			timeout_height: (
				packet.timeout_height.revision_number,
				packet.timeout_height.revision_height,
			),
			timeout_timestamp: packet.timeout_timestamp.nanoseconds(),
		}
	}
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Proof {
	/// Trie proof
	pub proof: Vec<u8>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
pub struct IdentifiedChannel {
	pub channel_id: Vec<u8>,
	pub port_id: Vec<u8>,
	/// Protobuf encoded `ibc::core::ics04_channel::connection::ChannelEnd`
	pub channel_end: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
pub struct IdentifiedClientState {
	pub client_id: Vec<u8>,
	/// Protobuf encoded `AnyClientState`
	pub client_state: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
pub struct IdentifiedConnection {
	pub connection_id: Vec<u8>,
	/// Protobuf encoded `ibc::core::ics03_connection::connection::ConnectionEnd`
	pub connection_end: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
pub struct QueryClientStateResponse {
	/// Protobuf encoded `AnyClientState`
	pub client_state: Vec<u8>,
	/// Trie proof
	pub proof: Vec<u8>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
pub struct QueryClientStatesResponse {
	pub client_states: Vec<Vec<u8>>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
pub struct QueryConsensusStateResponse {
	pub consensus_state: Vec<u8>,
	/// Trie proof
	pub proof: Vec<u8>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
pub struct QueryConnectionResponse {
	/// Protobuf encoded `ibc::core::ics03_connection::connection::ConnectionEnd`
	pub connection: Vec<u8>,
	/// Trie proof
	pub proof: Vec<u8>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
pub struct QueryChannelResponse {
	/// Protobuf encoded `ibc::core::ics04_channel::connection::ChannelEnd`
	pub channel: Vec<u8>,
	/// Trie proof
	pub proof: Vec<u8>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
pub struct QueryChannelsResponse {
	pub channels: Vec<IdentifiedChannel>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
pub struct QueryConnectionsResponse {
	pub connections: Vec<IdentifiedConnection>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
pub struct QueryNextSequenceReceiveResponse {
	pub sequence: u64,
	/// Trie proof
	pub proof: Vec<u8>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
pub struct QueryPacketCommitmentResponse {
	pub commitment: Vec<u8>,
	/// Trie proof
	pub proof: Vec<u8>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
pub struct PacketState {
	pub port_id: Vec<u8>,
	pub channel_id: Vec<u8>,
	pub sequence: u64,
	pub data: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
pub struct QueryPacketCommitmentsResponse {
	pub commitments: Vec<PacketState>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
pub struct QueryPacketAcknowledgementResponse {
	pub ack: Vec<u8>,
	/// Trie proof
	pub proof: Vec<u8>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
pub struct QueryPacketAcknowledgementsResponse {
	pub acks: Vec<PacketState>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode)]
pub struct QueryPacketReceiptResponse {
	pub receipt: bool,
	/// Trie proof
	pub proof: Vec<u8>,
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
	pub proof: Vec<u8>,
	pub height: u64,
}
