use crate::{
	mock::{ExtBuilder, MockRuntime, TestPallet},
	pallet::{Error, VammMap},
	tests::{get_swap_config, get_vamm_state, run_to_block, RUN_CASES},
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
		x1 in u64::MIN..=1000,
		x2 in u64::MIN..=1000,
	) {
		// The time now and expected time for the vamm to close must be
		// different, if they are equal the trade is still allowed to occur.
		prop_assume!(x1 != x2);

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			// Make the current time be greater than the time when the vamm is
			// set to close, doing this we ensure we can't make swaps due to the
			// vamm be closed.
			if x1 < x2 {
				run_to_block(x1);
				vamm_state.closed = Some(x2);
			} else {
				run_to_block(x2);
				vamm_state.closed = Some(x1);
			}
			swap_config.vamm_id = 0;
			VammMap::<MockRuntime>::insert(0, vamm_state);
			let swap = TestPallet::swap(&swap_config);
			assert_err!(swap, Error::<MockRuntime>::VammIsClosed);
		})
	}
}
