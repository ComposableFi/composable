//! Mocks for the bonded finance pallet.

#![cfg(test)]

use super::*;
use frame_support::{
	construct_runtime,
	pallet_prelude::*,
	parameter_types,
	traits::{EnsureOrigin, Everything},
	PalletId,
};
use frame_system::{EnsureRoot, RawOrigin};
use orml_tokens::CurrencyAdapter;
use orml_traits::parameter_type_with_key;
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{ConvertInto, IdentityLookup, Zero},
};

pub type BlockNumber = u64;
pub type Moment = u64;
pub type Balance = u128;
pub type Amount = i128;
pub type AccountId = u128;

pub const MIN_VESTED_TRANSFER: u128 = 100;
pub const NATIVE_CURRENCY_ID: MockCurrencyId = MockCurrencyId::PICA;
pub const MIN_REWARD: u128 = 1_000_000;
pub const MILLISECS_PER_BLOCK: u64 = 6000;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CHARLIE: AccountId = 3;

#[derive(
	PartialOrd,
	Ord,
	PartialEq,
	Eq,
	Debug,
	Copy,
	Clone,
	codec::Encode,
	codec::Decode,
	codec::MaxEncodedLen,
	serde::Serialize,
	serde::Deserialize,
	TypeInfo,
	proptest_derive::Arbitrary,
)]
#[allow(clippy::upper_case_acronyms)] // currencies should be CONSTANT_CASE
pub enum MockCurrencyId {
	PICA,
	BTC,
	ETH,
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = BlockNumber;
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
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = Everything;
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

pub struct EnsureAliceOrBob;
impl EnsureOrigin<RuntimeOrigin> for EnsureAliceOrBob {
	type Success = AccountId;

	fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
		Into::<Result<RawOrigin<AccountId>, RuntimeOrigin>>::into(o).and_then(|o| match o {
			RawOrigin::Signed(ALICE) => Ok(ALICE),
			RawOrigin::Signed(BOB) => Ok(BOB),
			r => Err(RuntimeOrigin::from(r)),
		})
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn successful_origin() -> RuntimeOrigin {
		RuntimeOrigin::from(RawOrigin::Signed(Default::default()))
	}
}

parameter_type_with_key! {
	  pub ExistentialDeposits: |_currency_id: MockCurrencyId| -> Balance {
			Zero::zero()
	  };
}

parameter_types! {
	  pub MaxLocks: u32 = 64;
}

type ReserveIdentifier = [u8; 8];
impl orml_tokens::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = MockCurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = MaxLocks;
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = frame_support::traits::ConstU32<2>;
	type DustRemovalWhitelist = Everything;
	type OnNewTokenAccount = ();
	type OnKilledTokenAccount = ();
  type OnSlash = ();
  type OnDeposit = ();
  type OnTransfer = ();
}

parameter_types! {
	pub const MinimumPeriod: u64 = MILLISECS_PER_BLOCK / 2;
}

impl pallet_timestamp::Config for Runtime {
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub const MaxVestingSchedule: u32 = 2;
	pub const MinVestedTransfer: u64 = MIN_VESTED_TRANSFER as _;
}

impl pallet_vesting::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Tokens;
	type MinVestedTransfer = MinVestedTransfer;
	type VestedTransferOrigin = EnsureRoot<AccountId>;
	type UpdateSchedulesOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
	type MaxVestingSchedules = MaxVestingSchedule;
	type Moment = Moment;
	type Time = Timestamp;
	type VestingScheduleId = u128;
}

parameter_types! {
	// cspell:disable-next
	pub const BondedFinanceId: PalletId = PalletId(*b"bondedfi");
	pub const Stake: Balance = 10_000;
	pub const NativeCurrencyId: MockCurrencyId = NATIVE_CURRENCY_ID;
	pub const MinReward: Balance = MIN_REWARD;
}

impl Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type NativeCurrency = CurrencyAdapter<Runtime, NativeCurrencyId>;
	type Currency = Tokens;
	type Vesting = Vesting;
	type BondOfferId = u64;
	type Convert = ConvertInto;
	type PalletId = BondedFinanceId;
	type Stake = Stake;
	type MinReward = MinReward;
	type AdminOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
		Vesting: pallet_vesting::{Pallet, Storage, Call, Event<T>, Config<T>},
		Tokens: orml_tokens::{Pallet, Call, Storage, Config<T>, Event<T>},
		BondedFinance: pallet::{Pallet, Call, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
	}
);

pub struct ExtBuilder;

impl ExtBuilder {
	pub fn build() -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

		orml_tokens::GenesisConfig::<Runtime> { balances: vec![] }
			.assimilate_storage(&mut t)
			.unwrap();

		pallet_vesting::GenesisConfig::<Runtime> { vesting: vec![] }
			.assimilate_storage(&mut t)
			.unwrap();

		t.into()
	}
}
