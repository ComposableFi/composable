pub use beefy_primitives::{
    known_payload_ids::MMR_ROOT_ID,
    mmr::{BeefyNextAuthoritySet, MmrLeaf},
    Commitment, SignedCommitment,
};
use codec::{Decode, Encode};
use sp_core::H256;
use sp_core_hashing::keccak_256;
use sp_std::prelude::*;

pub const HASH_LENGTH: usize = 32;
pub const SIGNATURE_LEN: usize = 65;
#[derive(Debug, Clone, Encode, Decode)]
pub struct MmrLeafWithIndex {
    pub leaf: MmrLeaf<u32, H256, H256>, // see beefy_primitives
    pub index: u64,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct MmrUpdateProof {
    pub signed_commitment: SignedCommitment<u32, Vec<u8>>,
    pub latest_mmr_leaf_with_index: MmrLeafWithIndex,
    pub mmr_proof: Vec<H256>,
    pub authority_proof: Vec<H256>,
}

#[derive(Debug, Clone)]
pub struct KeccakHasher;

impl rs_merkle::Hasher for KeccakHasher {
    type Hash = [u8; 32];
    fn hash(x: &[u8]) -> Self::Hash {
        keccak_256(x)
    }
}
