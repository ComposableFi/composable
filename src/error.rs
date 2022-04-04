use sp_std::prelude::*;
#[derive(sp_std::fmt::Debug, PartialEq, Eq)]
pub enum BeefyClientError {
    /// Failed to read a value from storage
    StorageReadError,
    /// Failed to write a value to storage
    StorageWriteError,
    /// Error decoding some value
    DecodingError,
    /// Invalid Mmr Update
    InvalidMmrUpdate,
    /// Incomplete Signature threshold
    IncompleteSignatureThreshold,
    /// Error recovering public key from signature
    InvalidSignature,
    /// Some invalid merkle root hash
    InvalidRootHash,
    /// Some invalid mmr proof
    InvalidMmrProof,
    /// Invalid authority proof
    InvalidAuthorityProof,
}
