use crate as pallet_assets_registry;
pub use composable_traits::assets::XcmAssetLocation;
use frame_support::{ord_parameter_types, parameter_types, traits::Everything};
use frame_system as system;
use frame_system::EnsureSignedBy;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

pub type AccountId = u32;
type Block = frame_system::mocking::MockBlock<Test>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;

pub const ROOT: AccountId = 0_u32;
pub const ALICE: AccountId = 1_u32;
pub const BOB: AccountId = 2_u32;
pub const CHARLIE: AccountId = 3_u32;

pub const DECIMALS: u8 = 12;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		AssetsRegistry: pallet_assets_registry::{Pallet, Call, Storage, Event<T>},
	}
);

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
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

ord_parameter_types! {
	pub const RootAccount: AccountId = ROOT;
}

impl pallet_assets_registry::Config for Test {
	type Event = Event;
	type LocalAssetId = u128;
	type ForeignAssetId = u128;
	type Location = XcmAssetLocation;
	type UpdateAdminOrigin = EnsureSignedBy<RootAccount, AccountId>;
	type LocalAdminOrigin = pallet_assets_registry::EnsureLocalAdmin<Test>;
	type ForeignAdminOrigin = pallet_assets_registry::EnsureForeignAdmin<Test>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
