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

use codec::Encode;
use ibc::core::{
	ics02_client::{client_consensus::AnyConsensusState, client_state::AnyClientState},
	ics03_connection::connection::ConnectionEnd,
	ics24_host::identifier::ConnectionId,
};
use std::{str::FromStr, sync::Arc};

use ibc::Height;
use ibc_primitives::{ConnectionHandshakeProof, Proof};
use ibc_proto::{
	cosmos::base::v1beta1::Coin,
	ibc::{
		applications::transfer::v1::{QueryDenomTraceResponse, QueryDenomTracesResponse},
		core::{
			channel::v1::{
				IdentifiedChannel, QueryChannelResponse, QueryChannelsResponse,
				QueryNextSequenceReceiveResponse, QueryPacketAcknowledgementResponse,
				QueryPacketAcknowledgementsResponse, QueryPacketCommitmentResponse,
				QueryPacketCommitmentsResponse, QueryPacketReceiptResponse,
			},
			client::v1::{
				IdentifiedClientState, QueryClientStateResponse, QueryConsensusStateResponse,
			},
			connection::v1::{
				IdentifiedConnection, QueryConnectionResponse, QueryConnectionsResponse,
			},
		},
	},
};
use ibc_runtime_api::IbcRuntimeApi;
use jsonrpc_core::{Error as JsonRpcError, ErrorCode, Result, Value};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT, Header as HeaderT},
};
use tendermint_proto::Protobuf;

#[derive(codec::Encode, codec::Decode, Serialize, Deserialize)]
pub struct ConnHandshakeProof {
	/// Protobuf encoded client state
	pub client_state: IdentifiedClientState,
	/// Trie proof for connection state, client state and consensus state
	pub proof: Vec<Vec<u8>>,
	pub height: ibc_proto::ibc::core::client::v1::Height,
}

/// IBC RPC methods.
#[rpc]
pub trait IbcApi<BlockNumber> {
	/// Generate proof for given key
	#[rpc(name = "ibc_generateProof")]
	fn generate_proof(&self, height: u32, key: Vec<Vec<u8>>) -> Result<Proof>;

	/// Query latest height
	#[rpc(name = "ibc_queryLatestHeight")]
	fn query_latest_height(&self) -> Result<BlockNumber>;

	/// Query balance of an address on chain
	#[rpc(name = "ibc_queryBalanceWithAddress")]
	fn query_balance_with_address(&self, addr: Vec<u8>) -> Result<Coin>;

	/// Query a client state
	#[rpc(name = "ibc_queryClientState")]
	fn query_client_state(
		&self,
		height: u32,
		src_client_id: String,
	) -> Result<QueryClientStateResponse>;

	/// Query client consensus state
	#[rpc(name = "ibc_queryClientConsensusState")]
	fn query_client_consensus_state(
		&self,
		client_id: String,
		revision_height: u64,
		revision_number: u64,
		latest_consensus_state: bool,
	) -> Result<QueryConsensusStateResponse>;

	/// Query upgraded client state
	#[rpc(name = "ibc_queryUpgradedClient")]
	fn query_upgraded_client(&self, height: u32) -> Result<QueryClientStateResponse>;

	/// Query upgraded consensus state for client
	#[rpc(name = "ibc_queryUpgradedConnectionState")]
	fn query_upgraded_cons_state(&self, height: u32) -> Result<QueryConsensusStateResponse>;

	/// Query all client states
	#[rpc(name = "ibc_queryClients")]
	fn query_clients(&self) -> Result<Vec<IdentifiedClientState>>;

	/// Query a connection state
	#[rpc(name = "ibc_queryConnection")]
	fn query_connection(
		&self,
		height: u32,
		connection_id: String,
	) -> Result<QueryConnectionResponse>;

	/// Query all connection states
	#[rpc(name = "ibc_queryConnections")]
	fn query_connections(&self) -> Result<QueryConnectionsResponse>;

	/// Query all connection states for associated client
	#[rpc(name = "ibc_queryConnectionUsingClient")]
	fn query_connection_using_client(
		&self,
		height: u32,
		client_id: String,
	) -> Result<IdentifiedConnection>;

	/// Generate proof for connection handshake
	#[rpc(name = "ibc_generateConnectionHandshakeProof")]
	fn generate_conn_handshake_proof(
		&self,
		height: u32,
		client_id: String,
		conn_id: String,
	) -> Result<ConnectionHandshakeProof>;

	/// Query a channel state
	#[rpc(name = "ibc_queryChannel")]
	fn query_channel(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
	) -> Result<QueryChannelResponse>;

	/// Query client state for channel and port id
	#[rpc(name = "ibc_queryChannelClient")]
	fn query_channel_client(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
	) -> Result<IdentifiedClientState>;

	/// Query all channel states for associated connection
	#[rpc(name = "ibc_queryConnectionChannels")]
	fn query_connection_channels(
		&self,
		height: u32,
		connection_id: String,
	) -> Result<QueryChannelsResponse>;

	/// Query all channel states
	#[rpc(name = "ibc_queryChannels")]
	fn query_channels(&self) -> Result<QueryChannelsResponse>;

	/// Query packet commitments
	#[rpc(name = "ibc_queryPacketCommitments")]
	fn query_packet_commitments(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
	) -> Result<QueryPacketCommitmentsResponse>;

	/// Query packet acknowledgements
	#[rpc(name = "ibc_queryPacketAcknowledgements")]
	fn query_packet_acknowledgements(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
	) -> Result<QueryPacketAcknowledgementsResponse>;

	/// Query unreceived packet commitments
	#[rpc(name = "ibc_queryUnreceivedPackets")]
	fn query_unreceived_packets(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>>;

	/// Query the unreceived acknowledgements
	#[rpc(name = "ibc_queryUnreceivedAcknowledgement")]
	fn query_unreceived_acknowledgements(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>>;

	/// Query next sequence to be received on channel
	#[rpc(name = "ibc_queryNextSeqRecv")]
	fn query_next_seq_recv(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
	) -> Result<QueryNextSequenceReceiveResponse>;

	/// Query packet commitment
	#[rpc(name = "ibc_queryPacketCommitment")]
	fn query_packet_commitment(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse>;

	/// Query packet acknowledgement
	#[rpc(name = "ibc_queryPacketAcknowledgement")]
	fn query_packet_acknowledgement(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse>;

	/// Query packet receipt
	#[rpc(name = "ibc_queryPacketReceipt")]
	fn query_packet_receipt(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
		seq: u64,
	) -> Result<QueryPacketReceiptResponse>;

	/// Query the denom trace for an ibc denom
	#[rpc(name = "ibc_queryDenomTrace")]
	fn query_denom_trace(&self, denom: String) -> Result<QueryDenomTraceResponse>;

	/// Query the denom traces for an ibc denoms matching offset
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
	/// A copy of the chain spec.
	pub chain_spec: Box<dyn sc_chain_spec::ChainSpec>,
	_marker: std::marker::PhantomData<B>,
}

impl<C, B> IbcRpcHandler<C, B> {
	/// Create new `IbcRpcHandler` with the given reference to the client.
	pub fn new(client: Arc<C>, chain_spec: Box<dyn sc_chain_spec::ChainSpec>) -> Self {
		Self { client, chain_spec, _marker: Default::default() }
	}
}

impl<C, Block> IbcApi<<<Block as BlockT>::Header as HeaderT>::Number> for IbcRpcHandler<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: IbcRuntimeApi<Block>,
{
	fn generate_proof(&self, height: u32, keys: Vec<Vec<u8>>) -> Result<Proof> {
		let api = self.client.runtime_api();
		let block_hash = self
			.client
			.hash(height.into())
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error retreiving block hash"))?;
		let at = BlockId::Hash(block_hash);
		api.generate_proof(&at, keys)
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error generating proof"))
	}

	fn query_latest_height(&self) -> Result<<<Block as BlockT>::Header as HeaderT>::Number> {
		if let Ok(Some(height)) = self.client.number(self.client.info().best_hash) {
			Ok(height)
		} else {
			Err(runtime_error_into_rpc_error("Could not get latest height"))
		}
	}

	fn query_balance_with_address(&self, addr: Vec<u8>) -> Result<Coin> {
		let api = self.client.runtime_api();
		let at = BlockId::Hash(self.client.info().best_hash);
		let denom =
			match self.chain_spec.properties().get("tokenSymbol").cloned().unwrap_or_default() {
				Value::String(symbol) => symbol,
				_ => "".to_string(),
			};

		match api.query_balance_with_address(&at, addr).ok().flatten() {
			Some(amt) => Ok(Coin { denom, amount: format!("{}", amt) }),
			None => Err(runtime_error_into_rpc_error("Error querying balance")),
		}
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
		let result: ibc_primitives::QueryClientStateResponse = api
			.client_state(&at, client_id.as_bytes().to_vec())
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error querying client state"))?;
		let client_state = AnyClientState::decode_vec(&result.client_state)
			.map_err(|_| runtime_error_into_rpc_error("Error querying client state"))?;
		Ok(QueryClientStateResponse {
			client_state: Some(client_state.into()),
			proof: result.proof.encode(),
			proof_height: Some(ibc_proto::ibc::core::client::v1::Height {
				revision_number: 0,
				revision_height: result.height,
			}),
		})
	}

	fn query_client_consensus_state(
		&self,
		client_id: String,
		revision_height: u64,
		revision_number: u64,
		latest_cs: bool,
	) -> Result<QueryConsensusStateResponse> {
		let api = self.client.runtime_api();
		let at = BlockId::Hash(self.client.info().best_hash);
		let client_height = ibc::Height::new(revision_number, revision_height);
		let height = client_height.encode_vec().map_err(|e| runtime_error_into_rpc_error(e))?;
		let result: ibc_primitives::QueryConsensusStateResponse = api
			.client_consensus_state(&at, client_id.as_bytes().to_vec(), height, latest_cs)
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error querying client consensus state"))?;
		let consensus_state = AnyConsensusState::decode_vec(&result.consensus_state)
			.map_err(|| runtime_error_into_rpc_error("Error querying client consensus state"))?;
		Ok(QueryConsensusStateResponse {
			consensus_state: Some(consensus_state.into()),
			proof: result.proof.encode(),
			proof_height: Some(ibc_proto::ibc::core::client::v1::Height {
				revision_number: 0,
				revision_height: result.height,
			}),
		})
	}
	// TODO: Not required in first version
	fn query_upgraded_client(&self, _height: u32) -> Result<QueryClientStateResponse> {
		Err(runtime_error_into_rpc_error("Unimplemented"))
	}

	fn query_upgraded_cons_state(&self, _height: u32) -> Result<QueryConsensusStateResponse> {
		Err(runtime_error_into_rpc_error("Unimplemented"))
	}

	fn query_clients(&self) -> Result<Vec<IdentifiedClientState>> {
		let api = self.client.runtime_api();
		let at = BlockId::Hash(self.client.info().best_hash);

		let client_states: Option<Vec<(Vec<u8>, Vec<u8>)>> = api.clients(&at).ok().flatten();
		match client_states {
			Some((client_id, client_states)) => client_states.into_iter().map(|state| {
				let client_state = AnyClientState::decode_vec(&state)
					.map_err(|_| runtime_error_into_rpc_error("Failed to decode client state"))?;
				Ok(IdentifiedClientState {
					client_id: String::from_utf8(client_id)
						.map_err(|_| runtime_error_into_rpc_error("Failed to decode client id"))?,
					client_state: Some(client_state.into()),
				})
			}),
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

		let result: ibc_primitives::QueryConnectionResponse = api
			.connection(&at, connection_id.as_bytes().to_vec())
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Failed to fetch connection state"))?;
		let connection_end =
			ibc::core::ics03_connection::connection::ConnectionEnd::decode_vec(&result.connection)
				.map_err(|_| runtime_error_into_rpc_error("Failed to decode connection end"))?;
		Ok(QueryConnectionResponse {
			connection: Some(connection_end.into()),
			proof: result.proof.encode(),
			proof_height: Some(ibc_proto::ibc::core::client::v1::Height {
				revision_number: 0,
				revision_height: result.height,
			}),
		})
	}

	fn query_connections(&self) -> Result<QueryConnectionsResponse> {
		let api = self.client.runtime_api();

		let at = BlockId::Hash(self.client.info().best_hash);
		let result: ibc_primitives::QueryConnectionsResponse = api
			.connections(&at)
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Failed to fetch connections"))?;

		let connections = result
			.connections
			.into_iter()
			.map(|identified_connection| {
				let connection_id = String::from_utf8(identified_connection.connection_id)
					.map_err(|| runtime_error_into_rpc_error("Failed to decode connection id"))?;
				let connection_id = ConnectionId::from_str(&connection_id)
					.map_err(|| runtime_error_into_rpc_error("Failed to decode connection id"))?;
				let connection_end = ConnectionEnd::decode_vec(
					&identified_connection.connection_end,
				)
				.map_err(|| runtime_error_into_rpc_error("Failed to decode connection end"))?;
				let identified_connection =
					ibc::core::ics03_connection::connection::IdentifiedConnectionEnd::new(
						connection_id,
						connection_end,
					);
				let identified_connection: IdentifiedConnection = identified_connection.into();
				Ok(identified_connection)
			})
			.collect::<Result<Vec<_>>>()?;
		Ok(QueryConnectionsResponse {
			connections,
			pagination: None,
			height: Some(ibc_proto::ibc::core::client::v1::Height {
				revision_number: 0,
				revision_height: result.height,
			}),
		})
	}

	fn query_connection_using_client(
		&self,
		height: u32,
		client_id: String,
	) -> Result<IdentifiedConnection> {
		let api = self.client.runtime_api();
		let block_hash = self
			.client
			.hash(height.into())
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		let result: ibc_primitives::IdentifiedConnection = api
			.connection_using_client(&at, client_id.as_bytes().to_vec())
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Failed to fetch connections"))?;
		let connection_id = String::from_utf8(result.connection_id)
			.map_err(|| runtime_error_into_rpc_error("Failed to decode connection id"))?;
		let connection_id = ConnectionId::from_str(&connection_id)
			.map_err(|| runtime_error_into_rpc_error("Failed to decode connection id"))?;
		let connection_end = ConnectionEnd::decode_vec(&result.connection_end)
			.map_err(|| runtime_error_into_rpc_error("Failed to decode connection end"))?;
		let identified_connection =
			ibc::core::ics03_connection::connection::IdentifiedConnectionEnd::new(
				connection_id,
				connection_end,
			);
		let identified_connection: IdentifiedConnection = identified_connection.into();
		Ok(identified_connection)
	}

	fn generate_conn_handshake_proof(
		&self,
		height: u32,
		client_id: String,
		conn_id: String,
	) -> Result<ConnHandshakeProof> {
		let api = self.client.runtime_api();
		let block_hash = self
			.client
			.hash(height.into())
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		let result: ConnectionHandshakeProof = api
			.connection_handshake_proof(
				&at,
				client_id.as_bytes().to_vec(),
				conn_id.as_bytes().to_vec(),
			)
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error generating handshake proof"))?;
		let client_state = AnyClientState::decode_vec(&state)
			.map_err(|_| runtime_error_into_rpc_error("Failed to decode client state"))?;
		Ok(ConnHandshakeProof {
			client_state: IdentifiedClientState {
				client_id,
				client_state: Some(client_state.into()),
			},
			proof: result.proof,
			height: ibc_proto::ibc::core::client::v1::Height {
				revision_number: 0,
				revision_height: result.height,
			},
		})
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
		api.channel(&at, channel_id.as_bytes().to_vec(), port_id.as_bytes().to_vec())
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Failed to fetch channel state"))
	}

	fn query_channel_client(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
	) -> Result<IdentifiedClientState> {
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
		let api = self.client.runtime_api();
		let block_hash = self
			.client
			.hash(height.into())
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error retreiving block hash"))?;
		let at = BlockId::Hash(block_hash);

		api.unreceived_packets(&at, channel_id, port_id, seqs)
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Failed to unreceived packet sequences"))
	}

	fn query_unreceived_acknowledgements(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>> {
		let api = self.client.runtime_api();
		let block_hash = self
			.client
			.hash(height.into())
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Error retreiving block hash"))?;
		let at = BlockId::Hash(block_hash);

		api.unreceived_acknowledgements(&at, channel_id, port_id, seqs)
			.ok()
			.flatten()
			.ok_or(runtime_error_into_rpc_error("Failed to unreceived packet sequences"))
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

	fn query_denom_trace(&self, _denom: String) -> Result<QueryDenomTraceResponse> {
		Err(runtime_error_into_rpc_error("Unimplemented"))
	}

	fn query_denom_traces(
		&self,
		_offset: String,
		_limit: u64,
		_height: u32,
	) -> Result<QueryDenomTracesResponse> {
		Err(runtime_error_into_rpc_error("Unimplemented"))
	}
}
