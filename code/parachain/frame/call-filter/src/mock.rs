#![cfg(test)]

use super::*;
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup};
use support::{construct_runtime, ord_parameter_types, parameter_types, traits::Everything};
use system::EnsureSignedBy;

pub type AccountId = u128;
pub type Balance = u128;
pub const ALICE: AccountId = 1;

mod call_filter {
	pub use super::super::*;
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl system::Config for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type Index = u64;
	type BlockNumber = u64;
	type RuntimeCall = RuntimeCall;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<AccountId>;
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
	type MaxConsumers = ConstU32<16>;
}

parameter_types! {
	pub const NativeTokenExistentialDeposit: Balance = 10;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = NativeTokenExistentialDeposit;
	type AccountStore = System;
	type MaxLocks = ();
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = ();
	type WeightInfo = ();
}

ord_parameter_types! {
	pub const One: AccountId = 1;
}

#[derive(PartialEq, Eq, Copy, Clone, Encode, Decode, MaxEncodedLen, TypeInfo, core::fmt::Debug)]
pub struct MaxStringSize;
impl Get<u32> for MaxStringSize {
	fn get() -> u32 {
		100
	}
}

impl Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type EnableOrigin = EnsureSignedBy<One, AccountId>;
	type DisableOrigin = EnsureSignedBy<One, AccountId>;
	type Hook = ();
	type WeightInfo = ();
	type MaxStringSize = MaxStringSize;
}

type UncheckedExtrinsic = system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = system::mocking::MockBlock<Runtime>;

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: system::{Pallet, Call, Config, Storage, Event<T>},
		Filter: call_filter::{Pallet, Storage, Call, Event<T>},
		Balances: pallet_balances::{Pallet, Storage, Call, Event<T>},
	}
);

pub struct ExtBuilder;

impl Default for ExtBuilder {
	fn default() -> Self {
		ExtBuilder
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let t = system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
		t.into()
	}
}
