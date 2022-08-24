use frame_support::storage::child::ChildInfo;
use parachain::{ParachainClient, ParachainClientConfig};
use primitives::IbcProvider;
use sp_core::Hasher;
use sp_keystore::{testing::KeyStore, SyncCryptoStore, SyncCryptoStorePtr};
use sp_runtime::{traits::BlakeTwo256, KeyTypeId, MultiSigner};

use codec::{Codec, Decode};
use ibc::{
	core::ics24_host::{
		identifier::{ChannelId, ClientId, PortId},
		path::ClientStatePath,
	},
	Height,
};
use log::LevelFilter;
use sp_trie::{KeySpacedDB, TrieDB};
use state_machine::{LayoutV0, StorageProof};
use std::{str::FromStr, sync::Arc};

use common::{Args, DefaultConfig};
mod common;
use prost::Message;
use sp_trie::Trie;

#[derive(derive_more::From, derive_more::Display, Debug)]
pub enum Error<H: Hasher> {
	#[display(fmt = "Trie Error: {:?}", _0)]
	Trie(Box<sp_trie::TrieError<LayoutV0<H>>>),
	#[display(fmt = "Error verifying key: {key:?}, Expected: {expected:?}, Got: {got:?}")]
	ValueMismatch { key: Option<String>, expected: Option<Vec<u8>>, got: Option<Vec<u8>> },
	#[display(fmt = "Couldn't find child root in proofs")]
	ChildRootNotFound,
}

pub fn read_child_proof_check<H, I>(
	root: H::Out,
	proof: StorageProof,
	child_info: ChildInfo,
	items: I,
) -> Result<(), Error<H>>
where
	H: Hasher,
	H::Out: Ord + Codec + 'static,
	I: IntoIterator<Item = (Vec<u8>, Option<Vec<u8>>)>,
{
	let memory_db = proof.into_memory_db::<H>();
	let trie = TrieDB::<LayoutV0<H>>::new(&memory_db, &root)?;
	let child_root = trie
		.get(child_info.prefixed_storage_key().as_slice())?
		.map(|r| {
			let mut hash = H::Out::default();

			// root is fetched from DB, not writable by runtime, so it's always valid.
			hash.as_mut().copy_from_slice(&r[..]);

			hash
		})
		.ok_or_else(|| Error::<H>::ChildRootNotFound)?;

	let child_db = KeySpacedDB::new(&memory_db, child_info.keyspace());
	let child_trie = TrieDB::<LayoutV0<H>>::new(&child_db, &child_root)?;

	for (key, value) in items {
		let recovered =
			child_trie.get(&key)?.map(|val| Decode::decode(&mut &val[..]).ok()).flatten();

		if recovered != value {
			Err(Error::ValueMismatch {
				key: String::from_utf8(key).ok(),
				expected: value,
				got: recovered,
			})?
		}
	}
	Ok(())
}

#[tokio::test]
async fn test() {
	env_logger::builder().filter(Some("*"), LevelFilter::Info);
	let args = Args::default();
	let alice = sp_keyring::AccountKeyring::Alice;
	let alice_pub_key = MultiSigner::Sr25519(alice.public());

	let key_store: SyncCryptoStorePtr = Arc::new(KeyStore::new());
	let key_type_id = KeyTypeId::from(0u32);

	SyncCryptoStore::insert_unknown(&*key_store, key_type_id, "//Alice", &alice.public().0)
		.unwrap();
	// Create client configurations
	let config_a = ParachainClientConfig {
		para_id: args.para_id_a,
		parachain_rpc_url: args.chain_a,
		relay_chain_rpc_url: args.relay_chain.clone(),
		client_id: None,
		commitment_prefix: args.connection_prefix_a.as_bytes().to_vec(),
		public_key: alice_pub_key.clone(),
		key_store: key_store.clone(),
		key_type_id,
	};

	let chain = ParachainClient::<DefaultConfig>::new(config_a).await.unwrap();

	let client_state_response = chain
		.query_client_state(Height::new(2001, 7), ClientId::from_str("11-beefy-0").unwrap())
		.await
		.unwrap();
	let proof: Vec<Vec<u8>> = Decode::decode(&mut &client_state_response.proof[..]).unwrap();
	let storage_proof = StorageProof::new(proof);
	let key = format!("ibc/{}", ClientStatePath(ClientId::from_str("11-beefy-0").unwrap()))
		.as_bytes()
		.to_vec();
	let child_info = ChildInfo::new_default(b"ibc/");
	let block_hash = chain.para_client.rpc().block_hash(Some(7u32.into())).await.unwrap();
	let header = chain.para_client.rpc().header(block_hash).await.unwrap().unwrap();

	read_child_proof_check::<BlakeTwo256, _>(
		header.state_root.into(),
		storage_proof,
		child_info,
		vec![(key.clone(), client_state_response.client_state.map(|a| a.encode_to_vec()))],
	)
	.expect("failed to verify key");
}
