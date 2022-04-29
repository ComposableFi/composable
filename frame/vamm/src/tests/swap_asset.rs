use crate::{
	mock::{ExtBuilder, MockRuntime, TestPallet},
	pallet::Error,
	tests::{get_swap_config, get_vamm_state, RUN_CASES},
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
