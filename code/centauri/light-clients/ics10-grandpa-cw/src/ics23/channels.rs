use crate::{Bytes, STORAGE_PREFIX};
use cosmwasm_std::Storage;
use cosmwasm_storage::{prefixed, prefixed_read, PrefixedStorage, ReadonlyPrefixedStorage};
use ibc::{
	core::{
		ics04_channel::channel::ChannelEnd,
		ics24_host::{
			identifier::{ChannelId, PortId},
			path::ChannelEndsPath,
		},
	},
	protobuf::Protobuf,
};

/// (port_id, channel_id) => ChannelEnd
/// trie key path: "channelEnds/ports/{port_id}/channels/{channel_id}"
pub struct Channels<'a>(PrefixedStorage<'a>);

impl Channels<'_> {
	pub fn new<'a>(storage: &'a mut dyn Storage) -> Self {
		Channels(prefixed(storage, STORAGE_PREFIX))
	}

	pub fn key(port_id: PortId, channel_id: ChannelId) -> Vec<u8> {
		let channel_path = ChannelEndsPath(port_id, channel_id);
		let channel_path = format!("{}", channel_path);
		channel_path.into_bytes()
	}

	pub fn get(&self, key: (PortId, ChannelId)) -> Option<Bytes> {
		let channel_path = Self::key(key.0, key.1);
		self.0.get(&channel_path)
	}

	pub fn insert(&mut self, key: (PortId, ChannelId), channel: ChannelEnd) {
		let channel_path = Self::key(key.0, key.1);
		self.0.set(&channel_path, &channel.encode_vec());
	}
}

pub struct ReadonlyChannels<'a>(ReadonlyPrefixedStorage<'a>);

impl ReadonlyChannels<'_> {
	pub fn new<'a>(storage: &'a dyn Storage) -> Self {
		ReadonlyChannels(prefixed_read(storage, STORAGE_PREFIX))
	}

	pub fn get(&self, key: (PortId, ChannelId)) -> Option<Bytes> {
		let channel_path = Channels::key(key.0, key.1);
		self.0.get(&channel_path)
	}
}
