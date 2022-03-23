#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]

use ibc_primitives::*;
use scale_info::prelude::vec::Vec;

sp_api::decl_runtime_apis! {
	/// IBC Runtime Apis
	pub trait IbcRuntimeApi {
		/// Returns the balance of this address
		fn query_balance_with_address(addr: Vec<u8>) -> Option<u128>;

		/// Quuery offchain packets
		fn query_packets(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<OffchainPacketType>>;

		/// Generate trie proof for these keys
		fn generate_proof(keys: Vec<Vec<u8>>) -> Option<Proof>;

		/// Return client state at height
		fn client_state(client_id: Vec<u8>) -> Option<QueryClientStateResponse>;

		/// Return the consensus state for the given client at a height
		fn client_consensus_state(client_id: Vec<u8>, client_height: Vec<u8>, latest_cs: bool) -> Option<QueryConsensusStateResponse>;

		/// Returns client states for all clients on chain
		fn clients() -> Option<Vec<(Vec<u8>, Vec<u8>)>>;

		/// Query the given connection state with proof
		fn connection(connection_id: Vec<u8>) -> Option<QueryConnectionResponse>;

		/// Returns all connections registered on chain
		fn connections() -> Option<QueryConnectionsResponse>;

		/// Returns all connections associated with the given client
		fn connection_using_client(client_id: Vec<u8>) -> Option<IdentifiedConnection>;

		/// Returns Connection handshake proof
		fn connection_handshake_proof(client_id: Vec<u8>, conn_id: Vec<u8>) -> Option<ConnectionHandshakeProof>;

		fn channel(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<QueryChannelResponse>;

		/// Should return the client state for the client supporting this channel
		fn channel_client(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<IdentifiedClientState>;

		/// Returns all channels associated with this connection
		fn connection_channels(connection_id: Vec<u8>) -> Option<QueryChannelsResponse>;

		/// Returns all channels registered on chain
		fn channels() -> Option<QueryChannelsResponse>;

		fn packet_commitments(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<QueryPacketCommitmentsResponse>;

		fn packet_acknowledgements(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<QueryPacketAcknowledgementsResponse>;

		fn unreceived_packets(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<u64>>;

		fn unreceived_acknowledgements(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<u64>>;

		fn next_seq_recv(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<QueryNextSequenceReceiveResponse>;

		fn packet_commitment(channel_id: Vec<u8>, port_id: Vec<u8>, seq: u64) -> Option<QueryPacketCommitmentResponse>;

		fn packet_acknowledgement(channel_id: Vec<u8>, port_id: Vec<u8>, seq: u64) -> Option<QueryPacketAcknowledgementResponse>;

		fn packet_receipt(channel_id: Vec<u8>, port_id: Vec<u8>, seq: u64) -> Option<QueryPacketReceiptResponse>;

		fn denom_trace(denom: Vec<u8>) -> Option<QueryDenomTraceResponse>;

		fn denom_traces(offset: Vec<u8>, limit: u64, height: u32) -> Option<QueryDenomTracesResponse>;
	}
}
