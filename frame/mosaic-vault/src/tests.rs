
use crate as mosaic_vault;
use crate::{
    mocks::{
        currency_factory::MockCurrencyId
    }
};

use frame_support::{
    construct_runtime,
    parameter_types,
    traits::{Everything}
};
use orml_traits::parameter_type_with_key;
use num_traits::Zero;
use sp_core::H256;
use frame_system as system;
use sp_runtime::{
    testing::Header,
    traits::{ConvertInto, IdentityLookup},
};

pub type BlockNumber = u64;
pub type Balance = u128;
pub type Amount = i128;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

parameter_types!{
    pub const BlockHashCount: u64 = 250;
}

impl system::Config for Test {
    type AccountId = u64;
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = BlockNumber;
    type Call = Call;
    type Hash = H256;
    type Hashing = sp_runtime::traits::BlakeTwo256;
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
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: MockCurrencyId| -> Balance {
		Zero::zero()
	};
}


impl orml_tokens::Config for Test {
    type Event = Event;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = MockCurrencyId;
    type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = ();
	type DustRemovalWhitelist = Everything;
}

// impl mosaic_vault::Config for Test {
//     type Event = Event;
//     // type Currency 
// }

// Configure a mock runtime to test the pallet.
construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		// System: frame_system::{Module, Call, Config, Storage, Event<T>},
		// TemplateModule: pallet_template::{Module, Call, Storage, Event<T>},
        Tokens: orml_tokens::{Pallet, Storage, Event<T>, Config<T>},
        System: system::{ Pallet, Call, Storage, Config, Event<T>},
       // MosaicVault: mosaic_vault::{ Pallet, Call, Storage, Event<T> }
	}
);

// #[test]
// fn error_works(){
//     new_test_ext().execute_with(|| {
//         assert_err!(
//             TestingPallet::add_value(Origin::signed(1), 51),
//             "value must be <= maximum add amount constant"
//         );
//     })
//