use crate::fees::FinalPriceConverter;

use super::*;
use common::{fees::WeightToFeeConverter, xcmp::*};
use composable_traits::xcm::assets::RemoteAssetRegistryInspect;
use cumulus_primitives_core::{IsSystem, ParaId};
use frame_support::{
	log, parameter_types,
	traits::{Everything, Nothing, OriginTrait, ProcessMessageError},
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
use primitives::currency::{ForeignAssetId, VersionedMultiLocation};
use sp_runtime::traits::Convert;
use sp_std::marker::PhantomData;
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
	AllowTopLevelPaidExecutionFrom, EnsureXcmOrigin, FixedWeightBounds, ParentIsPreset,
	RelayChainAsNative, SiblingParachainAsNative, SiblingParachainConvertsVia,
	SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation, TakeRevenue,
	TakeWeightCredit, WithComputedOrigin,
};
use xcm_executor::{
	traits::{ConvertOrigin, DropAssets, MatchesFungible, ShouldExecute},
	Assets, XcmExecutor,
};

#[cfg(feature = "testnet")]
parameter_types! {
	pub const RelayNetwork: NetworkId = NetworkId::Rococo;
}

#[cfg(not(feature = "testnet"))]
parameter_types! {
	pub const RelayNetwork: NetworkId = NetworkId::Kusama;
}

parameter_types! {
	pub RelayOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
	pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
	pub UniversalLocation: InteriorMultiLocation = X2(GlobalConsensus(RelayNetwork::get()), Parachain(ParachainInfo::parachain_id().into()));
}

pub struct AllowKnownQueryResponsesSubstituteQuerierIfNone<ResponseHandler>(
	PhantomData<ResponseHandler>,
);
impl<ResponseHandler: OnResponse> ShouldExecute
	for AllowKnownQueryResponsesSubstituteQuerierIfNone<ResponseHandler>
{
	fn should_execute<RuntimeCall>(
		origin: &MultiLocation,
		instructions: &mut [Instruction<RuntimeCall>],
		max_weight: Weight,
		weight_credit: &mut Weight,
	) -> Result<(), ProcessMessageError> {
		for i in instructions.iter_mut() {
			if let QueryResponse { ref mut querier, .. } = i {
				if querier.is_none() {
					//need this line because querier is None
					//and here is where it failed in pallet-xcm
					//https://github.com/paritytech/polkadot/blob/release-v0.9.43/xcm/pallet-xcm/src/lib.rs#L2001-L2010
					//so need to substitute it with expected querier
					*querier = Some(MultiLocation { parents: 0, interior: Here });
				}
			}
		}

		AllowKnownQueryResponses::<ResponseHandler>::should_execute(
			origin,
			instructions,
			max_weight,
			weight_credit,
		)?;
		Ok(())
	}
}

pub type Barrier = (
	AllowKnownQueryResponsesSubstituteQuerierIfNone<PolkadotXcm>,
	AllowSubscriptionsFrom<ParentOrSiblings>,
	AllowTopLevelPaidExecutionFrom<Everything>,
	TakeWeightCredit,
	WithComputedOrigin<
		AllowTopLevelPaidExecutionFrom<orml_xcm_builder_kusama::TinkernetMultisigMultiLocation>,
		UniversalLocation,
		ConstU32<8>,
	>,
);

pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm, ()>,
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
	// Mapping Tinkernet multisig to the correctly derived AccountId32.
	orml_xcm_builder_kusama::TinkernetMultisigAsAccountId<AccountId>,
	AccountId32MultihopTx<AccountId>,
);

pub struct AccountId32MultihopTx<AccountId>(PhantomData<AccountId>);
impl<AccountId: From<[u8; 32]> + Into<[u8; 32]> + Clone>
	xcm_executor::traits::Convert<MultiLocation, AccountId> for AccountId32MultihopTx<AccountId>
{
	fn convert(location: MultiLocation) -> Result<AccountId, MultiLocation> {
		let id = match location {
			MultiLocation {
				parents: 0,
				interior:
					X4(
						PalletInstance(_),
						GeneralIndex(_),
						AccountId32 { id, network: _ },
						AccountId32 { id: _, network: _ },
					),
			} => id,
			MultiLocation {
				parents: 0,
				interior:
					X5(
						PalletInstance(_),
						GeneralIndex(_),
						AccountId32 { id, network: _ },
						AccountId32 { id: _, network: _ },
						AccountId32 { id: _, network: _ },
					),
			} => id,
			MultiLocation {
				parents: 0,
				interior:
					X6(
						PalletInstance(_),
						GeneralIndex(_),
						AccountId32 { id, network: _ },
						AccountId32 { id: _, network: _ },
						AccountId32 { id: _, network: _ },
						AccountId32 { id: _, network: _ },
					),
			} => id,
			MultiLocation {
				parents: 0,
				interior:
					X7(
						PalletInstance(_),
						GeneralIndex(_),
						AccountId32 { id, network: _ },
						AccountId32 { id: _, network: _ },
						AccountId32 { id: _, network: _ },
						AccountId32 { id: _, network: _ },
						AccountId32 { id: _, network: _ },
					),
			} => id,
			MultiLocation {
				parents: 0,
				interior:
					X8(
						PalletInstance(_),
						GeneralIndex(_),
						AccountId32 { id, network: _ },
						AccountId32 { id: _, network: _ },
						AccountId32 { id: _, network: _ },
						AccountId32 { id: _, network: _ },
						AccountId32 { id: _, network: _ },
						AccountId32 { id: _, network: _ },
					),
			} => id,
			_ => return Err(location),
		};

		Ok(id.into())
	}

	fn reverse(who: AccountId) -> Result<MultiLocation, AccountId> {
		Err(who)
	}
}

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
	orml_xcm_builder_kusama::TinkernetMultisigAsNativeOrigin<RuntimeOrigin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<RuntimeOrigin>,
);

pub type LocalAssetTransactor = MultiCurrencyAdapterWrapper<
	crate::Assets,
	UnknownTokens,
	IsNativeConcrete<CurrencyId, AssetsIdConverter>,
	AccountId,
	LocationToAccountId,
	CurrencyId,
	AssetsIdConverter,
	DepositToAlternative<TreasuryAccount, Tokens, CurrencyId, AccountId, Balance>,
	PalletMultihopXcmIbc,
>;

pub struct MultiCurrencyAdapterWrapper<
	MultiCurrency,
	UnknownAsset,
	Match,
	AccountId,
	AccountIdConvert,
	CurrencyId,
	CurrencyIdConvert,
	DepositFailureHandler,
	MultiCurrencyCallback,
>(
	PhantomData<(
		MultiCurrency,
		UnknownAsset,
		Match,
		AccountId,
		AccountIdConvert,
		CurrencyId,
		CurrencyIdConvert,
		DepositFailureHandler,
		MultiCurrencyCallback,
	)>,
);

impl<
		MultiCurrency: orml_traits::MultiCurrency<AccountId, CurrencyId = CurrencyId>,
		UnknownAsset: orml_xcm_support::UnknownAsset,
		Match: MatchesFungible<MultiCurrency::Balance>,
		AccountId: sp_std::fmt::Debug + Clone,
		AccountIdConvert: xcm_executor::traits::Convert<MultiLocation, AccountId>,
		CurrencyIdConvert: Convert<MultiAsset, Option<CurrencyId>>,
		DepositFailureHandler: orml_xcm_support::OnDepositFail<CurrencyId, AccountId, MultiCurrency::Balance>,
		DepositCallback: composable_traits::xcm::assets::MultiCurrencyCallback<AssetId = CurrencyId>,
	> xcm_executor::traits::TransactAsset
	for MultiCurrencyAdapterWrapper<
		MultiCurrency,
		UnknownAsset,
		Match,
		AccountId,
		AccountIdConvert,
		CurrencyId,
		CurrencyIdConvert,
		DepositFailureHandler,
		DepositCallback,
	>
{
	fn deposit_asset(
		asset: &MultiAsset,
		location: &MultiLocation,
		context: &XcmContext,
	) -> xcm::v3::Result {
		let result = MultiCurrencyAdapter::<
			MultiCurrency,
			UnknownAsset,
			Match,
			AccountId,
			AccountIdConvert,
			CurrencyId,
			CurrencyIdConvert,
			DepositFailureHandler,
		>::deposit_asset(asset, location, context);
		// let currency_id = CurrencyIdConvert::convert(asset.clone());
		match (
			AccountIdConvert::convert_ref(location),
			CurrencyIdConvert::convert(asset.clone()),
			Match::matches_fungible(asset),
		) {
			// known asset
			(Ok(_), Some(currency_id), Some(_)) => {
				let _ =
					DepositCallback::deposit_asset(asset, location, context, result, currency_id);
			},
			// unknown asset
			_ => {
				frame_support::log::error!(
					target: "xcmp",
					"deposit_asset failed: {:?} {:?} {:?}",
					AccountIdConvert::convert_ref(location),
					CurrencyIdConvert::convert(asset.clone()),
					Match::matches_fungible(asset),
				);
			},
		}
		result
	}

	fn withdraw_asset(
		asset: &MultiAsset,
		location: &MultiLocation,
		maybe_context: Option<&XcmContext>,
	) -> sp_std::result::Result<Assets, XcmError> {
		MultiCurrencyAdapter::<
			MultiCurrency,
			UnknownAsset,
			Match,
			AccountId,
			AccountIdConvert,
			CurrencyId,
			CurrencyIdConvert,
			DepositFailureHandler,
		>::withdraw_asset(asset, location, maybe_context)
	}

	fn transfer_asset(
		asset: &MultiAsset,
		from: &MultiLocation,
		to: &MultiLocation,
		context: &XcmContext,
	) -> sp_std::result::Result<Assets, XcmError> {
		MultiCurrencyAdapter::<
			MultiCurrency,
			UnknownAsset,
			Match,
			AccountId,
			AccountIdConvert,
			CurrencyId,
			CurrencyIdConvert,
			DepositFailureHandler,
		>::transfer_asset(asset, from, to, context)
	}
}

pub struct ForeignAssetsToXcm;

impl Convert<CurrencyId, Option<MultiLocation>> for ForeignAssetsToXcm {
	fn convert(a: CurrencyId) -> Option<MultiLocation> {
		match AssetsRegistry::asset_to_remote(a) {
			// XCMed assets
			Some(ForeignAssetId::Xcm(VersionedMultiLocation::V3(xcm))) => Some(xcm),
			// IBCed assets
			_ => Some(MultiLocation {
				parents: 0,
				interior: X2(
					PalletInstance(<AssetsRegistry as PalletInfoAccess>::index() as u8),
					GeneralIndex(a.into()),
				),
			}),
		}
	}
}

impl Convert<MultiLocation, Option<CurrencyId>> for ForeignAssetsToXcm {
	fn convert(a: MultiLocation) -> Option<CurrencyId> {
		AssetsRegistry::location_to_asset(ForeignAssetId::Xcm(VersionedMultiLocation::V3(a)))
	}
}

type AssetsIdConverter = CurrencyIdConvert<
	ForeignAssetsToXcm,
	primitives::topology::Picasso,
	AssetsRegistry,
	ParachainInfo,
>;

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
	fn drop_assets(origin: &MultiLocation, assets: Assets, context: &XcmContext) -> Weight {
		let multi_assets: Vec<MultiAsset> = assets.into();
		let mut can_return_on_request = vec![];
		log::info!(target : "xcmp", "drop_assets");
		let mut weight = <_>::default();
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
			weight += PolkadotXcm::drop_assets(origin, can_return_on_request.into(), context);
		}
		weight
	}
}

pub type CaptureAssetTrap = CaptureDropAssets<
	ToTreasury<AssetsIdConverter, crate::Assets, TreasuryAccount>,
	FinalPriceConverter,
	AssetsIdConverter,
>;

use xcm_executor::traits::OnResponse;
pub struct XcmExecutorHandler;
impl OnResponse for XcmExecutorHandler {
	fn expecting_response(
		origin: &MultiLocation,
		query_id: u64,
		querier: Option<&MultiLocation>,
	) -> bool {
		PolkadotXcm::expecting_response(origin, query_id, querier)
	}
	/// Handler for receiving a `response` from `origin` relating to `query_id` initiated by
	/// `querier`.
	fn on_response(
		origin: &MultiLocation,
		query_id: u64,
		querier: Option<&MultiLocation>,
		response: Response,
		max_weight: Weight,
		context: &XcmContext,
	) -> Weight {
		//need this line because querier is None
		//and here is where it failed in pallet-xcm
		//https://github.com/paritytech/polkadot/blob/release-v0.9.43/xcm/pallet-xcm/src/lib.rs#L2001-L2010
		//so need to substitute it with expected querier
		let mut querier = querier;
		if querier.is_none() {
			querier = Some(&MultiLocation { parents: 0, interior: Here });
		}
		PolkadotXcm::on_response(origin, query_id, querier, response, max_weight, context)
	}
}

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	type AssetTransactor = LocalAssetTransactor;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = MultiNativeAsset<AbsoluteReserveProvider>;
	type IsTeleporter = ();
	type UniversalLocation = UniversalLocation;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type Trader = Trader;
	type AssetTrap = CaptureAssetTrap;
	type ResponseHandler = XcmExecutorHandler;
	type SubscriptionService = PolkadotXcm;
	type AssetClaims = PolkadotXcm;
	type AssetLocker = ();
	type AssetExchanger = ();
	type PalletInstancesInfo = AllPalletsWithSystem;
	type MaxAssetsIntoHolding = ConstU32<64>;
	type FeeManager = ();
	type MessageExporter = ();
	type UniversalAliases = Nothing;
	type CallDispatcher = RuntimeCall;
	type SafeCallFilter = Everything;
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

		let location = primitives::currency::VersionedMultiLocation::V3(*location);
		if let Some(Parachain(id)) = interior {
			if let Some(amount) = AssetsRegistry::min_xcm_fee(*id, location.into()) {
				return Some(amount)
			}
		}

		match (parents, interior) {
			(1, None) => Some(400_000),
			_ => None,
		}
	};
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
	type MaxAssetsForTransfer = XcmMaxAssetsForTransfer;
	type ReserveProvider = RelativeReserveProvider;
	type UniversalLocation = UniversalLocation;
}

impl orml_unknown_tokens::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

#[cfg(feature = "runtime-benchmarks")]
parameter_types! {
	pub ReachableDest: Option<MultiLocation> = Some(Parent.into());
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
	type WeightInfo = crate::weights::pallet_xcm::WeightInfo<Runtime>;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type CurrencyMatcher = ();
	type TrustedLockers = ();
	type SovereignAccountOf = ();
	type MaxLockers = ConstU32<8>;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = VERSION_DISCOVERY_QUEUE_SIZE;
	type UniversalLocation = UniversalLocation;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
	#[cfg(feature = "runtime-benchmarks")]
	type ReachableDest = ReachableDest;

	type AdminOrigin = EnsureRoot<Self::AccountId>;

	type MaxRemoteLockConsumers = ConstU32<32>;

	type RemoteLockConsumerIdentifier = super::assets::BalanceIdentifier;
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
	type VersionWrapper = PolkadotXcm;
	type ChannelInfo = ParachainSystem;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type WeightInfo = cumulus_pallet_xcmp_queue::weights::SubstrateWeight<Self>;
	type ControllerOrigin = EnsureRootOrHalfNativeTechnical;
	type ExecuteOverweightOrigin = EnsureRootOrHalfNativeTechnical;
	type PriceForSiblingDelivery = ();
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = EnsureRootOrTwoThirdNativeCouncil;
}
