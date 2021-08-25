use frame_support::pallet_prelude::*;
use sp_runtime::Perquintill;
use sp_std::collections::btree_map::BTreeMap;

#[derive(Encode, Decode, Default, Debug, PartialEq)]
pub struct VaultConfig<AccountId, CurrencyId>
where
    AccountId: core::cmp::Ord,
{
    pub asset_id: CurrencyId,
    pub reserved: Perquintill,
    pub strategies: BTreeMap<AccountId, Perquintill>,
}

#[derive(Copy, Clone, Encode, Decode, Default, Debug, PartialEq)]
pub struct VaultInfo<CurrencyId> {
    pub asset_id: CurrencyId,
    pub lp_token_id: CurrencyId,
}

#[derive(Copy, Clone, Encode, Decode, Default, Debug, PartialEq)]
pub struct StrategyOverview<Balance> {
    /// The reported balance of the strategy
    pub balance: Balance,
    /// Sum of all withdrawn funds.
    pub lifetime_withdrawn: Balance,
    /// Sum of all deposited funds.
    pub lifetime_deposited: Balance,
}
