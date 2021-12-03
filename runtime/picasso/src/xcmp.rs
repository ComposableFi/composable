//! Setup of XCMP for parachain to allow cross chain transfers and other operations.
//! Very similar to https://github.com/galacticcouncil/Basilisk-node/blob/master/runtime/basilisk/src/xcm.rs
#![allow(unused_imports)] // allow until v2 xcm released (instead creating 2 runtimes)
#![cfg_attr(not(feature = "std"), no_std)]

use super::*; // recursive dependency onto runtime

use codec::{Decode, Encode};
use composable_traits::assets::{RemoteAssetRegistry, XcmAssetLocation};
use cumulus_primitives_core::ParaId;
use support::{
	construct_runtime, log, match_type, parameter_types,
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

use orml_traits::parameter_type_with_key;
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};

use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use sp_std::prelude::*;
use xcm::latest::{prelude::*, Error};
use xcm_builder::{
	AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
	AllowTopLevelPaidExecutionFrom, EnsureXcmOrigin, FixedWeightBounds, LocationInverter,
	ParentIsDefault, RelayChainAsNative, SiblingParachainAsNative, SiblingParachainConvertsVia,
	SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit,
};
use xcm_executor::{
	traits::{TransactAsset, WeightTrader},
	Assets, Config, XcmExecutor,
};

parameter_types! {
	// pub const RelayLocation: MultiLocation = MultiLocation::X1(Junction::Parent);
	pub const RelayNetwork: NetworkId = NetworkId::Kusama;
	pub RelayOrigin: Origin = cumulus_pallet_xcm::Origin::Relay.into();
	pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
}

// here we should add any partner network for zero cost transactions
match_type! {
	pub type SpecParachain: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: X1(Parachain(2000)) } |
			MultiLocation { parents: 1, interior: X1(Parachain(3000)) }
	};
}

pub type Barrier = (
	TakeWeightCredit,
	AllowTopLevelPaidExecutionFrom<Everything>,
	xcm_builder::AllowUnpaidExecutionFrom<SpecParachain>,
	// Expected responses are OK.
	AllowKnownQueryResponses<RelayerXcm>,
	// Subscriptions for version tracking are OK.
	AllowSubscriptionsFrom<Everything>,
);

/// No local origins on this chain are allowed to dispatch XCM sends/executions.
/// https://medium.com/kusama-network/kusamas-governance-thwarts-would-be-attacker-9023180f6fb
#[cfg(not(feature = "develop"))]
pub type LocalOriginToLocation = ();
#[cfg(feature = "develop")]
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
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
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
>;

parameter_types! {
	pub const BaseXcmWeight: Weight = 0;
	pub const MaxInstructions: u32 = 10_000;
}

#[cfg(feature = "develop")]
pub struct TradePassthrough();

/// any payment to pass
#[cfg(feature = "develop")]
impl WeightTrader for TradePassthrough {
	fn new() -> Self {
		Self()
	}

	fn buy_weight(&mut self, _weight: Weight, payment: Assets) -> Result<Assets, Error> {
		// Just let it through for now
		Ok(payment)
	}
}

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type Call = Call;
	type XcmSender = XcmRouter;

	#[cfg(not(feature = "develop"))]
	type AssetTransactor = ();
	// How to withdraw and deposit an asset.
	#[cfg(feature = "develop")]
	type AssetTransactor = LocalAssetTransactor;

	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = MultiNativeAsset;
	type IsTeleporter = (); // <- should be enough to allow teleportation of PICA
	type LocationInverter = LocationInverter<Ancestry>;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<BaseXcmWeight, Call, MaxInstructions>;

	#[cfg(not(feature = "develop"))]
	type Trader = ();
	#[cfg(feature = "develop")]
	type Trader = TradePassthrough;

	#[cfg(not(feature = "develop"))]
	type ResponseHandler = ();
	#[cfg(feature = "develop")]
	type ResponseHandler = RelayerXcm;

	type SubscriptionService = RelayerXcm;
	type AssetClaims = RelayerXcm;
	type AssetTrap = RelayerXcm;
}

parameter_types! {
	pub SelfLocation: MultiLocation = MultiLocation::new(1, X1(Parachain(ParachainInfo::parachain_id().into())));
}

#[cfg(feature = "develop")]
impl orml_xtokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type CurrencyIdConvert = CurrencyIdConvert;
	type AccountIdToMultiLocation = AccountIdToMultiLocation;
	type SelfLocation = SelfLocation;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type Weigher = FixedWeightBounds<BaseXcmWeight, Call, MaxInstructions>;
	type BaseXcmWeight = BaseXcmWeight;
	type LocationInverter = LocationInverter<Ancestry>;
}

#[cfg(feature = "develop")]
impl orml_unknown_tokens::Config for Runtime {
	type Event = Event;
}

pub struct AccountIdToMultiLocation;
impl Convert<AccountId, MultiLocation> for AccountIdToMultiLocation {
	fn convert(account: AccountId) -> MultiLocation {
		X1(AccountId32 { network: NetworkId::Any, id: account.into() }).into()
	}
}

/// Converts currency to and from local and remote
#[cfg(feature = "develop")]
pub struct CurrencyIdConvert;

/// converts local currency into remote,
/// native currency is built in
#[cfg(feature = "develop")]
impl sp_runtime::traits::Convert<CurrencyId, Option<MultiLocation>> for CurrencyIdConvert {
	fn convert(id: CurrencyId) -> Option<MultiLocation> {
		match id {
			CurrencyId::INVALID => {
				log::trace!(
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
			_ => {
				if let Some(location) =
					<AssetsRegistry as RemoteAssetRegistry>::asset_to_location(id).map(Into::into)
				{
					Some(location)
				} else {
					log::trace!(
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
#[cfg(feature = "develop")]
impl Convert<MultiLocation, Option<CurrencyId>> for CurrencyIdConvert {
	fn convert(location: MultiLocation) -> Option<CurrencyId> {
		log::trace!("converting {:?} on {:?}", &location, ParachainInfo::parachain_id());
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
#[cfg(feature = "develop")]
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

impl pallet_xcm::Config for Runtime {
	type Event = Event;
	type SendXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
	/// https://medium.com/kusama-network/kusamas-governance-thwarts-would-be-attacker-9023180f6fb
	#[cfg(not(feature = "develop"))]
	type XcmExecuteFilter = Nothing;
	#[cfg(feature = "develop")]
	type XcmExecuteFilter = Everything;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Everything;
	type XcmReserveTransferFilter = Everything;
	type LocationInverter = LocationInverter<Ancestry>;
	type Weigher = FixedWeightBounds<BaseXcmWeight, Call, MaxInstructions>;
	type Origin = Origin;
	type Call = Call;

	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
}

impl cumulus_pallet_xcm::Config for Runtime {
	type Event = Event;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type Event = Event;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type VersionWrapper = ();
	type ChannelInfo = ParachainSystem;
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
	type Event = Event;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = system::EnsureRoot<AccountId>;
}
