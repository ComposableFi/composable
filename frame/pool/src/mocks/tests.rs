use super::currency_factory::MockCurrencyId;
use crate as pallet_pool;

use vault as pallet_vault;

use frame_support::{
	construct_runtime, parameter_types,
	traits::{Everything, GenesisBuild},
	PalletId,
};
use frame_system as system;
use num_traits::Zero;
use orml_traits::parameter_type_with_key;
use sp_core::H256;
use sp_runtime::{
	Perquintill,
	testing::Header,
	traits::{ConvertInto, IdentityLookup},
};

pub type BlockNumber = u64;
pub type AccountId = u128;
pub type Balance = u128;
pub type Amount = i128;
pub type Weight = Perquintill;

pub const MINIMUM_DEPOSIT: Balance = 1_000;
pub const MAXIMUM_DEPOSIT: Balance = 10_000;

pub const ALICE: AccountId = 0;
pub const BOB: AccountId = 1;
pub const CHARLIE: AccountId = 2;
// pub const JEREMY: AccountId = 3;
// pub const ACCOUNT_FREE_START: AccountId = JEREMY + 1;

// pub const ACCOUNT_INITIAL_AMOUNT: u128 = 1_000_000;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl system::Config for Test {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Call = Call;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type BlockWeights = ();
	type BlockLength = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = Everything;
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
}

parameter_types! {
	pub const BalanceExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = BalanceExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

parameter_types! {
	pub const MaxStrategies: usize = 255;
	pub const CreationDeposit: Balance = 10;
	pub const ExistentialDeposit: Balance = 1000;
	pub const RentPerBlock: Balance = 1;
	pub const TestPalletID: PalletId = PalletId(*b"test_pid");
	pub const StrategyTestPalletID: PalletId = PalletId(*b"sest_pid");
	pub const MinimumDeposit: Balance = 0;
	pub const MinimumWithdrawal: Balance = 0;
	pub const TombstoneDuration: BlockNumber = 10;
}

impl pallet_vault::Config for Test {
	type Event = Event;
	type Currency = Tokens;
	type AssetId = MockCurrencyId;
	type Balance = Balance;
	type MaxStrategies = MaxStrategies;
	type CurrencyFactory = Factory;
	type Convert = ConvertInto;
	type PalletId = TestPalletID;
	type CreationDeposit = CreationDeposit;
	type ExistentialDeposit = ExistentialDeposit;
	type RentPerBlock = RentPerBlock;
	type NativeCurrency = Balances;
	type MinimumDeposit = MinimumDeposit;
	type MinimumWithdrawal = MinimumWithdrawal;
	type TombstoneDuration = TombstoneDuration;
	type VaultId = u64;
	type WeightInfo = ();
}

parameter_types! {
	pub const NativeAssetId: MockCurrencyId = MockCurrencyId::A;
	pub const TestPoolPalletID: PalletId = PalletId(*b"testpool");
	
	// pub Epsilon: Perquintill = Perquintill::from_float(0.0000000000000001);
	pub Epsilon: Perquintill = Weight::from_float(0.01);
}

impl pallet_pool::Config for Test {
	type Event = Event;
	type Vault = Vaults;
	type Currency = Tokens;
	type CurrencyFactory = Factory;
	type PoolId = u64;
	type NativeAssetId = NativeAssetId;
	type AssetId = MockCurrencyId;
	type Balance = Balance;
	type Weight = Weight;
	type Convert = ConvertInto;
	type CreationFee = CreationDeposit;
	type ExistentialDeposit = ExistentialDeposit;
	type Epsilon = Epsilon;
	type PalletId = TestPoolPalletID;
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
	type DustRemovalWhitelist = Everything;
}

impl crate::mocks::currency_factory::Config for Test {
	type Event = Event;
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
		Tokens: orml_tokens::{Pallet, Storage, Event<T>, Config<T>},
		Factory: crate::mocks::currency_factory::{Pallet, Call, Storage, Event<T>},
		Vaults: pallet_vault::{Pallet, Call, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},

        Pools: pallet_pool::{Pallet, Call, Storage, Event<T>},
	}
);

pub struct ExtBuilder {
	balances: Vec<(AccountId, MockCurrencyId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self { balances: Vec::new() }
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

		orml_tokens::GenesisConfig::<Test> { balances: self.balances }
			.assimilate_storage(&mut t)
			.unwrap();

		t.into()
	}
}