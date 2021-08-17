//! Traits on which this pallet relies

use crate::VaultIndex;
use frame_support::pallet_prelude::*;

pub trait Assets<AssetId, Balance, AccountId> {
    /// Creates new asset.
    ///
    /// # Implementors
    /// Implementors may use the vault_index, which is guaranteed to be unique and never seen
    /// before, but most likely should just use an internal incrementing counter.
    fn create(vault_index: VaultIndex) -> Result<AssetId, DispatchError>;
}
