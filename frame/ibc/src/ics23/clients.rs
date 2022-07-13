use crate::Config;
use frame_support::storage::child;
use ibc::core::ics24_host::{identifier::ClientId, path::ClientTypePath};
use ibc_trait::apply_prefix_and_encode;
use sp_std::marker::PhantomData;

/// client_id => client_type
pub struct Clients<T>(PhantomData<T>);

impl<T: Config> Clients<T> {
	pub fn get(client_id: Vec<u8>) -> Option<Vec<u8>> {
		let client_id = ClientId::from_str(&String::from_utf8(client_id).ok()?).ok()?;
		let client_type_path = format!("{}", ClientTypePath(client_id));
		let client_type_key =
			apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![client_type_path]).ok()?;
		child::get_raw(&T::CHILD_INFO, &client_type_key)
	}

	pub fn insert(client_id: Vec<u8>, client_type: Vec<u8>) {
		let client_id = ClientId::from_str(&String::from_utf8(client_id).ok()?).ok()?;
		let client_type_path = format!("{}", ClientTypePath(client_id));
		let client_type_key =
			apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![client_type_path]).ok()?;
		child::put_raw(&T::CHILD_INFO, &client_type_key, &client_type);
	}

	pub fn contains_key(client_id: Vec<u8>) -> bool {
		let client_id = ClientId::from_str(&String::from_utf8(client_id).ok()?).ok()?;
		let client_type_path = format!("{}", ClientTypePath(client_id));
		let client_type_key =
			apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![client_type_path]).ok()?;
		child::exists(&T::CHILD_INFO, &client_type_key)
	}
}
