use super::*;
use common::{
	fees::{PriceConverter, WeightToFeeConverter},
	governance::native::{EnsureRootOrHalfNativeTechnical, NativeCouncilCollective},
	topology,
	xcmp::*,
};
use composable_traits::xcm::assets::{RemoteAssetRegistryInspect, XcmAssetLocation};
use cumulus_primitives_core::ParaId;
use frame_support::{
	log, parameter_types,
	traits::{Everything, Nothing, OriginTrait, PalletInfoAccess},
	weights::Weight,
};
use orml_traits::{
	location::{AbsoluteReserveProvider, RelativeReserveProvider, Reserve},
	parameter_type_with_key,
};
use orml_xcm_support::{
	DepositToAlternative, IsNativeConcrete, MultiCurrencyAdapter, MultiNativeAsset,
};
use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use sp_runtime::traits::{Convert, Zero};
use sp_std::{marker::PhantomData, prelude::*};
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
	AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom, BackingToPlurality, EnsureXcmOrigin,
	FixedWeightBounds, LocationInverter, ParentIsPreset, RelayChainAsNative,
	SiblingParachainAsNative, SiblingParachainConvertsVia, SignedAccountId32AsNative,
	SignedToAccountId32, SovereignSignedViaLocation, TakeRevenue, TakeWeightCredit,
};
use xcm_executor::{
	traits::{DropAssets, FilterAssetLocation},
	Assets, XcmExecutor,
};

parameter_types! {
	pub const RelayNetwork: NetworkId = NetworkId::Kusama;
	pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
	pub AssetsPalletLocation: MultiLocation =
		PalletInstance(<super::Assets as PalletInfoAccess>::index() as u8).into();
}

pub type Barrier = (
	AllowUnpaidExecutionFrom<ThisChain<ParachainInfo>>,
	AllowKnownQueryResponses<RelayerXcm>,
	AllowSubscriptionsFrom<ParentOrSiblings>,
	AllowTopLevelPaidExecutionFrom<Everything>,
	TakeWeightCredit,
);

pub type LocalOriginToLocation = SignedToAccountId32<Origin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, ()>,
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
	SovereignSignedViaLocation<LocationToAccountId, Origin>,
	// Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
	// recognized.
	RelayChainAsNative<RelayOrigin, Origin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognized.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, Origin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `Origin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, Origin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<Origin>,
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

pub struct RelayReserveFromParachain;
impl FilterAssetLocation for RelayReserveFromParachain {
	fn filter_asset_location(asset: &MultiAsset, origin: &MultiLocation) -> bool {
		// NOTE: In Acala there is not such thing
		// if asset is KSM and send from some parachain then allow for  that
		AbsoluteReserveProvider::reserve(asset) == Some(MultiLocation::parent()) &&
			matches!(origin, MultiLocation { parents: 1, interior: X1(Parachain(_)) })
	}
}

type IsReserveAssetLocationFilter =
	(MultiNativeAsset<AbsoluteReserveProvider>, RelayReserveFromParachain);

type AssetsIdConverter =
	CurrencyIdConvert<AssetsRegistry, CurrencyId, ParachainInfo, StaticAssetsMap>;

pub type Trader = TransactionFeePoolTrader<
	AssetsIdConverter,
	PriceConverter<AssetsRegistry>,
	ToTreasury<AssetsIdConverter, crate::Assets, TreasuryAccount>,
	WeightToFeeConverter,
>;

pub struct CaptureDropAssets<
	Treasury: TakeRevenue,
	PriceConverter,
	AssetConverter: Convert<MultiLocation, Option<CurrencyId>>,
>(PhantomData<(Treasury, PriceConverter, AssetConverter)>);

/// if asset  put  into Holding Registry of XCM VM, but did nothing to this
/// or if  too small to pay weight,
/// it will get here
/// if asset location and origin is known, put into treasury,  
/// else if asset location and origin not know, hash it until it will be added
impl<
		Treasury: TakeRevenue,
		PriceConverter,
		AssetConverter: Convert<MultiLocation, Option<CurrencyId>>,
	> DropAssets for CaptureDropAssets<Treasury, PriceConverter, AssetConverter>
{
	fn drop_assets(origin: &MultiLocation, assets: Assets) -> Weight {
		let multi_assets: Vec<MultiAsset> = assets.into();
		let mut can_return_on_request = vec![];
		log::info!(target : "xcmp", "drop_assets");
		let mut weight = Weight::zero();
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
	PriceConverter<AssetsRegistry>,
	AssetsIdConverter,
>;

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type Call = Call;
	type XcmSender = XcmRouter;
	type AssetTransactor = LocalAssetTransactor;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = IsReserveAssetLocationFilter;
	type IsTeleporter = (); // <- should be enough to allow teleportation of PICA
	type LocationInverter = LocationInverter<Ancestry>;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;

	type Trader = Trader;
	type ResponseHandler = RelayerXcm;

	type SubscriptionService = RelayerXcm;
	type AssetClaims = RelayerXcm;
	type AssetTrap = CaptureAssetTrap;
}

parameter_type_with_key! {
	pub ParachainMinFee: |location: MultiLocation| -> Option<Balance> {
		OutgoingFee::<AssetsRegistry>::outgoing_fee(location)
	};
}

impl orml_xtokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type CurrencyIdConvert = AssetsIdConverter;
	type AccountIdToMultiLocation = AccountIdToMultiLocation;
	type SelfLocation = topology::this::Local;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
	type BaseXcmWeight = BaseXcmWeight;
	type LocationInverter = LocationInverter<Ancestry>;
	type MaxAssetsForTransfer = XcmMaxAssetsForTransfer;
	type MinXcmFee = ParachainMinFee;
	type MultiLocationsFilter = Everything;
	type ReserveProvider = RelativeReserveProvider;
}

impl orml_unknown_tokens::Config for Runtime {
	type Event = Event;
}

// make setup as in Acala, max instructions seems reasonable, for weight may consider to  settle
// with our PICA
parameter_types! {
	// One XCM operation is 200_000_000 weight, cross-chain transfer ~= 2x of transfer.
	pub const UnitWeightCost: Weight = 200_000_000;
	pub const MaxInstructions: u32 = 100;
}

pub fn xcm_asset_fee_estimator(instructions: u8, asset_id: CurrencyId) -> Balance {
	assert!((instructions as u32) <= MaxInstructions::get());
	let total_weight = UnitWeightCost::get() * instructions as u64;
	Trader::weight_to_asset(total_weight, asset_id)
		.expect("use only in simulator")
		.1
}

pub fn xcm_fee_estimator(instructions: u8) -> Weight {
	assert!((instructions as u32) <= MaxInstructions::get());
	UnitWeightCost::get() * instructions as u64
}

parameter_types! {
	pub const CollectiveBodyId: BodyId = BodyId::Unit;
}

parameter_types! {
	pub const CouncilBodyId: BodyId = BodyId::Executive;
}

pub type CouncilToPlurality =
	BackingToPlurality<Origin, collective::Origin<Runtime, NativeCouncilCollective>, CouncilBodyId>;

pub struct RootToParachainMultiLocation<Origin, AccountId, Network>(
	PhantomData<(Origin, AccountId, Network)>,
);
impl<Origin: OriginTrait + Clone, AccountId: Into<[u8; 32]>, Network: Get<NetworkId>>
	xcm_executor::traits::Convert<Origin, MultiLocation>
	for RootToParachainMultiLocation<Origin, AccountId, Network>
where
	Origin::PalletsOrigin: From<system::RawOrigin<AccountId>>
		+ TryInto<system::RawOrigin<AccountId>, Error = Origin::PalletsOrigin>,
{
	fn convert(o: Origin) -> Result<MultiLocation, Origin> {
		o.try_with_caller(|caller| match caller.try_into() {
			Ok(system::RawOrigin::Root) =>
				Ok(Junction::Parachain(ParachainInfo::parachain_id().into()).into()),
			Ok(other) => Err(other.into()),
			Err(other) => Err(other),
		})
	}
}

pub type SendXcmOriginLocation =
	(CouncilToPlurality, RootToParachainMultiLocation<Origin, AccountId, RelayNetwork>);

impl pallet_xcm::Config for Runtime {
	type Event = Event;
	type SendXcmOrigin = EnsureXcmOrigin<Origin, SendXcmOriginLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
	type XcmExecuteFilter = Nothing;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Everything;
	type XcmReserveTransferFilter = Everything;
	type LocationInverter = LocationInverter<Ancestry>;
	type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
	type Origin = Origin;
	type Call = Call;

	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = VERSION_DISCOVERY_QUEUE_SIZE;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
}

impl cumulus_pallet_xcm::Config for Runtime {
	type Event = Event;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type Event = Event;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type VersionWrapper = RelayerXcm;
	type ChannelInfo = ParachainSystem;
	type ExecuteOverweightOrigin = EnsureRootOrHalfNativeCouncil;
	type ControllerOrigin = EnsureRootOrHalfNativeTechnical;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type WeightInfo = cumulus_pallet_xcmp_queue::weights::SubstrateWeight<Self>;
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
	type Event = Event;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = EnsureRootOrHalfNativeCouncil;
}
