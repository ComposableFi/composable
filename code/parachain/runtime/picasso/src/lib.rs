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
#[cfg(all(feature = "std", feature = "builtin-wasm"))]
pub const WASM_BINARY_V2: Option<&[u8]> = Some(include_bytes!(env!("PICASSO_RUNTIME")));
#[cfg(not(feature = "builtin-wasm"))]
pub const WASM_BINARY_V2: Option<&[u8]> = None;

pub mod governance;
mod weights;
pub mod xcmp;

pub use xcmp::{MaxInstructions, UnitWeightCost, XcmConfig};

use governance::*;

use common::{
	governance::native::*, impls::DealWithFees, multi_existential_deposits, AccountId,
	AccountIndex, Address, Amount, AuraId, Balance, BlockNumber, BondOfferId, ForeignAssetId, Hash,
	MaxStringSize, Moment, NativeExistentialDeposit, PriceConverter, Signature,
	AVERAGE_ON_INITIALIZE_RATIO, DAYS, HOURS, MAXIMUM_BLOCK_WEIGHT, MILLISECS_PER_BLOCK,
	NORMAL_DISPATCH_RATIO, SLOT_DURATION,
};

use composable_traits::assets::Asset;
use primitives::currency::{CurrencyId, ValidateCurrencyId};
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{AccountIdConversion, AccountIdLookup, BlakeTwo256, Block as BlockT, Zero},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult,
};

use composable_support::rpc_helpers::SafeRpcWrapper;
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

// A few exports that help ease life for downstream crates.
pub use frame_support::{
	construct_runtime, parameter_types,
	traits::{
		ConstBool, ConstU128, ConstU16, ConstU32, Contains, Everything, KeyOwnerProofSystem,
		Nothing, Randomness, StorageInfo,
	},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
		ConstantMultiplier, DispatchClass, IdentityFee, Weight, WeightToFeeCoefficient,
		WeightToFeeCoefficients, WeightToFeePolynomial,
	},
	PalletId, StorageValue,
};

pub use governance::TreasuryAccount;

use codec::{Codec, Encode, EncodeLike};
use frame_support::traits::{fungibles, EqualPrivilegeOnly, OnRuntimeUpgrade};
use frame_system as system;
use scale_info::TypeInfo;
use sp_runtime::AccountId32;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{FixedPointNumber, Perbill, Permill, Perquintill};
use sp_std::fmt::Debug;
use system::{
	limits::{BlockLength, BlockWeights},
	EnsureRoot,
};
use transaction_payment::{Multiplier, TargetedFeeAdjustment};

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
	spec_version: 1402,
	impl_version: 2,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 0,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

use orml_traits::{parameter_type_with_key, LockIdentifier, MultiCurrency};
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
	pub NativeAssetId: CurrencyId = CurrencyId::PICA;
}

impl assets_registry::Config for Runtime {
	type Event = Event;
	type LocalAssetId = CurrencyId;
	type Balance = Balance;
	type ForeignAssetId = composable_traits::xcm::assets::XcmAssetLocation;
	type UpdateAssetRegistryOrigin = EnsureRootOrHalfNativeCouncil;
	type ParachainOrGovernanceOrigin = EnsureRootOrHalfNativeCouncil;
	type CurrencyFactory = CurrencyFactory;
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
	type ExistentialDeposit = NativeExistentialDeposit;
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
	type OnChargeTransaction = transaction_payment::CurrencyAdapter<
		Balances,
		DealWithFees<Runtime, NativeTreasury, Balances>,
	>;
	type WeightToFee = WeightToFee;
	type FeeMultiplierUpdate =
		TargetedFeeAdjustment<Self, TargetBlockFullness, AdjustmentVariable, MinimumMultiplier>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
}

/// Struct implementing `asset_tx_payment::HandleCredit` that determines the behavior when fees are
/// paid in something other than the native token.
pub struct TransferToTreasuryOrDrop;
impl asset_tx_payment::HandleCredit<AccountId, Tokens> for TransferToTreasuryOrDrop {
	fn handle_credit(credit: fungibles::CreditOf<AccountId, Tokens>) {
		// `old_free` is only the `free` balance of an account
		let old_free = <Tokens as MultiCurrency<AccountId>>::free_balance(
			credit.asset(),
			&TreasuryAccount::get(),
		);

		// `new_free` is `old_free + credit.peek()`
		let new_free = old_free.saturating_add(credit.peek());

		// NOTE: After our runtime depends on paritytech/substrate PR#12569, this function will need
		// to be re-evaluated as `set_balance` might not do what it is meant to do when implemented
		// for pallet balances or orml tokens.
		// https://github.com/paritytech/substrate/pull/12569
		let _ = <Tokens as fungibles::Unbalanced<AccountId>>::set_balance(
			credit.asset(),
			&TreasuryAccount::get(),
			new_free,
		);
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

impl utility::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = weights::utility::WeightInfo<Runtime>;
}

impl currency_factory::Config for Runtime {
	type Event = Event;
	type AssetId = CurrencyId;
	type AddOrigin = EnsureRootOrHalfNativeCouncil;
	type WeightInfo = weights::currency_factory::WeightInfo<Runtime>;
	type Balance = Balance;
}

parameter_types! {
	pub const CrowdloanRewardsId: PalletId = PalletId(*b"pal_crow");
	pub const CrowdloanRewardsLockId: LockIdentifier = *b"clr_lock";
	pub const InitialPayment: Perbill = Perbill::from_percent(25);
	pub const OverFundedThreshold: Perbill = Perbill::from_percent(1);
	pub const VestingStep: Moment = (7 * DAYS as Moment) * (MILLISECS_PER_BLOCK as Moment);
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
	  pub const MaxVestingSchedule: u32 = 128;
	  pub MinVestedTransfer: u64 = CurrencyId::milli::<u64>();
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
	  pub MinReward: Balance = 10 * CurrencyId::unit::<Balance>();
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

/// The calls we permit to be executed by extrinsics
pub struct BaseCallFilter;
impl Contains<Call> for BaseCallFilter {
	fn contains(call: &Call) -> bool {
		!(call_filter::Pallet::<Runtime>::contains(call) ||
			matches!(call, Call::Tokens(_) | Call::Indices(_) | Call::Treasury(_)))
	}
}

impl call_filter::Config for Runtime {
	type Event = Event;
	type UpdateOrigin = EnsureRootOrOneThirdNativeTechnical;
	type Hook = ();
	type WeightInfo = ();
	type MaxStringSize = MaxStringSize;
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
		Democracy: democracy::<Instance1> = 33,
		TechnicalCollective: collective::<Instance2> = 70,
		TechnicalMembership: membership::<Instance2> = 71,

		// helpers/utilities
		Scheduler: scheduler = 34,
		Utility: utility = 35,
		Preimage: preimage = 36,

		// XCM helpers.
		XcmpQueue: cumulus_pallet_xcmp_queue = 40,
		RelayerXcm: pallet_xcm = 41,
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

		CallFilter: call_filter = 100,
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

// Migration for scheduler pallet to move from a plain Call to a CallOrHash.
pub struct SchedulerMigrationV3;
impl OnRuntimeUpgrade for SchedulerMigrationV3 {
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		Scheduler::migrate_v2_to_v3()
	}
}

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
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
		[currency_factory, CurrencyFactory]
		[bonded_finance, BondedFinance]
		[vesting, Vesting]
		[assets_registry, AssetsRegistry]
	);
}

impl_runtime_apis! {
	impl assets_runtime_api::AssetsRuntimeApi<Block, CurrencyId, AccountId, Balance, ForeignAssetId> for Runtime {
		fn balance_of(SafeRpcWrapper(asset_id): SafeRpcWrapper<CurrencyId>, account_id: AccountId) -> SafeRpcWrapper<Balance> /* Balance */ {
			SafeRpcWrapper(<Assets as fungibles::Inspect::<AccountId>>::balance(asset_id, &account_id))
		}

		fn list_assets() -> Vec<Asset<ForeignAssetId>> {
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
