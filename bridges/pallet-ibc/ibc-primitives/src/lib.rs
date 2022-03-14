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
pub struct QueryClientStateResponse {
	pub client_state: Vec<u8>,
	pub proof: Vec<u8>,
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryClientStatesResponse {
	pub client_states: Vec<Vec<u8>>,
	pub proof: Vec<u8>,
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryConsensusStateResponse {
	pub consensus_state: Vec<u8>,
	pub proof: Vec<u8>,
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryConnectionResponse {
	pub connection: Vec<u8>,
	pub proof: Vec<u8>,
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryChannelResponse {
	pub channel: Vec<u8>,
	pub proof: Vec<u8>,
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryChannelsResponse {
	pub channels: Vec<Vec<u8>>,
	pub proof: Vec<u8>,
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryConnectionsResponse {
	pub connections: Vec<Vec<u8>>,
	pub proof: Vec<u8>,
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryNextSequenceReceiveResponse {
	pub sequence: Vec<u8>,
	pub proof: Vec<u8>,
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryPacketCommitmentResponse {
	pub commitment: Vec<u8>,
	pub proof: Vec<u8>,
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryPacketCommitmentsResponse {
	pub commitments: Vec<Vec<u8>>,
	pub proof: Vec<u8>,
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryPacketAcknowledgementResponse {
	pub ack: Vec<u8>,
	pub proof: Vec<u8>,
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryPacketAcknowledgementsResponse {
	pub acks: Vec<Vec<u8>>,
	pub proof: Vec<u8>,
	pub height: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryPacketReceiptResponse {
	pub receipt: Vec<u8>,
	pub proof: Vec<u8>,
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
	pub client_state: Vec<u8>,
	pub client_state_proof: Vec<u8>,
	pub connection_state_proof: Vec<u8>,
	pub consensus_proof: Vec<u8>,
	pub height: Vec<u8>,
}
