//! Setup of XCMP for parachain to allow cross chain transfers and other operations.
//! Very similar to https://github.com/galacticcouncil/Basilisk-node/blob/master/runtime/basilisk/src/xcm.rs
#![allow(unused_imports)] // allow until v2 xcm released (instead creating 2 runtimes)

use super::*; // recursive dependency onto runtime

use codec::{Decode, Encode};
use composable_traits::assets::{RemoteAssetRegistry, XcmAssetLocation};
use cumulus_primitives_core::ParaId;
use frame_support::{
	construct_runtime, ensure, log, match_type, parameter_types,
	traits::{Contains, Everything, KeyOwnerProofSystem, Nothing, Randomness, StorageInfo},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
		DispatchClass, IdentityFee, Weight, WeightToFeeCoefficient, WeightToFeeCoefficients,
		WeightToFeePolynomial,
	},
	PalletId,
};

use orml_xcm_support::{IsNativeConcrete, MultiCurrencyAdapter, MultiNativeAsset};

use sp_runtime::{
	traits::{AccountIdLookup, BlakeTwo256, Convert, ConvertInto, Zero},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult,
};

use orml_traits::{location::Reserve, parameter_type_with_key};
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};

use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use sp_std::{marker::PhantomData, prelude::*};
use xcm::latest::{prelude::*, Error};
use xcm_builder::{
	AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
	AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom, EnsureXcmOrigin, FixedWeightBounds,
	LocationInverter, ParentIsDefault, RelayChainAsNative, SiblingParachainAsNative,
	SiblingParachainConvertsVia, SignedAccountId32AsNative, SignedToAccountId32,
	SovereignSignedViaLocation, TakeWeightCredit,
};
use xcm_executor::{
	traits::{FilterAssetLocation, ShouldExecute, TransactAsset, WeightTrader},
	Assets, Config, XcmExecutor,
};

parameter_types! {
	pub KsmLocation: MultiLocation = MultiLocation::parent();
	pub const RelayNetwork: NetworkId = NetworkId::Kusama;
	pub RelayOrigin: Origin = cumulus_pallet_xcm::Origin::Relay.into();
	pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
}

// Here we should add any partner network for zero cost transactions
// 1000 is statmeing - see the Kusama runtime setup
// (1, Here) - jump 1 up, and say here - Relay
// (1, 1000) - jump 1 up and go to child 1000
match_type! {
	pub type WellKnownsChains: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
			MultiLocation { parents: 1, interior: X1(Parachain(1000)) }
	};
}

/// this is debug struct implementing as many XCMP interfaces as possible
/// it just dumps content, no modification.
/// returns default expected
pub struct XcmpDebug;

impl xcm_executor::traits::ShouldExecute for XcmpDebug {
	fn should_execute<Call>(
		origin: &MultiLocation,
		message: &mut Xcm<Call>,
		max_weight: Weight,
		weight_credit: &mut Weight,
	) -> Result<(), ()> {
		log::trace!(target: "should_execute", "{:?} {:?} {:?} {:?}", origin, message, max_weight, weight_credit);
		Err(())
	}
}

/// NOTE: there could be payments taken on other side, so cannot rely on this to work end to end
pub struct DebugAllowUnpaidExecutionFrom<T>(PhantomData<T>);
impl<T: Contains<MultiLocation>> ShouldExecute for DebugAllowUnpaidExecutionFrom<T> {
	fn should_execute<Call>(
		origin: &MultiLocation,
		_message: &mut Xcm<Call>,
		_max_weight: Weight,
		_weight_credit: &mut Weight,
	) -> Result<(), ()> {
		log::trace!(
			target: "xcm::barriers",
			"AllowUnpaidExecutionFrom origin: {:?}, message: {:?}, max_weight: {:?}, weight_credit: {:?}, contains: {:?}",
			origin, _message, _max_weight, _weight_credit, T::contains(origin),
		);
		ensure!(T::contains(origin), ());
		Ok(())
	}
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
	// The parent (Relay-chain) origin converts to the default `AccountId`.
	ParentIsDefault<AccountId>,
	// Sibling parachain origins convert to AccountId via the `ParaId::into`.
	SiblingParachainConvertsVia<Sibling, AccountId>,
	// Straight up local `AccountId32` origins just alias directly to `AccountId`.
	AccountId32Aliases<RelayNetwork, AccountId>,
);

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with XCM's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
	// Sovereign account converter; this attempts to derive an `AccountId` from the origin location
	// using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
	// foreign chains who want to have a local sovereign account on this chain which they control.
	SovereignSignedViaLocation<LocationToAccountId, Origin>,
	// Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
	// recognised.
	RelayChainAsNative<RelayOrigin, Origin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognised.
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

pub type LocalAssetTransactor = MultiCurrencyAdapter<
	crate::Assets,
	UnknownTokens,
	IsNativeConcrete<CurrencyId, CurrencyIdConvert>,
	AccountId,
	LocationToAccountId,
	CurrencyId,
	CurrencyIdConvert,
	// TODO(hussein-aitlahcen): DepositFailureHandler
	(),
>;

// TODO: port multi pay from Acala, mostly KSM/DOT
pub struct TransactionFeePoolTrader<Converter> {
	_marker: PhantomData<Converter>,
}

impl<Converter: Convert<MultiLocation, Option<CurrencyId>>> WeightTrader
	for TransactionFeePoolTrader<Converter>
{
	fn new() -> Self {
		Self { _marker: PhantomData::<Converter>::default() }
	}

	fn buy_weight(&mut self, weight: Weight, payment: Assets) -> Result<Assets, Error> {
		// this is for trusted chains origin, see `f` if any
		// TODO: dicuss if we need payments from Relay chain or common goods chains?
		if weight.is_zero() {
			return Ok(payment)
		}

		// only support first fungible assets now.
		let asset_id = payment
			.fungible
			.iter()
			.next()
			.map_or(Err(XcmError::TooExpensive), |v| Ok(v.0))?;

		if let AssetId::Concrete(ref multi_location) = asset_id.clone() {
			if let Some(token_id) = Converter::convert(multi_location.clone()) {
				if token_id == CurrencyId::PICA || token_id == CurrencyId::KSM {
					// CU-1wme602
					// TODO: port swap for KSM/DOT from Acala (here it is just 1/50 constant)
					// NOTE: Acala does many currencies, but we can limit to only 1 (because
					// everybody can swap into Relay native currency)
					let required = if token_id == CurrencyId::KSM { weight / 50 } else { weight };
					let required =
						MultiAsset { id: asset_id.clone(), fun: Fungible(required as u128) };

					log::trace!(target : "xcml::buy_weight", "{:?} {:?} ", required, payment );
					let unused =
						payment.checked_sub(required).map_err(|_| XcmError::TooExpensive)?;
					// TODO: port refund_weight and trasfner from treasury from Acala
					return Ok(unused)
				}
			}
		}

		log::info!(target : "xcml::buy_weight", "required {:?}; provided {:?};", weight, payment );
		Err(XcmError::TooExpensive)
	}
}
pub struct RelayReserverFromParachain;
impl FilterAssetLocation for RelayReserverFromParachain {
	fn filter_asset_location(asset: &MultiAsset, origin: &MultiLocation) -> bool {
		// NOTE: In Acala there is not such thing
		// if asset is KSM and send from some parachain then allow for  that
		asset.reserve() == Some(MultiLocation::parent()) &&
			matches!(origin, MultiLocation { parents: 1, interior: X1(Parachain(_)) })
	}
}

pub struct DebugMultiNativeAsset;
impl FilterAssetLocation for DebugMultiNativeAsset {
	fn filter_asset_location(asset: &MultiAsset, origin: &MultiLocation) -> bool {
		log::trace!(
			target: "xcmp::filter_asset_location",
			"asset: {:?}; origin: {:?}; reserve: {:?};",
			&asset,
			&origin,
			&asset.clone().reserve(),
		);
		false
	}
}

type IsReserveAssetLocationFilter =
	(DebugMultiNativeAsset, MultiNativeAsset, RelayReserverFromParachain);

type Trader = TransactionFeePoolTrader<CurrencyIdConvert>;

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
	type AssetTrap = RelayerXcm;
}

parameter_types! {
	pub SelfLocation: MultiLocation = MultiLocation::new(1, X1(Parachain(ParachainInfo::parachain_id().into())));
	// safe value to start to transfer 1 asset only in one message (as in Acala)
	pub const MaxAssetsForTransfer: usize = 1;
}

parameter_types! {
	pub const BaseXcmWeight: Weight = 100_000_000;
}

impl orml_xtokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type CurrencyIdConvert = CurrencyIdConvert;
	type AccountIdToMultiLocation = AccountIdToMultiLocation;
	type SelfLocation = SelfLocation;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
	type BaseXcmWeight = BaseXcmWeight;
	type LocationInverter = LocationInverter<Ancestry>;
	type MaxAssetsForTransfer = MaxAssetsForTransfer;
}

impl orml_unknown_tokens::Config for Runtime {
	type Event = Event;
}

/// Is configured to convert some account id to account id on other network
/// as of now it is same as in Acala/Hydra
pub struct AccountIdToMultiLocation;
impl Convert<AccountId, MultiLocation> for AccountIdToMultiLocation {
	fn convert(account: AccountId) -> MultiLocation {
		//  considers any other network using globally unique ids
		X1(AccountId32 { network: NetworkId::Any, id: account.into() }).into()
	}
}

/// Converts currency to and from local and remote
pub struct CurrencyIdConvert;

/// converts local currency into remote,
/// native currency is built in
impl sp_runtime::traits::Convert<CurrencyId, Option<MultiLocation>> for CurrencyIdConvert {
	fn convert(id: CurrencyId) -> Option<MultiLocation> {
		match id {
			CurrencyId::INVALID => {
				log::info!(
					target: "xcmp:convert",
					"mapping for {:?} on {:?} parachain not found",
					id,
					ParachainInfo::parachain_id()
				);
				None
			},
			CurrencyId::PICA => Some(MultiLocation::new(
				1,
				X2(Parachain(ParachainInfo::parachain_id().into()), GeneralKey(id.encode())),
			)),
			CurrencyId::KSM => Some(MultiLocation::parent()),
			_ => {
				if let Some(location) =
					<AssetsRegistry as RemoteAssetRegistry>::asset_to_location(id).map(Into::into)
				{
					Some(location)
				} else {
					log::trace!(
						target: "xcmp:convert",
						"mapping for {:?} on {:?} parachain not found",
						id,
						ParachainInfo::parachain_id()
					);
					None
				}
			},
		}
	}
}

/// converts from Relay parent chain to child chain currency
/// expected that currency in location is in format well known for local chain
impl Convert<MultiLocation, Option<CurrencyId>> for CurrencyIdConvert {
	fn convert(location: MultiLocation) -> Option<CurrencyId> {
		log::trace!(target: "xcmp::convert", "converting {:?} on {:?}", &location, ParachainInfo::parachain_id());
		match location {
			MultiLocation { parents, interior: X2(Parachain(id), GeneralKey(key)) }
				if parents == 1 && ParaId::from(id) == ParachainInfo::parachain_id() =>
			{
				// Handling native asset for this parachain
				if let Ok(currency_id) = CurrencyId::decode(&mut &key[..]) {
					// we currently have only one native asset
					match currency_id {
						CurrencyId::PICA => Some(CurrencyId::PICA),
						_ => {
							log::error!("failed converting currency");
							None
						},
					}
				} else {
					log::error!("failed converting currency");
					None
				}
			},
			// TODO: make this const expression to filter parent
			MultiLocation { parents: 1, interior: Here } => {
				// ISSUE: will need to be more clever on DOT
				Some(CurrencyId::KSM)
			},
			// delegate to asset-registry
			_ => {
				let result = <AssetsRegistry as RemoteAssetRegistry>::location_to_asset(
					XcmAssetLocation(location),
				)
				.map(Into::into);
				if result.is_none() {
					log::error!("failed converting currency");
				}
				result
			},
		}
	}
}

/// covert remote to local, usually when receiving transfer
impl Convert<MultiAsset, Option<CurrencyId>> for CurrencyIdConvert {
	fn convert(asset: MultiAsset) -> Option<CurrencyId> {
		log::trace!("converting {:?}", &asset);
		if let MultiAsset { id: Concrete(location), .. } = asset {
			Self::convert(location)
		} else {
			log::error!("failed to find remote asset");
			None
		}
	}
}

// make setup as in Acala, max instructions seems resoanble, for weigth may consider to  settle with
// our PICA
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

/// Cumulus is the default implementation of queue integrated with Polkadot and Kusama runtimes
impl cumulus_pallet_xcm::Config for Runtime {
	type Event = Event;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type Event = Event;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type VersionWrapper = ();
	type ChannelInfo = ParachainSystem;
	// NOTE: we could consider allowance for some chains (see Acala tests ports  PRs)
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
	type Event = Event;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = system::EnsureRoot<AccountId>;
}
