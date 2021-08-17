use frame_support::pallet_prelude::*;

#[derive(Encode, Decode, Default, Debug, PartialEq)]
pub struct VaultConfig {}

#[derive(Encode, Decode, Default, Debug, PartialEq)]
pub struct Vault<AssetId> {
    pub(crate) config: VaultConfig,
    pub(crate) lp_token_id: AssetId,
}
