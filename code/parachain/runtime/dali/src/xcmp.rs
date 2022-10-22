//! Setup of XCMP for parachain to allow cross chain transfers and other operations.
//! Very similar to https://github.com/galacticcouncil/Basilisk-node/blob/master/runtime/basilisk/src/xcm.rs
use super::*;
use common::{
	governance::native::EnsureRootOrHalfNativeTechnical, topology, xcmp::*, PriceConverter,
};
use composable_traits::{
	oracle::MinimalOracle,
	xcm::assets::{RemoteAssetRegistryInspect, XcmAssetLocation},
};
use cumulus_primitives_core::ParaId;
use frame_support::{
	log, parameter_types,
	traits::{Everything, Nothing},
	weights::Weight,
	WeakBoundedVec,
};
use orml_traits::{
	location::{AbsoluteReserveProvider, Reserve},
	parameter_type_with_key,
};
use orml_xcm_support::{
	DepositToAlternative, IsNativeConcrete, MultiCurrencyAdapter, MultiNativeAsset,
};
use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use primitives::currency::WellKnownCurrency;
use sp_runtime::traits::{Convert, Zero};
use sp_std::{marker::PhantomData, prelude::*};
use xcm::{latest::prelude::*, v1::Junction::PalletInstance};
use xcm_builder::{
	AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
	AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom, EnsureXcmOrigin, FixedWeightBounds,
	LocationInverter, ParentIsPreset, RelayChainAsNative, SiblingParachainAsNative,
	SiblingParachainConvertsVia, SignedAccountId32AsNative, SignedToAccountId32,
	SovereignSignedViaLocation, TakeRevenue, TakeWeightCredit,
};
use xcm_executor::{
	traits::{DropAssets, FilterAssetLocation},
	Assets, XcmExecutor,
};

parameter_types! {
	pub KsmLocation: MultiLocation = MultiLocation::parent();
	pub const RelayNetwork: NetworkId = NetworkId::Kusama;
	pub RelayOrigin: Origin = cumulus_pallet_xcm::Origin::Relay.into();
	pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
}

pub type Barrier = (
	XcmpDebug,
	AllowUnpaidExecutionFrom<ThisChain<ParachainInfo>>,
	// Expected responses are OK.
	AllowKnownQueryResponses<RelayerXcm>,
	// Subscriptions for version tracking are OK.
	AllowSubscriptionsFrom<Everything>,
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
	// Superuser converter for the Relay-chain (Parent) location. This will allow it to issue a
	// transaction from the Root origin.
	xcm_builder::ParentAsSuperuser<Origin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `Origin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, Origin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<Origin>,
);

pub struct StaticAssetsMap;

impl XcmpAssets for StaticAssetsMap {
	fn remote_to_local(location: MultiLocation) -> Option<CurrencyId> {
		match location {
			MultiLocation { parents: 1, interior: X2(Parachain(para_id), GeneralKey(key)) } =>
				match (para_id, &key[..]) {
					(topology::karura::ID, topology::karura::KUSD_KEY) => Some(CurrencyId::kUSD),
					_ => None,
				},
			MultiLocation {
				parents: 1,
				interior: X3(Parachain(para_id), PalletInstance(pallet_instance), GeneralIndex(key)),
			} => match (para_id, pallet_instance, key) {
				(
					topology::common_good_assets::ID,
					topology::statemine::ASSETS,
					topology::statemine::USDT,
				) => Some(CurrencyId::USDT),
				_ => None,
			},
			_ => None,
		}
	}

	fn local_to_remote(id: CurrencyId, this_para_id: u32) -> Option<MultiLocation> {
		match id {
			CurrencyId::NATIVE => Some(MultiLocation::new(
				1,
				X2(
					Parachain(this_para_id),
					GeneralKey(
						frame_support::storage::weak_bounded_vec::WeakBoundedVec::force_from(
							id.encode(),
							None,
						),
					),
				),
			)),
			CurrencyId::RELAY_NATIVE => Some(MultiLocation::parent()),
			CurrencyId::kUSD => Some(MultiLocation {
				parents: 1,
				interior: X2(
					Parachain(topology::karura::ID),
					GeneralKey(WeakBoundedVec::force_from(
						topology::karura::KUSD_KEY.to_vec(),
						None,
					)),
				),
			}),
			CurrencyId::USDT => Some(MultiLocation {
				parents: 1,
				interior: X3(
					Parachain(topology::common_good_assets::ID),
					PalletInstance(topology::common_good_assets::ASSETS),
					GeneralIndex(topology::common_good_assets::USDT),
				),
			}),

			_ => None,
		}
	}
}

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
	(DebugMultiNativeAsset, MultiNativeAsset<AbsoluteReserveProvider>, RelayReserveFromParachain);

type AssetsIdConverter =
	CurrencyIdConvert<AssetsRegistry, CurrencyId, ParachainInfo, StaticAssetsMap>;

pub type Trader = TransactionFeePoolTrader<
	AssetsIdConverter,
	PriceConverter<AssetsRegistry>,
	ToTreasury<AssetsIdConverter, crate::Assets, TreasuryAccount>,
	WeightToFee,
>;

pub struct CaptureDropAssets<
	Treasury: TakeRevenue,
	PriceConverter: MinimalOracle,
	AssetConverter: Convert<MultiLocation, Option<CurrencyId>>,
>(PhantomData<(Treasury, PriceConverter, AssetConverter)>);

/// if asset  put  into Holding Registry of XCM VM, but did nothing to this
/// or if  too small to pay weight,
/// it will get here
/// if asset location and origin is known, put into treasury,  
/// else if asset location and origin not know, hash it until it will be added
impl<
		Treasury: TakeRevenue,
		PriceConverter: MinimalOracle,
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

parameter_types! {
	pub SelfLocation: MultiLocation = MultiLocation::new(1, X1(Parachain(ParachainInfo::parachain_id().into())));
}

parameter_type_with_key! {
	pub ParachainMinFee: |location: MultiLocation| -> Option<Balance> {
		#[allow(clippy::match_ref_pats)] // false positive
		#[allow(clippy::match_single_binding)]
		match (location.parents, location.first_interior()) {
			// relay KSM
			(1, None) => Some(400_000_000_000),
			(1, Some(Parachain(id)))  =>  {
				let location = XcmAssetLocation::new(location.clone());
				AssetsRegistry::min_xcm_fee(ParaId::from(*id), location).or(Some(u128::MAX))
			},
			_ => Some(u128::MAX),
		}
	};
}

impl orml_xtokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type CurrencyIdConvert = AssetsIdConverter;
	type AccountIdToMultiLocation = AccountIdToMultiLocation;
	type SelfLocation = SelfLocation;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
	type BaseXcmWeight = BaseXcmWeight;
	type LocationInverter = LocationInverter<Ancestry>;
	type MaxAssetsForTransfer = XcmMaxAssetsForTransfer;
	type MinXcmFee = ParachainMinFee;
	type MultiLocationsFilter = Everything;
	type ReserveProvider = AbsoluteReserveProvider;
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

impl pallet_xcm::Config for Runtime {
	type Event = Event;
	type SendXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
	/// https://medium.com/kusama-network/kusamas-governance-thwarts-would-be-attacker-9023180f6fb
	type XcmExecuteFilter = Nothing;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Everything;
	type XcmReserveTransferFilter = Everything;
	type LocationInverter = LocationInverter<Ancestry>;
	type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
	type Origin = Origin;
	type Call = Call;

	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
}

/// cumulus is default implementation  of queue integrated with polkadot and kusama runtimes
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
