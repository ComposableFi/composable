use frame_support::pallet_prelude::*;
use sp_runtime::Perquintill;
use sp_std::collections::btree_map::BTreeMap;

#[derive(Encode, Decode, Default, Debug, PartialEq)]
pub struct VaultConfig<AccountId, AssetId>
where
    AccountId: core::cmp::Ord + core::hash::Hash,
{
    pub asset_id: AssetId,
    pub reserved: Perquintill,
    pub strategies: BTreeMap<AccountId, Perquintill>,
}

#[derive(Encode, Decode, Default, Debug, PartialEq)]
pub struct Vault<AssetId, Balance> {
    pub asset_id: AssetId,
    pub lp_token_id: AssetId,
    pub assets_under_management: Balance,
}

#[derive(Encode, Decode, Default, Debug, PartialEq)]
pub struct StrategyOverview<Balance> {
    pub withdrawn: Balance,
}
