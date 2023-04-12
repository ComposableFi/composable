use crate::*;

use frame_support::{parameter_types, traits::Everything};
use frame_system as system;
use orml_traits::parameter_type_with_key;
use primitives::currency::ForeignAssetId;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, ConvertInto, IdentityLookup, Zero},
};
use system::EnsureRoot;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
pub type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = u128;
pub type AssetId = u128;
pub type Amount = i128;
pub type Balance = u128;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;

pub const MINIMUM_BALANCE: Balance = 1;

#[allow(dead_code)]
pub const INVALID: AssetId = 0;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system = 1,
		Balances: pallet_balances = 2,
		GovernanceRegistry: governance_registry = 3,
		Tokens: orml_tokens = 4,
		AssetsRegistry: assets_registry = 5,
		AssetsTransactorRouter: crate = 6,

	}
);

parameter_type_with_key! {
	pub ExistentialDeposits: |_a: AssetId| -> Balance {
		Zero::zero()
	};
}

parameter_types! {
	pub const NativeAssetId: AssetId = 1;
}

impl Config for Test {
	type AssetId = AssetId;
	type AssetLocation = ForeignAssetId;
	type Balance = Balance;
	type NativeAssetId = NativeAssetId;
	type NativeTransactor = Balances;
	type LocalTransactor = Tokens;
	type ForeignTransactor = Tokens;
	type GovernanceRegistry = GovernanceRegistry;
	type WeightInfo = ();
	type AdminOrigin = EnsureRoot<AccountId>;
	type AssetsRegistry = AssetsRegistry;
}

impl assets_registry::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type LocalAssetId = AssetId;
	type ForeignAssetId = ForeignAssetId;
	type UpdateAssetRegistryOrigin = EnsureRoot<AccountId>;
	type ParachainOrGovernanceOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
	type Balance = Balance;
	type Convert = ConvertInto;
}

parameter_types! {
	pub const MaxLocks: u32 = 256;
}

pub struct CurrencyHooks;
impl orml_traits::currency::MutationHooks<AccountId, AssetId, Balance> for CurrencyHooks {
	type OnDust = ();
	type OnSlash = ();
	type PreDeposit = ();
	type PostDeposit = ();
	type PreTransfer = ();
	type PostTransfer = ();
	type OnNewTokenAccount = ();
	type OnKilledTokenAccount = ();
}

type ReserveIdentifier = [u8; 8];
impl orml_tokens::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = AssetId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type MaxLocks = MaxLocks;
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = frame_support::traits::ConstU32<2>;
	type DustRemovalWhitelist = Everything;
	type CurrencyHooks = CurrencyHooks;
}

impl governance_registry::Config for Test {
	type AssetId = AssetId;
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default()
		.build_storage::<Test>()
		.expect("able to construct mock storage")
		.into()
}
