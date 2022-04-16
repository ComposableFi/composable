use crate as pallet_instrumental_strategy;

use frame_support::{
	parameter_types,
	traits::Everything,
};

use sp_runtime::{
	testing::Header,
	traits::IdentityLookup
};
use sp_core::H256;

pub type AccountId = u128;
pub type BlockNumber = u64;
pub type Balance = u128;
pub type CurrencyId = u128;

// -----------------------------------------------------------------------------------------------
//                                             Config                                             
// -----------------------------------------------------------------------------------------------

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for MockRuntime {
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

// -----------------------------------------------------------------------------------------------
//                                             Balances                                           
// -----------------------------------------------------------------------------------------------

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

// -----------------------------------------------------------------------------------------------
//                                      Instrumental Strategy                                    
// -----------------------------------------------------------------------------------------------

parameter_types! {
	pub const MaxStrategies: u32 = 10;
}

impl pallet_instrumental_strategy::Config for MockRuntime {
	type Event = Event;
	type WeightInfo = ();
	type AssetId = CurrencyId;
	type MaxStrategies = MaxStrategies;
}

// -----------------------------------------------------------------------------------------------
//                                        Construct Runtime                                      
// -----------------------------------------------------------------------------------------------

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
		InstrumentalStrategy: pallet_instrumental_strategy::{Pallet, Call, Storage, Event<T>},
	}
);

// -----------------------------------------------------------------------------------------------
//                                      Externalities Builder                                     
// -----------------------------------------------------------------------------------------------

#[derive(Default)]
pub struct ExtBuilder {
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let t = frame_system::GenesisConfig::default().build_storage::<MockRuntime>().unwrap();

		t.into()
	}
}