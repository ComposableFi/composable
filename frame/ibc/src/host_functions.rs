use ibc::{
	clients::{host_functions::HostFunctionsProvider, ics11_beefy::error::Error as Ics11Error},
	core::{
		ics02_client::error::Error as Ics02ClientError,
		ics23_commitment::error::Error as Ics23Error,
	},
};
use ibc_primitives::runtime_interface;
use sp_core::{storage::StateVersion, H256};
use sp_std::prelude::*;

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
		sp_io::trie::blake2_256_verify_proof(root, proof, key, value, StateVersion::V0)
			.then(|| ())
			.ok_or_else(|| {
				Ics02ClientError::beefy(Ics11Error::ics23_error(Ics23Error::verification_failure()))
			})
	}

	fn verify_non_membership_trie_proof(
		root: &[u8; 32],
		proof: &[Vec<u8>],
		key: &[u8],
	) -> Result<(), Ics02ClientError> {
		let root = H256::from_slice(root);
		runtime_interface::ibc::blake2_256_verify_non_membership_proof(&root, proof, key)
			.then(|| ())
			.ok_or_else(|| {
				Ics02ClientError::beefy(Ics11Error::ics23_error(Ics23Error::verification_failure()))
			})
	}

	fn sha256_digest(data: &[u8]) -> [u8; 32] {
		sp_io::hashing::sha2_256(data)
	}

	fn sha2_256(message: &[u8]) -> [u8; 32] {
		sp_io::hashing::sha2_256(message)
	}

	fn sha2_512(message: &[u8]) -> [u8; 64] {
		runtime_interface::ibc::sha2_512(message)
	}

	fn sha2_512_truncated(message: &[u8]) -> [u8; 32] {
		runtime_interface::ibc::sha2_512_truncated(message)
	}

	fn sha3_512(message: &[u8]) -> [u8; 64] {
		runtime_interface::ibc::sha3_512(message)
	}

	fn ripemd160(message: &[u8]) -> [u8; 20] {
		runtime_interface::ibc::ripemd160(message)
	}
}
