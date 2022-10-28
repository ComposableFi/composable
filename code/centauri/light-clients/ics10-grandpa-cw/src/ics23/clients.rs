use crate::STORAGE_PREFIX;
use cosmwasm_std::Storage;
use cosmwasm_storage::{prefixed, prefixed_read, PrefixedStorage, ReadonlyPrefixedStorage};
use ibc::core::ics24_host::{identifier::ClientId, path::ClientTypePath};
use sp_std::prelude::*;
use std::ops::Add;

/// client_id => client_type
/// trie key path: "clients/{}/clientType"
pub struct Clients<'a>(PrefixedStorage<'a>);

impl Clients<'_> {
	pub fn new<'a>(storage: &'a mut dyn Storage) -> Self {
		Clients(prefixed(storage, STORAGE_PREFIX))
	}

	pub fn key(client_id: ClientId) -> Vec<u8> {
		let client_type_path = format!("{}", ClientTypePath(client_id));
		client_type_path.into_bytes()
	}

	pub fn get(&self, client_id: &ClientId) -> Option<Vec<u8>> {
		self.0.get(&Self::key(client_id.clone()))
	}

	pub fn insert(&mut self, client_id: ClientId, client_type: Vec<u8>) {
		self.0.set(&Self::key(client_id), &client_type);
	}

	pub fn contains_key(&self, client_id: &ClientId) -> bool {
		self.get(client_id).is_some()
	}
}

pub struct ReadonlyClients<'a>(ReadonlyPrefixedStorage<'a>);

impl ReadonlyClients<'_> {
	pub fn new<'a>(storage: &'a dyn Storage) -> Self {
		ReadonlyClients(prefixed_read(storage, STORAGE_PREFIX))
	}

	pub fn get(&self, client_id: &ClientId) -> Option<Vec<u8>> {
		self.0.get(&Clients::key(client_id.clone()))
	}

	pub fn contains_key(&self, client_id: &ClientId) -> bool {
		self.get(client_id).is_some()
	}
}
