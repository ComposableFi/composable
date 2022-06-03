use crate::{
	mock::{ExtBuilder, MockRuntime, TestPallet, VammId},
	pallet::{self, Error},
	tests::{get_vamm_state, run_for_seconds, RUN_CASES},
	VammState,
};
use composable_traits::vamm::{AssetType, Vamm as VammTrait};
use frame_support::{assert_noop, assert_ok, assert_storage_noop};
use proptest::prelude::*;

// ----------------------------------------------------------------------------------------------------
//                                           Setup
// ----------------------------------------------------------------------------------------------------

type VammTimestamp = <MockRuntime as pallet::Config>::Moment;
type VammDecimal = <MockRuntime as pallet::Config>::Decimal;
type VammBalance = <MockRuntime as pallet::Config>::Balance;

// ----------------------------------------------------------------------------------------------------
//                                           Prop Compose
// ----------------------------------------------------------------------------------------------------

// -------------------------------------------------------------------------------------------------
//                                           Unit Tests
// -------------------------------------------------------------------------------------------------
#[test]
fn update_twap_fails_if_vamm_does_not_exist() {
	let vamm_state = VammState::default();
	let new_twap = Some(VammDecimal::from_inner(10));
	ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
		.build()
		.execute_with(|| {
			assert_noop!(
				TestPallet::update_twap(1, AssetType::Base, None),
				Error::<MockRuntime>::VammDoesNotExist
			);

			assert_noop!(
				TestPallet::update_twap(1, AssetType::Base, new_twap),
				Error::<MockRuntime>::VammDoesNotExist
			);
		})
}

#[test]
fn update_twap_fails_if_vamm_is_closed() {
	let vamm_state = VammState { closed: Some(VammTimestamp::MIN), ..Default::default() };
	let new_twap = Some(VammDecimal::from_inner(10));
	ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
		.build()
		.execute_with(|| {
			// For event emission
			run_for_seconds(vamm_state.closed.unwrap() + 1);

			assert_noop!(
				TestPallet::update_twap(0, AssetType::Base, None),
				Error::<MockRuntime>::VammIsClosed
			);

			assert_noop!(
				TestPallet::update_twap(0, AssetType::Base, new_twap),
				Error::<MockRuntime>::VammIsClosed
			);
		})
}

#[test]
fn update_twap_fails_if_new_twap_is_zero() {
	let vamm_state = VammState::default();
	let new_twap = Some(VammDecimal::from_inner(0));
	ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
		.build()
		.execute_with(|| {
			assert_noop!(
				TestPallet::update_twap(0, AssetType::Base, new_twap),
				Error::<MockRuntime>::NewTwapValueIsZero
			);
		})
}

#[test]
fn update_twap_fails_if_twap_timestamp_is_more_recent() {
	let timestamp = VammTimestamp::MIN;
	let timestamp_greater = VammTimestamp::MIN + 1;
	let vamm_state = VammState {
		base_asset_twap_timestamp: timestamp_greater,
		quote_asset_twap_timestamp: timestamp_greater,
		..Default::default()
	};
	let new_twap = Some(VammDecimal::from_inner(10));
	ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
		.build()
		.execute_with(|| {
			// For event emission
			run_for_seconds(timestamp);

			assert_noop!(
				TestPallet::update_twap(0, AssetType::Base, new_twap),
				Error::<MockRuntime>::AssetTwapTimestampIsMoreRecent
			);

			assert_noop!(
				TestPallet::update_twap(0, AssetType::Quote, new_twap),
				Error::<MockRuntime>::AssetTwapTimestampIsMoreRecent
			);

			assert_noop!(
				TestPallet::update_twap(0, AssetType::Base, None),
				Error::<MockRuntime>::AssetTwapTimestampIsMoreRecent
			);

			assert_noop!(
				TestPallet::update_twap(0, AssetType::Quote, None),
				Error::<MockRuntime>::AssetTwapTimestampIsMoreRecent
			);
		})
}

#[test]
fn update_twap_succeeds() {
	let timestamp = VammTimestamp::MIN;
	let mut timestamp_greater = VammTimestamp::MIN + 1;
	let twap = VammBalance::from(10_u128.pow(18));
	let new_twap = Some(VammDecimal::from_inner(10_u128.pow(18) * 5));
	let vamm_state = VammState {
		base_asset_twap_timestamp: timestamp,
		quote_asset_twap_timestamp: timestamp,
		base_asset_twap: twap,
		quote_asset_twap: twap,
		base_asset_reserves: twap,
		quote_asset_reserves: twap,
		funding_period: 3600,
		..Default::default()
	};
	ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
		.build()
		.execute_with(|| {
			// For event emission
			run_for_seconds(timestamp_greater);
			assert_ok!(TestPallet::update_twap(0, AssetType::Base, new_twap));

			timestamp_greater += 1;
			run_for_seconds(timestamp_greater);
			assert_ok!(TestPallet::update_twap(0, AssetType::Quote, new_twap));

			timestamp_greater += 1;
			run_for_seconds(timestamp_greater);
			assert_ok!(TestPallet::update_twap(0, AssetType::Base, None));

			timestamp_greater += 1;
			run_for_seconds(timestamp_greater);
			assert_ok!(TestPallet::update_twap(0, AssetType::Quote, None));
		})
}

#[test]
fn update_twap_updates_twaps_correctly() {
	let timestamp = VammTimestamp::MIN;
	let mut timestamp_greater = VammTimestamp::MIN + 1;
	let twap = VammBalance::from(10_u128.pow(18));
	let new_twap = Some(VammDecimal::from_inner(10_u128.pow(18) * 5));
	let vamm_state = VammState {
		base_asset_twap_timestamp: timestamp,
		quote_asset_twap_timestamp: timestamp,
		base_asset_twap: twap,
		quote_asset_twap: twap,
		base_asset_reserves: twap,
		quote_asset_reserves: twap,
		funding_period: 3600,
		..Default::default()
	};
	ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
		.build()
		.execute_with(|| {
			// For event emission
			run_for_seconds(timestamp_greater);
			assert_ok!(TestPallet::update_twap(0, AssetType::Base, new_twap));
			assert_eq!(
				VammDecimal::from_inner(TestPallet::get_vamm(0).unwrap().base_asset_twap),
				new_twap.unwrap()
			);

			timestamp_greater += 1;
			run_for_seconds(timestamp_greater);
			assert_ok!(TestPallet::update_twap(0, AssetType::Quote, new_twap));
			assert_eq!(
				VammDecimal::from_inner(TestPallet::get_vamm(0).unwrap().quote_asset_twap),
				new_twap.unwrap()
			);

			timestamp_greater += 1;
			run_for_seconds(timestamp_greater);
			assert_ok!(TestPallet::update_twap(0, AssetType::Base, None));
			assert_eq!(
				VammDecimal::from_inner(TestPallet::get_vamm(0).unwrap().base_asset_twap),
				VammDecimal::from_inner(4993055555555555555)
			);

			timestamp_greater += 1;
			run_for_seconds(timestamp_greater);
			assert_ok!(TestPallet::update_twap(0, AssetType::Quote, None));
			assert_eq!(
				VammDecimal::from_inner(TestPallet::get_vamm(0).unwrap().quote_asset_twap),
				VammDecimal::from_inner(4990277777777777777)
			);
		})
}

// TODO(Cardosaum): Check Event emission.

// -------------------------------------------------------------------------------------------------
//                                           Proptests
// -------------------------------------------------------------------------------------------------
