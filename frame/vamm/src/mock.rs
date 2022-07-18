// Allow use of .unwrap() in tests and unused Results from function calls
#![allow(clippy::disallowed_methods, unused_must_use)]

use crate as pallet_vamm;

use frame_support::{
	parameter_types,
	traits::{Everything, GenesisBuild},
	PalletId,
};
// use frame_system::{EnsureRoot, EnsureSignedBy};
use sp_core::{sr25519::Signature, H256};
use sp_runtime::{
	testing::Header,
	traits::{IdentifyAccount, IdentityLookup, Verify},
	FixedU128,
};

// ----------------------------------------------------------------------------------------------------
//                                           Construct Runtime
// ----------------------------------------------------------------------------------------------------

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<MockRuntime>;
type Block = frame_system::mocking::MockBlock<MockRuntime>;

frame_support::construct_runtime!(
	pub enum MockRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
		TestPallet: pallet_vamm::{Pallet, Storage, Event<T>},
	}
);

pub type Balance = u128;
pub type BlockNumber = u64;
pub type VammId = u128;
pub type Integer = i128;
pub type Moment = u64;

// ----------------------------------------------------------------------------------------------------
//                                                FRAME System
// ----------------------------------------------------------------------------------------------------

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for MockRuntime {
	type Origin = Origin;
	type Index = u128;
	type BlockNumber = BlockNumber;
	type Call = Call;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
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
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

// ----------------------------------------------------------------------------------------------------
//                                                Balances
// ----------------------------------------------------------------------------------------------------

parameter_types! {
	pub const BalanceExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for MockRuntime {
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

// ----------------------------------------------------------------------------------------------------
//                                                 Timestamp
// ----------------------------------------------------------------------------------------------------

parameter_types! {
	pub const MinimumPeriod: u64 = 5;
}

impl pallet_timestamp::Config for MockRuntime {
	type MinimumPeriod = MinimumPeriod;
	type Moment = u64;
	type OnTimestampSet = ();
	type WeightInfo = ();
}

// ----------------------------------------------------------------------------------------------------
//                                             VAMM
// ----------------------------------------------------------------------------------------------------

parameter_types! {
	pub const VammPalletId: PalletId = PalletId(*b"vamm____");
}

impl pallet_vamm::Config for MockRuntime {
	type Balance = Balance;
	type Decimal = FixedU128;
	type Event = Event;
	type Integer = Integer;
	type Moment = Moment;
	type TimeProvider = Timestamp;
	type VammId = VammId;
}

// ----------------------------------------------------------------------------------------------------
//                                             Externalities Builder
// ----------------------------------------------------------------------------------------------------

#[derive(Default)]
pub struct ExtBuilder {
	pub vamm_count: VammId,
	pub vamms: Vec<(
		VammId,
		pallet_vamm::types::VammState<
			<MockRuntime as pallet_vamm::Config>::Balance,
			<MockRuntime as pallet_vamm::Config>::Moment,
			<MockRuntime as pallet_vamm::Config>::Decimal,
		>,
	)>,
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut storage =
			frame_system::GenesisConfig::default().build_storage::<MockRuntime>().unwrap();

		pallet_vamm::GenesisConfig::<MockRuntime> {
			vamm_count: self.vamm_count,
			vamms: self.vamms,
		}
		.assimilate_storage(&mut storage)
		.unwrap();

		storage.into()
	}
}
