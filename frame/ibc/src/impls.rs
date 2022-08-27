use core::time::Duration;

use super::*;
use crate::{
	events::IbcEvent,
	host_functions::HostFunctions,
	ics23::{
		acknowledgements::Acknowledgements, channels::Channels, client_states::ClientStates,
		connections::Connections, consensus_states::ConsensusStates,
		next_seq_recv::NextSequenceRecv, packet_commitments::PacketCommitment,
		reciepts::PacketReceipt,
	},
	routing::Context,
};
use codec::{Decode, Encode};
use composable_traits::{
	defi::DeFiComposableConfig,
	xcm::assets::{RemoteAssetRegistryInspect, RemoteAssetRegistryMutate, XcmAssetLocation},
};
use frame_support::traits::Currency;
use ibc::{
	applications::transfer::{relay::send_transfer::send_transfer, PrefixedCoin},
	core::{
		ics02_client::{client_state::AnyClientState, context::ClientReader},
		ics03_connection::context::ConnectionReader,
		ics04_channel::{
			channel::ChannelEnd,
			context::{ChannelKeeper, ChannelReader},
			error::Error as Ics04Error,
			msgs::chan_open_init::{MsgChannelOpenInit, TYPE_URL as CHANNEL_OPEN_INIT_TYPE_URL},
			packet::{Packet, Sequence},
		},
		ics24_host::{
			identifier::*,
			path::{
				AcksPath, ChannelEndsPath, ClientConsensusStatePath, ClientStatePath,
				CommitmentsPath, ConnectionsPath, ReceiptsPath, SeqRecvsPath,
			},
		},
		ics26_routing::handler::MsgReceipt,
	},
	handler::HandlerOutputBuilder,
	signer::Signer,
	timestamp::Timestamp,
	Height,
};
use ibc_primitives::{
	apply_prefix, channel_id_from_bytes, client_id_from_bytes, connection_id_from_bytes,
	get_channel_escrow_address, port_id_from_bytes, ConnectionHandshake, Error as IbcHandlerError,
	IbcHandler, IdentifiedChannel, IdentifiedClientState, IdentifiedConnection, OffchainPacketType,
	PacketState, QueryChannelResponse, QueryChannelsResponse, QueryClientStateResponse,
	QueryConnectionResponse, QueryConnectionsResponse, QueryConsensusStateResponse,
	QueryNextSequenceReceiveResponse, QueryPacketAcknowledgementResponse,
	QueryPacketAcknowledgementsResponse, QueryPacketCommitmentResponse,
	QueryPacketCommitmentsResponse, QueryPacketReceiptResponse, SendPacketData,
};
use primitives::currency::CurrencyId;
use scale_info::prelude::{collections::BTreeMap, string::ToString};
use sp_runtime::traits::IdentifyAccount;
use tendermint_proto::Protobuf;

impl<T: Config> Pallet<T>
where
	T: Send + Sync,
	<T as DeFiComposableConfig>::MayBeAssetId: From<CurrencyId>,
	u32: From<<T as frame_system::Config>::BlockNumber>,
	<T as DeFiComposableConfig>::MayBeAssetId:
		From<<T::AssetRegistry as RemoteAssetRegistryMutate>::AssetId>,
	<T as DeFiComposableConfig>::MayBeAssetId:
		From<<T::AssetRegistry as RemoteAssetRegistryInspect>::AssetId>,
	<T::AssetRegistry as RemoteAssetRegistryInspect>::AssetId:
		From<<T as DeFiComposableConfig>::MayBeAssetId>,
	<T::AssetRegistry as RemoteAssetRegistryMutate>::AssetId:
		From<<T as DeFiComposableConfig>::MayBeAssetId>,
	<T::AssetRegistry as RemoteAssetRegistryInspect>::AssetNativeLocation: From<XcmAssetLocation>,
	<T::AssetRegistry as RemoteAssetRegistryMutate>::AssetNativeLocation: From<XcmAssetLocation>,
	<T as DeFiComposableConfig>::MayBeAssetId: From<<T as assets::Config>::AssetId>,
{
	pub fn execute_ibc_messages(
		ctx: &mut Context<T>,
		messages: Vec<ibc_proto::google::protobuf::Any>,
	) {
		let (events, logs, errors) = messages.into_iter().fold(
			(vec![], vec![], vec![]),
			|(mut events, mut logs, mut errors), msg| {
				match ibc::core::ics26_routing::handler::deliver::<_, HostFunctions<T>>(ctx, msg) {
					Ok(MsgReceipt { events: temp_events, log: temp_logs }) => {
						events.extend(temp_events);
						logs.extend(temp_logs);
					},
					Err(e) => errors.push(e),
				}
				(events, logs, errors)
			},
		);

		log::trace!(target: "pallet_ibc", "logs: {:#?}", logs);
		log::trace!(target: "pallet_ibc", "errors: {:#?}", errors);

		// todo: consolidate into one.
		if !events.is_empty() {
			Self::deposit_event(events.into())
		};
		if !errors.is_empty() {
			Self::deposit_event(errors.into())
		};
	}
}

impl<T: Config> Pallet<T>
where
	T: Send + Sync,
	u32: From<<T as frame_system::Config>::BlockNumber>,
	<T as DeFiComposableConfig>::MayBeAssetId: From<primitives::currency::CurrencyId>,
{
	pub fn timestamp() -> u64 {
		use frame_support::traits::UnixTime;
		use sp_runtime::traits::SaturatedConversion;
		let time = T::TimeProvider::now();
		time.as_nanos().saturated_into::<u64>()
	}

	// IBC Runtime Api helper methods
	/// Get a channel state
	pub fn channel(
		channel_id: Vec<u8>,
		port_id: Vec<u8>,
	) -> Result<QueryChannelResponse, Error<T>> {
		let port_id = port_id_from_bytes(port_id).map_err(|_| Error::<T>::DecodingError)?;
		let channel_id =
			channel_id_from_bytes(channel_id).map_err(|_| Error::<T>::DecodingError)?;
		let channel =
			Channels::<T>::get(port_id.clone(), channel_id).ok_or(Error::<T>::ChannelNotFound)?;
		let channel_path = format!("{}", ChannelEndsPath(port_id, channel_id));
		let key = apply_prefix(T::CONNECTION_PREFIX, vec![channel_path]);

		Ok(QueryChannelResponse { channel, trie_key: key, height: host_height::<T>() })
	}

	/// Get a connection state
	pub fn connection(connection_id: Vec<u8>) -> Result<QueryConnectionResponse, Error<T>> {
		let connection_id =
			connection_id_from_bytes(connection_id).map_err(|_| Error::<T>::DecodingError)?;
		let connection =
			Connections::<T>::get(&connection_id).ok_or(Error::<T>::ConnectionNotFound)?;

		let connection_path = format!("{}", ConnectionsPath(connection_id));
		let key = apply_prefix(T::CONNECTION_PREFIX, vec![connection_path]);

		Ok(QueryConnectionResponse { connection, trie_key: key, height: host_height::<T>() })
	}

	/// Get a client state
	pub fn client(client_id: Vec<u8>) -> Result<QueryClientStateResponse, Error<T>> {
		let client_id = client_id_from_bytes(client_id).map_err(|_| Error::<T>::DecodingError)?;
		let client_state =
			ClientStates::<T>::get(&client_id).ok_or(Error::<T>::ClientStateNotFound)?;
		let client_state_path = format!("{}", ClientStatePath(client_id));

		let key = apply_prefix(T::CONNECTION_PREFIX, vec![client_state_path]);

		Ok(QueryClientStateResponse { client_state, trie_key: key, height: host_height::<T>() })
	}

	/// Get all client states
	/// Returns a Vec of (client_id, client_state)
	pub fn clients() -> Vec<(Vec<u8>, Vec<u8>)> {
		ClientStates::<T>::iter()
			.map(|(client_id, client_state)| (client_id.as_bytes().to_vec(), client_state))
			.collect::<Vec<_>>()
	}

	/// Get a consensus state for client
	pub fn consensus_state(
		height: Vec<u8>,
		client_id: Vec<u8>,
		latest_cs: bool,
	) -> Result<QueryConsensusStateResponse, Error<T>> {
		let client_id = client_id_from_bytes(client_id).map_err(|_| Error::<T>::DecodingError)?;
		let height = if latest_cs {
			let client_state =
				ClientStates::<T>::get(&client_id).ok_or(Error::<T>::ClientStateNotFound)?;
			let client_state =
				AnyClientState::decode_vec(&client_state).map_err(|_| Error::<T>::DecodingError)?;
			client_state.latest_height()
		} else {
			Height::decode_vec(&height).map_err(|_| Error::<T>::DecodingError)?
		};
		let consensus_state = ConsensusStates::<T>::get(client_id.clone(), height)
			.ok_or(Error::<T>::ConsensusStateNotFound)?;

		let consensus_path = ClientConsensusStatePath {
			client_id,
			epoch: height.revision_number,
			height: height.revision_height,
		};

		let path = format!("{}", consensus_path);
		let key = apply_prefix(T::CONNECTION_PREFIX, vec![path]);

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
			.filter_map(|connection_id| {
				let conn_id = connection_id_from_bytes(connection_id.clone()).ok()?;

				Some(IdentifiedConnection {
					connection_end: Connections::<T>::get(&conn_id)?,
					connection_id,
				})
			})
			.collect::<Vec<_>>();

		Ok(connections)
	}

	fn channel_client_id(channel_end: &ChannelEnd) -> Result<ClientId, Error<T>> {
		let ctx = Context::<T>::default();
		let connection_id =
			channel_end.connection_hops.get(0).ok_or(Error::<T>::ConnectionNotFound)?;
		let connection_end =
			ctx.connection_end(connection_id).map_err(|_| Error::<T>::ConnectionNotFound)?;
		Ok(connection_end.client_id().clone())
	}

	/// Get client state for client which this channel is bound to
	pub fn channel_client(
		channel_id: Vec<u8>,
		port_id: Vec<u8>,
	) -> Result<IdentifiedClientState, Error<T>> {
		let channel_id =
			channel_id_from_bytes(channel_id).map_err(|_| Error::<T>::InvalidChannelId)?;
		let port_id = port_id_from_bytes(port_id).map_err(|_| Error::<T>::InvalidPortId)?;
		let ctx = Context::<T>::new();
		let channel_end = ctx
			.channel_end(&(port_id, channel_id))
			.map_err(|_| Error::<T>::ChannelNotFound)?;
		let client_id = Self::channel_client_id(&channel_end)?;
		let client_state =
			ClientStates::<T>::get(&client_id).ok_or(Error::<T>::ClientStateNotFound)?;
		Ok(IdentifiedClientState { client_id: client_id.as_bytes().to_vec(), client_state })
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
			.map(|(port_id_bytes, channel_id_bytes)| {
				let channel_id = channel_id_from_bytes(channel_id_bytes.clone())
					.map_err(|_| Error::<T>::DecodingError)?;
				let port_id = port_id_from_bytes(port_id_bytes.clone())
					.map_err(|_| Error::<T>::DecodingError)?;

				let channel_end =
					Channels::<T>::get(port_id, channel_id).ok_or(Error::<T>::ChannelNotFound)?;
				Ok(IdentifiedChannel {
					channel_id: channel_id_bytes,
					port_id: port_id_bytes,
					channel_end,
				})
			})
			.collect::<Result<Vec<_>, Error<T>>>()?;
		Ok(QueryChannelsResponse { channels, height: host_height::<T>() })
	}

	pub fn packet_commitments(
		channel_id_bytes: Vec<u8>,
		port_id_bytes: Vec<u8>,
	) -> Result<QueryPacketCommitmentsResponse, Error<T>> {
		let channel_id = channel_id_from_bytes(channel_id_bytes.clone())
			.map_err(|_| Error::<T>::DecodingError)?;
		let port_id =
			port_id_from_bytes(port_id_bytes.clone()).map_err(|_| Error::<T>::DecodingError)?;
		let commitments = PacketCommitment::<T>::iter()
			.filter_map(|((p, c, s), commitment)| {
				if p == port_id && c == channel_id {
					let packet_state = PacketState {
						port_id: port_id_bytes.clone(),
						channel_id: channel_id_bytes.clone(),
						sequence: s.into(),
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
		channel_id_bytes: Vec<u8>,
		port_id_bytes: Vec<u8>,
	) -> Result<QueryPacketAcknowledgementsResponse, Error<T>> {
		let channel_id = channel_id_from_bytes(channel_id_bytes.clone())
			.map_err(|_| Error::<T>::DecodingError)?;
		let port_id =
			port_id_from_bytes(port_id_bytes.clone()).map_err(|_| Error::<T>::DecodingError)?;
		let acks = Acknowledgements::<T>::iter()
			.filter_map(|((p, c, s), ack)| {
				if p == port_id && c == channel_id {
					let packet_state = PacketState {
						port_id: port_id_bytes.clone(),
						channel_id: channel_id_bytes.clone(),
						sequence: s.into(),
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
		let channel_id =
			channel_id_from_bytes(channel_id).map_err(|_| Error::<T>::DecodingError)?;
		let port_id = port_id_from_bytes(port_id).map_err(|_| Error::<T>::DecodingError)?;
		Ok(seqs
			.into_iter()
			.filter(|s| {
				!PacketReceipt::<T>::contains_key((port_id.clone(), channel_id, (*s).into()))
			})
			.collect())
	}

	pub fn unreceived_acknowledgements(
		channel_id_bytes: Vec<u8>,
		port_id_bytes: Vec<u8>,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Error<T>> {
		let channel_id =
			channel_id_from_bytes(channel_id_bytes).map_err(|_| Error::<T>::DecodingError)?;
		let port_id = port_id_from_bytes(port_id_bytes).map_err(|_| Error::<T>::DecodingError)?;
		Ok(seqs
			.into_iter()
			.filter(|s| {
				PacketCommitment::<T>::contains_key((port_id.clone(), channel_id, (*s).into()))
			})
			.collect())
	}

	pub fn next_seq_recv(
		channel_id: Vec<u8>,
		port_id: Vec<u8>,
	) -> Result<QueryNextSequenceReceiveResponse, Error<T>> {
		let port_id = port_id_from_bytes(port_id).map_err(|_| Error::<T>::DecodingError)?;
		let channel_id =
			channel_id_from_bytes(channel_id).map_err(|_| Error::<T>::DecodingError)?;
		let sequence = NextSequenceRecv::<T>::get(port_id.clone(), channel_id)
			.ok_or(Error::<T>::SendPacketError)?;
		let next_seq_recv_path = format!("{}", SeqRecvsPath(port_id, channel_id));
		let key = apply_prefix(T::CONNECTION_PREFIX, vec![next_seq_recv_path]);

		Ok(QueryNextSequenceReceiveResponse { sequence, trie_key: key, height: host_height::<T>() })
	}

	pub fn packet_commitment(
		channel_id: Vec<u8>,
		port_id: Vec<u8>,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse, Error<T>> {
		let port_id = port_id_from_bytes(port_id).map_err(|_| Error::<T>::DecodingError)?;
		let channel_id =
			channel_id_from_bytes(channel_id).map_err(|_| Error::<T>::DecodingError)?;
		let commitment = PacketCommitment::<T>::get((port_id.clone(), channel_id, seq.into()))
			.ok_or(Error::<T>::PacketCommitmentNotFound)?;
		let sequence = ibc::core::ics04_channel::packet::Sequence::from(seq);
		let commitment_path = format!("{}", CommitmentsPath { port_id, channel_id, sequence });
		let key = apply_prefix(T::CONNECTION_PREFIX, vec![commitment_path]);

		Ok(QueryPacketCommitmentResponse { commitment, trie_key: key, height: host_height::<T>() })
	}

	pub fn packet_acknowledgement(
		channel_id: Vec<u8>,
		port_id: Vec<u8>,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse, Error<T>> {
		let port_id = port_id_from_bytes(port_id).map_err(|_| Error::<T>::DecodingError)?;
		let channel_id =
			channel_id_from_bytes(channel_id).map_err(|_| Error::<T>::DecodingError)?;
		let sequence = ibc::core::ics04_channel::packet::Sequence::from(seq);
		let ack = Acknowledgements::<T>::get((port_id.clone(), channel_id, sequence))
			.ok_or(Error::<T>::PacketCommitmentNotFound)?;
		let acks_path = format!("{}", AcksPath { port_id, channel_id, sequence });
		let key = apply_prefix(T::CONNECTION_PREFIX, vec![acks_path]);

		Ok(QueryPacketAcknowledgementResponse { ack, trie_key: key, height: host_height::<T>() })
	}

	pub fn packet_receipt(
		channel_id: Vec<u8>,
		port_id: Vec<u8>,
		seq: u64,
	) -> Result<QueryPacketReceiptResponse, Error<T>> {
		let port_id = port_id_from_bytes(port_id).map_err(|_| Error::<T>::DecodingError)?;
		let channel_id =
			channel_id_from_bytes(channel_id).map_err(|_| Error::<T>::DecodingError)?;
		let sequence = ibc::core::ics04_channel::packet::Sequence::from(seq);
		let receipt = PacketReceipt::<T>::get((port_id.clone(), channel_id, sequence))
			.ok_or(Error::<T>::PacketReceiptNotFound)?;
		let receipt = String::from_utf8(receipt).map_err(|_| Error::<T>::DecodingError)?;
		let receipt_path = format!("{}", ReceiptsPath { port_id, channel_id, sequence });
		let key = apply_prefix(T::CONNECTION_PREFIX, vec![receipt_path]);
		let receipt = &receipt == "Ok";
		Ok(QueryPacketReceiptResponse { receipt, trie_key: key, height: host_height::<T>() })
	}

	pub fn connection_handshake(
		client_id: Vec<u8>,
		connection_id: Vec<u8>,
	) -> Result<ConnectionHandshake, Error<T>> {
		let client_id = client_id_from_bytes(client_id).map_err(|_| Error::<T>::DecodingError)?;
		let client_state =
			ClientStates::<T>::get(&client_id).ok_or(Error::<T>::ClientStateNotFound)?;
		let client_state_decoded =
			AnyClientState::decode_vec(&client_state).map_err(|_| Error::<T>::DecodingError)?;
		let height = client_state_decoded.latest_height();
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
		let client_state_key = apply_prefix(prefix, vec![client_state_path]);
		let connection_key = apply_prefix(prefix, vec![connection_path]);
		let consensus_key = apply_prefix(prefix, vec![consensus_path]);
		Ok(ConnectionHandshake {
			client_state,
			trie_keys: vec![client_state_key, connection_key, consensus_key],
			height: host_height::<T>(),
		})
	}

	pub fn query_balance_with_address(addr: Vec<u8>) -> Result<u128, Error<T>> {
		let hex_string = String::from_utf8(addr).map_err(|_| Error::<T>::DecodingError)?;
		let signer = Signer::from_str(&hex_string).map_err(|_| Error::<T>::DecodingError)?;
		let ibc_acc =
			T::AccountIdConversion::try_from(signer).map_err(|_| Error::<T>::DecodingError)?;
		let account_id = ibc_acc.into_account();
		let balance = format!("{:?}", T::Currency::free_balance(&account_id));
		Ok(balance.parse().unwrap_or_default())
	}

	pub fn offchain_key(channel_id: Vec<u8>, port_id: Vec<u8>) -> Vec<u8> {
		let pair = (T::INDEXING_PREFIX.to_vec(), channel_id, port_id);
		pair.encode()
	}

	pub fn acknowledgements_offchain_key(channel_id: Vec<u8>, port_id: Vec<u8>) -> Vec<u8> {
		let pair = (T::INDEXING_PREFIX.to_vec(), channel_id, port_id, b"ACK");
		pair.encode()
	}

	fn store_raw_acknowledgement(
		key: (PortId, ChannelId, Sequence),
		ack: Vec<u8>,
	) -> Result<(), Error<T>> {
		// store packet offchain
		let channel_id = key.1.to_string().as_bytes().to_vec();
		let port_id = key.0.as_bytes().to_vec();
		let seq = u64::from(key.2);
		// let key = Pallet::<T>::acknowledgements_offchain_key(channel_id, port_id);
		// let mut acks: BTreeMap<u64, Vec<u8>> =
		// 	sp_io::offchain::local_storage_get(sp_core::offchain::StorageKind::PERSISTENT, &key)
		// 		.and_then(|v| codec::Decode::decode(&mut &*v).ok())
		// 		.unwrap_or_default();
		// acks.insert(seq, ack);
		// sp_io::offchain_index::set(&key, acks.encode().as_slice());
		RawAcknowledgements::<T>::insert((channel_id, port_id), seq, ack);
		Ok(())
	}

	pub(crate) fn packet_cleanup() -> Result<(), Error<T>> {
		for (port_id_bytes, channel_id_bytes, _) in Channels::<T>::iter() {
			let channel_id = channel_id_from_bytes(channel_id_bytes.clone())
				.map_err(|_| Error::<T>::DecodingError)?;
			let port_id =
				port_id_from_bytes(port_id_bytes.clone()).map_err(|_| Error::<T>::DecodingError)?;

			let key = Pallet::<T>::offchain_key(channel_id_bytes.clone(), port_id_bytes.clone());
			let ack_key = Pallet::<T>::acknowledgements_offchain_key(
				channel_id_bytes.clone(),
				port_id_bytes.clone(),
			);
			// Clean up offchain packets
			if let Some(mut offchain_packets) =
				sp_io::offchain::local_storage_get(sp_core::offchain::StorageKind::PERSISTENT, &key)
					.and_then(|v| BTreeMap::<u64, OffchainPacketType>::decode(&mut &*v).ok())
			{
				let keys: Vec<u64> = offchain_packets.clone().into_keys().collect();
				for key in keys {
					if !PacketCommitment::<T>::contains_key((
						port_id.clone(),
						channel_id,
						key.into(),
					)) {
						let _ = offchain_packets.remove(&key);
					}
				}
				sp_io::offchain_index::set(&key, offchain_packets.encode().as_slice());
			}

			// Clean up offchain acknowledgements
			if let Some(mut acks) = sp_io::offchain::local_storage_get(
				sp_core::offchain::StorageKind::PERSISTENT,
				&ack_key,
			)
			.and_then(|v| BTreeMap::<u64, Vec<u8>>::decode(&mut &*v).ok())
			{
				let keys: Vec<u64> = acks.clone().into_keys().collect();
				for key in keys {
					if !Acknowledgements::<T>::contains_key((
						port_id.clone(),
						channel_id,
						key.into(),
					)) {
						let _ = acks.remove(&key);
					}
				}
				sp_io::offchain_index::set(&key, acks.encode().as_slice());
			}
		}

		Ok(())
	}

	pub fn get_offchain_packets(
		channel_id: Vec<u8>,
		port_id: Vec<u8>,
		sequences: Vec<u64>,
	) -> Result<Vec<OffchainPacketType>, Error<T>> {
		// let key = Pallet::<T>::offchain_key(channel_id, port_id);
		// let offchain_packets: BTreeMap<u64, OffchainPacketType> =
		// 	sp_io::offchain::local_storage_get(sp_core::offchain::StorageKind::PERSISTENT, &key)
		// 		.and_then(|v| codec::Decode::decode(&mut &*v).ok())
		// 		.unwrap_or_default();
		let packets = sequences
			.into_iter()
			.map(|seq| Packets::<T>::get((channel_id.clone(), port_id.clone()), seq))
			.collect();
		Ok(packets)
	}

	pub fn get_offchain_acks(
		channel_id: Vec<u8>,
		port_id: Vec<u8>,
		sequences: Vec<u64>,
	) -> Result<Vec<Vec<u8>>, Error<T>> {
		// let key = Pallet::<T>::acknowledgements_offchain_key(channel_id, port_id);
		// let acks: BTreeMap<u64, Vec<u8>> =
		// 	sp_io::offchain::local_storage_get(sp_core::offchain::StorageKind::PERSISTENT, &key)
		// 		.and_then(|v| codec::Decode::decode(&mut &*v).ok())
		// 		.unwrap_or_default();
		let acks = sequences
			.into_iter()
			.map(|seq| RawAcknowledgements::<T>::get((channel_id.clone(), port_id.clone()), seq))
			.collect();
		Ok(acks)
	}

	pub fn host_consensus_state(height: u32) -> Option<Vec<u8>> {
		let ctx = Context::<T>::new();
		// revision number is not used in this case so it's fine to use zero
		let height = Height::new(0, height as u64);
		ctx.host_consensus_state(height).ok().map(|cs_state| cs_state.encode_vec())
	}
}

impl<T: Config> Pallet<T> {
	pub fn is_send_enabled() -> bool {
		Params::<T>::get().send_enabled
	}

	pub fn is_receive_enabled() -> bool {
		Params::<T>::get().receive_enabled
	}

	pub fn register_asset_id(asset_id: <T as DeFiComposableConfig>::MayBeAssetId, denom: Vec<u8>) {
		IbcAssetIds::<T>::insert(asset_id, denom.clone());
		IbcDenoms::<T>::insert(denom, asset_id);
	}

	pub fn remove_channel_escrow_address(
		port_id: &PortId,
		channel_id: ChannelId,
	) -> Result<(), Ics04Error> {
		let escrow_address = get_channel_escrow_address(port_id, channel_id).map_err(|_| {
			Ics04Error::implementation_specific(
				"Failed to derive channel escrow address for removal".to_string(),
			)
		})?;
		let account_id = T::AccountIdConversion::try_from(escrow_address)
			.map_err(|_| {
				Ics04Error::implementation_specific(
					"Failed to derive channel escrow address for removal".to_string(),
				)
			})?
			.into_account();
		let _ = EscrowAddresses::<T>::try_mutate::<_, &'static str, _>(|addresses| {
			addresses.remove(&account_id);
			Ok(())
		});
		Ok(())
	}

	/// Returns true if address provided is an escrow address
	pub fn is_escrow_address(address: T::AccountId) -> bool {
		let set = EscrowAddresses::<T>::get();
		set.contains(&address)
	}
}

impl<T: Config> Pallet<T>
where
	<T as DeFiComposableConfig>::MayBeAssetId: From<CurrencyId>,
	CurrencyId: From<<T as DeFiComposableConfig>::MayBeAssetId>,
{
	pub fn get_denom_trace(asset_id: u128) -> Option<ibc_primitives::QueryDenomTraceResponse> {
		let asset_id: <T as DeFiComposableConfig>::MayBeAssetId = CurrencyId(asset_id).into();
		IbcAssetIds::<T>::get(asset_id)
			.map(|denom| ibc_primitives::QueryDenomTraceResponse { denom })
	}

	pub fn get_denom_traces(
		key: Option<u128>,
		offset: Option<u32>,
		mut limit: u64,
		count_total: bool,
	) -> ibc_primitives::QueryDenomTracesResponse {
		let mut iterator = if let Some(key) = key {
			let asset_id: <T as DeFiComposableConfig>::MayBeAssetId = CurrencyId(key).into();
			let raw_key = asset_id.encode();
			IbcAssetIds::<T>::iter_from(raw_key).skip(0)
		} else if let Some(offset) = offset {
			IbcAssetIds::<T>::iter().skip(offset as usize)
		} else {
			IbcAssetIds::<T>::iter().skip(0)
		};

		let mut denoms = vec![];
		for (_, denom) in iterator.by_ref() {
			denoms.push(denom);
			limit -= 1;
			if limit == 0 {
				break
			}
		}

		ibc_primitives::QueryDenomTracesResponse {
			denoms,
			total: count_total.then(|| IbcAssetIds::<T>::count() as u64),
			next_key: iterator.next().map(|(key, _)| {
				let asset_id: CurrencyId = key.into();
				asset_id.0
			}),
		}
	}
}

impl<T: Config> Pallet<T>
where
	<T as DeFiComposableConfig>::MayBeAssetId: From<CurrencyId>,
{
	pub fn ibc_denom_to_asset_id(
		full_denom: String,
		token: PrefixedCoin,
	) -> Option<<T as DeFiComposableConfig>::MayBeAssetId> {
		let is_local_asset = token.denom.trace_path().is_empty();
		if is_local_asset {
			if let Ok(asset_id) = CurrencyId::to_native_id(token.denom.base_denom().as_str()) {
				Some(asset_id.into())
			} else {
				let asset_id: CurrencyId =
					token.denom.base_denom().as_str().parse::<u128>().ok()?.into();
				Some(asset_id.into())
			}
		} else {
			IbcDenoms::<T>::get(full_denom.as_bytes().to_vec())
		}
	}
}

impl<T: Config + Send + Sync> IbcHandler for Pallet<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
	<T as DeFiComposableConfig>::MayBeAssetId:
		From<<T::AssetRegistry as RemoteAssetRegistryMutate>::AssetId>,
	<T as DeFiComposableConfig>::MayBeAssetId:
		From<<T::AssetRegistry as RemoteAssetRegistryInspect>::AssetId>,
	<T::AssetRegistry as RemoteAssetRegistryInspect>::AssetId:
		From<<T as DeFiComposableConfig>::MayBeAssetId>,
	<T::AssetRegistry as RemoteAssetRegistryMutate>::AssetId:
		From<<T as DeFiComposableConfig>::MayBeAssetId>,
	<T::AssetRegistry as RemoteAssetRegistryInspect>::AssetNativeLocation: From<XcmAssetLocation>,
	<T::AssetRegistry as RemoteAssetRegistryMutate>::AssetNativeLocation: From<XcmAssetLocation>,
	<T as DeFiComposableConfig>::MayBeAssetId: From<<T as assets::Config>::AssetId>,
	<T as DeFiComposableConfig>::MayBeAssetId: From<primitives::currency::CurrencyId>,
{
	fn latest_height_and_timestamp(
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<(Height, Timestamp), IbcHandlerError> {
		let ctx = Context::<T>::new();
		let source_channel_end =
			ctx.channel_end(&(port_id.clone(), *channel_id)).map_err(|_| {
				IbcHandlerError::ChannelOrPortError {
					msg: Some(format!(
						"Failed to fetch Channel end for channel {} from storage",
						channel_id
					)),
				}
			})?;
		let client_id = Self::channel_client_id(&source_channel_end).map_err(|_| {
			IbcHandlerError::ClientIdError {
				msg: Some(format!("Could not find client id for {:?}/{:?}", port_id, channel_id)),
			}
		})?;

		let client_state =
			ctx.client_state(&client_id).map_err(|_| IbcHandlerError::ClientStateError {
				msg: Some(format!("CLient state not found for {:?}", client_id)),
			})?;
		let consensus_state = ctx
			.consensus_state(&client_id, client_state.latest_height())
			.map_err(|_| IbcHandlerError::Other {
				msg: Some(format!(
					"Consensus state not found for {:?} at {:?}",
					client_id,
					client_state.latest_height()
				)),
			})?;
		Ok((client_state.latest_height(), consensus_state.timestamp()))
	}

	fn send_packet(data: SendPacketData) -> Result<(), IbcHandlerError> {
		let source_channel = data.channel_id;
		let source_port = data.port_id;
		let mut ctx = Context::<T>::new();
		let source_channel_end =
			ctx.channel_end(&(source_port.clone(), source_channel)).map_err(|_| {
				IbcHandlerError::ChannelOrPortError {
					msg: Some(format!(
						"Failed to fetch Channel end for channel {} from storage",
						source_channel
					)),
				}
			})?;
		let client_id = Self::channel_client_id(&source_channel_end).map_err(|_| {
			IbcHandlerError::ClientIdError {
				msg: Some(format!(
					"Could not find client id for {:?}/{:?}",
					source_port, source_channel
				)),
			}
		})?;

		let client_state =
			ctx.client_state(&client_id).map_err(|_| IbcHandlerError::ClientStateError {
				msg: Some(format!("CLient state not found for {:?}", client_id)),
			})?;
		let consensus_state = ctx
			.consensus_state(&client_id, client_state.latest_height())
			.map_err(|_| IbcHandlerError::Other {
				msg: Some(format!(
					"Consensus state not found for {:?} at {:?}",
					client_id,
					client_state.latest_height()
				)),
			})?;
		let sequence =
			ctx.get_next_sequence_send(&(source_port.clone(), source_channel))
				.map_err(|_| IbcHandlerError::SendPacketError {
					msg: Some(format!("Failed to get next_sequence_send for {}", source_channel)),
				})?;

		let destination_port = source_channel_end.counterparty().port_id().clone();
		let destination_channel = *source_channel_end.counterparty().channel_id().ok_or(
			IbcHandlerError::ChannelOrPortError {
				msg: Some(
					"Failed to find counterparty channel_id in source channel end".to_string(),
				),
			},
		)?;
		let timestamp = (consensus_state.timestamp() +
			Duration::from_nanos(data.timeout_timestamp_offset))
		.map_err(|_| IbcHandlerError::TimestampOrHeightError {
			msg: Some("Failed to convert timeout timestamp".to_string()),
		})?;
		let packet = Packet {
			sequence,
			source_port,
			source_channel,
			destination_port,
			destination_channel,
			data: data.data,
			timeout_height: client_state.latest_height().add(data.timeout_height_offset),
			timeout_timestamp: timestamp,
		};

		let send_packet_result =
			ibc::core::ics04_channel::handler::send_packet::send_packet(&ctx, packet)
				.map_err(|e| IbcHandlerError::SendPacketError { msg: Some(e.to_string()) })?;
		ctx.store_packet_result(send_packet_result.result)
			.map_err(|e| IbcHandlerError::SendPacketError { msg: Some(e.to_string()) })?;
		Self::deposit_event(send_packet_result.events.into());
		Ok(())
	}

	fn open_channel(
		port_id: PortId,
		channel_end: ChannelEnd,
	) -> Result<ChannelId, IbcHandlerError> {
		let mut ctx = crate::routing::Context::<T>::new();
		let channel_counter = ctx
			.channel_counter()
			.map_err(|e| IbcHandlerError::ChannelInitError { msg: Some(e.to_string()) })?;
		let channel_id = ChannelId::new(channel_counter);
		// Signer does not matter in this case
		let value = MsgChannelOpenInit {
			port_id,
			channel: channel_end,
			signer: Signer::from_str(MODULE_ID)
				.map_err(|_| IbcHandlerError::ChannelInitError { msg: None })?,
		}
		.encode_vec();
		let msg = ibc_proto::google::protobuf::Any {
			type_url: CHANNEL_OPEN_INIT_TYPE_URL.to_string(),
			value,
		};
		let res = ibc::core::ics26_routing::handler::deliver::<
			_,
			crate::host_functions::HostFunctions<T>,
		>(&mut ctx, msg)
		.map_err(|e| IbcHandlerError::ChannelInitError { msg: Some(e.to_string()) })?;
		Self::deposit_event(res.events.into());
		Ok(channel_id)
	}

	fn write_acknowlegdement(packet: &Packet, ack: Vec<u8>) -> Result<(), IbcHandlerError> {
		let mut ctx = Context::<T>::default();
		Self::store_raw_acknowledgement(
			(packet.source_port.clone(), packet.source_channel, packet.sequence),
			ack.clone(),
		)
		.map_err(|e| IbcHandlerError::AcknowledgementError {
			msg: Some(format!("Failed to store acknowledgement off chain {:?}", e)),
		})?;
		let ack = ctx.ack_commitment(ack.into());
		ctx.store_packet_acknowledgement(
			(packet.source_port.clone(), packet.source_channel, packet.sequence),
			ack,
		)
		.map_err(|e| IbcHandlerError::WriteAcknowledgementError { msg: Some(e.to_string()) })?;
		let host_height = ctx.host_height();
		let event = IbcEvent::WriteAcknowledgement {
			revision_height: host_height.revision_height,
			revision_number: host_height.revision_number,
			packet: packet.clone().into(),
		};
		Self::deposit_event(Event::<T>::Events { events: vec![event] });
		Ok(())
	}

	fn send_transfer(
		msg: ibc::applications::transfer::msgs::transfer::MsgTransfer<
			ibc::applications::transfer::PrefixedCoin,
		>,
	) -> Result<(), IbcHandlerError> {
		let mut handler_output = HandlerOutputBuilder::default();
		let mut ctx = Context::<T>::default();
		send_transfer::<_, _>(&mut ctx, &mut handler_output, msg)
			.map_err(|e| IbcHandlerError::SendTransferError { msg: Some(e.to_string()) })?;
		let result = handler_output.with_result(());
		Self::deposit_event(result.events.into());
		Ok(())
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn create_client() -> Result<ClientId, IbcHandlerError> {
		use crate::benchmarks::tendermint_benchmark_utils::create_mock_state;
		use ibc::core::ics02_client::{
			client_consensus::AnyConsensusState,
			client_state::ClientState,
			msgs::create_client::{MsgCreateAnyClient, TYPE_URL},
		};

		let (mock_client_state, mock_cs_state) = create_mock_state();
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let msg = MsgCreateAnyClient::new(
			AnyClientState::Tendermint(mock_client_state),
			AnyConsensusState::Tendermint(mock_cs_state),
			Signer::from_str("pallet_ibc").unwrap(),
		)
		.unwrap()
		.encode_vec();
		let msg = ibc_proto::google::protobuf::Any { type_url: TYPE_URL.to_string(), value: msg };
		let mut ctx = crate::routing::Context::<T>::new();
		ibc::core::ics26_routing::handler::deliver::<_, crate::host_functions::HostFunctions<T>>(
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
