#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]

use ibc_primitives::*;
#[cfg(not(feature = "std"))]
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
	/// IBC Runtime Apis
	pub trait IbcRuntimeApi {
		/// Get Ibc child trie prefix
		fn child_trie_key() -> Vec<u8>;
		/// Get parachain id
		fn para_id() -> u32;

		/// Returns the balance of this address
		fn query_balance_with_address(addr: Vec<u8>) -> Option<u128>;

		/// Query offchain packets
		fn query_packets(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<OffchainPacketType>>;

		/// Query offchain acks
		fn query_acknowledgements(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<Vec<u8>>>;

		/// Returns client state at height
		fn client_state(client_id: Vec<u8>) -> Option<QueryClientStateResponse>;

		/// Returns protobuf encoded `AnyConsensusState` consensus state for host chain
		fn host_consensus_state(height: u32) -> Option<Vec<u8>>;

		/// Return the consensus state for the given client at a height
		fn client_consensus_state(client_id: Vec<u8>, client_height: Vec<u8>, latest_cs: bool) -> Option<QueryConsensusStateResponse>;

		/// Returns client states for all clients on chain
		fn clients() -> Option<Vec<(Vec<u8>, Vec<u8>)>>;

		/// Query the given connection state with proof
		fn connection(connection_id: Vec<u8>) -> Option<QueryConnectionResponse>;

		/// Returns all connections registered on chain
		fn connections() -> Option<QueryConnectionsResponse>;

		/// Returns all connections associated with the given client
		fn connection_using_client(client_id: Vec<u8>) -> Option<Vec<IdentifiedConnection>>;

		fn channel(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<QueryChannelResponse>;

		/// Should return the client state for the client supporting this channel
		fn channel_client(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<IdentifiedClientState>;

		/// Returns all channels associated with this connection
		fn connection_channels(connection_id: Vec<u8>) -> Option<QueryChannelsResponse>;

		/// Returns all channels registered on chain
		fn channels() -> Option<QueryChannelsResponse>;

		fn connection_handshake(client_id: Vec<u8>, connection_id: Vec<u8>) -> Option<ConnectionHandshake>;

		fn packet_commitments(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<QueryPacketCommitmentsResponse>;

		fn packet_acknowledgements(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<QueryPacketAcknowledgementsResponse>;

		fn unreceived_packets(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<u64>>;

		fn unreceived_acknowledgements(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<u64>>;

		fn next_seq_recv(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<QueryNextSequenceReceiveResponse>;

		fn packet_commitment(channel_id: Vec<u8>, port_id: Vec<u8>, seq: u64) -> Option<QueryPacketCommitmentResponse>;

		fn packet_acknowledgement(channel_id: Vec<u8>, port_id: Vec<u8>, seq: u64) -> Option<QueryPacketAcknowledgementResponse>;

		fn packet_receipt(channel_id: Vec<u8>, port_id: Vec<u8>, seq: u64) -> Option<QueryPacketReceiptResponse>;

		fn denom_trace(asset_id: u128) -> Option<QueryDenomTraceResponse>;

		/// Key is the asset id from which to start looking up results
		fn denom_traces(key: Option<u128>, offset: Option<u32>, limit: u64, count_total: bool) -> QueryDenomTracesResponse;

		fn block_events(extrinsic_index: Option<u32>) -> Vec<pallet_ibc::events::IbcEvent>;
	}
}
