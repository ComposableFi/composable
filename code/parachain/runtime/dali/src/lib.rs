//! Test runtime.
//! Consider to set minimal values (duration, times, and limits) to be easy to test within day of
//! work. Example, use several hours, instead of several days.
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
#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "512"]

// Make the WASM binary available.
#[cfg(all(feature = "std", feature = "builtin-wasm"))]
pub const WASM_BINARY_V2: Option<&[u8]> = Some(include_bytes!(env!("DALI_RUNTIME")));
#[cfg(not(feature = "builtin-wasm"))]
pub const WASM_BINARY_V2: Option<&[u8]> = None;

extern crate alloc;

mod governance;
mod weights;
mod xcmp;

use lending::MarketId;
use orml_traits::{parameter_type_with_key, LockIdentifier};
// TODO: consider moving this to shared runtime
pub use xcmp::{MaxInstructions, UnitWeightCost};

use common::{
	governance::native::{
		EnsureRootOrHalfNativeCouncil, EnsureRootOrOneThirdNativeTechnical, NativeTreasury,
	},
	impls::DealWithFees,
	multi_existential_deposits, AccountId, AccountIndex, Address, Amount, AuraId, Balance,
	BlockNumber, BondOfferId, FinancialNftInstanceId, Hash, MaxStringSize, Moment,
	MosaicRemoteAssetId, NativeExistentialDeposit, PoolId, PriceConverter, Signature,
	AVERAGE_ON_INITIALIZE_RATIO, DAYS, HOURS, MAXIMUM_BLOCK_WEIGHT, MILLISECS_PER_BLOCK,
	NORMAL_DISPATCH_RATIO, SLOT_DURATION,
};
use composable_support::rpc_helpers::SafeRpcWrapper;
use composable_traits::{
	assets::Asset,
	defi::{CurrencyPair, Rate},
	dex::{Amm, PriceAggregate, RemoveLiquiditySimulationResult},
};
use primitives::currency::{CurrencyId, ValidateCurrencyId};
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{
		AccountIdConversion, AccountIdLookup, BlakeTwo256, Block as BlockT, Bounded, Convert,
		ConvertInto, Zero,
	},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult,
};

use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

// A few exports that help ease life for downstream crates.
pub use frame_support::{
	construct_runtime, parameter_types,
	traits::{Contains, Everything, Get, KeyOwnerProofSystem, Nothing, Randomness, StorageInfo},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
		DispatchClass, IdentityFee, Weight, WeightToFeeCoefficient, WeightToFeeCoefficients,
		WeightToFeePolynomial,
	},
	PalletId, StorageValue,
};

use codec::{Codec, Encode, EncodeLike};
use composable_traits::{account_proxy::ProxyType, fnft::FnftAccountProxyType};
use frame_support::{
	traits::{
		fungibles, ConstBool, ConstU32, EqualPrivilegeOnly, InstanceFilter, OnRuntimeUpgrade,
	},
	weights::ConstantMultiplier,
};
use frame_system as system;
use scale_info::TypeInfo;
use sp_runtime::AccountId32;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{FixedPointNumber, Perbill, Permill, Perquintill};
use sp_std::{collections::btree_map::BTreeMap, fmt::Debug, vec::Vec};
use system::{
	limits::{BlockLength, BlockWeights},
	EnsureRoot,
};
use transaction_payment::{Multiplier, TargetedFeeAdjustment};
pub use xcmp::XcmConfig;

use crate::{governance::PreimageByteDeposit, xcmp::XcmRouter};

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
	spec_name: create_runtime_str!("dali"),
	impl_name: create_runtime_str!("dali"),
	authoring_version: 1,
	// The version of the runtime specification. A full node will not attempt to use its native
	//   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
	//   `spec_version`, and `authoring_version` are the same between Wasm and native.
	// This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
	//   the compatible custom types.
	spec_version: 2402,
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

parameter_type_with_key! {
	// Minimum amount an account has to hold to stay in state
	pub MultiExistentialDeposits: |currency_id: CurrencyId| -> Balance {
		multi_existential_deposits::<AssetsRegistry>(currency_id)
	};
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
	type MaxConsumers = ConstU32<16>;
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
	pub BasicDeposit: Balance = 8 * CurrencyId::unit::<Balance>();
	pub FieldDeposit: Balance = 256 * CurrencyId::milli::<Balance>();
	pub const MaxAdditionalFields: u32 = 32;
	pub const MaxRegistrars: u32 = 8;
	pub const MaxSubAccounts: u32 = 32;
	pub SubAccountDeposit: Balance = 2 * CurrencyId::unit::<Balance>();
}

impl identity::Config for Runtime {
	type BasicDeposit = BasicDeposit;
	type Currency = Balances;
	type Event = Event;
	type FieldDeposit = FieldDeposit;
	type ForceOrigin = EnsureRoot<AccountId>;
	type MaxAdditionalFields = MaxAdditionalFields;
	type MaxRegistrars = MaxRegistrars;
	type MaxSubAccounts = MaxSubAccounts;
	type RegistrarOrigin = EnsureRoot<AccountId>;
	type Slashed = Treasury;
	type SubAccountDeposit = SubAccountDeposit;
	type WeightInfo = weights::identity::WeightInfo<Runtime>;
}

parameter_types! {
	pub DepositBase: u64 = CurrencyId::unit();
	pub DepositFactor: u64 = 32 * CurrencyId::milli::<u64>();
	pub const MaxSignatories: u16 = 100;
}

impl multisig::Config for Runtime {
	type Call = Call;
	type Currency = Balances;
	type DepositBase = DepositBase;
	type DepositFactor = DepositFactor;
	type Event = Event;
	type MaxSignatories = MaxSignatories;
	type WeightInfo = weights::multisig::WeightInfo<Runtime>;
}

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
	type Event = Event;
	type DustRemoval = Treasury;
	type ExistentialDeposit = common::NativeExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = weights::balances::WeightInfo<Runtime>;
}

parameter_types! {
	/// 1 milli-pica/byte should be fine
	pub TransactionByteFee: Balance = CurrencyId::milli::<Balance>();

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
		let p = CurrencyId::milli::<Balance>();
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
	type Event = Event;
	type OnChargeTransaction =
		transaction_payment::CurrencyAdapter<Balances, DealWithFees<Runtime, NativeTreasury>>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type WeightToFee = WeightToFee;
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
	type FeeMultiplierUpdate =
		TargetedFeeAdjustment<Self, TargetBlockFullness, AdjustmentVariable, MinimumMultiplier>;
}

pub struct TransferToTreasuryOrDrop;
impl asset_tx_payment::HandleCredit<AccountId, Tokens> for TransferToTreasuryOrDrop {
	fn handle_credit(credit: fungibles::CreditOf<AccountId, Tokens>) {
		let _ =
			<Tokens as fungibles::Balanced<AccountId>>::resolve(&TreasuryAccount::get(), credit);
	}
}

impl asset_tx_payment::Config for Runtime {
	type Fungibles = Tokens;
	type OnChargeAssetTransaction = asset_tx_payment::FungiblesAdapter<
		PriceConverter<AssetsRegistry>,
		TransferToTreasuryOrDrop,
	>;

	type UseUserConfiguration = ConstBool<true>;

	type WeightInfo = weights::asset_tx_payment::WeightInfo<Runtime>;

	type ConfigurationOrigin = EnsureRootOrHalfNativeCouncil;

	type ConfigurationExistentialDeposit = common::NativeExistentialDeposit;

	type PayableCall = Call;

	type Lock = Assets;

	type BalanceConverter = PriceConverter<AssetsRegistry>;
}

impl sudo::Config for Runtime {
	type Event = Event;
	type Call = Call;
}

parameter_types! {
	/// Deposit required to get an index.
	pub IndexDeposit: Balance = 100 * CurrencyId::unit::<Balance>();
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
			system::CheckNonZeroSender::<Runtime>::new(),
			system::CheckSpecVersion::<Runtime>::new(),
			system::CheckTxVersion::<Runtime>::new(),
			system::CheckGenesis::<Runtime>::new(),
			system::CheckEra::<Runtime>::from(era),
			system::CheckNonce::<Runtime>::from(nonce),
			system::CheckWeight::<Runtime>::new(),
			asset_tx_payment::ChargeAssetTxPayment::<Runtime>::from(tip, None),
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
	pub MinStake: Balance = 1000 * CurrencyId::unit::<Balance>();
	pub const MaxAnswerBound: u32 = 25;
	pub const MaxAssetsCount: u32 = 100_000;
	pub const MaxHistory: u32 = 20;
	pub const MaxPrePrices: u32 = 40;
	pub const TwapWindow: u16 = 3;
	// cspell:disable-next
	pub const OraclePalletId: PalletId = PalletId(*b"plt_orac");
	pub const MsPerBlock: u64 = MILLISECS_PER_BLOCK as u64;
}

impl oracle::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type Currency = Balances;
	type AssetId = CurrencyId;
	type PriceValue = Balance;
	type AuthorityId = oracle::crypto::BathurstStId;
	type MinStake = MinStake;
	type StakeLock = StakeLock;
	type StalePrice = StalePrice;
	type AddOracle = EnsureRootOrHalfNativeCouncil;
	type RewardOrigin = EnsureRootOrHalfNativeCouncil;
	type MaxAnswerBound = MaxAnswerBound;
	type MaxAssetsCount = MaxAssetsCount;
	type TreasuryAccount = TreasuryAccount;
	type MaxHistory = MaxHistory;
	type TwapWindow = TwapWindow;
	type MaxPrePrices = MaxPrePrices;
	type MsPerBlock = MsPerBlock;
	type WeightInfo = weights::oracle::WeightInfo<Runtime>;
	type LocalAssets = CurrencyFactory;
	type Moment = Moment;
	type Time = Timestamp;
	type PalletId = OraclePalletId;
}

// Parachain stuff.
// See https://github.com/paritytech/cumulus/blob/polkadot-v0.9.8/polkadot-parachains/rococo/src/lib.rs for details.
parameter_types! {
	/// 1/4 of block weight is reserved for XCMP
	pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 4;
	/// 1/4 of block weight is reserved for handling Downward messages
	pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 4;
}

impl cumulus_pallet_parachain_system::Config for Runtime {
	type Event = Event;
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

//TODO set
parameter_types! {
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
	type Event = Event;
	type Currency = Balances;
	type UpdateOrigin = EnsureRootOrHalfNativeCouncil;
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

type ReserveIdentifier = [u8; 8];
impl orml_tokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = weights::tokens::WeightInfo<Runtime>;
	type ExistentialDeposits = MultiExistentialDeposits;
	type OnDust = orml_tokens::TransferDust<Runtime, TreasuryAccount>;
	type MaxLocks = MaxLocks;
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = ConstU32<2>;
	type DustRemovalWhitelist = DustRemovalWhitelist;
	type OnNewTokenAccount = ();
	type OnKilledTokenAccount = ();
}

parameter_types! {
	pub const LiquidRewardId: PalletId = PalletId(*b"Liquided");
	pub const CrowdloanCurrencyId: CurrencyId = CurrencyId::CROWD_LOAN;
	/// total contributed to our crowdloan.
	pub const TokenTotal: Balance = 200_000_000_000_000_000;
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

parameter_types! {
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
	RuntimeBlockWeights::get().max_block;
	pub const MaxScheduledPerBlock: u32 = 50;
  pub const NoPreimagePostponement: Option<u32> = Some(10);
}

impl scheduler::Config for Runtime {
	type Event = Event;
	type Origin = Origin;
	type PalletsOrigin = OriginCaller;
	type Call = Call;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = EnsureRoot<AccountId>;
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type MaxScheduledPerBlock = MaxScheduledPerBlock;
	type PreimageProvider = Preimage;
	type NoPreimagePostponement = NoPreimagePostponement;
	type WeightInfo = scheduler::weights::SubstrateWeight<Runtime>;
}

impl utility::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = weights::utility::WeightInfo<Runtime>;
}

impl InstanceFilter<Call> for ProxyType {
	fn filter(&self, c: &Call) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::Governance => matches!(
				c,
				Call::Democracy(..) |
					Call::Council(..) | Call::TechnicalCollective(..) |
					Call::Treasury(..) | Call::Utility(..)
			),
			ProxyType::CancelProxy => {
				// TODO (vim): We might not need this
				matches!(c, Call::Proxy(pallet_account_proxy::Call::reject_announcement { .. }))
			},
		}
	}
	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
			(_, ProxyType::Any) => false,
			_ => false,
		}
	}
}

parameter_types! {
	pub MaxProxies : u32 = 4;
	pub MaxPending : u32 = 32;
	// just make dali simple to proxy
	pub ProxyPrice: Balance = 0;
}

impl pallet_account_proxy::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type Currency = Assets;
	type ProxyType = ProxyType;
	type ProxyDepositBase = ProxyPrice;
	type ProxyDepositFactor = ProxyPrice;
	type MaxProxies = MaxProxies;
	type WeightInfo = weights::account_proxy::WeightInfo<Runtime>;
	type MaxPending = MaxPending;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = ProxyPrice;
	type AnnouncementDepositFactor = ProxyPrice;
}

parameter_types! {
	pub const FnftPalletId: PalletId = PalletId(*b"pal_fnft");
}

impl pallet_fnft::Config for Runtime {
	type Event = Event;
	type MaxProperties = ConstU32<16>;
	type FinancialNftCollectionId = CurrencyId;
	type FinancialNftInstanceId = FinancialNftInstanceId;
	type ProxyType = ProxyType;
	type AccountProxy = Proxy;
	type ProxyTypeSelector = FnftAccountProxyType;
	type PalletId = FnftPalletId;
}

parameter_types! {
	pub const PreimageMaxSize: u32 = 4096 * 1024;
	pub PreimageBaseDeposit: Balance = 10 * CurrencyId::unit::<Balance>();
}

impl preimage::Config for Runtime {
	type WeightInfo = preimage::weights::SubstrateWeight<Runtime>;
	type Event = Event;
	type Currency = Balances;
	type ManagerOrigin = EnsureRoot<AccountId>;
	type MaxSize = PreimageMaxSize;
	type BaseDeposit = PreimageBaseDeposit;
	type ByteDeposit = PreimageByteDeposit;
}

parameter_types! {
	pub const MaxStrategies: usize = 255;
	pub NativeAssetId: CurrencyId = CurrencyId::PICA;
	pub CreationDeposit: Balance = 10 * CurrencyId::unit::<Balance>();
	pub VaultExistentialDeposit: Balance = 1000 * CurrencyId::unit::<Balance>();
	pub RentPerBlock: Balance = CurrencyId::milli();
	pub const VaultMinimumDeposit: Balance = 10_000;
	pub const VaultMinimumWithdrawal: Balance = 10_000;
	pub const VaultPalletId: PalletId = PalletId(*b"cubic___");
	pub const TombstoneDuration: BlockNumber = DAYS * 7;
}

impl vault::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type CurrencyFactory = CurrencyFactory;
	type AssetId = CurrencyId;
	type Currency = Assets;
	type Convert = sp_runtime::traits::ConvertInto;
	type PalletId = VaultPalletId;
	type MaxStrategies = MaxStrategies;
	type CreationDeposit = CreationDeposit;
	type ExistentialDeposit = VaultExistentialDeposit;
	type RentPerBlock = RentPerBlock;
	type NativeCurrency = Balances;
	type MinimumDeposit = VaultMinimumDeposit;
	type MinimumWithdrawal = VaultMinimumWithdrawal;
	type TombstoneDuration = TombstoneDuration;
	type VaultId = u64;
	type WeightInfo = weights::vault::WeightInfo<Runtime>;
}

impl currency_factory::Config for Runtime {
	type Event = Event;
	type AssetId = CurrencyId;
	type AddOrigin = EnsureRootOrHalfNativeCouncil;
	type WeightInfo = weights::currency_factory::WeightInfo<Runtime>;
	type Balance = Balance;
}

impl assets_registry::Config for Runtime {
	type Event = Event;
	type LocalAssetId = CurrencyId;
	type CurrencyFactory = CurrencyFactory;
	type ForeignAssetId = composable_traits::xcm::assets::XcmAssetLocation;
	type UpdateAssetRegistryOrigin = EnsureRootOrHalfNativeCouncil;
	type ParachainOrGovernanceOrigin = EnsureRootOrHalfNativeCouncil;
	type Balance = Balance;
	type WeightInfo = weights::assets_registry::WeightInfo<Runtime>;
}

impl assets::Config for Runtime {
	type NativeAssetId = NativeAssetId;
	type GenerateCurrencyId = CurrencyFactory;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type NativeCurrency = Balances;
	type MultiCurrency = Tokens;
	type WeightInfo = ();
	type AdminOrigin = EnsureRootOrHalfNativeCouncil;
	type GovernanceRegistry = GovernanceRegistry;
	type CurrencyValidator = ValidateCurrencyId;
}

parameter_types! {
	  pub const CrowdloanRewardsId: PalletId = PalletId(*b"pal_crow");
	  pub const CrowdloanRewardsLockId: LockIdentifier = *b"clr_lock";
	  pub const InitialPayment: Perbill = Perbill::from_percent(25);
	  pub const OverFundedThreshold: Perbill = Perbill::from_percent(1);
	  pub const VestingStep: Moment = 1;
	  pub const Prefix: &'static [u8] = b"picasso-";
	  pub const LockCrowdloanRewards: bool = true;
}

impl crowdloan_rewards::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type RewardAsset = Assets;
	type AdminOrigin = EnsureRootOrHalfNativeCouncil;
	type Convert = sp_runtime::traits::ConvertInto;
	type RelayChainAccountId = sp_runtime::AccountId32;
	type InitialPayment = InitialPayment;
	type OverFundedThreshold = OverFundedThreshold;
	type VestingStep = VestingStep;
	type Prefix = Prefix;
	type WeightInfo = weights::crowdloan_rewards::WeightInfo<Runtime>;
	type PalletId = CrowdloanRewardsId;
	type Moment = Moment;
	type Time = Timestamp;
	type LockId = CrowdloanRewardsLockId;
	type LockByDefault = LockCrowdloanRewards;
}

parameter_types! {
	pub const StakingRewardsPalletId : PalletId = PalletId(*b"stk_rwrd");
	pub const MaxStakingDurationPresets : u32 = 10;
	pub const MaxRewardConfigsPerPool : u32 = 10;
	pub const PicaAssetId : CurrencyId = CurrencyId::PICA;
	pub const PbloAssetId : CurrencyId = CurrencyId::PBLO;
	pub const XPicaAssetId: CurrencyId = CurrencyId::xPICA;
	pub const XPbloAssetId: CurrencyId = CurrencyId::xPBLO;
	pub const PicaStakeFinancialNftCollectionId: CurrencyId = CurrencyId::PICA_STAKE_FNFT_COLLECTION;
	pub const PbloStakeFinancialNftCollectionId: CurrencyId = CurrencyId::PBLO_STAKE_FNFT_COLLECTION;
	pub const StakingRewardsLockId: LockIdentifier = *b"stk_lock";
}

impl pallet_staking_rewards::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type AssetId = CurrencyId;
	type Assets = Assets;
	type CurrencyFactory = CurrencyFactory;
	type UnixTime = Timestamp;
	type ReleaseRewardsPoolsBatchSize = frame_support::traits::ConstU8<13>;
	type PalletId = StakingRewardsPalletId;
	type MaxStakingDurationPresets = MaxStakingDurationPresets;
	type MaxRewardConfigsPerPool = MaxRewardConfigsPerPool;
	type RewardPoolCreationOrigin = EnsureRootOrHalfNativeCouncil;
	type WeightInfo = weights::pallet_staking_rewards::WeightInfo<Runtime>;
	type RewardPoolUpdateOrigin = EnsureRootOrHalfNativeCouncil;
	type FinancialNft = Fnft;
	type FinancialNftInstanceId = FinancialNftInstanceId;
	type PicaAssetId = PicaAssetId;
	type PbloAssetId = PbloAssetId;
	type XPicaAssetId = XPicaAssetId;
	type XPbloAssetId = XPbloAssetId;
	type PicaStakeFinancialNftCollectionId = PicaStakeFinancialNftCollectionId;
	type PbloStakeFinancialNftCollectionId = PbloStakeFinancialNftCollectionId;
	type LockId = StakingRewardsLockId;
	type TreasuryAccount = TreasuryAccount;
	type ExistentialDeposits = MultiExistentialDeposits;
}

/// The calls we permit to be executed by extrinsics
// TODO(hussein-aitlahcen):
// remove IBC pallets from the call filter once centauri is merged
pub struct BaseCallFilter;
impl Contains<Call> for BaseCallFilter {
	fn contains(call: &Call) -> bool {
		!(call_filter::Pallet::<Runtime>::contains(call) ||
			matches!(
				call,
				Call::Tokens(_) |
					Call::Indices(_) | Call::Treasury(_) |
					Call::IbcPing(_) | Call::Transfer(_) |
					Call::Ibc(_)
			))
	}
}

impl call_filter::Config for Runtime {
	type Event = Event;
	type UpdateOrigin = EnsureRootOrOneThirdNativeTechnical;
	type Hook = ();
	type WeightInfo = ();
	type MaxStringSize = MaxStringSize;
}

parameter_types! {
	pub const MaxVestingSchedule: u32 = 100;
	pub MinVestedTransfer: u64 = 10 * CurrencyId::unit::<u64>();
}

impl vesting::Config for Runtime {
	type Currency = Assets;
	type Event = Event;
	type MaxVestingSchedules = MaxVestingSchedule;
	type MinVestedTransfer = MinVestedTransfer;
	type VestedTransferOrigin = EnsureRootOrHalfNativeCouncil;
	type WeightInfo = weights::vesting::WeightInfo<Runtime>;
	type Moment = Moment;
	type Time = Timestamp;
	type VestingScheduleId = u128;
}

parameter_types! {
	// cspell:disable-next
	pub const BondedFinanceId: PalletId = PalletId(*b"bondedfi");
	pub MinReward: Balance = 100 * CurrencyId::unit::<Balance>();
	pub Stake: Balance = 10 * CurrencyId::unit::<Balance>();
}

impl bonded_finance::Config for Runtime {
	type AdminOrigin = EnsureRootOrHalfNativeCouncil;
	type BondOfferId = BondOfferId;
	type Convert = sp_runtime::traits::ConvertInto;
	type Currency = Assets;
	type Event = Event;
	type MinReward = MinReward;
	type NativeCurrency = Balances;
	type PalletId = BondedFinanceId;
	type Stake = Stake;
	type Vesting = Vesting;
	type WeightInfo = weights::bonded_finance::WeightInfo<Runtime>;
}

parameter_types! {
	pub const DutchAuctionId: PalletId = PalletId(*b"dtch_ctn");
}

impl composable_traits::defi::DeFiComposableConfig for Runtime {
	type MayBeAssetId = CurrencyId;
	type Balance = Balance;
}

impl dutch_auction::Config for Runtime {
	type NativeCurrency = Balances;
	type Event = Event;
	type MultiCurrency = Assets;
	type PalletId = DutchAuctionId;
	type OrderId = u128;
	type UnixTime = Timestamp;
	type WeightInfo = weights::dutch_auction::WeightInfo<Runtime>;
	type PositionExistentialDeposit = NativeExistentialDeposit;
	type XcmOrigin = Origin;
	type AdminOrigin = EnsureRootOrHalfNativeCouncil;
	type XcmSender = XcmRouter;
}

parameter_types! {
	pub const MosaicId: PalletId = PalletId(*b"plmosaic");
	pub const MinimumTTL: BlockNumber = 10;
	pub const MinimumTimeLockPeriod: BlockNumber = 20;
}

impl mosaic::Config for Runtime {
	type Event = Event;
	type PalletId = MosaicId;
	type Assets = Assets;
	type MinimumTTL = MinimumTTL;
	type MinimumTimeLockPeriod = MinimumTimeLockPeriod;
	type BudgetPenaltyDecayer = mosaic::BudgetPenaltyDecayer<Balance, BlockNumber>;
	type NetworkId = u32;
	type RemoteAssetId = MosaicRemoteAssetId;
	type ControlOrigin = EnsureRootOrHalfNativeCouncil;
	type WeightInfo = weights::mosaic::WeightInfo<Runtime>;
	type RemoteAmmId = u128; // TODO: Swap to U256?
	type AmmMinimumAmountOut = u128;
}

pub type LiquidationStrategyId = u32;
pub type OrderId = u128;

parameter_types! {
	pub const LiquidationsPalletId: PalletId = PalletId(*b"liqdatns");
}

impl liquidations::Config for Runtime {
	type Event = Event;
	type UnixTime = Timestamp;
	type DutchAuction = DutchAuction;
	type LiquidationStrategyId = LiquidationStrategyId;
	type OrderId = OrderId;
	type WeightInfo = weights::liquidations::WeightInfo<Runtime>;
	type PalletId = LiquidationsPalletId;
	type CanModifyStrategies = EnsureRootOrHalfNativeCouncil;
	type XcmSender = XcmRouter;
	type MaxLiquidationStrategiesAmount = ConstU32<10>;
}

parameter_types! {
	pub const MaxLendingCount: u32 = 10;
	pub LendingPalletId: PalletId = PalletId(*b"liqiudat");
	pub OracleMarketCreationStake: Balance = 300;
	pub const MaxLiquidationBatchSize: u32 = 1000;
}

impl lending::Config for Runtime {
	type Event = Event;
	type Oracle = Oracle;
	type VaultId = u64;
	type Vault = Vault;
	type CurrencyFactory = CurrencyFactory;
	type MultiCurrency = Assets;
	type Liquidation = Liquidations;
	type UnixTime = Timestamp;
	type MaxMarketCount = MaxLendingCount;
	type AuthorityId = oracle::crypto::BathurstStId;
	type WeightInfo = weights::lending::WeightInfo<Runtime>;
	type LiquidationStrategyId = u32;
	type OracleMarketCreationStake = OracleMarketCreationStake;
	type PalletId = LendingPalletId;
	type NativeCurrency = Balances;
	type MaxLiquidationBatchSize = MaxLiquidationBatchSize;
	type WeightToFee = WeightToFee;
}

parameter_types! {
  pub PabloId: PalletId = PalletId(*b"pall_pab");
  pub LbpMinSaleDuration: BlockNumber = 3 * HOURS;
  pub LbpMaxSaleDuration: BlockNumber = 30 * DAYS;
  pub LbpMaxInitialWeight: Permill = Permill::from_percent(95);
  pub LbpMinFinalWeight: Permill = Permill::from_percent(5);
  pub TWAPInterval: u64 = (MILLISECS_PER_BLOCK as u64) * 10;
  pub const MaxStakingRewardPools: u32 = 10;
  pub const MillisecsPerBlock: u32 = MILLISECS_PER_BLOCK;
}

impl pablo::Config for Runtime {
	type Event = Event;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type Convert = ConvertInto;
	type CurrencyFactory = CurrencyFactory;
	type Assets = Assets;
	type PoolId = PoolId;
	type PalletId = PabloId;
	type LocalAssets = CurrencyFactory;
	type LbpMinSaleDuration = LbpMinSaleDuration;
	type LbpMaxSaleDuration = LbpMaxSaleDuration;
	type LbpMaxInitialWeight = LbpMaxInitialWeight;
	type LbpMinFinalWeight = LbpMinFinalWeight;
	type PoolCreationOrigin = EnsureRootOrHalfNativeCouncil;
	// TODO: consider making it is own origin
	type EnableTwapOrigin = EnsureRootOrHalfNativeCouncil;
	type TWAPInterval = TWAPInterval;
	type Time = Timestamp;
	type WeightInfo = weights::pablo::WeightInfo<Runtime>;
	type MaxStakingRewardPools = MaxStakingRewardPools;
	type MaxRewardConfigsPerPool = MaxRewardConfigsPerPool;
	type MaxStakingDurationPresets = MaxStakingDurationPresets;
	type ManageStaking = StakingRewards;
	type ProtocolStaking = StakingRewards;
	type MsPerBlock = MillisecsPerBlock;
	type PicaAssetId = PicaAssetId;
	type PbloAssetId = PbloAssetId;
	type XPicaAssetId = XPicaAssetId;
	type XPbloAssetId = XPbloAssetId;
	type PicaStakeFinancialNftCollectionId = PicaStakeFinancialNftCollectionId;
	type PbloStakeFinancialNftCollectionId = PbloStakeFinancialNftCollectionId;
}

parameter_types! {
	#[derive(TypeInfo, codec::MaxEncodedLen, codec::Encode)]
	pub const MaxHopsCount: u32 = 4;
	pub DexRouterPalletID: PalletId = PalletId(*b"dex_rout");
}

impl dex_router::Config for Runtime {
	type Event = Event;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type MaxHopsInRoute = MaxHopsCount;
	type PoolId = PoolId;
	type Pablo = Pablo;
	type PalletId = DexRouterPalletID;
	// TODO: consider making it is own origin
	type UpdateRouteOrigin = EnsureRootOrHalfNativeCouncil;
	type WeightInfo = weights::dex_router::WeightInfo<Runtime>;
}

parameter_types! {
	pub const ExpectedBlockTime: u64 = SLOT_DURATION;
}

#[derive(Clone)]
pub struct IbcAccount(AccountId);

impl sp_runtime::traits::IdentifyAccount for IbcAccount {
	type AccountId = AccountId;
	fn into_account(self) -> Self::AccountId {
		self.0
	}
}

impl TryFrom<pallet_ibc::Signer> for IbcAccount
where
	AccountId: From<[u8; 32]>,
{
	type Error = &'static str;

	/// Convert a signer to an IBC account.
	/// Only valid hex strings are supported for now.
	fn try_from(signer: pallet_ibc::Signer) -> Result<Self, Self::Error> {
		let acc_str = signer.as_ref();
		if acc_str.starts_with("0x") {
			match acc_str.strip_prefix("0x") {
				Some(hex_string) => TryInto::<[u8; 32]>::try_into(
					hex::decode(hex_string).map_err(|_| "Error decoding invalid hex string")?,
				)
				.map_err(|_| "Invalid account id hex string")
				.map(|acc| Self(acc.into())),
				_ => Err("Signer does not hold a valid hex string"),
			}
		}
		// Do SS58 decoding instead
		else {
			let bytes = ibc_primitives::runtime_interface::ibc::ss58_to_account_id_32(acc_str)
				.map_err(|_| "Invalid SS58 address")?;
			Ok(Self(bytes.into()))
		}
	}
}

parameter_types! {
	pub TransferPalletID: PalletId = PalletId(*b"transfer");
}

impl ibc_transfer::Config for Runtime {
	type Event = Event;
	type MultiCurrency = Assets;
	type IbcHandler = Ibc;
	type AccountIdConversion = IbcAccount;
	type AssetRegistry = AssetsRegistry;
	type CurrencyFactory = CurrencyFactory;
	type AdminOrigin = EnsureRoot<AccountId>;
	type PalletId = TransferPalletID;
	type WeightInfo = crate::weights::ibc_transfer::WeightInfo<Self>;
}

impl pallet_ibc::Config for Runtime {
	type TimeProvider = Timestamp;
	type Event = Event;
	type Currency = Balances;
	const INDEXING_PREFIX: &'static [u8] = b"ibc/";
	const CONNECTION_PREFIX: &'static [u8] = b"ibc/";
	const CHILD_TRIE_KEY: &'static [u8] = b"ibc/";
	type ExpectedBlockTime = ExpectedBlockTime;
	type WeightInfo = crate::weights::pallet_ibc::WeightInfo<Self>;
	type AdminOrigin = EnsureRoot<AccountId>;
}

impl pallet_ibc_ping::Config for Runtime {
	type Event = Event;
	type IbcHandler = Ibc;
}

/// Native <-> Cosmwasm account mapping
/// TODO(hussein-aitlahcen): Probably nicer to have SS58 representation here.
pub struct AccountToAddr;
impl Convert<alloc::string::String, Result<AccountId, ()>> for AccountToAddr {
	fn convert(a: alloc::string::String) -> Result<AccountId, ()> {
		match a.strip_prefix("0x") {
			Some(account_id) => Ok(<[u8; 32]>::try_from(hex::decode(account_id).map_err(|_| ())?)
				.map_err(|_| ())?
				.into()),
			_ => Err(()),
		}
	}
}
impl Convert<AccountId, alloc::string::String> for AccountToAddr {
	fn convert(a: AccountId) -> alloc::string::String {
		alloc::format!("0x{}", hex::encode(a))
	}
}

/// Native <-> Cosmwasm asset mapping
pub struct AssetToDenom;
impl Convert<alloc::string::String, Result<CurrencyId, ()>> for AssetToDenom {
	fn convert(currency_id: alloc::string::String) -> Result<CurrencyId, ()> {
		core::str::FromStr::from_str(&currency_id).map_err(|_| ())
	}
}
impl Convert<CurrencyId, alloc::string::String> for AssetToDenom {
	fn convert(CurrencyId(currency_id): CurrencyId) -> alloc::string::String {
		alloc::format!("{}", currency_id)
	}
}

parameter_types! {
  pub const CosmwasmPalletId: PalletId = PalletId(*b"cosmwasm");
  pub const ChainId: &'static str = "composable-network-dali";
  pub const MaxFrames: u32 = 64;
	pub const MaxCodeSize: u32 = 512 * 1024;
  pub const MaxInstrumentedCodeSize: u32 = 1024 * 1024;
  pub const MaxMessageSize: u32 = 256 * 1024;
  pub const MaxContractLabelSize: u32 = 64;
  pub const MaxContractTrieIdSize: u32 = Hash::len_bytes() as u32;
  pub const MaxInstantiateSaltSize: u32 = 128;
  pub const MaxFundsAssets: u32 = 32;
  pub const CodeTableSizeLimit: u32 = 4096;
  pub const CodeGlobalVariableLimit: u32 = 256;
  pub const CodeParameterLimit: u32 = 128;
  pub const CodeBranchTableSizeLimit: u32 = 256;
  // Not really required as it's embedded.
  pub const CodeStackLimit: u32 = u32::MAX;

  // TODO: benchmark for proper values
  pub const CodeStorageByteDeposit: u32 = 1;
  pub const ContractStorageByteReadPrice: u32 = 1;
  pub const ContractStorageByteWritePrice: u32 = 1;
}

impl cosmwasm::Config for Runtime {
	type Event = Event;
	type AccountIdExtended = AccountId;
	type PalletId = CosmwasmPalletId;
	type MaxFrames = MaxFrames;
	type MaxCodeSize = MaxCodeSize;
	type MaxInstrumentedCodeSize = MaxInstrumentedCodeSize;
	type MaxMessageSize = MaxMessageSize;
	type AccountToAddr = AccountToAddr;
	type AssetToDenom = AssetToDenom;
	type Balance = Balance;
	type AssetId = CurrencyId;
	type Assets = Assets;
	type NativeAsset = Balances;
	type ChainId = ChainId;
	type MaxContractLabelSize = MaxContractLabelSize;
	type MaxContractTrieIdSize = MaxContractTrieIdSize;
	type MaxInstantiateSaltSize = MaxInstantiateSaltSize;
	type MaxFundsAssets = MaxFundsAssets;
	type CodeTableSizeLimit = CodeTableSizeLimit;
	type CodeGlobalVariableLimit = CodeGlobalVariableLimit;
	type CodeParameterLimit = CodeParameterLimit;
	type CodeBranchTableSizeLimit = CodeBranchTableSizeLimit;
	type CodeStackLimit = CodeStackLimit;
	type CodeStorageByteDeposit = CodeStorageByteDeposit;
	type ContractStorageByteReadPrice = ContractStorageByteReadPrice;
	type ContractStorageByteWritePrice = ContractStorageByteWritePrice;
	type UnixTime = Timestamp;
	// TODO: proper weights
	type WeightInfo = cosmwasm::weights::SubstrateWeight<Runtime>;
}

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
		AssetTxPayment : asset_tx_payment  = 12,
		Indices: indices = 5,
		Balances: balances = 6,
		Identity: identity = 7,
		Multisig: multisig = 8,

		// Parachains stuff
		ParachainSystem: cumulus_pallet_parachain_system = 10,
		ParachainInfo: parachain_info = 11,

		// Collator support. the order of these 5 are important and shall not change.
		Authorship: authorship = 20,
		CollatorSelection: collator_selection = 21,
		Session: session = 22,
		Aura: aura = 23,
		AuraExt: cumulus_pallet_aura_ext = 24,

		// Runtime Native token Governance utilities
		Council: collective::<Instance1> = 30,
		CouncilMembership: membership::<Instance1> = 31,
		Treasury: treasury::<Instance1> = 32,
		Democracy: democracy::<Instance1> = 33,
		TechnicalCollective: collective::<Instance2> = 70,
		TechnicalMembership: membership::<Instance2> = 71,


		// helpers/utilities
		Scheduler: scheduler = 34,
		Utility: utility = 35,
		Preimage: preimage = 36,
		Proxy: pallet_account_proxy = 37,

		// XCM helpers.
		XcmpQueue: cumulus_pallet_xcmp_queue = 40,
		RelayerXcm: pallet_xcm = 41,
		CumulusXcm: cumulus_pallet_xcm = 42,
		DmpQueue: cumulus_pallet_dmp_queue = 43,
		XTokens: orml_xtokens = 44,
		UnknownTokens: orml_unknown_tokens = 45,

		Tokens: orml_tokens = 51,
		Oracle: oracle = 52,
		CurrencyFactory: currency_factory = 53,
		Vault: vault = 54,
		AssetsRegistry: assets_registry = 55,
		GovernanceRegistry: governance_registry = 56,
		Assets: assets = 57,
		CrowdloanRewards: crowdloan_rewards = 58,
		Vesting: vesting = 59,
		BondedFinance: bonded_finance = 60,
		DutchAuction: dutch_auction = 61,
		Mosaic: mosaic = 62,
		Liquidations: liquidations = 63,
		Lending: lending = 64,
		Pablo: pablo = 65,
		DexRouter: dex_router = 66,
		// Note the ordering below is important as staking rewards genesis
		// depends on fNFT being initialized before it.
		Fnft: pallet_fnft = 67,
		StakingRewards: pallet_staking_rewards = 68,

		CallFilter: call_filter = 140,

		// IBC Support, pallet-ibc should be the last in the list of pallets that use the ibc protocol
		IbcPing: pallet_ibc_ping = 151,
		Transfer: ibc_transfer = 152,
		Ibc: pallet_ibc = 153,

	  // Cosmwasm support
	  Cosmwasm: cosmwasm = 180
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
	asset_tx_payment::ChargeAssetTxPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;

// Migration for scheduler pallet to move from a plain Call to a CallOrHash.
pub struct SchedulerMigrationV3;
impl OnRuntimeUpgrade for SchedulerMigrationV3 {
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		Scheduler::migrate_v2_to_v3()
	}
}
/// Executive: handles dispatch to the various modules.
pub type Executive = executive::Executive<
	Runtime,
	Block,
	system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
	SchedulerMigrationV3,
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
	// TODO: broken
		// [collator_selection, CollatorSelection]
		[indices, Indices]
		[membership, CouncilMembership]
		[treasury, Treasury]
		[scheduler, Scheduler]
		[collective, Council]
		[utility, Utility]
		[identity, Identity]
		[multisig, Multisig]
		[vault, Vault]
		[vesting, Vesting]
		[oracle, Oracle]
		[dutch_auction, DutchAuction]
		[currency_factory, CurrencyFactory]
		[mosaic, Mosaic]
		[liquidations, Liquidations]
		[bonded_finance, BondedFinance]
		[lending, Lending]
		[assets_registry, AssetsRegistry]
		[pablo, Pablo]
		[pallet_staking_rewards, StakingRewards]
		[pallet_account_proxy, Proxy]
		[dex_router, DexRouter]
		[cosmwasm, Cosmwasm]
	// TODO: Broken
		// [pallet_ibc, Ibc]
		// [ibc_transfer, Transfer]
	);
}

impl_runtime_apis! {
	impl lending_runtime_api::LendingRuntimeApi<Block, MarketId> for Runtime {
		fn current_interest_rate(_market_id: MarketId) -> SafeRpcWrapper<Rate> {
			SafeRpcWrapper(
				// TODO: Actually implement this
				Rate::max_value()
				// lending::BorrowIndex::<Runtime>::get(market_id)
				// 	.unwrap_or_else(Rate::zero)
			)
		}
	}

	impl assets_runtime_api::AssetsRuntimeApi<Block, CurrencyId, AccountId, Balance> for Runtime {
		fn balance_of(SafeRpcWrapper(asset_id): SafeRpcWrapper<CurrencyId>, account_id: AccountId) -> SafeRpcWrapper<Balance> /* Balance */ {
			SafeRpcWrapper(<Assets as fungibles::Inspect::<AccountId>>::balance(asset_id, &account_id))
		}

		fn list_assets() -> Vec<Asset> {
			CurrencyId::list_assets()
		}
	}

	impl crowdloan_rewards_runtime_api::CrowdloanRewardsRuntimeApi<Block, AccountId, Balance> for Runtime {
		fn amount_available_to_claim_for(account_id: AccountId) -> SafeRpcWrapper<Balance> {
			SafeRpcWrapper (
				crowdloan_rewards::amount_available_to_claim_for::<Runtime>(account_id)
					.unwrap_or_else(|_| Balance::zero())
			)
		}
	}

	impl pablo_runtime_api::PabloRuntimeApi<Block, AccountId, PoolId, CurrencyId, Balance> for Runtime {
		fn prices_for(
			pool_id: PoolId,
			base_asset_id: CurrencyId,
			quote_asset_id: CurrencyId,
			amount: Balance
		) -> PriceAggregate<SafeRpcWrapper<PoolId>, SafeRpcWrapper<CurrencyId>, SafeRpcWrapper<Balance>> {
			pablo::prices_for::<Runtime>(
				pool_id,
				base_asset_id,
				quote_asset_id,
				amount
			)
			.map(|p| PriceAggregate{
				pool_id: SafeRpcWrapper(p.pool_id),
				base_asset_id: SafeRpcWrapper(p.base_asset_id),
				quote_asset_id: SafeRpcWrapper(p.quote_asset_id),
				spot_price: SafeRpcWrapper(p.spot_price)
			})
			.unwrap_or_else(|_| PriceAggregate{
				pool_id: SafeRpcWrapper(pool_id),
				base_asset_id: SafeRpcWrapper(base_asset_id),
				quote_asset_id: SafeRpcWrapper(quote_asset_id),
				spot_price: SafeRpcWrapper(0_u128)
			})
		}

		fn simulate_add_liquidity(
			who: SafeRpcWrapper<AccountId>,
			pool_id: SafeRpcWrapper<PoolId>,
			amounts: BTreeMap<SafeRpcWrapper<CurrencyId>, SafeRpcWrapper<Balance>>,
		) -> SafeRpcWrapper<Balance> {
			let amounts: BTreeMap<CurrencyId, Balance> = amounts.iter().map(|(k, v)| (k.0,v.0)).collect();
			SafeRpcWrapper(
				<Pablo as Amm>::simulate_add_liquidity(
					&who.0,
					pool_id.0,
					amounts,
				)
				.unwrap_or_else(|_| Zero::zero())
			)
		}

		fn simulate_remove_liquidity(
			who: SafeRpcWrapper<AccountId>,
			pool_id: SafeRpcWrapper<PoolId>,
			lp_amount: SafeRpcWrapper<Balance>,
			min_expected_amounts: BTreeMap<SafeRpcWrapper<CurrencyId>, SafeRpcWrapper<Balance>>,
		) -> RemoveLiquiditySimulationResult<SafeRpcWrapper<CurrencyId>, SafeRpcWrapper<Balance>> {
			let min_expected_amounts: BTreeMap<_, _> = min_expected_amounts.iter().map(|(k, v)| (k.0, v.0)).collect();
			let currency_pair = <Pablo as Amm>::currency_pair(pool_id.0).unwrap_or_else(|_| CurrencyPair::new(CurrencyId::INVALID, CurrencyId::INVALID));
			let lp_token = <Pablo as Amm>::lp_token(pool_id.0).unwrap_or(CurrencyId::INVALID);
			let simulate_remove_liquidity_result = <Pablo as Amm>::simulate_remove_liquidity(&who.0, pool_id.0, lp_amount.0, min_expected_amounts)
				.unwrap_or_else(|_|
					RemoveLiquiditySimulationResult{
						assets: BTreeMap::from([
									(currency_pair.base, Zero::zero()),
									(currency_pair.quote, Zero::zero()),
									(lp_token, Zero::zero())
						])
					}
				);
			let mut new_map = BTreeMap::new();
			for (k,v) in simulate_remove_liquidity_result.assets.iter() {
				new_map.insert(SafeRpcWrapper(*k), SafeRpcWrapper(*v));
			}
			RemoveLiquiditySimulationResult{
				assets: new_map
			}

		}
	}

	impl cosmwasm_runtime_api::CosmwasmRuntimeApi<Block, AccountId, CurrencyId, Balance, Vec<u8>> for Runtime {
		fn query(
			contract: AccountId,
			gas: u64,
			query_request: Vec<u8>,
		) -> Result<Vec<u8>, Vec<u8>>{
			match cosmwasm::query::<Runtime>(
				contract,
				gas,
				query_request,
			) {
				Ok(response) => Ok(response.0),
				Err(err) => Err(alloc::format!("{:?}", err).into_bytes())
			}
		}

		fn instantiate(
			instantiator: AccountId,
			code_id: u64,
			salt: Vec<u8>,
			admin: Option<AccountId>,
			label: Vec<u8>,
			funds: BTreeMap<CurrencyId, (Balance, bool)>,
			gas: u64,
			message: Vec<u8>,
		) -> Result<AccountId, Vec<u8>> {
			cosmwasm::instantiate::<Runtime>(
				instantiator,
				code_id,
				salt,
				admin,
				label,
				funds,
				gas,
				message
			).map_err(|err| alloc::format!("{:?}", err).into_bytes())
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

	impl<Call, AccountId> simnode_apis::CreateTransactionApi<Block, AccountId, Call> for Runtime
		where
			Call: Codec,
			AccountId: Codec + EncodeLike<AccountId32> + Into<AccountId32> + Clone+ PartialEq + TypeInfo + Debug,
	{
		fn create_transaction(call: Call, signer: AccountId) -> Vec<u8> {
			use sp_runtime::{
				generic::Era, MultiSignature,
				traits::StaticLookup,
			};
			use sp_core::sr25519;
			let nonce = frame_system::Pallet::<Runtime>::account_nonce(signer.clone());
			let extra = (
				system::CheckNonZeroSender::<Runtime>::new(),
				system::CheckSpecVersion::<Runtime>::new(),
				system::CheckTxVersion::<Runtime>::new(),
				system::CheckGenesis::<Runtime>::new(),
				system::CheckEra::<Runtime>::from(Era::Immortal),
				system::CheckNonce::<Runtime>::from(nonce),
				system::CheckWeight::<Runtime>::new(),
				asset_tx_payment::ChargeAssetTxPayment::<Runtime>::from(0, None),
			);
			let signature = MultiSignature::from(sr25519::Signature([0_u8;64]));
			let address = AccountIdLookup::unlookup(signer.into());
			let ext = generic::UncheckedExtrinsic::<Address, Call, Signature, SignedExtra>::new_signed(
				call,
				address,
				signature,
				extra,
			);
			ext.encode()
		}
	}

	impl ibc_runtime_api::IbcRuntimeApi<Block> for Runtime {
		fn para_id() -> u32 {
			<Runtime as cumulus_pallet_parachain_system::Config>::SelfParaId::get().into()
		}

		fn child_trie_key() -> Vec<u8> {
			<Runtime as pallet_ibc::Config>::CHILD_TRIE_KEY.to_vec()
		}

		fn query_balance_with_address(addr: Vec<u8>) -> Option<u128> {
			Ibc::query_balance_with_address(addr).ok()
		}

		fn query_packets(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<ibc_primitives::OffchainPacketType>> {
			Ibc::get_offchain_packets(channel_id, port_id, seqs).ok()
		}

		fn query_acknowledgements(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<Vec<u8>>> {
			Ibc::get_offchain_acks(channel_id, port_id, seqs).ok()
		}

		fn client_state(client_id: Vec<u8>) -> Option<ibc_primitives::QueryClientStateResponse> {
			Ibc::client(client_id).ok()
		}

		fn host_consensus_state(height: u32) -> Option<Vec<u8>> {
			Ibc::host_consensus_state(height)
		}

		fn client_consensus_state(client_id: Vec<u8>, client_height: Vec<u8>, latest_cs: bool) -> Option<ibc_primitives::QueryConsensusStateResponse> {
			Ibc::consensus_state(client_height, client_id, latest_cs).ok()
		}

		fn clients() -> Option<Vec<(Vec<u8>, Vec<u8>)>> {
			Some(Ibc::clients())
		}

		fn connection(connection_id: Vec<u8>) -> Option<ibc_primitives::QueryConnectionResponse>{
			Ibc::connection(connection_id).ok()
		}

		fn connections() -> Option<ibc_primitives::QueryConnectionsResponse> {
			Ibc::connections().ok()
		}

		fn connection_using_client(client_id: Vec<u8>) -> Option<Vec<ibc_primitives::IdentifiedConnection>>{
			Ibc::connection_using_client(client_id).ok()
		}

		fn connection_handshake(client_id: Vec<u8>, connection_id: Vec<u8>) -> Option<ibc_primitives::ConnectionHandshake> {
			Ibc::connection_handshake(client_id, connection_id).ok()
		}

		fn channel(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<ibc_primitives::QueryChannelResponse> {
			Ibc::channel(channel_id, port_id).ok()
		}

		fn channel_client(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<ibc_primitives::IdentifiedClientState> {
			Ibc::channel_client(channel_id, port_id).ok()
		}

		fn connection_channels(connection_id: Vec<u8>) -> Option<ibc_primitives::QueryChannelsResponse> {
			Ibc::connection_channels(connection_id).ok()
		}

		fn channels() -> Option<ibc_primitives::QueryChannelsResponse> {
			Ibc::channels().ok()
		}

		fn packet_commitments(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<ibc_primitives::QueryPacketCommitmentsResponse> {
			Ibc::packet_commitments(channel_id, port_id).ok()
		}

		fn packet_acknowledgements(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<ibc_primitives::QueryPacketAcknowledgementsResponse>{
			Ibc::packet_acknowledgements(channel_id, port_id).ok()
		}

		fn unreceived_packets(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<u64>> {
			Ibc::unreceived_packets(channel_id, port_id, seqs).ok()
		}

		fn unreceived_acknowledgements(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<u64>> {
			Ibc::unreceived_acknowledgements(channel_id, port_id, seqs).ok()
		}

		fn next_seq_recv(channel_id: Vec<u8>, port_id: Vec<u8>) -> Option<ibc_primitives::QueryNextSequenceReceiveResponse> {
			Ibc::next_seq_recv(channel_id, port_id).ok()
		}

		fn packet_commitment(channel_id: Vec<u8>, port_id: Vec<u8>, seq: u64) -> Option<ibc_primitives::QueryPacketCommitmentResponse> {
			Ibc::packet_commitment(channel_id, port_id, seq).ok()
		}

		fn packet_acknowledgement(channel_id: Vec<u8>, port_id: Vec<u8>, seq: u64) -> Option<ibc_primitives::QueryPacketAcknowledgementResponse> {
			Ibc::packet_acknowledgement(channel_id, port_id, seq).ok()
		}

		fn packet_receipt(channel_id: Vec<u8>, port_id: Vec<u8>, seq: u64) -> Option<ibc_primitives::QueryPacketReceiptResponse> {
			Ibc::packet_receipt(channel_id, port_id, seq).ok()
		}

		fn denom_trace(asset_id: u128) -> Option<ibc_primitives::QueryDenomTraceResponse> {
			Transfer::get_denom_trace(asset_id)
		}

		fn denom_traces(key: Option<u128>, offset: Option<u32>, limit: u64, count_total: bool) -> ibc_primitives::QueryDenomTracesResponse {
			Transfer::get_denom_traces(key, offset, limit, count_total)
		}

		fn block_events(extrinsic_index: Option<u32>) -> Vec<pallet_ibc::events::IbcEvent> {
			let mut raw_events = frame_system::Pallet::<Self>::read_events_no_consensus().into_iter();
			if let Some(idx) = extrinsic_index {
				raw_events.find_map(|e| {
					let frame_system::EventRecord{ event, phase, ..} = *e;
					match (event, phase) {
						(Event::Ibc(pallet_ibc::Event::IbcEvents{ events }), frame_system::Phase::ApplyExtrinsic(index)) if index == idx => Some(events),
						_ => None
					}
				}).unwrap_or_default()
			}
			else {
				raw_events.filter_map(|e| {
					let frame_system::EventRecord{ event, ..} = *e;

					match event {
						Event::Ibc(pallet_ibc::Event::IbcEvents{ events }) => {
								Some(events)
							},
						_ => None
					}
				}).flatten().collect()
			}
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
			use system_benchmarking::Pallet as SystemBench;
			use session_benchmarking::Pallet as SessionBench;

			let mut list = Vec::<BenchmarkList>::new();
			list_benchmarks!(list, extra);
			let storage_info = AllPalletsWithSystem::storage_info();
			return (list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{Benchmarking, BenchmarkBatch, TrackedStorageKey};

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
