use crate::runtime_interface;
use ibc::{
	clients::{host_functions::HostFunctionsProvider, ics11_beefy::error::Error as Ics11Error},
	core::{
		ics02_client::error::Error as Ics02ClientError,
		ics23_commitment::error::Error as Ics23Error,
	},
};
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

	fn verify_membership_trie_proof(
		root: &H256,
		proof: &[Vec<u8>],
		key: &[u8],
		value: &[u8],
	) -> Result<(), Ics02ClientError> {
		sp_io::trie::blake2_256_verify_proof(*root, proof, key, value, StateVersion::V0)
			.then(|| ())
			.ok_or_else(|| {
				Ics02ClientError::beefy(Ics11Error::ics23_error(Ics23Error::verification_failure()))
			})
	}

	fn verify_non_membership_trie_proof(
		root: &H256,
		proof: &[Vec<u8>],
		key: &[u8],
	) -> Result<(), Ics02ClientError> {
		runtime_interface::trie::blake2_256_verify_non_membership_proof(root, proof, key)
			.then(|| ())
			.ok_or_else(|| {
				Ics02ClientError::beefy(Ics11Error::ics23_error(Ics23Error::verification_failure()))
			})
	}

	fn sha256_digest(data: &[u8]) -> [u8; 32] {
		sp_io::hashing::sha2_256(data)
	}

	// These are temporary implementations to fix build errors, will be replaced with
	// host functions in https://github.com/ComposableFi/composable/pull/1122
	fn ripemd160(message: &[u8]) -> [u8; 20] {
		use ripemd::Digest;
		let mut hasher = ripemd::Ripemd160::new();
		hasher.update(message);
		let hash = hasher.finalize();
		let mut res = [0u8; 20];
		res.copy_from_slice(&hash);
		res
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

	fn sha2_256(message: &[u8]) -> [u8; 32] {
		sp_io::hashing::sha2_256(message)
	}

	fn sha2_512(message: &[u8]) -> [u8; 64] {
		use sha2::Digest;
		let mut hasher = sha2::Sha512::new();
		hasher.update(message);
		let hash = hasher.finalize();
		let mut res = [0u8; 64];
		res.copy_from_slice(&hash);
		res
	}

	fn sha2_512_truncated(message: &[u8]) -> [u8; 32] {
		use sha2::Digest;
		let mut hasher = sha2::Sha512::new();
		hasher.update(message);
		let hash = hasher.finalize();
		let mut res = [0u8; 32];
		res.copy_from_slice(&hash[..32]);
		res
	}

	fn sha3_512(message: &[u8]) -> [u8; 64] {
		use sha3::Digest;
		let mut hasher = sha3::Sha3_512::new();
		hasher.update(message);
		let hash = hasher.finalize();
		let mut res = [0u8; 64];
		res.copy_from_slice(&hash);
		res
	}
}
