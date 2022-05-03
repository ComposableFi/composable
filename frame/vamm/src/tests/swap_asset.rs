use crate::{
	mock::{ExtBuilder, MockRuntime, TestPallet},
	pallet::{Error, VammMap},
	tests::{get_swap_config, get_vamm_state, run_to_block, then_and_now, RUN_CASES},
};
use composable_traits::vamm::Vamm as VammTrait;
use frame_support::assert_err;
use proptest::prelude::*;

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn swap_invalid_vamm_error(
		vamm_state in get_vamm_state(Default::default()),
		swap_config in get_swap_config(Default::default()),
	) {
		prop_assume!(swap_config.vamm_id != 0);

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			let swap = TestPallet::swap(&swap_config);
			assert_err!(swap, Error::<MockRuntime>::VammDoesNotExist);
		})
	}
}

proptest! {
	#[test]
	fn fails_to_swap_assets_if_vamm_is_closed(
		mut vamm_state in get_vamm_state(Default::default()),
		mut swap_config in get_swap_config(Default::default()),
		(close, now) in then_and_now()
	) {
		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			// Make the current time be greater than the time when the vamm is
			// set to close, doing this we ensure we can't make swaps due to the
			// vamm be closed.
			vamm_state.closed = Some(close);
			run_to_block(now);
			swap_config.vamm_id = 0;
			VammMap::<MockRuntime>::insert(0, vamm_state);
			let swap = TestPallet::swap(&swap_config);
			assert_err!(swap, Error::<MockRuntime>::VammIsClosed);
		})
	}
}
