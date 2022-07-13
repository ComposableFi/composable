use crate::Config;
use frame_support::storage::{child, child::ChildInfo};
use ibc::core::ics24_host::{
	identifier::{ChannelId, PortId},
	path::SeqRecvsPath,
};
use ibc_trait::apply_prefix_and_encode;
use sp_std::marker::PhantomData;

/// (port_identifier, channel_identifier) => Sequence
pub struct NextSequenceRecv<T>(PhantomData<T>);

impl<T: Config> NextSequenceRecv<T> {
	pub fn get(port_id: PortId, channel_id: ChannelId) -> Option<u64> {
		let next_seq_send_path = format!("{}", SeqRecvsPath(port_id.clone(), channel_id));
		let next_seq_send_key =
			apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![next_seq_send_path]);
		child::get(&ChildInfo::new_default(T::CHILD_INFO_KEY), &next_seq_send_key)
	}

	pub fn insert(port_id: PortId, channel_id: ChannelId, seq: u64) {
		let next_seq_send_path = format!("{}", SeqRecvsPath(port_id.clone(), channel_id));
		let next_seq_send_key =
			apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![next_seq_send_path]);
		child::put(&ChildInfo::new_default(T::CHILD_INFO_KEY), &next_seq_send_key, &seq)
	}
}
