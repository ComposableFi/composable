#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		// // impl_runtime_apis will generate code that contains a `panic!`. Implementations should still avoid using panics.
		// clippy::panic
	)
)]
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available
// TODO: XCMP/governance/.. setups here are outdated
#[cfg(all(feature = "std", feature = "builtin-wasm"))]
pub const WASM_BINARY_V2: Option<&[u8]> = Some(include_bytes!(env!("COMPOSABLE_RUNTIME")));
#[cfg(not(feature = "builtin-wasm"))]
pub const WASM_BINARY_V2: Option<&[u8]> = None;

mod weights;
mod xcmp;

use common::{
	rewards::StakingPot, AccountId, AccountIndex, Address, Amount, AuraId, Balance, BlockNumber,
	ForeignAssetId, Hash, Moment, Signature, AVERAGE_ON_INITIALIZE_RATIO, DAYS, HOURS,
	MAXIMUM_BLOCK_WEIGHT, MILLISECS_PER_BLOCK, NORMAL_DISPATCH_RATIO, SLOT_DURATION,
};
use composable_support::rpc_helpers::SafeRpcWrapper;
use composable_traits::assets::Asset;
use orml_traits::parameter_type_with_key;
use primitives::currency::{CurrencyId, ValidateCurrencyId};
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{AccountIdConversion, AccountIdLookup, BlakeTwo256, Block as BlockT, Zero},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;
// A few exports that help ease life for downstream crates.
use codec::Encode;
pub use frame_support::{
	construct_runtime,
	pallet_prelude::*,
	parameter_types,
	traits::{
		Contains, EitherOfDiverse, Everything, KeyOwnerProofSystem, LockIdentifier, Nothing,
		Randomness, StorageInfo,
	},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
		IdentityFee, Weight, WeightToFeeCoefficient, WeightToFeeCoefficients,
		WeightToFeePolynomial,
	},
	PalletId, StorageValue,
};
use frame_support::{
	traits::{EqualPrivilegeOnly, OnRuntimeUpgrade},
	weights::ConstantMultiplier,
};
use frame_system as system;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{FixedPointNumber, Perbill, Permill, Perquintill};
use system::{
	limits::{BlockLength, BlockWeights},
	EnsureRoot,
};
use transaction_payment::{Multiplier, TargetedFeeAdjustment};
pub type CouncilInstance = collective::Instance1;
pub type EnsureRootOrHalfCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	collective::EnsureProportionAtLeast<AccountId, CouncilInstance, 1, 2>,
>;

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
	spec_name: create_runtime_str!("composable"),
	impl_name: create_runtime_str!("composable"),
	authoring_version: 1,
	// The version of the runtime specification. A full node will not attempt to use its native
	//   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
	//   `spec_version`, and `authoring_version` are the same between Wasm and native.
	spec_version: 10_005,
	impl_version: 3,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 0,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

parameter_types! {
	// how much block hashes to keep
	pub const BlockHashCount: BlockNumber = 250;
	pub const Version: RuntimeVersion = VERSION;
	// 5mb with 25% of that reserved for system extrinsics.
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

	pub const SS58Prefix: u8 = 50;
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
	type RuntimeCall = RuntimeCall;
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
	type RuntimeEvent = RuntimeEvent;
	/// The ubiquitous origin type.
	type RuntimeOrigin = RuntimeOrigin;
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
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl randomness_collective_flip::Config for Runtime {}

parameter_types! {
	// Maximum authorities/collators for aura
	pub const MaxAuthorities: u32 = 100;
}

impl aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = MaxAuthorities;
}

impl cumulus_pallet_aura_ext::Config for Runtime {}

parameter_types! {
	/// Minimum period in between blocks, for now we leave it at half
	/// the expected slot duration
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the Unix epoch.
	type Moment = Moment;
	/// What to do when SLOT_DURATION has passed?
	type OnTimestampSet = Aura;
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = weights::timestamp::WeightInfo<Runtime>;
}

parameter_types! {
	/// Minimum amount an account has to hold to stay in state.
	// minimum account balance is given as 0.1 PICA ~ 100 CurrencyId::milli()
	pub ExistentialDeposit: Balance = 100 * CurrencyId::milli::<Balance>();
	/// Max locks that can be placed on an account. Capped for storage
	/// concerns.
	pub const MaxLocks: u32 = 50;
}

impl balances::Config for Runtime {
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = Treasury;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = weights::balances::WeightInfo<Runtime>;
}

parameter_types! {
	/// 1 milli-pica/byte should be fine
	pub TransactionByteFee: Balance = CurrencyId::milli();

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
	pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000_u128);
	pub const OperationalFeeMultiplier: u8 = 5;
}

pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
	type Balance = Balance;
	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		let p = CurrencyId::milli::<Balance>();
		let q = 10 * Balance::from(ExtrinsicBaseWeight::get().ref_time());
		smallvec::smallvec![WeightToFeeCoefficient {
			degree: 1,
			negative: false,
			coeff_frac: Perbill::from_rational(p % q, q),
			coeff_integer: p / q,
		}]
	}
}

type NativeTreasury = treasury::Instance1;

impl transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction =
		transaction_payment::CurrencyAdapter<Balances, StakingPot<Runtime, NativeTreasury>>;
	type WeightToFee = WeightToFee;
	type FeeMultiplierUpdate =
		TargetedFeeAdjustment<Self, TargetBlockFullness, AdjustmentVariable, MinimumMultiplier>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
}

impl sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
}

parameter_types! {
	/// Deposit required to get an index.
	pub IndexDeposit: Balance = 100 * CurrencyId::unit::<Balance>();
}

impl indices::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AccountIndex = AccountIndex;
	type Currency = Balances;
	type Deposit = IndexDeposit;
	type WeightInfo = weights::indices::WeightInfo<Runtime>;
}

pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;

impl<LocalCall> system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
	RuntimeCall: From<LocalCall>,
{
	fn create_transaction<C: system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: RuntimeCall,
		public: <Signature as sp_runtime::traits::Verify>::Signer,
		account: AccountId,
		nonce: AccountIndex,
	) -> Option<(
		RuntimeCall,
		<UncheckedExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload,
	)> {
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
			system::CheckNonZeroSender::<Runtime>::new(),
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
	RuntimeCall: From<C>,
{
	type OverarchingCall = RuntimeCall;
	type Extrinsic = UncheckedExtrinsic;
}

//TODO set
parameter_types! {
	pub const StakeLock: BlockNumber = 50;
	pub const StalePrice: BlockNumber = 5;

	/// TODO: discuss with omar/cosmin
	pub MinStake: Balance = 1000 * CurrencyId::unit::<Balance>();
	pub const MaxAnswerBound: u32 = 25;
	pub const MaxAssetsCount: u32 = 100_000;
	pub const MaxHistory: u32 = 20;
}

// Parachain stuff.
// See https://github.com/paritytech/cumulus/blob/polkadot-v0.9.8/polkadot-parachains/rococo/src/lib.rs for details.
parameter_types! {
	/// 1/4 of block weight is reserved for XCMP
	pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
	/// 1/4 of block weight is reserved for handling Downward messages
	pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
}

impl cumulus_pallet_parachain_system::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnSystemEvent = ();
	type SelfParaId = parachain_info::Pallet<Runtime>;
	type OutboundXcmpMessageSource = XcmpQueue;
	type DmpMessageHandler = DmpQueue;
	type ReservedDmpWeight = ReservedDmpWeight;
	type XcmpMessageHandler = XcmpQueue;
	type ReservedXcmpWeight = ReservedXcmpWeight;
	type CheckAssociatedRelayNumber = cumulus_pallet_parachain_system::RelayNumberStrictlyIncreases;
}

impl parachain_info::Config for Runtime {}

parameter_types! {
	pub const UncleGenerations: u32 = 0;
}

impl authorship::Config for Runtime {
	type FindAuthor = session::FindAccountFromAuthorIndex<Self, Aura>;
	type UncleGenerations = UncleGenerations;
	type FilterUncle = ();
	type EventHandler = (CollatorSelection,);
}

parameter_types! {
	pub const Period: u32 = 6 * HOURS;
	pub const Offset: u32 = 0;
}

impl session::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
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
	type WeightInfo = weights::session::WeightInfo<Runtime>;
}

parameter_types! {
	/// Lifted from Statemine:
	/// https://github.com/paritytech/cumulus/blob/935bac869a72baef17e46d2ae1abc8c0c650cef5/polkadot-parachains/statemine/src/lib.rs?#L666-L672
	pub const PotId: PalletId = PalletId(*b"PotStake");
	pub const MaxCandidates: u32 = 1000;
	pub const SessionLength: BlockNumber = 6 * HOURS;
	pub const MaxInvulnerables: u32 = 100;
	pub const MinCandidates: u32 = 5;
}

impl collator_selection::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
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
	type WeightInfo = weights::collator_selection::WeightInfo<Runtime>;
}

impl assets::Config for Runtime {
	type NativeAssetId = NativeAssetId;
	type GenerateCurrencyId = CurrencyFactory;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type NativeCurrency = Balances;
	type MultiCurrency = Tokens;
	type WeightInfo = ();
	type AdminOrigin = EnsureRootOrHalfCouncil;
	type GovernanceRegistry = GovernanceRegistry;
	type CurrencyValidator = ValidateCurrencyId;
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
		let account: AccountId = TreasuryPalletId::get().into_account_truncating();
		let account2: AccountId = PotId::get().into_account_truncating();
		vec![&account, &account2].contains(&a)
	}
}

parameter_types! {
	pub TreasuryAccount: AccountId = TreasuryPalletId::get().into_account_truncating();
}

pub struct CurrencyHooks;
impl orml_traits::currency::MutationHooks<AccountId, CurrencyId, Balance> for CurrencyHooks {
	type OnDust = orml_tokens::TransferDust<Runtime, TreasuryAccount>;
	type OnSlash = ();
	type PreDeposit = ();
	type PostDeposit = ();
	type PreTransfer = ();
	type PostTransfer = ();
	type OnNewTokenAccount = ();
	type OnKilledTokenAccount = ();
}

type ReserveIdentifier = [u8; 8];
impl orml_tokens::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = weights::tokens::WeightInfo<Runtime>;
	type ExistentialDeposits = ExistentialDeposits;
	type MaxLocks = MaxLocks;
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = frame_support::traits::ConstU32<2>;
	type DustRemovalWhitelist = DustRemovalWhitelist;
	type CurrencyHooks = CurrencyHooks;
}

parameter_types! {
	pub const TreasuryPalletId: PalletId = PalletId(*b"picatrsy");
	/// percentage of proposal that most be bonded by the proposer
	pub const ProposalBond: Permill = Permill::from_percent(5);
	// TODO: rationale?
	pub ProposalBondMinimum: Balance = 5 * CurrencyId::unit::<Balance>();
	pub ProposalBondMaximum: Balance = 1000 * CurrencyId::unit::<Balance>();
	pub const SpendPeriod: BlockNumber = 7 * DAYS;
	pub const Burn: Permill = Permill::from_percent(0);

	pub const MaxApprovals: u32 = 30;
}

impl treasury::Config<NativeTreasury> for Runtime {
	type PalletId = TreasuryPalletId;
	type Currency = Balances;
	type ApproveOrigin = EnsureRootOrHalfCouncil;
	type RejectOrigin = EnsureRootOrHalfCouncil;
	type RuntimeEvent = RuntimeEvent;
	type OnSlash = Treasury;
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type ProposalBondMaximum = ProposalBondMaximum;
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
	type MaxApprovals = MaxApprovals;
	type BurnDestination = ();
	type WeightInfo = weights::treasury::WeightInfo<Runtime>;
	// TODO: add bounties?
	type SpendFunds = ();
	type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>;
}

parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = 7 * DAYS;
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
}

impl membership::Config<membership::Instance1> for Runtime {
	type RuntimeEvent = RuntimeEvent;
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
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
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
  pub const NoPreimagePostponement: Option<u32> = Some(10);
}

impl scheduler::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type PalletsOrigin = OriginCaller;
	type RuntimeCall = RuntimeCall;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = EnsureRoot<AccountId>;
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type MaxScheduledPerBlock = MaxScheduledPerBlock;
	type Preimages = Preimage;
	type WeightInfo = weights::scheduler::WeightInfo<Runtime>;
}

parameter_types! {
	pub const PreimageMaxSize: u32 = 4096 * 1024;
	pub PreimageBaseDeposit: Balance = 10 * CurrencyId::unit::<Balance>();
}

impl preimage::Config for Runtime {
	type WeightInfo = preimage::weights::SubstrateWeight<Runtime>;
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type ManagerOrigin = EnsureRoot<AccountId>;
	type BaseDeposit = PreimageBaseDeposit;
	type ByteDeposit = PreimageByteDeposit;
}

impl utility::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = weights::utility::WeightInfo<Runtime>;
}

impl governance_registry::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AssetId = CurrencyId;
	type WeightInfo = ();
}

impl currency_factory::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AssetId = CurrencyId;
	type AddOrigin = EnsureRootOrHalfCouncil;
	type WeightInfo = weights::currency_factory::WeightInfo<Runtime>;
	type Balance = Balance;
}

parameter_types! {
	pub const LaunchPeriod: BlockNumber = 5 * DAYS;
	pub const VotingPeriod: BlockNumber = 5 * DAYS;
	pub const FastTrackVotingPeriod: BlockNumber = 3 * HOURS;

	pub MinimumDeposit: Balance = 100 * CurrencyId::unit::<Balance>();
	pub const EnactmentPeriod: BlockNumber = 2 * DAYS;
	pub const CooloffPeriod: BlockNumber = 7 * DAYS;
	// TODO: prod value
	pub PreimageByteDeposit: Balance = CurrencyId::milli();
	pub const InstantAllowed: bool = true;
	pub const MaxVotes: u32 = 100;
	pub const MaxProposals: u32 = 100;
	pub const DemocracyId: LockIdentifier = *b"democrac";
	pub RootOrigin: RuntimeOrigin = frame_system::RawOrigin::Root.into();
}

impl democracy::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
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
	type Slash = Treasury;

	type CooloffPeriod = CooloffPeriod;
	type MaxProposals = MaxProposals;
	type MaxVotes = MaxVotes;
	type PalletsOrigin = OriginCaller;

	type Preimages = Preimage;
	type MaxDeposits = ConstU32<100>;
	type MaxBlacklisted = ConstU32<100>;

	type Scheduler = Scheduler;
	type WeightInfo = democracy::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	  pub const CrowdloanRewardsId: PalletId = PalletId(*b"pal_crow");
	  pub const CrowdloanRewardsLockId: LockIdentifier = *b"clr_lock";
	  pub const InitialPayment: Perbill = Perbill::from_percent(25);
	  pub const OverFundedThreshold: Perbill = Perbill::from_percent(1);
	  pub const VestingStep: Moment = (7 * DAYS as Moment) * (MILLISECS_PER_BLOCK as Moment);
	  pub const Prefix: &'static [u8] = b"composable-";
	  pub const LockCrowdloanRewards: bool = false;
}

impl crowdloan_rewards::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type RewardAsset = Assets;
	type AdminOrigin = EnsureRootOrHalfCouncil;
	type Convert = sp_runtime::traits::ConvertInto;
	type RelayChainAccountId = [u8; 32];
	type InitialPayment = InitialPayment;
	type OverFundedThreshold = OverFundedThreshold;
	type VestingStep = VestingStep;
	type Prefix = Prefix;
	type WeightInfo = ();
	type PalletId = CrowdloanRewardsId;
	type Moment = Moment;
	type Time = Timestamp;
	type LockId = CrowdloanRewardsLockId;
	type LockByDefault = LockCrowdloanRewards;
}

parameter_types! {
	pub const MaxStrategies: usize = 255;
	pub NativeAssetId: CurrencyId = CurrencyId::PICA;
	pub CreationDeposit: Balance = 10 * CurrencyId::unit::<Balance>();
	pub VaultExistentialDeposit: Balance = 1000 * CurrencyId::unit::<Balance>();
	pub RentPerBlock: Balance = CurrencyId::milli::<Balance>();
	pub const VaultMinimumDeposit: Balance = 10_000;
	pub const VaultMinimumWithdrawal: Balance = 10_000;
	pub const VaultPalletId: PalletId = PalletId(*b"cubic___");
}

/// The calls we permit to be executed by extrinsics
pub struct BaseCallFilter;

impl Contains<RuntimeCall> for BaseCallFilter {
	fn contains(call: &RuntimeCall) -> bool {
		// TODO: not up to date as whole composable setup
		!matches!(
			call,
			RuntimeCall::Tokens(_) |
				RuntimeCall::Indices(_) |
				RuntimeCall::Democracy(_) |
				RuntimeCall::Treasury(_)
		)
	}
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: system = 0,
		Timestamp: timestamp = 1,
		Sudo: sudo = 2,
		RandomnessCollectiveFlip: randomness_collective_flip = 3,
		TransactionPayment: transaction_payment = 4,
		Indices: indices = 5,
		Balances: balances = 6,

		// Parachains stuff
		ParachainSystem: cumulus_pallet_parachain_system = 10,
		ParachainInfo: parachain_info = 11,

		// Collator support. the order of these 5 are important and shall not change.
		Authorship: authorship = 20,
		CollatorSelection: collator_selection = 21,
		Session: session = 22,
		Aura: aura = 23,
		AuraExt: cumulus_pallet_aura_ext = 24,

		// Governance utilities
		Council: collective::<Instance1> = 30,
		CouncilMembership: membership::<Instance1> = 31,
		Treasury: treasury::<Instance1> = 32,
		Democracy: democracy = 33,
		Scheduler: scheduler = 34,
		Utility: utility = 35,
		Preimage: preimage = 36,

		// XCM helpers.
		XcmpQueue: cumulus_pallet_xcmp_queue = 40,
		RelayerXcm: pallet_xcm = 41,
		CumulusXcm: cumulus_pallet_xcm = 42,
		DmpQueue: cumulus_pallet_dmp_queue = 43,

		Tokens: orml_tokens = 52,

		CurrencyFactory: currency_factory = 53,
		CrowdloanRewards: crowdloan_rewards = 56,
		Assets: assets = 57,
		GovernanceRegistry: governance_registry = 58,
	}
);

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	system::CheckNonZeroSender<Runtime>,
	system::CheckSpecVersion<Runtime>,
	system::CheckTxVersion<Runtime>,
	system::CheckGenesis<Runtime>,
	system::CheckEra<Runtime>,
	system::CheckNonce<Runtime>,
	system::CheckWeight<Runtime>,
	transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

// Migration for scheduler pallet to move from a plain RuntimeCall to a CallOrHash.
pub struct SchedulerMigrationV1toV4;
impl OnRuntimeUpgrade for SchedulerMigrationV1toV4 {
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		Scheduler::migrate_v1_to_v4()
	}
}
/// Executive: handles dispatch to the various modules.
pub type Executive = executive::Executive<
	Runtime,
	Block,
	system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
	(
		SchedulerMigrationV1toV4,
		preimage::migration::v1::Migration<Runtime>,
		scheduler::migration::v3::MigrateToV4<Runtime>,
		democracy::migrations::v1::Migration<Runtime>,
	),
>;

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
	use frame_benchmarking::define_benchmarks;
	define_benchmarks!(
		[frame_system, SystemBench::<Runtime>]
		[balances, Balances]
		[session, SessionBench::<Runtime>]
		[timestamp, Timestamp]
	// TODO(hussein) Still broken on v0.9.30
		// [collator_selection, CollatorSelection]
		[indices, Indices]
		[membership, CouncilMembership]
		[treasury, Treasury]
		[scheduler, Scheduler]
		[collective, Council]
		[utility, Utility]
	[democracy, Democracy]
	);
}

impl_runtime_apis! {
	impl assets_runtime_api::AssetsRuntimeApi<Block, CurrencyId, AccountId, Balance, ForeignAssetId> for Runtime {
		fn balance_of(SafeRpcWrapper(asset_id): SafeRpcWrapper<CurrencyId>, account_id: AccountId) -> SafeRpcWrapper<Balance> /* Balance */ {
			SafeRpcWrapper(<Assets as frame_support::traits::fungibles::Inspect::<AccountId>>::balance(asset_id, &account_id))
		}

		fn list_assets() -> Vec<Asset<Balance, ForeignAssetId>> {
			CurrencyId::list_assets()
		}
	}

	impl crowdloan_rewards_runtime_api::CrowdloanRewardsRuntimeApi<Block, AccountId, Balance> for Runtime {
		fn amount_available_to_claim_for(account_id: AccountId) -> SafeRpcWrapper<Balance> {
			SafeRpcWrapper(
				crowdloan_rewards::amount_available_to_claim_for::<Runtime>(account_id)
					.unwrap_or_else(|_| Balance::zero())
			)
		}
	}

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
		fn collect_collation_info(header: &<Block as BlockT>::Header) -> cumulus_primitives_core::CollationInfo {
			ParachainSystem::collect_collation_info(header)
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
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use frame_system_benchmarking::Pallet as SystemBench;
			use session_benchmarking::Pallet as SessionBench;

			let mut list = Vec::<BenchmarkList>::new();
			list_benchmarks!(list, extra);
			let storage_info = AllPalletsWithSystem::storage_info();
			(list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{Benchmarking, BenchmarkBatch, TrackedStorageKey};

			use frame_system_benchmarking::Pallet as SystemBench;
			impl frame_system_benchmarking::Config for Runtime {}

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
			add_benchmarks!(params, batches);
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
