use super::currency_factory::MockCurrencyId;
use crate as pallet_vault;
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
	testing::Header,
	traits::{ConvertInto, IdentityLookup},
};

pub type BlockNumber = u64;
pub type AccountId = u128;
pub type Balance = u128;
pub type Amount = i128;

pub const MINIMUM_BALANCE: Balance = 1000;

pub const ALICE: AccountId = 0;
pub const BOB: AccountId = 1;
pub const CHARLIE: AccountId = 2;
pub const JEREMY: AccountId = 3;
pub const ACCOUNT_FREE_START: AccountId = JEREMY + 1;

pub const ACCOUNT_INITIAL_AMOUNT: u128 = 1_000_000;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl system::Config for Test {
	type RuntimeOrigin = RuntimeOrigin;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type RuntimeCall = RuntimeCall;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
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
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const BalanceExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
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
	pub const NativeAssetId: MockCurrencyId = MockCurrencyId::A;
	pub const CreationDeposit: Balance = 10;
	pub const ExistentialDeposit: Balance = 1000;
	pub const RentPerBlock: Balance = 1;
	pub const TestPalletID: PalletId = PalletId(*b"test_pid");
	// cspell:disable-next
	pub const StrategyTestPalletID: PalletId = PalletId(*b"sest_pid");
	pub const MinimumDeposit: Balance = 0;
	pub const MinimumWithdrawal: Balance = 0;
	pub const TombstoneDuration: BlockNumber = 10;
}

impl pallet_vault::Config for Test {
	type RuntimeEvent = RuntimeEvent;
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

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: MockCurrencyId| -> Balance {
		Zero::zero()
	};
}

type ReserveIdentifier = [u8; 8];
impl orml_tokens::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = MockCurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = ();
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = frame_support::traits::ConstU32<2>;
	type DustRemovalWhitelist = Everything;
	type OnKilledTokenAccount = ();
	type OnNewTokenAccount = ();
	type OnSlash = ();
	type OnDeposit = ();
	type OnTransfer = ();
}

impl crate::mocks::currency_factory::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
}

impl crate::mocks::strategy::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Vault = Vaults;
	type Currency = Tokens;
	type PalletId = StrategyTestPalletID;
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
		Vaults: pallet_vault::{Pallet, Call, Storage, Event<T>},
		Factory: crate::mocks::currency_factory::{Pallet, Call, Storage, Event<T>},
		Strategy: crate::mocks::strategy::{Pallet, Call, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
	}
);

#[derive(Default)]
pub struct ExtBuilder {
	balances: Vec<(AccountId, MockCurrencyId, Balance)>,
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

		pallet_balances::GenesisConfig::<Test> { balances: vec![(ALICE, 1000000)] }
			.assimilate_storage(&mut t)
			.unwrap();

		orml_tokens::GenesisConfig::<Test> { balances: self.balances }
			.assimilate_storage(&mut t)
			.unwrap();

		t.into()
	}
}
