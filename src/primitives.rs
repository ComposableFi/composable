use codec::{Decode, Encode};
use sp_core::H256;
use sp_core_hashing::keccak_256;
use sp_std::prelude::*;

type BeefyPayloadId = [u8; 2];
pub const MMR_ROOT_ID: [u8; 2] = *b"mh";

/// Default Merging & Hashing behavior for MMR.
pub struct Hasher<H>(sp_std::marker::PhantomData<H>);

impl<H: sp_std::convert::AsRef<[u8]> + sp_std::convert::From<[u8; 32]>> mmr_lib::Merge
    for Hasher<H>
{
    type Item = H;

    fn merge(left: &Self::Item, right: &Self::Item) -> Self::Item {
        let mut concat = left.as_ref().to_vec();
        concat.extend_from_slice(right.as_ref());

        keccak_256(&concat).into()
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct Commitment {
    pub payload: Vec<(BeefyPayloadId, Vec<u8>)>,
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

#[derive(Debug, Clone, Encode, Decode)]
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

#[derive(Debug, Clone, Encode, Decode)]
pub struct BeefyAuthoritySet {
    // id of set, should be strictly increasing by 1.
    pub id: u64,
    // len of all authorities, mmr_root_hash is considered final if we have sigs from
    // 2/3 + 1 of all authorities
    pub len: u64,
    // merkle root of all authority public keys
    pub merkle_root: H256,
}
