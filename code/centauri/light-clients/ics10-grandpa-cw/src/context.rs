use cosmwasm_std::{DepsMut, Storage};
use ibc::core::ics02_client::context::{ClientKeeper, ClientTypes};

use crate::types::{AnyClient, AnyClientMessage, AnyClientState, AnyConsensusState};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Context {
	storage: Box<dyn Storage>,
}

impl ClientTypes for Context {
	type AnyClientMessage = AnyClientMessage;
	type AnyClientState = AnyClientState;
	type AnyConsensusState = AnyConsensusState;
	type ClientDef = AnyClient;
}

impl ClientKeeper for Context
where
	Self: Clone + std::fmt::Debug + Eq,
{
	fn store_client_type(
		&mut self,
		client_id: ibc::core::ics24_host::identifier::ClientId,
		client_type: ibc::core::ics02_client::client_state::ClientType,
	) -> Result<(), ibc::core::ics02_client::error::Error> {
		todo!()
	}

	fn store_client_state(
		&mut self,
		client_id: ibc::core::ics24_host::identifier::ClientId,
		client_state: Self::AnyClientState,
	) -> Result<(), ibc::core::ics02_client::error::Error> {
		todo!()
	}

	fn store_consensus_state(
		&mut self,
		client_id: ibc::core::ics24_host::identifier::ClientId,
		height: ibc::Height,
		consensus_state: Self::AnyConsensusState,
	) -> Result<(), ibc::core::ics02_client::error::Error> {
		todo!()
	}

	fn increase_client_counter(&mut self) {
		todo!()
	}

	fn store_update_time(
		&mut self,
		client_id: ibc::core::ics24_host::identifier::ClientId,
		height: ibc::Height,
		timestamp: ibc::timestamp::Timestamp,
	) -> Result<(), ibc::core::ics02_client::error::Error> {
		todo!()
	}

	fn store_update_height(
		&mut self,
		client_id: ibc::core::ics24_host::identifier::ClientId,
		height: ibc::Height,
		host_height: ibc::Height,
	) -> Result<(), ibc::core::ics02_client::error::Error> {
		todo!()
	}

	fn validate_self_client(
		&self,
		client_state: &Self::AnyClientState,
	) -> Result<(), ibc::core::ics02_client::error::Error> {
		let (relay_chain, para_id, latest_para_height) = match client_state {
			AnyClientState::Beefy(client_state) => {
				if client_state.is_frozen() {
					Err(ICS02Error::implementation_specific(format!("client state is frozen")))?
				}

				(client_state.relay_chain, client_state.para_id, client_state.latest_para_height)
			},
			AnyClientState::Grandpa(client_state) => {
				if client_state.is_frozen() {
					Err(ICS02Error::implementation_specific(format!("client state is frozen")))?
				}

				(client_state.relay_chain, client_state.para_id, client_state.latest_para_height)
			},
			client => Err(ICS02Error::unknown_client_type(format!("{}", client.client_type())))?,
		};

		// if relay_chain != T::RelayChain::get() {
		// 	Err(ICS02Error::implementation_specific(format!("relay chain mis-match")))?
		// }

		// let self_para_id: u32 = T::ParaId::get().into();
		// if para_id != self_para_id {
		// 	Err(ICS02Error::implementation_specific(format!("para-id mis-match")))?
		// }

		let block_number: u32 = todo!();
		// <frame_system::Pallet<T>>::block_number().into();

		// this really shouldn't be possible
		if latest_para_height >= block_number {
			Err(ICS02Error::implementation_specific(format!(
				"client has latest_para_height {} greater than or equal to chain height {block_number}",
				latest_para_height
			)))?
		}
	}
}
