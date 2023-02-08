use super::*;
use common::{fees::WeightToFeeConverter, xcmp::*};
use composable_traits::xcm::assets::{RemoteAssetRegistryInspect, XcmAssetLocation};
use cumulus_primitives_core::{IsSystem, ParaId};
use frame_support::{
	construct_runtime, log, match_types, parameter_types,
	traits::{
		Contains, Everything, KeyOwnerProofSystem, Nothing, OriginTrait, PalletInfoAccess,
		Randomness, StorageInfo,
	},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight},
		IdentityFee, Weight, WeightToFeeCoefficient, WeightToFeeCoefficients,
		WeightToFeePolynomial,
	},
	PalletId,
};
use orml_traits::{
	location::{AbsoluteReserveProvider, RelativeReserveProvider},
	parameter_type_with_key,
};

use orml_xcm_support::{
	DepositToAlternative, IsNativeConcrete, MultiCurrencyAdapter, MultiNativeAsset,
};
use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use sp_runtime::traits::Convert;
use sp_std::marker::PhantomData;
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
	AllowTopLevelPaidExecutionFrom, EnsureXcmOrigin, FixedWeightBounds, LocationInverter,
	ParentIsPreset, RelayChainAsNative, SiblingParachainAsNative, SiblingParachainConvertsVia,
	SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation, TakeRevenue,
	TakeWeightCredit,
};
use xcm_executor::{
	traits::{ConvertOrigin, DropAssets},
	Assets, XcmExecutor,
};

use crate::fees::FinalPriceConverter;

parameter_types! {
	pub KsmLocation: MultiLocation = MultiLocation::parent();
	pub const RelayNetwork: NetworkId = NetworkId::Kusama;
	pub RelayOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
	pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
}

impl orml_xtokens::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type CurrencyIdConvert = AssetsIdConverter;
	type AccountIdToMultiLocation = AccountIdToMultiLocation;
	type SelfLocation = ThisLocal;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type MinXcmFee = ParachainMinFee;
	type MultiLocationsFilter = Everything;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type BaseXcmWeight = BaseXcmWeight;
	type LocationInverter = LocationInverter<Ancestry>;
	type MaxAssetsForTransfer = XcmMaxAssetsForTransfer;
	type ReserveProvider = RelativeReserveProvider;
}

impl orml_unknown_tokens::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

pub type Barrier = (
	AllowKnownQueryResponses<RelayerXcm>,
	AllowSubscriptionsFrom<ParentOrSiblings>,
	AllowTopLevelPaidExecutionFrom<Everything>,
	TakeWeightCredit,
);

pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, RelayerXcm>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
);

/// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
	// The parent (Relay-chain) origin converts to the parent `AccountId`.
	ParentIsPreset<AccountId>,
	// Sibling parachain origins convert to AccountId via the `ParaId::into`.
	SiblingParachainConvertsVia<Sibling, AccountId>,
	// Straight up local `AccountId32` origins just alias directly to `AccountId`.
	AccountId32Aliases<RelayNetwork, AccountId>,
);

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
	// Sovereign account converter; this attempts to derive an `AccountId` from the origin location
	// using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
	// foreign chains who want to have a local sovereign account on this chain which they control.
	SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	// Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
	// recognized.
	RelayChainAsNative<RelayOrigin, RuntimeOrigin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognized.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `Origin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<RuntimeOrigin>,
);

pub struct StaticAssetsMap;
impl XcmpAssets for StaticAssetsMap {}

pub type LocalAssetTransactor = MultiCurrencyAdapter<
	crate::Assets,
	UnknownTokens,
	IsNativeConcrete<CurrencyId, AssetsIdConverter>,
	AccountId,
	LocationToAccountId,
	CurrencyId,
	AssetsIdConverter,
	DepositToAlternative<TreasuryAccount, Tokens, CurrencyId, AccountId, Balance>,
>;

type IsReserveAssetLocationFilter =
	(MultiNativeAsset<AbsoluteReserveProvider>, RelayReserveFromParachain);

pub struct SetupAfterRfc13;

impl RemoteAssetRegistryInspect for SetupAfterRfc13 {
	type AssetId = CurrencyId;

	type AssetNativeLocation = XcmAssetLocation;

	type Balance = Balance;

	fn asset_to_remote(
		_asset_id: Self::AssetId,
	) -> Option<composable_traits::xcm::assets::ForeignMetadata<Self::AssetNativeLocation>> {
		None
	}

	fn location_to_asset(_location: Self::AssetNativeLocation) -> Option<Self::AssetId> {
		None
	}

	fn min_xcm_fee(
		_parachain_id: ParaId,
		_remote_asset_id: Self::AssetNativeLocation,
	) -> Option<Self::Balance> {
		None
	}

	fn get_foreign_assets_list() -> Vec<Asset<Self::Balance, Self::AssetNativeLocation>> {
		Default::default()
	}
}

type AssetsIdConverter =
	CurrencyIdConvert<SetupAfterRfc13, CurrencyId, ParachainInfo, StaticAssetsMap>;

pub type Trader = TransactionFeePoolTrader<
	AssetsIdConverter,
	FinalPriceConverter,
	ToTreasury<AssetsIdConverter, crate::Assets, TreasuryAccount>,
	WeightToFeeConverter,
>;

pub struct CaptureDropAssets<
	Treasury: TakeRevenue,
	PriceConverter,
	AssetConverter: Convert<MultiLocation, Option<CurrencyId>>,
>(PhantomData<(Treasury, PriceConverter, AssetConverter)>);

impl<
		Treasury: TakeRevenue,
		PriceConverter,
		AssetConverter: Convert<MultiLocation, Option<CurrencyId>>,
	> DropAssets for CaptureDropAssets<Treasury, PriceConverter, AssetConverter>
{
	fn drop_assets(origin: &MultiLocation, assets: Assets) -> u64 {
		let multi_assets: Vec<MultiAsset> = assets.into();
		let mut can_return_on_request = vec![];
		log::info!(target : "xcmp", "drop_assets");
		let mut weight = 0_u64;
		for asset in multi_assets {
			if let MultiAsset { id: Concrete(location), fun: Fungible(_amount) } = asset.clone() {
				if let Some(_converted) = AssetConverter::convert(location) {
					Treasury::take_revenue(asset);
				} else {
					can_return_on_request.push(asset);
				}
			} else {
				can_return_on_request.push(asset);
			}
		}
		if !can_return_on_request.is_empty() {
			weight += RelayerXcm::drop_assets(origin, can_return_on_request.into());
		}
		weight
	}
}

pub type CaptureAssetTrap = CaptureDropAssets<
	ToTreasury<AssetsIdConverter, crate::Assets, TreasuryAccount>,
	FinalPriceConverter,
	AssetsIdConverter,
>;

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	type AssetTransactor = LocalAssetTransactor;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = IsReserveAssetLocationFilter;
	type IsTeleporter = ();
	type LocationInverter = LocationInverter<Ancestry>;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type Trader = Trader;
	type AssetTrap = CaptureAssetTrap;

	type ResponseHandler = RelayerXcm;
	type SubscriptionService = RelayerXcm;
	type AssetClaims = RelayerXcm;
}

parameter_type_with_key! {
	// 1. use configured pessimistic asset min fee for target chain / asset pair
	// 2. use built int
	// 3. allow to transfer anyway (let not lock assets on our chain for now)
	// until XCM v4
	pub ParachainMinFee: |location: MultiLocation| -> Option<Balance> {
		#[allow(clippy::match_ref_pats)] // false positive
		#[allow(clippy::match_single_binding)]
		let parents = location.parents;
		let interior = location.first_interior();

		if let Some(Parachain(_)) = interior {
			return None;
		}

		match (parents, interior) {
			(1, None) => Some(400_000),
			_ => None,
		}
	};
}

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, ()>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = Nothing;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Nothing;
	type XcmReserveTransferFilter = Everything;
	type LocationInverter = LocationInverter<Ancestry>;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;

	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = VERSION_DISCOVERY_QUEUE_SIZE;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
}

impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

pub struct SystemParachainAsSuperuser<Origin>(PhantomData<Origin>);
impl<Origin: OriginTrait> ConvertOrigin<Origin> for SystemParachainAsSuperuser<Origin> {
	fn convert_origin(
		origin: impl Into<MultiLocation>,
		kind: OriginKind,
	) -> Result<Origin, MultiLocation> {
		let origin = origin.into();
		if kind == OriginKind::Superuser &&
			matches!(
				origin,
				MultiLocation {
					parents: 1,
					interior: X1(Parachain(id)),
				} if ParaId::from(id).is_system(),
			) {
			Ok(Origin::root())
		} else {
			log::trace!(target: "xcmp::convert_origin", "failed to covert origin");
			Err(origin)
		}
	}
}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type VersionWrapper = RelayerXcm;
	type ChannelInfo = ParachainSystem;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type WeightInfo = cumulus_pallet_xcmp_queue::weights::SubstrateWeight<Self>;
	type ControllerOrigin = EnsureRoot<Self::AccountId>;
	type ExecuteOverweightOrigin = EnsureRoot<Self::AccountId>;
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = EnsureRoot<Self::AccountId>;
}

// #WIP: UnkonwTOkens, root only
