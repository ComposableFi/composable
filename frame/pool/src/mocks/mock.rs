use super::currency_factory::MockCurrencyId;

use crate as pallet_pool;
use frame_support::parameter_types;
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Pools: pallet_pool::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

// parameter_types! {
// 	pub const MaxStrategies: usize = 255;
// 	pub const NativeAssetId: MockCurrencyId = MockCurrencyId::A;
// 	pub const CreationDeposit: Balance = 10;
// 	pub const ExistentialDeposit: Balance = 1000;
// 	pub const RentPerBlock: Balance = 1;
// 	pub const TestPalletID: PalletId = PalletId(*b"test_pid");
// 	pub const StrategyTestPalletID: PalletId = PalletId(*b"sest_pid");
// 	pub const MinimumDeposit: Balance = 0;
// 	pub const MinimumWithdrawal: Balance = 0;
// }

impl pallet_pool::Config for Test {
	type Event = Event;
	type AssetId = MockCurrencyId;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
