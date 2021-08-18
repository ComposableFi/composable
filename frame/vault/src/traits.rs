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

    /// Transfers assets on behalf of `from`.
    ///
    /// # Implementors
    ///
    /// Implementations should decide on requiring an allowance mechanism if the calling account is
    /// not `from`. However that is not strictly necessary as pallets are not added to the runtime
    /// by untrusted sources.
    fn transfer_from(
        asset: &AssetId,
        from: &AccountId,
        to: &AccountId,
        amount: Balance,
    ) -> DispatchResult;

    fn mint_to(asset: &AssetId, to: &AccountId, amount: Balance) -> DispatchResult;

    /// Returns the total supply for a given asset.
    ///
    /// # Implementors
    /// `pallet-vaults` will always pass an existing `AssetId`, so return value should realistically
    /// never be `None`.
    fn total_supply(asset: &AssetId) -> Option<Balance>;
}
