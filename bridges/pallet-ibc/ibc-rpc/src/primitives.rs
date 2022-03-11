use ibc::{
	core::{
		ics02_client::{client_consensus::AnyConsensusState, client_state::AnyClientState},
		ics03_connection::connection::ConnectionEnd,
		ics04_channel::{channel::ChannelEnd, packet::Sequence},
	},
	Height,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryClientStateResponse {
	pub client_state: AnyClientState,
	pub proof: Vec<u8>,
	pub height: Height,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryConsensusStateResponse {
	pub consensus_state: Vec<u8>,
	pub proof: Vec<u8>,
	pub height: Height,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryConnectionResponse {
	pub connection: ConnectionEnd,
	pub proof: Vec<u8>,
	pub height: Height,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryChannelResponse {
	pub channel: ChannelEnd,
	pub proof: Vec<u8>,
	pub height: Height,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryChannelsResponse {
	pub channels: Vec<ChannelEnd>,
	pub proof: Vec<u8>,
	pub height: Height,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryConnectionsResponse {
	pub connections: Vec<ConnectionEnd>,
	pub proof: Vec<u8>,
	pub height: Height,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryNextSequenceReceiveResponse {
	pub sequence: Sequence,
	pub proof: Vec<u8>,
	pub height: Height,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryPacketCommitmentResponse {
	pub commitment: Vec<u8>,
	pub proof: Vec<u8>,
	pub height: Height,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryPacketCommitmentsResponse {
	pub commitments: Vec<Vec<u8>>,
	pub proof: Vec<u8>,
	pub height: Height,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryPacketAcknowledgementResponse {
	pub ack: Vec<u8>,
	pub proof: Vec<u8>,
	pub height: Height,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryPacketAcknowledgementsResponse {
	pub acks: Vec<Vec<u8>>,
	pub proof: Vec<u8>,
	pub height: Height,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryPacketReceiptResponse {
	pub receipt: Vec<u8>,
	pub proof: Vec<u8>,
	pub height: Height,
}

// Temporary structs
#[derive(Clone, Serialize, Deserialize)]
pub struct Coin {
	pub amt: u128,
	pub denom: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryDenomTraceResponse {
	pub trace: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryDenomTracesResponse {
	pub trace: Vec<String>,
}

pub type ClientStateProof = Vec<u8>;
pub type ConsensusProof = Vec<u8>;
pub type ConnectionProof = Vec<u8>;
