#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

mod weights;
use common::{
	impls::DealWithFees, AccountId, AccountIndex, Amount, AuraId, Balance, BlockNumber,
	CouncilInstance, EnsureRootOrHalfCouncil, Hash, Signature, AVERAGE_ON_INITIALIZE_RATIO, DAYS,
	HOURS, MAXIMUM_BLOCK_WEIGHT, MILLI_PICA, NORMAL_DISPATCH_RATIO, PICA, SLOT_DURATION,
};
use dutch_auction::DeFiComposableConfig;
use liquidations::DeFiComposablePallet;
use orml_traits::parameter_type_with_key;
use primitives::currency::CurrencyId;
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, ConvertInto, Zero},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, DispatchError,
};

use composable_traits::dex::{Orderbook, TakeResult};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use sp_runtime::traits::AccountIdConversion;

// A few exports that help ease life for downstream crates.
pub use support::{
	construct_runtime, match_type, parameter_types,
	traits::{Contains, Everything, KeyOwnerProofSystem, Randomness, StorageInfo},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
		DispatchClass, IdentityFee, Weight, WeightToFeeCoefficient, WeightToFeeCoefficients,
		WeightToFeePolynomial,
	},
	PalletId, StorageValue,
};

use codec::Encode;
use frame_system as system;
use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{FixedPointNumber, Perbill, Permill, Perquintill};
use system::{
	limits::{BlockLength, BlockWeights},
	EnsureRoot,
};
use transaction_payment::{Multiplier, TargetedFeeAdjustment};
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountId32Aliases, AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom, EnsureXcmOrigin,
	FixedWeightBounds, LocationInverter, NativeAsset, ParentAsSuperuser, ParentIsDefault,
	RelayChainAsNative, SiblingParachainAsNative, SiblingParachainConvertsVia,
	SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit,
};
use xcm_executor::XcmExecutor;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;

	impl_opaque_keys! {
		pub struct SessionKeys {
			pub aura: Aura,
		}
	}
}

// To learn more about runtime versioning and what each of the following value means:
//   https://substrate.dev/docs/en/knowledgebase/runtime/upgrades#runtime-versioning
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("picasso"),
	impl_name: create_runtime_str!("picasso"),
	authoring_version: 1,
	// The version of the runtime specification. A full node will not attempt to use its native
	//   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
	//   `spec_version`, and `authoring_version` are the same between Wasm and native.
	// This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
	//   the compatible custom types.
	spec_version: 100,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

parameter_types! {
	pub const BlockHashCount: BlockNumber = 250;
	pub const Version: RuntimeVersion = VERSION;
	pub RuntimeBlockLength: BlockLength =
		BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
		.base_block(BlockExecutionWeight::get())
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = ExtrinsicBaseWeight::get();
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
			// Operational transactions have some extra reserved space, so that they
			// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
			weights.reserved = Some(
				MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
			);
		})
		.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
		.build_or_panic();

	pub const SS58Prefix: u8 = 49;
}

// Configure FRAME pallets to include in runtime.

impl system::Config for Runtime {
	/// The basic call filter to use in dispatchable.
	type BaseCallFilter = BaseCallFilter;
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = RuntimeBlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = RuntimeBlockLength;
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The aggregated dispatch type that is available for extrinsics.
	type Call = Call;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = AccountIdLookup<AccountId, AccountIndex>;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = AccountIndex;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header type.
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// The ubiquitous event type.
	type Event = Event;
	/// The ubiquitous origin type.
	type Origin = Origin;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// Version of the runtime.
	type Version = Version;
	/// The data to be stored in an account.
	type AccountData = balances::AccountData<Balance>;

	/// Converts a module to the index of the module in `construct_runtime!`.
	///
	/// This type is being generated by `construct_runtime!`.
	type PalletInfo = PalletInfo;
	/// What to do if a new account is created.
	type OnNewAccount = ();
	/// What to do if an account is fully reaped from the system.
	type OnKilledAccount = ();
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = weights::frame_system::WeightInfo<Runtime>;
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
	/// The action to take on a Runtime Upgrade. Used not default since we're a parachain.
	type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
}

impl randomness_collective_flip::Config for Runtime {}

parameter_types! {
	pub const MaxAuthorities: u32 = 1_000;
}

impl aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = MaxAuthorities;
}

impl cumulus_pallet_aura_ext::Config for Runtime {}

parameter_types! {
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the Unix epoch.
	type Moment = u64;
	type OnTimestampSet = Aura;
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = weights::timestamp::WeightInfo<Runtime>;
}

/// minimum account balance is given as 0.1 PICA ~ 100 MILLI_PICA
pub const EXISTENTIAL_DEPOSIT: Balance = 100 * MILLI_PICA;

parameter_types! {
	pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
	pub const MaxLocks: u32 = 50;
}

impl balances::Config for Runtime {
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = Treasury;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = weights::balances::WeightInfo<Runtime>;
}

parameter_types! {
	/// 1 milli-pica/byte should be fine
	pub const TransactionByteFee: Balance = MILLI_PICA;

	// The portion of the `NORMAL_DISPATCH_RATIO` that we adjust the fees with. Blocks filled less
	/// than this will decrease the weight and more will increase.
	pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
	/// The adjustment variable of the runtime. Higher values will cause `TargetBlockFullness` to
	/// change the fees more rapidly. This low value causes changes to occur slowly over time.
	pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(3, 100_000);
	/// Minimum amount of the multiplier. This value cannot be too low. A test case should ensure
	/// that combined with `AdjustmentVariable`, we can recover from the minimum.
	/// See `multiplier_can_grow_from_zero` in integration_tests.rs.
	/// This value is currently only used by pallet-transaction-payment as an assertion that the
	/// next multiplier is always > min value.
	pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000u128);
	pub const OperationalFeeMultiplier: u8 = 5;
}

pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
	type Balance = Balance;
	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		let p = MILLI_PICA;
		let q = 10 * Balance::from(ExtrinsicBaseWeight::get());
		smallvec::smallvec![WeightToFeeCoefficient {
			degree: 1,
			negative: false,
			coeff_frac: Perbill::from_rational(p % q, q),
			coeff_integer: p / q,
		}]
	}
}

impl transaction_payment::Config for Runtime {
	type OnChargeTransaction =
		transaction_payment::CurrencyAdapter<Balances, DealWithFees<Runtime>>;
	type TransactionByteFee = TransactionByteFee;
	type WeightToFee = WeightToFee;
	type FeeMultiplierUpdate =
		TargetedFeeAdjustment<Self, TargetBlockFullness, AdjustmentVariable, MinimumMultiplier>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
}

impl sudo::Config for Runtime {
	type Event = Event;
	type Call = Call;
}

parameter_types! {
	/// Index deposit requires a 100 PICA
	pub const IndexDeposit: Balance = 100 * PICA;
}

impl indices::Config for Runtime {
	type Event = Event;
	type AccountIndex = AccountIndex;
	type Currency = Balances;
	type Deposit = IndexDeposit;
	type WeightInfo = weights::indices::WeightInfo<Runtime>;
}

pub type SignedPayload = generic::SignedPayload<Call, SignedExtra>;

impl<LocalCall> system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
	Call: From<LocalCall>,
{
	fn create_transaction<C: system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call,
		public: <Signature as sp_runtime::traits::Verify>::Signer,
		account: AccountId,
		nonce: AccountIndex,
	) -> Option<(Call, <UncheckedExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload)> {
		use sp_runtime::{
			generic::{Era, SignedPayload},
			traits::StaticLookup,
			SaturatedConversion,
		};
		let tip = 0;
		// take the biggest period possible.
		let period =
			BlockHashCount::get().checked_next_power_of_two().map(|c| c / 2).unwrap_or(2) as u64;
		let current_block = System::block_number()
			.saturated_into::<u64>()
			// The `System::block_number` is initialized with `n+1`,
			// so the actual block number is `n`.
			.saturating_sub(1);
		let era = Era::mortal(period, current_block);
		let extra = (
			system::CheckSpecVersion::<Runtime>::new(),
			system::CheckTxVersion::<Runtime>::new(),
			system::CheckGenesis::<Runtime>::new(),
			system::CheckEra::<Runtime>::from(era),
			system::CheckNonce::<Runtime>::from(nonce),
			system::CheckWeight::<Runtime>::new(),
			transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
		);
		let raw_payload = SignedPayload::new(call, extra)
			.map_err(|_e| {
				// log::warn!("Unable to create signed payload: {:?}", e);
			})
			.ok()?;
		let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;
		let address = AccountIdLookup::unlookup(account);
		let (call, extra, _) = raw_payload.deconstruct();
		Some((call, (address, signature, extra)))
	}
}

impl system::offchain::SigningTypes for Runtime {
	type Public = <Signature as sp_runtime::traits::Verify>::Signer;
	type Signature = Signature;
}

impl<C> system::offchain::SendTransactionTypes<C> for Runtime
where
	Call: From<C>,
{
	type OverarchingCall = Call;
	type Extrinsic = UncheckedExtrinsic;
}

//TODO set
parameter_types! {
	pub const StakeLock: BlockNumber = 50;
	pub const StalePrice: BlockNumber = 5;

	/// TODO: discuss with omar/cosmin
	pub const MinStake: Balance = 1000 * PICA;
	pub const RequestCost: Balance = PICA;
	pub const RewardAmount: Balance = 5 * PICA;
	// Shouldn't this be a ratio based on locked amount?
	pub const SlashAmount: Balance = 5;
	pub const MaxAnswerBound: u32 = 25;
	pub const MaxAssetsCount: u32 = 100_000;

}

impl oracle::Config for Runtime {
	type Currency = Balances;
	type Event = Event;
	type AuthorityId = oracle::crypto::BathurstStId;
	type AssetId = CurrencyId;
	type PriceValue = u128;
	type StakeLock = StakeLock;
	type MinStake = MinStake;
	type StalePrice = StalePrice;
	type AddOracle = EnsureRootOrHalfCouncil;
	type RequestCost = RequestCost;
	type RewardAmount = RewardAmount;
	type SlashAmount = SlashAmount;
	type MaxAnswerBound = MaxAnswerBound;
	type MaxAssetsCount = MaxAssetsCount;
	type WeightInfo = weights::oracle::WeightInfo<Runtime>;
}

// Parachain stuff.
// See https://github.com/paritytech/cumulus/blob/polkadot-v0.9.8/polkadot-parachains/rococo/src/lib.rs for details.
parameter_types! {
	pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 4;
	pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 4;
}

impl cumulus_pallet_parachain_system::Config for Runtime {
	type Event = Event;
	type OnValidationData = ();
	type SelfParaId = parachain_info::Pallet<Runtime>;
	type OutboundXcmpMessageSource = XcmpQueue;
	type DmpMessageHandler = DmpQueue;
	type ReservedDmpWeight = ReservedDmpWeight;
	type XcmpMessageHandler = XcmpQueue;
	type ReservedXcmpWeight = ReservedXcmpWeight;
}

impl parachain_info::Config for Runtime {}

parameter_types! {
	// pub const RelayLocation: MultiLocation = MultiLocation::X1(Junction::Parent);
	pub const RelayNetwork: NetworkId = NetworkId::Kusama;
	pub RelayOrigin: Origin = cumulus_pallet_xcm::Origin::Relay.into();
	pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
}

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
	ParentAsSuperuser<Origin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `Origin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, Origin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<Origin>,
);

parameter_types! {
	// One XCM operation is 1_000_000 weight - almost certainly a conservative estimate.
	pub UnitWeightCost: Weight = 1_000_000;
	pub const MaxInstructions: u32 = 100;
}

// For test purposes.
match_type! {
	pub type SpecParachain: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: X1(Parachain(2000)) } |
			MultiLocation { parents: 1, interior: X1(Parachain(2001)) }
	};
}

pub type Barrier = (
	TakeWeightCredit,
	AllowTopLevelPaidExecutionFrom<Everything>,
	AllowUnpaidExecutionFrom<SpecParachain>,
);

pub struct XcmConfig;

impl xcm_executor::Config for XcmConfig {
	type Call = Call;
	type XcmSender = XcmRouter;
	// How to withdraw and deposit an asset.
	type AssetTransactor = ();
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = NativeAsset;
	type IsTeleporter = (); // <- should be enough to allow teleportation of PICA
	type LocationInverter = LocationInverter<Ancestry>;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
	type Trader = ();
	type ResponseHandler = (); // Don't handle responses for now.
	type SubscriptionService = PolkadotXcm;
	type AssetClaims = PolkadotXcm;
	type AssetTrap = PolkadotXcm;
}

/// No local origins on this chain are allowed to dispatch XCM sends/executions.
pub type LocalOriginToLocation = SignedToAccountId32<Origin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, ()>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
);

impl pallet_xcm::Config for Runtime {
	type Event = Event;
	type SendXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
	type XcmExecuteFilter = Everything;
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

parameter_types! {
	pub const UncleGenerations: u32 = 0;
}

impl authorship::Config for Runtime {
	type FindAuthor = session::FindAccountFromAuthorIndex<Self, Aura>;
	type UncleGenerations = UncleGenerations;
	type FilterUncle = ();
	type EventHandler = (CollatorSelection,);
}

//TODO set
parameter_types! {
	pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(33);
	pub const Period: u32 = 6 * HOURS;
	pub const Offset: u32 = 0;
}

impl session::Config for Runtime {
	type Event = Event;
	type ValidatorId = <Self as system::Config>::AccountId;
	// we don't have stash and controller, thus we don't need the convert as well.
	type ValidatorIdOf = collator_selection::IdentityCollator;
	type ShouldEndSession = session::PeriodicSessions<Period, Offset>;
	type NextSessionRotation = session::PeriodicSessions<Period, Offset>;
	type SessionManager = CollatorSelection;
	// Essentially just Aura, but lets be pedantic.
	type SessionHandler =
		<opaque::SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
	type Keys = opaque::SessionKeys;
	type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
	type WeightInfo = weights::session::WeightInfo<Runtime>;
}

//TODO set
parameter_types! {
	pub const PotId: PalletId = PalletId(*b"PotStake");
	pub const MaxCandidates: u32 = 1000;
	pub const SessionLength: BlockNumber = 6 * HOURS;
	pub const MaxInvulnerables: u32 = 100;
	pub const MinCandidates: u32 = 5;
}

impl collator_selection::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type UpdateOrigin = EnsureRootOrHalfCouncil;
	type PotId = PotId;
	type MaxCandidates = MaxCandidates;
	type MinCandidates = MinCandidates;
	type MaxInvulnerables = MaxInvulnerables;
	// should be a multiple of session or things will get inconsistent
	type KickThreshold = Period;
	type ValidatorId = <Self as system::Config>::AccountId;
	type ValidatorIdOf = collator_selection::IdentityCollator;
	type ValidatorRegistration = Session;
	// TODO: benchmark for runtime
	type WeightInfo = ();
}

parameter_type_with_key! {
	// TODO:
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		Zero::zero()
	};
}

pub struct DustRemovalWhitelist;
impl Contains<AccountId> for DustRemovalWhitelist {
	fn contains(a: &AccountId) -> bool {
		let account: AccountId = TreasuryPalletId::get().into_account();
		let account2: AccountId = PotId::get().into_account();
		vec![&account, &account2].contains(&a)
	}
}

parameter_types! {
				pub TreasuryAccount: AccountId = TreasuryPalletId::get().into_account();
}

impl orml_tokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = weights::tokens::WeightInfo<Runtime>;
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = orml_tokens::TransferDust<Runtime, TreasuryAccount>;
	type MaxLocks = MaxLocks;
	type DustRemovalWhitelist = DustRemovalWhitelist;
}

parameter_types! {
	pub const LiquidRewardId: PalletId = PalletId(*b"Liquided");
	pub const CrowdloanCurrencyId: CurrencyId = CurrencyId::CROWD_LOAN;
	/// total contributed to our crowdloan.
	pub const TokenTotal: Balance = 200_000_000_000_000_000;
}

impl crowdloan_bonus::Config for Runtime {
	type Event = Event;
	type LiquidRewardId = LiquidRewardId;
	type CurrencyId = CrowdloanCurrencyId;
	type TokenTotal = TokenTotal;
	type JumpStart = EnsureRootOrHalfCouncil;
	type Currency = Tokens;
	type Balance = Balance;
	type NativeCurrency = Balances;
	type WeightInfo = weights::crowdloan_bonus::WeightInfo<Runtime>;
}

parameter_types! {
	pub const TreasuryPalletId: PalletId = PalletId(*b"picatrsy");
	/// percentage of proposal that most be bonded by the proposer
	pub const ProposalBond: Permill = Permill::from_percent(5);
	// TODO: rationale?
	pub const ProposalBondMinimum: Balance = 5 * PICA;
	pub const SpendPeriod: BlockNumber = 7 * DAYS;
	pub const Burn: Permill = Permill::from_percent(0);

	pub const MaxApprovals: u32 = 30;
}

impl treasury::Config for Runtime {
	type PalletId = TreasuryPalletId;
	type Currency = Balances;
	type ApproveOrigin = EnsureRootOrHalfCouncil;
	type RejectOrigin = EnsureRootOrHalfCouncil;
	type Event = Event;
	type OnSlash = Treasury;
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
	type MaxApprovals = MaxApprovals;
	type BurnDestination = ();
	type WeightInfo = weights::treasury::WeightInfo<Runtime>;
	// TODO: add bounties?
	type SpendFunds = ();
}

parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = 7 * DAYS;
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
}

impl membership::Config<membership::Instance1> for Runtime {
	type Event = Event;
	type AddOrigin = EnsureRootOrHalfCouncil;
	type RemoveOrigin = EnsureRootOrHalfCouncil;
	type SwapOrigin = EnsureRootOrHalfCouncil;
	type ResetOrigin = EnsureRootOrHalfCouncil;
	type PrimeOrigin = EnsureRootOrHalfCouncil;
	type MembershipInitialized = Council;
	type MembershipChanged = Council;
	type MaxMembers = CouncilMaxMembers;
	type WeightInfo = weights::membership::WeightInfo<Runtime>;
}

impl collective::Config<CouncilInstance> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = collective::PrimeDefaultVote;
	type WeightInfo = weights::collective::WeightInfo<Runtime>;
}

parameter_types! {
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
	RuntimeBlockWeights::get().max_block;
	pub const MaxScheduledPerBlock: u32 = 50;
}

impl scheduler::Config for Runtime {
	type Event = Event;
	type Origin = Origin;
	type PalletsOrigin = OriginCaller;
	type Call = Call;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = EnsureRoot<AccountId>;
	type MaxScheduledPerBlock = MaxScheduledPerBlock;
	type WeightInfo = weights::scheduler::WeightInfo<Runtime>;
}

impl utility::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type WeightInfo = weights::utility::WeightInfo<Runtime>;
}

parameter_types! {
	pub const LaunchPeriod: BlockNumber = 5 * DAYS;
	pub const VotingPeriod: BlockNumber = 5 * DAYS;
	pub const FastTrackVotingPeriod: BlockNumber = 3 * HOURS;
	pub MinimumDeposit: Balance = 100 * PICA;
	pub const EnactmentPeriod: BlockNumber = 2 * DAYS;
	pub const CooloffPeriod: BlockNumber = 7 * DAYS;
	// TODO: prod value
	pub PreimageByteDeposit: Balance = MILLI_PICA;
	pub const InstantAllowed: bool = true;
	pub const MaxVotes: u32 = 100;
	pub const MaxProposals: u32 = 100;
}

impl democracy::Config for Runtime {
	type Proposal = Call;
	type Event = Event;
	type Currency = Balances;
	type EnactmentPeriod = EnactmentPeriod;
	type LaunchPeriod = LaunchPeriod;
	type VotingPeriod = VotingPeriod;
	type VoteLockingPeriod = EnactmentPeriod;
	type MinimumDeposit = MinimumDeposit;

	// TODO: prod values
	type ExternalOrigin = EnsureRootOrHalfCouncil;
	type ExternalMajorityOrigin = EnsureRootOrHalfCouncil;
	type ExternalDefaultOrigin = EnsureRootOrHalfCouncil;

	type FastTrackOrigin = EnsureRootOrHalfCouncil;
	type InstantOrigin = EnsureRootOrHalfCouncil;
	type InstantAllowed = InstantAllowed;

	type FastTrackVotingPeriod = FastTrackVotingPeriod;
	type CancellationOrigin = EnsureRootOrHalfCouncil;
	type BlacklistOrigin = EnsureRootOrHalfCouncil;
	type CancelProposalOrigin = EnsureRootOrHalfCouncil;
	type VetoOrigin = collective::EnsureMember<AccountId, CouncilInstance>;
	type OperationalPreimageOrigin = collective::EnsureMember<AccountId, CouncilInstance>;
	type Slash = Treasury;

	type CooloffPeriod = CooloffPeriod;
	type MaxProposals = MaxProposals;
	type MaxVotes = MaxVotes;
	type PalletsOrigin = OriginCaller;

	type PreimageByteDeposit = PreimageByteDeposit;
	type Scheduler = Scheduler;
	type WeightInfo = weights::democracy::WeightInfo<Runtime>;
}

parameter_types! {
	pub const MaxStrategies: usize = 255;
	pub const NativeAssetId: CurrencyId = CurrencyId::PICA;
	pub const CreationDeposit: Balance = 10 * PICA;
	pub const VaultExistentialDeposit: Balance = 1000 * PICA;
	pub const RentPerBlock: Balance = MILLI_PICA;
	pub const VaultMinimumDeposit: Balance = 10_000;
	pub const VaultMinimumWithdrawal: Balance = 10_000;
	pub const VaultPalletId: PalletId = PalletId(*b"cubic___");
}

impl vault::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type CurrencyFactory = Factory;
	type AssetId = CurrencyId;
	type Currency = Tokens;
	type Convert = ConvertInto;
	type PalletId = VaultPalletId;
	type MaxStrategies = MaxStrategies;
	type CreationDeposit = CreationDeposit;
	type ExistentialDeposit = VaultExistentialDeposit;
	type RentPerBlock = RentPerBlock;
	type NativeAssetId = NativeAssetId;
	type MinimumDeposit = VaultMinimumDeposit;
	type MinimumWithdrawal = VaultMinimumWithdrawal;
}

parameter_types! {
	pub const DynamicCurrencyIdInitial: CurrencyId = CurrencyId::LOCAL_LP_TOKEN_START;
}

impl currency_factory::Config for Runtime {
	type Event = Event;
	type DynamicCurrencyId = CurrencyId;
	type DynamicCurrencyIdInitial = DynamicCurrencyIdInitial;
}

parameter_types! {
	// Benchmarks of pallet-lending utilize more than 20Gb memory
	// and don't finish if MaxLendingCount is u32::MAX.
	pub const MaxLendingCount: u32 = 100_000;
}

impl lending::Config for Runtime {
	type Oracle = Oracle;
	type VaultId = u64;
	type Vault = Vault;
	type Event = Event;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type Currency = Tokens;
	type CurrencyFactory = Factory;
	type MarketDebtCurrency = Tokens;
	type Liquidation = Liquidations;
	type UnixTime = Timestamp;
	type MaxLendingCount = MaxLendingCount;
	type AuthorityId = lending::crypto::TestAuthId;
	type WeightInfo = weights::lending::WeightInfo<Runtime>;
}

impl DeFiComposablePallet for Runtime {
	type AssetId = CurrencyId;
}

impl DeFiComposableConfig for Runtime {
	type AssetId = CurrencyId;
	type Balance = Balance;
	type Currency = Tokens;
}

pub struct MockOrderbook;
impl Orderbook for MockOrderbook {
	type AssetId = CurrencyId;
	type Balance = Balance;
	type AccountId = AccountId;
	type OrderId = u128;
	fn post(
		_account_from: &Self::AccountId,
		_asset: Self::AssetId,
		_want: Self::AssetId,
		_source_amount: Self::Balance,
		_source_price: Self::Balance,
		_amm_slippage: Permill,
	) -> Result<Self::OrderId, DispatchError> {
		Ok(0)
	}
	fn market_sell(
		_account: &Self::AccountId,
		_asset: Self::AssetId,
		_want: Self::AssetId,
		_amount: Self::Balance,
		_amm_slippage: Permill,
	) -> Result<Self::OrderId, DispatchError> {
		Ok(0)
	}
	fn take(
		_account: &Self::AccountId,
		_orders: impl Iterator<Item = Self::OrderId>,
		_up_to: Self::Balance,
	) -> Result<TakeResult<Self::Balance>, DispatchError> {
		Ok(TakeResult { amount: 0, total_price: 0 })
	}

	fn is_order_executed(_order_id: &Self::OrderId) -> bool {
		false
	}
}

impl dutch_auction::Config for Runtime {
	type Event = Event;
	type DexOrderId = u128;
	type OrderId = u128;
	type UnixTime = Timestamp;
	type Orderbook = MockOrderbook;
}

impl liquidations::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type UnixTime = Timestamp;
	type Lending = Lending;
	type DutchAuction = Auctions;
}

impl ping::Config for Runtime {
	type Event = Event;
	type Origin = Origin;
	type Call = Call;
	type XcmSender = XcmRouter;
}

// For test purposes.
impl cumulus_ping::Config for Runtime {
	type Event = Event;
	type Origin = Origin;
	type Call = Call;
	type XcmSender = XcmRouter;
}

impl assets_registry::Config for Runtime {
	type Event = Event;
	type LocalAssetId = u128;
	type ForeignAssetId = u128;
	type UpdateAdminOrigin = EnsureRootOrHalfCouncil;
	type LocalAdminOrigin = assets_registry::EnsureLocalAdmin<Runtime>;
	type ForeignAdminOrigin = assets_registry::EnsureForeignAdmin<Runtime>;
}

/// The calls we permit to be executed by extrinsics
pub struct BaseCallFilter;

impl Contains<Call> for BaseCallFilter {
	fn contains(call: &Call) -> bool {
		if call_filter::Pallet::<Runtime>::contains(call) {
			return false
		}
		!matches!(
			call,
			Call::Balances(_) | Call::Indices(_) | Call::Democracy(_) | Call::Treasury(_)
		)
	}
}

impl call_filter::Config for Runtime {
	type Event = Event;
	type UpdateOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: system::{Pallet, Call, Config, Storage, Event<T>} = 0,
		Timestamp: timestamp::{Pallet, Call, Storage, Inherent} = 1,
		Sudo: sudo::{Pallet, Call, Config<T>, Storage, Event<T>} = 2,
		RandomnessCollectiveFlip: randomness_collective_flip::{Pallet, Storage} = 3,
		TransactionPayment: transaction_payment::{Pallet, Storage} = 4,
		Indices: indices::{Pallet, Call, Storage, Config<T>, Event<T>} = 5,
		Balances: balances::{Pallet, Call, Storage, Config<T>, Event<T>} = 6,

		// Parachains stuff
		ParachainSystem: cumulus_pallet_parachain_system::{Pallet, Call, Config, Storage, Inherent, Event<T>} = 10,
		ParachainInfo: parachain_info::{Pallet, Storage, Config} = 11,

		// Collator support. the order of these 5 are important and shall not change.
		Authorship: authorship::{Pallet, Call, Storage} = 20,
		CollatorSelection: collator_selection::{Pallet, Call, Storage, Event<T>, Config<T>} = 21,
		Session: session::{Pallet, Call, Storage, Event, Config<T>} = 22,
		Aura: aura::{Pallet, Storage, Config<T>} = 23,
		AuraExt: cumulus_pallet_aura_ext::{Pallet, Config} = 24,

		// Governance utilities
		Council: collective::<Instance1>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>} = 30,
		CouncilMembership: membership::<Instance1>::{Pallet, Call, Storage, Event<T>, Config<T>} = 31,
		Treasury: treasury::{Pallet, Call, Storage, Config, Event<T>} = 32,
		Democracy: democracy::{Pallet, Call, Storage, Config<T>, Event<T>} = 33,
		Scheduler: scheduler::{Pallet, Call, Storage, Event<T>} = 34,
		Utility: utility::{Pallet, Call, Event} = 35,

		// XCM helpers.
		XcmpQueue: cumulus_pallet_xcmp_queue::{Pallet, Call, Storage, Event<T>} = 40,
		PolkadotXcm: pallet_xcm::{Pallet, Call, Event<T>, Origin} = 41,
		CumulusXcm: cumulus_pallet_xcm::{Pallet, Call, Event<T>, Origin} = 42,
		DmpQueue: cumulus_pallet_dmp_queue::{Pallet, Call, Storage, Event<T>} = 43,

		Oracle: oracle::{Pallet, Call, Storage, Event<T>} = 50,
		Tokens: orml_tokens::{Pallet, Call, Storage, Event<T>} = 51,
		Factory: currency_factory::{Pallet, Storage, Event<T>} = 52,
		Vault: vault::{Pallet, Call, Storage, Event<T>} = 53,
		Lending: lending::{Pallet, Call, Storage, Event<T>} = 54,
		LiquidCrowdloan: crowdloan_bonus::{Pallet, Call, Storage, Event<T>} = 55,
		Liquidations: liquidations::{Pallet, Call, Event<T>} = 56,
		Auctions: dutch_auction::{Pallet, Event<T>} = 57,
		Ping: ping::{Pallet, Call, Storage, Event<T>} = 58,
		AssetsRegistry: assets_registry::{Pallet, Call, Storage, Event<T>} = 59,

		Spambot: cumulus_ping::{Pallet, Call, Storage, Event<T>} = 90,

		CallFilter: call_filter::{Pallet, Call, Storage, Event<T>} = 100,
	}
);

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, AccountIndex>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	system::CheckSpecVersion<Runtime>,
	system::CheckTxVersion<Runtime>,
	system::CheckGenesis<Runtime>,
	system::CheckEra<Runtime>,
	system::CheckNonce<Runtime>,
	system::CheckWeight<Runtime>,
	transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive =
	executive::Executive<Runtime, Block, system::ChainContext<Runtime>, Runtime, AllPallets>;

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block);
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData,
		) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> sp_consensus_aura::SlotDuration {
			sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
		}

		fn authorities() -> Vec<AuraId> {
			Aura::authorities().into_inner()
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			opaque::SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
		fn collect_collation_info() -> cumulus_primitives_core::CollationInfo {
			ParachainSystem::collect_collation_info()
		}
	}

	impl system_rpc_runtime_api::AccountNonceApi<Block, AccountId, AccountIndex> for Runtime {
		fn account_nonce(account: AccountId) -> AccountIndex {
			System::account_nonce(account)
		}
	}

	impl transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
		fn query_info(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<benchmarking::BenchmarkList>,
			Vec<support::traits::StorageInfo>,
		) {
			use benchmarking::{list_benchmark, Benchmarking, BenchmarkList};
			use support::traits::StorageInfoTrait;
			use system_benchmarking::Pallet as SystemBench;

			let mut list = Vec::<BenchmarkList>::new();

			list_benchmark!(list, extra, frame_system, SystemBench::<Runtime>);
			list_benchmark!(list, extra, balances, Balances);
			list_benchmark!(list, extra, timestamp, Timestamp);
			list_benchmark!(list, extra, oracle, Oracle);
			list_benchmark!(list, extra, collator_selection, CollatorSelection);
			list_benchmark!(list, extra, indices, Indices);
			list_benchmark!(list, extra, membership, CouncilMembership);
			list_benchmark!(list, extra, treasury, Treasury);
			list_benchmark!(list, extra, scheduler, Scheduler);
			list_benchmark!(list, extra, democracy, Democracy);
			list_benchmark!(list, extra, collective, Council);
			list_benchmark!(list, extra, lending, Lending);
			list_benchmark!(list, extra, crowdloan_bonus, LiquidCrowdloan);
			list_benchmark!(list, extra, utility, Utility);

			let storage_info = AllPalletsWithSystem::storage_info();

			return (list, storage_info)
		}

		fn dispatch_benchmark(
			config: benchmarking::BenchmarkConfig
		) -> Result<Vec<benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};

			use system_benchmarking::Pallet as SystemBench;
			impl system_benchmarking::Config for Runtime {}

			use session_benchmarking::Pallet as SessionBench;
			impl session_benchmarking::Config for Runtime {}

			let whitelist: Vec<TrackedStorageKey> = vec![
				// Block Number
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
				// Total Issuance
				hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
				// Execution Phase
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
				// Event Count
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
				// System Events
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
			];

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);

			add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
			add_benchmark!(params, batches, balances, Balances);
			add_benchmark!(params, batches, timestamp, Timestamp);
			add_benchmark!(params, batches, oracle, Oracle);
			add_benchmark!(params, batches, session, SessionBench::<Runtime>);
			add_benchmark!(params, batches, collator_selection, CollatorSelection);
			add_benchmark!(params, batches, indices, Indices);
			add_benchmark!(params, batches, membership, CouncilMembership);
			add_benchmark!(params, batches, treasury, Treasury);
			add_benchmark!(params, batches, scheduler, Scheduler);
			add_benchmark!(params, batches, democracy, Democracy);
			add_benchmark!(params, batches, collective, Council);
			add_benchmark!(params, batches, lending, Lending);
			add_benchmark!(params, batches, crowdloan_bonus, LiquidCrowdloan);
			add_benchmark!(params, batches, utility, Utility);

			if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
			Ok(batches)
		}
	}
}

struct CheckInherents;

impl cumulus_pallet_parachain_system::CheckInherents<Block> for CheckInherents {
	fn check_inherents(
		block: &Block,
		relay_state_proof: &cumulus_pallet_parachain_system::RelayChainStateProof,
	) -> sp_inherents::CheckInherentsResult {
		let relay_chain_slot = relay_state_proof
			.read_slot()
			.expect("Could not read the relay chain slot from the proof");

		let inherent_data =
			cumulus_primitives_timestamp::InherentDataProvider::from_relay_chain_slot_and_duration(
				relay_chain_slot,
				sp_std::time::Duration::from_secs(6),
			)
			.create_inherent_data()
			.expect("Could not create the timestamp inherent data");

		inherent_data.check_extrinsics(block)
	}
}

cumulus_pallet_parachain_system::register_validate_block!(
	Runtime = Runtime,
	BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
	CheckInherents = CheckInherents,
);
