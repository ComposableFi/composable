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
#![recursion_limit = "512"]
#![allow(incomplete_features)] // see other usage -
#![feature(adt_const_params)]

// Make the WASM binary available
#[cfg(all(feature = "std", feature = "builtin-wasm"))]
pub const WASM_BINARY_V2: Option<&[u8]> = Some(include_bytes!(env!("PICASSO_RUNTIME")));
#[cfg(not(feature = "builtin-wasm"))]
pub const WASM_BINARY_V2: Option<&[u8]> = None;

extern crate alloc;

mod fees;
pub mod governance;
pub mod ibc;
mod migrations;
mod prelude;
pub mod version;
mod weights;
pub mod xcmp;
pub use common::xcmp::{MaxInstructions, UnitWeightCost};
pub use fees::{AssetsPaymentHeader, FinalPriceConverter};
use frame_support::dispatch::DispatchError;
use version::{Version, VERSION};
pub use xcmp::XcmConfig;

pub use crate::fees::WellKnownForeignToNativePriceConverter;

use common::{
	fees::{multi_existential_deposits, NativeExistentialDeposit, WeightToFeeConverter},
	governance::native::*,
	rewards::StakingPot,
	AccountId, AccountIndex, Address, Amount, AuraId, Balance, BlockNumber, BondOfferId, Hash,
	Moment, PoolId, ReservedDmpWeight, ReservedXcmpWeight, Signature, AVERAGE_ON_INITIALIZE_RATIO,
	DAYS, HOURS, MAXIMUM_BLOCK_WEIGHT, MILLISECS_PER_BLOCK, NORMAL_DISPATCH_RATIO, SLOT_DURATION,
};
use composable_support::rpc_helpers::SafeRpcWrapper;
use composable_traits::{
	assets::Asset,
	dex::{Amm, PriceAggregate},
};
use primitives::currency::ForeignAssetId;

mod gates;
use gates::*;
use governance::*;
use prelude::*;
use primitives::currency::{CurrencyId, ValidateCurrencyId};
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
	generic, impl_opaque_keys,
	traits::{
		AccountIdConversion, AccountIdLookup, BlakeTwo256, Block as BlockT, ConvertInto, Zero,
	},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, Either, FixedI128,
};
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

// A few exports that help ease life for downstream crates.
use codec::Encode;
use frame_support::traits::{fungibles, EqualPrivilegeOnly, OnRuntimeUpgrade};

pub use frame_support::{
	construct_runtime,
	pallet_prelude::DispatchClass,
	parameter_types,
	traits::{
		ConstBool, ConstU128, ConstU16, ConstU32, Everything, KeyOwnerProofSystem, Nothing,
		Randomness, StorageInfo,
	},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight},
		ConstantMultiplier, IdentityFee, Weight, WeightToFeeCoefficient, WeightToFeeCoefficients,
		WeightToFeePolynomial,
	},
	PalletId, StorageValue,
};
use frame_system as system;
pub use governance::TreasuryAccount;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{FixedPointNumber, Perbill, Permill, Perquintill};
use system::{
	limits::{BlockLength, BlockWeights},
	EnsureRoot,
};

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

use orml_traits::{parameter_type_with_key, LockIdentifier};
parameter_type_with_key! {
	// Minimum amount an account has to hold to stay in state
	pub MultiExistentialDeposits: |currency_id: CurrencyId| -> Balance {
		multi_existential_deposits::<AssetsRegistry, WellKnownForeignToNativePriceConverter>(currency_id)
	};
}

parameter_types! {
	// how much block hashes to keep
	pub const BlockHashCount: BlockNumber = 250;
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
	type MaxConsumers = ConstU32<16>;
}

impl randomness_collective_flip::Config for Runtime {}

parameter_types! {
	pub NativeAssetId: CurrencyId = CurrencyId::PICA;
}

impl assets_registry::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type LocalAssetId = CurrencyId;
	type Balance = Balance;
	type ForeignAssetId = primitives::currency::ForeignAssetId;
	type UpdateAssetRegistryOrigin = EnsureRootOrTwoThirdNativeCouncil;
	type ParachainOrGovernanceOrigin = EnsureRootOrTwoThirdNativeCouncil;
	type WeightInfo = weights::assets_registry::WeightInfo<Runtime>;
	type Convert = ConvertInto;
}

parameter_types! {
	pub PabloPalletId: PalletId = PalletId(*b"pal_pblo");
	pub TWAPInterval: u64 = (MILLISECS_PER_BLOCK as u64) * 10;
	pub LPTokenExistentialDeposit: Balance = 100;
}

impl pablo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type Convert = sp_runtime::traits::ConvertInto;
	type Assets = AssetsTransactorRouter;
	type LPTokenFactory = AssetsTransactorRouter;
	type PoolId = PoolId;
	type PalletId = PabloPalletId;
	type PoolCreationOrigin = EnsureRootOrTwoThirdNativeCouncil;
	type EnableTwapOrigin = EnsureRootOrTwoThirdNativeCouncil;
	type Time = Timestamp;
	type TWAPInterval = TWAPInterval;
	type WeightInfo = weights::pablo::WeightInfo<Runtime>;
	type LPTokenExistentialDeposit = LPTokenExistentialDeposit;
}

impl assets_transactor_router::Config for Runtime {
	type NativeAssetId = NativeAssetId;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type NativeTransactor = Balances;
	type LocalTransactor = Tokens;
	type ForeignTransactor = Tokens;
	type WeightInfo = ();
	type AdminOrigin = EnsureRootOrHalfNativeCouncil;
	type GovernanceRegistry = GovernanceRegistry;
	type AssetLocation = primitives::currency::ForeignAssetId;
	type AssetsRegistry = AssetsRegistry;
}

impl assets::Config for Runtime {
	type NativeAssetId = NativeAssetId;
	type GenerateCurrencyId = CurrencyFactory;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type NativeCurrency = Balances;
	type MultiCurrency = Tokens;
	type WeightInfo = ();
	type AdminOrigin = EnsureRootOrTwoThirdNativeCouncil;
	type GovernanceRegistry = GovernanceRegistry;
	type CurrencyValidator = ValidateCurrencyId;
}

type FarmingRewardsInstance = reward::Instance1;

impl reward::Config<FarmingRewardsInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SignedFixedPoint = FixedI128;
	type PoolId = CurrencyId;
	type StakeId = AccountId;
	type CurrencyId = CurrencyId;
}

parameter_types! {
	pub const RewardPeriod: BlockNumber = 5; //1 minute
	pub const FarmingPalletId: PalletId = PalletId(*b"mod/farm");
	pub FarmingAccount: AccountId = FarmingPalletId::get().into_account_truncating();
}

impl farming::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AssetId = CurrencyId;
	type FarmingPalletId = FarmingPalletId;
	type TreasuryAccountId = FarmingAccount;
	type RewardPeriod = RewardPeriod;
	type RewardPools = FarmingRewards;
	type MultiCurrency = AssetsTransactorRouter;
	type WeightInfo = ();
}

parameter_types! {
	pub const StakeLock: BlockNumber = 50;
	pub const StalePrice: BlockNumber = 5;

	// TODO
	pub MinStake: Balance = 200_000 * CurrencyId::unit::<Balance>();
	pub const MinAnswerBound: u32 = 7;
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
	type RuntimeEvent = RuntimeEvent;
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
	type MinAnswerBound = MinAnswerBound;
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
	type RuntimeEvent = RuntimeEvent;
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
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type DepositBase = DepositBase;
	type DepositFactor = DepositFactor;
	type RuntimeEvent = RuntimeEvent;
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

type MaxLocks = ConstU32<50>;

impl balances::Config for Runtime {
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = Treasury;
	type ExistentialDeposit = NativeExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = weights::balances::WeightInfo<Runtime>;
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
			AssetsPaymentHeader::from(tip, None),
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

impl authorship::Config for Runtime {
	type FindAuthor = session::FindAccountFromAuthorIndex<Self, Aura>;
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
	type UpdateOrigin = EnsureRootOrTwoThirdNativeCouncil;
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
	type ExistentialDeposits = MultiExistentialDeposits;
	type MaxLocks = MaxLocks;
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = ConstU32<2>;
	type DustRemovalWhitelist = DustRemovalWhitelist;
	type CurrencyHooks = CurrencyHooks;
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

impl preimage::Config for Runtime {
	type WeightInfo = preimage::weights::SubstrateWeight<Runtime>;
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type ManagerOrigin = EnsureRoot<AccountId>;
	type BaseDeposit = ConstU128<100_000_000_000_000>;
	type ByteDeposit = ConstU128<1_000_000_000_000>;
}

impl utility::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = weights::utility::WeightInfo<Runtime>;
}

impl currency_factory::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AssetId = CurrencyId;
	type AddOrigin = EnsureRootOrTwoThirdNativeCouncil;
	type WeightInfo = weights::currency_factory::WeightInfo<Runtime>;
	type Balance = Balance;
}

parameter_types! {
	pub const CrowdloanRewardsId: PalletId = PalletId(*b"pal_crow");
	pub const CrowdloanRewardsLockId: LockIdentifier = *b"clr_lock";
	pub const InitialPayment: Perbill = Perbill::from_percent(50);
	pub const OverFundedThreshold: Perbill = Perbill::from_percent(1);
	pub const VestingStep: Moment = (DAYS as Moment) * (MILLISECS_PER_BLOCK as Moment);
	pub const Prefix: &'static [u8] = b"picasso-";
	pub const LockCrowdloanRewards: bool = true;
}

parameter_types! {
	pub MaxProxies : u32 = 4;
	pub MaxPending : u32 = 32;
}
// Minimal deposit required to place a proxy announcement as per native existential deposit.
pub type ProxyPrice = NativeExistentialDeposit;

impl proxy::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Assets;
	type ProxyType = composable_traits::account_proxy::ProxyType;
	type ProxyDepositBase = ProxyPrice;
	type ProxyDepositFactor = ProxyPrice;
	type MaxProxies = MaxProxies;
	type WeightInfo = weights::proxy::WeightInfo<Runtime>;
	type MaxPending = MaxPending;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = ProxyPrice;
	type AnnouncementDepositFactor = ProxyPrice;
}

impl crowdloan_rewards::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type RewardAsset = Assets;
	type AdminOrigin = EnsureRootOrTwoThirdNativeCouncil;
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
	  pub const MaxVestingSchedule: u32 = 128;
	  pub MinVestedTransfer: u64 = CurrencyId::milli::<u64>();
}

impl vesting::Config for Runtime {
	type Currency = Assets;
	type RuntimeEvent = RuntimeEvent;
	type MaxVestingSchedules = MaxVestingSchedule;
	type MinVestedTransfer = MinVestedTransfer;
	type VestedTransferOrigin = EnsureRootOrTwoThirdNativeCouncil;
	type UpdateSchedulesOrigin = EnsureRootOrTwoThirdNativeCouncil;
	type WeightInfo = weights::vesting::WeightInfo<Runtime>;
	type Moment = Moment;
	type Time = Timestamp;
	type VestingScheduleId = u128;
}

parameter_types! {
	// cspell:disable-next
	  pub const BondedFinanceId: PalletId = PalletId(*b"bondedfi");
	  pub MinReward: Balance = 10 * CurrencyId::unit::<Balance>();
	  pub Stake: Balance = 10 * CurrencyId::unit::<Balance>();
}

impl bonded_finance::Config for Runtime {
	type AdminOrigin = EnsureRootOrTwoThirdNativeCouncil;
	type BondOfferId = BondOfferId;
	type Convert = sp_runtime::traits::ConvertInto;
	type Currency = Assets;
	type RuntimeEvent = RuntimeEvent;
	type MinReward = MinReward;
	type NativeCurrency = Balances;
	type PalletId = BondedFinanceId;
	type Stake = Stake;
	type Vesting = Vesting;
	type WeightInfo = weights::bonded_finance::WeightInfo<Runtime>;
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

		// Governance utilities
		Council: collective::<Instance1> = 30,
		CouncilMembership: membership::<Instance1> = 31,
		Treasury: treasury::<Instance1> = 32,
		Democracy: democracy = 33,
		TechnicalCommittee: collective::<Instance2> = 72,
		TechnicalCommitteeMembership: membership::<Instance2> = 73,

		ReleaseCommittee: collective::<Instance3> = 74,
		ReleaseMembership: membership::<Instance3> = 75,

		// helpers/utilities
		Scheduler: scheduler = 34,
		Utility: utility = 35,
		Preimage: preimage = 36,
		Proxy: proxy = 37,

		// XCM helpers.
		XcmpQueue: cumulus_pallet_xcmp_queue = 40,
		PolkadotXcm: pallet_xcm = 41,
		CumulusXcm: cumulus_pallet_xcm = 42,
		DmpQueue: cumulus_pallet_dmp_queue = 43,
		XTokens: orml_xtokens = 44,
		UnknownTokens: orml_unknown_tokens = 45,

		Tokens: orml_tokens = 52,
		CurrencyFactory: currency_factory = 53,
		GovernanceRegistry: governance_registry = 54,
		Assets: assets = 55,
		CrowdloanRewards: crowdloan_rewards = 56,
		Vesting: vesting = 57,
		BondedFinance: bonded_finance = 58,
		AssetsRegistry: assets_registry = 59,
		Pablo: pablo = 60,
		Oracle: oracle = 61,
		AssetsTransactorRouter: assets_transactor_router = 62,
		FarmingRewards: reward::<Instance1> = 63,
		Farming: farming = 64,

		CallFilter: call_filter = 100,

		Ibc: pallet_ibc = 190,
		Ics20Fee: pallet_ibc::ics20_fee = 191,
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
	AssetsPaymentHeader,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = executive::Executive<
	Runtime,
	Block,
	system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
	crate::migrations::Migrations,
>;

#[allow(unused_imports)]
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
		[indices, Indices]
		[membership, CouncilMembership]
		[treasury, Treasury]
		[scheduler, Scheduler]
		[collective, Council]
		[utility, Utility]
		[identity, Identity]
		[multisig, Multisig]
		[proxy, Proxy]
		[currency_factory, CurrencyFactory]
		[bonded_finance, BondedFinance]
		[vesting, Vesting]
		[assets_registry, AssetsRegistry]
		[pablo, Pablo]
		[democracy, Democracy]
		[oracle, Oracle]
	);
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

impl_runtime_apis! {
	impl assets_runtime_api::AssetsRuntimeApi<Block, CurrencyId, AccountId, Balance, ForeignAssetId> for Runtime {
		fn balance_of(SafeRpcWrapper(asset_id): SafeRpcWrapper<CurrencyId>, account_id: AccountId) -> SafeRpcWrapper<Balance> /* Balance */ {
			SafeRpcWrapper(<Assets as fungibles::Inspect::<AccountId>>::balance(asset_id, &account_id))
		}

		fn list_assets() -> Vec<Asset<Balance, ForeignAssetId>> {
			// Hardcoded assets
			use common::fees::ForeignToNativePriceConverter;
			let assets = CurrencyId::list_assets().into_iter().map(|mut asset| {
				// Add hardcoded ratio and ED for well known assets
				asset.ratio = WellKnownForeignToNativePriceConverter::get_ratio(CurrencyId(asset.id));
				asset.existential_deposit = multi_existential_deposits::<AssetsRegistry, WellKnownForeignToNativePriceConverter>(&asset.id.into());
				asset
			}).map(|xcm|
			  Asset {
				decimals : xcm.decimals,
				existential_deposit : xcm.existential_deposit,
				id : xcm.id,
				foreign_id : xcm.foreign_id.map(Into::into),
				name : xcm.name,
				ratio : xcm.ratio,
			  }
			).collect::<Vec<_>>();

			// Assets from the assets-registry pallet
			let all_assets =  assets_registry::Pallet::<Runtime>::get_all_assets();

			// Override asset data for hardcoded assets that have been manually updated, and append
			// new assets without duplication
			all_assets.into_iter().fold(assets, |mut acc, mut asset| {
				if let Some(found_asset) = acc.iter_mut().find(|asset_i| asset_i.id == asset.id) {
					// Update a found asset with data from assets-registry
					found_asset.decimals = asset.decimals;
					found_asset.foreign_id = asset.foreign_id.clone();
					found_asset.ratio = asset.ratio;
				} else {
					asset.existential_deposit = multi_existential_deposits::<AssetsRegistry, WellKnownForeignToNativePriceConverter>(&asset.id.into());
					acc.push(asset.clone())
				}
				acc
			})
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
			SafeRpcWrapper(
				<Pablo as Amm>::simulate_add_liquidity(
					&who.0,
					pool_id.0,
					amounts.iter().map(|(k, v)| (k.0, v.0)).collect(),
				)
				.unwrap_or_else(|_| Zero::zero())
			)
		}

		fn simulate_remove_liquidity(
			who: SafeRpcWrapper<AccountId>,
			pool_id: SafeRpcWrapper<PoolId>,
			lp_amount: SafeRpcWrapper<Balance>,
			min_expected_amounts: BTreeMap<SafeRpcWrapper<CurrencyId>, SafeRpcWrapper<Balance>>,
		) -> BTreeMap<SafeRpcWrapper<CurrencyId>, SafeRpcWrapper<Balance>> {
			<Pablo as Amm>::simulate_remove_liquidity(
				&who.0,
				pool_id.0,
				lp_amount.0,
				min_expected_amounts
					.iter()
					.map(|(k, v)| (k.0, v.0))
					.collect()
				)
				.map(|simulation_result| {
					simulation_result
						.into_iter()
						.map(|(k, v)| (SafeRpcWrapper(k), SafeRpcWrapper(v)))
						.collect()
				})
				.unwrap_or_default()
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

	impl reward_rpc_runtime_api::RewardApi<
		Block,
		AccountId,
		CurrencyId,
		Balance,
		BlockNumber,
		sp_runtime::FixedU128
	> for Runtime {
		fn compute_farming_reward(account_id: AccountId, pool_currency_id: CurrencyId, reward_currency_id: CurrencyId) -> Result<reward_rpc_runtime_api::BalanceWrapper<Balance>, DispatchError> {
			let amount = <FarmingRewards as reward::RewardsApi<CurrencyId, AccountId, Balance>>::compute_reward(&pool_currency_id, &account_id, reward_currency_id)?;
			let balance = reward_rpc_runtime_api::BalanceWrapper::<Balance> { amount };
			Ok(balance)
		}
		fn estimate_farming_reward(
			account_id: AccountId,
			pool_currency_id: CurrencyId,
			reward_currency_id: CurrencyId,
		) -> Result<reward_rpc_runtime_api::BalanceWrapper<Balance>, DispatchError> {
			<FarmingRewards as reward::RewardsApi<CurrencyId, AccountId, Balance>>::withdraw_reward(&pool_currency_id, &account_id, reward_currency_id)?;
			<FarmingRewards as reward::RewardsApi<CurrencyId, AccountId, Balance>>::distribute_reward(&pool_currency_id, reward_currency_id, Farming::total_rewards(&pool_currency_id, &reward_currency_id))?;
			let amount = <FarmingRewards as reward::RewardsApi<CurrencyId, AccountId, Balance>>::compute_reward(&pool_currency_id, &account_id, reward_currency_id)?;
			let balance = reward_rpc_runtime_api::BalanceWrapper::<Balance> { amount };
			Ok(balance)
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
			return (list, storage_info)
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

	impl ibc_runtime_api::IbcRuntimeApi<Block, CurrencyId> for Runtime {
		fn para_id() -> u32 {
			<Runtime as cumulus_pallet_parachain_system::Config>::SelfParaId::get().into()
		}

		fn child_trie_key() -> Vec<u8> {
			<Runtime as pallet_ibc::Config>::PalletPrefix::get().to_vec()
		}

		fn query_balance_with_address(addr: Vec<u8>, asset_id:CurrencyId) -> Option<u128> {
			Ibc::query_balance_with_address(addr, asset_id).ok()
		}

		fn query_send_packet_info(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<ibc_primitives::PacketInfo>> {
			Ibc::get_send_packet_info(channel_id, port_id, seqs).ok()
		}

		fn query_recv_packet_info(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<ibc_primitives::PacketInfo>> {
			Ibc::get_recv_packet_info(channel_id, port_id, seqs).ok()
		}

		fn client_update_time_and_height(client_id: Vec<u8>, revision_number: u64, revision_height: u64) -> Option<(u64, u64)>{
			Ibc::client_update_time_and_height(client_id, revision_number, revision_height).ok()
		}

		fn client_state(client_id: Vec<u8>) -> Option<ibc_primitives::QueryClientStateResponse> {
			Ibc::client(client_id).ok()
		}

		fn client_consensus_state(client_id: Vec<u8>, revision_number: u64, revision_height: u64, latest_cs: bool) -> Option<ibc_primitives::QueryConsensusStateResponse> {
			Ibc::consensus_state(client_id, revision_number, revision_height, latest_cs).ok()
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

		fn denom_trace(asset_id: CurrencyId) -> Option<ibc_primitives::QueryDenomTraceResponse> {
			Ibc::get_denom_trace(asset_id)
		}

		fn denom_traces(key: Option<CurrencyId>, offset: Option<u32>, limit: u64, count_total: bool) -> ibc_primitives::QueryDenomTracesResponse {
			let key = key.map(Either::Left).or_else(|| offset.map(Either::Right));
			Ibc::get_denom_traces(key, limit, count_total)
		}

		fn block_events(extrinsic_index: Option<u32>) -> Vec<Result<pallet_ibc::events::IbcEvent, pallet_ibc::errors::IbcError>> {
			let mut raw_events = frame_system::Pallet::<Self>::read_events_no_consensus();
			if let Some(idx) = extrinsic_index {
				raw_events.find_map(|e| {
					let frame_system::EventRecord{ event, phase, ..} = *e;
					match (event, phase) {
						(RuntimeEvent::Ibc(pallet_ibc::Event::Events{ events }), frame_system::Phase::ApplyExtrinsic(index)) if index == idx => Some(events),
						_ => None
					}
				}).unwrap_or_default()
			}
			else {
				raw_events.filter_map(|e| {
					let frame_system::EventRecord{ event, ..} = *e;

					match event {
						RuntimeEvent::Ibc(pallet_ibc::Event::Events{ events }) => {
								Some(events)
							},
						_ => None
					}
				}).flatten().collect()
			}
		}
	 }
}
