//! Interfaces to managed assets
use core::marker::PhantomData;

use codec::{Decode, Encode, MaxEncodedLen};
use composable_support::collections::vec::bounded::BiBoundedVec;
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, DispatchResult, RuntimeDebug};
use sp_std::vec::Vec;

use crate::currency::{Exponent, Rational64};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

pub const ASSET_METADATA_NAME_LENGTH: usize = 64;
pub const ASSET_METADATA_SYMBOL_LENGTH: usize = 16;

pub type BiBoundedAssetName = BiBoundedVec<u8, 1, ASSET_METADATA_NAME_LENGTH>;
pub type BiBoundedAssetSymbol = BiBoundedVec<u8, 1, ASSET_METADATA_SYMBOL_LENGTH>;

#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct BasicAssetMetadata {
	pub symbol: BiBoundedVec<u8, 1, 8>,
	pub name: BiBoundedVec<u8, 1, 32>,
}

impl BasicAssetMetadata {
	pub fn try_from(symbol: &[u8], name: &[u8]) -> Option<Self> {
		Some(Self {
			symbol: BiBoundedVec::try_from(symbol.to_vec()).ok()?,
			name: BiBoundedVec::try_from(name.to_vec()).ok()?,
		})
	}
}

#[derive(Decode, Encode, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Asset<Balance, ForeignId> {
	pub name: Option<Vec<u8>>,
	pub id: u128,
	pub decimals: Exponent,
	pub ratio: Option<Rational64>,
	pub foreign_id: Option<ForeignId>,
	pub existential_deposit: Balance,
}

/// Struct containing the information used to create an asset
#[derive(Decode, Encode, Debug, Clone, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AssetInfo<Balance> {
	/// Name of the asset.
	pub name: Option<BiBoundedAssetName>,
	/// Symbol of the asset.
	pub symbol: Option<BiBoundedAssetSymbol>,
	/// The number of decimals this asset uses to represent one unit.
	pub decimals: u8,
	/// The minimum balance of the asset for an account to be stored on chain.
	pub existential_deposit: Balance,
	/// The ratio of 1 native asset to 1 of this asset. Only used for BYOG assets. Set to `None` to
	/// prevent payment in this asset, only transferring.
	pub ratio: Option<Rational64>,
}

/// Stuct for updating the stored information for an asset.
///
/// All fields are wrapped by an `Option`. Only fiels with an outter `Some` should be updated.
#[derive(Decode, Encode, Debug, Clone, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AssetInfoUpdate<Balance> {
	/// Name of the asset.
	pub name: Option<Option<BiBoundedAssetName>>,
	/// Symbol of the asset.
	pub symbol: Option<Option<BiBoundedAssetSymbol>>,
	/// The number of decimals this asset uses to represent one unit.
	pub decimals: Option<u8>,
	/// The minimum balance of the asset for an account to be stored on chain.
	pub existential_deposit: Option<Balance>,
	/// The ratio of 1 native asset to 1 of this asset. Only used for BYOG assets. Set to
	/// `Some(None)` to prevent payment in this asset, only transferring.
	pub ratio: Option<Option<Rational64>>,
}

pub trait AssetTypeInspect {
	type AssetId;

	fn inspect(asset: &Self::AssetId) -> AssetType;
}

pub enum AssetType {
	Foreign,
	Local,
}
/// Routing of indepent parts of the `AssetMetadata` from `pallet-assets-registry`
pub trait InspectRegistryMetadata {
	type AssetId;

	/// Return the name of an asset.
	fn asset_name(asset_id: &Self::AssetId) -> Option<Vec<u8>>;
	/// Return the symbol of an asset.
	fn symbol(asset_id: &Self::AssetId) -> Option<Vec<u8>>;
	/// Return the decimals of an asset.
	fn decimals(asset_id: &Self::AssetId) -> Option<u8>;
}

pub trait MutateRegistryMetadata {
	type AssetId;

	/// Sets the metadata of an asset
	///
	/// `name` & `symbol` have an wrapping `Option`, setting this to `None`
	/// will set it to none in storage.
	fn set_metadata(
		asset_id: &Self::AssetId,
		name: Option<BiBoundedAssetName>,
		symbol: Option<BiBoundedAssetSymbol>,
		decimals: u8,
	) -> DispatchResult;

	/// Update the metadata of an asset.
	///
	/// All metadata feilds are optional, only those provided as `Some` will be updated, the
	/// rest will be unchanged. `name` & `symbol` have an inner `Option`, setting this to `None`
	/// will set it to none in storage.
	fn update_metadata(
		asset_id: &Self::AssetId,
		name: Option<Option<BiBoundedAssetName>>,
		symbol: Option<Option<BiBoundedAssetSymbol>>,
		decimals: Option<u8>,
	) -> DispatchResult;
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub enum LocalOrForeignAssetId<LocalAssetId, ForeignAssetId> {
	Local(LocalAssetId),
	Foreign(ForeignAssetId),
}

pub trait CreateAsset {
	type LocalAssetId;
	type ForeignAssetId;
	type Balance;

	/// Create a local asset
	///
	/// If `Ok`, returns the ID of the newly created asset.
	///
	/// # Parameters
	/// * `protocol_id` - The unique ID of the protocol that owns this asset (often a `PalletId`)
	/// * `nonce` - A nonce controlled by the owning protocol that uniquely identifies the asset in
	///   the scope of the protocol
	/// * `asset_info` - Structure containing relevant information to register the asset
	fn create_local_asset(
		protocol_id: [u8; 8],
		nonce: u64,
		asset_info: AssetInfo<Self::Balance>,
	) -> Result<Self::LocalAssetId, DispatchError>;

	/// Create a foreign asset
	///
	/// If `Ok`, returns the ID of the newly created asset.
	///
	/// # Parameters
	/// * `foreign_asset_id` - Foreign asset ID or relative location
	/// * `asset_info` - Structure containing relevant information to register the asset
	fn create_foreign_asset(
		foreign_asset_id: Self::ForeignAssetId,
		asset_info: AssetInfo<Self::Balance>,
	) -> Result<Self::LocalAssetId, DispatchError>;
}

/// Used temporarily while the Picasso runtime is migrated to the new Asset Transactor Router
pub struct DummyAssetCreator<LocalAssetId, ForeignAssetId, Balance> {
	_phantom_data: (PhantomData<LocalAssetId>, PhantomData<ForeignAssetId>, PhantomData<Balance>),
}

impl<LocalAssetId, ForeignAssetId, Balance> CreateAsset
	for DummyAssetCreator<LocalAssetId, ForeignAssetId, Balance>
{
	type LocalAssetId = LocalAssetId;
	type ForeignAssetId = ForeignAssetId;
	type Balance = Balance;

	fn create_local_asset(
		_protocol_id: [u8; 8],
		_nonce: u64,
		_asset_info: AssetInfo<Self::Balance>,
	) -> Result<Self::LocalAssetId, DispatchError> {
		Err(DispatchError::Other("TODO: Implement non-dummy in runtime!"))
	}

	fn create_foreign_asset(
		_foreign_asset_id: Self::ForeignAssetId,
		_asset_info: AssetInfo<Self::Balance>,
	) -> Result<Self::LocalAssetId, DispatchError> {
		Err(DispatchError::Other("TODO: Implement non-dummy in runtime!"))
	}
}
