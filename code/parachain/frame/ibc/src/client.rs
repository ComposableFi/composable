use super::*;
use core::str::FromStr;

use crate::{
	ics23::{client_states::ClientStates, clients::Clients, consensus_states::ConsensusStates},
	impls::host_height,
	routing::Context,
};
use frame_support::traits::Get;
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

impl<T: Config + Send + Sync> ClientReader for Context<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	fn client_type(&self, client_id: &ClientId) -> Result<ClientType, ICS02Error> {
		log::trace!(target: "pallet_ibc", "in client : [client_type] >> client_id = {:?}", client_id);

		if <Clients<T>>::contains_key(client_id) {
			let data = <Clients<T>>::get(client_id)
				.ok_or_else(|| ICS02Error::client_not_found(client_id.clone()))?;
			let data = String::from_utf8(data).map_err(|e| {
				ICS02Error::implementation_specific(format!(
					"[client_type]: error decoding client type bytes to string {}",
					e
				))
			})?;
			match ClientType::from_str(&data) {
				Err(_err) => Err(ICS02Error::unknown_client_type(data.to_string())),
				Ok(val) => {
					log::trace!(target: "pallet_ibc", "in client : [client_type] >> client_type : {:?}", val);
					Ok(val)
				},
			}
		} else {
			log::trace!(target: "pallet_ibc", "in client : [client_type] >> read client_type is None");
			Err(ICS02Error::client_not_found(client_id.clone()))
		}
	}

	fn client_state(&self, client_id: &ClientId) -> Result<AnyClientState, ICS02Error> {
		log::trace!(target: "pallet_ibc", "in client : [client_state] >> client_id = {:?}", client_id);
		let data = <ClientStates<T>>::get(client_id)
			.ok_or_else(|| ICS02Error::client_not_found(client_id.clone()))?;
		let state = AnyClientState::decode_vec(&data)
			.map_err(|_| ICS02Error::client_not_found(client_id.clone()))?;
		log::trace!(target: "pallet_ibc", "in client : [client_state] >> any client_state: {:?}", state);
		Ok(state)
	}

	fn consensus_state(
		&self,
		client_id: &ClientId,
		height: Height,
	) -> Result<AnyConsensusState, ICS02Error> {
		log::trace!(target: "pallet_ibc",
			"in client : [consensus_state] >> client_id = {:?}, height = {:?}",
			client_id,
			height
		);

		let native_height = height;
		let value = <ConsensusStates<T>>::get(client_id.clone(), height)
			.ok_or_else(|| ICS02Error::consensus_state_not_found(client_id.clone(), height))?;

		let any_consensus_state = AnyConsensusState::decode_vec(&value)
			.map_err(|_| ICS02Error::consensus_state_not_found(client_id.clone(), native_height))?;
		log::trace!(target: "pallet_ibc",
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
		let mut cs_states = ConsensusStates::<T>::iter_key_prefix(client_id)
			.map(|(height, cs_state)| {
				let cs = AnyConsensusState::decode_vec(&cs_state).map_err(|e| {
					ICS02Error::implementation_specific(format!(
						"[next_consensus_state]: error decoding consensus state from bytes {}",
						e
					))
				})?;
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
		let mut cs_states = ConsensusStates::<T>::iter_key_prefix(client_id)
			.map(|(height, cs_state)| {
				let cs = AnyConsensusState::decode_vec(&cs_state).map_err(|e| {
					ICS02Error::implementation_specific(format!(
						"[next_consensus_state]: error decoding consensus state from bytes {}",
						e
					))
				})?;
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
	#[allow(clippy::disallowed_methods)]
	fn host_timestamp(&self) -> Timestamp {
		use frame_support::traits::UnixTime;
		use sp_runtime::traits::SaturatedConversion;
		let time = T::TimeProvider::now();
		let ts = Timestamp::from_nanoseconds(time.as_nanos().saturated_into::<u64>())
			.map_err(|e| panic!("{:?}, caused by {:?} from pallet timestamp_pallet", e, time));
		// Should not panic, if timestamp is invalid after the genesis block then there's a major
		// error in pallet timestamp
		ts.unwrap()
	}

	fn host_height(&self) -> Height {
		log::trace!(target: "pallet_ibc", "in client: [host_height]");
		let current_height = host_height::<T>();
		let para_id: u32 = parachain_info::Pallet::<T>::get().into();
		Height::new(para_id.into(), current_height)
	}

	fn host_consensus_state(&self, height: Height) -> Result<AnyConsensusState, ICS02Error> {
		let bounded_map = HostConsensusStates::<T>::get();
		let local_state = bounded_map.get(&height.revision_height).ok_or_else(|| {
			ICS02Error::implementation_specific(format!(
				"[host_consensus_state]: consensus state not found for host at height {}",
				height
			))
		})?;
		let timestamp = Timestamp::from_nanoseconds(local_state.timestamp)
			.map_err(|e| {
				ICS02Error::implementation_specific(format!(
					"[host_consensus_state]: error decoding timestamp{}",
					e
				))
			})?
			.into_tm_time()
			.ok_or_else(|| {
				ICS02Error::implementation_specific(
					"[host_consensus_state]: Could not convert timestamp into tendermint time"
						.to_string(),
				)
			})?;
		let consensus_state = ibc::clients::ics11_beefy::consensus_state::ConsensusState {
			timestamp,
			root: local_state.commitment_root.clone().into(),
		};

		Ok(AnyConsensusState::Beefy(consensus_state))
	}

	fn client_counter(&self) -> Result<u64, ICS02Error> {
		let count = ClientCounter::<T>::get();
		log::trace!(target: "pallet_ibc", "in client : [client_counter] >> client_counter: {:?}", count);

		Ok(count as u64)
	}
}

impl<T: Config + Send + Sync> ClientKeeper for Context<T> {
	fn store_client_type(
		&mut self,
		client_id: ClientId,
		client_type: ClientType,
	) -> Result<(), ICS02Error> {
		log::trace!(target: "pallet_ibc",
			"in client : [store_client_type] >> client id = {:?}, client_type = {:?}",
			client_id,
			client_type
		);

		let client_type = client_type.as_str().as_bytes().to_vec();
		<Clients<T>>::insert(&client_id, client_type);
		Ok(())
	}

	fn increase_client_counter(&mut self) {
		log::trace!(target: "pallet_ibc", "in client : [increase_client_counter]");
		// increment counter
		if let Some(val) = <ClientCounter<T>>::get().checked_add(1) {
			<ClientCounter<T>>::put(val);
		}
	}

	fn store_client_state(
		&mut self,
		client_id: ClientId,
		client_state: AnyClientState,
	) -> Result<(), ICS02Error> {
		log::trace!(target: "pallet_ibc",
			"in client : [store_client_state] >> client_id: {:?}, client_state = {:?}",
			client_id,
			client_state
		);

		let data = client_state.encode_vec();
		// store client states key-value
		<ClientStates<T>>::insert(&client_id, data);

		Ok(())
	}

	fn store_consensus_state(
		&mut self,
		client_id: ClientId,
		height: Height,
		consensus_state: AnyConsensusState,
	) -> Result<(), ICS02Error> {
		log::trace!(target: "pallet_ibc", "in client : [store_consensus_state] >> client_id: {:?}, height = {:?}, consensus_state = {:?}",
			client_id, height, consensus_state);

		let data = consensus_state.encode_vec();
		// todo: pruning
		ConsensusStates::<T>::insert(client_id, height, data);
		Ok(())
	}

	fn store_update_time(
		&mut self,
		client_id: ClientId,
		height: Height,
		timestamp: Timestamp,
	) -> Result<(), ICS02Error> {
		let height = height.encode_vec();
		let timestamp = timestamp.nanoseconds();
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
		let height = height.encode_vec();
		let host_height = host_height.encode_vec();
		let client_id = client_id.as_bytes().to_vec();
		ClientUpdateHeight::<T>::insert(client_id, height, host_height);
		Ok(())
	}
}
