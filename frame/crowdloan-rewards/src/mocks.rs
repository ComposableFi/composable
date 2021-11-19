use crate::{self as pallet_crowdloan_rewards, models::RemoteAccount};
use frame_support::{
	construct_runtime, parameter_types,
	traits::{EnsureOrigin, Everything, GenesisBuild},
	PalletId,
};
use frame_system as system;
use orml_traits::parameter_type_with_key;
use sp_core::H256;
use sp_keystore::{testing::KeyStore, SyncCryptoStore};
use sp_runtime::{
	testing::Header,
	traits::{ConvertInto, IdentityLookup, One},
	Perbill,
};
use system::{EnsureOneOf, EnsureRoot, EnsureSigned, EnsureSignedBy};

pub type CurrencyId = u64;
pub type BlockNumber = u64;
pub type AccountId = u64;
pub type RelayChainAccountId = [u8; 32];
pub type Balance = u128;
pub type Amount = i128;

pub const MILLISECS_PER_BLOCK: u64 = 6000;
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;
pub const WEEKS: BlockNumber = DAYS * 7;

pub const VESTING_STEP: BlockNumber = 1 * WEEKS;
pub const INITIAL_PAYMENT: Perbill = Perbill::from_percent(50);

pub const MINIMUM_BALANCE: Balance = 1000;

pub const ALICE: AccountId = 0;
pub const BOB: AccountId = 1;
pub const CHARLIE: AccountId = 2;
pub const JEREMY: AccountId = 3;
pub const ACCOUNT_FREE_START: AccountId = JEREMY + 1;

pub const ACCOUNT_INITIAL_AMOUNT: u128 = 1_000_000;

// picasso-{account_id}
pub const PROOF_PREFIX: &[u8] = b"picasso-";

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
	type AccountData = balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = Everything;
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
}

parameter_types! {
		pub const ExistentialDeposit: u64 = 3;
}

impl balances::Config for Test {
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type MaxLocks = ();
	type ReserveIdentifier = [u8; 8];
	type MaxReserves = ();
	type WeightInfo = ();
}

parameter_types! {
	pub const InitialPayment: Perbill = INITIAL_PAYMENT;
	pub const VestingStep: BlockNumber = VESTING_STEP;
	pub const Prefix: &'static [u8] = PROOF_PREFIX;
}

impl pallet_crowdloan_rewards::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type Balance = Balance;
	type Convert = ConvertInto;
	type RelayChainAccountId = RelayChainAccountId;
	type InitialPayment = InitialPayment;
	type VestingStep = VestingStep;
	type Prefix = Prefix;
	type AdminOrigin = EnsureRoot<AccountId>;
	type AssociationOrigin = EnsureRoot<AccountId>;
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
		Balances: balances::{Pallet, Storage, Event<T>, Config<T>},
	  CrowdloanRewards: pallet_crowdloan_rewards::{Pallet, Storage, Call, Event<T>},
	}
);

pub struct ExtBuilder {
	balances: Vec<(AccountId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self { balances: Default::default() }
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		balances::GenesisConfig::<Test> { balances: self.balances }
			.assimilate_storage(&mut t)
			.unwrap();
		t.into()
	}
}
