use sp_core::H256;
use sp_std::prelude::*;

#[derive(Debug, Clone)]
pub struct Commitment {
    mmr_root_hash: H256,
    block_number: u32,
    validator_set_id: u64,
}

#[derive(Debug, Clone)]
pub struct AuthoritySignature {
    signature: [u8; 65],
    authority_index: u64,
}

#[derive(Debug, Clone)]
pub struct SignedCommitment {
    commitment: Commitment,
    signatures: Vec<AuthoritySignature>,
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
    leaf: MmrLeaf<u32, H256, H256>, // see beefy_primitives
    index: u64,
}

#[derive(Debug, Clone)]
pub struct MmrUpdateProof {
    signed_commitment: SignedCommitment,
    latest_mmr_leaf_with_index: MmrLeafWithIndex,
    mmr_proof: Vec<H256>,
}

#[derive(Debug, Clone)]
pub struct BeefyAuthoritySet {
    // id of set, should be strictly increasing by 1.
    pub(crate) id: u64,
    // len of all authorities, mmr_root_hash is considered final if we have sigs from
    // 2/3 + 1 of all authorities
    pub(crate) len: u64,
    // merkle root of all authority public keys
    pub(crate) merkle_root: H256,
}
