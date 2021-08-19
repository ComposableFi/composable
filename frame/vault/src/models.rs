use frame_support::pallet_prelude::*;
use sp_runtime::Perquintill;
use sp_std::collections::btree_map::BTreeMap;

#[derive(Encode, Decode, Default, Debug, PartialEq)]
pub struct VaultConfig<AccountId, CurrencyId>
where
    AccountId: core::cmp::Ord + core::hash::Hash,
{
    pub asset_id: CurrencyId,
    pub reserved: Perquintill,
    pub strategies: BTreeMap<AccountId, Perquintill>,
}

#[derive(Encode, Decode, Default, Debug, PartialEq)]
pub struct Vault<CurrencyId, Balance> {
    pub asset_id: CurrencyId,
    pub lp_token_id: CurrencyId,
    pub assets_under_management: Balance,
}

#[derive(Encode, Decode, Default, Debug, PartialEq)]
pub struct StrategyOverview<Balance, Report> {
    pub withdrawn: Balance,
    pub report: Report,
}
