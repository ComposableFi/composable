use crate::Config;
use frame_support::storage::{child, child::ChildInfo, ChildTriePrefixIterator};
use ibc::core::{
	ics04_channel::channel::ChannelEnd,
	ics24_host::{
		identifier::{ChannelId, PortId},
		path::ChannelEndsPath,
		Path,
	},
};
use ibc_trait::apply_prefix_and_encode;
use sp_std::marker::PhantomData;
use std::str::FromStr;
use tendermint_proto::Protobuf;

// todo: pruning
/// (port_identifier, channel_identifier) => ChannelEnd
pub struct Channels<T>(PhantomData<T>);

impl<T: Config> Channels<T> {
	pub fn get(port_id: PortId, channel_id: ChannelId) -> Option<Vec<u8>> {
		let channel_path = format!("{}", ChannelEndsPath(port_id, channel_id));
		let channel_key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![channel_path]);
		child::get(&ChildInfo::new_default(T::CHILD_TRIE_KEY), &channel_key)
	}

	pub fn insert(port_id: PortId, channel_id: ChannelId, channel_end: &ChannelEnd) {
		let channel_path = format!("{}", ChannelEndsPath(port_id, channel_id));
		let channel_key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![channel_path]);
		child::put(
			&ChildInfo::new_default(T::CHILD_TRIE_KEY),
			&channel_key,
			&channel_end.encode_vec(),
		);
	}

	pub fn iter() -> impl Iterator<Item = (Vec<u8>, Vec<u8>, Vec<u8>)> {
		let prefix = format!("channelEnds/ports/");
		let key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![prefix.clone()]);
		ChildTriePrefixIterator::with_prefix(&ChildInfo::new_default(T::CHILD_TRIE_KEY), &key)
			.filter_map(move |(key, value)| {
				let path = format!("{prefix}{}", String::from_utf8(key).ok()?);
				if let Path::ChannelEnds(ChannelEndsPath(port_id, channel_id)) =
					Path::from_str(&path).ok()?
				{
					return Some((
						port_id.as_bytes().to_vec(),
						channel_id.to_string().as_bytes().to_vec(),
						value,
					))
				}
				None
			})
	}
}
