use self::currency_factory::MockCurrencyId;
use crate as pallet_vault;
use frame_support::{construct_runtime, parameter_types, traits::GenesisBuild};
use frame_system as system;
use num_traits::Zero;
use orml_traits::parameter_type_with_key;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{ConvertInto, IdentityLookup},
};

pub mod currency_factory;
pub mod strategy;

pub type BlockNumber = u64;
pub type AccountId = u32;
pub type Balance = u128;
pub type Amount = i128;

pub const MINIMUM_BALANCE: Balance = 10000;

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
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type BaseCallFilter = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}

parameter_types! {
    pub const MaxStrategies: usize = 255;
}

impl pallet_vault::Config for Test {
    type Event = Event;
    type Currency = Tokens;
    type CurrencyId = MockCurrencyId;
    type Balance = Balance;
    type MaxStrategies = MaxStrategies;
    type StrategyReport = ();
    type CurrencyFactory = Factory;
    type Convert = ConvertInto;
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
        Vault: pallet_vault::{Pallet, Call, Storage, Event<T>},
        Factory: crate::mocks::currency_factory::{Pallet, Call, Storage, Event<T>},
    }
);

pub struct ExtBuilder {
    balances: Vec<(AccountId, MockCurrencyId, Balance)>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            balances: Vec::new(),
        }
    }
}

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        orml_tokens::GenesisConfig::<Test> {
            balances: self.balances,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        t.into()
    }
}
