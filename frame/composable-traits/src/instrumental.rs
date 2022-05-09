use frame_support::{
    pallet_prelude::*,
    sp_std::fmt::Debug,
};
use codec::Codec;
use sp_runtime::Perquintill;

#[derive(Clone, Copy, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
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

pub trait InstrumentalProtocolStrategy {
    type VaultId: Clone + Codec + Debug + PartialEq + Default + Parameter;
	type AssetId;

    fn associate_vault(vault_id: &Self::VaultId) -> Result<Self::VaultId, DispatchError>;
    
    // TODO: (Kevin)
    //  - can probably be a template method and call add_liquidity and remove_liquidity implementations
    fn rebalance() -> DispatchResult;
}