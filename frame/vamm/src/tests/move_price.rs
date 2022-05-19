use crate::{
	mock::{ExtBuilder, MockRuntime, System, TestPallet},
	pallet::{Error, Event, VammMap},
	tests::{any_move_price_config, any_vamm_state, run_for_seconds, VammTimestamp, RUN_CASES},
	VammState,
};
use composable_traits::vamm::Vamm as VammTrait;
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn move_price_succeeds(
		mut vamm_state in any_vamm_state(),
		mut move_price_config in any_move_price_config(),
	) {
		// We must ensure all move price configs operate only on the existent
		// vamm.
		move_price_config.vamm_id = 0;

		// Ensure vamm is open before starting operation.
		vamm_state.closed = None;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_ok!(TestPallet::move_price(&move_price_config));
		})
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn move_price_fails_if_vamm_does_not_exists(
		mut vamm_state in any_vamm_state(),
		move_price_config in any_move_price_config(),
	) {
		// We must ensure all move price configs operate only on non existent
		// vamms.
		prop_assume!(move_price_config.vamm_id != 0);

		// Ensure vamm is open before starting operation.
		vamm_state.closed = None;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_noop!(
				TestPallet::move_price(&move_price_config),
				Error::<MockRuntime>::VammDoesNotExist
			);
		})
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn move_price_fails_if_vamm_is_closed(
		mut vamm_state in any_vamm_state(),
		mut move_price_config in any_move_price_config(),
		random_seconds in 1..=1_000_000_000_u64,
	) {
		// We must ensure all move price configs operate only on the existent
		// vamm.
		move_price_config.vamm_id = 0;

		// Ensure vamm is closed before starting operation.
		vamm_state.closed = Some(VammTimestamp::MIN);

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			// for closed assertion takes place.
			run_for_seconds(random_seconds);

			assert_noop!(
				TestPallet::move_price(&move_price_config),
				Error::<MockRuntime>::VammIsClosed
			);
		})
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn move_price_fails_if_base_asset_reserve_is_zero(
		mut vamm_state in any_vamm_state(),
		mut move_price_config in any_move_price_config(),
	) {
		// We must ensure all move price configs operate only on the existent
		// vamm.
		move_price_config.vamm_id = 0;

		// Ensure vamm is open before starting operation.
		vamm_state.closed = None;

		// Ensure base asset reserve is zero
		move_price_config.base_asset_reserves = 0;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_noop!(
				TestPallet::move_price(&move_price_config),
				Error::<MockRuntime>::BaseAssetReserveIsZero
			);
		})
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn move_price_fails_if_quote_asset_reserve_is_zero(
		mut vamm_state in any_vamm_state(),
		mut move_price_config in any_move_price_config(),
	) {
		// We must ensure all move price configs operate only on the existent
		// vamm.
		move_price_config.vamm_id = 0;

		// Ensure vamm is open before starting operation.
		vamm_state.closed = None;

		// Ensure base asset reserve is zero
		move_price_config.quote_asset_reserves = 0;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_noop!(
				TestPallet::move_price(&move_price_config),
				Error::<MockRuntime>::QuoteAssetReserveIsZero
			);
		})
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn move_price_updates_runtime_correctly(
		mut vamm_state in any_vamm_state(),
		mut move_price_config in any_move_price_config(),
	) {
		// We must ensure all move price configs operate only on the existent
		// vamm.
		move_price_config.vamm_id = 0;

		// Ensure vamm is open before starting operation.
		vamm_state.closed = None;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_ok!(TestPallet::move_price(&move_price_config));
			assert_eq!(
				VammMap::<MockRuntime>::get(0).unwrap(),
				VammState {
					base_asset_reserves: move_price_config.base_asset_reserves,
					quote_asset_reserves: move_price_config.quote_asset_reserves,
					peg_multiplier: vamm_state.peg_multiplier,
					invariant: TestPallet::compute_invariant(
						move_price_config.base_asset_reserves,
						move_price_config.quote_asset_reserves
					).unwrap(),
					closed: vamm_state.closed,
				}
			);
		})
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn move_price_deposits_event_correctly(
		mut vamm_state in any_vamm_state(),
		mut move_price_config in any_move_price_config(),
	) {
		// We must ensure all move price configs operate only on the existent
		// vamm.
		move_price_config.vamm_id = 0;

		// Ensure vamm is open before starting operation.
		vamm_state.closed = None;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			// for event emission
			run_for_seconds(10);

			assert_ok!(TestPallet::move_price(&move_price_config));

			System::assert_last_event(
				Event::PriceMoved {
					vamm_id: move_price_config.vamm_id,
					base_asset_reserves: move_price_config.base_asset_reserves,
					quote_asset_reserves: move_price_config.quote_asset_reserves,
					invariant: TestPallet::compute_invariant(
						move_price_config.base_asset_reserves,
						move_price_config.quote_asset_reserves
					).unwrap(),
				}.into()
			)
		})
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn move_price_returned_correct_invariant(
		mut vamm_state in any_vamm_state(),
		mut move_price_config in any_move_price_config(),
	) {
		// We must ensure all move price configs operate only on the existent
		// vamm.
		move_price_config.vamm_id = 0;

		// Ensure vamm is open before starting operation.
		vamm_state.closed = None;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_ok!(
				TestPallet::move_price(&move_price_config),
				TestPallet::compute_invariant(
					move_price_config.base_asset_reserves,
					move_price_config.quote_asset_reserves
				).unwrap()
			);
		})
	}
}
