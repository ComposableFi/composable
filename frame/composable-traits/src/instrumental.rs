use frame_support::{
    pallet_prelude::*,
    sp_std::fmt::Debug,
};
use codec::Codec;

pub trait Instrumental {
    type AccountId: core::cmp::Ord;
	type AssetId;
	type Balance;
	type VaultId: Clone + Codec + Debug + PartialEq + Default + Parameter;

    fn create(
        asset: &Self::AssetId,
    ) -> Result<Self::VaultId, DispatchError>;

    fn add_liquidity(
        issuer: &Self::AccountId,
        asset: &Self::AssetId,
        amount: Self::Balance
    ) -> Result<(), DispatchError>;

    fn remove_liquidity(
        issuer: &Self::AccountId,
        asset: &Self::AssetId,
        amount: Self::Balance
    ) -> Result<(), DispatchError>;
}