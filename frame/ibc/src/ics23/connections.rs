use crate::Config;
use frame_support::storage::{child, child::ChildInfo, ChildTriePrefixIterator};
use ibc::core::{
	ics03_connection::connection::ConnectionEnd,
	ics24_host::{
		identifier::{ ConnectionId},
		path::{ ConnectionsPath},
	},
};
use ibc_trait::apply_prefix_and_encode;
use sp_std::marker::PhantomData;
use tendermint_proto::Protobuf;

/// connection_id => ConnectionEnd
pub struct Connections<T>(PhantomData<T>);

impl<T: Config> Connections<T> {
	pub fn get(connection_id: &ConnectionId) -> Option<Vec<u8>> {
		let connection_path = format!("{}", ConnectionsPath(connection_id.clone()));
		let connection_key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![connection_path]);
		child::get(&ChildInfo::new_default(T::CHILD_INFO_KEY), &connection_key)
	}

	pub fn insert(connection_id: &ConnectionId, connection_end: &ConnectionEnd) {
		let connection_path = format!("{}", ConnectionsPath(connection_id.clone()));
		let connection_key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![connection_path]);
		child::put(
			&ChildInfo::new_default(T::CHILD_INFO_KEY),
			&connection_key,
			&connection_end.encode_vec(),
		);
	}

	pub fn iter() -> ChildTriePrefixIterator<(Vec<u8>, Vec<u8>)> {
		let prefix_path = format!("connections/");
		let key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![prefix_path]);
		ChildTriePrefixIterator::with_prefix(&ChildInfo::new_default(T::CHILD_INFO_KEY), &key)
	}
}
