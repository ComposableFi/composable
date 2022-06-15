use crate::{
	mock::{ExtBuilder, MockRuntime, TestPallet},
	pallet::Error,
	tests::{
		any_vamm_state, balance_range_lower_half, balance_range_upper_half, get_swap_config,
		get_vamm_state, run_to_block, TestSwapConfig, RUN_CASES,
	},
};
use composable_traits::vamm::{AssetType, Direction, Vamm as VammTrait, MINIMUM_TWAP_PERIOD};
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn swap_base_remove_insufficient_funds_error(
		mut vamm_state in get_vamm_state(Default::default()),
		mut swap_config in get_swap_config(
			TestSwapConfig {
				direction: Some(Direction::Remove),
				vamm_id: Some(0),
				asset: Some(AssetType::Base),
				..Default::default()}),
		input_amount in balance_range_upper_half(),
		base_asset_reserves in balance_range_lower_half(),
	) {
		prop_assume!(input_amount > base_asset_reserves);
		prop_assume!(swap_config.direction == Direction::Remove);

		// Ensure vamm is open before starting operation to swap assets.
		vamm_state.closed = None;

		// Ensure we don't throw `AssetTwapTimestampIsMoreRecent` error.
		vamm_state.twap_timestamp = 0;

		// Ensure we don't throw `FailedToComputeLastTwapWeight` error.
		vamm_state.twap_period = (MINIMUM_TWAP_PERIOD + 1).into();

		swap_config.input_amount = input_amount;
		vamm_state.base_asset_reserves = base_asset_reserves;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			// Ensure we don't throw `AssetTwapTimestampIsMoreRecent` error.
			run_to_block(1);
			assert_noop!(
				TestPallet::swap(&swap_config),
				Error::<MockRuntime>::InsufficientFundsForTrade
			);
		})
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn swap_base_remove_succeeds(
		mut vamm_state in any_vamm_state(),
		mut swap_config in get_swap_config(
			TestSwapConfig {
				direction: Some(Direction::Remove),
				vamm_id: Some(0),
				asset: Some(AssetType::Base),
				..Default::default()}),
	) {
		// Ensure vamm is open before starting operation to swap assets.
		vamm_state.closed = None;

		// Ensure input amount will not cause `InsufficientFundsForTrade`,
		// `Overflow`, `Underflow`, etc.
		swap_config.input_amount = 0;

		// Disable output limit check
		swap_config.output_amount_limit = 0;

		// Ensure we don't throw `AssetTwapTimestampIsMoreRecent` error.
		vamm_state.twap_timestamp = 0;

		// Ensure we don't throw `FailedToComputeLastTwapWeight` error.
		vamm_state.twap_period = (MINIMUM_TWAP_PERIOD + 1).into();

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			// Ensure we don't throw `AssetTwapTimestampIsMoreRecent` error.
			run_to_block(1);
			assert_ok!(TestPallet::swap(&swap_config));
		})
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn swap_base_add_trade_exptrapolates_maximum_supported_amount_error(
		mut vamm_state in get_vamm_state(Default::default()),
		mut swap_config in get_swap_config(
			TestSwapConfig {
				direction: Some(Direction::Add),
				vamm_id: Some(0),
				asset: Some(AssetType::Base),
				..Default::default()}),
		input_amount in balance_range_upper_half(),
		base_asset_reserves in balance_range_upper_half(),
	) {
		prop_assume!(swap_config.direction == Direction::Add);

		// Ensure vamm is open before starting operation to swap assets.
		vamm_state.closed = None;

		// Ensure we don't throw `AssetTwapTimestampIsMoreRecent` error.
		vamm_state.twap_timestamp = 0;

		// Ensure we don't throw `FailedToComputeLastTwapWeight` error.
		vamm_state.twap_period = (MINIMUM_TWAP_PERIOD + 1).into();

		swap_config.input_amount = input_amount;
		vamm_state.base_asset_reserves = base_asset_reserves;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			// Ensure we don't throw `AssetTwapTimestampIsMoreRecent` error.
			run_to_block(1);
			assert_noop!(
				TestPallet::swap(&swap_config),
				Error::<MockRuntime>::TradeExtrapolatesMaximumSupportedAmount
			);
		})
	}
}
