use super::*;
use codec::{Decode, Encode};
use frame_support::traits::Currency;
use ibc::{
	core::{
		ics02_client::{client_consensus::AnyConsensusState, client_state::AnyClientState},
		ics04_channel::{
			channel::ChannelEnd,
			context::{ChannelKeeper, ChannelReader},
			msgs::chan_open_init::{MsgChannelOpenInit, TYPE_URL as CHANNEL_OPEN_INIT_TYPE_URL},
			packet::{Packet, Sequence},
		},
		ics05_port::{
			capabilities::{PortCapability, PortCapabilityType, TypedCapability},
			context::{PortKeeper, PortReader},
		},
		ics24_host::{
			identifier::*,
			path::{
				AcksPath, ChannelEndsPath, ClientConsensusStatePath, ClientStatePath,
				ClientTypePath, CommitmentsPath, ConnectionsPath, ReceiptsPath, SeqAcksPath,
				SeqRecvsPath, SeqSendsPath,
			},
		},
	},
	signer::Signer,
};
use ibc_primitives::{
	ConnectionHandshake, IdentifiedChannel, IdentifiedClientState, IdentifiedConnection,
	OffchainPacketType, PacketState, QueryChannelResponse, QueryChannelsResponse,
	QueryClientStateResponse, QueryConnectionResponse, QueryConnectionsResponse,
	QueryConsensusStateResponse, QueryNextSequenceReceiveResponse,
	QueryPacketAcknowledgementResponse, QueryPacketAcknowledgementsResponse,
	QueryPacketCommitmentResponse, QueryPacketCommitmentsResponse, QueryPacketReceiptResponse,
	SendPacketData,
};
use ibc_trait::{
	apply_prefix_and_encode, channel_id_from_bytes, client_id_from_bytes, connection_id_from_bytes,
	port_id_from_bytes, Error as IbcHandlerError, IbcTrait,
};
use scale_info::prelude::{collections::BTreeMap, string::ToString};
use sp_std::time::Duration;
use tendermint_proto::Protobuf;

impl<T: Config> Pallet<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	pub fn build_trie_inputs() -> Result<Vec<(Vec<u8>, Vec<u8>)>, Error<T>> {
		let mut inputs = Vec::new();

		// Insert client state in trie
		for (client_id, client_state) in ClientStates::<T>::iter() {
			let client_type = Clients::<T>::get(&client_id);
			let id = ClientId::from_str(
				&String::from_utf8(client_id).map_err(|_| Error::<T>::DecodingError)?,
			)
			.map_err(|_| Error::<T>::DecodingError)?;
			let client_state_path = format!("{}", ClientStatePath(id.clone()));
			let client_type_path = format!("{}", ClientTypePath(id.clone()));
			let client_type_key =
				apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![client_type_path])
					.map_err(|_| Error::<T>::DecodingError)?;
			let client_state_key =
				apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![client_state_path])
					.map_err(|_| Error::<T>::DecodingError)?;
			inputs.push((client_state_key, client_state));
			inputs.push((client_type_key, client_type))
		}

		// Insert consensus states in trie
		let consensus_states = ConsensusStates::<T>::iter();
		for (client_id, height, consensus_state) in consensus_states {
			let client_id = ClientId::from_str(
				&String::from_utf8(client_id).map_err(|_| Error::<T>::DecodingError)?,
			)
			.map_err(|_| Error::<T>::DecodingError)?;
			let height = ibc::Height::decode(&*height).map_err(|_| Error::<T>::DecodingError)?;
			let consensus_path = ClientConsensusStatePath {
				client_id,
				epoch: height.revision_number,
				height: height.revision_height,
			};
			let path = format!("{}", consensus_path);
			let key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![path])
				.map_err(|_| Error::<T>::DecodingError)?;
			inputs.push((key, consensus_state));
		}

		// Insert connection ends in trie
		for (connection, connection_end) in Connections::<T>::iter() {
			let connection_id = ConnectionId::from_str(
				&String::from_utf8(connection).map_err(|_| Error::<T>::DecodingError)?,
			)
			.map_err(|_| Error::<T>::DecodingError)?;
			let path = format!("{}", ConnectionsPath(connection_id));
			let key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![path])
				.map_err(|_| Error::<T>::DecodingError)?;
			inputs.push((key, connection_end))
		}

		// Insert channel ends and sequences in trie
		for (port, channel, channel_end) in Channels::<T>::iter() {
			let next_seq_send = NextSequenceSend::<T>::get(&port, &channel);
			let next_seq_recv = NextSequenceRecv::<T>::get(&port, &channel);
			let next_seq_ack = NextSequenceAck::<T>::get(&port, &channel);
			let channel_id =
				channel_id_from_bytes(channel).map_err(|_| Error::<T>::DecodingError)?;
			let port_id = port_id_from_bytes(port).map_err(|_| Error::<T>::DecodingError)?;
			let channel_path = format!("{}", ChannelEndsPath(port_id.clone(), channel_id.clone()));
			let next_seq_send_path =
				format!("{}", SeqSendsPath(port_id.clone(), channel_id.clone()));
			let next_seq_recv_path =
				format!("{}", SeqRecvsPath(port_id.clone(), channel_id.clone()));
			let next_seq_ack_path = format!("{}", SeqAcksPath(port_id.clone(), channel_id.clone()));
			let next_seq_send_key =
				apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![next_seq_send_path])
					.map_err(|_| Error::<T>::DecodingError)?;
			let next_seq_recv_key =
				apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![next_seq_recv_path])
					.map_err(|_| Error::<T>::DecodingError)?;
			let next_seq_ack_key =
				apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![next_seq_ack_path])
					.map_err(|_| Error::<T>::DecodingError)?;
			let channel_key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![channel_path])
				.map_err(|_| Error::<T>::DecodingError)?;
			inputs.push((channel_key, channel_end));
			inputs.push((next_seq_ack_key, next_seq_ack.encode()));
			inputs.push((next_seq_send_key, next_seq_send.encode()));
			inputs.push((next_seq_recv_key, next_seq_recv.encode()));
		}

		// Insert packet commitments in trie
		for ((port, channel, sequence), commitment) in PacketCommitment::<T>::iter() {
			let channel_id =
				channel_id_from_bytes(channel).map_err(|_| Error::<T>::DecodingError)?;
			let port_id = port_id_from_bytes(port).map_err(|_| Error::<T>::DecodingError)?;
			let sequence = ibc::core::ics04_channel::packet::Sequence::from(sequence);

			let commitment_path = CommitmentsPath { port_id, channel_id, sequence };

			let commitment_path = format!("{}", commitment_path);
			let commitment_key =
				apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![commitment_path])
					.map_err(|_| Error::<T>::DecodingError)?;
			inputs.push((commitment_key, commitment))
		}

		// Insert packet acknowledgements in trie
		for ((port, channel, sequence), ack) in Acknowledgements::<T>::iter() {
			let channel_id =
				channel_id_from_bytes(channel).map_err(|_| Error::<T>::DecodingError)?;
			let port_id = port_id_from_bytes(port).map_err(|_| Error::<T>::DecodingError)?;
			let sequence = ibc::core::ics04_channel::packet::Sequence::from(sequence);

			let ack_path = AcksPath { port_id, channel_id, sequence };

			let ack_path = format!("{}", ack_path);
			let ack_key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![ack_path])
				.map_err(|_| Error::<T>::DecodingError)?;
			inputs.push((ack_key, ack));
		}

		// Insert packet receipts in trie
		for ((port, channel, sequence), receipt) in PacketReceipt::<T>::iter() {
			let channel_id =
				channel_id_from_bytes(channel).map_err(|_| Error::<T>::DecodingError)?;
			let port_id = port_id_from_bytes(port).map_err(|_| Error::<T>::DecodingError)?;
			let sequence = ibc::core::ics04_channel::packet::Sequence::from(sequence);

			let receipt_path = ReceiptsPath { port_id, channel_id, sequence };

			let receipt_path = format!("{}", receipt_path);
			let receipt_key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![receipt_path])
				.map_err(|_| Error::<T>::DecodingError)?;
			inputs.push((receipt_key, receipt))
		}

		Ok(inputs)
	}

	pub(crate) fn build_ibc_state_root() -> Result<sp_core::H256, Error<T>> {
		let inputs = Self::build_trie_inputs()?;
		Ok(sp_io::trie::blake2_256_root(inputs, sp_core::storage::StateVersion::V0))
	}

	pub(crate) fn extract_ibc_state_root() -> Result<Vec<u8>, Error<T>> {
		let root = Self::build_ibc_state_root()?;
		Ok(root.as_bytes().to_vec())
	}

	// IBC Runtime Api helper methods
	/// Get a channel state
	pub fn channel(
		channel_id: Vec<u8>,
		port_id: Vec<u8>,
	) -> Result<QueryChannelResponse, Error<T>> {
		let channel = Channels::<T>::get(port_id.clone(), channel_id.clone());
		let port_id = port_id_from_bytes(port_id.clone()).map_err(|_| Error::<T>::DecodingError)?;
		let channel_id =
			channel_id_from_bytes(channel_id.clone()).map_err(|_| Error::<T>::DecodingError)?;

		let channel_path = format!("{}", ChannelEndsPath(port_id.clone(), channel_id.clone()));
		let key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![channel_path])
			.map_err(|_| Error::<T>::DecodingError)?;

		Ok(QueryChannelResponse { channel, trie_key: key, height: host_height::<T>() })
	}

	/// Get a connection state
	pub fn connection(connection_id: Vec<u8>) -> Result<QueryConnectionResponse, Error<T>> {
		let connection = Connections::<T>::get(connection_id.clone());
		let connection_id =
			connection_id_from_bytes(connection_id).map_err(|_| Error::<T>::DecodingError)?;

		let connection_path = format!("{}", ConnectionsPath(connection_id));
		let key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![connection_path])
			.map_err(|_| Error::<T>::DecodingError)?;

		Ok(QueryConnectionResponse { connection, trie_key: key, height: host_height::<T>() })
	}

	/// Get a client state
	pub fn client(client_id: Vec<u8>) -> Result<QueryClientStateResponse, Error<T>> {
		let client_state = ClientStates::<T>::get(client_id.clone());
		let client_id = client_id_from_bytes(client_id).map_err(|_| Error::<T>::DecodingError)?;

		let client_state_path = format!("{}", ClientStatePath(client_id));

		let key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![client_state_path])
			.map_err(|_| Error::<T>::DecodingError)?;

		Ok(QueryClientStateResponse { client_state, trie_key: key, height: host_height::<T>() })
	}

	/// Get all client states
	/// Returns a Vec of (client_id, client_state)
	pub fn clients() -> Result<Vec<(Vec<u8>, Vec<u8>)>, Error<T>> {
		let client_states = ClientStates::<T>::iter().collect::<Vec<_>>();

		Ok(client_states)
	}

	/// Get a consensus state for client
	pub fn consensus_state(
		height: Vec<u8>,
		client_id: Vec<u8>,
		latest_cs: bool,
	) -> Result<QueryConsensusStateResponse, Error<T>> {
		let height = if latest_cs {
			let client_state = ClientStates::<T>::get(client_id.clone());
			let client_state =
				AnyClientState::decode_vec(&client_state).map_err(|_| Error::<T>::DecodingError)?;
			client_state
				.latest_height()
				.encode_vec()
				.map_err(|_| Error::<T>::DecodingError)?
		} else {
			height
		};
		let consensus_state = ConsensusStates::<T>::get(client_id.clone(), height.clone());
		let client_id =
			client_id_from_bytes(client_id.clone()).map_err(|_| Error::<T>::DecodingError)?;

		let height = ibc::Height::decode(&*height).map_err(|_| Error::<T>::DecodingError)?;
		let consensus_path = ClientConsensusStatePath {
			client_id,
			epoch: height.revision_number,
			height: height.revision_height,
		};

		let path = format!("{}", consensus_path);
		let key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![path])
			.map_err(|_| Error::<T>::DecodingError)?;

		Ok(QueryConsensusStateResponse {
			consensus_state,
			trie_key: key,
			height: host_height::<T>(),
		})
	}

	/// Get all connection states for a client
	pub fn connection_using_client(
		client_id: Vec<u8>,
	) -> Result<Vec<IdentifiedConnection>, Error<T>> {
		let connection_ids = ConnectionClient::<T>::get(client_id);
		let connections = connection_ids
			.into_iter()
			.map(|connection_id| IdentifiedConnection {
				connection_end: Connections::<T>::get(connection_id.clone()),
				connection_id,
			})
			.collect::<Vec<_>>();

		Ok(connections)
	}

	/// Get client state for client which this channel is bound to
	pub fn channel_client(
		channel_id: Vec<u8>,
		port_id: Vec<u8>,
	) -> Result<IdentifiedClientState, Error<T>> {
		for (connection_id, channels) in ChannelsConnection::<T>::iter() {
			if channels.contains(&(port_id.clone(), channel_id.clone())) {
				if let Some((client_id, ..)) = ConnectionClient::<T>::iter()
					.find(|(.., connection_ids)| connection_ids.contains(&connection_id))
				{
					let client_state = ClientStates::<T>::get(client_id.clone());
					return Ok(IdentifiedClientState { client_id, client_state })
				}
			}
		}
		Err(Error::<T>::ClientStateNotFound)
	}

	/// Get all channel states
	pub fn channels() -> Result<QueryChannelsResponse, Error<T>> {
		let channels = Channels::<T>::iter()
			.map(|(port_id, channel_id, channel_end)| {
				Ok(IdentifiedChannel { channel_id, port_id, channel_end })
			})
			.collect::<Result<Vec<_>, Error<T>>>()?;

		Ok(QueryChannelsResponse { channels, height: host_height::<T>() })
	}

	/// Get all connection states
	pub fn connections() -> Result<QueryConnectionsResponse, Error<T>> {
		let connections = Connections::<T>::iter()
			.map(|(connection_id, connection_end)| {
				Ok(IdentifiedConnection { connection_id, connection_end })
			})
			.collect::<Result<Vec<_>, Error<T>>>()?;

		Ok(QueryConnectionsResponse { connections, height: host_height::<T>() })
	}

	/// Get all channels bound to this connection
	pub fn connection_channels(connection_id: Vec<u8>) -> Result<QueryChannelsResponse, Error<T>> {
		let identifiers = ChannelsConnection::<T>::get(connection_id.clone());

		let channels = identifiers
			.into_iter()
			.map(|(port_id, channel_id)| {
				let channel_end = Channels::<T>::get(port_id.clone(), channel_id.clone());
				Ok(IdentifiedChannel { channel_id, port_id, channel_end })
			})
			.collect::<Result<Vec<_>, Error<T>>>()?;
		Ok(QueryChannelsResponse { channels, height: host_height::<T>() })
	}

	pub fn packet_commitments(
		channel_id: Vec<u8>,
		port_id: Vec<u8>,
	) -> Result<QueryPacketCommitmentsResponse, Error<T>> {
		let commitments = PacketCommitment::<T>::iter()
			.filter_map(|((p, c, s), commitment)| {
				if p == port_id && c == channel_id {
					let packet_state = PacketState {
						port_id: port_id.clone(),
						channel_id: channel_id.clone(),
						sequence: s,
						data: commitment,
					};
					Some(packet_state)
				} else {
					None
				}
			})
			.collect::<Vec<_>>();

		Ok(QueryPacketCommitmentsResponse { commitments, height: host_height::<T>() })
	}

	pub fn packet_acknowledgements(
		channel_id: Vec<u8>,
		port_id: Vec<u8>,
	) -> Result<QueryPacketAcknowledgementsResponse, Error<T>> {
		let acks = Acknowledgements::<T>::iter()
			.filter_map(|((p, c, s), ack)| {
				if p == port_id && c == channel_id {
					let packet_state = PacketState {
						port_id: port_id.clone(),
						channel_id: channel_id.clone(),
						sequence: s,
						data: ack,
					};
					Some(packet_state)
				} else {
					None
				}
			})
			.collect::<Vec<_>>();
		Ok(QueryPacketAcknowledgementsResponse { acks, height: host_height::<T>() })
	}

	pub fn unreceived_packets(
		channel_id: Vec<u8>,
		port_id: Vec<u8>,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Error<T>> {
		Ok(seqs
			.into_iter()
			.filter(|s| {
				!PacketReceipt::<T>::contains_key((port_id.clone(), channel_id.clone(), *s))
			})
			.collect())
	}

	pub fn unreceived_acknowledgements(
		channel_id: Vec<u8>,
		port_id: Vec<u8>,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Error<T>> {
		Ok(seqs
			.into_iter()
			.filter(|s| {
				PacketCommitment::<T>::contains_key((port_id.clone(), channel_id.clone(), *s))
			})
			.collect())
	}

	pub fn next_seq_recv(
		channel_id: Vec<u8>,
		port_id: Vec<u8>,
	) -> Result<QueryNextSequenceReceiveResponse, Error<T>> {
		let sequence = NextSequenceRecv::<T>::get(port_id.clone(), channel_id.clone());
		let port_id = port_id_from_bytes(port_id).map_err(|_| Error::<T>::DecodingError)?;
		let channel_id =
			channel_id_from_bytes(channel_id).map_err(|_| Error::<T>::DecodingError)?;
		let next_seq_recv_path = format!("{}", SeqRecvsPath(port_id, channel_id));
		let key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![next_seq_recv_path])
			.map_err(|_| Error::<T>::DecodingError)?;

		Ok(QueryNextSequenceReceiveResponse { sequence, trie_key: key, height: host_height::<T>() })
	}

	pub fn packet_commitment(
		channel_id: Vec<u8>,
		port_id: Vec<u8>,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse, Error<T>> {
		let commitment = PacketCommitment::<T>::get((port_id.clone(), channel_id.clone(), seq));
		let port_id = port_id_from_bytes(port_id).map_err(|_| Error::<T>::DecodingError)?;
		let channel_id =
			channel_id_from_bytes(channel_id).map_err(|_| Error::<T>::DecodingError)?;
		let sequence = ibc::core::ics04_channel::packet::Sequence::from(seq);
		let commitment_path = format!("{}", CommitmentsPath { port_id, channel_id, sequence });
		let key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![commitment_path])
			.map_err(|_| Error::<T>::DecodingError)?;

		Ok(QueryPacketCommitmentResponse { commitment, trie_key: key, height: host_height::<T>() })
	}

	pub fn packet_acknowledgement(
		channel_id: Vec<u8>,
		port_id: Vec<u8>,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse, Error<T>> {
		let ack = Acknowledgements::<T>::get((port_id.clone(), channel_id.clone(), seq));
		let port_id = port_id_from_bytes(port_id).map_err(|_| Error::<T>::DecodingError)?;
		let channel_id =
			channel_id_from_bytes(channel_id).map_err(|_| Error::<T>::DecodingError)?;
		let sequence = ibc::core::ics04_channel::packet::Sequence::from(seq);
		let acks_path = format!("{}", AcksPath { port_id, channel_id, sequence });
		let key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![acks_path])
			.map_err(|_| Error::<T>::DecodingError)?;

		Ok(QueryPacketAcknowledgementResponse { ack, trie_key: key, height: host_height::<T>() })
	}

	pub fn packet_receipt(
		channel_id: Vec<u8>,
		port_id: Vec<u8>,
		seq: u64,
	) -> Result<QueryPacketReceiptResponse, Error<T>> {
		let receipt = PacketReceipt::<T>::get((port_id.clone(), channel_id.clone(), seq));
		let receipt = String::from_utf8(receipt).map_err(|_| Error::<T>::DecodingError)?;
		let port_id = port_id_from_bytes(port_id).map_err(|_| Error::<T>::DecodingError)?;
		let channel_id =
			channel_id_from_bytes(channel_id).map_err(|_| Error::<T>::DecodingError)?;
		let sequence = ibc::core::ics04_channel::packet::Sequence::from(seq);
		let receipt_path = format!("{}", ReceiptsPath { port_id, channel_id, sequence });
		let key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![receipt_path])
			.map_err(|_| Error::<T>::DecodingError)?;
		let receipt = if &receipt == "Ok" { true } else { false };
		Ok(QueryPacketReceiptResponse { receipt, trie_key: key, height: host_height::<T>() })
	}

	pub fn connection_handshake(
		client_id: Vec<u8>,
		connection_id: Vec<u8>,
	) -> Result<ConnectionHandshake, Error<T>> {
		let client_state = ClientStates::<T>::get(client_id.clone());
		let client_state_decoded =
			AnyClientState::decode_vec(&client_state).map_err(|_| Error::<T>::DecodingError)?;
		let height = client_state_decoded.latest_height();
		let client_id =
			client_id_from_bytes(client_id.clone()).map_err(|_| Error::<T>::DecodingError)?;
		let connection_id = connection_id_from_bytes(connection_id.clone())
			.map_err(|_| Error::<T>::DecodingError)?;
		let prefix = T::CONNECTION_PREFIX;
		let connection_path = format!("{}", ConnectionsPath(connection_id));
		let consensus_path = ClientConsensusStatePath {
			client_id: client_id.clone(),
			epoch: height.revision_number,
			height: height.revision_height,
		};
		let client_state_path = format!("{}", ClientStatePath(client_id.clone()));
		let consensus_path = format!("{}", consensus_path);
		let client_state_key = apply_prefix_and_encode(prefix, vec![client_state_path])
			.map_err(|_| Error::<T>::DecodingError)?;
		let connection_key = apply_prefix_and_encode(prefix, vec![connection_path])
			.map_err(|_| Error::<T>::DecodingError)?;
		let consensus_key = apply_prefix_and_encode(prefix, vec![consensus_path])
			.map_err(|_| Error::<T>::DecodingError)?;

		Ok(ConnectionHandshake {
			client_state,
			trie_keys: vec![client_state_key, connection_key, consensus_key],
			height: host_height::<T>(),
		})
	}

	pub fn query_balance_with_address(addr: Vec<u8>) -> Result<u128, Error<T>> {
		let account_id =
			T::AccountId::decode(&mut &*addr).map_err(|_| Error::<T>::DecodingError)?;
		let balance = format!("{:?}", T::Currency::free_balance(&account_id));
		Ok(balance.parse().unwrap_or_default())
	}

	pub fn offchain_key(channel_id: Vec<u8>, port_id: Vec<u8>) -> Vec<u8> {
		let pair = (T::INDEXING_PREFIX.to_vec(), channel_id, port_id);
		pair.encode()
	}

	pub(crate) fn packet_cleanup() {
		for (port_id, channel_id) in Channels::<T>::iter_keys() {
			let key = Pallet::<T>::offchain_key(channel_id.clone(), port_id.clone());
			if let Some(mut offchain_packets) =
				sp_io::offchain::local_storage_get(sp_core::offchain::StorageKind::PERSISTENT, &key)
					.and_then(|v| BTreeMap::<u64, OffchainPacketType>::decode(&mut &*v).ok())
			{
				let keys: Vec<u64> = offchain_packets.clone().into_keys().collect();
				for key in keys {
					if !PacketCommitment::<T>::contains_key((
						port_id.clone(),
						channel_id.clone(),
						key,
					)) {
						let _ = offchain_packets.remove(&key);
					}
				}
				sp_io::offchain_index::set(&key, offchain_packets.encode().as_slice());
			}
		}
	}

	pub fn get_offchain_packets(
		channel_id: Vec<u8>,
		port_id: Vec<u8>,
		sequences: Vec<u64>,
	) -> Result<Vec<OffchainPacketType>, Error<T>> {
		let key = Pallet::<T>::offchain_key(channel_id, port_id);
		let offchain_packets: BTreeMap<u64, OffchainPacketType> =
			sp_io::offchain::local_storage_get(sp_core::offchain::StorageKind::PERSISTENT, &key)
				.and_then(|v| codec::Decode::decode(&mut &*v).ok())
				.unwrap_or_default();
		sequences
			.into_iter()
			.map(|seq| {
				offchain_packets
					.get(&seq)
					.map(|packet_ref| packet_ref.clone())
					.ok_or(Error::<T>::Other)
			})
			.collect()
	}
}

impl<T: Config + Send + Sync> IbcTrait for Pallet<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	fn send_packet(data: SendPacketData) -> Result<(), IbcHandlerError> {
		let channel_id = data.channel_id;
		let port_id = data.port_id;

		let connection_id = ChannelsConnection::<T>::iter()
			.find_map(|(connection, identifiers)| {
				if identifiers.contains(&(port_id.clone(), channel_id.clone())) {
					Some(connection)
				} else {
					None
				}
			})
			.ok_or(IbcHandlerError::SendPacketError)?;

		let client_id = ConnectionClient::<T>::iter()
			.find_map(
				|(client_id, conns)| {
					if conns.contains(&connection_id) {
						Some(client_id)
					} else {
						None
					}
				},
			)
			.ok_or(IbcHandlerError::SendPacketError)?;
		let client_state = ClientStates::<T>::get(client_id.clone());
		let client_state = AnyClientState::decode_vec(&client_state)
			.map_err(|_| IbcHandlerError::SendPacketError)?;
		let latest_height = client_state.latest_height();
		let encoded_height =
			latest_height.encode_vec().map_err(|_| IbcHandlerError::SendPacketError)?;
		let consensus_state = ConsensusStates::<T>::get(client_id, encoded_height);
		let consensus_state = AnyConsensusState::decode_vec(&consensus_state)
			.map_err(|_| IbcHandlerError::SendPacketError)?;
		let latest_timestamp = consensus_state.timestamp();
		let mut ctx = crate::routing::Context::<T>::new();
		let next_seq_send = NextSequenceSend::<T>::get(port_id.clone(), channel_id.clone());
		let sequence = Sequence::from(next_seq_send);
		let source_port =
			port_id_from_bytes(port_id.clone()).map_err(|_| IbcHandlerError::SendPacketError)?;
		let typed_cap: TypedCapability<PortCapabilityType> = data.capability.into();
		let port_cap: PortCapability = typed_cap.into();
		if !ctx.authenticate(source_port.clone(), &port_cap) {
			return Err(IbcHandlerError::InvalidCapability)
		}
		let source_channel = channel_id_from_bytes(channel_id.clone())
			.map_err(|_| IbcHandlerError::SendPacketError)?;
		let destination_port =
			port_id_from_bytes(data.dest_port_id).map_err(|_| IbcHandlerError::SendPacketError)?;
		let destination_channel = channel_id_from_bytes(data.dest_channel_id)
			.map_err(|_| IbcHandlerError::SendPacketError)?;
		let timestamp = (latest_timestamp + Duration::from_nanos(data.timeout_timestamp_offset))
			.map_err(|_| IbcHandlerError::SendPacketError)?;
		let packet = Packet {
			sequence,
			source_port,
			source_channel,
			destination_port,
			destination_channel,
			data: data.data,
			timeout_height: latest_height.add(data.timeout_height_offset),
			timeout_timestamp: timestamp,
		};

		let send_packet_result =
			ibc::core::ics04_channel::handler::send_packet::send_packet(&ctx, packet.clone())
				.map_err(|_| IbcHandlerError::SendPacketError)?;
		ctx.store_packet_result(send_packet_result.result)
			.map_err(|_| IbcHandlerError::SendPacketError)?;

		// store packet offchain
		let key = Pallet::<T>::offchain_key(channel_id, port_id);
		let mut offchain_packets: BTreeMap<u64, OffchainPacketType> =
			sp_io::offchain::local_storage_get(sp_core::offchain::StorageKind::PERSISTENT, &key)
				.and_then(|v| codec::Decode::decode(&mut &*v).ok())
				.unwrap_or_default();
		let offchain_packet: OffchainPacketType = packet.into();
		offchain_packets.insert(next_seq_send, offchain_packet);
		sp_io::offchain_index::set(&key, offchain_packets.encode().as_slice());
		Ok(())
	}

	fn bind_port(port_id: PortId) -> Result<PortCapability, IbcHandlerError> {
		let mut ctx = crate::routing::Context::<T>::new();
		let port_cap = ctx.bind_port(port_id).map_err(|_| IbcHandlerError::BindPortError)?;
		Ok(port_cap)
	}

	fn open_channel(
		port_id: PortId,
		capability: PortCapability,
		channel_end: ChannelEnd,
	) -> Result<ChannelId, IbcHandlerError> {
		let mut ctx = crate::routing::Context::<T>::new();
		if !ctx.authenticate(port_id.clone(), &capability) {
			return Err(IbcHandlerError::ChannelInitError)
		}
		let channel_counter =
			ctx.channel_counter().map_err(|_| IbcHandlerError::ChannelInitError)?;
		let channel_id = ChannelId::new(channel_counter);
		let value = MsgChannelOpenInit { port_id, channel: channel_end, signer: Signer::new("") }
			.encode_vec()
			.unwrap();
		let msg = ibc_proto::google::protobuf::Any {
			type_url: CHANNEL_OPEN_INIT_TYPE_URL.to_string(),
			value,
		};
		ibc::core::ics26_routing::handler::deliver(&mut ctx, msg)
			.map_err(|_| IbcHandlerError::ChannelInitError)?;
		Ok(channel_id)
	}
}

pub fn host_height<T: Config>() -> u64
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	let block_number: u32 = <frame_system::Pallet<T>>::block_number().into();
	block_number.into()
}
