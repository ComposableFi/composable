use super::*;

use crate::routing::Context;
use ibc::{
	core::{
		ics02_client::{client_consensus::AnyConsensusState, client_state::AnyClientState},
		ics03_connection::{
			connection::ConnectionEnd,
			context::{ConnectionKeeper, ConnectionReader},
			error::Error as ICS03Error,
		},
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ClientId, ConnectionId},
	},
	Height,
};
use tendermint_proto::Protobuf;

impl<T: Config> ConnectionReader for Context<T> {
	fn connection_end(&self, conn_id: &ConnectionId) -> Result<ConnectionEnd, ICS03Error> {
		log::info!("in connection : [connection_end]");
		log::info!("in connection : [connection_end] >> connection_id = {:?}", conn_id);

		if <Connections<T>>::contains_key(conn_id.as_bytes()) {
			let data = <Connections<T>>::get(conn_id.as_bytes());
			let ret = ConnectionEnd::decode_vec(&*data)
				.map_err(|_| ICS03Error::implementation_specific())?;
			log::info!("in connection : [connection_end] >>  connection_end = {:?}", ret);
			Ok(ret)
		} else {
			log::info!("in connection : [connection_end] >> read connection end returns None");
			Err(ICS03Error::connection_mismatch(conn_id.clone()))
		}
	}

	fn client_state(&self, client_id: &ClientId) -> Result<AnyClientState, ICS03Error> {
		log::info!("in connection : [client_state]");
		log::info!("in connection : [client_state] >> client_id = {:?}", client_id);

		// ClientReader::client_state(self, client_id)
		if <ClientStates<T>>::contains_key(client_id.as_bytes()) {
			let data = <ClientStates<T>>::get(client_id.as_bytes());
			let state = AnyClientState::decode_vec(&*data)
				.map_err(|_| ICS03Error::implementation_specific())?;
			log::info!("in connection : [client_state] >> client_state: {:?}", state);
			Ok(state)
		} else {
			log::info!("in connection : [client_state] >> read client_state is None");

			Err(ICS03Error::frozen_client(client_id.clone()))
		}
	}

	fn host_current_height(&self) -> Height {
		log::info!("in connection : [host_current_height]");

		let block_number = format!("{:?}", <frame_system::Pallet<T>>::block_number());
		let current_height: u64 = block_number.parse().unwrap_or_default();
		// let current_height = block_number;

		<OldHeight<T>>::put(current_height);

		log::info!(
			"in connection : [host_current_height] >> Host current height = {:?}",
			Height::new(0, current_height)
		);
		Height::new(0, current_height)
	}

	fn host_oldest_height(&self) -> Height {
		log::info!("in connection : [host_oldest_height]");

		let height = <OldHeight<T>>::get();

		log::info!(
			"in connection : [host_oldest_height] >> Host oldest height = {:?}",
			Height::new(0, height)
		);
		Height::new(0, height)
	}

	fn connection_counter(&self) -> Result<u64, ICS03Error> {
		log::info!("in connection : [connection_counter]");
		let count = Connections::<T>::count();
		log::info!("in connection : [connection_counter] >> Connection_counter = {:?}", count);

		Ok(count as u64)
	}

	fn commitment_prefix(&self) -> CommitmentPrefix {
		log::info!("in connection : [commitment_prefix]");
		log::info!("in connection : [commitment_prefix] >> CommitmentPrefix = {:?}", "ibc");
		T::CONNECTION_PREFIX.to_vec().try_into().unwrap()
	}

	fn client_consensus_state(
		&self,
		client_id: &ClientId,
		height: Height,
	) -> Result<AnyConsensusState, ICS03Error> {
		log::info!("in connection : [client_consensus_state]");
		log::info!(
			"in connection : [client_consensus_state] client_id = {:?}, height = {:?}",
			client_id,
			height
		);

		// ClientReader::consensus_state(self, client_id, height)
		let height = height.encode_vec().unwrap();
		let value = <ConsensusStates<T>>::get(client_id.as_bytes());

		for item in value.iter() {
			if item.0 == height {
				let any_consensus_state = AnyConsensusState::decode_vec(&*item.1)
					.map_err(|_| ICS03Error::implementation_specific())?;
				return Ok(any_consensus_state)
			}
		}
		Err(ICS03Error::missing_consensus_height())
	}

	// TODO: Define consenssus state for substrate chains in ibc-rs and modify this after
	fn host_consensus_state(&self, _height: Height) -> Result<AnyConsensusState, ICS03Error> {
		Err(ICS03Error::implementation_specific())
	}
}

impl<T: Config> ConnectionKeeper for Context<T> {
	fn store_connection(
		&mut self,
		connection_id: ConnectionId,
		connection_end: &ConnectionEnd,
	) -> Result<(), ICS03Error> {
		log::info!("in connection : [store_connection]");
		log::info!(
			"in connection : [store_connection] >> connection_id: {:?}, connection_end: {:?}",
			connection_id,
			connection_end
		);

		let data = connection_end.encode_vec().unwrap();
		<Connections<T>>::insert(connection_id.as_bytes().to_vec(), data);

		let temp = ConnectionReader::connection_end(self, &connection_id);
		log::info!("in connection : [store_connection] >> read store after: {:?}", temp);
		Ok(())
	}

	fn store_connection_to_client(
		&mut self,
		connection_id: ConnectionId,
		client_id: &ClientId,
	) -> Result<(), ICS03Error> {
		log::info!("in connection : [store_connection_to_client]");
		log::info!(
			"in connection : [store_connection_to_client] >> connection_id = {:?},\
		 client_id = {:?}",
			connection_id,
			client_id
		);

		<ConnectionClient<T>>::insert(
			client_id.as_bytes().to_vec(),
			connection_id.as_bytes().to_vec(),
		);
		Ok(())
	}

	fn increase_connection_counter(&mut self) {
		log::info!("in connection : [increase_connection_counter]");
		// connections uses a counted storage map
	}
}
