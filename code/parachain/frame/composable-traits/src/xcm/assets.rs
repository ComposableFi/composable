//! Interfaces to managed assets
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{dispatch::DispatchResult, pallet_prelude::ConstU32, WeakBoundedVec};
use polkadot_parachain::primitives::Id;
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_std::vec::Vec;
use xcm::latest::MultiLocation;

use crate::{
	assets::Asset,
	currency::{Exponent, Rational64},
};

/// works only with concrete assets
#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct XcmAssetLocation(
	#[cfg_attr(feature = "std", serde(with = "MultiLocationDef"))] pub xcm::latest::MultiLocation,
);

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
	fn asset_to_remote(
		asset_id: Self::AssetId,
	) -> Option<ForeignMetadata<Self::AssetNativeLocation>>;

	/// Return asset for given reserve location.
	fn location_to_asset(location: Self::AssetNativeLocation) -> Option<Self::AssetId>;

	/// if I want to send XCM message to `parachain_id` and pay with `remote_asset_id`,
	/// what minimal amount I should send as fee
	fn min_xcm_fee(
		parachain_id: Id,
		remote_asset_id: Self::AssetNativeLocation,
	) -> Option<Self::Balance>;

	// NOTE: can extend later to have fee per parachain, so if needed we can reduce `spam` from
	// other networks regardless of what they use for payments as of now any XCM message pays shared
	// common basic fee for sure fn min_xcm_native_in_fee(parachain_id: Id) ->
	// Option<Self::Balance>;

	/// Return information about foreign assets stored on assets registry
	fn get_foreign_assets_list() -> Vec<Asset<Self::AssetNativeLocation>>;
}

pub trait RemoteAssetRegistryMutate {
	type AssetId;
	type AssetNativeLocation;
	type Balance;

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
		ratio: Option<Rational64>,
		decimals: Option<Exponent>,
	) -> DispatchResult;

	/// allows change  ratio of how much remote assets is needed for unit of native
	fn update_ratio(
		location: Self::AssetNativeLocation,
		ratio: Option<Rational64>,
	) -> DispatchResult;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct ForeignMetadata<AssetNativeLocation> {
	pub decimals: Option<Exponent>,
	pub location: AssetNativeLocation,
}

#[cfg(feature = "std")]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Deserialize, Serialize)]
#[serde(remote = "xcm::latest::MultiLocation")]
pub struct MultiLocationDef {
	/// The number of parent junctions at the beginning of this `MultiLocation`.
	pub parents: u8,
	/// The interior (i.e. non-parent) junctions that this `MultiLocation` contains.
	#[serde(with = "JunctionsDef")]
	pub interior: xcm::latest::Junctions,
}

/// Non-parent junctions that can be constructed, up to the length of 8. This specific `Junctions`
/// implementation uses a Rust `enum` in order to make pattern matching easier.
///
/// Parent junctions cannot be constructed with this type. Refer to `MultiLocation` for
/// instructions on constructing parent junctions.
#[cfg(feature = "std")]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Deserialize, Serialize)]
#[serde(remote = "xcm::latest::Junctions")]
pub enum JunctionsDef {
	/// The interpreting consensus system.
	Here,
	/// A relative path comprising 1 junction.
	X1(#[serde(with = "JunctionDef")] xcm::latest::Junction),
	/// A relative path comprising 2 junctions.
	X2(
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
	),
	/// A relative path comprising 3 junctions.
	X3(
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
	),
	/// A relative path comprising 4 junctions.
	X4(
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
	),
	/// A relative path comprising 5 junctions.
	X5(
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
	),
	/// A relative path comprising 6 junctions.
	X6(
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
	),
	/// A relative path comprising 7 junctions.
	X7(
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
	),
	/// A relative path comprising 8 junctions.
	X8(
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
		#[serde(with = "JunctionDef")] xcm::latest::Junction,
	),
}

/// A single item in a path to describe the relative location of a consensus system.
///
/// Each item assumes a pre-existing location as its context and is defined in terms of it.
#[cfg(feature = "std")]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Deserialize, Serialize)]
#[serde(remote = "xcm::latest::Junction")]
pub enum JunctionDef {
	/// An indexed parachain belonging to and operated by the context.
	///
	/// Generally used when the context is a Polkadot Relay-chain.
	Parachain(u32),
	/// A 32-byte identifier for an account of a specific network that is respected as a sovereign
	/// endpoint within the context.
	///
	/// Generally used when the context is a Substrate-based chain.
	AccountId32 {
		#[serde(with = "NetworkIdDef")]
		network: xcm::latest::NetworkId,
		id: [u8; 32],
	},
	/// An 8-byte index for an account of a specific network that is respected as a sovereign
	/// endpoint within the context.
	///
	/// May be used when the context is a Frame-based chain and includes e.g. an indices pallet.
	AccountIndex64 {
		#[serde(with = "NetworkIdDef")]
		network: xcm::latest::NetworkId,
		index: u64,
	},
	/// A 20-byte identifier for an account of a specific network that is respected as a sovereign
	/// endpoint within the context.
	///
	/// May be used when the context is an Ethereum or Bitcoin chain or smart-contract.
	AccountKey20 {
		#[serde(with = "NetworkIdDef")]
		network: xcm::latest::NetworkId,
		key: [u8; 20],
	},
	/// An instanced, indexed pallet that forms a constituent part of the context.
	///
	/// Generally used when the context is a Frame-based chain.
	PalletInstance(u8),
	/// A nondescript index within the context location.
	///
	/// Usage will vary widely owing to its generality.
	///
	/// NOTE: Try to avoid using this and instead use a more specific item.
	GeneralIndex(u128),
	/// A nondescript datum acting as a key within the context location.
	///
	/// Usage will vary widely owing to its generality.
	///
	/// NOTE: Try to avoid using this and instead use a more specific item.
	GeneralKey(WeakBoundedVec<u8, ConstU32<32>>),
	/// The unambiguous child.
	///
	/// Not currently used except as a fallback when deriving ancestry.
	OnlyChild,
	/// A pluralistic body existing within consensus.
	///
	/// Typical to be used to represent a governance origin of a chain, but could in principle be
	/// used to represent things such as multisigs also.
	Plurality {
		#[serde(with = "BodyIdDef")]
		id: xcm::latest::BodyId,
		#[serde(with = "BodyPartDef")]
		part: xcm::latest::BodyPart,
	},
}

/// A global identifier of an account-bearing consensus system.
#[cfg(feature = "std")]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Deserialize, Serialize)]
#[serde(remote = "xcm::latest::NetworkId")]
pub enum NetworkIdDef {
	/// Unidentified/any.
	Any,
	/// Some named network.
	Named(WeakBoundedVec<u8, ConstU32<32>>),
	/// The Polkadot Relay chain
	Polkadot,
	/// Kusama.
	Kusama,
}

/// An identifier of a pluralistic body.
#[cfg(feature = "std")]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Deserialize, Serialize)]
#[serde(remote = "xcm::latest::BodyId")]
pub enum BodyIdDef {
	/// The only body in its context.
	Unit,
	/// A named body.
	Named(WeakBoundedVec<u8, ConstU32<32>>),
	/// An indexed body.
	Index(u32),
	/// The unambiguous executive body (for Polkadot, this would be the Polkadot council).
	Executive,
	/// The unambiguous technical body (for Polkadot, this would be the Technical Committee).
	Technical,
	/// The unambiguous legislative body (for Polkadot, this could be considered the opinion of a
	/// majority of lock-voters).
	Legislative,
	/// The unambiguous judicial body (this doesn't exist on Polkadot, but if it were to get a
	/// "grand oracle", it may be considered as that).
	Judicial,
}

/// A part of a pluralistic body.
#[cfg(feature = "std")]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Deserialize, Serialize)]
#[serde(remote = "xcm::latest::BodyPart")]
pub enum BodyPartDef {
	/// The body's declaration, under whatever means it decides.
	Voice,
	/// A given number of members of the body.
	Members { count: u32 },
	/// A given number of members of the body, out of some larger caucus.
	Fraction { nom: u32, denom: u32 },
	/// No less than the given proportion of members of the body.
	AtLeastProportion { nom: u32, denom: u32 },
	/// More than than the given proportion of members of the body.
	MoreThanProportion { nom: u32, denom: u32 },
}
