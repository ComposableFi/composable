//! Primitive types used in the library

use beefy_primitives::mmr::MmrLeafVersion;
pub use beefy_primitives::mmr::{BeefyNextAuthoritySet, MmrLeaf};
use codec::{Decode, Encode};
use sp_core::H256;
use sp_std::prelude::*;

/// Hash length definition for hashing algorithms used
pub const HASH_LENGTH: usize = 32;
/// Authority Signature type
pub type TSignature = [u8; 65];
/// Represents a Hash in this library
pub type Hash = [u8; 32];

#[derive(Clone, sp_std::fmt::Debug, PartialEq, Eq, Encode, Decode)]
/// Authority signature and its index in the signatures array
pub struct SignatureWithAuthorityIndex {
    /// Authority signature
    pub signature: TSignature,
    /// Index in signatures vector
    pub index: u32,
}

#[derive(Clone, sp_std::fmt::Debug, PartialEq, Eq, Encode, Decode)]
/// Signed commitment
pub struct SignedCommitment {
    /// Commitment
    pub commitment: beefy_primitives::Commitment<u32>,
    /// Signatures for this commitment
    pub signatures: Vec<SignatureWithAuthorityIndex>,
}

#[derive(sp_std::fmt::Debug, Clone, PartialEq, Eq, Encode, Decode)]
/// Mmr Update with proof
pub struct MmrUpdateProof {
    /// Signed commitment
    pub signed_commitment: SignedCommitment,
    /// Latest leaf added to mmr
    pub latest_mmr_leaf: MmrLeaf<u32, H256, H256, H256>,
    /// Proof for the latest mmr leaf
    pub mmr_proof: pallet_mmr_primitives::Proof<H256>,
    /// Proof for authorities in current session
    pub authority_proof: Vec<Hash>,
}

#[derive(sp_std::fmt::Debug, Clone, PartialEq, Eq, Encode, Decode)]
/// A partial representation of the mmr leaf
pub struct PartialMmrLeaf {
    /// Leaf version
    pub version: MmrLeafVersion,
    /// Parent block number and hash
    pub parent_number_and_hash: (u32, H256),
    /// Next beefy authorities
    pub beefy_next_authority_set: BeefyNextAuthoritySet<H256>,
}
#[derive(sp_std::fmt::Debug, Clone, PartialEq, Eq, Encode, Decode)]
/// Parachain header definition
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
    /// Timestamp extrinsic
    pub timestamp_extrinsic: Vec<u8>,
}

#[derive(sp_std::fmt::Debug, Clone, PartialEq, Eq, Encode, Decode)]
/// Parachain headers update with proof
pub struct ParachainsUpdateProof {
    /// Parachai headers
    pub parachain_headers: Vec<ParachainHeader>,
    /// Mmr Batch proof for parachain headers
    pub mmr_proof: pallet_mmr_primitives::BatchProof<H256>,
}
