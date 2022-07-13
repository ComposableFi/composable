use crate::Config;
use frame_support::storage::{child, child::ChildInfo};
use ibc::core::ics24_host::{identifier::ClientId, path::ClientStatePath};
use ibc_trait::apply_prefix_and_encode;
use sp_std::marker::PhantomData;

/// client_id => client_states
pub struct ClientStates<T>(PhantomData<T>);

impl<T: Config> ClientStates<T> {
	pub fn get(client_id: &ClientId) -> Option<Vec<u8>> {
		let client_state_path = format!("{}", ClientStatePath(client_id.clone()));
		let client_state_key =
			apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![client_state_path]);
		child::get(&ChildInfo::new_default(T::CHILD_INFO_KEY), &client_state_key)
	}

	pub fn insert(client_id: &ClientId, client_state: Vec<u8>) {
		let client_state_path = format!("{}", ClientStatePath(client_id.clone()));
		let client_state_key =
			apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![client_state_path]);
		child::put(&ChildInfo::new_default(T::CHILD_INFO_KEY), &client_state_key, &client_state);
	}

	pub fn contains_key(client_id: &ClientId) -> bool {
		let client_state_path = format!("{}", ClientStatePath(client_id.clone()));
		let client_state_key =
			apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![client_state_path]);
		child::exists(&ChildInfo::new_default(T::CHILD_INFO_KEY), &client_state_key)
	}
}
