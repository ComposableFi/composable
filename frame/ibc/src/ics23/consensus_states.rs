use frame_support::storage::{child, ChildTriePrefixIterator};
use ibc::{
	core::ics24_host::{identifier::ClientId, path::ClientConsensusStatePath},
	Height,
};
use ibc_trait::apply_prefix_and_encode;
use sp_std::marker::PhantomData;
use tendermint_proto::Protobuf;
use crate::Config;

/// client_id, height => consensus_state
/// todo: only store up to 250 (height => consensus_state) per client_id
pub struct ConsensusStates<T>(PhantomData<T>);

impl<T: Config> ConsensusStates<T> {
	pub fn get(client_id: Vec<u8>, height: Vec<u8>) -> Option<Vec<u8>> {
		let client_id = ClientId::from_str(&String::from_utf8(client_id).ok()?).ok()?;
		let height = ibc::Height::decode(&mut &*height).ok()?;
		let consensus_path = ClientConsensusStatePath {
			client_id,
			epoch: height.revision_number,
			height: height.revision_height,
		};
		let path = format!("{}", consensus_path);
		let key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![path]).ok()?;
        child::get_raw(&T::CHILD_INFO, &key)
	}

    pub fn insert(client_id: Vec<u8>, height: Height, consensus_state: Vec<u8>) {
        let client_id = ClientId::from_str(&String::from_utf8(client_id).ok()?).ok()?;
        let consensus_path = ClientConsensusStatePath {
            client_id,
            epoch: height.revision_number,
            height: height.revision_height,
        };
        let path = format!("{}", consensus_path);
        let key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![path]).ok()?;
        child::put_raw(&T::CHILD_INFO, &key, &consensus_state)
    }

    pub fn iter_key_prefix(client_id: Vec<u8>) -> ChildTriePrefixIterator<(Vec<u8>, Vec<u8>)> {
        let prefix_path = format!("clients/{}/consensusStates", String::from_utf8(client_id).ok()?);
        let key = apply_prefix_and_encode(T::CONNECTION_PREFIX, vec![prefix_path]).ok()?;
        // todo: convert key_without_prefix to height.
        ChildTriePrefixIterator::with_prefix(&T::CHILD_INFO, &key)
    }
}
