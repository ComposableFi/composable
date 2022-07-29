//! Light client error definition
use codec::alloc::string::String;
use sp_std::prelude::*;
#[derive(sp_std::fmt::Debug, derive_more::From)]
/// Error definition for the light client
pub enum BeefyClientError {
    /// Outdated commitment
    #[from(ignore)]
    OutdatedCommitment {
        /// Latest beefy height stored
        latest_beefy_height: u32,
        /// Commitment block number received
        commitment_block_number: u32,
    },
    /// Mmr root hash not found in commitment
    MmrRootHashNotFound,
    /// Invalid Authority set id received
    #[from(ignore)]
    AuthoritySetMismatch {
        /// Current authority set id
        current_set_id: u64,
        /// Next authority set id
        next_set_id: u64,
        /// Authority set id in commitment
        commitment_set_id: u64,
    },
    /// Incomplete Signature threshold
    IncompleteSignatureThreshold,
    /// Error recovering public key from signature
    InvalidSignature,
    /// Some invalid merkle root hash
    #[from(ignore)]
    InvalidRootHash {
        /// Root hash
        root_hash: Vec<u8>,
        /// Root hash length
        len: u64,
    },
    /// Some invalid mmr proof
    InvalidMmrProof,
    /// Invalid authority proof
    InvalidAuthorityProof,
    /// Invalid merkle proof
    InvalidMerkleProof,
    /// Mmr Error
    MmrVerificationError(mmr_lib::Error),
    #[cfg(any(test, feature = "prover"))]
    /// subxt error
    Subxt(subxt::BasicError),
    #[cfg(any(test, feature = "prover"))]
    /// subxt rpc error
    SubxtRRpc(subxt::rpc::RpcError),
    #[cfg(any(test, feature = "prover"))]
    /// Trie error
    TrieProof(Box<sp_trie::TrieError<sp_trie::LayoutV0<sp_runtime::traits::BlakeTwo256>>>),
    /// Codec error
    Codec(codec::Error),
    /// Custom error
    #[from(ignore)]
    Custom(String),
}
