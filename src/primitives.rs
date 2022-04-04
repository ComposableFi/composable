pub use beefy_primitives::mmr::{BeefyNextAuthoritySet, MmrLeaf};
use codec::{Decode, Encode};
use sp_core::H256;
use sp_core_hashing::keccak_256;
use sp_std::prelude::*;

pub const HASH_LENGTH: usize = 32;
pub type TSignature = [u8; 65];
pub type Hash = [u8; 32];

#[derive(Clone, sp_std::fmt::Debug, PartialEq, Eq, Encode, Decode)]
pub struct SignatureWithAuthorityIndex {
    pub signature: TSignature,
    pub index: u32,
}

#[derive(Clone, sp_std::fmt::Debug, PartialEq, Eq, Encode, Decode)]
pub struct SignedCommitment {
    pub commitment: beefy_primitives::Commitment<u32>,
    pub signatures: Vec<SignatureWithAuthorityIndex>,
}

#[derive(sp_std::fmt::Debug, Clone, Encode, Decode)]
pub struct MmrUpdateProof {
    pub signed_commitment: SignedCommitment,
    pub latest_mmr_leaf: MmrLeaf<u32, H256, H256>,
    pub mmr_proof: pallet_mmr_primitives::Proof<H256>,
    pub authority_proof: Vec<Hash>,
}

#[derive(Clone)]
pub struct KeccakHasher;

impl rs_merkle::Hasher for KeccakHasher {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> [u8; 32] {
        keccak_256(data)
    }
}
