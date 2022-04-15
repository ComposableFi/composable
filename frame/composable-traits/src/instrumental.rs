use frame_support::{
    pallet_prelude::*,
    sp_std::fmt::Debug,
};
use codec::Codec;
use sp_runtime::Perquintill;

#[derive(Clone, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
pub struct InstrumentalVaultConfig<AssetId, Percent> {
    pub asset_id: AssetId,
    pub percent_deployable: Percent,
}

pub trait Instrumental {
    type AccountId: core::cmp::Ord;
	type AssetId;
	type Balance;
	type VaultId: Clone + Codec + Debug + PartialEq + Default + Parameter;

    fn account_id() -> Self::AccountId;

    fn create(
        config: InstrumentalVaultConfig<Self::AssetId, Perquintill>,
    ) -> Result<Self::VaultId, DispatchError>;

    fn add_liquidity(
        issuer: &Self::AccountId,
        asset: &Self::AssetId,
        amount: Self::Balance
    ) -> DispatchResult;

    fn remove_liquidity(
        issuer: &Self::AccountId,
        asset: &Self::AssetId,
        amount: Self::Balance
    ) -> DispatchResult;
}