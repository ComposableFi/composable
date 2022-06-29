use crate::{
	mock::{ExtBuilder, MockRuntime, TestPallet, VammId},
	pallet::Error,
	tests::{
		helpers::{as_decimal, run_for_seconds},
		helpers_propcompose::any_vamm_state,
		Timestamp, VammState, RUN_CASES,
	},
};
use composable_traits::vamm::{AssetType, Vamm as VammTrait};
use frame_support::{assert_noop, assert_ok, assert_storage_noop};
use proptest::prelude::*;
use sp_runtime::FixedPointNumber;

// -------------------------------------------------------------------------------------------------
//                                            Unit Tests
// -------------------------------------------------------------------------------------------------

#[test]
fn should_fail_if_vamm_does_not_exist() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			TestPallet::get_twap(0, AssetType::Base),
			Error::<MockRuntime>::VammDoesNotExist
		);
		assert_noop!(
			TestPallet::get_twap(0, AssetType::Quote),
			Error::<MockRuntime>::VammDoesNotExist
		);
	})
}

#[test]
fn should_fail_if_vamm_is_closed() {
	let vamm_state = VammState {
		base_asset_reserves: as_decimal(4).into_inner(),
		quote_asset_reserves: as_decimal(8).into_inner(),
		peg_multiplier: 1,
		closed: Some(Timestamp::MIN),
		..Default::default()
	};
	ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
		.build()
		.execute_with(|| {
			// for closed assertion takes place.
			run_for_seconds(1);
			assert_noop!(
				TestPallet::get_twap(0, AssetType::Base),
				Error::<MockRuntime>::VammIsClosed
			);
			assert_noop!(
				TestPallet::get_twap(0, AssetType::Quote),
				Error::<MockRuntime>::VammIsClosed
			);
		})
}

// -------------------------------------------------------------------------------------------------
//                                             Proptests
// -------------------------------------------------------------------------------------------------

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	// ----------------------------------------------------------------------------------------------------
	//                                          TWAP - Base Asset
	// ----------------------------------------------------------------------------------------------------
	#[test]
	fn should_succeed_not_modifying_storage_base(
		vamm_state in any_vamm_state()
	) {
		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_ok!(TestPallet::get_twap(0, AssetType::Base));
			assert_storage_noop!(TestPallet::get_twap(0, AssetType::Base));
		})
	}

	#[test]
	fn should_fail_if_vamm_does_not_exist_base(
		vamm_state in any_vamm_state(),
		vamm_id in 1..=VammId::MAX
	) {
		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_noop!(
				TestPallet::get_twap(vamm_id, AssetType::Base),
				Error::<MockRuntime>::VammDoesNotExist
			);
		})
	}

	#[test]
	fn should_fail_if_vamm_is_closed_base(
		mut vamm_state in any_vamm_state(),
		closed in (0..18446744073709551_u64).prop_map(Some),
	) {
		// Make sure the market will be closed.
		vamm_state.closed = closed;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			// In order to check if vamm is closed or not we must simulate time
			// passing.
			run_for_seconds(
				match vamm_state.closed {Some(t) => t+1, _ => 1}
			);

			assert_noop!(
				TestPallet::get_twap(0, AssetType::Base),
				Error::<MockRuntime>::VammIsClosed
			);
		})
	}

	// ----------------------------------------------------------------------------------------------------
	//                                          TWAP - Quote Asset
	// ----------------------------------------------------------------------------------------------------
	#[test]
	fn should_succeed_not_modifying_storage_quote(
		vamm_state in any_vamm_state()
	) {
		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_ok!(TestPallet::get_twap(0, AssetType::Quote));
			assert_storage_noop!(TestPallet::get_twap(0, AssetType::Quote));
		})
	}

	#[test]
	fn should_fail_if_vamm_does_not_exist_quote(
		vamm_state in any_vamm_state(),
		vamm_id in 1..=VammId::MAX
	) {
		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_noop!(
				TestPallet::get_twap(vamm_id, AssetType::Quote),
				Error::<MockRuntime>::VammDoesNotExist
			);
		})
	}

	#[test]
	fn should_fail_if_vamm_is_closed_quote(
		mut vamm_state in any_vamm_state(),
		closed in (0..18446744073709551_u64).prop_map(Some),
	) {
		// Make sure the market will be closed.
		vamm_state.closed = closed;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			// In order to check if vamm is closed or not we must simulate time
			// passing.
			run_for_seconds(
				match vamm_state.closed {Some(t) => t+1, _ => 1}
			);

			assert_noop!(
				TestPallet::get_twap(0, AssetType::Quote),
				Error::<MockRuntime>::VammIsClosed
			);
		})
	}
}
