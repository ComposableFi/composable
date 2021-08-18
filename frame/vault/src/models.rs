use frame_support::pallet_prelude::*;

#[derive(Encode, Decode, Default, Debug, PartialEq)]
pub struct VaultConfig<AssetId> {
    pub asset_id: AssetId,
}

#[derive(Encode, Decode, Default, Debug, PartialEq)]
pub struct Vault<AssetId, Balance> {
    pub(crate) config: VaultConfig<AssetId>,

    pub(crate) lp_token_id: AssetId,
    pub assets_under_management: Balance,
}
