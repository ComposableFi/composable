use super::*;
use core::{ops::Deref, str::FromStr, time::Duration};
use frame_support::traits::Get;
use scale_info::prelude::string::ToString;

use crate::routing::Context;
use ibc::{
	core::{
		ics02_client::{
			client_consensus::AnyConsensusState, client_state::AnyClientState,
			context::ClientReader,
		},
		ics03_connection::{connection::ConnectionEnd, context::ConnectionReader},
		ics04_channel::{
			channel::ChannelEnd,
			context::{ChannelKeeper, ChannelReader},
			error::Error as ICS04Error,
			packet::{Receipt, Sequence},
		},
		ics05_port::{
			capabilities::ChannelCapability, context::PortReader, error::Error as Ics05Error,
		},
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
		ics26_routing::context::ModuleId,
	},
	timestamp::Timestamp,
	Height,
};
use tendermint_proto::Protobuf;

impl<T: Config + Sync + Send> ChannelReader for Context<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
	Self: ConnectionReader + ClientReader,
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

	/// Returns the ConnectionState for the given identifier `connection_id`.
	fn connection_end(&self, connection_id: &ConnectionId) -> Result<ConnectionEnd, ICS04Error> {
		if <Connections<T>>::contains_key(connection_id.as_bytes()) {
			let data = <Connections<T>>::get(connection_id.as_bytes());
			let ret = ConnectionEnd::decode_vec(&*data)
				.map_err(|_| ICS04Error::implementation_specific())?;
			log::trace!("In channel : [connection_end] >> connection_end = {:?}", ret);
			Ok(ret)
		} else {
			log::trace!("in channel : [channel_end] >> read connection end returns None");
			Err(ICS04Error::connection_not_open(connection_id.clone()))
		}
	}

	fn connection_channels(
		&self,
		conn_id: &ConnectionId,
	) -> Result<Vec<(PortId, ChannelId)>, ICS04Error> {
		if <ChannelsConnection<T>>::contains_key(conn_id.as_bytes()) {
			let port_and_channel_id = <ChannelsConnection<T>>::get(conn_id.as_bytes());

			let mut result = vec![];

			for item in port_and_channel_id {
				let port_id =
					String::from_utf8(item.0).map_err(|_| ICS04Error::implementation_specific())?;
				let port_id = PortId::from_str(port_id.as_str())
					.map_err(|_| ICS04Error::implementation_specific())?;

				let channel_id =
					String::from_utf8(item.1).map_err(|_| ICS04Error::implementation_specific())?;
				let channel_id = ChannelId::from_str(channel_id.as_str())
					.map_err(|_| ICS04Error::implementation_specific())?;

				result.push((port_id, channel_id));
			}

			log::trace!(
				"in channel : [connection_channels] >> Vector<(PortId, ChannelId)> =  {:?}",
				result
			);
			return Ok(result)
		} else {
			Err(ICS04Error::connection_not_open(conn_id.clone()))
		}
	}

	/// Returns the ClientState for the given identifier `client_id`. Necessary dependency towards
	/// proof verification.
	fn client_state(&self, client_id: &ClientId) -> Result<AnyClientState, ICS04Error> {
		let data = <ClientStates<T>>::get(client_id.as_bytes());
		let state = AnyClientState::decode_vec(&*data)
			.map_err(|_| ICS04Error::implementation_specific())?;
		log::trace!("in channel : [client_state] >> Any client state: {:?}", state);
		Ok(state)
	}

	fn client_consensus_state(
		&self,
		client_id: &ClientId,
		height: Height,
	) -> Result<AnyConsensusState, ICS04Error> {
		let height = height.encode_vec().map_err(|_| ICS04Error::implementation_specific())?;
		let value = <ConsensusStates<T>>::get(client_id.as_bytes(), height);

		let any_consensus_state = AnyConsensusState::decode_vec(&*value)
			.map_err(|_| ICS04Error::implementation_specific())?;
		log::trace!(
			"in channel: [client_consensus_state] >> any consensus state = {:?}",
			any_consensus_state
		);
		Ok(any_consensus_state)
	}

	fn authenticated_capability(&self, port_id: &PortId) -> Result<ChannelCapability, ICS04Error> {
		match PortReader::lookup_module_by_port(self, port_id) {
			Ok((.., key)) =>
				if !PortReader::authenticate(self, port_id.clone(), &key) {
					Err(ICS04Error::invalid_port_capability())
				} else {
					let key = key.deref().clone();
					Ok(key.into())
				},
			Err(e) if e.detail() == Ics05Error::unknown_port(port_id.clone()).detail() =>
				Err(ICS04Error::no_port_capability(port_id.clone())),
			Err(_) => Err(ICS04Error::implementation_specific()),
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
	) -> Result<String, ICS04Error> {
		let seq = u64::from(key.2);

		if <PacketCommitment<T>>::contains_key((
			key.0.as_bytes(),
			key.1.to_string().as_bytes(),
			seq,
		)) {
			let data =
				<PacketCommitment<T>>::get((key.0.as_bytes(), key.1.to_string().as_bytes(), seq));
			let data =
				String::from_utf8(data).map_err(|_| ICS04Error::implementation_specific())?;
			log::trace!("in channel : [get_packet_commitment] >> packet_commitment = {:?}", data);
			Ok(data)
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
			let data =
				String::from_utf8(data).map_err(|_| ICS04Error::implementation_specific())?;
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
	) -> Result<String, ICS04Error> {
		let seq = u64::from(key.2);

		if <Acknowledgements<T>>::contains_key((
			key.0.as_bytes(),
			key.1.to_string().as_bytes(),
			seq,
		)) {
			let data =
				<Acknowledgements<T>>::get((key.0.as_bytes(), key.1.to_string().as_bytes(), seq));
			let ack = String::from_utf8(data).map_err(|_| ICS04Error::implementation_specific())?;
			log::trace!(
				"in channel : [get_packet_acknowledgement] >> packet_acknowledgement = {:?}",
				ack
			);
			Ok(ack)
		} else {
			log::trace!(
				"in channel : [get_packet_acknowledgement] >> get acknowledgement not found"
			);
			Err(ICS04Error::packet_acknowledgement_not_found(key.2))
		}
	}

	/// A hashing function for packet commitments
	fn hash(&self, value: String) -> String {
		let r = sp_io::hashing::sha2_256(value.as_bytes());

		let mut tmp = String::new();
		for item in r.iter() {
			tmp.push_str(&format!("{:02x}", item));
		}
		log::trace!("in channel: [hash] >> result = {:?}", tmp.clone());
		tmp
	}

	/// Returns the current height of the local chain.
	fn host_height(&self) -> Height {
		ConnectionReader::host_current_height(self)
	}

	/// Returns the current timestamp of the local chain.
	fn host_timestamp(&self) -> Timestamp {
		ClientReader::host_timestamp(self)
	}

	fn host_consensus_state(&self, height: Height) -> Result<AnyConsensusState, ICS04Error> {
		ConnectionReader::host_consensus_state(self, height)
			.map_err(|_| ICS04Error::implementation_specific())
	}

	fn pending_host_consensus_state(&self) -> Result<AnyConsensusState, ICS04Error> {
		ClientReader::pending_host_consensus_state(self)
			.map_err(|_| ICS04Error::implementation_specific())
	}

	fn client_update_time(
		&self,
		client_id: &ClientId,
		height: Height,
	) -> Result<Timestamp, ICS04Error> {
		let height = height.encode_vec().map_err(|_| ICS04Error::implementation_specific())?;
		let client_id = client_id.as_bytes().to_vec();
		let timestamp = ClientUpdateTime::<T>::get(&client_id, &height);

		Timestamp::from_nanoseconds(
			u64::decode(&mut &*timestamp).map_err(|_| ICS04Error::implementation_specific())?,
		)
		.map_err(|_| ICS04Error::implementation_specific())
	}

	fn client_update_height(
		&self,
		client_id: &ClientId,
		height: Height,
	) -> Result<Height, ICS04Error> {
		let height = height.encode_vec().map_err(|_| ICS04Error::implementation_specific())?;
		let client_id = client_id.as_bytes().to_vec();
		let host_height = ClientUpdateHeight::<T>::get(&client_id, &height);

		Height::decode_vec(&host_height).map_err(|_| ICS04Error::implementation_specific())
	}

	/// Returns a counter on the number of channel ids have been created thus far.
	/// The value of this counter should increase only via method
	/// `ChannelKeeper::increase_channel_counter`.
	fn channel_counter(&self) -> Result<u64, ICS04Error> {
		let count = ChannelCounter::<T>::get();
		log::trace!("in channel: [channel_counter] >> channel_counter = {:?}", count);
		Ok(count)
	}

	fn max_expected_time_per_block(&self) -> Duration {
		let expected = T::ExpectedBlockTime::get();
		Duration::from_nanos(expected)
	}

	fn lookup_module_by_channel(
		&self,
		_channel_id: &ChannelId,
		port_id: &PortId,
	) -> Result<(ModuleId, ChannelCapability), ICS04Error> {
		let (module_id, cap) =
			self.lookup_module_by_port(port_id).map_err(|_| ICS04Error::route_not_found())?;
		let cap = cap.deref().clone();
		Ok((module_id, ChannelCapability::from(cap)))
	}
}

impl<T: Config + Sync + Send> ChannelKeeper for Context<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	fn store_packet_commitment(
		&mut self,
		key: (PortId, ChannelId, Sequence),
		timestamp: Timestamp,
		heigh: Height,
		data: Vec<u8>,
	) -> Result<(), ICS04Error> {
		let input = format!("{:?},{:?},{:?}", timestamp, heigh, data);
		let seq = u64::from(key.2);

		// insert packet commitment key-value
		<PacketCommitment<T>>::insert(
			(key.0.as_bytes().to_vec(), key.1.to_string().as_bytes().to_vec(), seq),
			ChannelReader::hash(self, input).as_bytes(),
		);

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
		ack: Vec<u8>,
	) -> Result<(), ICS04Error> {
		let ack = format!("{:?}", ack);
		let seq = u64::from(key.2);

		// store packet acknowledgement key-value
		<Acknowledgements<T>>::insert(
			(key.0.as_bytes().to_vec(), key.1.to_string().as_bytes().to_vec(), seq),
			ChannelReader::hash(self, ack).encode(),
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
		let channel_end =
			channel_end.encode_vec().map_err(|_| ICS04Error::implementation_specific())?;

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
