use crate::{
	mock::{Balance, Event, ExtBuilder, MockRuntime, Origin, System, Timestamp, Vamm, VammId},
	pallet,
	pallet::{Error, VammMap},
};

use composable_traits::vamm::{Vamm as VammTrait, VammState};

use proptest::prelude::*;

use frame_support::{
	assert_err, assert_noop, assert_ok, assert_storage_noop,
	pallet_prelude::Hooks,
	sp_std::collections::btree_map::BTreeMap,
	traits::{fungibles::Inspect, UnixTime},
};

use proptest::{
	num::f64::{NEGATIVE, POSITIVE, ZERO},
	prelude::*,
};
use sp_runtime::{
	traits::{One, Zero},
	FixedI128,
};

// ----------------------------------------------------------------------------------------------------
//                                             Setup
// ----------------------------------------------------------------------------------------------------

impl Default for ExtBuilder {
	fn default() -> Self {
		Self { vamm_count: 0u128 }
	}
}

fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 0 {
			Timestamp::on_finalize(System::block_number());
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		// Time is set in milliseconds, so at each block we increment the timestamp by 1000ms = 1s
		let _ = Timestamp::set(Origin::none(), System::block_number() * 1000);
		System::on_initialize(System::block_number());
		Timestamp::on_initialize(System::block_number());
	}
}

// #[test]
// fn create_vamm_emits_event() {
// 	ExtBuilder::default().build(1).execute_with(|| {
// 		let config = VaultConfigBuilder::default().build();
// 		assert_ok!(Vamm::create());

// 		System::assert_last_event(Event::Created(pallet::Event::Created { vamm_id: 1u64, state }));
// 	});
// }
