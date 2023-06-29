use crate::{self as pallet_assets_registry, weights::SubstrateWeight};
use frame_support::{
	ord_parameter_types, parameter_types,
	traits::{EitherOfDiverse, Everything},
};
use frame_system::{self as system, EnsureRoot, EnsureSignedBy};
use primitives::currency::ForeignAssetId;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, ConvertInto, IdentityLookup},
};

pub type AccountId = u32;
type Balance = u128;
type Block = frame_system::mocking::MockBlock<Runtime>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;

pub const ROOT: AccountId = 0_u32;
pub const ALICE: AccountId = 1_u32;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		AssetsRegistry: pallet_assets_registry,
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Runtime {
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

parameter_types! {
	pub const NativeED: Balance = 0;
}

type AssetId = u128;

impl pallet_assets_registry::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type LocalAssetId = AssetId;
	type Balance = Balance;
	type ForeignAssetId = ForeignAssetId;
	type UpdateAssetRegistryOrigin = EitherOfDiverse<
		EnsureSignedBy<RootAccount, AccountId>, // for tests
		EnsureRoot<AccountId>,                  // for benchmarks
	>;
	type ParachainOrGovernanceOrigin = EitherOfDiverse<
		EnsureSignedBy<RootAccount, AccountId>, // for tests
		EnsureRoot<AccountId>,                  // for benchmarks
	>;
	type WeightInfo = SubstrateWeight<Self>;
	type Convert = ConvertInto;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default()
		.build_storage::<Runtime>()
		.expect("Storage is valid")
		.into()
}
