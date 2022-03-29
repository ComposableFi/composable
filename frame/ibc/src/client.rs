use super::*;
use core::str::FromStr;

use crate::routing::Context;
use ibc::{
	core::{
		ics02_client::{
			client_consensus::AnyConsensusState,
			client_state::AnyClientState,
			client_type::ClientType,
			context::{ClientKeeper, ClientReader},
			error::Error as ICS02Error,
		},
		ics24_host::identifier::ClientId,
	},
	timestamp::Timestamp,
	Height,
};
use tendermint_proto::Protobuf;

impl<T: Config> ClientReader for Context<T> {
	fn client_type(&self, client_id: &ClientId) -> Result<ClientType, ICS02Error> {
		log::info!("in client : [client_type]");
		log::info!("in client : [client_type] >> client_id = {:?}", client_id);

		if <Clients<T>>::contains_key(client_id.as_bytes()) {
			let data = <Clients<T>>::get(client_id.as_bytes());
			let mut data: &[u8] = &data;
			let data =
				Vec::<u8>::decode(&mut data).map_err(|_| ICS02Error::implementation_specific())?;
			let data =
				String::from_utf8(data).map_err(|_| ICS02Error::implementation_specific())?;
			match ClientType::from_str(&data) {
				Err(_err) => Err(ICS02Error::unknown_client_type(format!("{}", data))),
				Ok(val) => {
					log::info!("in client : [client_type] >> client_type : {:?}", val);
					Ok(val)
				},
			}
		} else {
			log::info!("in client : [client_type] >> read client_type is None");
			Err(ICS02Error::client_not_found(client_id.clone()))
		}
	}

	fn client_state(&self, client_id: &ClientId) -> Result<AnyClientState, ICS02Error> {
		log::info!("in client : [client_state]");
		log::info!("in client : [client_state] >> client_id = {:?}", client_id);

		if <ClientStates<T>>::contains_key(client_id.as_bytes()) {
			let data = <ClientStates<T>>::get(client_id.as_bytes());
			let state = AnyClientState::decode_vec(&*data)
				.map_err(|_| ICS02Error::implementation_specific())?;
			log::info!("in client : [client_state] >> any client_state: {:?}", state);
			Ok(state)
		} else {
			log::info!("in client : [client_state] >> read any client state is None");
			Err(ICS02Error::client_not_found(client_id.clone()))
		}
	}

	fn consensus_state(
		&self,
		client_id: &ClientId,
		height: Height,
	) -> Result<AnyConsensusState, ICS02Error> {
		log::info!("in client : [consensus_state]");
		log::info!(
			"in client : [consensus_state] >> client_id = {:?}, height = {:?}",
			client_id,
			height
		);

		let native_height = height.clone();
		let height = height.encode_vec().map_err(|_| ICS02Error::implementation_specific())?;
		let value = <ConsensusStates<T>>::get(client_id.as_bytes(), height);

		let any_consensus_state = AnyConsensusState::decode_vec(&*value)
			.map_err(|_| ICS02Error::consensus_state_not_found(client_id.clone(), native_height))?;
		log::info!(
			"in client : [consensus_state] >> any consensus state = {:?}",
			any_consensus_state
		);
		Ok(any_consensus_state)
	}

	fn next_consensus_state(
		&self,
		client_id: &ClientId,
		height: Height,
	) -> Result<Option<AnyConsensusState>, ICS02Error> {
		let client_id = client_id.as_bytes().to_vec();
		let mut cs_states = ConsensusStates::<T>::iter_key_prefix(client_id.clone())
			.map(|height| {
				let cs_state = ConsensusStates::<T>::get(client_id.clone(), height.clone());
				let height = Height::decode_vec(&height)
					.map_err(|_| ICS02Error::implementation_specific())?;
				let cs = AnyConsensusState::decode_vec(&cs_state)
					.map_err(|_| ICS02Error::implementation_specific())?;
				Ok((height, cs))
			})
			.collect::<Result<Vec<_>, ICS02Error>>()?;

		cs_states.sort_by(|a, b| a.0.cmp(&b.0));

		for cs in cs_states {
			if cs.0 > height {
				return Ok(Some(cs.1))
			}
		}

		Ok(None)
	}

	fn prev_consensus_state(
		&self,
		client_id: &ClientId,
		height: Height,
	) -> Result<Option<AnyConsensusState>, ICS02Error> {
		let client_id = client_id.as_bytes().to_vec();
		let mut cs_states = ConsensusStates::<T>::iter_key_prefix(client_id.clone())
			.map(|height| {
				let cs_state = ConsensusStates::<T>::get(client_id.clone(), height.clone());
				let height = Height::decode_vec(&height)
					.map_err(|_| ICS02Error::implementation_specific())?;
				let cs = AnyConsensusState::decode_vec(&cs_state)
					.map_err(|_| ICS02Error::implementation_specific())?;
				Ok((height, cs))
			})
			.collect::<Result<Vec<_>, ICS02Error>>()?;

		cs_states.sort_by(|a, b| b.0.cmp(&a.0));

		for cs in cs_states {
			if cs.0 < height {
				return Ok(Some(cs.1))
			}
		}

		Ok(None)
	}

	fn host_height(&self) -> Height {
		log::info!("in client: [host_height]");

		let block_number = format!("{:?}", <frame_system::Pallet<T>>::block_number());
		let current_height = block_number
			.parse()
			.map_err(|e| panic!("{:?}, caused by {:?} from frame_system::Pallet", e, block_number));
		Height::new(0, current_height.unwrap())
	}

	// TODO: Revisit after consensus state for beefy light client is defined in chains is defined in
	// ibc-rs
	fn host_consensus_state(&self, _height: Height) -> Result<AnyConsensusState, ICS02Error> {
		Err(ICS02Error::implementation_specific())
	}

	fn pending_host_consensus_state(&self) -> Result<AnyConsensusState, ICS02Error> {
		Err(ICS02Error::implementation_specific())
	}

	fn client_counter(&self) -> Result<u64, ICS02Error> {
		log::info!("in client : [client_counter]");
		let count = Clients::<T>::count();
		log::info!("in client : [client_counter] >> client_counter: {:?}", count);

		Ok(count as u64)
	}
}

impl<T: Config> ClientKeeper for Context<T> {
	fn store_client_type(
		&mut self,
		client_id: ClientId,
		client_type: ClientType,
	) -> Result<(), ICS02Error> {
		log::info!("in client : [store_client_type]");
		log::info!(
			"in client : [store_client_type] >> client id = {:?}, client_type = {:?}",
			client_id,
			client_type
		);

		let client_id = client_id.as_bytes().to_vec();
		let client_type = client_type.as_str().encode();
		<Clients<T>>::insert(client_id, client_type);
		Ok(())
	}

	fn increase_client_counter(&mut self) {
		log::info!("in client : [increase_client_counter]");
		// Clients uses a counted storage map
	}

	fn store_client_state(
		&mut self,
		client_id: ClientId,
		client_state: AnyClientState,
	) -> Result<(), ICS02Error> {
		log::info!("in client : [store_client_state]");
		log::info!(
			"in client : [store_client_state] >> client_id: {:?}, client_state = {:?}",
			client_id,
			client_state
		);

		let data = client_state.encode_vec().map_err(|_| ICS02Error::implementation_specific())?;
		// store client states key-value
		<ClientStates<T>>::insert(client_id.as_bytes().to_vec(), data);

		Ok(())
	}

	fn store_consensus_state(
		&mut self,
		client_id: ClientId,
		height: Height,
		consensus_state: AnyConsensusState,
	) -> Result<(), ICS02Error> {
		log::info!("in client : [store_consensus_state]");
		log::info!("in client : [store_consensus_state] >> client_id: {:?}, height = {:?}, consensus_state = {:?}",
			client_id, height, consensus_state);

		let height = height.encode_vec().map_err(|_| ICS02Error::implementation_specific())?;
		let data = consensus_state
			.encode_vec()
			.map_err(|_| ICS02Error::implementation_specific())?;
		ConsensusStates::<T>::insert(client_id.as_bytes().to_vec(), height, data);
		Ok(())
	}

	fn store_update_time(
		&mut self,
		client_id: ClientId,
		height: Height,
		timestamp: Timestamp,
	) -> Result<(), ICS02Error> {
		let height = height.encode_vec().map_err(|_| ICS02Error::implementation_specific())?;
		let timestamp = timestamp.nanoseconds().encode();
		let client_id = client_id.as_bytes().to_vec();
		ClientUpdateTime::<T>::insert(client_id, height, timestamp);
		Ok(())
	}

	fn store_update_height(
		&mut self,
		client_id: ClientId,
		height: Height,
		host_height: Height,
	) -> Result<(), ICS02Error> {
		let height = height.encode_vec().map_err(|_| ICS02Error::implementation_specific())?;
		let host_height =
			host_height.encode_vec().map_err(|_| ICS02Error::implementation_specific())?;
		let client_id = client_id.as_bytes().to_vec();
		ClientUpdateHeight::<T>::insert(client_id, height, host_height);
		Ok(())
	}
}
