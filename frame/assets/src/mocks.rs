use crate::*;

use composable_traits::currency::CurrencyFactory;
use frame_support::{parameter_types, traits::Everything};
use frame_system as system;
use num_traits::Zero;
use orml_traits::parameter_type_with_key;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use system::EnsureRoot;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
pub type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = u64;
pub type AssetId = u64;
pub type Amount = i128;
pub type Balance = u64;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>} = 1,
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>} = 2,
		GovernanceRegistry: governance_registry::{Pallet, Call, Storage, Event<T>} = 3,
		Tokens: orml_tokens::{Pallet, Call, Storage, Config<T>, Event<T>} = 4,
		Assets: crate::{Pallet, Call, Storage} = 5,
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

pub struct CurrencyIdGenerator;

impl CurrencyFactory<AssetId> for CurrencyIdGenerator {
	fn create() -> Result<AssetId, sp_runtime::DispatchError> {
		Ok(1_u64)
	}
}

impl Config for Test {
	type AssetId = AssetId;
	type Balance = Balance;
	type NativeAssetId = NativeAssetId;
	type GenerateCurrencyId = CurrencyIdGenerator;
	type NativeCurrency = Balances;
	type MultiCurrency = Tokens;
	type GovernanceRegistry = GovernanceRegistry;
	type WeightInfo = ();
	type AdminOrigin = EnsureRoot<AccountId>;
}

impl orml_tokens::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = AssetId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = ();
	type DustRemovalWhitelist = Everything;
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

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	let genesis = pallet_balances::GenesisConfig::<Test> {
		balances: vec![(0, 1000), (1, 1000), (2, 1000), (3, 1000)],
	};
	genesis.assimilate_storage(&mut t).unwrap();
	t.into()
}
