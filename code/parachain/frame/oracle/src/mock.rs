use crate as pallet_oracle;
use crate::*;

use frame_support::{
	ord_parameter_types,
	pallet_prelude::ConstU32,
	parameter_types,
	traits::{EnsureOneOf, Everything},
	PalletId,
};
use frame_system as system;
use frame_system::EnsureSignedBy;
use sp_core::{sr25519, sr25519::Signature, H256};
use sp_runtime::{
	testing::{Header, TestXt},
	traits::{BlakeTwo256, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, Verify},
};
use system::EnsureRoot;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type Moment = composable_traits::time::Timestamp;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Oracle: pallet_oracle::{Pallet, Call, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
	}
);

pub const MILLISECS_PER_BLOCK: u64 = 12000;

parameter_types! {
	pub const MinimumPeriod: u64 = MILLISECS_PER_BLOCK / 2;
}

impl pallet_timestamp::Config for Test {
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

// pub type StalePrice = Get<u64>;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

pub type BlockNumber = u64;

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
	type AccountId = sp_core::sr25519::Public;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

pub type Balance = u128;
impl pallet_balances::Config for Test {
	type Balance = Balance;
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
	pub const StakeLock: u64 = 1;
	pub const MinStake: Balance = 1;
	pub const StalePrice: u64 = 2;
	pub const MaxAnswerBound: u32 = 5;
	pub const MaxAssetsCount: u32 = 2;
	pub const MaxHistory: u32 = 3;
	pub const MaxPrePrices: u32 = 12;
	pub const TwapWindow: u16 = 3;
}

ord_parameter_types! {
	pub const RootAccount: AccountId = get_root_account();
}

pub type Extrinsic = TestXt<Call, ()>;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

impl frame_system::offchain::SigningTypes for Test {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
	Call: From<LocalCall>,
{
	type OverarchingCall = Call;
	type Extrinsic = Extrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Test
where
	Call: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call,
		_public: <Signature as Verify>::Signer,
		_account: AccountId,
		nonce: u64,
	) -> Option<(Call, <Extrinsic as ExtrinsicT>::SignaturePayload)> {
		Some((call, (nonce, ())))
	}
}

pub type AssetId = u128;
pub type PriceValue = u128;

parameter_types! {
	pub const TreasuryAccountId : AccountId= sr25519::Public([10u8; 32]);
	// cspell:disable-next
	pub const OraclePalletId: PalletId = PalletId(*b"plt_orac");
	pub const MsPerBlock: u64 = MILLISECS_PER_BLOCK;
}

impl pallet_oracle::Config for Test {
	type Event = Event;
	type AuthorityId = crypto::BathurstStId;
	type Currency = Balances;
	type AssetId = AssetId;
	type PriceValue = PriceValue;
	type StakeLock = StakeLock;
	type StalePrice = StalePrice;
	type MinStake = MinStake;
	type AddOracle =
		EnsureOneOf<EnsureSignedBy<RootAccount, sp_core::sr25519::Public>, EnsureRoot<AccountId>>;
	type MaxAnswerBound = MaxAnswerBound;
	type MaxAssetsCount = MaxAssetsCount;
	type MaxHistory = MaxHistory;
	type MaxPrePrices = MaxPrePrices;
	type WeightInfo = ();
	type LocalAssets = ();
	type TreasuryAccount = TreasuryAccountId;
	type Moment = Moment;
	type Time = Timestamp;
	type TwapWindow = TwapWindow;
	type RewardOrigin = EnsureRoot<AccountId>;
	type PalletId = OraclePalletId;
	type MsPerBlock = MsPerBlock;
	type Balance = Balance;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	let genesis = pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(get_account_1(), 100),
			(get_root_account(), 100),
			(get_account_4(), 100),
			(get_account_3(), 100),
			(get_account_5(), 100),
			(get_treasury_account(), 100),
		],
	};
	genesis.assimilate_storage(&mut t).unwrap();
	t.into()
}

pub const fn get_account_1() -> AccountId {
	sr25519::Public([1u8; 32])
}

pub const fn get_root_account() -> AccountId {
	sr25519::Public([2u8; 32])
}

pub const fn get_account_3() -> AccountId {
	sr25519::Public([3u8; 32])
}

pub fn get_account_4() -> AccountId {
	sr25519::Public([4u8; 32])
}

pub fn get_account_5() -> AccountId {
	sr25519::Public([5u8; 32])
}

pub fn get_treasury_account() -> AccountId {
	sr25519::Public([10u8; 32])
}
