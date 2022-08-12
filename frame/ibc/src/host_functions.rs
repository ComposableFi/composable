use crate::{state_machine, Config};
use alloc::{format, string::ToString};
use codec::Encode;
use ibc::{
	clients::{host_functions::HostFunctionsProvider, ics11_beefy::error::Error as Ics11Error},
	core::ics02_client::error::Error as Ics02ClientError,
};
use ibc_primitives::runtime_interface;
use sp_core::{storage::ChildInfo, H256};
use sp_runtime::traits::BlakeTwo256;
use sp_std::{marker::PhantomData, prelude::*};
use sp_trie::{LayoutV0, StorageProof};

#[derive(Clone)]
pub struct HostFunctions<T>(PhantomData<T>);

impl<T: Clone> Default for HostFunctions<T> {
	fn default() -> Self {
		Self(PhantomData::default())
	}
}

impl<T> HostFunctionsProvider for HostFunctions<T>
where
	T: Config + Send + Sync + 'static,
{
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
			return false
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
		let child_info = ChildInfo::new_default(T::CHILD_TRIE_KEY);

		state_machine::read_child_proof_check::<BlakeTwo256, _>(root, proof, child_info, items)
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

		state_machine::read_child_proof_check::<BlakeTwo256, _>(root, proof, child_info, items)
			.map_err(|e| Ics02ClientError::beefy(Ics11Error::verification_error(e.to_string())))?;

		Ok(())
	}

	fn verify_timestamp_extrinsic(
		root: &[u8; 32],
		proof: &[Vec<u8>],
		value: &[u8],
	) -> Result<(), Ics02ClientError> {
		// Timestamp extrinsic should be the first inherent and hence the first extrinsic
		// https://github.com/paritytech/substrate/blob/d602397a0bbb24b5d627795b797259a44a5e29e9/primitives/trie/src/lib.rs#L99-L101
		let key = codec::Compact(0u32).encode();
		sp_trie::verify_trie_proof::<LayoutV0<BlakeTwo256>, _, _, _>(
			&H256::from_slice(&root[..]),
			proof,
			&vec![(key, Some(value))],
		)
		.map_err(|_| {
			Ics02ClientError::beefy(Ics11Error::verification_error(format!(
				"extrinsic proof verification failed"
			)))
		})
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
