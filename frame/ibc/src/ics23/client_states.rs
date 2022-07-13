use crate::{Config, Error};
use frame_support::storage::child;
use ibc::core::ics24_host::{identifier::ClientId, path::ClientStatePath};
use ibc_trait::apply_prefix_and_encode;
use sp_core::storage::ChildInfo;
use sp_std::marker::PhantomData;

/// client_id => client_states
pub struct ClientStates<T>(PhantomData<T>);

impl<T: Config> ClientStates<T> {
	pub fn get(client_id: Vec<u8>) -> Option<Vec<u8>> {
		let client_id = ClientId::from_str(&String::from_utf8(client_id).ok()?).ok()?;
		let client_state_path = format!("{}", ClientStatePath(client_id.clone()));
		let client_state_key =
			apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![client_state_path]).ok()?;
		child::get_raw(&T::CHILD_INFO, &client_state_key)
	}

	pub fn insert(client_id: Vec<u8>, client_state: Vec<u8>) {
		let client_id = ClientId::from_str(&String::from_utf8(client_id).ok()?).ok()?;
		let client_state_path = format!("{}", ClientStatePath(client_id.clone()));
		let client_state_key =
			apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![client_state_path]).ok()?;
		child::put_raw(&T::CHILD_INFO, &client_state_key, &client_state);
	}

	pub fn contains_key(client_id: Vec<u8>) -> bool {
		let client_id = ClientId::from_str(&String::from_utf8(client_id).ok()?).ok()?;
		let client_state_path = format!("{}", ClientStatePath(client_id.clone()));
		let client_state_key =
			apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![client_state_path]).ok()?;
		child::exists(&T::CHILD_INFO, &client_state_key)
	}
}
