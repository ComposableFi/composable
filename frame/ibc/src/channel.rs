use super::*;
use core::{str::FromStr, time::Duration};
use frame_support::traits::Get;
use ibc_primitives::OffchainPacketType;
use scale_info::prelude::{collections::BTreeMap, string::ToString};

use crate::routing::Context;
use ibc::{
	core::{
		ics04_channel::{
			channel::ChannelEnd,
			commitment::{AcknowledgementCommitment, PacketCommitment as PacketCommitmentType},
			context::{ChannelKeeper, ChannelReader},
			error::Error as ICS04Error,
			packet::{Receipt, Sequence},
		},
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	timestamp::Timestamp,
	Height,
};

use tendermint_proto::Protobuf;

impl<T: Config + Sync + Send> ChannelReader for Context<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	/// Returns the ChannelEnd for the given `port_id` and `chan_id`.
	fn channel_end(&self, port_channel_id: &(PortId, ChannelId)) -> Result<ChannelEnd, ICS04Error> {
		log::trace!(
			"in channel : [channel_end] >> port_id = {:?}, channel_id = {:?}",
			port_channel_id.0,
			port_channel_id.1
		);
		let data = <Channels<T>>::get(
			port_channel_id.0.as_bytes(),
			port_channel_id.1.to_string().as_bytes(),
		);
		let channel_end = ChannelEnd::decode_vec(&*data).map_err(|_| {
			ICS04Error::channel_not_found(port_channel_id.clone().0, port_channel_id.clone().1)
		})?;
		log::trace!("in channel : [channel_end] >> channel_end = {:?}", channel_end);
		Ok(channel_end)
	}

	fn connection_channels(
		&self,
		conn_id: &ConnectionId,
	) -> Result<Vec<(PortId, ChannelId)>, ICS04Error> {
		if <ChannelsConnection<T>>::contains_key(conn_id.as_bytes()) {
			let port_and_channel_id = <ChannelsConnection<T>>::get(conn_id.as_bytes());

			let mut result = vec![];

			for item in port_and_channel_id {
				let port_id = String::from_utf8(item.0).map_err(|e| {
					ICS04Error::implementation_specific(format!(
						"[connection_channels]: error decoding port_id: {}",
						e
					))
				})?;
				let port_id = PortId::from_str(port_id.as_str()).map_err(|e| {
					ICS04Error::implementation_specific(format!(
						"[connection_channels]: invalid port id string: {}",
						e
					))
				})?;

				let channel_id = String::from_utf8(item.1).map_err(|e| {
					ICS04Error::implementation_specific(format!(
						"[connection_channels]: error decoding channel_id: {}",
						e
					))
				})?;
				let channel_id = ChannelId::from_str(channel_id.as_str()).map_err(|e| {
					ICS04Error::implementation_specific(format!(
						"[connection_channels]: error decoding channel_id: {}",
						e
					))
				})?;

				result.push((port_id, channel_id));
			}

			log::trace!(
				"in channel : [connection_channels] >> Vector<(PortId, ChannelId)> =  {:?}",
				result
			);
			Ok(result)
		} else {
			Err(ICS04Error::connection_not_open(conn_id.clone()))
		}
	}

	fn get_next_sequence_send(
		&self,
		port_channel_id: &(PortId, ChannelId),
	) -> Result<Sequence, ICS04Error> {
		let seq = <NextSequenceSend<T>>::get(
			port_channel_id.0.as_bytes(),
			port_channel_id.1.to_string().as_bytes(),
		);
		log::trace!("in channel : [get_next_sequence] >> sequence  = {:?}", seq);
		Ok(Sequence::from(seq))
	}

	fn get_next_sequence_recv(
		&self,
		port_channel_id: &(PortId, ChannelId),
	) -> Result<Sequence, ICS04Error> {
		let seq = <NextSequenceRecv<T>>::get(
			port_channel_id.0.as_bytes(),
			port_channel_id.1.to_string().as_bytes(),
		);
		log::trace!("in channel : [get_next_sequence_recv] >> sequence = {:?}", seq);
		Ok(Sequence::from(seq))
	}

	fn get_next_sequence_ack(
		&self,
		port_channel_id: &(PortId, ChannelId),
	) -> Result<Sequence, ICS04Error> {
		let seq = <NextSequenceAck<T>>::get(
			port_channel_id.0.as_bytes(),
			port_channel_id.1.to_string().as_bytes(),
		);
		log::trace!("in channel : [get_next_sequence_ack] >> sequence = {:?}", seq);
		Ok(Sequence::from(seq))
	}

	fn get_packet_commitment(
		&self,
		key: &(PortId, ChannelId, Sequence),
	) -> Result<PacketCommitmentType, ICS04Error> {
		let seq = u64::from(key.2);

		if <PacketCommitment<T>>::contains_key((
			key.0.as_bytes(),
			key.1.to_string().as_bytes(),
			seq,
		)) {
			let data =
				<PacketCommitment<T>>::get((key.0.as_bytes(), key.1.to_string().as_bytes(), seq));
			log::trace!("in channel : [get_packet_commitment] >> packet_commitment = {:?}", data);
			Ok(data.into())
		} else {
			log::trace!(
				"in channel : [get_packet_commitment] >> read get packet commitment return None"
			);
			Err(ICS04Error::packet_commitment_not_found(key.2))
		}
	}

	fn get_packet_receipt(
		&self,
		key: &(PortId, ChannelId, Sequence),
	) -> Result<Receipt, ICS04Error> {
		let seq = u64::from(key.2);

		if <PacketReceipt<T>>::contains_key((key.0.as_bytes(), key.1.to_string().as_bytes(), seq)) {
			let data =
				<PacketReceipt<T>>::get((key.0.as_bytes(), key.1.to_string().as_bytes(), seq));
			let data = String::from_utf8(data).map_err(|e| {
				ICS04Error::implementation_specific(format!(
					"[get_packet_receipt]: error decoding packet receipt: {}",
					e
				))
			})?;
			let data = match data.as_ref() {
				"Ok" => Receipt::Ok,
				_ => return Err(ICS04Error::packet_receipt_not_found(seq.into())),
			};
			log::trace!("in channel : [get_packet_receipt] >> packet_receipt = {:?}", data);
			Ok(data)
		} else {
			log::trace!("in channel : [get_packet_receipt] >> read get packet receipt not found");
			Err(ICS04Error::packet_receipt_not_found(key.2))
		}
	}

	fn get_packet_acknowledgement(
		&self,
		key: &(PortId, ChannelId, Sequence),
	) -> Result<AcknowledgementCommitment, ICS04Error> {
		let seq = u64::from(key.2);

		if <Acknowledgements<T>>::contains_key((
			key.0.as_bytes(),
			key.1.to_string().as_bytes(),
			seq,
		)) {
			let ack =
				<Acknowledgements<T>>::get((key.0.as_bytes(), key.1.to_string().as_bytes(), seq));
			log::trace!(
				"in channel : [get_packet_acknowledgement] >> packet_acknowledgement = {:?}",
				ack
			);
			Ok(ack.into())
		} else {
			log::trace!(
				"in channel : [get_packet_acknowledgement] >> get acknowledgement not found"
			);
			Err(ICS04Error::packet_acknowledgement_not_found(key.2))
		}
	}

	/// A hashing function for packet commitments
	fn hash(&self, value: Vec<u8>) -> Vec<u8> {
		sp_io::hashing::sha2_256(&*value).to_vec()
	}

	fn client_update_time(
		&self,
		client_id: &ClientId,
		height: Height,
	) -> Result<Timestamp, ICS04Error> {
		let height = height.encode_vec().map_err(|e| {
			ICS04Error::implementation_specific(format!(
				"[client_update_time]: error encoding height: {}",
				e
			))
		})?;
		let client_id = client_id.as_bytes().to_vec();
		let timestamp = ClientUpdateTime::<T>::get(&client_id, &height);

		Timestamp::from_nanoseconds(timestamp).map_err(|e| {
			ICS04Error::implementation_specific(format!(
				"[client_update_time]:  error decoding timestamp from nano seconds: {}",
				e
			))
		})
	}

	fn client_update_height(
		&self,
		client_id: &ClientId,
		height: Height,
	) -> Result<Height, ICS04Error> {
		let height = height.encode_vec().map_err(|e| {
			ICS04Error::implementation_specific(format!(
				"[client_update_height]: error encoding height: {}",
				e
			))
		})?;
		let client_id = client_id.as_bytes().to_vec();
		let host_height = ClientUpdateHeight::<T>::get(&client_id, &height);

		Height::decode_vec(&host_height).map_err(|e| {
			ICS04Error::implementation_specific(format!(
				"[client_update_height]: error decoding height: {}",
				e
			))
		})
	}

	/// Returns a counter on the number of channel ids have been created thus far.
	/// The value of this counter should increase only via method
	/// `ChannelKeeper::increase_channel_counter`.
	fn channel_counter(&self) -> Result<u64, ICS04Error> {
		let count = ChannelCounter::<T>::get();
		log::trace!("in channel: [channel_counter] >> channel_counter = {:?}", count);
		Ok(count.into())
	}

	fn max_expected_time_per_block(&self) -> Duration {
		let expected = T::ExpectedBlockTime::get();
		Duration::from_nanos(expected)
	}
}

impl<T: Config + Sync + Send> ChannelKeeper for Context<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	fn store_packet_commitment(
		&mut self,
		key: (PortId, ChannelId, Sequence),
		commitment: PacketCommitmentType,
	) -> Result<(), ICS04Error> {
		let seq = u64::from(key.2);
		<PacketCommitment<T>>::insert(
			(key.0.as_bytes().to_vec(), key.1.to_string().as_bytes().to_vec(), seq),
			commitment.into_vec(),
		);

		Ok(())
	}

	fn store_packet(
		&mut self,
		key: (PortId, ChannelId, Sequence),
		packet: ibc::core::ics04_channel::packet::Packet,
	) -> Result<(), ICS04Error> {
		// store packet offchain
		let channel_id = key.1.to_string().as_bytes().to_vec();
		let port_id = key.0.as_bytes().to_vec();
		let seq = u64::from(key.2);
		let key = Pallet::<T>::offchain_key(channel_id, port_id);
		let mut offchain_packets: BTreeMap<u64, OffchainPacketType> =
			sp_io::offchain::local_storage_get(sp_core::offchain::StorageKind::PERSISTENT, &key)
				.and_then(|v| codec::Decode::decode(&mut &*v).ok())
				.unwrap_or_default();
		let offchain_packet: OffchainPacketType = packet.into();
		offchain_packets.insert(seq, offchain_packet);
		sp_io::offchain_index::set(&key, offchain_packets.encode().as_slice());
		Ok(())
	}

	fn delete_packet_commitment(
		&mut self,
		key: (PortId, ChannelId, Sequence),
	) -> Result<(), ICS04Error> {
		let seq = u64::from(key.2);

		// delete packet commitment
		<PacketCommitment<T>>::remove((
			key.0.as_bytes().to_vec(),
			key.1.to_string().as_bytes().to_vec(),
			seq,
		));

		Ok(())
	}

	fn store_packet_receipt(
		&mut self,
		key: (PortId, ChannelId, Sequence),
		receipt: Receipt,
	) -> Result<(), ICS04Error> {
		let receipt = match receipt {
			Receipt::Ok => "Ok".as_bytes(),
		};

		let seq = u64::from(key.2);

		<PacketReceipt<T>>::insert(
			(key.0.as_bytes().to_vec(), key.1.to_string().as_bytes().to_vec(), seq),
			receipt,
		);

		Ok(())
	}

	fn store_packet_acknowledgement(
		&mut self,
		key: (PortId, ChannelId, Sequence),
		ack_commitment: AcknowledgementCommitment,
	) -> Result<(), ICS04Error> {
		let seq = u64::from(key.2);

		// store packet acknowledgement key-value
		<Acknowledgements<T>>::insert(
			(key.0.as_bytes().to_vec(), key.1.to_string().as_bytes().to_vec(), seq),
			ack_commitment.into_vec(),
		);

		Ok(())
	}

	fn delete_packet_acknowledgement(
		&mut self,
		key: (PortId, ChannelId, Sequence),
	) -> Result<(), ICS04Error> {
		let seq = u64::from(key.2);

		// remove acknowledgements
		<Acknowledgements<T>>::remove((
			key.0.as_bytes().to_vec(),
			key.1.to_string().as_bytes().to_vec(),
			seq,
		));

		Ok(())
	}

	fn store_connection_channels(
		&mut self,
		conn_id: ConnectionId,
		port_channel_id: &(PortId, ChannelId),
	) -> Result<(), ICS04Error> {
		let conn_id = conn_id.as_bytes().to_vec();

		let port_channel_id = (
			port_channel_id.0.as_bytes().to_vec(),
			port_channel_id.1.to_string().as_bytes().to_vec(),
		);

		if <ChannelsConnection<T>>::contains_key(conn_id.clone()) {
			log::trace!("in channel: [store_connection_channels] >> insert port_channel_id");
			<ChannelsConnection<T>>::try_mutate(conn_id, |val| -> Result<(), &'static str> {
				val.push(port_channel_id);
				Ok(())
			})
			.expect("store connection channels error");
		} else {
			log::trace!("in channel: [store_connection_channels] >> init ChannelsConnection");
			let temp_connection_channels = vec![port_channel_id];
			<ChannelsConnection<T>>::insert(conn_id, temp_connection_channels);
		}

		Ok(())
	}

	/// Stores the given channel_end at a path associated with the port_id and channel_id.
	fn store_channel(
		&mut self,
		port_channel_id: (PortId, ChannelId),
		channel_end: &ChannelEnd,
	) -> Result<(), ICS04Error> {
		let channel_end = channel_end.encode_vec().map_err(|e| {
			ICS04Error::implementation_specific(format!(
				"[store_channel]: error encoding channel end: {}",
				e
			))
		})?;

		// store channels key-value
		<Channels<T>>::insert(
			port_channel_id.0.as_bytes().to_vec(),
			port_channel_id.1.to_string().as_bytes().to_vec(),
			channel_end,
		);

		Ok(())
	}

	fn store_next_sequence_send(
		&mut self,
		port_channel_id: (PortId, ChannelId),
		seq: Sequence,
	) -> Result<(), ICS04Error> {
		let seq = u64::from(seq);

		<NextSequenceSend<T>>::insert(
			port_channel_id.0.as_bytes().to_vec(),
			port_channel_id.1.to_string().as_bytes().to_vec(),
			seq,
		);

		Ok(())
	}

	fn store_next_sequence_recv(
		&mut self,
		port_channel_id: (PortId, ChannelId),
		seq: Sequence,
	) -> Result<(), ICS04Error> {
		let seq = u64::from(seq);

		<NextSequenceRecv<T>>::insert(
			port_channel_id.0.as_bytes().to_vec(),
			port_channel_id.1.to_string().as_bytes().to_vec(),
			seq,
		);

		Ok(())
	}

	fn store_next_sequence_ack(
		&mut self,
		port_channel_id: (PortId, ChannelId),
		seq: Sequence,
	) -> Result<(), ICS04Error> {
		let seq = u64::from(seq);

		<NextSequenceAck<T>>::insert(
			port_channel_id.0.as_bytes().to_vec(),
			port_channel_id.1.to_string().as_bytes().to_vec(),
			seq,
		);

		Ok(())
	}

	/// Called upon channel identifier creation (Init or Try message processing).
	/// Increases the counter which keeps track of how many channels have been created.
	/// Should never fail.
	fn increase_channel_counter(&mut self) {
		log::trace!("in channel: [increase_channel_counter]");
		let _ = ChannelCounter::<T>::try_mutate::<_, (), _>(|val| {
			*val = val.saturating_add(1);
			Ok(())
		});
	}
}
