use super::*;
use crate::routing::Context;
use codec::{Decode, Encode};
use composable_traits::{
	defi::DeFiComposableConfig,
	xcm::assets::{RemoteAssetRegistryInspect, RemoteAssetRegistryMutate, XcmAssetLocation},
};
use frame_support::traits::Currency;
use ibc::{
	applications::transfer::{
		acknowledgement::{Acknowledgement as Ics20Acknowledgement, ACK_SUCCESS_B64},
		error::Error as Ics20Error,
		packet::PacketData,
		relay::{
			on_ack_packet::process_ack_packet, on_recv_packet::process_recv_packet,
			on_timeout_packet::process_timeout_packet, send_transfer::send_transfer,
		},
	},
	core::{
		ics02_client::{
			client_state::{AnyClientState, ClientState},
			context::ClientReader,
		},
		ics04_channel::{
			channel::ChannelEnd,
			context::{ChannelKeeper, ChannelReader},
			msgs::{
				acknowledgement::Acknowledgement,
				chan_open_init::{MsgChannelOpenInit, TYPE_URL as CHANNEL_OPEN_INIT_TYPE_URL},
			},
			packet::{Packet, Sequence},
		},
		ics24_host::{
			identifier::*,
			path::{
				AcksPath, ChannelEndsPath, ClientConsensusStatePath, ClientStatePath,
				ClientTypePath, CommitmentsPath, ConnectionsPath, ReceiptsPath, SeqAcksPath,
				SeqRecvsPath, SeqSendsPath,
			},
		},
		ics26_routing::context::ModuleOutputBuilder,
	},
	handler::HandlerOutputBuilder,
	signer::Signer,
	Height,
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
use sp_runtime::traits::IdentifyAccount;
use tendermint_proto::Protobuf;

impl<T: Config> Pallet<T>
where
	T: Send + Sync,
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
			let height =
				ibc::Height::decode(&mut &*height).map_err(|_| Error::<T>::DecodingError)?;
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
			let channel_path = format!("{}", ChannelEndsPath(port_id.clone(), channel_id));
			let next_seq_send_path = format!("{}", SeqSendsPath(port_id.clone(), channel_id));
			let next_seq_recv_path = format!("{}", SeqRecvsPath(port_id.clone(), channel_id));
			let next_seq_ack_path = format!("{}", SeqAcksPath(port_id.clone(), channel_id));
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
		let port_id = port_id_from_bytes(port_id).map_err(|_| Error::<T>::DecodingError)?;
		let channel_id =
			channel_id_from_bytes(channel_id).map_err(|_| Error::<T>::DecodingError)?;

		let channel_path = format!("{}", ChannelEndsPath(port_id, channel_id));
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
			client_state.latest_height().encode_vec()
		} else {
			height
		};
		let consensus_state = ConsensusStates::<T>::get(client_id.clone(), height.clone());
		let client_id = client_id_from_bytes(client_id).map_err(|_| Error::<T>::DecodingError)?;

		let height = ibc::Height::decode(&mut &*height).map_err(|_| Error::<T>::DecodingError)?;
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
		let identifiers = ChannelsConnection::<T>::get(connection_id);

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
		let receipt = &receipt == "Ok";
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
		let client_id = client_id_from_bytes(client_id).map_err(|_| Error::<T>::DecodingError)?;
		let connection_id =
			connection_id_from_bytes(connection_id).map_err(|_| Error::<T>::DecodingError)?;
		let prefix = T::CONNECTION_PREFIX;
		let connection_path = format!("{}", ConnectionsPath(connection_id));
		let consensus_path = ClientConsensusStatePath {
			client_id: client_id.clone(),
			epoch: height.revision_number,
			height: height.revision_height,
		};
		let client_state_path = format!("{}", ClientStatePath(client_id));
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
		let hex_string = String::from_utf8(addr).map_err(|_| Error::<T>::DecodingError)?;
		let signer = Signer::from_str(&hex_string).map_err(|_| Error::<T>::DecodingError)?;
		let ibc_acc = <T as transfer::Config>::AccountIdConversion::try_from(signer)
			.map_err(|_| Error::<T>::DecodingError)?;
		let account_id = ibc_acc.into_account();
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
			.map(|seq| offchain_packets.get(&seq).cloned().ok_or(Error::<T>::Other))
			.collect()
	}

	pub fn host_consensus_state(height: u32) -> Option<Vec<u8>> {
		let ctx = Context::<T>::new();
		// revision number is not used in this case so it's fine to use zero
		let height = ibc::Height::new(0, height as u64);
		ctx.host_consensus_state(height).ok().map(|cs_state| cs_state.encode_vec())
	}
}

impl<T: Config> Pallet<T> {
	#[cfg(any(test, feature = "runtime-benchmarks"))]
	pub fn insert_default_consensus_state(height: u64) {
		let state = IbcConsensusState::default();
		HostConsensusStates::<T>::try_mutate::<_, (), _>(|val| {
			val.try_insert(height, state).unwrap();
			Ok(())
		})
		.unwrap();
	}
}

impl<T: Config + Send + Sync> IbcTrait for Pallet<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
	<T as DeFiComposableConfig>::MayBeAssetId:
		From<<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryMutate>::AssetId>,
	<T as DeFiComposableConfig>::MayBeAssetId:
		From<<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryInspect>::AssetId>,
	<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryInspect>::AssetId:
		From<<T as DeFiComposableConfig>::MayBeAssetId>,
	<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryMutate>::AssetId:
		From<<T as DeFiComposableConfig>::MayBeAssetId>,
	<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryInspect>::AssetNativeLocation:
		From<XcmAssetLocation>,
	<<T as transfer::Config>::AssetRegistry as RemoteAssetRegistryMutate>::AssetNativeLocation:
		From<XcmAssetLocation>,
	<T as DeFiComposableConfig>::MayBeAssetId: From<<T as assets::Config>::AssetId>,
	<T as DeFiComposableConfig>::MayBeAssetId: From<primitives::currency::CurrencyId>,
{
	fn client_revision_number(
		port_id: Vec<u8>,
		channel_id: Vec<u8>,
	) -> Result<u64, IbcHandlerError> {
		let connection_id = ChannelsConnection::<T>::iter()
			.find_map(|(connection, identifiers)| {
				if identifiers.contains(&(port_id.clone(), channel_id.clone())) {
					Some(connection)
				} else {
					None
				}
			})
			.ok_or(IbcHandlerError::ConnectionIdError)?;

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
			.ok_or(IbcHandlerError::ClientIdError)?;
		let client_state = ClientStates::<T>::get(client_id);
		let client_state = AnyClientState::decode_vec(&client_state)
			.map_err(|_| IbcHandlerError::ClientStateError)?;
		Ok(client_state.chain_id().version())
	}

	fn send_packet(data: SendPacketData) -> Result<(), IbcHandlerError> {
		let channel_id = data.channel_id;
		let port_id = data.port_id;

		let revision_number = if let Some(revision_number) = data.revision_number {
			revision_number
		} else {
			Self::client_revision_number(port_id.clone(), channel_id.clone())?
		};
		let mut ctx = crate::routing::Context::<T>::new();
		let next_seq_send = NextSequenceSend::<T>::get(port_id.clone(), channel_id.clone());
		let sequence = Sequence::from(next_seq_send);
		let source_port =
			port_id_from_bytes(port_id).map_err(|_| IbcHandlerError::ChannelOrPortError)?;
		let source_channel =
			channel_id_from_bytes(channel_id).map_err(|_| IbcHandlerError::ChannelOrPortError)?;
		let source_channel_end = ctx
			.channel_end(&(source_port.clone(), source_channel))
			.map_err(|_| IbcHandlerError::ChannelOrPortError)?;

		let destination_port = source_channel_end.counterparty().port_id().clone();
		let destination_channel = *source_channel_end
			.counterparty()
			.channel_id()
			.ok_or(IbcHandlerError::ChannelOrPortError)?;
		let timestamp = ibc::timestamp::Timestamp::from_nanoseconds(data.timeout_timestamp)
			.map_err(|_| IbcHandlerError::TimestampOrHeightError)?;
		let packet = Packet {
			sequence,
			source_port,
			source_channel,
			destination_port,
			destination_channel,
			data: data.data,
			timeout_height: Height::new(revision_number, data.timeout_height),
			timeout_timestamp: timestamp,
		};

		let send_packet_result =
			ibc::core::ics04_channel::handler::send_packet::send_packet(&ctx, packet)
				.map_err(|_| IbcHandlerError::SendPacketError)?;
		ctx.store_packet_result(send_packet_result.result)
			.map_err(|_| IbcHandlerError::SendPacketError)?;
		Ok(())
	}

	fn open_channel(
		port_id: PortId,
		channel_end: ChannelEnd,
	) -> Result<ChannelId, IbcHandlerError> {
		let mut ctx = crate::routing::Context::<T>::new();
		let channel_counter =
			ctx.channel_counter().map_err(|_| IbcHandlerError::ChannelInitError)?;
		let channel_id = ChannelId::new(channel_counter);
		// Signer does not matter in this case
		let value = MsgChannelOpenInit {
			port_id,
			channel: channel_end,
			signer: Signer::from_str(MODULE_ID).map_err(|_| IbcHandlerError::ChannelInitError)?,
		}
		.encode_vec();
		let msg = ibc_proto::google::protobuf::Any {
			type_url: CHANNEL_OPEN_INIT_TYPE_URL.to_string(),
			value,
		};
		ibc::core::ics26_routing::handler::deliver::<_, crate::host_functions::HostFunctions>(
			&mut ctx, msg,
		)
		.map_err(|_| IbcHandlerError::ChannelInitError)?;
		Ok(channel_id)
	}

	fn send_transfer(
		msg: ibc::applications::transfer::msgs::transfer::MsgTransfer<
			ibc::applications::transfer::PrefixedCoin,
		>,
	) -> Result<(), IbcHandlerError> {
		let mut handler_output = HandlerOutputBuilder::default();
		let mut ctx = Context::<T>::default();
		send_transfer::<_, _>(&mut ctx, &mut handler_output, msg)
			.map_err(|_| IbcHandlerError::SendTransferError)?;
		Ok(())
	}

	fn on_receive_packet(
		output: &mut ModuleOutputBuilder,
		packet: &Packet,
	) -> Result<(), IbcHandlerError> {
		let mut ctx = Context::<T>::default();
		let packet_data: PacketData = serde_json::from_slice(packet.data.as_slice())
			.map_err(|_| IbcHandlerError::DecodingError)?;
		process_recv_packet(&ctx, output, packet, packet_data)
			.and_then(|write_fn| write_fn(&mut ctx).map_err(Ics20Error::unknown_msg_type))
			.map_err(|_| IbcHandlerError::ReceivePacketError)
	}

	fn on_ack_packet(
		_output: &mut ModuleOutputBuilder,
		packet: &Packet,
		ack: &Acknowledgement,
	) -> Result<(), IbcHandlerError> {
		let mut ctx = Context::<T>::default();
		let packet_data: PacketData = serde_json::from_slice(packet.data.as_slice())
			.map_err(|_| IbcHandlerError::DecodingError)?;
		let ack = String::from_utf8(ack.as_ref().to_vec())
			.map(|val| {
				if val.as_bytes() == ACK_SUCCESS_B64 {
					Ics20Acknowledgement::Success(ACK_SUCCESS_B64.to_vec())
				} else {
					Ics20Acknowledgement::Error(val)
				}
			})
			.map_err(|_| IbcHandlerError::DecodingError)?;
		process_ack_packet(&mut ctx, packet, &packet_data, &ack)
			.map_err(|_| IbcHandlerError::AcknowledgementError)
	}

	fn on_timeout_packet(
		_output: &mut ModuleOutputBuilder,
		packet: &Packet,
	) -> Result<(), IbcHandlerError> {
		let mut ctx = Context::<T>::default();
		let packet_data: PacketData = serde_json::from_slice(packet.data.as_slice())
			.map_err(|_| IbcHandlerError::DecodingError)?;
		process_timeout_packet(&mut ctx, packet, &packet_data)
			.map_err(|_| IbcHandlerError::TimeoutError)
	}

	fn write_acknowlegdement(packet: &Packet, ack: Vec<u8>) -> Result<(), IbcHandlerError> {
		let mut ctx = Context::<T>::default();
		let ack = ctx.ack_commitment(ack.into());
		ctx.store_packet_acknowledgement(
			(packet.source_port.clone(), packet.source_channel, packet.sequence),
			ack,
		)
		.map_err(|_| IbcHandlerError::WriteAcknowledgementError)
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn create_client() -> Result<ClientId, IbcHandlerError> {
		use crate::benchmarks::tendermint_benchmark_utils::create_mock_state;
		use ibc::core::ics02_client::{
			client_consensus::AnyConsensusState,
			msgs::create_client::{MsgCreateAnyClient, TYPE_URL},
		};

		let (mock_client_state, mock_cs_state) = create_mock_state();
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let msg = MsgCreateAnyClient::new(
			AnyClientState::Tendermint(mock_client_state),
			Some(AnyConsensusState::Tendermint(mock_cs_state)),
			Signer::from_str("pallet_ibc").unwrap(),
		)
		.unwrap()
		.encode_vec();
		let msg = ibc_proto::google::protobuf::Any { type_url: TYPE_URL.to_string(), value: msg };
		let mut ctx = crate::routing::Context::<T>::new();
		ibc::core::ics26_routing::handler::deliver::<_, crate::host_functions::HostFunctions>(
			&mut ctx, msg,
		)
		.unwrap();
		Ok(client_id)
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn create_connection(
		client_id: ClientId,
		connection_id: ConnectionId,
	) -> Result<(), IbcHandlerError> {
		use ibc::core::ics03_connection::{
			connection::{ConnectionEnd, Counterparty, State},
			context::ConnectionKeeper,
			version::Version,
		};
		let delay_period = core::time::Duration::from_nanos(1000);
		let counter_party = Counterparty::new(
			client_id.clone(),
			Some(ConnectionId::new(1)),
			<T as Config>::CONNECTION_PREFIX.to_vec().try_into().unwrap(),
		);
		let connection_end = ConnectionEnd::new(
			State::Open,
			client_id.clone(),
			counter_party,
			vec![Version::default()],
			delay_period,
		);
		let mut ctx = crate::routing::Context::<T>::new();
		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();
		Ok(())
	}
}

pub fn host_height<T: Config>() -> u64
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	let block_number: u32 = <frame_system::Pallet<T>>::block_number().into();
	block_number.into()
}
