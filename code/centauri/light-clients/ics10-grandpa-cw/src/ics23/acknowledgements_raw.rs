use crate::{Bytes, STORAGE_PREFIX};
use cosmwasm_std::Storage;
use cosmwasm_storage::{prefixed, prefixed_read, PrefixedStorage, ReadonlyPrefixedStorage};
use ibc::core::{
	ics04_channel::packet::Sequence,
	ics24_host::identifier::{ChannelId, PortId},
};

/// (port_id, channel_id, sequence) => Acknowledgement
pub struct AcknowledgementsRaw<'a>(PrefixedStorage<'a>);

impl AcknowledgementsRaw<'_> {
	pub fn new<'a>(storage: &'a mut dyn Storage) -> Self {
		AcknowledgementsRaw(prefixed(storage, STORAGE_PREFIX))
	}

	pub fn key((port_id, channel_id, sequence): (PortId, ChannelId, Sequence)) -> Vec<u8> {
		let mut key_buf = Vec::new();
		let channel_id = channel_id.to_string().as_bytes().to_vec();
		let port_id = port_id.as_bytes().to_vec();
		let seq = u64::from(sequence);
		key_buf.extend(channel_id);
		key_buf.extend(port_id);
		key_buf.extend(seq.to_le_bytes());
		key_buf
	}

	pub fn get(&self, key: (PortId, ChannelId, Sequence)) -> Option<Bytes> {
		let ack_raw_path = Self::key(key);
		self.0.get(&ack_raw_path)
	}

	pub fn insert(&mut self, key: (PortId, ChannelId, Sequence), raw_acknowledgement: Bytes) {
		let ack_raw_path = Self::key(key);
		self.0.set(&ack_raw_path, &raw_acknowledgement);
	}

	pub fn remove(&mut self, key: (PortId, ChannelId, Sequence)) {
		let ack_raw_path = Self::key(key);
		self.0.remove(&ack_raw_path);
	}

	pub fn contains_key(&self, key: (PortId, ChannelId, Sequence)) -> bool {
		let ack_raw_path = Self::key(key);
		self.0.get(&ack_raw_path).is_some()
	}
}

pub struct ReadonlyAcknowledgementsRaw<'a>(ReadonlyPrefixedStorage<'a>);

impl ReadonlyAcknowledgementsRaw<'_> {
	pub fn new<'a>(storage: &'a dyn Storage) -> Self {
		ReadonlyAcknowledgementsRaw(prefixed_read(storage, STORAGE_PREFIX))
	}

	pub fn get(&self, key: (PortId, ChannelId, Sequence)) -> Option<Bytes> {
		let ack_raw_path = AcknowledgementsRaw::key(key);
		self.0.get(&ack_raw_path)
	}

	pub fn contains_key(&self, key: (PortId, ChannelId, Sequence)) -> bool {
		let ack_raw_path = AcknowledgementsRaw::key(key);
		self.0.get(&ack_raw_path).is_some()
	}
}
