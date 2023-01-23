#![allow(non_snake_case)]

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
mod migrations;
mod prelude;
mod versions;
mod weights;
pub mod xcmp;

use alloc::string::String;
use core::str::FromStr;
pub use versions::*;

use lending::MarketId;
use orml_traits::{parameter_type_with_key, LockIdentifier};
// TODO: consider moving this to shared runtime
pub use xcmp::{MaxInstructions, UnitWeightCost};

use common::{
	fees::{
		multi_existential_deposits, NativeExistentialDeposit, PriceConverter, WeightToFeeConverter,
	},
	governance::native::{
		EnsureRootOrHalfNativeCouncil, EnsureRootOrOneThirdNativeTechnical, NativeTreasury,
	},
	rewards::StakingPot,
	AccountId, AccountIndex, Address, Amount, AuraId, Balance, BlockNumber, BondOfferId,
	FinancialNftInstanceId, ForeignAssetId, Hash, MaxStringSize, Moment, MosaicRemoteAssetId,
	PoolId, Signature, AVERAGE_ON_INITIALIZE_RATIO, DAYS, HOURS, MAXIMUM_BLOCK_WEIGHT,
	MILLISECS_PER_BLOCK, NORMAL_DISPATCH_RATIO, SLOT_DURATION,
};
use composable_support::rpc_helpers::SafeRpcWrapper;
use composable_traits::{
	assets::Asset,
	defi::Rate,
	dex::{Amm, PriceAggregate},
	xcm::assets::RemoteAssetRegistryInspect,
};
use cosmwasm::instrument::CostRules;
use primitives::currency::{CurrencyId, ValidateCurrencyId};
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
	generic, impl_opaque_keys,
	traits::{
		AccountIdConversion, AccountIdLookup, BlakeTwo256, Block as BlockT, Bounded, Convert,
		ConvertInto, Zero,
	},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, DispatchError, Either,
};

use sp_std::prelude::*;

// A few exports that help ease life for downstream crates.
pub use frame_support::{
	construct_runtime,
	pallet_prelude::DispatchClass,
	parameter_types,
	traits::{
		ConstBool, Contains, Everything, Get, KeyOwnerProofSystem, Nothing, Randomness, StorageInfo,
	},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
		IdentityFee, Weight, WeightToFeeCoefficient, WeightToFeeCoefficients,
		WeightToFeePolynomial,
	},
	PalletId, StorageValue,
};

use codec::Encode;
use common::fees::WellKnownForeignToNativePriceConverter;
use composable_traits::{
	account_proxy::{AccountProxyWrapper, ProxyType},
	currency::{CurrencyFactory as CurrencyFactoryT, RangeId, Rational64},
	fnft::FnftAccountProxyType,
	xcm::assets::{RemoteAssetRegistryMutate, XcmAssetLocation},
};
use frame_support::{
	traits::{fungibles, ConstU32, EqualPrivilegeOnly, InstanceFilter, OnRuntimeUpgrade},
	weights::ConstantMultiplier,
};
use frame_system as system;
use ibc::core::{
	ics24_host::identifier::PortId,
	ics26_routing::context::{Module, ModuleId},
};
use pallet_ibc::{
	light_client_common::RelayChain, routing::ModuleRouter, DenomToAssetId, IbcAssetIds, IbcAssets,
	IbcDenoms,
};
use scale_info::TypeInfo;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
use sp_runtime::Either::*;
pub use sp_runtime::{FixedPointNumber, Perbill, Permill, Perquintill};
use sp_std::{collections::btree_map::BTreeMap, fmt::Debug, vec::Vec};
use system::{
	limits::{BlockLength, BlockWeights},
	EnsureRoot,
};
use transaction_payment::{Multiplier, TargetedFeeAdjustment};
use xcm::{latest::MultiLocation, prelude::X1, v1::Junction};
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

parameter_type_with_key! {
	// Minimum amount an account has to hold to stay in state
	pub MultiExistentialDeposits: |currency_id: CurrencyId| -> Balance {
		multi_existential_deposits::<AssetsRegistry>(currency_id)
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
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = Treasury;
	type ExistentialDeposit = NativeExistentialDeposit;
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

impl transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction =
		transaction_payment::CurrencyAdapter<Balances, StakingPot<Runtime, NativeTreasury>>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type WeightToFee = WeightToFeeConverter;
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
	type FeeMultiplierUpdate =
		TargetedFeeAdjustment<Self, TargetBlockFullness, AdjustmentVariable, MinimumMultiplier>;
}

/// Struct implementing `asset_tx_payment::HandleCredit` that determines the behavior when fees are
/// paid in something other than the native token.
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

	type ConfigurationExistentialDeposit = NativeExistentialDeposit;

	type PayableCall = RuntimeCall;

	type Lock = Assets;

	type BalanceConverter = PriceConverter<AssetsRegistry>;
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

//TODO set
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
	type RuntimeEvent = RuntimeEvent;
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
	type OnSlash = ();
	type OnTransfer = ();
	type OnDeposit = ();
}

parameter_types! {
	pub const LiquidRewardId: PalletId = PalletId(*b"Liquided");
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
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type PalletsOrigin = OriginCaller;
	type RuntimeCall = RuntimeCall;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = EnsureRoot<AccountId>;
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type MaxScheduledPerBlock = MaxScheduledPerBlock;
	type PreimageProvider = Preimage;
	type NoPreimagePostponement = NoPreimagePostponement;
	type WeightInfo = scheduler::weights::SubstrateWeight<Runtime>;
}

impl utility::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = weights::utility::WeightInfo<Runtime>;
}

impl InstanceFilter<RuntimeCall> for ProxyType {
	fn filter(&self, c: &RuntimeCall) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::Governance => matches!(
				c,
				RuntimeCall::Democracy(..) |
					RuntimeCall::Council(..) |
					RuntimeCall::TechnicalCommittee(..) |
					RuntimeCall::Treasury(..) |
					RuntimeCall::Utility(..)
			),
			ProxyType::CancelProxy => {
				// TODO (vim): We might not need this
				matches!(c, RuntimeCall::Proxy(proxy::Call::reject_announcement { .. }))
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

impl proxy::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Assets;
	type ProxyType = ProxyType;
	type ProxyDepositBase = ProxyPrice;
	type ProxyDepositFactor = ProxyPrice;
	type MaxProxies = MaxProxies;
	type WeightInfo = weights::proxy::WeightInfo<Runtime>;
	type MaxPending = MaxPending;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = ProxyPrice;
	type AnnouncementDepositFactor = ProxyPrice;
}

type AccountProxyWrapperInstance = AccountProxyWrapper<Runtime>;
parameter_types! {
	pub const FnftPalletId: PalletId = PalletId(*b"pal_fnft");
}

impl pallet_fnft::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type MaxProperties = ConstU32<16>;
	type FinancialNftCollectionId = CurrencyId;
	type FinancialNftInstanceId = FinancialNftInstanceId;
	type ProxyType = ProxyType;
	type AccountProxy = AccountProxyWrapperInstance;
	type ProxyTypeSelector = FnftAccountProxyType;
	type PalletId = FnftPalletId;
	type WeightInfo = weights::fnft::WeightInfo<Runtime>;
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
	type RuntimeEvent = RuntimeEvent;
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
	type RuntimeEvent = RuntimeEvent;
	type AssetId = CurrencyId;
	type AddOrigin = EnsureRootOrHalfNativeCouncil;
	type WeightInfo = weights::currency_factory::WeightInfo<Runtime>;
	type Balance = Balance;
}

impl assets_registry::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
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
	type RuntimeEvent = RuntimeEvent;
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
	pub const StakingRewardsLockId: LockIdentifier = *b"stk_lock";
}

impl pallet_staking_rewards::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
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
	type LockId = StakingRewardsLockId;
	type TreasuryAccount = TreasuryAccount;
	type ExistentialDeposits = MultiExistentialDeposits;
}

/// The calls we permit to be executed by extrinsics
// TODO(hussein-aitlahcen):
// remove IBC pallets from the call filter once centauri is merged
pub struct BaseCallFilter;
impl Contains<RuntimeCall> for BaseCallFilter {
	fn contains(call: &RuntimeCall) -> bool {
		!(call_filter::Pallet::<Runtime>::contains(call) ||
			matches!(
				call,
				RuntimeCall::Tokens(_) | RuntimeCall::Indices(_) | RuntimeCall::Treasury(_)
			))
	}
}

impl call_filter::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
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
	type RuntimeEvent = RuntimeEvent;
	type MaxVestingSchedules = MaxVestingSchedule;
	type MinVestedTransfer = MinVestedTransfer;
	type VestedTransferOrigin = EnsureRootOrHalfNativeCouncil;
	type UpdateSchedulesOrigin = EnsureRootOrHalfNativeCouncil;
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
	type RuntimeEvent = RuntimeEvent;
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
	type RuntimeEvent = RuntimeEvent;
	type MultiCurrency = Assets;
	type PalletId = DutchAuctionId;
	type OrderId = u128;
	type UnixTime = Timestamp;
	type WeightInfo = weights::dutch_auction::WeightInfo<Runtime>;
	type PositionExistentialDeposit = NativeExistentialDeposit;
	type XcmOrigin = RuntimeOrigin;
	type AdminOrigin = EnsureRootOrHalfNativeCouncil;
	type XcmSender = XcmRouter;
}

parameter_types! {
	pub const MosaicId: PalletId = PalletId(*b"plmosaic");
	pub const MinimumTTL: BlockNumber = 10;
	pub const MinimumTimeLockPeriod: BlockNumber = 20;
}

impl mosaic::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
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
	type RuntimeEvent = RuntimeEvent;
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
	type RuntimeEvent = RuntimeEvent;
	type Oracle = Oracle;
	type VaultId = u64;
	type Vault = Vault;
	type VaultLender = Vault;
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
	type WeightToFee = WeightToFeeConverter;
}

parameter_types! {
  pub PabloId: PalletId = PalletId(*b"pall_pab");
  pub TWAPInterval: u64 = (MILLISECS_PER_BLOCK as u64) * 10;
}

impl pablo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type Convert = ConvertInto;
	type CurrencyFactory = CurrencyFactory;
	type Assets = Assets;
	type PoolId = PoolId;
	type PalletId = PabloId;
	type LocalAssets = CurrencyFactory;
	type PoolCreationOrigin = EnsureRootOrHalfNativeCouncil;
	// TODO: consider making it is own origin
	type EnableTwapOrigin = EnsureRootOrHalfNativeCouncil;
	type TWAPInterval = TWAPInterval;
	type Time = Timestamp;
	type WeightInfo = weights::pablo::WeightInfo<Runtime>;
}

parameter_types! {
	#[derive(TypeInfo, codec::MaxEncodedLen, codec::Encode)]
	pub const MaxHopsCount: u32 = 4;
	pub DexRouterPalletID: PalletId = PalletId(*b"dex_rout");
}

impl dex_router::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
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
  pub WasmCostRules: CostRules<Runtime> = Default::default();
}

impl cosmwasm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
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
	type WasmCostRules = WasmCostRules;
	type UnixTime = Timestamp;
	type WeightInfo = cosmwasm::weights::SubstrateWeight<Runtime>;
	type IbcRelayerAccount = TreasuryAccount;
	type IbcRelayer = cosmwasm::NoRelayer<Runtime>;
	type PalletHook = ();
}

parameter_types! {
	pub const RelayChainId: RelayChain = RelayChain::Rococo;
	pub const SpamProtectionDeposit: Balance = 1_000_000_000_000;
	pub const MinimumConnectionDelay: u64 = 0;
}

type CosmwasmRouter = cosmwasm::ibc::Router<Runtime>;

#[allow(clippy::derivable_impls)]
impl Default for Runtime {
	fn default() -> Self {
		Self {}
	}
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Router {
	pallet_ibc_ping: pallet_ibc_ping::IbcModule<Runtime>,
	pallet_cosmwasm: CosmwasmRouter,
}

impl ModuleRouter for Router {
	fn get_route_mut(&mut self, module_id: &ModuleId) -> Option<&mut dyn Module> {
		match module_id.as_ref() {
			pallet_ibc_ping::MODULE_ID => Some(&mut self.pallet_ibc_ping),
			_ => self.pallet_cosmwasm.get_route_mut(module_id),
		}
	}

	fn has_route(module_id: &ModuleId) -> bool {
		matches!(module_id.as_ref(), pallet_ibc_ping::MODULE_ID) ||
			CosmwasmRouter::has_route(module_id)
	}

	fn lookup_module_by_port(port_id: &PortId) -> Option<ModuleId> {
		match port_id.as_str() {
			pallet_ibc_ping::PORT_ID => ModuleId::from_str(pallet_ibc_ping::MODULE_ID).ok(),
			_ => CosmwasmRouter::lookup_module_by_port(port_id),
		}
	}
}

pub struct IbcDenomToAssetIdConversion;

impl DenomToAssetId<Runtime> for IbcDenomToAssetIdConversion {
	type Error = DispatchError;

	fn from_denom_to_asset_id(denom: &String) -> Result<CurrencyId, Self::Error> {
		let denom_bytes = denom.as_bytes().to_vec();
		if let Some(id) = IbcDenoms::<Runtime>::get(&denom_bytes) {
			return Ok(id)
		}

		let asset_id =
			<currency_factory::Pallet<Runtime> as CurrencyFactoryT>::create(RangeId::IBC_ASSETS)?;

		IbcDenoms::<Runtime>::insert(denom_bytes.clone(), asset_id);
		IbcAssetIds::<Runtime>::insert(asset_id, denom_bytes);

		let location = XcmAssetLocation::new(MultiLocation::new(
			1,
			X1(Junction::GeneralIndex(asset_id.into())),
		));
		assets_registry::Pallet::<Runtime>::set_reserve_location(
			asset_id,
			location,
			Rational64::one(),
			Some(12),
		)?;

		Ok(asset_id)
	}

	fn from_asset_id_to_denom(id: CurrencyId) -> Option<String> {
		IbcAssetIds::<Runtime>::get(id).and_then(|denom| String::from_utf8(denom).ok())
	}

	fn ibc_assets(start_key: Option<Either<CurrencyId, u32>>, limit: u64) -> IbcAssets<CurrencyId> {
		let mut iterator = match start_key {
			None => IbcAssetIds::<Runtime>::iter().skip(0),
			Some(Left(asset_id)) => {
				let raw_key = asset_id.encode();
				IbcAssetIds::<Runtime>::iter_from(raw_key).skip(0)
			},
			Some(Right(offset)) => IbcAssetIds::<Runtime>::iter().skip(offset as usize),
		};

		let denoms = iterator.by_ref().take(limit as usize).map(|(_, denom)| denom).collect();
		let maybe_currency_id = iterator.next().map(|(id, ..)| id);
		IbcAssets {
			denoms,
			total_count: IbcAssetIds::<Runtime>::count() as u64,
			next_id: maybe_currency_id,
		}
	}
}

impl pallet_ibc::Config for Runtime {
	type TimeProvider = Timestamp;
	type RuntimeEvent = RuntimeEvent;
	type NativeCurrency = Balances;
	type Balance = Balance;
	type AssetId = CurrencyId;
	type NativeAssetId = NativeAssetId;
	type IbcDenomToAssetIdConversion = IbcDenomToAssetIdConversion;
	const PALLET_PREFIX: &'static [u8] = b"ibc/";
	const LIGHT_CLIENT_PROTOCOL: pallet_ibc::LightClientProtocol =
		pallet_ibc::LightClientProtocol::Grandpa;
	type AccountIdConversion = ibc_primitives::IbcAccount<AccountId>;
	type Fungibles = Assets;
	type ExpectedBlockTime = ExpectedBlockTime;
	type Router = Router;
	type MinimumConnectionDelay = MinimumConnectionDelay;
	type ParaId = parachain_info::Pallet<Runtime>;
	type RelayChain = RelayChainId;
	type WeightInfo = ();
	type AdminOrigin = EnsureRoot<AccountId>;
	type SentryOrigin = EnsureRoot<AccountId>;
	type SpamProtectionDeposit = SpamProtectionDeposit;
}

impl pallet_ibc_ping::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type IbcHandler = Ibc;
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
		Democracy: democracy = 33,
		TechnicalCommittee: collective::<Instance2> = 70,
		TechnicalCommitteeMembership: membership::<Instance2> = 71,


		// helpers/utilities
		Scheduler: scheduler = 34,
		Utility: utility = 35,
		Preimage: preimage = 36,
		Proxy: proxy = 37,

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

		// Cosmwasm support
		Cosmwasm: cosmwasm = 180,

		// IBC support
		Ibc: pallet_ibc = 190,
		IbcPing: pallet_ibc_ping = 191
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
pub type UncheckedExtrinsic =
	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

/// Executive: handles dispatch to the various modules.
pub type Executive = executive::Executive<
	Runtime,
	Block,
	system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
	migrations::Migrations,
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
		// TODO(hussein): broken as of v0.9.30
			// [lending, Lending]
		/*
		  2023-01-17 13:01:50 panicked at 'Timestamp slot must match `CurrentSlot`', /sources/b44e579ebfa6a9c3cb36e320617fc592078911c53e5f75fcffd348678aec1f49/pallet-aura-4.0.0-dev/src/lib.rs:299:9
	Error:
	   0: Invalid input: Error executing and verifying runtime benchmark: Execution aborted due to trap: wasm trap: wasm `unreachable` instruction executed
		  WASM backtrace:

			  0: 0x7c44 - <unknown>!rust_begin_unwind
			  1: 0x15456 - <unknown>!core::panicking::panic_fmt::hbd719b21b7458dd3
			  2: 0x518a89 - <unknown>!pallet_aura::<impl frame_support::traits::hooks::OnTimestampSet<<T as pallet_timestamp::pallet::Config>::Moment> for pallet_aura::pallet::Pallet<T>>::on_timestamp_set::h18f88ba233c0b591
			  3: 0x54665d - <unknown>!pallet_timestamp::<impl pallet_timestamp::pallet::Pallet<T>>::set_timestamp::ha4c72baaa2c36987
			  4: 0x801c62 - <unknown>!pallet_lending::benchmarking::setup::produce_block::hf5f1f77bfd4bce7c
			  5: 0x8e59c3 - <unknown>!<pallet_lending::benchmarking::SelectedBenchmark as frame_benchmarking::utils::BenchmarkingSetup<T>>::instance::h48a1d91aceee3bf1
			  6: 0x90679c - <unknown>!pallet_lending::benchmarking::<impl frame_benchmarking::utils::Benchmarking for pallet_lending::pallet::Pallet<T>>::run_benchmark::hb135064c01208619
			  7: 0x8d23f7 - <unknown>!<dali_runtime::Runtime as frame_benchmarking::utils::runtime_decl_for_Benchmark::BenchmarkV1<sp_runtime::generic::block::Block<sp_runtime::generic::header::Header<u32,sp_runtime::traits::BlakeTwo256>,sp_runtime::generic::unchecked_extrinsic::UncheckedExtrinsic<sp_runtime::multiaddress::MultiAddress<<<sp_runtime::MultiSignature as sp_runtime::traits::Verify>::Signer as sp_runtime::traits::IdentifyAccount>::AccountId,u32>,dali_runtime::RuntimeCall,sp_runtime::MultiSignature,(frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<dali_runtime::Runtime>,frame_system::extensions::check_spec_version::CheckSpecVersion<dali_runtime::Runtime>,frame_system::extensions::check_tx_version::CheckTxVersion<dali_runtime::Runtime>,frame_system::extensions::check_genesis::CheckGenesis<dali_runtime::Runtime>,frame_system::extensions::check_mortality::CheckMortality<dali_runtime::Runtime>,frame_system::extensions::check_nonce::CheckNonce<dali_runtime::Runtime>,frame_system::extensions::check_weight::CheckWeight<dali_runtime::Runtime>,pallet_asset_tx_payment::ChargeAssetTxPayment<dali_runtime::Runtime>)>>>>::dispatch_benchmark::h2e63e652b0d76345
			  8: 0x965f70 - <unknown>!Benchmark_dispatch_benchmark
		*/
			[assets_registry, AssetsRegistry]
			[pablo, Pablo]
			[pallet_staking_rewards, StakingRewards]
			[proxy, Proxy]
			[dex_router, DexRouter]
			[cosmwasm, Cosmwasm]
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

	impl assets_runtime_api::AssetsRuntimeApi<Block, CurrencyId, AccountId, Balance, ForeignAssetId> for Runtime {
		fn balance_of(SafeRpcWrapper(asset_id): SafeRpcWrapper<CurrencyId>, account_id: AccountId) -> SafeRpcWrapper<Balance> /* Balance */ {
			SafeRpcWrapper(<Assets as fungibles::Inspect::<AccountId>>::balance(asset_id, &account_id))
		}

		fn list_assets() -> Vec<Asset<Balance, ForeignAssetId>> {
			// Hardcoded assets
			let assets = CurrencyId::list_assets().into_iter().map(|mut asset| {
				// Add hardcoded ratio and ED for well known assets
				asset.ratio = WellKnownForeignToNativePriceConverter::get_ratio(CurrencyId(asset.id));
				asset.existential_deposit = multi_existential_deposits::<AssetsRegistry>(&asset.id.into());
				asset
			}).collect::<Vec<_>>();
			// Assets from the assets-registry pallet
			let foreign_assets = assets_registry::Pallet::<Runtime>::get_foreign_assets_list();

			// Override asset data for hardcoded assets that have been manually updated, and append
			// new assets without duplication
			foreign_assets.into_iter().fold(assets, |mut acc, mut foreign_asset| {
				if let Some(i) = acc.iter().position(|asset_i| asset_i.id == foreign_asset.id) {
					// Assets that have been updated
					if let Some(asset) = acc.get_mut(i) {
						// Update asset with data from assets-registry
						asset.decimals = foreign_asset.decimals;
						asset.foreign_id = foreign_asset.foreign_id.clone();
						asset.ratio = foreign_asset.ratio;
					}
				} else {
					foreign_asset.existential_deposit = multi_existential_deposits::<AssetsRegistry>(&foreign_asset.id.into());
					acc.push(foreign_asset.clone())
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
		fn version() -> sp_version::RuntimeVersion {
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

	impl ibc_runtime_api::IbcRuntimeApi<Block, CurrencyId> for Runtime {
		fn para_id() -> u32 {
			<Runtime as cumulus_pallet_parachain_system::Config>::SelfParaId::get().into()
		}

		fn child_trie_key() -> Vec<u8> {
			<Runtime as pallet_ibc::Config>::PALLET_PREFIX.to_vec()
		}

		fn query_balance_with_address(addr: Vec<u8>) -> Option<u128> {
			Ibc::query_balance_with_address(addr).ok()
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
			let mut raw_events = frame_system::Pallet::<Self>::read_events_no_consensus().into_iter();
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
