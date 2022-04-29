use crate::{
	mock::{ExtBuilder, MockRuntime, TestPallet},
	pallet::{Error, VammMap},
	tests::{
		balance_range_lower_half, balance_range_upper_half, get_swap_config, get_vamm_state,
		TestSwapConfig, RUN_CASES,
	},
};
use composable_traits::vamm::{AssetType, Direction, Vamm as VammTrait};
use frame_support::assert_err;
use proptest::prelude::*;

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn swap_quote_remove_insufficient_funds_error(
		mut vamm_state in get_vamm_state(Default::default()),
		mut swap_config in get_swap_config(
			TestSwapConfig {
				direction: Some(Direction::Remove),
				vamm_id: Some(0),
				asset: Some(AssetType::Quote),
				..Default::default()}),
		input_amount in balance_range_upper_half(),
		quote_asset_reserves in balance_range_lower_half(),
	) {
		prop_assume!(input_amount > quote_asset_reserves);
		prop_assume!(swap_config.direction == Direction::Remove);

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			swap_config.input_amount = input_amount;
			vamm_state.quote_asset_reserves = quote_asset_reserves;
			VammMap::<MockRuntime>::mutate(0, |vamm| {
				*vamm = Some(vamm_state);
			});

			let swap = TestPallet::swap(&swap_config);
			assert_err!(swap, Error::<MockRuntime>::InsufficientFundsForTrade);
		})
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn swap_quote_add_trade_exptrapolates_maximum_supported_amount_error(
		mut vamm_state in get_vamm_state(Default::default()),
		mut swap_config in get_swap_config(
			TestSwapConfig {
				direction: Some(Direction::Add),
				vamm_id: Some(0),
				asset: Some(AssetType::Quote),
				..Default::default()}),
		input_amount in balance_range_upper_half(),
		quote_asset_reserves in balance_range_upper_half(),
	) {
		prop_assume!(swap_config.direction == Direction::Add);

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			swap_config.input_amount = input_amount;
			vamm_state.quote_asset_reserves = quote_asset_reserves;
			VammMap::<MockRuntime>::mutate(0, |vamm| {
				*vamm = Some(vamm_state);
			});

			let swap = TestPallet::swap(&swap_config);
			assert_err!(swap, Error::<MockRuntime>::TradeExtrapolatesMaximumSupportedAmount);
		})
	}
}
