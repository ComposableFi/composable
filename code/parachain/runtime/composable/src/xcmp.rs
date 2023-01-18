//! Setup of XCMP for parachain to allow cross chain transfers and other operations.
//! Very similar to https://github.com/galacticcouncil/Basilisk-node/blob/master/runtime/basilisk/src/xcm.rs
#![allow(unused_imports)] // allow until v2 xcm released (instead creating 2 runtimes)

use super::*; // recursive dependency onto runtime
use codec::{Decode, Encode};
use common::xcmp::CurrencyIdConvert;
use composable_traits::{
	currency::LocalAssets,
	xcm::assets::{RemoteAssetRegistryInspect, XcmAssetLocation},
};
use core::marker::PhantomData;
use cumulus_primitives_core::{IsSystem, ParaId};
use frame_support::{
	construct_runtime, log, match_types, parameter_types,
	traits::{
		Contains, Everything, KeyOwnerProofSystem, Nothing, OriginTrait, PalletInfoAccess,
		Randomness, StorageInfo,
	},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
		IdentityFee, Weight, WeightToFeeCoefficient, WeightToFeeCoefficients,
		WeightToFeePolynomial,
	},
	PalletId,
};
use orml_traits::{location::AbsoluteReserveProvider, parameter_type_with_key};
use orml_xcm_support::{
	DepositToAlternative, IsNativeConcrete, MultiCurrencyAdapter, MultiNativeAsset,
};
use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use scale_info::TypeInfo;
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata, H160, H256};
use sp_runtime::{
	traits::{AccountIdLookup, BlakeTwo256, Convert, ConvertInto, Zero},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult,
};
use sp_std::prelude::*;
use xcm::latest::{prelude::*, Error};
use xcm_builder::{
	AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
	AllowTopLevelPaidExecutionFrom, EnsureXcmOrigin, FixedWeightBounds, LocationInverter,
	ParentIsPreset, RelayChainAsNative, SiblingParachainAsNative, SiblingParachainConvertsVia,
	SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit,
};
use xcm_executor::{
	traits::{ConvertOrigin, TransactAsset, WeightTrader},
	Assets, Config, XcmExecutor,
};

parameter_types! {
	// pub const RelayLocation: MultiLocation = MultiLocation::X1(Junction::Parent);
	pub const RelayNetwork: NetworkId = NetworkId::Kusama;
	pub RelayOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
	pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
}

// here we should add any partner network for zero cost transactions
match_types! {
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
pub type LocalOriginToLocation = ();

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
	SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	// Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
	// recognized.
	RelayChainAsNative<RelayOrigin, RuntimeOrigin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognized.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
	// Superuser converter for the Relay-chain (Parent) location. This will allow it to issue a
	// transaction from the Root origin.
	xcm_builder::ParentAsSuperuser<RuntimeOrigin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `Origin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<RuntimeOrigin>,
);

#[cfg(feature = "develop")]
pub type LocalAssetTransactor = MultiCurrencyAdapter<
	crate::Assets,
	UnknownTokens,
	IsNativeConcrete<CurrencyId, CurrencyIdConvert>,
	AccountId,
	LocationToAccountId,
	CurrencyId,
	CurrencyIdConvert,
	DepositFailureHandler,
>;

parameter_types! {
	pub const BaseXcmWeight: u64 = 0;
	pub const MaxInstructions: u32 = 10_000;
}

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;

	#[cfg(not(feature = "develop"))]
	type AssetTransactor = ();
	// How to withdraw and deposit an asset.
	#[cfg(feature = "develop")]
	type AssetTransactor = LocalAssetTransactor;

	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = MultiNativeAsset<AbsoluteReserveProvider>;
	type IsTeleporter = (); // <- should be enough to allow teleportation of PICA
	type LocationInverter = LocationInverter<Ancestry>;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;

	type Trader = ();

	type ResponseHandler = ();

	type SubscriptionService = RelayerXcm;
	type AssetClaims = RelayerXcm;
	type AssetTrap = RelayerXcm;
}

parameter_types! {
	pub SelfLocation: MultiLocation = MultiLocation::new(1, X1(Parachain(ParachainInfo::parachain_id().into())));
}

pub struct AccountIdToMultiLocation;
impl Convert<AccountId, MultiLocation> for AccountIdToMultiLocation {
	fn convert(account: AccountId) -> MultiLocation {
		X1(Junction::AccountId32 { network: NetworkId::Any, id: account.into() }).into()
	}
}

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	/// https://medium.com/kusama-network/kusamas-governance-thwarts-would-be-attacker-9023180f6fb
	type XcmExecuteFilter = Nothing;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Everything;
	type XcmReserveTransferFilter = Everything;
	type LocationInverter = LocationInverter<Ancestry>;
	type Weigher = FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;

	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
}

impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type VersionWrapper = ();
	type ChannelInfo = ParachainSystem;
	type ExecuteOverweightOrigin = EnsureRootOrHalfCouncil;
	type ControllerOrigin = EnsureRootOrHalfCouncil;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type WeightInfo = cumulus_pallet_xcmp_queue::weights::SubstrateWeight<Self>;
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = EnsureRootOrHalfCouncil;
}
