use crate::{format, Config};
use alloc::string::String;
use frame_support::storage::{child, child::ChildInfo, ChildTriePrefixIterator};
use ibc::{
	core::ics24_host::{identifier::ClientId, path::ClientConsensusStatePath},
	Height,
};
use ibc_trait::apply_prefix;
use sp_std::{marker::PhantomData, prelude::*, str::FromStr};

/// client_id, height => consensus_state
/// trie key path: "clients/{client_id}/consensusStates/{height}"
/// todo: only store up to 250 (height => consensus_state) per client_id
pub struct ConsensusStates<T>(PhantomData<T>);

impl<T: Config> ConsensusStates<T> {
	pub fn get(client_id: ClientId, height: Height) -> Option<Vec<u8>> {
		let consensus_path = ClientConsensusStatePath {
			client_id,
			epoch: height.revision_number,
			height: height.revision_height,
		};
		let path = format!("{}", consensus_path);
		let key = apply_prefix(T::CONNECTION_PREFIX, vec![path]);
		child::get(&ChildInfo::new_default(T::CHILD_TRIE_KEY), &key)
	}

	pub fn insert(client_id: ClientId, height: Height, consensus_state: Vec<u8>) {
		let consensus_path = ClientConsensusStatePath {
			client_id,
			epoch: height.revision_number,
			height: height.revision_height,
		};
		let path = format!("{}", consensus_path);
		let key = apply_prefix(T::CONNECTION_PREFIX, vec![path]);
		child::put(&ChildInfo::new_default(T::CHILD_TRIE_KEY), &key, &consensus_state)
	}

	pub fn iter_key_prefix(client_id: &ClientId) -> impl Iterator<Item = (Height, Vec<u8>)> {
		let prefix_path = format!("clients/{}/consensusStates/", client_id);
		let key = apply_prefix(T::CONNECTION_PREFIX, vec![prefix_path]);
		ChildTriePrefixIterator::with_prefix(&ChildInfo::new_default(T::CHILD_TRIE_KEY), &key)
			.filter_map(|(key, value)| {
				let height = Height::from_str(&String::from_utf8(key).ok()?).ok()?;
				Some((height, value))
			})
	}
}

#[cfg(test)]
mod tests {
	use super::ConsensusStates;
	use crate::mock::*;
	use ibc::core::{
		ics02_client::{client_type::ClientType, height::Height},
		ics24_host::identifier::ClientId,
	};
	use sp_io::TestExternalities;

	#[test]
	fn test_child_trie_prefix_iterator() {
		TestExternalities::default().execute_with(|| {
			let client_id = ClientId::new(ClientType::Beefy, 1).unwrap();

			for height in 0..100u64 {
				let height = Height { revision_height: height, revision_number: 2000 };
				ConsensusStates::<Test>::insert(client_id.clone(), height, [255u8; 32].to_vec());
			}

			let item = ConsensusStates::<Test>::get(
				client_id.clone(),
				Height { revision_height: 99, revision_number: 2000 },
			);
			println!("item: {:#?}", item);

			let keys = ConsensusStates::<Test>::iter_key_prefix(&client_id).collect::<Vec<_>>();

			println!("iter_key_prefix: {:#?}", keys)
		});
	}
}
