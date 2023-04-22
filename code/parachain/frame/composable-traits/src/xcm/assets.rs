//! Interfaces to managed assets
use crate::assets::{AssetInfo, AssetInfoUpdate};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::dispatch::DispatchResult;
use scale_info::TypeInfo;
use sp_std::vec::Vec;

use crate::{assets::Asset, currency::Exponent};

pub trait RemoteAssetRegistryInspect {
	/// Local asset id.
	/// Each implemented of this trait must hedge common id space for well known local assets
	/// initialized via genesis config.
	type AssetId;

	/// Pointer to asset location relative to here.
	/// Imagine imagine path NativeAsset<->UNI<->ETH<->Relayer<->Picasso<->Self, it will look like
	/// for local asset id  as (1 level up to runtime, relayer id, down to ETH consensus, down to
	/// UNI contract, Native Asset of UNI). So final interpreting consensus will work with asset ids
	/// as it is local for itself. So from any local asset of runtime, can find any known asset on
	/// any connected network. Other schemas like XCM with only one parent or libp2p multiaddress OR
	/// IBC like address each pallet working with foreign remote assets should specific proper impl
	/// for this
	type AssetNativeLocation;

	type Balance;

	/// Return reserve location for given asset.
	fn asset_to_remote(asset_id: Self::AssetId) -> Option<Self::AssetNativeLocation>;

	/// Return asset for given reserve location.
	fn location_to_asset(location: Self::AssetNativeLocation) -> Option<Self::AssetId>;

	/// if I want to send XCM message to `parachain_id` and pay with `remote_asset_id`,
	/// what minimal amount I should send as fee
	fn min_xcm_fee(
		parachain_id: u32,
		remote_asset_id: Self::AssetNativeLocation,
	) -> Option<Self::Balance>;

	// NOTE: can extend later to have fee per parachain, so if needed we can reduce `spam` from
	// other networks regardless of what they use for payments as of now any XCM message pays shared
	// common basic fee for sure fn min_xcm_native_in_fee(parachain_id: Id) ->
	// Option<Self::Balance>;

	/// Return information about foreign assets stored on assets registry
	fn get_foreign_assets_list() -> Vec<Asset<Self::Balance, Self::AssetNativeLocation>>;
}

pub trait RemoteAssetRegistryMutate {
	type AssetId;
	type AssetNativeLocation;
	type Balance;

	fn register_asset(
		asset_id: Self::AssetId,
		location: Option<Self::AssetNativeLocation>,
		asset_info: AssetInfo<Self::Balance>,
	) -> DispatchResult;

	fn update_asset(
		asset_id: Self::AssetId,
		asset_info: AssetInfoUpdate<Self::Balance>,
	) -> DispatchResult;

	/// Set asset native location.
	///
	/// Adds mapping between native location and local asset id and vice versa.
	/// It is assumed that it is possible to use origin as chain who holds reserve of tokens.
	///
	/// Inputs:
	/// `asset_id` local asset id created using `CurrencyFactory`
	/// `location` - remote location
	/// `ed` - minimal amount of registered asset allowed to form account
	/// `ratio` - of native asset to remote; amount of foreign asset multiplied by ratio will give
	/// equivalent amount of native; `decimals` - if asset decimals is not 12, than value must be
	/// provided Emits `LocationSet` event when successful.
	/// `asset_id` - local asset id create via `CurrencyFactory`
	/// `location` - remote location relative to this chain
	fn set_reserve_location(
		asset_id: Self::AssetId,
		location: Self::AssetNativeLocation,
	) -> DispatchResult;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct ForeignMetadata<AssetNativeLocation> {
	pub decimals: Option<Exponent>,
	pub location: AssetNativeLocation,
}
