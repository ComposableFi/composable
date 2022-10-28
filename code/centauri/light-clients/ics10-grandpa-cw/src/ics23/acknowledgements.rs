use crate::{Bytes, STORAGE_PREFIX};
use cosmwasm_std::Storage;
use cosmwasm_storage::{prefixed, prefixed_read, PrefixedStorage, ReadonlyPrefixedStorage};
use ibc::core::{
	ics04_channel::{commitment::AcknowledgementCommitment, packet::Sequence},
	ics24_host::{
		identifier::{ChannelId, PortId},
		path::CommitmentsPath,
	},
};

/// (port_id, channel_id, sequence) => hash
/// trie key path: "acks/ports/{port_id}/channels/{channel_id}/sequences/{sequence}"
pub struct Acknowledgements<'a>(PrefixedStorage<'a>);

impl Acknowledgements<'_> {
	pub fn new(storage: &mut dyn Storage) -> Self {
		Acknowledgements(prefixed(storage, STORAGE_PREFIX))
	}

	pub fn key((port_id, channel_id, sequence): (PortId, ChannelId, Sequence)) -> Vec<u8> {
		let ack_path = CommitmentsPath { port_id, channel_id, sequence };
		let ack_path = format!("{}", ack_path);
		ack_path.into_bytes()
	}

	pub fn get(&self, key: (PortId, ChannelId, Sequence)) -> Option<Bytes> {
		let ack_path = Self::key(key);
		self.0.get(&ack_path)
	}

	pub fn insert(&mut self, key: (PortId, ChannelId, Sequence), ack: AcknowledgementCommitment) {
		let ack_path = Self::key(key);
		self.0.set(&ack_path, &ack.into_vec());
	}

	pub fn remove(&mut self, key: (PortId, ChannelId, Sequence)) {
		let ack_path = Self::key(key);
		self.0.remove(&ack_path);
	}

	pub fn contains_key(&self, key: (PortId, ChannelId, Sequence)) -> bool {
		self.get(key).is_some()
	}
}

pub struct ReadonlyAcknowledgements<'a>(ReadonlyPrefixedStorage<'a>);

impl ReadonlyAcknowledgements<'_> {
	pub fn new<'a>(storage: &'a dyn Storage) -> Self {
		ReadonlyAcknowledgements(prefixed_read(storage, STORAGE_PREFIX))
	}

	pub fn get(&self, key: (PortId, ChannelId, Sequence)) -> Option<Bytes> {
		let ack_path = Acknowledgements::key(key);
		self.0.get(&ack_path)
	}

	pub fn contains_key(&self, key: (PortId, ChannelId, Sequence)) -> bool {
		self.get(key).is_some()
	}
}
