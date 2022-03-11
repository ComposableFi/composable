// Copyright (C) 2021-2022 ComposableFi.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
#![warn(missing_docs)]

mod primitives;

use std::sync::Arc;

use codec::{Codec, Encode};
use ibc::{
	core::ics02_client::{client_consensus::AnyConsensusState, client_state::AnyClientState},
	Height,
};
use jsonrpc_core::Result;
use jsonrpc_derive::rpc;

use crate::primitives::{
	ClientStateProof, Coin, ConnectionProof, ConsensusProof, QueryChannelResponse,
	QueryChannelsResponse, QueryClientStateResponse, QueryConnectionResponse,
	QueryConnectionsResponse, QueryConsensusStateResponse, QueryDenomTraceResponse,
	QueryDenomTracesResponse, QueryNextSequenceReceiveResponse, QueryPacketAcknowledgementResponse,
	QueryPacketAcknowledgementsResponse, QueryPacketCommitmentResponse,
	QueryPacketCommitmentsResponse, QueryPacketReceiptResponse,
};
use ibc_runtime_api::IbcRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;

/// IBC RPC methods.
#[rpc]
pub trait IbcApi<Header, Hash, Transaction> {
	#[rpc(name = "ibc_queryTransaction")]
	fn query_transaction(&self, tx_hash: Hash) -> Result<Transaction>;

	#[rpc(name = "ibc_queryTransactions")]
	fn query_transactions(&self, page: u32, limit: u32) -> Result<Vec<Transaction>>;

	#[rpc(name = "ibc_queryLatestHeight")]
	fn query_latest_height(&self) -> Result<u64>;

	#[rpc(name = "ibc_queryHeaderAtHeight")]
	fn query_header_at_height(&self, height: u64) -> Result<Header>;

	#[rpc(name = "ibc_queryBalance")]
	fn query_balance(&self, key_name: String) -> Result<Coin>;

	#[rpc(name = "ibc_queryBalanceWithAddress")]
	fn query_balance_with_address(&self, addr: String) -> Result<Coin>;

	#[rpc(name = "ibc_queryUnbondingPeriod")]
	fn query_unbonding_period(&self) -> Result<u64>;

	#[rpc(name = "ibc_queryClientState")]
	fn query_client_state(&self, height: u64, client_id: String) -> Result<AnyClientState>;

	#[rpc(name = "ibc_queryClientStateResponse")]
	fn query_client_state_response(
		&self,
		height: u64,
		src_client_Id: String,
	) -> Result<QueryClientStateResponse>;

	#[rpc(name = "ibc_queryClientConsensusState")]
	fn query_client_consensus_state(
		&self,
		client_id: String,
		client_height: ibc::Height,
	) -> Result<QueryConsensusStateResponse>;

	#[rpc(name = "ibc_queryUpgradedClient")]
	fn query_upgraded_client(&self, height: u64) -> Result<QueryClientStateResponse>;

	#[rpc(name = "ibc_queryUpgradedConnectionState")]
	fn query_upgraded_cons_state(&self, height: u64) -> Result<QueryConsensusStateResponse>;

	#[rpc(name = "ibc_queryConsensusState")]
	fn query_consensus_state(&self, height: u64, client_id: String) -> Result<Vec<u8>>;

	#[rpc(name = "ibc_queryClients")]
	fn query_clients(&self) -> Result<Vec<AnyClientState>>;

	#[rpc(name = "ibc_autoUpdateClient")]
	fn auto_update_client(
		&self,
		dst: String,
		thresholdTime: u64,
		src_clientId: String,
		dst_clientId: String,
	) -> Result<u64>;

	#[rpc(name = "ibc_findMatchingClient")]
	fn find_matching_client(&self, client_state: AnyClientState) -> Result<Option<String>>;

	#[rpc(name = "ibc_queryConnection")]
	fn query_connection(
		&self,
		height: u64,
		connection_id: String,
	) -> Result<QueryConnectionResponse>;

	#[rpc(name = "ibc_queryConnections")]
	fn query_connections(&self) -> Result<QueryConnectionsResponse>;

	#[rpc(name = "ibc_queryConnectionUsingClient")]
	fn query_connections_using_client(
		&self,
		height: u64,
		client_id: String,
	) -> Result<QueryConnectionsResponse>;

	#[rpc(name = "ibc_generateConnectionHandshakeProof")]
	fn generate_conn_handshake_proof(
		&self,
		height: u64,
		client_id: String,
		conn_id: String,
	) -> Result<(AnyClientState, ClientStateProof, ConsensusProof, ConnectionProof, Height)>;

	#[rpc(name = "ibc_newClientState")]
	fn new_client_state(
		&self,
		dst_update_header: Header,
		dst_trusting_period: u64,
		dst_unbonding_period: u64,
		allow_update_after_expiry: bool,
		allow_update_after_misbehaviour: bool,
	) -> Result<AnyClientState>;

	#[rpc(name = "ibc_queryChannel")]
	fn query_channel(
		&self,
		height: u64,
		channel_id: String,
		port_id: String,
	) -> Result<QueryChannelResponse>;

	#[rpc(name = "ibc_queryChannelClient")]
	fn query_channel_client(
		&self,
		height: u64,
		channel_id: String,
		port_id: String,
	) -> Result<AnyClientState>;

	#[rpc(name = "ibc_queryConnectionChannels")]
	fn query_connection_channels(
		&self,
		height: u64,
		connection_id: String,
	) -> Result<QueryChannelsResponse>;

	#[rpc(name = "ibc_queryChannels")]
	fn query_channels(&self) -> Result<QueryChannelsResponse>;

	#[rpc(name = "ibc_queryPacketCommitments")]
	fn query_packet_commitments(
		&self,
		height: u64,
		channel_id: String,
		port_id: String,
	) -> Result<QueryPacketCommitmentsResponse>;

	#[rpc(name = "ibc_queryPacketAcknowledgements")]
	fn query_packet_acknowledgements(
		&self,
		height: u64,
		channel_id: String,
		port_id: String,
	) -> Result<QueryPacketAcknowledgementsResponse>;

	#[rpc(name = "ibc_queryUnreceivedPackets")]
	fn query_unreceived_packets(
		&self,
		height: u64,
		channel_id: String,
		port_id: String,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>>;

	#[rpc(name = "ibc_queryUnreceivedAcknowledgement")]
	fn query_unreceived_acknowledgements(
		&self,
		height: u64,
		channel_id: String,
		port_id: String,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>>;

	#[rpc(name = "ibc_queryNextSeqRecv")]
	fn query_next_seq_recv(
		&self,
		height: u64,
		channel_id: String,
		port_id: String,
	) -> Result<QueryNextSequenceReceiveResponse>;

	#[rpc(name = "ibc_queryPacketCommitment")]
	fn query_packet_commitment(
		&self,
		height: u64,
		channel_id: String,
		port_id: String,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse>;

	#[rpc(name = "ibc_queryPacketAcknowledgement")]
	fn query_packet_acknowledgement(
		&self,
		height: u64,
		channel_id: String,
		port_id: String,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse>;

	#[rpc(name = "ibc_queryPacketReceipt")]
	fn query_packet_receipt(
		&self,
		height: u64,
		channel_id: String,
		port_id: String,
		seq: u64,
	) -> Result<QueryPacketReceiptResponse>;

	#[rpc(name = "ibc_queryDenomTrace")]
	fn query_denom_trace(&self, denom: String) -> Result<QueryDenomTraceResponse>;

	#[rpc(name = "ibc_queryDenomTraces")]
	fn query_denom_traces(
		&self,
		offset: String,
		limit: u64,
		height: u64,
	) -> Result<QueryDenomTracesResponse>;
}

/// An implementation of IBC specific RPC methods.
pub struct IbcRpcHandler<C, B> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<B>,
}

impl<C, B> IbcRpcHandler<C, B> {
	/// Create new `IbcRpcHandler` with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block, Transaction> IbcApi<<Block as BlockT>::Header, <Block as BlockT>::Hash, Transaction>
	for IbcRpcHandler<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: IbcRuntimeApi<Block, <Block as BlockT>::Header>,
{
	fn query_transaction(&self, tx_hash: <Block as BlockT>::Hash) -> Result<Transaction> {
		todo!()
	}

	fn query_transactions(&self, page: u32, limit: u32) -> Result<Vec<Transaction>> {
		todo!()
	}

	fn query_latest_height(&self) -> Result<u64> {
		todo!()
	}

	fn query_header_at_height(&self, height: u64) -> Result<<Block as BlockT>::Header> {
		todo!()
	}

	fn query_balance(&self, key_name: String) -> Result<Coin> {
		todo!()
	}

	fn query_balance_with_address(&self, addr: String) -> Result<Coin> {
		todo!()
	}

	fn query_unbonding_period(&self) -> Result<u64> {
		todo!()
	}

	fn query_client_state(&self, height: u64, client_id: String) -> Result<AnyClientState> {
		todo!()
	}

	fn query_client_state_response(
		&self,
		height: u64,
		src_client_Id: String,
	) -> Result<QueryClientStateResponse> {
		todo!()
	}

	fn query_client_consensus_state(
		&self,
		client_id: String,
		client_height: Height,
	) -> Result<QueryConsensusStateResponse> {
		todo!()
	}

	fn query_upgraded_client(&self, height: u64) -> Result<QueryClientStateResponse> {
		todo!()
	}

	fn query_upgraded_cons_state(&self, height: u64) -> Result<QueryConsensusStateResponse> {
		todo!()
	}

	fn query_consensus_state(&self, height: u64, client_id: String) -> Result<Vec<u8>> {
		todo!()
	}

	fn query_clients(&self) -> Result<Vec<AnyClientState>> {
		todo!()
	}

	fn auto_update_client(
		&self,
		dst: String,
		thresholdTime: u64,
		src_clientId: String,
		dst_clientId: String,
	) -> Result<u64> {
		todo!()
	}

	fn find_matching_client(&self, client_state: AnyClientState) -> Result<Option<String>> {
		todo!()
	}

	fn query_connection(
		&self,
		height: u64,
		connection_id: String,
	) -> Result<QueryConnectionResponse> {
		todo!()
	}

	fn query_connections(&self) -> Result<QueryConnectionsResponse> {
		todo!()
	}

	fn query_connections_using_client(
		&self,
		height: u64,
		client_id: String,
	) -> Result<QueryConnectionsResponse> {
		todo!()
	}

	fn generate_conn_handshake_proof(
		&self,
		height: u64,
		client_id: String,
		conn_id: String,
	) -> Result<(AnyClientState, ClientStateProof, ConsensusProof, ConnectionProof, Height)> {
		todo!()
	}

	fn new_client_state(
		&self,
		dst_update_header: <Block as BlockT>::Header,
		dst_trusting_period: u64,
		dst_unbonding_period: u64,
		allow_update_after_expiry: bool,
		allow_update_after_misbehaviour: bool,
	) -> Result<AnyClientState> {
		todo!()
	}

	fn query_channel(
		&self,
		height: u64,
		channel_id: String,
		port_id: String,
	) -> Result<QueryChannelResponse> {
		todo!()
	}

	fn query_channel_client(
		&self,
		height: u64,
		channel_id: String,
		port_id: String,
	) -> Result<AnyClientState> {
		todo!()
	}

	fn query_connection_channels(
		&self,
		height: u64,
		connection_id: String,
	) -> Result<QueryChannelsResponse> {
		todo!()
	}

	fn query_channels(&self) -> Result<QueryChannelsResponse> {
		todo!()
	}

	fn query_packet_commitments(
		&self,
		height: u64,
		channel_id: String,
		port_id: String,
	) -> Result<QueryPacketCommitmentsResponse> {
		todo!()
	}

	fn query_packet_acknowledgements(
		&self,
		height: u64,
		channel_id: String,
		port_id: String,
	) -> Result<QueryPacketAcknowledgementsResponse> {
		todo!()
	}

	fn query_unreceived_packets(
		&self,
		height: u64,
		channel_id: String,
		port_id: String,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>> {
		todo!()
	}

	fn query_unreceived_acknowledgements(
		&self,
		height: u64,
		channel_id: String,
		port_id: String,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>> {
		todo!()
	}

	fn query_next_seq_recv(
		&self,
		height: u64,
		channel_id: String,
		port_id: String,
	) -> Result<QueryNextSequenceReceiveResponse> {
		todo!()
	}

	fn query_packet_commitment(
		&self,
		height: u64,
		channel_id: String,
		port_id: String,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse> {
		todo!()
	}

	fn query_packet_acknowledgement(
		&self,
		height: u64,
		channel_id: String,
		port_id: String,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse> {
		todo!()
	}

	fn query_packet_receipt(
		&self,
		height: u64,
		channel_id: String,
		port_id: String,
		seq: u64,
	) -> Result<QueryPacketReceiptResponse> {
		todo!()
	}

	fn query_denom_trace(&self, denom: String) -> Result<QueryDenomTraceResponse> {
		todo!()
	}

	fn query_denom_traces(
		&self,
		offset: String,
		limit: u64,
		height: u64,
	) -> Result<QueryDenomTracesResponse> {
		todo!()
	}
}
