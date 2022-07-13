use crate::Config;
use frame_support::storage::{child, child::ChildInfo};
use ibc::core::ics24_host::{
	identifier::{ChannelId, PortId},
	path::SeqAcksPath,
};
use ibc_trait::apply_prefix_and_encode;
use sp_std::marker::PhantomData;

/// (port_identifier, channel_identifier) => Sequence
pub struct NextSequenceAck<T>(PhantomData<T>);

impl<T: Config> NextSequenceAck<T> {
	pub fn get(port_id: PortId, channel_id: ChannelId) -> Option<u64> {
		let next_seq_ack_path = format!("{}", SeqAcksPath(port_id.clone(), channel_id));
		let next_seq_ack_key =
			apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![next_seq_ack_path]);
		child::get(&ChildInfo::new_default(T::CHILD_INFO_KEY), &next_seq_ack_key)
	}

	pub fn insert(port_id: PortId, channel_id: ChannelId, seq: u64) {
		let next_seq_ack_path = format!("{}", SeqAcksPath(port_id.clone(), channel_id));
		let next_seq_ack_key =
			apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![next_seq_ack_path]);
		child::put(&ChildInfo::new_default(T::CHILD_INFO_KEY), &next_seq_ack_key, &seq)
	}
}
