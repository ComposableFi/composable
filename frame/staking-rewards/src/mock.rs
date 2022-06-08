use crate::*;
use composable_traits::{
	financial_nft::{FinancialNftProtocol, NftClass, NftVersion},
	time::DurationSeconds,
};
use frame_support::{
	parameter_types,
	traits::{Everything, Hooks},
	PalletId,
};
use frame_system as system;
use orml_traits::parameter_type_with_key;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup, Zero},
};
use system::EnsureRoot;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
pub type Block = frame_system::mocking::MockBlock<Test>;

pub type AccountId = u128;
pub type AssetId = u128;
pub type Balance = u128;
pub type Amount = i128;
pub type Moment = u64;
pub type InstanceId = u128;
pub type BlockNumber = u64;

pub const MILLISECS_PER_BLOCK: Moment = 12_000;

/// One minute in term of block
pub const REWARD_EPOCH_DURATION_BLOCK: BlockNumber = 60_000 / MILLISECS_PER_BLOCK;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>} = 1,
		Tokens: orml_tokens::{Pallet, Call, Storage, Config<T>, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
		NFT: pallet_nft::{Pallet, Storage , Event<T>},
		StakingRewards: crate::{Pallet, Storage, Call, Event<T>},
	}
);

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: AssetId| -> Balance {
		Zero::zero()
	};
}

parameter_types! {
	pub const MinimumPeriod: Moment = MILLISECS_PER_BLOCK / 2;
}

impl pallet_timestamp::Config for Test {
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub MaxLocks: u32 = 64;
}

type ReserveIdentifier = [u8; 8];
impl orml_tokens::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = AssetId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = MaxLocks;
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = frame_support::traits::ConstU32<2>;
	type DustRemovalWhitelist = Everything;
}

impl pallet_nft::Config for Test {
	type Event = Event;
}

parameter_types! {
	pub const StakingRewardPalletId: PalletId = PalletId(*b"pal_stkr");
	pub const MaxStakingPresets: u32 = 10;
	pub const MaxRewardAssets: u32 = 10;
	pub const EpochDuration: DurationSeconds = MILLISECS_PER_BLOCK * REWARD_EPOCH_DURATION_BLOCK / 1000;
	pub const ElementToProcessPerBlock: u32 = 100;
}

impl Config for Test {
	type Event = Event;
	type AssetId = AssetId;
	type Balance = Balance;
	type Assets = Tokens;
	type Time = Timestamp;
	type GovernanceOrigin = EnsureRoot<AccountId>;
	type PalletId = StakingRewardPalletId;
	type MaxStakingPresets = MaxStakingPresets;
	type MaxRewardAssets = MaxRewardAssets;
	type EpochDuration = EpochDuration;
	type ElementToProcessPerBlock = ElementToProcessPerBlock;
}

impl FinancialNftProtocol<AccountId> for Test {
	type ClassId = NftClass;
	type InstanceId = InstanceId;
	type Version = NftVersion;
	type NFTProvider = NFT;
}

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

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	t.into()
}

#[allow(dead_code)]
pub fn run_to_block(n: BlockNumber) {
	StakingRewards::on_finalize(System::block_number());
	for b in (System::block_number() + 1)..=n {
		next_block(b);
		if b != n {
			StakingRewards::on_finalize(System::block_number());
		}
	}
}

pub fn process_block(n: BlockNumber) {
	next_block(n);
	StakingRewards::on_finalize(n);
}

pub fn next_block(n: u64) {
	System::set_block_number(n);
	Timestamp::set_timestamp(MILLISECS_PER_BLOCK * n);
	StakingRewards::on_initialize(n);
}
