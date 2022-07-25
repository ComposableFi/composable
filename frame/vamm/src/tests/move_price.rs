use crate::{
	mock::{ExtBuilder, MockRuntime, System, TestPallet},
	pallet::{Error, Event, VammMap},
	tests::{
		helpers::run_for_seconds,
		helpers_propcompose::{any_move_price_config, any_vamm_state},
		Timestamp, RUN_CASES,
	},
};
use composable_traits::vamm::Vamm as VammTrait;
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;

// -------------------------------------------------------------------------------------------------
//                                             Proptests
// -------------------------------------------------------------------------------------------------

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn should_succeed_moving_price(
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
		});
	}

	#[test]
	fn should_fail_if_vamm_does_not_exist(
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
		});
	}

	#[test]
	fn should_fail_if_vamm_is_closed(
		mut vamm_state in any_vamm_state(),
		mut move_price_config in any_move_price_config(),
		random_seconds in 1..=1_000_000_000_u64,
	) {
		// We must ensure all move price configs operate only on the existent
		// vamm.
		move_price_config.vamm_id = 0;

		// Ensure vamm is closed before starting operation.
		vamm_state.closed = Some(Timestamp::MIN);

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
		});
	}

	#[test]
	fn should_fail_if_base_asset_reserve_is_zero(
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
		});
	}

	#[test]
	fn should_fail_if_quote_asset_reserve_is_zero(
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
		});
	}

	#[test]
	fn should_succeed_updating_runtime_correctly(
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
			let vamm_state = VammMap::<MockRuntime>::get(0).unwrap();
			assert_eq!(vamm_state.base_asset_reserves, move_price_config.base_asset_reserves);
			assert_eq!(vamm_state.quote_asset_reserves, move_price_config.quote_asset_reserves);
			assert_eq!(vamm_state.invariant, TestPallet::compute_invariant(
				move_price_config.base_asset_reserves,
				move_price_config.quote_asset_reserves).unwrap()
			);
		});
	}

	#[test]
	fn should_succeed_depositing_event_correctly(
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
			run_for_seconds(1);

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
			);
		});
	}

	#[test]
	fn should_succeed_returning_correct_invariant(
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
		});
	}
}
