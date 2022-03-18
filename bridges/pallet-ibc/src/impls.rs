use super::*;
use codec::Encode;
use frame_support::traits::Currency;
use ibc::core::{
	ics02_client::{
		client_consensus::AnyConsensusState, client_state::AnyClientState, client_type::ClientType,
	},
	ics24_host::{
		identifier::*,
		path::{
			AcksPath, ChannelEndsPath, ClientConnectionsPath, ClientConsensusStatePath,
			ClientStatePath, ClientTypePath, CommitmentsPath, ConnectionsPath, ReceiptsPath,
			SeqAcksPath, SeqRecvsPath, SeqSendsPath,
		},
	},
};
use ibc::core::ics02_client::context::ClientReader;
use ibc_primitives::{
	ConnectionHandshakeProof, IdentifiedChannel, IdentifiedClientState, IdentifiedConnection,
	IdentifiedConsensusState, PacketState, QueryChannelResponse, QueryChannelsResponse,
	QueryClientStateResponse, QueryConnectionResponse, QueryConnectionsResponse,
	QueryConsensusStateResponse, QueryNextSequenceReceiveResponse,
	QueryPacketAcknowledgementResponse, QueryPacketAcknowledgementsResponse,
	QueryPacketCommitmentResponse, QueryPacketCommitmentsResponse, QueryPacketReceiptResponse,
};
use sp_runtime::traits::BlakeTwo256;
use sp_trie::{TrieDBMut, TrieMut};
use tendermint_proto::Protobuf;

impl<T: Config> Pallet<T> {
	pub(crate) fn build_ibc_state_trie<'a>(
		db: &'a mut sp_trie::MemoryDB<BlakeTwo256>,
		root: &'a mut sp_core::H256,
	) -> Result<TrieDBMut<'a, sp_trie::LayoutV0<BlakeTwo256>>, Error<T>> {
		let mut trie = <TrieDBMut<sp_trie::LayoutV0<BlakeTwo256>>>::new(db, root);
		let prefix = T::CONNECTION_PREFIX.to_vec();

		// Insert client and consensus states in trie
		for (client_id, client_state) in ClientStates::<T>::iter() {
			let consensus_states = ConsensusStates::<T>::get(&client_id);
			let client_connection = ConnectionClient::<T>::get(&client_id);
			let client_type = Clients::<T>::get(&client_id);

			let id = ClientId::from_str(
				&String::from_utf8(client_id).map_err(|_| Error::<T>::DecodingError)?,
			)
			.map_err(|_| Error::<T>::DecodingError)?;
			let client_state_path = format!("{}", ClientStatePath(id.clone()));
			let client_type_path = format!("{}", ClientTypePath(id.clone()));
			let mut client_type_key = prefix.clone();
			let mut client_state_key = prefix.clone();
			client_state_key.extend_from_slice(client_state_path.as_bytes());
			client_type_key.extend_from_slice(client_type_path.as_bytes());
			trie.insert(&client_state_key, &client_state)
				.map_err(|_| Error::<T>::TrieInsertError)?;
			trie.insert(&client_type_key, &client_type)
				.map_err(|_| Error::<T>::TrieInsertError)?;

			let client_connections_path = format!("{}", ClientConnectionsPath(id.clone()));
			let mut client_connections_path_key = prefix.clone();
			client_connections_path_key.extend_from_slice(client_connections_path.as_bytes());
			trie.insert(&client_connections_path_key, &client_connection)
				.map_err(|_| Error::<T>::TrieInsertError)?;
			for (height, consensus_state) in consensus_states {
				let height =
					ibc::Height::decode(&*height).map_err(|_| Error::<T>::DecodingError)?;
				let consensus_path = ClientConsensusStatePath {
					client_id: id.clone(),
					epoch: height.revision_number,
					height: height.revision_height,
				};
				let mut key = prefix.clone();
				let path = format!("{}", consensus_path);
				key.extend_from_slice(&path.as_bytes());
				trie.insert(&key, &consensus_state).map_err(|_| Error::<T>::TrieInsertError)?;
			}
		}

		// Insert connection ends in trie
		for (connection, connection_end) in Connections::<T>::iter() {
			let mut key = prefix.clone();
			let connection_id = ConnectionId::from_str(
				&String::from_utf8(connection).map_err(|_| Error::<T>::DecodingError)?,
			)
			.map_err(|_| Error::<T>::DecodingError)?;
			let path = format!("{}", ConnectionsPath(connection_id));
			key.extend_from_slice(path.as_bytes());
			trie.insert(&key, &connection_end).map_err(|_| Error::<T>::TrieInsertError)?;
		}

		// Insert channel ends and sequences in trie
		for (port, channel, channel_end) in Channels::<T>::iter() {
			let next_seq_send = NextSequenceSend::<T>::get(&port, &channel);
			let next_seq_recv = NextSequenceRecv::<T>::get(&port, &channel);
			let next_seq_ack = NextSequenceAck::<T>::get(&port, &channel);
			let mut channel_key = prefix.clone();
			let channel_id = channel_id_from_bytes::<T>(channel)?;
			let port_id = port_id_from_bytes::<T>(port)?;
			let channel_path = format!("{}", ChannelEndsPath(port_id.clone(), channel_id.clone()));
			let next_seq_send_path =
				format!("{}", SeqSendsPath(port_id.clone(), channel_id.clone()));
			let next_seq_recv_path =
				format!("{}", SeqRecvsPath(port_id.clone(), channel_id.clone()));
			let next_seq_ack_path = format!("{}", SeqAcksPath(port_id.clone(), channel_id.clone()));
			let mut next_seq_send_key = prefix.clone();
			let mut next_seq_recv_key = prefix.clone();
			let mut next_seq_ack_key = prefix.clone();
			next_seq_recv_key.extend_from_slice(next_seq_recv_path.as_bytes());
			next_seq_send_key.extend_from_slice(next_seq_send_path.as_bytes());
			next_seq_ack_key.extend_from_slice(next_seq_ack_path.as_bytes());

			channel_key.extend_from_slice(channel_path.as_bytes());
			trie.insert(&channel_key, &channel_end)
				.map_err(|_| Error::<T>::TrieInsertError)?;

			trie.insert(&next_seq_ack_key, &next_seq_ack)
				.map_err(|_| Error::<T>::TrieInsertError)?;
			trie.insert(&next_seq_send_key, &next_seq_send)
				.map_err(|_| Error::<T>::TrieInsertError)?;
			trie.insert(&next_seq_recv_key, &next_seq_recv)
				.map_err(|_| Error::<T>::TrieInsertError)?;
		}

		// Insert packet commitments in trie
		for ((port, channel, sequence), commitment) in PacketCommitment::<T>::iter() {
			let channel_id = channel_id_from_bytes::<T>(channel)?;
			let port_id = port_id_from_bytes::<T>(port)?;
			let sequence = ibc::core::ics04_channel::packet::Sequence::from(
				u64::decode(&mut &*sequence).map_err(|_| Error::<T>::DecodingError)?,
			);

			let commitment_path = CommitmentsPath { port_id, channel_id, sequence };

			let mut commitment_key = prefix.clone();
			let commitment_path_str = format!("{}", commitment_path);
			commitment_key.extend_from_slice(commitment_path_str.as_bytes());

			trie.insert(&commitment_key, &commitment)
				.map_err(|_| Error::<T>::TrieInsertError)?;
		}

		// Insert packet acknowledgements in trie
		for ((port, channel, sequence), ack) in Acknowledgements::<T>::iter() {
			let channel_id = channel_id_from_bytes::<T>(channel)?;
			let port_id = port_id_from_bytes::<T>(port)?;
			let sequence = ibc::core::ics04_channel::packet::Sequence::from(
				u64::decode(&mut &*sequence).map_err(|_| Error::<T>::DecodingError)?,
			);

			let ack_path = AcksPath { port_id, channel_id, sequence };

			let mut ack_key = prefix.clone();
			let ack_path_str = format!("{}", ack_path);
			ack_key.extend_from_slice(ack_path_str.as_bytes());

			trie.insert(&ack_key, &ack).map_err(|_| Error::<T>::TrieInsertError)?;
		}

		// Insert packet receipts in trie
		for ((port, channel, sequence), receipt) in PacketReceipt::<T>::iter() {
			let channel_id = channel_id_from_bytes::<T>(channel)?;
			let port_id = port_id_from_bytes::<T>(port)?;
			let sequence = ibc::core::ics04_channel::packet::Sequence::from(
				u64::decode(&mut &*sequence).map_err(|_| Error::<T>::DecodingError)?,
			);

			let receipt_path = ReceiptsPath { port_id, channel_id, sequence };

			let mut receipt_key = prefix.clone();
			let receipt_path_str = format!("{}", receipt_path);
			receipt_key.extend_from_slice(receipt_path_str.as_bytes());

			trie.insert(&receipt_key, &receipt).map_err(|_| Error::<T>::TrieInsertError)?;
		}

		Ok(trie)
	}

	pub(crate) fn extract_ibc_state_root() -> Result<Vec<u8>, Error<T>> {
		let mut db = sp_trie::MemoryDB::<BlakeTwo256>::default();
		let mut root = Default::default();
		let mut trie = Self::build_ibc_state_trie(&mut db, &mut root)?;
		Ok(trie.root().as_bytes().to_vec())
	}

	pub fn generate_proof(keys: Vec<Vec<u8>>) -> Result<Vec<Vec<u8>>, Error<T>> {
		let keys = keys.iter().collect::<Vec<_>>();
		let mut db = sp_trie::MemoryDB::<BlakeTwo256>::default();
		let root = {
			let mut root = Default::default();
			let mut trie = Self::build_ibc_state_trie(&mut db, &mut root)?;
			trie.root().clone()
		};
		sp_trie::generate_trie_proof::<sp_trie::LayoutV0<BlakeTwo256>, _, _, _>(&db, root, keys)
			.map_err(|_| Error::<T>::ProofGenerationError)
	}

	// IBC Runtime Api helper methods
	/// Get a channel state
	pub fn channel(channel_id: String, port_id: String) -> Result<QueryChannelResponse, Error<T>> {
		let channel_id_bytes = channel_id.as_bytes().to_vec();
		let port_id_bytes = port_id.as_bytes().to_vec();
		let channel = Channels::<T>::get(port_id_bytes.clone(), channel_id_bytes.clone());
		let port_id = port_id_from_bytes(port_id_bytes)?;
		let channel_id = channel_id_from_bytes(channel_id_bytes)?;

		let mut key = T::CONNECTION_PREFIX.to_vec();
		let channel_path = format!("{}", ChannelEndsPath(port_id.clone(), channel_id.clone()));
		key.extend_from_slice(channel_path.as_bytes());

		Ok(QueryChannelResponse {
			channel,
			proof: Self::generate_proof(vec![key])?,
			height: host_height::<T>()?.encode_vec().map_err(|_| Error::<T>::EncodingError)?,
		})
	}

	/// Get a connection state
	pub fn connection(connection_id: String) -> Result<QueryConnectionResponse, Error<T>> {
		let connection_id_bytes = connection_id.as_bytes().to_vec();
		let connection = Connections::<T>::get(connection_id_bytes.clone());
		let mut key = T::CONNECTION_PREFIX.to_vec();
		let connection_id = connection_id_from_bytes(connection_id_bytes)?;

		let connection_path = format!("{}", ConnectionsPath(connection_id));
		key.extend_from_slice(connection_path.as_bytes());

		Ok(QueryConnectionResponse {
			connection,
			proof: Self::generate_proof(vec![key])?,
			height: host_height::<T>()?.encode_vec().map_err(|_| Error::<T>::EncodingError)?,
		})
	}

	/// Get a client state
	pub fn client(client_id: String) -> Result<QueryClientStateResponse, Error<T>> {
		let client_id_bytes = client_id.as_bytes().to_vec();
		let client_state = ClientStates::<T>::get(client_id_bytes.clone());
		let client_state =
			AnyClientState::decode_vec(&client_state).map_err(|_| Error::<T>::DecodingError)?;
		let client_type = Clients::<T>::get(client_id_bytes.clone());
		let client_type = client_type_from_bytes(client_type)?;
		// TODO: Revisit when more client states are defined in ibc_rs
		let client_state = match client_state {
			AnyClientState::Tendermint(state) => IdentifiedClientState {
				client_id,
				client_type: client_type.to_string(),
				client_state: state.encode_vec().map_err(|_| Error::<T>::DecodingError)?,
			},
			_ => return Err(Error::<T>::DecodingError),
		};

		let mut key = T::CONNECTION_PREFIX.to_vec();
		let client_id = client_id_from_bytes(client_id_bytes)?;

		let client_state_path = format!("{}", ClientStatePath(client_id));

		key.extend_from_slice(client_state_path.as_bytes());

		Ok(QueryClientStateResponse {
			client_state,
			proof: Self::generate_proof(vec![key])?,
			height: host_height::<T>()?.encode_vec().map_err(|_| Error::<T>::EncodingError)?,
		})
	}

	/// Get all client states
	pub fn clients() -> Result<Vec<IdentifiedClientState>, Error<T>> {
		let client_states = ClientStates::<T>::iter()
			.map(|(id, state)| {
				let client_id = client_id_from_bytes(id.clone())?;
				let client_type = Clients::<T>::get(id);
				let client_type = client_type_from_bytes(client_type)?;
				// TODO: Revisit when more client states are defined in ibc_rs
				let client_state =
					AnyClientState::decode_vec(&state).map_err(|_| Error::<T>::DecodingError)?;
				match client_state {
					AnyClientState::Tendermint(state) => {
						if let Ok(state) = state.encode_vec().map_err(|_| Error::<T>::DecodingError)
						{
							Ok(IdentifiedClientState {
								client_id: client_id.to_string(),
								client_type: client_type.to_string(),
								client_state: state,
							})
						} else {
							Err(Error::<T>::DecodingError)
						}
					},
					_ => Err(Error::<T>::DecodingError),
				}
			})
			.collect::<Result<Vec<_>, Error<T>>>()?;

		Ok(client_states)
	}

	/// Get a consensus state for client
	pub fn consensus_state(
		height: Vec<u8>,
		client_id: String,
	) -> Result<QueryConsensusStateResponse, Error<T>> {
		let client_id_bytes = client_id.as_bytes().to_vec();
		let client_type = Clients::<T>::get(client_id_bytes.clone());
		let client_type = client_type_from_bytes(client_type)?;
		let consensus_states = ConsensusStates::<T>::get(client_id_bytes.clone());
		let mut key = T::CONNECTION_PREFIX.to_vec();
		let client_id = client_id_from_bytes(client_id_bytes)?;

		let (.., consensus_state) = consensus_states
			.into_iter()
			.find(|(maybe_height, ..)| maybe_height == &height)
			.ok_or(Error::<T>::ConsensusStateNotFound)?;
		let consensus_state = AnyConsensusState::decode_vec(&consensus_state)
			.map_err(|_| Error::<T>::DecodingError)?;
		// TODO: Revisit when more consensus states are defined in ibc_rs
		let consensus_state = match consensus_state {
			AnyConsensusState::Tendermint(state) => IdentifiedConsensusState {
				client_id: client_id.to_string(),
				client_type: client_type.to_string(),
				consensus_state: state.encode_vec().map_err(|_| Error::<T>::DecodingError)?,
			},
			_ => return Err(Error::<T>::DecodingError),
		};

		let height = ibc::Height::decode(&*height).map_err(|_| Error::<T>::DecodingError)?;
		let consensus_path = ClientConsensusStatePath {
			client_id,
			epoch: height.revision_number,
			height: height.revision_height,
		};

		let path = format!("{}", consensus_path);
		key.extend_from_slice(path.as_bytes());

		Ok(QueryConsensusStateResponse {
			consensus_state,
			proof: Self::generate_proof(vec![key])?,
			height: host_height::<T>()?.encode_vec().map_err(|_| Error::<T>::EncodingError)?,
		})
	}

	/// Get all connection states for a client
	pub fn connection_using_client(client_id: String) -> Result<IdentifiedConnection, Error<T>> {
		let connection_id = ConnectionClient::<T>::get(client_id.as_bytes().to_vec());
		let connection = Connections::<T>::get(connection_id.clone());

		let connection_id = connection_id_from_bytes(connection_id)?;

		Ok(IdentifiedConnection {
			connection_id: connection_id.to_string(),
			connection_end: connection,
		})
	}

	/// Get client state for client which this channel is bound to
	pub fn channel_client(
		channel_id: String,
		port_id: String,
	) -> Result<IdentifiedClientState, Error<T>> {
		for (connection_id, channels) in ChannelsConnection::<T>::iter() {
			if channels.contains(&(port_id.as_bytes().to_vec(), channel_id.as_bytes().to_vec())) {
				if let Some((client_id, ..)) = ConnectionClient::<T>::iter()
					.find(|(.., connection)| &connection_id == connection)
				{
					let client_type = Clients::<T>::get(client_id.clone());
					let client_type = client_type_from_bytes(client_type)?;
					let client_state = ClientStates::<T>::get(client_id.clone());
					let client_state = AnyClientState::decode_vec(&client_state)
						.map_err(|_| Error::<T>::DecodingError)?;
					let client_id = client_id_from_bytes(client_id)?;
					// TODO: Revisit when more client states are defined in ibc_rs
					let client_state = match client_state {
						AnyClientState::Tendermint(state) =>
							state.encode_vec().map_err(|_| Error::<T>::DecodingError)?,
						_ => return Err(Error::<T>::DecodingError),
					};

					return Ok(IdentifiedClientState {
						client_id: client_id.to_string(),
						client_type: client_type.to_string(),
						client_state,
					})
				}
			}
		}
		Err(Error::<T>::ClientStateNotFound)
	}

	/// Get all channel states
	pub fn channels() -> Result<QueryChannelsResponse, Error<T>> {
		let channels = Channels::<T>::iter()
			.map(|(port_id, channel_id, channel_end)| {
				Ok(IdentifiedChannel {
					channel_id: channel_id_from_bytes(channel_id)?.to_string(),
					port_id: port_id_from_bytes(port_id)?.to_string(),
					channel_end,
				})
			})
			.collect::<Result<Vec<_>, Error<T>>>()?;

		Ok(QueryChannelsResponse {
			channels,
			height: host_height::<T>()?.encode_vec().map_err(|_| Error::<T>::EncodingError)?,
		})
	}

	/// Get all connection states
	pub fn connections() -> Result<QueryConnectionsResponse, Error<T>> {
		let connections = Connections::<T>::iter()
			.map(|(connection_id, connection_end)| {
				Ok(IdentifiedConnection {
					connection_id: connection_id_from_bytes(connection_id)?.to_string(),
					connection_end,
				})
			})
			.collect::<Result<Vec<_>, Error<T>>>()?;

		Ok(QueryConnectionsResponse {
			connections,
			height: host_height::<T>()?.encode_vec().map_err(|_| Error::<T>::EncodingError)?,
		})
	}

	/// Get all channels bound to this connection
	pub fn connection_channels(connection_id: String) -> Result<QueryChannelsResponse, Error<T>> {
		let identifiers = ChannelsConnection::<T>::get(connection_id.as_bytes().to_vec());

		let channels = identifiers
			.into_iter()
			.map(|(port_id, channel_id)| {
				let channel_end = Channels::<T>::get(port_id.clone(), channel_id.clone());
				Ok(IdentifiedChannel {
					channel_id: channel_id_from_bytes(channel_id)?.to_string(),
					port_id: port_id_from_bytes(port_id)?.to_string(),
					channel_end,
				})
			})
			.collect::<Result<Vec<_>, Error<T>>>()?;
		Ok(QueryChannelsResponse {
			channels,
			height: host_height::<T>()?.encode_vec().map_err(|_| Error::<T>::EncodingError)?,
		})
	}

	pub fn packet_commitments(
		channel_id: String,
		port_id: String,
	) -> Result<QueryPacketCommitmentsResponse, Error<T>> {
		let channel_id_bytes = channel_id.as_bytes().to_vec();
		let port_id_bytes = port_id.as_bytes().to_vec();
		let commitments = PacketCommitment::<T>::iter()
			.filter_map(|((p, c, s), commitment)| {
				if p == port_id_bytes && c == channel_id_bytes {
					let packet_state = PacketState {
						port_id: port_id.clone(),
						channel_id: channel_id.clone(),
						sequence: u64::decode(&mut &*s).unwrap_or_default(),
						data: commitment,
					};
					Some(packet_state)
				} else {
					None
				}
			})
			.collect::<Vec<_>>();

		Ok(QueryPacketCommitmentsResponse {
			commitments,
			height: host_height::<T>()?.encode_vec().map_err(|_| Error::<T>::EncodingError)?,
		})
	}

	pub fn packet_acknowledgements(
		channel_id: String,
		port_id: String,
	) -> Result<QueryPacketAcknowledgementsResponse, Error<T>> {
		let channel_id_bytes = channel_id.as_bytes().to_vec();
		let port_id_bytes = port_id.as_bytes().to_vec();
		let acks = Acknowledgements::<T>::iter()
			.filter_map(|((p, c, s), ack)| {
				if p == port_id_bytes && c == channel_id_bytes {
					let packet_state = PacketState {
						port_id: port_id.clone(),
						channel_id: channel_id.clone(),
						sequence: u64::decode(&mut &*s).unwrap_or_default(),
						data: ack,
					};
					Some(packet_state)
				} else {
					None
				}
			})
			.collect::<Vec<_>>();
		Ok(QueryPacketAcknowledgementsResponse {
			acks,
			height: host_height::<T>()?.encode_vec().map_err(|_| Error::<T>::EncodingError)?,
		})
	}

	pub fn unreceived_packets(
		channel_id: String,
		port_id: String,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Error<T>> {
		let channel_id = channel_id.as_bytes().to_vec();
		let port_id = port_id.as_bytes().to_vec();

		Ok(seqs
			.into_iter()
			.filter(|s| {
				let sequence = s.encode();
				!PacketReceipt::<T>::contains_key((port_id.clone(), channel_id.clone(), sequence))
			})
			.collect())
	}

	pub fn unreceived_acknowledgements(
		channel_id: String,
		port_id: String,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Error<T>> {
		let channel_id = channel_id.as_bytes().to_vec();
		let port_id = port_id.as_bytes().to_vec();

		Ok(seqs
			.into_iter()
			.filter(|s| {
				let sequence = s.encode();
				PacketCommitment::<T>::contains_key((port_id.clone(), channel_id.clone(), sequence))
			})
			.collect())
	}

	pub fn next_seq_recv(
		channel_id: String,
		port_id: String,
	) -> Result<QueryNextSequenceReceiveResponse, Error<T>> {
		let channel_id_bytes = channel_id.as_bytes().to_vec();
		let port_id_bytes = port_id.as_bytes().to_vec();

		let sequence = u64::decode(
			&mut NextSequenceRecv::<T>::get(port_id_bytes.clone(), channel_id_bytes.clone())
				.as_slice(),
		)
		.map_err(|_| Error::<T>::DecodingError)?;
		let port_id = port_id_from_bytes(port_id_bytes)?;
		let channel_id = channel_id_from_bytes(channel_id_bytes)?;
		let next_seq_recv_path = format!("{}", SeqRecvsPath(port_id, channel_id));
		let mut key = T::CONNECTION_PREFIX.to_vec();
		key.extend_from_slice(next_seq_recv_path.as_bytes());

		Ok(QueryNextSequenceReceiveResponse {
			sequence,
			proof: Self::generate_proof(vec![key])?,
			height: host_height::<T>()?.encode_vec().map_err(|_| Error::<T>::EncodingError)?,
		})
	}

	pub fn packet_commitment(
		channel_id: String,
		port_id: String,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse, Error<T>> {
		let channel_id_bytes = channel_id.as_bytes().to_vec();
		let port_id_bytes = port_id.as_bytes().to_vec();
		let seq_bytes = seq.encode();
		let commitment = PacketCommitment::<T>::get((
			port_id_bytes.clone(),
			channel_id_bytes.clone(),
			seq_bytes,
		));
		let port_id = port_id_from_bytes(port_id_bytes)?;
		let channel_id = channel_id_from_bytes(channel_id_bytes)?;
		let sequence = ibc::core::ics04_channel::packet::Sequence::from(seq);
		let commitment_path = format!("{}", CommitmentsPath { port_id, channel_id, sequence });
		let mut key = T::CONNECTION_PREFIX.to_vec();
		key.extend_from_slice(commitment_path.as_bytes());

		Ok(QueryPacketCommitmentResponse {
			commitment,
			proof: Self::generate_proof(vec![key])?,
			height: host_height::<T>()?.encode_vec().map_err(|_| Error::<T>::EncodingError)?,
		})
	}

	pub fn packet_acknowledgement(
		channel_id: String,
		port_id: String,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse, Error<T>> {
		let channel_id_bytes = channel_id.as_bytes().to_vec();
		let port_id_bytes = port_id.as_bytes().to_vec();
		let seq_bytes = seq.encode();
		let ack = Acknowledgements::<T>::get((
			port_id_bytes.clone(),
			channel_id_bytes.clone(),
			seq_bytes,
		));
		let port_id = port_id_from_bytes(port_id_bytes)?;
		let channel_id = channel_id_from_bytes(channel_id_bytes)?;
		let sequence = ibc::core::ics04_channel::packet::Sequence::from(seq);
		let acks_path = format!("{}", AcksPath { port_id, channel_id, sequence });
		let mut key = T::CONNECTION_PREFIX.to_vec();
		key.extend_from_slice(acks_path.as_bytes());

		Ok(QueryPacketAcknowledgementResponse {
			ack,
			proof: Self::generate_proof(vec![key])?,
			height: host_height::<T>()?.encode_vec().map_err(|_| Error::<T>::EncodingError)?,
		})
	}

	pub fn packet_receipt(
		channel_id: String,
		port_id: String,
		seq: u64,
	) -> Result<QueryPacketReceiptResponse, Error<T>> {
		let channel_id_bytes = channel_id.as_bytes().to_vec();
		let port_id_bytes = port_id.as_bytes().to_vec();
		let seq_bytes = seq.encode();
		let receipt =
			PacketReceipt::<T>::get((port_id_bytes.clone(), channel_id_bytes.clone(), seq_bytes));
		let port_id = port_id_from_bytes(port_id_bytes)?;
		let channel_id = channel_id_from_bytes(channel_id_bytes)?;
		let sequence = ibc::core::ics04_channel::packet::Sequence::from(seq);
		let receipt_path = format!("{}", ReceiptsPath { port_id, channel_id, sequence });
		let mut key = T::CONNECTION_PREFIX.to_vec();
		key.extend_from_slice(receipt_path.as_bytes());

		Ok(QueryPacketReceiptResponse {
			receipt,
			proof: Self::generate_proof(vec![key])?,
			height: host_height::<T>()?.encode_vec().map_err(|_| Error::<T>::EncodingError)?,
		})
	}

	pub fn generate_connection_handshake_proof(
		client_id: String,
		connection_id: String,
	) -> Result<ConnectionHandshakeProof, Error<T>> {
		let client_state = ClientStates::<T>::get(client_id.as_bytes().to_vec());
		let client_state_decoded =
			AnyClientState::decode_vec(&client_state).map_err(|_| Error::<T>::DecodingError)?;
		let client_type = Clients::<T>::get(client_id.as_bytes().to_vec());
		let client_type = client_type_from_bytes(client_type)?;
		let height = client_state_decoded.latest_height();
		let client_id = client_id_from_bytes(client_id.as_bytes().to_vec())?;
		let connection_id = connection_id_from_bytes(connection_id.as_bytes().to_vec())?;
		let prefix = T::CONNECTION_PREFIX.to_vec();
		let connection_path = format!("{}", ConnectionsPath(connection_id));
		let consensus_path = ClientConsensusStatePath {
			client_id: client_id.clone(),
			epoch: height.revision_number,
			height: height.revision_height,
		};
		let client_state_path = format!("{}", ClientStatePath(client_id.clone()));
		let consensus_path = format!("{}", consensus_path);
		let mut client_state_key = prefix.clone();
		client_state_key.extend_from_slice(client_state_path.as_bytes());
		let mut connection_key = prefix.clone();
		connection_key.extend_from_slice(connection_path.as_bytes());
		let mut consensus_key = prefix.clone();
		consensus_key.extend_from_slice(consensus_path.as_bytes());

		Ok(ConnectionHandshakeProof {
			client_state: IdentifiedClientState {
				client_id: client_id.to_string(),
				client_type: client_type.to_string(),
				client_state,
			},
			proof: Self::generate_proof(vec![client_state_key, connection_key, consensus_key])?,
			height: host_height::<T>()?.encode_vec().map_err(|_| Error::<T>::EncodingError)?,
		})
	}

	pub fn query_balance_with_address(addr: Vec<u8>) -> Result<u128, Error<T>> {
		let account_id =
			T::AccountId::decode(&mut &*addr).map_err(|_| Error::<T>::DecodingError)?;
		let balance = format!("{:?}", T::Currency::free_balance(&account_id));
		Ok(balance.parse().unwrap_or_default())
	}

	// WIP
	pub fn check_and_process_timeouts() -> Result<(), Error<T>>{
		let mut ctx = routing::Context::<T>::new();
		for client_id in ClientStates::<T>::iter_keys() {
			let client_id = client_id_from_bytes(client_id)?;
			let client_state = ctx.client_state(&client_id).map_err(|_| Error::<T>::ClientStateNotFound)?;
			let latest_height = client_state.latest_height();
			let consensus_state = ctx.consensus_state(&client_id, latest_height).map_err(|_| Error::<T>::ConsensusStateNotFound)?;
			let latest_timestamp = consensus_state.timestamp();


		}

		Ok(())
	}
}

fn port_id_from_bytes<T: Config>(port: Vec<u8>) -> Result<PortId, Error<T>> {
	PortId::from_str(&String::from_utf8(port).map_err(|_| Error::<T>::DecodingError)?)
		.map_err(|_| Error::<T>::DecodingError)
}

fn channel_id_from_bytes<T: Config>(channel: Vec<u8>) -> Result<ChannelId, Error<T>> {
	ChannelId::from_str(&String::from_utf8(channel).map_err(|_| Error::<T>::DecodingError)?)
		.map_err(|_| Error::<T>::DecodingError)
}

fn connection_id_from_bytes<T: Config>(connection: Vec<u8>) -> Result<ConnectionId, Error<T>> {
	ConnectionId::from_str(&String::from_utf8(connection).map_err(|_| Error::<T>::DecodingError)?)
		.map_err(|_| Error::<T>::DecodingError)
}

fn client_id_from_bytes<T: Config>(client_id: Vec<u8>) -> Result<ClientId, Error<T>> {
	ClientId::from_str(&String::from_utf8(client_id).map_err(|_| Error::<T>::DecodingError)?)
		.map_err(|_| Error::<T>::DecodingError)
}

fn client_type_from_bytes<T: Config>(client_type: Vec<u8>) -> Result<ClientType, Error<T>> {
	ClientType::from_str(&String::from_utf8(client_type).map_err(|_| Error::<T>::DecodingError)?)
		.map_err(|_| Error::<T>::DecodingError)
}

fn host_height<T: Config>() -> Result<ibc::Height, Error<T>> {
	let block_number = format!("{:?}", <frame_system::Pallet<T>>::block_number());
	let current_height = block_number.parse().map_err(|_| Error::<T>::DecodingError)?;
	Ok(ibc::Height::new(0, current_height))
}
