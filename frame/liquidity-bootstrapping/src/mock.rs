use crate as pallet_liquidity_bootstrapping;
use frame_support::{parameter_types, traits::Everything, PalletId};
use frame_system as system;
use orml_traits::parameter_type_with_key;
use sp_arithmetic::traits::Zero;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, ConvertInto, IdentityLookup},
	Permill,
};
use system::EnsureRoot;

pub type AccountId = u128;
pub type AssetId = u128;
pub type Balance = u128;
pub type Amount = i128;
pub type PoolId = u32;
pub type BlockNumber = u64;

#[allow(dead_code)]
pub static ALICE: AccountId = 1;
#[allow(dead_code)]
pub static BOB: AccountId = 2;
#[allow(dead_code)]
pub static CHARLIE: AccountId = 3;

#[allow(dead_code)]
pub const PROJECT_TOKEN: AssetId = 0;
#[allow(dead_code)]
pub const USDT: AssetId = 1;

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
		Tokens: orml_tokens::{Pallet, Call, Storage, Config<T>, Event<T>},
	  CurrencyFactory: pallet_currency_factory::{Pallet, Storage, Event<T>},
		LBP: pallet_liquidity_bootstrapping::{Pallet, Storage, Event<T>},
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
	type BlockNumber = BlockNumber;
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

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: AssetId| -> Balance {
		Zero::zero()
	};
}

impl orml_tokens::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = AssetId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = ();
	type DustRemovalWhitelist = Everything;
}

impl pallet_currency_factory::Config for Test {
	type Event = Event;
	type AssetId = AssetId;
	type AddOrigin = EnsureRoot<AccountId>;
	type Balance = Balance;
	type WeightInfo = ();
}

parameter_types! {
	pub LBPId: PalletId = PalletId(*b"pall_lbp");
  pub MinSaleDuration: BlockNumber = 3600 / 12;
  pub MaxSaleDuration: BlockNumber = 30 * 24 * 3600 / 12;
  pub MaxInitialWeight: Permill = Permill::from_percent(95);
  pub MinFinalWeight: Permill = Permill::from_percent(5);
}

impl pallet_liquidity_bootstrapping::Config for Test {
	type Event = Event;
	type AssetId = AssetId;
	type Balance = Balance;
	type Convert = ConvertInto;
	type Assets = Tokens;
	type PoolId = PoolId;
	type LocalAssets = CurrencyFactory;
	type PalletId = LBPId;
	type MinSaleDuration = MinSaleDuration;
	type MaxSaleDuration = MaxSaleDuration;
	type MaxInitialWeight = MaxInitialWeight;
	type MinFinalWeight = MinFinalWeight;
	type WeightInfo = ();
	type AdminOrigin = EnsureRoot<AccountId>;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
