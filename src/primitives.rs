use codec::{Decode, Encode};
use sp_core::H256;
use sp_std::prelude::*;

#[derive(Debug, Clone, Encode, Decode)]
pub struct Commitment {
    pub payload: Vec<u8>,
    pub block_number: u32,
    pub validator_set_id: u64,
}

#[derive(Debug, Clone)]
pub struct AuthoritySignature {
    pub signature: [u8; 65],
    pub authority_index: u64,
}

#[derive(Debug, Clone)]
pub struct SignedCommitment {
    pub commitment: Commitment,
    pub signatures: Vec<AuthoritySignature>,
}

#[derive(Debug, Clone)]
pub struct MmrLeaf<BlockNumber, Hash, MerkleRoot> {
    pub version: u8,
    pub parent_number_and_hash: (BlockNumber, Hash),
    pub beefy_next_authority_set: BeefyAuthoritySet,
    pub parachain_heads: MerkleRoot,
}

#[derive(Debug, Clone)]
pub struct MmrLeafWithIndex {
    pub leaf: MmrLeaf<u32, H256, H256>, // see beefy_primitives
    pub index: u64,
}

#[derive(Debug, Clone)]
pub struct MmrUpdateProof {
    pub signed_commitment: SignedCommitment,
    pub latest_mmr_leaf_with_index: MmrLeafWithIndex,
    pub mmr_proof: Vec<H256>,
}

#[derive(Debug, Clone)]
pub struct BeefyAuthoritySet {
    // id of set, should be strictly increasing by 1.
    pub id: u64,
    // len of all authorities, mmr_root_hash is considered final if we have sigs from
    // 2/3 + 1 of all authorities
    pub len: u64,
    // merkle root of all authority public keys
    pub merkle_root: H256,
}
