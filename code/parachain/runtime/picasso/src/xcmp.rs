//! Setup of XCMP for parachain to allow cross chain transfers and other operations.
//! Very similar to https://github.com/galacticcouncil/Basilisk-node/blob/master/runtime/basilisk/src/xcm.rs
#![allow(unused_imports)] // allow until v2 xcm released (instead creating 2 runtimes)
use super::*; // recursive dependency onto runtime
use codec::{Decode, Encode};
use common::{xcmp::*, PriceConverter};
use composable_traits::{
	defi::Ratio,
	oracle::MinimalOracle,
	xcm::assets::{RemoteAssetRegistryInspect, XcmAssetLocation},
};
use cumulus_primitives_core::{IsSystem, ParaId};
use frame_support::{
	construct_runtime, ensure, log, parameter_types,
	traits::{
		Contains, Everything, KeyOwnerProofSystem, Nothing, OriginTrait, Randomness, StorageInfo,
	},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
		DispatchClass, IdentityFee, Weight, WeightToFeeCoefficient, WeightToFeeCoefficients,
		WeightToFeePolynomial,
	},
	PalletId, RuntimeDebug,
};
use orml_traits::{
	location::{AbsoluteReserveProvider, RelativeReserveProvider, Reserve},
	parameter_type_with_key, MultiCurrency,
};

use orml_xcm_support::{
	DepositToAlternative, IsNativeConcrete, MultiCurrencyAdapter, MultiNativeAsset, OnDepositFail,
};
use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use primitives::currency::WellKnownCurrency;
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
	traits::{AccountIdLookup, BlakeTwo256, Convert, ConvertInto, Zero},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, DispatchError,
};
use sp_std::marker::PhantomData;
use xcm::latest::{prelude::*, Error};
use xcm_builder::{
	AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
	AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom, EnsureXcmOrigin, FixedWeightBounds,
	LocationInverter, ParentIsPreset, RelayChainAsNative, SiblingParachainAsNative,
	SiblingParachainConvertsVia, SignedAccountId32AsNative, SignedToAccountId32,
	SovereignSignedViaLocation, TakeRevenue, TakeWeightCredit,
};
use xcm_executor::{
	traits::{
		ConvertOrigin, DropAssets, FilterAssetLocation, ShouldExecute, TransactAsset, WeightTrader,
	},
	Assets, Config, XcmExecutor,
};
parameter_types! {
	pub KsmLocation: MultiLocation = MultiLocation::parent();
	pub const RelayNetwork: NetworkId = NetworkId::Kusama;
	pub RelayOrigin: Origin = cumulus_pallet_xcm::Origin::Relay.into();
	pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
}

pub type Barrier = (
	XcmpDebug,
	//DebugAllowUnpaidExecutionFrom<WellKnownsChains>,
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

pub mod parachains {
	pub mod karura {
		pub const ID: u32 = 3000;
		pub const KUSD_KEY: &[u8] = &[0, 129];
	}
}

impl XcmpAssets for StaticAssetsMap {
	fn remote_to_local(location: MultiLocation) -> Option<CurrencyId> {
		match location {
			MultiLocation { parents: 1, interior: X2(Parachain(para_id), GeneralKey(key)) } =>
				match (para_id, &key[..]) {
					(parachains::karura::ID, parachains::karura::KUSD_KEY) =>
						Some(CurrencyId::kUSD),
					_ => None,
				},
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

			// if amount is not enough, it should be trapped by target chain or discarded as spam, so bear the risk
			// we use Acala's team XTokens which are opinionated - PANIC in case of zero
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
			Err(origin)
		}
	}
}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type Event = Event;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type VersionWrapper = RelayerXcm;
	type ChannelInfo = ParachainSystem;
	type ControllerOrigin = EnsureRootOrHalfNativeTechnical;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	// NOTE: we could consider allowance for some chains (see Acala tests ports  PRs)
	type ExecuteOverweightOrigin = EnsureRootOrHalfNativeCouncil;
	type WeightInfo = cumulus_pallet_xcmp_queue::weights::SubstrateWeight<Self>;
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
	type Event = Event;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = EnsureRootOrHalfNativeCouncil;
}
