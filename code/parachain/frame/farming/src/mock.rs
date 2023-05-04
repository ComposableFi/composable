use crate::{self as farming, Config, Error};
use frame_support::{
	parameter_types,
	traits::{ConstU32, Everything},
	PalletId,
};
use orml_traits::parameter_type_with_key;
use sp_arithmetic::FixedI128;
use sp_core::H256;
use sp_runtime::{
	generic::Header as GenericHeader,
	traits::{AccountIdConversion, BlakeTwo256, IdentityLookup},
};

type Header = GenericHeader<BlockNumber, BlakeTwo256>;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
		Tokens: orml_tokens::{Pallet, Storage, /*Config<T>,*/ Event<T>},
		Rewards: reward::{Pallet, Call, Storage, Event<T>},
		Farming: farming::{Pallet, Call, Storage, Event<T>},
	}
);

pub type CurrencyId = u128;

pub type AccountId = u64;
pub type Balance = u128;
pub type BlockNumber = u128;
pub type Index = u64;
pub type SignedFixedPoint = FixedI128;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = Index;
	type BlockNumber = BlockNumber;
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
	pub const MaxLocks: u32 = 50;
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		0
	};
}

impl orml_tokens::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type Amount = i128;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type CurrencyHooks = ();
	type MaxLocks = MaxLocks;
	type DustRemovalWhitelist = Everything;
	type MaxReserves = ConstU32<0>; // we don't use named reserves
	type ReserveIdentifier = (); // we don't use named reserves
}

impl reward::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type SignedFixedPoint = SignedFixedPoint;
	type PoolId = CurrencyId;
	type StakeId = AccountId;
	type CurrencyId = CurrencyId;
}

parameter_types! {
	pub const FarmingPalletId: PalletId = PalletId(*b"farmings");
	pub TreasuryAccountId: AccountId = PalletId(*b"treasury").into_account_truncating();
	pub const RewardPeriod: BlockNumber = 10;
}

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type FarmingPalletId = FarmingPalletId;
	type TreasuryAccountId = TreasuryAccountId;
	type RewardPeriod = RewardPeriod;
	type RewardPools = Rewards;
	type AssetId = CurrencyId;
	type MultiCurrency = Tokens;
	type WeightInfo = ();
}

pub type TestEvent = RuntimeEvent;
pub type TestError = Error<Test>;

pub struct ExtBuilder;

impl ExtBuilder {
	pub fn build() -> sp_io::TestExternalities {
		let storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

		storage.into()
	}
}

pub fn run_test<T>(test: T)
where
	T: FnOnce(),
{
	ExtBuilder::build().execute_with(|| {
		System::set_block_number(1);
		test();
	});
}
