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

use ibc_primitives::*;

use std::sync::Arc;

use codec::{Codec, Encode};
use ibc::{
	core::ics02_client::{client_consensus::AnyConsensusState, client_state::AnyClientState},
	Height,
};
use ibc_runtime_api::IbcRuntimeApi;
use jsonrpc_core::{futures::future::ok, Error as JsonRpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use tendermint_proto::Protobuf;

/// IBC RPC methods.
#[rpc]
pub trait IbcApi<Header, Hash> {
	#[rpc(name = "ibc_generateProof")]
	fn generate_proof(&self, height: u32, key: Vec<u8>) -> Result<Proof>;

	#[rpc(name = "ibc_queryLatestHeight")]
	fn query_latest_height(&self) -> Result<u32>;

	#[rpc(name = "ibc_queryBalanceWithAddress")]
	fn query_balance_with_address(&self, addr: String) -> Result<Coin>;

	#[rpc(name = "ibc_queryClientState")]
	fn query_client_state(
		&self,
		height: u32,
		src_client_Id: String,
	) -> Result<QueryClientStateResponse>;

	#[rpc(name = "ibc_queryClientConsensusState")]
	fn query_client_consensus_state(
		&self,
		client_id: String,
		client_height: ibc::Height,
	) -> Result<QueryConsensusStateResponse>;

	#[rpc(name = "ibc_queryUpgradedClient")]
	fn query_upgraded_client(&self, height: u32) -> Result<QueryClientStateResponse>;

	#[rpc(name = "ibc_queryUpgradedConnectionState")]
	fn query_upgraded_cons_state(&self, height: u32) -> Result<QueryConsensusStateResponse>;

	#[rpc(name = "ibc_queryClients")]
	fn query_clients(&self) -> Result<Vec<Vec<u8>>>;

	#[rpc(name = "ibc_queryConnection")]
	fn query_connection(
		&self,
		height: u32,
		connection_id: String,
	) -> Result<QueryConnectionResponse>;

	#[rpc(name = "ibc_queryConnections")]
	fn query_connections(&self) -> Result<QueryConnectionsResponse>;

	#[rpc(name = "ibc_queryConnectionUsingClient")]
	fn query_connection_using_client(
		&self,
		height: u32,
		client_id: String,
	) -> Result<QueryConnectionResponse>;

	#[rpc(name = "ibc_generateConnectionHandshakeProof")]
	fn generate_conn_handshake_proof(
		&self,
		height: u32,
		client_id: String,
		conn_id: String,
	) -> Result<ConnectionHandshakeProof>;

	#[rpc(name = "ibc_newClientState")]
	fn new_client_state(&self) -> Result<AnyClientState>;

	#[rpc(name = "ibc_queryChannel")]
	fn query_channel(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
	) -> Result<QueryChannelResponse>;

	#[rpc(name = "ibc_queryChannelClient")]
	fn query_channel_client(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
	) -> Result<AnyClientState>;

	#[rpc(name = "ibc_queryConnectionChannels")]
	fn query_connection_channels(
		&self,
		height: u32,
		connection_id: String,
	) -> Result<QueryChannelsResponse>;

	#[rpc(name = "ibc_queryChannels")]
	fn query_channels(&self) -> Result<QueryChannelsResponse>;

	#[rpc(name = "ibc_queryPacketCommitments")]
	fn query_packet_commitments(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
	) -> Result<QueryPacketCommitmentsResponse>;

	#[rpc(name = "ibc_queryPacketAcknowledgements")]
	fn query_packet_acknowledgements(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
	) -> Result<QueryPacketAcknowledgementsResponse>;

	#[rpc(name = "ibc_queryUnreceivedPackets")]
	fn query_unreceived_packets(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>>;

	#[rpc(name = "ibc_queryUnreceivedAcknowledgement")]
	fn query_unreceived_acknowledgements(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>>;

	#[rpc(name = "ibc_queryNextSeqRecv")]
	fn query_next_seq_recv(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
	) -> Result<QueryNextSequenceReceiveResponse>;

	#[rpc(name = "ibc_queryPacketCommitment")]
	fn query_packet_commitment(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse>;

	#[rpc(name = "ibc_queryPacketAcknowledgement")]
	fn query_packet_acknowledgement(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse>;

	#[rpc(name = "ibc_queryPacketReceipt")]
	fn query_packet_receipt(
		&self,
		height: u32,
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
		height: u32,
	) -> Result<QueryDenomTracesResponse>;
}

const RUNTIME_ERROR: i64 = 9000;

/// Converts a runtime trap into an RPC error.
fn runtime_error_into_rpc_error(err: impl std::fmt::Display) -> JsonRpcError {
	JsonRpcError {
		code: ErrorCode::ServerError(RUNTIME_ERROR),
		message: "Runtime trapped".into(),
		data: Some(err.to_string().into()),
	}
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

impl<C, Block, Transaction> IbcApi<<Block as BlockT>::Header, <Block as BlockT>::Hash>
	for IbcRpcHandler<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: IbcRuntimeApi<Block, <Block as BlockT>::Header>,
{
	fn generate_proof(&self, height: u32, key: Vec<u8>) -> Result<Proof> {
		Err(runtime_error_into_rpc_error("Unimplemented"))
	}

	fn query_latest_height(&self) -> Result<u32> {
		let api = self.client.runtime_api();
		let at = BlockId::Hash(self.client.info().best_hash);

		api.latest_height(&at)
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error fetching height"))
	}

	// Query balance of relayer on chain
	fn query_balance_with_address(&self, addr: String) -> Result<Coin> {
		Err(runtime_error_into_rpc_error("Unimplemented"))
	}

	fn query_client_state(
		&self,
		height: u32,
		client_id: String,
	) -> Result<QueryClientStateResponse> {
		let api = self.client.runtime_api();
		let block_hash = self
			.client
			.hash(height.into())
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		api.client_state(&at, client_id)
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error querying client state"))
	}

	fn query_client_consensus_state(
		&self,
		client_id: String,
		client_height: Height,
	) -> Result<QueryConsensusStateResponse> {
		let api = self.client.runtime_api();
		let at = BlockId::Hash(self.client.info().best_hash);
		let height = client_height.encode_vec().map_err(|e| runtime_error_into_rpc_error(e))?;
		api.client_consensus_state(&at, client_id, height)
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error querying client consensus state"))
	}
	// Look for client upgrade in ibc-rs
	fn query_upgraded_client(&self, height: u32) -> Result<QueryClientStateResponse> {
		Err(runtime_error_into_rpc_error("Unimplemented"))
	}

	fn query_upgraded_cons_state(&self, height: u32) -> Result<QueryConsensusStateResponse> {
		Err(runtime_error_into_rpc_error("Unimplemented"))
	}

	fn query_clients(&self) -> Result<Vec<Vec<u8>>> {
		let api = self.client.runtime_api();
		let at = BlockId::Hash(self.client.info().best_hash);

		let client_states = api.clients(&at).ok().flatten();
		match client_states {
			Some(res) => Ok(res),
			_ => Err(runtime_error_into_rpc_error("Failed to fetch client states")),
		}
	}

	fn query_connection(
		&self,
		height: u32,
		connection_id: String,
	) -> Result<QueryConnectionResponse> {
		let api = self.client.runtime_api();
		let block_hash = self
			.client
			.hash(height.into())
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);

		api.connection(&at, connection_id)
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Failed to fetch connection state"))
	}

	fn query_connections(&self) -> Result<QueryConnectionsResponse> {
		let api = self.client.runtime_api();

		let at = BlockId::Hash(self.client.info().best_hash);
		api.connections(&at)
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Failed to fetch connections"))
	}

	fn query_connection_using_client(
		&self,
		height: u32,
		client_id: String,
	) -> Result<QueryConnectionResponse> {
		let api = self.client.runtime_api();
		let block_hash = self
			.client
			.hash(height.into())
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		api.connection_using_client(&at, client_id)
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Failed to fetch connections"))
	}

	fn generate_conn_handshake_proof(
		&self,
		height: u32,
		client_id: String,
		conn_id: String,
	) -> Result<ConnectionHandshakeProof> {
		Err(runtime_error_into_rpc_error("Unimplemented"))
	}

	fn new_client_state(&self) -> Result<AnyClientState> {
		Err(runtime_error_into_rpc_error("Unimplemented"))
	}

	fn query_channel(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
	) -> Result<QueryChannelResponse> {
		let api = self.client.runtime_api();
		let block_hash = self
			.client
			.hash(height.into())
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		api.channel(&at, channel_id, port_id)
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Failed to fetch channel state"))
	}

	fn query_channel_client(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
	) -> Result<AnyClientState> {
		let api = self.client.runtime_api();
		let block_hash = self
			.client
			.hash(height.into())
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);

		api.channel_client(&at, channel_id, port_id)
			.ok()
			.flatten()
			.map(|state| AnyClientState::decode_vec(&state).ok())
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Failed to Client state for channel"))
	}

	fn query_connection_channels(
		&self,
		height: u32,
		connection_id: String,
	) -> Result<QueryChannelsResponse> {
		let api = self.client.runtime_api();
		let block_hash = self
			.client
			.hash(height.into())
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);

		api.connection_channels(&at, connection_id)
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Failed to fetch channels state for connection"))
	}

	fn query_channels(&self) -> Result<QueryChannelsResponse> {
		let api = self.client.runtime_api();
		let at = BlockId::Hash(self.client.info().best_hash);

		api.channels(&at)
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Failed to fetch channels"))
	}

	fn query_packet_commitments(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
	) -> Result<QueryPacketCommitmentsResponse> {
		let api = self.client.runtime_api();
		let block_hash = self
			.client
			.hash(height.into())
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);

		api.packet_commitments(&at, channel_id, port_id)
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Failed to fetch commitments"))
	}

	fn query_packet_acknowledgements(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
	) -> Result<QueryPacketAcknowledgementsResponse> {
		let api = self.client.runtime_api();
		let block_hash = self
			.client
			.hash(height.into())
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);

		api.packet_acknowledgements(&at, channel_id, port_id)
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Failed to fetch acknowledgements"))
	}

	fn query_unreceived_packets(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>> {
		Err(runtime_error_into_rpc_error("Unimplemented"))
	}

	fn query_unreceived_acknowledgements(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>> {
		Err(runtime_error_into_rpc_error("Unimplemented"))
	}

	fn query_next_seq_recv(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
	) -> Result<QueryNextSequenceReceiveResponse> {
		let api = self.client.runtime_api();
		let block_hash = self
			.client
			.hash(height.into())
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		api.next_seq_recv(&at, channel_id, port_id)
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error fetching next sequence"))
	}

	fn query_packet_commitment(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse> {
		let api = self.client.runtime_api();
		let block_hash = self
			.client
			.hash(height.into())
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		api.packet_commitment(&at, channel_id, port_id, seq)
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error fetching next sequence"))
	}

	fn query_packet_acknowledgement(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse> {
		let api = self.client.runtime_api();
		let block_hash = self
			.client
			.hash(height.into())
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		api.packet_acknowledgement(&at, channel_id, port_id, seq)
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error fetching next sequence"))
	}

	fn query_packet_receipt(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
		seq: u64,
	) -> Result<QueryPacketReceiptResponse> {
		let api = self.client.runtime_api();
		let block_hash = self
			.client
			.hash(height.into())
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		api.packet_receipt(&at, channel_id, port_id, seq)
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error fetching next sequence"))
	}

	fn query_denom_trace(&self, denom: String) -> Result<QueryDenomTraceResponse> {
		Err(runtime_error_into_rpc_error("Unimplemented"))
	}

	fn query_denom_traces(
		&self,
		offset: String,
		limit: u64,
		height: u32,
	) -> Result<QueryDenomTracesResponse> {
		Err(runtime_error_into_rpc_error("Unimplemented"))
	}
}
