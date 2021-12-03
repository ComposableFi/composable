//! Interfaces to managed assets
use codec::{Decode, Encode};
use frame_support::dispatch::DispatchResult;
use scale_info::TypeInfo;
use xcm::latest::MultiLocation;

/// works only with concrete assets
#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, TypeInfo)]
//#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct XcmAssetLocation(pub xcm::latest::MultiLocation);

impl XcmAssetLocation {
	/// relay native asset
	pub const RELAY_NATIVE: XcmAssetLocation = XcmAssetLocation(MultiLocation::parent());

	/// local native, is equivalent to (1, LOCAL_PARACHAIN_ID), and to (1, LOCAL_PARACHAIN_ID, 1)
	/// and to (0, 1)
	pub const LOCAL_NATIVE: XcmAssetLocation = XcmAssetLocation(MultiLocation::here());
}

impl Default for XcmAssetLocation {
	fn default() -> Self {
		XcmAssetLocation(xcm::latest::MultiLocation::here())
	}
}

impl From<XcmAssetLocation> for xcm::latest::MultiLocation {
	fn from(this: XcmAssetLocation) -> Self {
		this.0
	}
}

impl XcmAssetLocation {
	pub fn new(multi_location: xcm::latest::MultiLocation) -> Self {
		Self(multi_location)
	}
}

pub trait RemoteAssetRegistry {
	/// Local asset id.
	/// Each implemented of this trait must hedge common id space for well known local assets
	/// initialized via genesis config.
	type AssetId;

	/// Pointer to asset location relative to here.
	/// Imagine imagine path NativeAsset<->UNI<->ETH<->Relayer<->Picasso<->Self, it will look like
	/// for local asset id  as (1 level up to runtime, relayer id, down to ETH consensus, down to
	/// UNI contract, Native Asset of UNI). So final interpreting consensus will work with asset ids
	/// as it is local for itself. So from any local asset of runtime, can find any known asset on
	/// any connected network. Other schemas like XCM with only one parent or libp2p multiadress OR
	/// IBC like address each pallet working with foreign remote assets should specific proper impl
	/// for this
	type AssetNativeLocation;

	/// Set asset native location.
	///
	/// Adds mapping between native location and local asset id and vice versa.
	///
	/// Emits `LocationSet` event when successful.
	/// `asset_id` - local id
	/// `location` - remote location relative to this chain
	fn set_location(asset_id: Self::AssetId, location: Self::AssetNativeLocation)
		-> DispatchResult;

	/// Return location for given asset.
	fn asset_to_location(asset_id: Self::AssetId) -> Option<Self::AssetNativeLocation>;

	/// Return asset for given location.
	fn location_to_asset(location: Self::AssetNativeLocation) -> Option<Self::AssetId>;
}
