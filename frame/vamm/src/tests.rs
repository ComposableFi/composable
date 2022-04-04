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

}

// #[test]
// fn create_vamm_emits_event() {
// 	ExtBuilder::default().build(1).execute_with(|| {
// 		let config = VaultConfigBuilder::default().build();
// 		assert_ok!(Vamm::create());

// 		System::assert_last_event(Event::Created(pallet::Event::Created { vamm_id: 1u64, state }));
// 	});
// }
