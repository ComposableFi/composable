use crate::error::BeefyClientError;
use crate::primitives::BeefyAuthoritySet;

pub trait StorageRead {
    fn latest_height() -> Result<u32, BeefyClientError>;
    fn latest_mmr_root_hash() -> Result<sp_core::H256, BeefyClientError>;
    fn current_authority_set() -> Result<BeefyAuthoritySet, BeefyClientError>;
    fn next_authority_set() -> Result<BeefyAuthoritySet, BeefyClientError>;
}

pub trait StorageWrite {
    fn set_latest_height(height: u32) -> Result<(), BeefyClientError>;
    fn set_latest_mmr_root_hash(root_hash: Vec<u8>) -> Result<(), BeefyClientError>;
    fn set_current_authority_set(authority_set: BeefyAuthoritySet) -> Result<(), BeefyClientError>;
    fn set_next_authority_set(authority_set: BeefyAuthoritySet) -> Result<(), BeefyClientError>;
}
