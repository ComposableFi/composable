pub use beefy_primitives::mmr::{BeefyNextAuthoritySet, MmrLeaf};
use codec::{Decode, Encode};
use sp_core::H256;
use sp_core_hashing::keccak_256;
use sp_std::prelude::*;

#[derive(Debug, Clone, Encode, Decode)]
pub struct MmrLeafWithIndex {
    pub leaf: MmrLeaf<u32, H256, H256>, // see beefy_primitives
    pub index: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
pub struct Commitment {
    pub mmr_root_hash: H256,
    pub block_number: u32,
    pub validator_set_id: u64,
}

type TSignature = [u8; 65];
type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
pub struct SignedCommitment {
    pub commitment: Commitment,
    pub signatures: Vec<Option<TSignature>>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct MmrUpdateProof {
    pub signed_commitment: SignedCommitment,
    pub latest_mmr_leaf_with_index: MmrLeafWithIndex,
    pub mmr_proof: Vec<H256>,
    pub authority_proof: Vec<Hash>,
}

#[derive(Debug, Clone)]
pub struct KeccakHasher;

impl rs_merkle::Hasher for KeccakHasher {
    type Hash = Hash;
    fn hash(x: &[u8]) -> Self::Hash {
        keccak_256(x)
    }
}
