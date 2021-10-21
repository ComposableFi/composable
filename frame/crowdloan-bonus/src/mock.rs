use crate as pallet_crowdloan_bonus;
use frame_support::{ord_parameter_types, parameter_types, traits::Everything, PalletId};
use frame_system as system;
use frame_system::EnsureSignedBy;
use num_traits::Zero;
use orml_traits::parameter_type_with_key;
use primitives::currency::CurrencyId;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
pub type Amount = i128;
pub type Balance = u128;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		LiquidCrowdloan: pallet_crowdloan_bonus::{Pallet, Call, Storage, Event<T>},
		Tokens: orml_tokens::{Pallet, Storage, Event<T>, Config<T>},
		NativeBalances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Sudo: sudo::{Pallet, Call, Config<T>, Storage, Event<T>},
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
	type AccountId = u128;
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
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		Zero::zero()
	};
}

impl orml_tokens::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = ();
	type DustRemovalWhitelist = Everything;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 5;
}

impl pallet_balances::Config for Test {
	type Balance = u64;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

parameter_types! {
	pub const LiquidRewardId: PalletId = PalletId(*b"Liquided");
}

ord_parameter_types! {
	pub const RootAccount: u128 = 2;
	pub const CrowdloanCurrencyId: CurrencyId = CurrencyId::CROWD_LOAN;
	pub const TokenTotal: Balance = 200;
}

impl sudo::Config for Test {
	type Call = Call;
	type Event = Event;
}

impl pallet_crowdloan_bonus::Config for Test {
	type Event = Event;
	type LiquidRewardId = LiquidRewardId;
	type CurrencyId = CrowdloanCurrencyId;
	type JumpStart = EnsureSignedBy<RootAccount, u128>;
	type Currency = Tokens;
	type TokenTotal = TokenTotal;
	type Balance = Balance;
	type NativeCurrency = NativeBalances;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
