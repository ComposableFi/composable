use codec::Codec;
use frame_support::{pallet_prelude::*, sp_std::fmt::Debug};
use sp_runtime::Perquintill;

/// An indication of pool state. Shows whether the transfer of assets is currently taking place with
/// the current pool.
#[derive(Copy, Clone, Encode, Decode, Debug, PartialEq, MaxEncodedLen, TypeInfo)]
pub enum State {
	/// Indicates that there is currently no asset transfering going on for this asset
	/// and it can be initialized.
	Normal,
	/// Indicates that an asset is currently being transferred from one pool to another
	/// for this asset, so it is not possible to initialize a new transfer.
	Transferring,
}

/// An indication of access rights for admin accounts.
#[derive(Copy, Clone, Encode, Decode, Debug, PartialEq, MaxEncodedLen, TypeInfo)]
pub enum AccessRights {
	/// Account has full access rights.
	Full,
	/// Account has access only to `rebalance` function
	/// Account has access only to [`rebalance`](InstrumentalStrategy::rebalance())
	/// function.
	Rebalance,
	/// Account has access only to `set_pool_id_for_asset` function
	/// [`set_pool_id_for_asset`](InstrumentalStrategy::set_pool_id_for_asset()) function.
	SetPoolId,
	/// Account has access only to `add_vault_id` function
	/// [`associate_vault`](InstrumentalStrategy::associate_vault()) function.
	AssociateVaultId,
	/// Account has access only to `set_access` function
	/// [`associate_vault`](InstrumentalStrategy::associate_vault()) function.
	SetAccess,
}

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
		amount: Self::Balance,
	) -> DispatchResult;

	fn remove_liquidity(
		issuer: &Self::AccountId,
		asset: &Self::AssetId,
		amount: Self::Balance,
	) -> DispatchResult;
}

pub trait InstrumentalDynamicStrategy {
	type AccountId: core::cmp::Ord;
	type AssetId;

	fn get_optimum_strategy_for(asset: Self::AssetId) -> Result<Self::AccountId, DispatchError>;
}

pub trait InstrumentalStrategy {
	type AccountId: core::cmp::Ord;
	type AssetId;
	type VaultId: Clone + Codec + Debug + PartialEq + Default + Parameter;
	type PoolId;

	fn account_id() -> Self::AccountId;

	fn caller_has_rights(account_id: Self::AccountId, access: AccessRights) -> DispatchResult;

	fn associate_vault(vault_id: &Self::VaultId) -> Result<(), DispatchError>;

	fn rebalance() -> DispatchResult;

	fn get_apy(asset: Self::AssetId) -> Result<u128, DispatchError>;

	fn set_pool_id_for_asset(
		asset_id: Self::AssetId,
		pool_id: Self::PoolId,
	) -> Result<(), DispatchError>;

	fn transferring_funds_from_old_pool_to_new(
		asset_id: Self::AssetId,
		old_pool_id: Self::PoolId,
		new_pool_id: Self::PoolId,
	) -> DispatchResult;

	fn set_access(account: &Self::AccountId, access: AccessRights) -> DispatchResult;
}
