use super::*;

use crate::{impls::host_height, routing::Context};
use frame_support::traits::Get;
use ibc::{
	core::{
		ics02_client::{
			client_consensus::AnyConsensusState, client_state::AnyClientState,
			context::ClientReader,
		},
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

impl<T: Config + Sync + Send> ConnectionReader for Context<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
	Self: ClientReader,
{
	fn connection_end(&self, conn_id: &ConnectionId) -> Result<ConnectionEnd, ICS03Error> {
		log::trace!("in connection : [connection_end] >> connection_id = {:?}", conn_id);

		let data = <Connections<T>>::get(conn_id.as_bytes());
		let ret = ConnectionEnd::decode_vec(&*data)
			.map_err(|_| ICS03Error::connection_mismatch(conn_id.clone()))?;
		log::trace!("in connection : [connection_end] >>  connection_end = {:?}", ret);
		Ok(ret)
	}

	fn client_state(&self, client_id: &ClientId) -> Result<AnyClientState, ICS03Error> {
		ClientReader::client_state(self, client_id)
			.map_err(|_| ICS03Error::implementation_specific())
	}

	fn host_current_height(&self) -> Height {
		let current_height = host_height::<T>();
		log::trace!(
			"in connection : [host_current_height] >> Host current height = {:?}",
			Height::new(0, current_height)
		);
		let para_id: u32 = parachain_info::Pallet::<T>::get().into();
		Height::new(para_id.into(), current_height)
	}

	fn host_oldest_height(&self) -> Height {
		let mut temp = frame_system::BlockHash::<T>::iter().collect::<Vec<_>>();
		temp.sort_by(|(a, ..), (b, ..)| a.cmp(b));
		let (block_number, ..) = temp.get(0).cloned().unwrap_or_default();
		let block_number = format!("{:?}", block_number);
		let height = block_number.parse().unwrap_or_default();
		let para_id: u32 = parachain_info::Pallet::<T>::get().into();
		log::trace!(
			"in connection : [host_oldest_height] >> Host oldest height = {:?}",
			Height::new(para_id.into(), height)
		);
		Height::new(para_id.into(), height)
	}

	fn connection_counter(&self) -> Result<u64, ICS03Error> {
		let count = Connections::<T>::count();
		log::trace!("in connection : [connection_counter] >> Connection_counter = {:?}", count);

		Ok(count as u64)
	}

	fn commitment_prefix(&self) -> CommitmentPrefix {
		log::trace!("in connection : [commitment_prefix] >> CommitmentPrefix = {:?}", "ibc");
		T::CONNECTION_PREFIX.to_vec().try_into().unwrap()
	}

	fn client_consensus_state(
		&self,
		client_id: &ClientId,
		height: Height,
	) -> Result<AnyConsensusState, ICS03Error> {
		ClientReader::consensus_state(self, client_id, height)
			.map_err(|_| ICS03Error::missing_consensus_height())
	}

	// TODO: Define consensus state for substrate chains in ibc-rs and modify this after
	#[cfg(not(any(test, feature = "runtime-benchmarks")))]
	fn host_consensus_state(&self, _height: Height) -> Result<AnyConsensusState, ICS03Error> {
		Err(ICS03Error::implementation_specific())
	}

	#[cfg(any(test, feature = "runtime-benchmarks"))]
	fn host_consensus_state(&self, _height: Height) -> Result<AnyConsensusState, ICS03Error> {
		use crate::benchmarks::tendermint_benchmark_utils::create_mock_state;
		let (.., cs_state) = create_mock_state();
		Ok(AnyConsensusState::Tendermint(cs_state))
	}
}

impl<T: Config + Sync + Send> ConnectionKeeper for Context<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	fn store_connection(
		&mut self,
		connection_id: ConnectionId,
		connection_end: &ConnectionEnd,
	) -> Result<(), ICS03Error> {
		log::trace!(
			"in connection : [store_connection] >> connection_id: {:?}, connection_end: {:?}",
			connection_id,
			connection_end
		);

		let data =
			connection_end.encode_vec().map_err(|_| ICS03Error::implementation_specific())?;
		<Connections<T>>::insert(connection_id.as_bytes().to_vec(), data);

		let temp = ConnectionReader::connection_end(self, &connection_id);
		log::trace!("in connection : [store_connection] >> read store after: {:?}", temp);
		Ok(())
	}

	fn store_connection_to_client(
		&mut self,
		connection_id: ConnectionId,
		client_id: &ClientId,
	) -> Result<(), ICS03Error> {
		log::trace!(
			"in connection : [store_connection_to_client] >> connection_id = {:?},\
		 client_id = {:?}",
			connection_id,
			client_id
		);

		ConnectionClient::<T>::try_mutate::<_, _, ICS03Error, _>(
			client_id.as_bytes().to_vec(),
			|val| {
				val.push(connection_id.as_bytes().to_vec());
				Ok(())
			},
		)
	}

	fn increase_connection_counter(&mut self) {
		log::trace!("in connection : [increase_connection_counter]");
		// connections uses a counted storage map
	}
}
