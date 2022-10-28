use crate::STORAGE_PREFIX;
use cosmwasm_std::Storage;
use cosmwasm_storage::{prefixed, PrefixedStorage};
use ibc::{
	core::{
		ics03_connection::connection::ConnectionEnd,
		ics24_host::{identifier::ConnectionId, path::ConnectionsPath},
	},
	protobuf::Protobuf,
};

/// connection_id => ConnectionEnd
/// trie key path: "connections/{}"
pub struct Connections<'a>(PrefixedStorage<'a>);

impl<'a> Connections<'a> {
	pub fn new(storage: &'a mut dyn Storage) -> Self {
		Connections(prefixed(storage, STORAGE_PREFIX))
	}

	pub fn key(connection_id: ConnectionId) -> Vec<u8> {
		let connection_path = format!("{}", ConnectionsPath(connection_id));
		connection_path.into_bytes()
	}

	pub fn get(&self, connection_id: &ConnectionId) -> Option<Vec<u8>> {
		self.0.get(&Self::key(connection_id.clone()))
	}

	pub fn insert(&mut self, connection_id: ConnectionId, connection_end: &ConnectionEnd) {
		self.0.set(&Self::key(connection_id), &connection_end.encode_vec());
	}
}
