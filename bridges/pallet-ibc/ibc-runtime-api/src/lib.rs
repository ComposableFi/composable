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
		/// Return latest height
		fn latest_height() -> Option<u32>;

		/// Returns the balance of this address
		fn query_balance_with_address(addr: String) -> Option<Vec<u8>>;

		/// Generate trie proof for these keys
		fn generate_proof(keys: Vec<Vec<u8>>) -> Option<Proof>;

		/// Return client state at height
		fn client_state(client_id: String) -> Option<QueryClientStateResponse>;

		/// Return the consensus state for the given client at a height
		fn client_consensus_state(client_id: String, client_height: Vec<u8>) -> Option<QueryConsensusStateResponse>;

		/// Returns client states for all clients on chain
		fn clients() -> Option<Vec<Vec<u8>>>;

		/// Query the given connection state with proof
		fn connection(connection_id: String) -> Option<QueryConnectionResponse>;

		/// Returns all connections registered on chain
		fn connections() -> Option<QueryConnectionsResponse>;

		/// Returns all connections associated with the given client
		fn connection_using_client(client_id: String) -> Option<QueryConnectionResponse>;

		/// Returns Connection handshake proof
		fn connection_handshake_proof(client_id: String, conn_id: String) -> Option<ConnectionHandshakeProof>;

		fn channel(channel_id: String, port_id: String) -> Option<QueryChannelResponse>;

		/// Should return the client state for the client supporting this channel
		fn channel_client(channel_id: String, port_id: String) -> Option<Vec<u8>>;

		/// Returns all channels associated with this connection
		fn connection_channels(connection_id: String) -> Option<QueryChannelsResponse>;

		/// Returns all channels registered on chain
		fn channels() -> Option<QueryChannelsResponse>;

		fn packet_commitments(channel_id: String, port_id: String) -> Option<QueryPacketCommitmentsResponse>;

		fn packet_acknowledgements(channel_id: String, port_id: String) -> Option<QueryPacketAcknowledgementsResponse>;

		fn unreceived_packets(channel_id: String, port_id: String, seqs: Vec<u64>) -> Option<Vec<u64>>;

		fn unreceived_acknowledgements(channel_id: String, port_id: String, seqs: Vec<u64>) -> Option<Vec<u64>>;

		fn next_seq_recv(channel_id: String, port_id: String) -> Option<QueryNextSequenceReceiveResponse>;

		fn packet_commitment(channel_id: String, port_id: String, seq: u64) -> Option<QueryPacketCommitmentResponse>;

		fn packet_acknowledgement(channel_id: String, port_id: String, seq: u64) -> Option<QueryPacketAcknowledgementResponse>;

		fn packet_receipt(channel_id: String, port_id: String, seq: u64) -> Option<QueryPacketReceiptResponse>;

		fn denom_trace(denom: String) -> Option<QueryDenomTraceResponse>;

		fn denom_traces(offset: String, limit: u64, height: u32) -> Option<QueryDenomTracesResponse>;
	}
}
