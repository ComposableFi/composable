use crate::{
	mock::{ExtBuilder, MockRuntime, TestPallet, VammId},
	pallet::Error,
	tests::{get_vamm_state, run_for_seconds, RUN_CASES},
};
use composable_traits::vamm::{AssetType, Vamm as VammTrait};
use frame_support::{assert_noop, assert_ok, assert_storage_noop};
use proptest::prelude::*;

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	// ----------------------------------------------------------------------------------------------------
	//                              TWAP - Base Asset
	// ----------------------------------------------------------------------------------------------------
	#[test]
	fn get_twap_base_asset_succeeds(
		vamm_state in get_vamm_state(Default::default())
	) {
		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_ok!(TestPallet::get_twap(&0, AssetType::Base));
			assert_storage_noop!(TestPallet::get_twap(&0, AssetType::Base));
		})
	}

	#[test]
	fn get_twap_base_asset_fails_if_vamm_does_not_exists(
		vamm_state in get_vamm_state(Default::default()),
		vamm_id in 1..=VammId::MAX
	) {
		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_noop!(
				TestPallet::get_twap(&vamm_id, AssetType::Base),
				Error::<MockRuntime>::VammDoesNotExist
			);
		})
	}

	#[test]
	fn get_twap_base_asset_fails_if_vamm_is_closed(
		mut vamm_state in get_vamm_state(Default::default()),
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
				TestPallet::get_twap(&0, AssetType::Base),
				Error::<MockRuntime>::VammIsClosed
			);
		})
	}
}
