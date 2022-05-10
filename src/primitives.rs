use beefy_primitives::mmr::MmrLeafVersion;
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

#[derive(sp_std::fmt::Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct MmrUpdateProof {
    pub signed_commitment: SignedCommitment,
    pub latest_mmr_leaf: MmrLeaf<u32, H256, H256>,
    pub mmr_proof: pallet_mmr_primitives::Proof<H256>,
    pub authority_proof: Vec<Hash>,
}

#[derive(sp_std::fmt::Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct PartialMmrLeaf {
    pub version: MmrLeafVersion,
    pub parent_number_and_hash: (u32, H256),
    pub beefy_next_authority_set: BeefyNextAuthoritySet<H256>,
}

#[derive(sp_std::fmt::Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct ParachainHeader {
    /// scale encoded parachain header
    pub parachain_header: Vec<u8>,
    /// Reconstructed mmr leaf
    pub partial_mmr_leaf: PartialMmrLeaf,
    /// parachain id
    pub para_id: u32,
    /// Proof for our parachain header inclusion in the parachain headers root
    pub parachain_heads_proof: Vec<Hash>,
    /// leaf index for parachain heads proof
    pub heads_leaf_index: u32,
    /// Total number of parachain heads
    pub heads_total_count: u32,
    /// Trie merkle proof of inclusion of the set timestamp extrinsic in header.extrinsic_root
    /// this already encodes the actual extrinsic
    pub extrinsic_proof: Vec<Vec<u8>>,
}

#[derive(sp_std::fmt::Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct ParachainsUpdateProof {
    pub parachain_headers: Vec<ParachainHeader>,
    pub mmr_proof: pallet_mmr_primitives::BatchProof<H256>,
}

#[derive(Clone)]
pub struct KeccakHasher;

impl rs_merkle::Hasher for KeccakHasher {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> [u8; 32] {
        keccak_256(data)
    }
}
