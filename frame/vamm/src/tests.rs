use crate::{
	mock::{Event, ExtBuilder, Origin, System, Vamm, VammId},
	pallet,
	pallet::Error,
};

use composable_traits::vamm::Vamm as VammTrait;

use proptest::prelude::*;

use frame_support::{
	assert_noop, assert_ok, assert_storage_noop, sp_std::collections::btree_map::BTreeMap,
	traits::fungibles::Inspect,
};

struct VammConfigBuilder {
	pub vamm_id: VammId,
}

// #[test]
// fn create_vamm_emits_event() {
// 	ExtBuilder::default().build(1).execute_with(|| {
// 		let config = VaultConfigBuilder::default().build();
// 		assert_ok!(Vamm::create());

// 		System::assert_last_event(Event::Created(pallet::Event::Created { vamm_id: 1u64, state }));
// 	});
// }
