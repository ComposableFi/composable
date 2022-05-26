//! Primitive types used in the library

use crate::primitives::BeefyNextAuthoritySet;
use codec::{Decode, Encode};
use sp_core::H256;
use sp_std::prelude::*;

#[derive(sp_std::fmt::Debug, Encode, Decode, PartialEq, Eq, Clone)]
/// Client state definition for the light client
pub struct ClientState {
    /// Latest beefy height
    pub latest_beefy_height: u32,
    /// Latest mmr root hash
    pub mmr_root_hash: H256,
    /// Authorities for the current session
    pub current_authorities: BeefyNextAuthoritySet<H256>,
    /// Authorities for the next session
    pub next_authorities: BeefyNextAuthoritySet<H256>,
    /// Beefy activation block
    pub beefy_activation_block: u32,
}

/// Host functions required by the light client for signature verification
pub trait HostFunctions {
    /// Keccak 256 hash function
    fn keccak_256(input: &[u8]) -> [u8; 32];
    /// Compressed Ecdsa public key recovery from a signature
    fn secp256k1_ecdsa_recover_compressed(
        signature: &[u8; 65],
        value: &[u8; 32],
    ) -> Option<Vec<u8>>;
}
