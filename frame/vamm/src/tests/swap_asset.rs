use crate::{
	mock::{ExtBuilder, MockRuntime, TestPallet},
	pallet::Error,
	tests::{
		balance_range_lower_half, balance_range_upper_half, get_swap_config, get_vamm_state,
		run_to_block, then_and_now, RUN_CASES,
	},
};
use composable_traits::vamm::Vamm as VammTrait;
use frame_support::assert_noop;
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
			assert_noop!(
				TestPallet::swap(&swap_config),
				Error::<MockRuntime>::VammDoesNotExist
			);
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
		// Make the current time be greater than the time when the vamm is
		// set to close, doing this we ensure we can't make swaps due to the
		// vamm be closed.
		vamm_state.closed = Some(close);
		swap_config.vamm_id = 0;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			run_to_block(now);

			assert_noop!(
				TestPallet::swap(&swap_config),
				Error::<MockRuntime>::VammIsClosed
			);
		})
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn fails_to_swap_assets_if_output_is_less_than_minimum_limit(
		mut vamm_state in get_vamm_state(Default::default()),
		mut swap_config in get_swap_config(Default::default()),
		base in balance_range_lower_half(),
		quote in balance_range_lower_half(),
		peg in balance_range_lower_half(),
		limit in balance_range_upper_half(),
	) {
		// Ensure vamm is open before start operation to swap assets.
		vamm_state.closed = None;

		// Ensure values won't overflow when computing swap.
		vamm_state.base_asset_reserves = base;
		vamm_state.quote_asset_reserves = quote;
		vamm_state.peg_multiplier = peg;

		// Ensure input amount will not cause `InsufficientFundsForTrade`,
		// `Overflow`, `Underflow`, etc.
		swap_config.input_amount = 0;

		swap_config.output_amount_limit = limit;
		swap_config.vamm_id = 0;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_noop!(
				TestPallet::swap(&swap_config),
				Error::<MockRuntime>::SwappedAmountLessThanMinimumLimit
			);
		})
	}
}
