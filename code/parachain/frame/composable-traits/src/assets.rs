//! Interfaces to managed assets
use codec::{Decode, Encode, MaxEncodedLen};
use composable_support::collections::vec::bounded::BiBoundedVec;
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, DispatchResult, RuntimeDebug};
use sp_std::vec::Vec;

use crate::currency::{Exponent, Rational64};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

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
	type BoundedName;
	type BoundedSymbol;

	/// Sets the metadata of an asset
	fn set_metadata(
		asset_id: &Self::AssetId,
		name: Vec<u8>,
		symbol: Vec<u8>,
		decimals: u8,
	) -> DispatchResult;

	/// Update the metadata of an asset.
	///
	/// All metadata feilds are optional, only those provided as `Some` will be updated, the
	/// rest will be unchanged.
	fn update_metadata(
		asset_id: &Self::AssetId,
		name: Option<Vec<u8>>,
		symbol: Option<Vec<u8>>,
		decimals: Option<u8>,
	) -> DispatchResult;
}

/// Structure to represent basic asset metadata such as: name, symbol, decimals.
#[derive(Clone, Encode, Decode, Eq, PartialEq, Default, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct AssetMetadata<BoundedName, BoundedSymbol> {
	/// Name of the asset.
	pub name: BoundedName,
	/// Symbol of the asset.
	pub symbol: BoundedSymbol,
	/// The number of decimals this asset uses to represent one unit.
	pub decimals: u8,
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
	/// * `name` - Name of the asset
	/// * `symbol` - Symbol of the asset
	/// * `decimals` - The number of decimals this asset uses to represent one unit
	/// * `existential_deposit` - The minimum balance for an account to be stored on chain
	fn create_local_asset(
		protocol_id: [u8; 8],
		nonce: u64,
		name: Vec<u8>,
		symbol: Vec<u8>,
		decimals: u8,
		ratio: Option<Rational64>,
		existential_deposit: Self::Balance,
	) -> Result<Self::LocalAssetId, DispatchError>;

	/// Create a foreign asset
	///
	/// If `Ok`, returns the ID of the newly created asset.
	///
	/// # Parameters
	/// * `foreign_asset_id` - Foreign asset ID or relative location
	/// * `name` - Name of the asset
	/// * `symbol` - Symbol of the asset
	/// * `decimals` - The number of decimals this asset uses to represent one unit
	/// * `existential_deposit` - The minimum balance for an account to be stored on chain
	fn create_foreign_asset(
		foreign_asset_id: Self::ForeignAssetId,
		name: Vec<u8>,
		symbol: Vec<u8>,
		decimals: u8,
		ratio: Option<Rational64>,
		existential_deposit: Self::Balance,
	) -> Result<Self::LocalAssetId, DispatchError>;
}
