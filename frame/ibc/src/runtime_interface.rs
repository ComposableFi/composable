use sp_core::H256;
use sp_runtime_interface::runtime_interface;
use sp_trie::LayoutV0;

#[runtime_interface]
pub trait Trie {
	fn blake2_256_verify_non_membership_proof(root: &H256, proof: &[Vec<u8>], key: &[u8]) -> bool {
		sp_trie::verify_trie_proof::<LayoutV0<sp_core::Blake2Hasher>, _, _, &[u8]>(
			root,
			proof,
			&[(key, None)],
		)
		.is_ok()
	}
}
