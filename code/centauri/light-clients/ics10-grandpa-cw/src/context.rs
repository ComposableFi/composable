use std::{fmt, fmt::Debug, marker::PhantomData, rc::Rc};

use cosmwasm_std::{DepsMut, Storage};
use cosmwasm_storage::Bucket;
use ibc::{
	core::{
		ics02_client::{
			client_def::ClientDef,
			client_state::ClientType,
			context::{ClientKeeper, ClientTypes},
		},
		ics24_host::identifier::ClientId,
		ics26_routing::context::ReaderContext,
	},
	timestamp::Timestamp,
	Height,
};
use ics10_grandpa::{
	client_def::GrandpaClient, client_message::ClientMessage, client_state::ClientState,
	consensus_state::ConsensusState,
};

pub struct Context<'a, H> {
	deps: DepsMut<'a>,
	_phantom: PhantomData<H>,
}

impl<'a, H> PartialEq for Context<'a, H> {
	fn eq(&self, _other: &Self) -> bool {
		true
	}
}

impl<'a, H> Eq for Context<'a, H> {}

impl<'a, H> Debug for Context<'a, H> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Context {{ deps: DepsMut }}")
	}
}

impl<'a, H> Clone for Context<'a, H> {
	fn clone(&self) -> Self {
		panic!("Context is not cloneable")
	}
}

impl<'a, H> Context<'a, H> {
	pub fn new(deps: DepsMut<'a>) -> Self {
		Self { deps, _phantom: Default::default() }
	}

	pub fn storage(&self) -> &dyn Storage {
		self.deps.storage
	}

	pub fn storage_mut(&mut self) -> &mut dyn Storage {
		self.deps.storage
	}
}
