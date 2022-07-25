use crate::{
	mock::{Balance, Event, ExtBuilder, MockRuntime, System, TestPallet},
	pallet::{self, Error},
	tests::{
		helpers::{
			any_sane_asset_amount, as_decimal, as_decimal_from_fraction, run_for_seconds,
			twap_update_delay,
		},
		helpers_propcompose::any_vamm_state,
		Decimal, Timestamp, RUN_CASES,
	},
	types::VammState,
};
use composable_tests_helpers::test::helper::default_acceptable_computation_error;
use composable_traits::vamm::{Vamm as VammTrait, VammConfig};
use frame_support::{assert_noop, assert_ok, assert_storage_noop};
use proptest::prelude::*;
use sp_runtime::FixedPointNumber;

// ----------------------------------------------------------------------------------------------------
//                                           Prop Compose
// ----------------------------------------------------------------------------------------------------

prop_compose! {
	fn any_new_twap()(
		twap in any_sane_asset_amount(),
	) (
		twap in Just(Decimal::from_inner(twap))
	) -> Decimal {
		twap
	}
}

// -------------------------------------------------------------------------------------------------
//                                           Unit Tests
// -------------------------------------------------------------------------------------------------

#[test]
fn should_succeed_computing_correct_reciprocal_twap() {
	assert_eq!(as_decimal(2).reciprocal().unwrap(), as_decimal_from_fraction(50, 100));
	assert_eq!(as_decimal(50).reciprocal().unwrap(), as_decimal_from_fraction(2, 100));
}

#[test]
fn update_twap_fails_if_vamm_does_not_exist() {
	let vamm_state = VammState::default();
	let base_twap = Some(as_decimal(10));
	ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
		.build()
		.execute_with(|| {
			assert_noop!(TestPallet::update_twap(1, None), Error::<MockRuntime>::VammDoesNotExist);

			assert_noop!(
				TestPallet::update_twap(1, base_twap),
				Error::<MockRuntime>::VammDoesNotExist
			);
		});
}

#[test]
fn update_twap_fails_if_vamm_is_closed() {
	let vamm_state = VammState {
		closed: Some(Timestamp::MIN),
		base_asset_reserves: as_decimal(42).into_inner(),
		quote_asset_reserves: as_decimal(1337).into_inner(),
		base_asset_twap: as_decimal(42),
		..Default::default()
	};
	let base_twap = Some(as_decimal(10));
	ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
		.build()
		.execute_with(|| {
			// For event emission
			run_for_seconds(vamm_state.closed.unwrap() + 1);

			assert_noop!(TestPallet::update_twap(0, base_twap), Error::<MockRuntime>::VammIsClosed);

			assert_noop!(TestPallet::update_twap(0, None), Error::<MockRuntime>::VammIsClosed);
		});
}

#[test]
fn update_twap_fails_if_new_twap_is_zero() {
	let vamm_state = VammState::default();
	let base_twap = Some(as_decimal(0));
	ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
		.build()
		.execute_with(|| {
			run_for_seconds(1);
			assert_storage_noop!(TestPallet::update_twap(0, base_twap));
		});
}

#[test]
fn update_twap_fails_if_twap_timestamp_is_more_recent() {
	let timestamp = Timestamp::MIN;
	let timestamp_greater = Timestamp::MIN + 1;
	let vamm_state = VammState {
		twap_timestamp: timestamp_greater,
		base_asset_reserves: as_decimal(42).into_inner(),
		quote_asset_reserves: as_decimal(1337).into_inner(),
		base_asset_twap: as_decimal(42),
		..Default::default()
	};
	let new_twap = Some(as_decimal(10));
	ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
		.build()
		.execute_with(|| {
			// For event emission
			run_for_seconds(timestamp);
			assert_noop!(
				TestPallet::update_twap(0, new_twap),
				Error::<MockRuntime>::AssetTwapTimestampIsMoreRecent
			);
			assert_noop!(
				TestPallet::update_twap(0, None),
				Error::<MockRuntime>::AssetTwapTimestampIsMoreRecent
			);
		});
}

#[test]
fn should_succeed_updating_twap_correctly() {
	let timestamp = Timestamp::MIN;
	let twap = as_decimal(1).into_inner();
	let new_twap = Some(as_decimal(5));
	let vamm_state = VammState::<Balance, Timestamp, Decimal> {
		twap_timestamp: timestamp,
		base_asset_twap: twap.into(),
		base_asset_reserves: twap,
		quote_asset_reserves: twap,
		twap_period: 3600,
		peg_multiplier: 1,
		..Default::default()
	};
	ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
		.build()
		.execute_with(|| {
			run_for_seconds(twap_update_delay(0));
			assert_ok!(TestPallet::update_twap(0, new_twap), new_twap.unwrap());
			assert_eq!(TestPallet::get_vamm(0).unwrap().base_asset_twap, new_twap.unwrap());

			run_for_seconds(twap_update_delay(0));
			assert_ok!(TestPallet::update_twap(0, None));
			assert_ne!(TestPallet::get_vamm(0).unwrap().base_asset_twap, new_twap.unwrap());
		});
}

#[test]
fn should_update_twap_correctly() {
	ExtBuilder::default().build().execute_with(|| {
		let vamm_creation = TestPallet::create(&VammConfig {
			base_asset_reserves: as_decimal(2).into_inner(),
			quote_asset_reserves: as_decimal(50).into_inner(),
			peg_multiplier: 1,
			twap_period: 3600,
		});
		let vamm_id = vamm_creation.unwrap();
		let original_base_twap = TestPallet::get_vamm(vamm_id).unwrap().base_asset_twap;
		assert_ok!(vamm_creation);

		// For event emission & twap update
		run_for_seconds(twap_update_delay(vamm_id));
		let new_base_twap = Some(as_decimal(100));
		assert_ok!(TestPallet::update_twap(vamm_id, new_base_twap));
		let vamm_state = TestPallet::get_vamm(vamm_id).unwrap();
		assert_eq!(vamm_state.base_asset_twap, new_base_twap.unwrap());
		System::assert_last_event(Event::TestPallet(pallet::Event::UpdatedTwap {
			vamm_id,
			base_twap: new_base_twap.unwrap(),
		}));

		// Run for long enough in order to approximate to the original twap value.
		run_for_seconds(twap_update_delay(vamm_id).saturating_pow(2));
		assert_ok!(TestPallet::update_twap(vamm_id, None));
		let vamm_state = TestPallet::get_vamm(vamm_id).unwrap();
		assert_ok!(default_acceptable_computation_error(
			vamm_state.base_asset_twap.into_inner(),
			original_base_twap.into_inner(),
		));
		System::assert_last_event(Event::TestPallet(pallet::Event::UpdatedTwap {
			vamm_id,
			base_twap: vamm_state.base_asset_twap,
		}));
	});
}

// -------------------------------------------------------------------------------------------------
//                                           Proptests
// -------------------------------------------------------------------------------------------------

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn update_twap_proptest_succeeds(
		vamm_state in any_vamm_state(),
		base_twap in any_new_twap()
	) {
		let now = vamm_state.twap_timestamp
							.min(Timestamp::MAX/1000)
							.saturating_add(1);
		let vamm_state = VammState {
			closed: None,
			twap_timestamp: vamm_state.twap_timestamp
										.min(now)
										.saturating_add(1),
			twap_period: vamm_state.twap_timestamp
										.saturating_add(1),
			..vamm_state
		};

		ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
			.build()
			.execute_with(|| {
				run_for_seconds(twap_update_delay(0));
				assert_ok!(
					TestPallet::update_twap(0, Some(base_twap)),
					base_twap
				);

				run_for_seconds(twap_update_delay(0));
				assert_ok!(
					TestPallet::update_twap(0, None)
				);
			});
	}
}
