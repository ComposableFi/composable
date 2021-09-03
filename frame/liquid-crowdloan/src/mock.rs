use crate as pallet_liquid_crowdloan;
use frame_support::{parameter_types, ord_parameter_types, PalletId};
use frame_system as system;
use frame_system::EnsureSignedBy;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup, ConvertInto},
};
use num_traits::Zero;
use orml_traits::parameter_type_with_key;
pub use composable_traits::{
	currency::CurrencyFactory,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
pub type Amount = i128;
pub type Balance = u128;
pub type MockCurrencyId = u128;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		LiquidCrowdloan: pallet_liquid_crowdloan::{Pallet, Call, Storage, Event<T>},
		Tokens: orml_tokens::{Pallet, Storage, Event<T>, Config<T>},
		Factory: pallet_currency_factory::{Pallet, Storage, Event<T>},
		NativeBalances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},

	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = ();
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
	pub ExistentialDeposits: |_currency_id: MockCurrencyId| -> Balance {
		Zero::zero()
	};
}

impl orml_tokens::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = MockCurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = ();
	type DustRemovalWhitelist = ();
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

impl pallet_currency_factory::Config for Test {
	type Event = Event;
	type CurrencyId = MockCurrencyId;
	type Convert = ConvertInto;

}


parameter_types! {
	pub const LiquidRewardId: PalletId = PalletId(*b"Liquided");
}

ord_parameter_types! {
	pub const RootAccount: u128 = 2;
}
impl pallet_liquid_crowdloan::Config for Test {
	type Event = Event;
	type LiquidRewardId = LiquidRewardId;
	type CurrencyFactory =  Factory;
	type CurrencyId = MockCurrencyId;
	type JumpStart = EnsureSignedBy<RootAccount, u128>;
	type Currency = Tokens;
	type Balance = Balance;
	type NativeCurrency = NativeBalances;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
