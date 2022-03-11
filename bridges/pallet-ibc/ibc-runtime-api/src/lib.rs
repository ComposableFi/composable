#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]

use codec::Codec;
use ibc_primitives::*;
use scale_info::prelude::string::String;

sp_api::decl_runtime_apis! {
	/// IBC Runtime Apis
	pub trait IbcRuntimeApi<Header>
	where
		Header: Codec
	{
		fn latest_height() -> Option<u64>;

		fn header_at_height(height: u64) -> Option<Header>;

		fn query_balance(key_name: String) -> Option<Vec<u8>>;

		fn query_balance_with_address(addr: String) -> Option<Vec<u8>>;

		fn unbonding_period() -> Option<u64>;

		fn client_state(height: u64, client_id: String) -> Option<QueryClientStateResponse>;

		fn client_consensus_state(client_id: String, client_height: Vec<u8>) -> Option<QueryConsensusStateResponse>;

		fn clients() -> Option<QueryClientStatesResponse>;

		fn find_matching_client(client_state: Vec<u8>) -> Option<String>;

		fn connection(height: u64, connection_id: String) -> Option<QueryConnectionResponse>;

		fn connections() -> Option<QueryConnectionsResponse>;

		fn connections_using_client(height: u64, client_id: String) -> Option<QueryConnectionsResponse>;

		/// Returns Connection handshake proof
		fn conn_handshake_proof(height: u64, client_id: String, conn_id: String) -> Option<ConnectionHandshakeProof>;

		fn channel(height: u64, channel_id: String, port_id: String) -> Option<QueryChannelResponse>;

		fn channel_client(height: u64, channel_id: String, port_id: String) -> Option<Vec<u8>>;

		fn connection_channels(height: u64, connection_id: String) -> Option<QueryChannelsResponse>;

		fn channels() -> Option<QueryChannelsResponse>;

		fn packet_commitments(height: u64, channel_id: String, port_id: String) -> Option<QueryPacketCommitmentsResponse>;

		fn packet_acknowledgements(height: u64, channel_id: String, port_id: String) -> Option<QueryPacketAcknowledgementsResponse>;

		fn unreceived_packets(height: u64, channel_id: String, port_id: String, seqs: Vec<u64>) -> Option<Vec<u64>>;

		fn unreceived_acknowledgements(height: u64, channel_id: String, port_id: String, seqs: Vec<u64>) -> Option<Vec<u64>>;

		fn next_seq_recv(height: u64, channel_id: String, port_id: String) -> Option<QueryNextSequenceReceiveResponse>;

		fn packet_commitment(height: u64, channel_id: String, port_id: String, seq: u64) -> Option<QueryPacketCommitmentResponse>;

		fn packet_acknowledgement(height: u64, channel_id: String, port_id: String, seq: u64) -> Option<QueryPacketAcknowledgementResponse>;

		fn packet_receipt(height: u64, channel_id: String, port_id: String, seq: u64) -> Option<QueryPacketReceiptResponse>;

		fn denom_trace(denom: String) -> Option<QueryDenomTraceResponse>;

		fn denom_traces(offset: String, limit: u64, height: u64) -> Option<QueryDenomTracesResponse>;
	}
}
