//! Mocks for the vesting module.

#![cfg(test)]

use super::*;
use composable_traits::vesting::VestingWindow::{BlockNumberBased, MomentBased};
use frame_support::{
	construct_runtime, parameter_types,
	traits::{EnsureOrigin, Everything},
};
use frame_system::{EnsureRoot, RawOrigin};
use orml_traits::parameter_type_with_key;
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{IdentityLookup, TrailingZeroInput},
};

use crate as vesting;

pub type Moment = u64;
pub type Balance = u64;
pub type Amount = i64;
pub type AccountId = u128;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CHARLIE: AccountId = 3;
pub const MILLISECS_PER_BLOCK: u64 = 6000;

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
)]
#[allow(clippy::upper_case_acronyms)] // currencies should be CONSTANT_CASE
pub enum MockCurrencyId {
	BTC,
	ETH,
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Runtime {
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
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
	type BaseCallFilter = Everything;
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
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

fn benchmark_vested_transfer_account() -> AccountId {
	AccountId::decode(&mut TrailingZeroInput::zeroes())
		.expect("infinite length input; no invalid inputs for type; qed")
}

pub struct EnsureAliceOrBob;
impl EnsureOrigin<Origin> for EnsureAliceOrBob {
	type Success = AccountId;

	fn try_origin(o: Origin) -> Result<Self::Success, Origin> {
		let benchmark_acc = benchmark_vested_transfer_account();
		Into::<Result<RawOrigin<AccountId>, Origin>>::into(o).and_then(|o| match o {
			RawOrigin::Signed(ALICE) => Ok(ALICE),
			RawOrigin::Signed(BOB) => Ok(BOB),
			RawOrigin::Signed(acc) =>
				if acc == benchmark_acc {
					Ok(benchmark_acc)
				} else {
					Err(Origin::from(RawOrigin::Signed(acc)))
				},
			r => Err(Origin::from(r)),
		})
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn successful_origin() -> Origin {
		Origin::from(RawOrigin::Signed(benchmark_vested_transfer_account()))
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

impl orml_tokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = MockCurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = MaxLocks;
	type DustRemovalWhitelist = Everything;
}

parameter_types! {
	pub const MaxVestingSchedule: u32 = 3;
	pub const MinVestedTransfer: u64 = 5;
}

impl Config for Runtime {
	type Event = Event;
	type Currency = Tokens;
	type MinVestedTransfer = MinVestedTransfer;
	type VestedTransferOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
	type MaxVestingSchedules = MaxVestingSchedule;
	type Moment = Moment;
	type Time = Timestamp;
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
		Vesting: vesting::{Pallet, Storage, Call, Event<T>, Config<T>},
		Tokens: orml_tokens::{Pallet, Call, Storage, Config<T>, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
	}
);

#[derive(Default)]
pub struct ExtBuilder;

impl ExtBuilder {
	pub fn build() -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

		orml_tokens::GenesisConfig::<Runtime> {
			balances: vec![(ALICE, MockCurrencyId::BTC, 100), (CHARLIE, MockCurrencyId::BTC, 65)],
		}
		.assimilate_storage(&mut t)
		.unwrap();

		vesting::GenesisConfig::<Runtime> {
			vesting: vec![
				// asset, who, VestingWindow {start, period}, period_count, per_period
				(MockCurrencyId::BTC, CHARLIE, BlockNumberBased { start: 2, period: 3 }, 1, 5),
				(MockCurrencyId::BTC, CHARLIE, BlockNumberBased { start: 2 + 3, period: 3 }, 3, 5),
				(MockCurrencyId::BTC, CHARLIE, MomentBased { start: 40000, period: 50000 }, 3, 5),
			],
		}
		.assimilate_storage(&mut t)
		.unwrap();

		t.into()
	}
}
