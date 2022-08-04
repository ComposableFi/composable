use alloc::string::String;
use codec::{Codec, Decode};
use sp_core::{storage::ChildInfo, Hasher};
use sp_std::prelude::*;
use sp_trie::{KeySpacedDB, LayoutV0, StorageProof, Trie, TrieDB};

#[derive(derive_more::From, derive_more::Display)]
pub enum Error<H: Hasher> {
	#[display(fmt = "Trie Error: {:?}", _0)]
	Trie(Box<sp_trie::TrieError<LayoutV0<H>>>),
	#[display(fmt = "Error verifying key: {key:?}, Expected: {expected:?}, Got: {got:?}")]
	ValueMismatch { key: Option<String>, expected: Option<Vec<u8>>, got: Option<Vec<u8>> },
	#[display(fmt = "Couldn't find child root in proofs")]
	ChildRootNotFound,
}

/// Lifted directly from [sp-state-machine](https://github.com/paritytech/substrate/blob/b27c470eaff379f512d1dec052aff5d551ed3b03/primitives/state-machine/src/lib.rs#L1138-L1161)
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
