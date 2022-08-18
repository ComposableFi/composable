pub use crate::mock::assets::*;
use crate::{self as clearing_house};
use composable_traits::{defi::DeFiComposableConfig, time::DurationSeconds};
use frame_support::{
	ord_parameter_types, parameter_types,
	traits::{ConstU16, ConstU32, ConstU64, EnsureOneOf, Everything, GenesisBuild},
	PalletId,
};
use frame_system as system;
use frame_system::{EnsureRoot, EnsureSignedBy};
use hex_literal::hex;
use orml_traits::parameter_type_with_key;
use sp_core::{sr25519, sr25519::Signature, H256};
use sp_runtime::{
	testing::{Header, TestXt},
	traits::{BlakeTwo256, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, Verify},
	FixedI128, FixedU128,
};

// -------------------------------------------------------------------------------------------------
//                                             Construct Runtime
// -------------------------------------------------------------------------------------------------

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

// Configure a mock runtime to test the pallet
frame_support::construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Balances: pallet_balances,
		GovernanceRegistry: governance_registry,
		Timestamp: pallet_timestamp,
		Tokens: orml_tokens,
		LpTokenFactory: pallet_currency_factory,
		Assets: pallet_assets,
		Vamm: pallet_vamm,
		Oracle: pallet_oracle,
		TestPallet: clearing_house,
	}
);

// -------------------------------------------------------------------------------------------------
//                                         Types & Constants
// -------------------------------------------------------------------------------------------------

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
// pub type AccountId = sp_core::sr25519::Public;
pub type Amount = i64;
pub type Balance = u128;
pub type BlockNumber = u64;
pub type Decimal = FixedI128;
pub type Integer = i128;
pub type MarketId = u64;
pub type ReserveIdentifier = [u8; 8]; // copied from 'frame/assets/src/mocks.rs'
pub type UnsignedDecimal = FixedU128;
pub type VammId = u64;
pub type Moment = u64;

pub const ALICE: AccountId = sp_core::sr25519::Public(hex!(
	"0000000000000000000000000000000000000000000000000000000000000000"
));
pub const BOB: AccountId = sp_core::sr25519::Public(hex!(
	"0000000000000000000000000000000000000000000000000000000000000001"
));
pub const TREASURY: AccountId = sr25519::Public([10_u8; 32]);

// -------------------------------------------------------------------------------------------------
//                                         FRAME System
// -------------------------------------------------------------------------------------------------

impl system::Config for Runtime {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

// -------------------------------------------------------------------------------------------------
//                                           Balances
// -------------------------------------------------------------------------------------------------

parameter_types! {
	pub const NativeExistentialDeposit: Balance = 0;
}

impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = NativeExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = ReserveIdentifier;
}

// -------------------------------------------------------------------------------------------------
//                                       Governance Registry
// -------------------------------------------------------------------------------------------------

impl governance_registry::Config for Runtime {
	type AssetId = AssetId;
	type WeightInfo = ();
	type Event = Event;
}

// -------------------------------------------------------------------------------------------------
//                                           Timestamp
// -------------------------------------------------------------------------------------------------

pub const MINIMUM_PERIOD_SECONDS: DurationSeconds = 5;

parameter_types! {
	pub const MinimumPeriod: u64 = MINIMUM_PERIOD_SECONDS;
}

impl pallet_timestamp::Config for Runtime {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

// -------------------------------------------------------------------------------------------------
//                                           ORML Tokens
// -------------------------------------------------------------------------------------------------

parameter_type_with_key! {
	pub TokensExistentialDeposit: |_currency_id: AssetId| -> Balance {
		0
	};
}

impl orml_tokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = AssetId;
	type WeightInfo = ();
	type ExistentialDeposits = TokensExistentialDeposit;
	type OnDust = ();
	type MaxLocks = ();
	type DustRemovalWhitelist = Everything;
	type MaxReserves = ConstU32<2>; // copied from 'frame/assets/src/mocks.rs'
	type ReserveIdentifier = ReserveIdentifier;
}

// -------------------------------------------------------------------------------------------------
//                                        Currency Factory
// -------------------------------------------------------------------------------------------------

impl pallet_currency_factory::Config for Runtime {
	type Event = Event;
	type AssetId = AssetId;
	type Balance = Balance;
	type AddOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
}

// -------------------------------------------------------------------------------------------------
//                                             Assets
// -------------------------------------------------------------------------------------------------

parameter_types! {
	pub const NativeAssetId: AssetId = PICA;
}

ord_parameter_types! {
	pub const RootAccount: AccountId = ALICE;
}

impl pallet_assets::Config for Runtime {
	type NativeAssetId = NativeAssetId;
	type GenerateCurrencyId = LpTokenFactory;
	type AssetId = AssetId;
	type Balance = Balance;
	type NativeCurrency = Balances;
	type MultiCurrency = Tokens;
	type WeightInfo = ();
	type AdminOrigin = EnsureSignedBy<RootAccount, AccountId>;
	type GovernanceRegistry = GovernanceRegistry;
}

// -------------------------------------------------------------------------------------------------
//                                              VAMM
// -------------------------------------------------------------------------------------------------

impl pallet_vamm::Config for Runtime {
	type Event = Event;
	type VammId = VammId;
	type Balance = Balance;
	type Decimal = UnsignedDecimal;
	type Integer = u128;
	type Moment = Moment;
	type TimeProvider = Timestamp;
}

// -------------------------------------------------------------------------------------------------
//                                             Oracle
//                      This section copied from frame/oracle/src/mocks.rs
// -------------------------------------------------------------------------------------------------

pub type Extrinsic = TestXt<Call, ()>;

impl frame_system::offchain::SigningTypes for Runtime {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Runtime
where
	Call: From<LocalCall>,
{
	type OverarchingCall = Call;
	type Extrinsic = Extrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
	Call: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call,
		_public: <Signature as Verify>::Signer,
		_account: AccountId,
		nonce: u64,
	) -> Option<(Call, <Extrinsic as ExtrinsicT>::SignaturePayload)> {
		Some((call, (nonce, ())))
	}
}

pub type PriceValue = u128;

parameter_types! {
	pub const MaxAnswerBound: u32 = 5;
	pub const MaxAssetsCount: u32 = 2;
	pub const MaxHistory: u32 = 3;
	pub const MaxPrePrices: u32 = 12;
	pub const MinStake: Balance = 1;
	pub const StakeLock: u64 = 1;
	pub const StalePrice: u64 = 2;
	pub const TreasuryAccountId : AccountId = TREASURY;
}

impl pallet_oracle::Config for Runtime {
	type Event = Event;
	type AuthorityId = pallet_oracle::crypto::BathurstStId;
	type Currency = Balances;
	type AssetId = AssetId;
	type PriceValue = PriceValue;
	type StakeLock = StakeLock;
	type StalePrice = StalePrice;
	type MinStake = MinStake;
	type AddOracle = EnsureOneOf<EnsureSignedBy<RootAccount, AccountId>, EnsureRoot<AccountId>>;
	type MaxAnswerBound = MaxAnswerBound;
	type MaxAssetsCount = MaxAssetsCount;
	type MaxHistory = MaxHistory;
	type MaxPrePrices = MaxPrePrices;
	type WeightInfo = ();
	type LocalAssets = ();
	type TreasuryAccount = TreasuryAccountId;
}

// -------------------------------------------------------------------------------------------------
//                                         Clearing House
// -------------------------------------------------------------------------------------------------

impl DeFiComposableConfig for Runtime {
	type Balance = Balance;
	type MayBeAssetId = AssetId;
}

parameter_types! {
	pub const MaxPositions: u32 = 5;
	pub const TestPalletId: PalletId = PalletId(*b"test_pid");
}

impl clearing_house::Config for Runtime {
	type Assets = Assets;
	type Decimal = Decimal;
	type Event = Event;
	type Integer = Integer;
	type MarketId = MarketId;
	type MaxPositions = MaxPositions;
	type Oracle = Oracle;
	type PalletId = TestPalletId;
	type UnixTime = Timestamp;
	type Vamm = Vamm;
	type VammConfig = composable_traits::vamm::VammConfig<Balance, Moment>;
	type VammId = VammId;
	type WeightInfo = ();
}

// -------------------------------------------------------------------------------------------------
//                                    Externalities Builder
// -------------------------------------------------------------------------------------------------

pub struct ExtBuilder {
	pub native_balances: Vec<(AccountId, Balance)>,
	pub balances: Vec<(AccountId, AssetId, Balance)>,
	pub collateral_type: Option<AssetId>,
	pub max_price_divergence: Decimal,
}

impl ExtBuilder {
	#[allow(clippy::disallowed_methods)]
	pub fn build(self) -> sp_io::TestExternalities {
		let mut storage =
			frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

		pallet_balances::GenesisConfig::<Runtime> { balances: self.native_balances }
			.assimilate_storage(&mut storage)
			.unwrap();

		orml_tokens::GenesisConfig::<Runtime> { balances: self.balances }
			.assimilate_storage(&mut storage)
			.unwrap();

		clearing_house::GenesisConfig::<Runtime> {
			collateral_type: self.collateral_type,
			max_price_divergence: self.max_price_divergence,
		}
		.assimilate_storage(&mut storage)
		.unwrap();

		pallet_vamm::GenesisConfig::<Runtime>::default()
			.assimilate_storage(&mut storage)
			.unwrap();

		storage.into()
	}
}
