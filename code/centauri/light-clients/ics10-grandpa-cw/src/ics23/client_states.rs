use crate::{ics23::clients::Clients, STORAGE_PREFIX};
use cosmwasm_std::Storage;
use cosmwasm_storage::{prefixed, PrefixedStorage, ReadonlyPrefixedStorage};
use ibc::core::ics24_host::{identifier::ClientId, path::ClientStatePath};

/// client_id => client_states
/// trie key path: "clients/{client_id}/clientState"
pub struct ClientStates<'a>(PrefixedStorage<'a>);

impl ClientStates<'_> {
	pub fn new<'a>(storage: &'a mut dyn Storage) -> Self {
		ClientStates(prefixed(storage, STORAGE_PREFIX))
	}

	pub fn key(client_id: ClientId) -> Vec<u8> {
		let client_state_path = format!("{}", ClientStatePath(client_id));
		client_state_path.into_bytes()
	}

	pub fn get(&self, client_id: &ClientId) -> Option<Vec<u8>> {
		self.0.get(&Self::key(client_id.clone()))
	}

	pub fn insert(&mut self, client_id: ClientId, client_state: Vec<u8>) {
		self.0.set(&Self::key(client_id), &client_state);
	}

	pub fn contains_key(&self, client_id: &ClientId) -> bool {
		self.get(client_id).is_some()
	}
}

pub struct ReadonlyClientStates<'a>(ReadonlyPrefixedStorage<'a>);

impl ReadonlyClientStates<'_> {
	pub fn new<'a>(storage: &'a dyn Storage) -> Self {
		ReadonlyClientStates(ReadonlyPrefixedStorage::new(storage, STORAGE_PREFIX))
	}

	pub fn get(&self, client_id: &ClientId) -> Option<Vec<u8>> {
		self.0.get(&ClientStates::key(client_id.clone()))
	}

	pub fn contains_key(&self, client_id: &ClientId) -> bool {
		self.get(client_id).is_some()
	}
}
