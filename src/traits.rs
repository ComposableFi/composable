use crate::error::BeefyClientError;
use crate::primitives::BeefyNextAuthoritySet;
use sp_core::H256;

pub trait StorageRead {
    fn latest_height() -> Result<u32, BeefyClientError>;
    fn latest_mmr_root_hash() -> Result<H256, BeefyClientError>;
    fn current_authority_set() -> Result<BeefyNextAuthoritySet<H256>, BeefyClientError>;
    fn next_authority_set() -> Result<BeefyNextAuthoritySet<H256>, BeefyClientError>;
}

pub trait StorageWrite {
    fn set_latest_height(height: u32) -> Result<(), BeefyClientError>;
    fn set_latest_mmr_root_hash(root_hash: H256) -> Result<(), BeefyClientError>;
    fn set_current_authority_set(
        authority_set: BeefyNextAuthoritySet<H256>,
    ) -> Result<(), BeefyClientError>;
    fn set_next_authority_set(
        authority_set: BeefyNextAuthoritySet<H256>,
    ) -> Result<(), BeefyClientError>;
}
