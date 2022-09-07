#![allow(unused_imports)]
use sp_core::H256;
use sp_runtime::AccountId32;
use sp_runtime_interface::runtime_interface;
use sp_std::prelude::*;

#[derive(codec::Encode, codec::Decode)]
pub enum SS58CodecError {
	/// Invalid SS58 String
	InvalidString,
	/// Invalid Account id
	InvalidAccountId,
}

#[runtime_interface]
pub trait Ibc {
	fn blake2_256_verify_non_membership_proof(root: &H256, proof: &[Vec<u8>], key: &[u8]) -> bool {
		use sp_trie::LayoutV0;
		sp_trie::verify_trie_proof::<LayoutV0<sp_core::Blake2Hasher>, _, _, &[u8]>(
			root,
			proof,
			&[(key, None)],
		)
		.is_ok()
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

	fn ripemd160(message: &[u8]) -> [u8; 20] {
		use ripemd::Digest;
		let mut hasher = ripemd::Ripemd160::new();
		hasher.update(message);
		let hash = hasher.finalize();
		let mut res = [0u8; 20];
		res.copy_from_slice(&hash);
		res
	}

	fn ss58_to_account_id_32(raw_str: &str) -> Result<[u8; 32], SS58CodecError> {
		use sp_core::crypto::Ss58Codec;
		AccountId32::from_string(raw_str)
			.map(|acc| acc.into())
			.map_err(|_| SS58CodecError::InvalidString)
	}

	fn account_id_to_ss58(bytes: [u8; 32]) -> Result<Vec<u8>, SS58CodecError> {
		use sp_core::crypto::Ss58Codec;
		let account_id = AccountId32::new(bytes);
		Ok(account_id.to_ss58check().as_bytes().to_vec())
	}
}
