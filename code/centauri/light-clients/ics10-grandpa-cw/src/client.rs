use crate::context::Context;
use grandpa_light_client_primitives::HostFunctions;
use ibc::{
	core::{
		ics02_client::{
			client_state::ClientType,
			context::{ClientKeeper, ClientTypes},
			error::Error,
		},
		ics24_host::identifier::ClientId,
	},
	timestamp::Timestamp,
	Height,
};
use ics10_grandpa::{
	client_def::GrandpaClient, client_message::ClientMessage, client_state::ClientState,
	consensus_state::ConsensusState,
};

impl<'a, H: HostFunctions> ClientTypes for Context<'a, H> {
	type AnyClientMessage = ClientMessage;
	type AnyClientState = ClientState<H>;
	type AnyConsensusState = ConsensusState;
	type ClientDef = GrandpaClient<H>;
}

impl<'a, H: HostFunctions> ClientKeeper for Context<'a, H> {
	fn store_client_type(
		&mut self,
		client_id: ClientId,
		client_type: ClientType,
	) -> Result<(), Error> {
		todo!()
	}

	fn store_client_state(
		&mut self,
		client_id: ClientId,
		client_state: Self::AnyClientState,
	) -> Result<(), Error> {
		todo!()
	}

	fn store_consensus_state(
		&mut self,
		client_id: ClientId,
		height: Height,
		consensus_state: Self::AnyConsensusState,
	) -> Result<(), Error> {
		todo!()
	}

	fn increase_client_counter(&mut self) {
		todo!()
	}

	fn store_update_time(
		&mut self,
		client_id: ClientId,
		height: Height,
		timestamp: Timestamp,
	) -> Result<(), Error> {
		todo!()
	}

	fn store_update_height(
		&mut self,
		client_id: ClientId,
		height: Height,
		host_height: Height,
	) -> Result<(), Error> {
		todo!()
	}

	fn validate_self_client(&self, client_state: &Self::AnyClientState) -> Result<(), Error> {
		todo!()
	}
}
