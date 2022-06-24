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

//! IBC RPC Implementation.

use codec::Encode;
use ibc::core::{
	ics02_client::{client_consensus::AnyConsensusState, client_state::AnyClientState},
	ics03_connection::connection::ConnectionEnd,
	ics04_channel::channel::{ChannelEnd, IdentifiedChannelEnd},
	ics24_host::identifier::{ChannelId, ConnectionId, PortId},
};

use std::{str::FromStr, sync::Arc};

use ibc_proto::{
	cosmos::base::v1beta1::Coin,
	ibc::{
		applications::transfer::v1::{QueryDenomTraceResponse, QueryDenomTracesResponse},
		core::{
			channel::v1::{
				Packet, PacketState, QueryChannelResponse, QueryChannelsResponse,
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
use jsonrpsee::{
	core::{Error as RpcError, RpcResult as Result},
	proc_macros::rpc,
	types::{error::CallError, ErrorObject},
};
use pallet_ibc::events::IbcEvent;
use sc_chain_spec::Properties;
use serde::{Deserialize, Serialize};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::BlockId,
	traits::{BlakeTwo256, Block as BlockT, Header as HeaderT},
};
use sp_trie::TrieMut;
use tendermint_proto::Protobuf;

/// Connection handshake proof
#[derive(Serialize, Deserialize)]
pub struct ConnHandshakeProof {
	/// Protobuf encoded client state
	pub client_state: IdentifiedClientState,
	/// Trie proof for connection state, client state and consensus state
	pub proof: Vec<u8>,
	/// Proof height
	pub height: ibc_proto::ibc::core::client::v1::Height,
}

/// Proof for a set of keys
#[derive(Serialize, Deserialize)]
pub struct Proof {
	/// Trie proof
	pub proof: Vec<u8>,
	/// Height at which proof was recovered
	pub height: ibc_proto::ibc::core::client::v1::Height,
}

/// Generate trie proof given inputs to trie and keys
pub fn generate_raw_proof(inputs: Vec<(Vec<u8>, Vec<u8>)>, keys: Vec<Vec<u8>>) -> Result<Vec<u8>> {
	let keys = keys.iter().collect::<Vec<_>>();
	let mut db = sp_trie::MemoryDB::<BlakeTwo256>::default();
	let root = {
		let mut root = sp_core::H256::default();
		let mut trie =
			<sp_trie::TrieDBMut<sp_trie::LayoutV0<BlakeTwo256>>>::new(&mut db, &mut root);
		for (key, value) in inputs {
			trie.insert(&*key, &*value)
				.map_err(|_| runtime_error_into_rpc_error("Failed to generate proof"))?;
		}
		*trie.root()
	};

	sp_trie::generate_trie_proof::<sp_trie::LayoutV0<BlakeTwo256>, _, _, _>(&db, root, keys)
		.map(|proof| proof.encode())
		.map_err(|_| runtime_error_into_rpc_error("Failed to generate proof"))
}

/// IBC RPC methods.
#[rpc(client, server)]
pub trait IbcApi<BlockNumber, Hash> {
	/// Query packet data
	#[method(name = "ibc_queryPackets")]
	fn query_packets(
		&self,
		channel_id: String,
		port_id: String,
		seqs: Vec<u64>,
	) -> Result<Vec<Packet>>;
	/// Generate proof for given key
	#[method(name = "ibc_queryProof")]
	fn query_proof(&self, height: u32, keys: Vec<Vec<u8>>) -> Result<Proof>;

	/// Query latest height
	#[method(name = "ibc_queryLatestHeight")]
	fn query_latest_height(&self) -> Result<BlockNumber>;

	/// Query balance of an address on chain, addr should be a valid hexadecimal or SS58 string,
	/// representing the account id.
	#[method(name = "ibc_queryBalanceWithAddress")]
	fn query_balance_with_address(&self, addr: String) -> Result<Coin>;

	/// Query a client state
	#[method(name = "ibc_queryClientState")]
	fn query_client_state(
		&self,
		height: u32,
		src_client_id: String,
	) -> Result<QueryClientStateResponse>;

	/// Query localchain consensus state
	#[method(name = "ibc_queryConsensusState")]
	fn query_consensus_state(&self, height: u32) -> Result<QueryConsensusStateResponse>;

	/// Query client consensus state
	#[method(name = "ibc_queryClientConsensusState")]
	fn query_client_consensus_state(
		&self,
		client_id: String,
		revision_height: u64,
		revision_number: u64,
		latest_consensus_state: bool,
	) -> Result<QueryConsensusStateResponse>;

	/// Query upgraded client state
	#[method(name = "ibc_queryUpgradedClient")]
	fn query_upgraded_client(&self, height: u32) -> Result<QueryClientStateResponse>;

	/// Query upgraded consensus state for client
	#[method(name = "ibc_queryUpgradedConnectionState")]
	fn query_upgraded_cons_state(&self, height: u32) -> Result<QueryConsensusStateResponse>;

	/// Query all client states
	#[method(name = "ibc_queryClients")]
	fn query_clients(&self) -> Result<Vec<IdentifiedClientState>>;

	/// Query a connection state
	#[method(name = "ibc_queryConnection")]
	fn query_connection(
		&self,
		height: u32,
		connection_id: String,
	) -> Result<QueryConnectionResponse>;

	/// Query all connection states
	#[method(name = "ibc_queryConnections")]
	fn query_connections(&self) -> Result<QueryConnectionsResponse>;

	/// Query all connection states for associated client
	#[method(name = "ibc_queryConnectionUsingClient")]
	fn query_connection_using_client(
		&self,
		height: u32,
		client_id: String,
	) -> Result<Vec<IdentifiedConnection>>;

	/// Generate proof for connection handshake
	#[method(name = "ibc_generateConnectionHandshakeProof")]
	fn generate_conn_handshake_proof(
		&self,
		height: u32,
		client_id: String,
		conn_id: String,
	) -> Result<ConnHandshakeProof>;

	/// Query a channel state
	#[method(name = "ibc_queryChannel")]
	fn query_channel(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
	) -> Result<QueryChannelResponse>;

	/// Query client state for channel and port id
	#[method(name = "ibc_queryChannelClient")]
	fn query_channel_client(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
	) -> Result<IdentifiedClientState>;

	/// Query all channel states for associated connection
	#[method(name = "ibc_queryConnectionChannels")]
	fn query_connection_channels(
		&self,
		height: u32,
		connection_id: String,
	) -> Result<QueryChannelsResponse>;

	/// Query all channel states
	#[method(name = "ibc_queryChannels")]
	fn query_channels(&self) -> Result<QueryChannelsResponse>;

	/// Query packet commitments
	#[method(name = "ibc_queryPacketCommitments")]
	fn query_packet_commitments(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
	) -> Result<QueryPacketCommitmentsResponse>;

	/// Query packet acknowledgements
	#[method(name = "ibc_queryPacketAcknowledgements")]
	fn query_packet_acknowledgements(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
	) -> Result<QueryPacketAcknowledgementsResponse>;

	/// Query unreceived packet commitments
	#[method(name = "ibc_queryUnreceivedPackets")]
	fn query_unreceived_packets(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>>;

	/// Query the unreceived acknowledgements
	#[method(name = "ibc_queryUnreceivedAcknowledgement")]
	fn query_unreceived_acknowledgements(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>>;

	/// Query next sequence to be received on channel
	#[method(name = "ibc_queryNextSeqRecv")]
	fn query_next_seq_recv(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
	) -> Result<QueryNextSequenceReceiveResponse>;

	/// Query packet commitment
	#[method(name = "ibc_queryPacketCommitment")]
	fn query_packet_commitment(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse>;

	/// Query packet acknowledgement
	#[method(name = "ibc_queryPacketAcknowledgement")]
	fn query_packet_acknowledgement(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse>;

	/// Query packet receipt
	#[method(name = "ibc_queryPacketReceipt")]
	fn query_packet_receipt(
		&self,
		height: u32,
		channel_id: String,
		port_id: String,
		seq: u64,
	) -> Result<QueryPacketReceiptResponse>;

	/// Query the denom trace for an ibc denom
	#[method(name = "ibc_queryDenomTrace")]
	fn query_denom_trace(&self, denom: String) -> Result<QueryDenomTraceResponse>;

	/// Query the denom traces for ibc denoms matching offset
	#[method(name = "ibc_queryDenomTraces")]
	fn query_denom_traces(
		&self,
		offset: String,
		limit: u64,
		height: u32,
	) -> Result<QueryDenomTracesResponse>;

	/// Query newly created clients in block
	#[method(name = "ibc_queryNewlyCreatedClients")]
	fn query_newly_created_clients(&self, block_hash: Hash) -> Result<Vec<IdentifiedClientState>>;
}

/// Converts a runtime trap into an RPC error.
fn runtime_error_into_rpc_error(e: impl std::fmt::Display) -> RpcError {
	RpcError::Call(CallError::Custom(ErrorObject::owned(
		9876, // no real reason for this value
		"Something wrong",
		Some(format!("{}", e)),
	)))
}

/// An implementation of IBC specific RPC methods.
pub struct IbcRpcHandler<C, B> {
	client: Arc<C>,
	/// A copy of the chain properties.
	pub chain_props: Properties,
	_marker: std::marker::PhantomData<B>,
}

impl<C, B> IbcRpcHandler<C, B> {
	/// Create new `IbcRpcHandler` with the given reference to the client.
	pub fn new(client: Arc<C>, chain_props: Properties) -> Self {
		Self { client, chain_props, _marker: Default::default() }
	}
}

impl<C, Block> IbcApiServer<<<Block as BlockT>::Header as HeaderT>::Number, Block::Hash>
	for IbcRpcHandler<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: IbcRuntimeApi<Block>,
{
	fn query_packets(
		&self,
		channel_id: String,
		port_id: String,
		seqs: Vec<u64>,
	) -> Result<Vec<Packet>> {
		let api = self.client.runtime_api();
		let at = BlockId::Hash(self.client.info().best_hash);
		let packets: Vec<ibc_primitives::OffchainPacketType> = api
			.query_packets(&at, channel_id.as_bytes().to_vec(), port_id.as_bytes().to_vec(), seqs)
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Error fetching packets"))?;

		packets
			.into_iter()
			.map(|packet| {
				Ok(Packet {
					sequence: packet.sequence,
					source_port: String::from_utf8(packet.source_port).map_err(|_| {
						runtime_error_into_rpc_error("Failed to decode source port")
					})?,
					source_channel: String::from_utf8(packet.source_channel).map_err(|_| {
						runtime_error_into_rpc_error("Failed to decode source channel")
					})?,
					destination_port: String::from_utf8(packet.destination_port).map_err(|_| {
						runtime_error_into_rpc_error("Failed to decode destination port")
					})?,
					destination_channel: String::from_utf8(packet.destination_channel).map_err(
						|_| runtime_error_into_rpc_error("Failed to decode destination channel"),
					)?,
					data: packet.data,
					timeout_height: Some(ibc_proto::ibc::core::client::v1::Height {
						revision_number: packet.timeout_height.0,
						revision_height: packet.timeout_height.1,
					}),
					timeout_timestamp: packet.timeout_timestamp,
				})
			})
			.collect()
	}

	fn query_proof(&self, height: u32, keys: Vec<Vec<u8>>) -> Result<Proof> {
		let api = self.client.runtime_api();
		let block_hash = self
			.client
			.hash(height.into())
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Error retreiving block hash"))?;
		let at = BlockId::Hash(block_hash);
		let para_id = api
			.para_id(&at)
			.map_err(|_| runtime_error_into_rpc_error("Error getting para id"))?;
		let inputs: Vec<(Vec<u8>, Vec<u8>)> = api
			.get_trie_inputs(&at)
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Error getting trie inputs"))?;
		Ok(Proof {
			proof: generate_raw_proof(inputs, keys)?,
			height: ibc_proto::ibc::core::client::v1::Height {
				revision_number: para_id.into(),
				revision_height: height as u64,
			},
		})
	}

	fn query_latest_height(&self) -> Result<<<Block as BlockT>::Header as HeaderT>::Number> {
		if let Ok(Some(height)) = self.client.number(self.client.info().best_hash) {
			Ok(height)
		} else {
			Err(runtime_error_into_rpc_error("Could not get latest height"))
		}
	}

	fn query_balance_with_address(&self, addr: String) -> Result<Coin> {
		let api = self.client.runtime_api();
		let at = BlockId::Hash(self.client.info().best_hash);
		let denom = format!("{}", self.chain_props.get("tokenSymbol").cloned().unwrap_or_default());

		match api.query_balance_with_address(&at, addr.as_bytes().to_vec()).ok().flatten() {
			Some(amt) => Ok(Coin {
				denom,
				amount: serde_json::to_string(&sp_core::U256::from(amt)).unwrap_or_default(),
			}),
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
			.ok_or_else(|| runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		let para_id = api
			.para_id(&at)
			.map_err(|_| runtime_error_into_rpc_error("Error getting para id"))?;
		let result: ibc_primitives::QueryClientStateResponse = api
			.client_state(&at, client_id.as_bytes().to_vec())
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Error querying client state"))?;
		let inputs: Vec<(Vec<u8>, Vec<u8>)> = api
			.get_trie_inputs(&at)
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Error getting trie inputs"))?;
		let client_state = AnyClientState::decode_vec(&result.client_state)
			.map_err(|_| runtime_error_into_rpc_error("Error querying client state"))?;
		Ok(QueryClientStateResponse {
			client_state: Some(client_state.into()),
			proof: generate_raw_proof(inputs, vec![result.trie_key])?,
			proof_height: Some(ibc_proto::ibc::core::client::v1::Height {
				revision_number: para_id.into(),
				revision_height: result.height,
			}),
		})
	}

	fn query_consensus_state(&self, height: u32) -> Result<QueryConsensusStateResponse> {
		let api = self.client.runtime_api();
		let block_hash = self
			.client
			.hash(height.into())
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		let result: Vec<u8> =
			api.host_consensus_state(&at, height).ok().flatten().ok_or_else(|| {
				runtime_error_into_rpc_error("Error querying host consensus state")
			})?;
		let consensus_state = AnyConsensusState::decode_vec(&result)
			.map_err(|_| runtime_error_into_rpc_error("Error querying host consensus state"))?;

		Ok(QueryConsensusStateResponse {
			consensus_state: Some(consensus_state.into()),
			proof: vec![],
			proof_height: None,
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
		let height = client_height.encode_vec();
		let para_id = api
			.para_id(&at)
			.map_err(|_| runtime_error_into_rpc_error("Error getting para id"))?;
		let result: ibc_primitives::QueryConsensusStateResponse = api
			.client_consensus_state(&at, client_id.as_bytes().to_vec(), height, latest_cs)
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Error querying client consensus state"))?;
		let consensus_state = AnyConsensusState::decode_vec(&result.consensus_state)
			.map_err(|_| runtime_error_into_rpc_error("Error querying client consensus state"))?;
		let inputs: Vec<(Vec<u8>, Vec<u8>)> = api
			.get_trie_inputs(&at)
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Error getting trie inputs"))?;
		Ok(QueryConsensusStateResponse {
			consensus_state: Some(consensus_state.into()),
			proof: generate_raw_proof(inputs, vec![result.trie_key])?,
			proof_height: Some(ibc_proto::ibc::core::client::v1::Height {
				revision_number: para_id.into(),
				revision_height: result.height,
			}),
		})
	}
	// TODO: Unimplemented
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
			Some(client_states) => client_states
				.into_iter()
				.map(|(client_id, client_state)| {
					let client_state = AnyClientState::decode_vec(&client_state).map_err(|_| {
						runtime_error_into_rpc_error("Failed to decode client state")
					})?;
					Ok(IdentifiedClientState {
						client_id: String::from_utf8(client_id).map_err(|_| {
							runtime_error_into_rpc_error("Failed to decode client id")
						})?,
						client_state: Some(client_state.into()),
					})
				})
				.collect(),
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
			.ok_or_else(|| runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		let para_id = api
			.para_id(&at)
			.map_err(|_| runtime_error_into_rpc_error("Error getting para id"))?;
		let result: ibc_primitives::QueryConnectionResponse = api
			.connection(&at, connection_id.as_bytes().to_vec())
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Failed to fetch connection state"))?;
		let connection_end =
			ibc::core::ics03_connection::connection::ConnectionEnd::decode_vec(&result.connection)
				.map_err(|_| runtime_error_into_rpc_error("Failed to decode connection end"))?;
		let inputs: Vec<(Vec<u8>, Vec<u8>)> = api
			.get_trie_inputs(&at)
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Error getting trie inputs"))?;
		Ok(QueryConnectionResponse {
			connection: Some(connection_end.into()),
			proof: generate_raw_proof(inputs, vec![result.trie_key])?,
			proof_height: Some(ibc_proto::ibc::core::client::v1::Height {
				revision_number: para_id.into(),
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
			.ok_or_else(|| runtime_error_into_rpc_error("Failed to fetch connections"))?;
		let para_id = api
			.para_id(&at)
			.map_err(|_| runtime_error_into_rpc_error("Error getting para id"))?;
		let connections = result
			.connections
			.into_iter()
			.map(|identified_connection| {
				let connection_id = String::from_utf8(identified_connection.connection_id)
					.map_err(|_| runtime_error_into_rpc_error("Failed to decode connection id"))?;
				let connection_id = ConnectionId::from_str(&connection_id)
					.map_err(|_| runtime_error_into_rpc_error("Failed to decode connection id"))?;
				let connection_end = ConnectionEnd::decode_vec(
					&identified_connection.connection_end,
				)
				.map_err(|_| runtime_error_into_rpc_error("Failed to decode connection end"))?;
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
				revision_number: para_id.into(),
				revision_height: result.height,
			}),
		})
	}

	fn query_connection_using_client(
		&self,
		height: u32,
		client_id: String,
	) -> Result<Vec<IdentifiedConnection>> {
		let api = self.client.runtime_api();
		let block_hash = self
			.client
			.hash(height.into())
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		let result: Vec<ibc_primitives::IdentifiedConnection> = api
			.connection_using_client(&at, client_id.as_bytes().to_vec())
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Failed to fetch connections"))?;
		result
			.into_iter()
			.map(|ident_conn| {
				let connection_id = String::from_utf8(ident_conn.connection_id)
					.map_err(|_| runtime_error_into_rpc_error("Failed to decode connection id"))?;
				let connection_id = ConnectionId::from_str(&connection_id)
					.map_err(|_| runtime_error_into_rpc_error("Failed to decode connection id"))?;
				let connection_end = ConnectionEnd::decode_vec(&ident_conn.connection_end)
					.map_err(|_| runtime_error_into_rpc_error("Failed to decode connection end"))?;
				let identified_connection =
					ibc::core::ics03_connection::connection::IdentifiedConnectionEnd::new(
						connection_id,
						connection_end,
					);
				let identified_connection: IdentifiedConnection = identified_connection.into();
				Ok(identified_connection)
			})
			.collect::<Result<Vec<_>>>()
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
			.ok_or_else(|| runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		let para_id = api
			.para_id(&at)
			.map_err(|_| runtime_error_into_rpc_error("Error getting para id"))?;
		let inputs: Vec<(Vec<u8>, Vec<u8>)> = api
			.get_trie_inputs(&at)
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Error getting trie inputs"))?;
		let result: ibc_primitives::ConnectionHandshake = api
			.connection_handshake(&at, client_id.as_bytes().to_vec(), conn_id.as_bytes().to_vec())
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Error getting trie inputs"))?;

		let client_state = AnyClientState::decode_vec(&result.client_state)
			.map_err(|_| runtime_error_into_rpc_error("Failed to decode client state"))?;
		Ok(ConnHandshakeProof {
			client_state: IdentifiedClientState {
				client_id,
				client_state: Some(client_state.into()),
			},
			proof: generate_raw_proof(inputs, result.trie_keys)?,
			height: ibc_proto::ibc::core::client::v1::Height {
				revision_number: para_id.into(),
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
			.ok_or_else(|| runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		let para_id = api
			.para_id(&at)
			.map_err(|_| runtime_error_into_rpc_error("Error getting para id"))?;
		let result: ibc_primitives::QueryChannelResponse = api
			.channel(&at, channel_id.as_bytes().to_vec(), port_id.as_bytes().to_vec())
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Failed to fetch channel state"))?;
		let channel = ibc::core::ics04_channel::channel::ChannelEnd::decode_vec(&result.channel)
			.map_err(|_| runtime_error_into_rpc_error("Failed to decode channel state"))?;
		let inputs: Vec<(Vec<u8>, Vec<u8>)> = api
			.get_trie_inputs(&at)
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Error getting trie inputs"))?;
		Ok(QueryChannelResponse {
			channel: Some(channel.into()),
			proof: generate_raw_proof(inputs, vec![result.trie_key])?,
			proof_height: Some(ibc_proto::ibc::core::client::v1::Height {
				revision_number: para_id.into(),
				revision_height: result.height,
			}),
		})
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
			.ok_or_else(|| runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		let result: ibc_primitives::IdentifiedClientState = api
			.channel_client(&at, channel_id.as_bytes().to_vec(), port_id.as_bytes().to_vec())
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Failed to Client state for channel"))?;

		let client_state = AnyClientState::decode_vec(&result.client_state)
			.map_err(|_| runtime_error_into_rpc_error("Failed to decode cleint state"))?;
		Ok(IdentifiedClientState {
			client_id: String::from_utf8(result.client_id)
				.map_err(|_| runtime_error_into_rpc_error("Failed to decode client id"))?,
			client_state: Some(client_state.into()),
		})
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
			.ok_or_else(|| runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		let para_id = api
			.para_id(&at)
			.map_err(|_| runtime_error_into_rpc_error("Error getting para id"))?;
		let result: ibc_primitives::QueryChannelsResponse = api
			.connection_channels(&at, connection_id.as_bytes().to_vec())
			.ok()
			.flatten()
			.ok_or_else(|| {
				runtime_error_into_rpc_error("Failed to fetch channels state for connection")
			})?;
		let channels = result
			.channels
			.into_iter()
			.map(|temp| {
				let port_id = PortId::from_str(
					&String::from_utf8(temp.port_id)
						.map_err(|_| runtime_error_into_rpc_error("Failed to decode port id"))?,
				)
				.map_err(|_| runtime_error_into_rpc_error("Failed to decode port id"))?;
				let channel_id = ChannelId::from_str(
					&String::from_utf8(temp.channel_id)
						.map_err(|_| runtime_error_into_rpc_error("Failed to decode port id"))?,
				)
				.map_err(|_| runtime_error_into_rpc_error("Failed to decode port id"))?;
				let channel_end = ChannelEnd::decode_vec(&temp.channel_end)
					.map_err(|_| runtime_error_into_rpc_error("Failed to decode port id"))?;
				let identified_channel =
					IdentifiedChannelEnd::new(port_id, channel_id, channel_end);
				let identified_channel: ibc_proto::ibc::core::channel::v1::IdentifiedChannel =
					identified_channel.into();
				Ok(identified_channel)
			})
			.collect::<Result<Vec<_>>>()?;

		Ok(QueryChannelsResponse {
			channels,
			pagination: None,
			height: Some(ibc_proto::ibc::core::client::v1::Height {
				revision_number: para_id.into(),
				revision_height: result.height,
			}),
		})
	}

	fn query_channels(&self) -> Result<QueryChannelsResponse> {
		let api = self.client.runtime_api();
		let at = BlockId::Hash(self.client.info().best_hash);
		let para_id = api
			.para_id(&at)
			.map_err(|_| runtime_error_into_rpc_error("Error getting para id"))?;
		let result: ibc_primitives::QueryChannelsResponse = api
			.channels(&at)
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Failed to fetch channels"))?;
		let channels = result
			.channels
			.into_iter()
			.map(|temp| {
				let port_id = PortId::from_str(
					&String::from_utf8(temp.port_id)
						.map_err(|_| runtime_error_into_rpc_error("Failed to decode port id"))?,
				)
				.map_err(|_| runtime_error_into_rpc_error("Failed to decode port id"))?;
				let channel_id = ChannelId::from_str(
					&String::from_utf8(temp.channel_id)
						.map_err(|_| runtime_error_into_rpc_error("Failed to decode port id"))?,
				)
				.map_err(|_| runtime_error_into_rpc_error("Failed to decode port id"))?;
				let channel_end = ChannelEnd::decode_vec(&temp.channel_end)
					.map_err(|_| runtime_error_into_rpc_error("Failed to decode port id"))?;
				let identified_channel =
					IdentifiedChannelEnd::new(port_id, channel_id, channel_end);
				let identified_channel: ibc_proto::ibc::core::channel::v1::IdentifiedChannel =
					identified_channel.into();
				Ok(identified_channel)
			})
			.collect::<Result<Vec<_>>>()?;

		Ok(QueryChannelsResponse {
			channels,
			pagination: None,
			height: Some(ibc_proto::ibc::core::client::v1::Height {
				revision_number: para_id.into(),
				revision_height: result.height,
			}),
		})
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
			.ok_or_else(|| runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		let para_id = api
			.para_id(&at)
			.map_err(|_| runtime_error_into_rpc_error("Error getting para id"))?;
		let result: ibc_primitives::QueryPacketCommitmentsResponse = api
			.packet_commitments(&at, channel_id.as_bytes().to_vec(), port_id.as_bytes().to_vec())
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Failed to fetch commitments"))?;
		let commitments = result
			.commitments
			.into_iter()
			.map(|packet_state| {
				let port_id = String::from_utf8(packet_state.port_id)
					.map_err(|_| runtime_error_into_rpc_error("Failed to decode port id"))?;
				let channel_id = String::from_utf8(packet_state.channel_id)
					.map_err(|_| runtime_error_into_rpc_error("Failed to decode port id"))?;
				Ok(PacketState {
					port_id,
					channel_id,
					sequence: packet_state.sequence,
					data: packet_state.data,
				})
			})
			.collect::<Result<Vec<_>>>()?;
		Ok(QueryPacketCommitmentsResponse {
			commitments,
			pagination: None,
			height: Some(ibc_proto::ibc::core::client::v1::Height {
				revision_number: para_id.into(),
				revision_height: result.height,
			}),
		})
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
			.ok_or_else(|| runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		let para_id = api
			.para_id(&at)
			.map_err(|_| runtime_error_into_rpc_error("Error getting para id"))?;
		let result: ibc_primitives::QueryPacketAcknowledgementsResponse = api
			.packet_acknowledgements(
				&at,
				channel_id.as_bytes().to_vec(),
				port_id.as_bytes().to_vec(),
			)
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Failed to fetch acknowledgements"))?;
		let acknowledgements = result
			.acks
			.into_iter()
			.map(|packet_state| {
				let port_id = String::from_utf8(packet_state.port_id)
					.map_err(|_| runtime_error_into_rpc_error("Failed to decode port id"))?;
				let channel_id = String::from_utf8(packet_state.channel_id)
					.map_err(|_| runtime_error_into_rpc_error("Failed to decode port id"))?;
				Ok(PacketState {
					port_id,
					channel_id,
					sequence: packet_state.sequence,
					data: packet_state.data,
				})
			})
			.collect::<Result<Vec<_>>>()?;
		Ok(QueryPacketAcknowledgementsResponse {
			acknowledgements,
			pagination: None,
			height: Some(ibc_proto::ibc::core::client::v1::Height {
				revision_number: para_id.into(),
				revision_height: result.height,
			}),
		})
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
			.ok_or_else(|| runtime_error_into_rpc_error("Error retreiving block hash"))?;
		let at = BlockId::Hash(block_hash);

		api.unreceived_packets(
			&at,
			channel_id.as_bytes().to_vec(),
			port_id.as_bytes().to_vec(),
			seqs,
		)
		.ok()
		.flatten()
		.ok_or_else(|| runtime_error_into_rpc_error("Failed to unreceived packet sequences"))
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
			.ok_or_else(|| runtime_error_into_rpc_error("Error retreiving block hash"))?;
		let at = BlockId::Hash(block_hash);

		api.unreceived_acknowledgements(
			&at,
			channel_id.as_bytes().to_vec(),
			port_id.as_bytes().to_vec(),
			seqs,
		)
		.ok()
		.flatten()
		.ok_or_else(|| runtime_error_into_rpc_error("Failed to unreceived packet sequences"))
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
			.ok_or_else(|| runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		let para_id = api
			.para_id(&at)
			.map_err(|_| runtime_error_into_rpc_error("Error getting para id"))?;
		let result: ibc_primitives::QueryNextSequenceReceiveResponse = api
			.next_seq_recv(&at, channel_id.as_bytes().to_vec(), port_id.as_bytes().to_vec())
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Error fetching next sequence"))?;
		let inputs: Vec<(Vec<u8>, Vec<u8>)> = api
			.get_trie_inputs(&at)
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Error getting trie inputs"))?;
		Ok(QueryNextSequenceReceiveResponse {
			next_sequence_receive: result.sequence,
			proof: generate_raw_proof(inputs, vec![result.trie_key])?,
			proof_height: Some(ibc_proto::ibc::core::client::v1::Height {
				revision_number: para_id.into(),
				revision_height: result.height,
			}),
		})
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
			.ok_or_else(|| runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		let para_id = api
			.para_id(&at)
			.map_err(|_| runtime_error_into_rpc_error("Error getting para id"))?;
		let result: ibc_primitives::QueryPacketCommitmentResponse = api
			.packet_commitment(
				&at,
				channel_id.as_bytes().to_vec(),
				port_id.as_bytes().to_vec(),
				seq,
			)
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Error fetching next sequence"))?;
		let inputs: Vec<(Vec<u8>, Vec<u8>)> = api
			.get_trie_inputs(&at)
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Error getting trie inputs"))?;
		Ok(QueryPacketCommitmentResponse {
			commitment: result.commitment,
			proof: generate_raw_proof(inputs, vec![result.trie_key])?,
			proof_height: Some(ibc_proto::ibc::core::client::v1::Height {
				revision_number: para_id.into(),
				revision_height: result.height,
			}),
		})
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
			.ok_or_else(|| runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		let para_id = api
			.para_id(&at)
			.map_err(|_| runtime_error_into_rpc_error("Error getting para id"))?;
		let result: ibc_primitives::QueryPacketAcknowledgementResponse = api
			.packet_acknowledgement(
				&at,
				channel_id.as_bytes().to_vec(),
				port_id.as_bytes().to_vec(),
				seq,
			)
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Error fetching next sequence"))?;
		let inputs: Vec<(Vec<u8>, Vec<u8>)> = api
			.get_trie_inputs(&at)
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Error getting trie inputs"))?;
		Ok(QueryPacketAcknowledgementResponse {
			acknowledgement: result.ack,
			proof: generate_raw_proof(inputs, vec![result.trie_key])?,
			proof_height: Some(ibc_proto::ibc::core::client::v1::Height {
				revision_number: para_id.into(),
				revision_height: result.height,
			}),
		})
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
			.ok_or_else(|| runtime_error_into_rpc_error("Error retreiving block hash"))?;

		let at = BlockId::Hash(block_hash);
		let para_id = api
			.para_id(&at)
			.map_err(|_| runtime_error_into_rpc_error("Error getting para id"))?;
		let result: ibc_primitives::QueryPacketReceiptResponse = api
			.packet_receipt(&at, channel_id.as_bytes().to_vec(), port_id.as_bytes().to_vec(), seq)
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Error fetching next sequence"))?;
		let inputs: Vec<(Vec<u8>, Vec<u8>)> = api
			.get_trie_inputs(&at)
			.ok()
			.flatten()
			.ok_or_else(|| runtime_error_into_rpc_error("Error getting trie inputs"))?;
		Ok(QueryPacketReceiptResponse {
			received: result.receipt,
			proof: generate_raw_proof(inputs, vec![result.trie_key])?,
			proof_height: Some(ibc_proto::ibc::core::client::v1::Height {
				revision_number: para_id.into(),
				revision_height: result.height,
			}),
		})
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

	fn query_newly_created_clients(
		&self,
		block_hash: Block::Hash,
	) -> Result<Vec<IdentifiedClientState>> {
		let api = self.client.runtime_api();
		let at = BlockId::Hash(block_hash);
		let events = api
			.block_events(&at)
			.map_err(|_| runtime_error_into_rpc_error("[ibc_rpc]: failed to read block events"))?;

		let mut identified_clients = vec![];
		for e in events {
			match e {
				IbcEvent::CreateClient { client_id, .. } => {
					let result: ibc_primitives::QueryClientStateResponse = api
						.client_state(&at, client_id.clone())
						.ok()
						.flatten()
						.ok_or_else(|| runtime_error_into_rpc_error("client state to exist"))?;

					let client_state = AnyClientState::decode_vec(&result.client_state)
						.map_err(|_| runtime_error_into_rpc_error("client state to be valid"))?;
					let client_state = IdentifiedClientState {
						client_id: String::from_utf8(client_id).map_err(|_| {
							runtime_error_into_rpc_error("client id should be valid utf8")
						})?,
						client_state: Some(client_state.into()),
					};
					identified_clients.push(client_state)
				},
				_ => continue,
			}
		}

		Ok(identified_clients)
	}
}
