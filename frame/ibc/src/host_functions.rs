use alloc::string::{String, ToString};
use alloc::format;
use codec::Codec;
use ibc::{
	clients::{host_functions::HostFunctionsProvider, ics11_beefy::error::Error as Ics11Error},
	core::ics02_client::error::Error as Ics02ClientError,
};
use ibc_primitives::runtime_interface;
use sp_core::{storage::ChildInfo, Hasher, H256};
use sp_runtime::traits::BlakeTwo256;
use sp_std::prelude::*;
use sp_trie::{KeySpacedDB, LayoutV0, StorageProof, Trie, TrieDB};

#[derive(Clone, Default)]
pub struct HostFunctions;

impl HostFunctionsProvider for HostFunctions {
	fn keccak_256(input: &[u8]) -> [u8; 32] {
		sp_io::hashing::keccak_256(input)
	}

	fn secp256k1_ecdsa_recover_compressed(signature: &[u8; 65], msg: &[u8; 32]) -> Option<Vec<u8>> {
		sp_io::crypto::secp256k1_ecdsa_recover_compressed(signature, msg)
			.ok()
			.map(|pub_key| pub_key.to_vec())
	}

	fn ed25519_verify(signature: &[u8; 64], msg: &[u8], pubkey: &[u8]) -> bool {
		let signature = sp_core::ed25519::Signature::from_raw(*signature);
		let public_key = if let Ok(pub_key) = sp_core::ed25519::Public::try_from(pubkey) {
			pub_key
		} else {
			return false;
		};
		sp_io::crypto::ed25519_verify(&signature, msg, &public_key)
	}

	fn verify_membership_trie_proof(
		root: &[u8; 32],
		proof: &[Vec<u8>],
		key: &[u8],
		value: &[u8],
	) -> Result<(), Ics02ClientError> {
		let root = H256::from_slice(root);
		let proof = StorageProof::new(proof.into_iter().map(Clone::clone));
		let items = vec![(key.to_vec(), Some(value.to_vec()))];
		let child_info = ChildInfo::new_default(b"/ibc");

		read_child_proof_check::<BlakeTwo256, _>(root, proof, child_info, items)
			.map_err(|e| Ics02ClientError::beefy(Ics11Error::verification_error(e.to_string())))?;

		Ok(())
	}

	fn verify_non_membership_trie_proof(
		root: &[u8; 32],
		proof: &[Vec<u8>],
		key: &[u8],
	) -> Result<(), Ics02ClientError> {
		let root = H256::from_slice(root);
		let proof = StorageProof::new(proof.into_iter().map(Clone::clone));
		let items = vec![(key.to_vec(), None)];
		let child_info = ChildInfo::new_default(b"/ibc");

		read_child_proof_check::<BlakeTwo256, _>(root, proof, child_info, items)
			.map_err(|e| Ics02ClientError::beefy(Ics11Error::verification_error(e.to_string())))?;

		Ok(())
	}

	fn verify_timestamp_extrinsic(
		root: &[u8; 32],
		proof: &[Vec<u8>],
		key: &[u8],
		value: &[u8],
	) -> Result<(), Ics02ClientError> {
		sp_trie::verify_trie_proof::<LayoutV0<BlakeTwo256>, _, _, _>(
			&H256::from_slice(&root[..]),
			proof,
			&vec![(key, Some(value))],
		)
		.map_err(|_| Ics02ClientError::beefy(Ics11Error::verification_error(format!("extrinsic proof verification failed"))))
	}

	fn sha256_digest(data: &[u8]) -> [u8; 32] {
		sp_io::hashing::sha2_256(data)
	}

	fn sha2_256(message: &[u8]) -> [u8; 32] {
		sp_io::hashing::sha2_256(message)
	}

	fn sha2_512(message: &[u8]) -> [u8; 64] {
		runtime_interface::sha2_512(message)
	}

	fn sha2_512_truncated(message: &[u8]) -> [u8; 32] {
		runtime_interface::sha2_512_truncated(message)
	}

	fn sha3_512(message: &[u8]) -> [u8; 64] {
		runtime_interface::sha3_512(message)
	}

	fn ripemd160(message: &[u8]) -> [u8; 20] {
		runtime_interface::ripemd160(message)
	}
}

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
		let recovered = child_trie.get(&key)?;

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
