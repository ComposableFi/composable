use crate::*;

use composable_traits::xcm::assets::XcmAssetLocation;
use frame_support::{
	parameter_types,
	traits::{Everything, GenesisBuild},
};
use frame_system as system;
use orml_traits::parameter_type_with_key;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup, Zero},
};
use system::EnsureRoot;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
pub type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = u64;
pub type AssetId = u128;
pub type Amount = i128;
pub type Balance = u64;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CHARLIE: AccountId = 3;
pub const DARWIN: AccountId = 4;

pub const ACCOUNT_FREE_START: AccountId = CHARLIE + 1;

pub const MINIMUM_BALANCE: Balance = 1;

#[allow(dead_code)]
pub const INVALID: AssetId = 0;
pub const ASSET_1: AssetId = 1;
pub const ASSET_2: AssetId = 2;
pub const ASSET_FREE_START: AssetId = ASSET_2 + 1;

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
	type AssetLocation = XcmAssetLocation;
	type Balance = Balance;
	type NativeAssetId = NativeAssetId;
	type NativeCurrency = Balances;
	type LocalTransactor = Tokens;
	// TODO(connor): Use second instance of `Tokens`
	type ForeignTransactor = Tokens;
	type GovernanceRegistry = GovernanceRegistry;
	type WeightInfo = ();
	type AdminOrigin = EnsureRoot<AccountId>;
	type AssetLookup = AssetsRegistry;
}

parameter_types! {
	pub const AssetSymbolMaxChars: u32 = 16;
	pub const AssetNameMaxChars: u32 = 32;
}

impl assets_registry::Config for Test {
	type Event = Event;
	type LocalAssetId = AssetId;
	type ForeignAssetId = XcmAssetLocation;
	type UpdateAssetRegistryOrigin = EnsureRoot<AccountId>;
	type ParachainOrGovernanceOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
	type Balance = Balance;
	type AssetSymbolMaxChars = AssetSymbolMaxChars;
	type AssetNameMaxChars = AssetNameMaxChars;
}

parameter_types! {
	pub const MaxLocks: u32 = 256;
}

type ReserveIdentifier = [u8; 8];
impl orml_tokens::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = AssetId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = MaxLocks;
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = frame_support::traits::ConstU32<2>;
	type DustRemovalWhitelist = Everything;
	type OnNewTokenAccount = ();
	type OnKilledTokenAccount = ();
}

impl governance_registry::Config for Test {
	type AssetId = AssetId;
	type WeightInfo = ();
	type Event = Event;
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
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
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
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

pub const BALANCES: [(AccountId, Balance); 4] =
	[(ALICE, 1000), (BOB, 1000), (CHARLIE, 1000), (DARWIN, 1000)];

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	let genesis = pallet_balances::GenesisConfig::<Test> { balances: Vec::from(BALANCES) };
	genesis.assimilate_storage(&mut t).unwrap();
	t.into()
}

pub fn new_test_ext_multi_currency() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into();

	let balances: Vec<(AccountId, AssetId, Balance)> =
		vec![(ALICE, ASSET_1, 1000), (BOB, ASSET_2, 1000)];

	orml_tokens::GenesisConfig::<Test> { balances }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
