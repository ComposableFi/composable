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

#[derive(Clone)]
pub struct HostFunctions;

impl HostFunctionsProvider for HostFunctions {
	fn keccak_256(input: &[u8]) -> [u8; 32] {
		sp_io::hashing::keccak_256(input)
	}

	fn secp256k1_ecdsa_recover_compressed(signature: &[u8; 65], msg: &[u8; 32]) -> Option<Vec<u8>> {
		sp_io::crypto::secp256k1_ecdsa_recover_compressed(signature, msg).ok().map(|pub_key| pub_key.to_vec())
	}

	fn ed25519_recover(_signature: &[u8; 64], _value: &[u8; 32]) -> Option<Vec<u8>> {
		todo!()
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
}
