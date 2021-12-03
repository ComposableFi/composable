#![cfg(test)]

use crate::{self as composable_xcm};
use frame_support::{construct_runtime, traits::Everything};
use sp_runtime::{
	testing::{Header, H256},
	traits::IdentityLookup,
};

type AccountId = u128;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
	pub enum Runtime
	where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		ComposableXcm: composable_xcm::{Pallet},
		System: frame_system::{Call, Config, Event<T>, Pallet, Storage},
	}
);

impl composable_xcm::Config for Runtime {
	type XcmExecutor = ();
}

impl frame_system::Config for Runtime {
	type AccountData = ();
	type AccountId = AccountId;
	type BaseCallFilter = Everything;
	type BlockHashCount = ();
	type BlockLength = ();
	type BlockNumber = u64;
	type BlockWeights = ();
	type Call = Call;
	type DbWeight = ();
	type Event = ();
	type Hash = H256;
	type Hashing = sp_runtime::traits::BlakeTwo256;
	type Header = Header;
	type Index = u64;
	type Lookup = IdentityLookup<AccountId>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type Origin = Origin;
	type PalletInfo = PalletInfo;
	type SS58Prefix = ();
	type SystemWeightInfo = ();
	type Version = ();
}

#[derive(Default)]
pub struct ExtBuilder;

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
		t.into()
	}
}
