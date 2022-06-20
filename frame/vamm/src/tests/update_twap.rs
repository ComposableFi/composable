use crate::{
	mock::{Balance, Event, ExtBuilder, MockRuntime, System, TestPallet},
	pallet::{self, Error},
	tests::{
		any_sane_asset_amount, any_vamm_state, run_for_seconds, run_to_block, Decimal, Timestamp,
		RUN_CASES,
	},
	VammState,
};
use composable_tests_helpers::test::helper::default_acceptable_computation_error;
use composable_traits::vamm::{Vamm as VammTrait, VammConfig};
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;
use sp_runtime::FixedPointNumber;

// ----------------------------------------------------------------------------------------------------
//                                           Prop Compose
// ----------------------------------------------------------------------------------------------------

prop_compose! {
	fn any_new_twap()(
		base_twap in any_sane_asset_amount(),
		quote_twap in any_sane_asset_amount()
	) (
		base_twap in prop_oneof![Just(None), Just(Some(Decimal::from_inner(base_twap)))],
		quote_twap in prop_oneof![Just(None), Just(Some(Decimal::from_inner(quote_twap)))]
	) -> (Option<Decimal>, Option<Decimal>) {
		(base_twap, quote_twap)
	}
}

// -------------------------------------------------------------------------------------------------
//                                           Unit Tests
// -------------------------------------------------------------------------------------------------

#[test]
fn update_twap_fails_if_vamm_does_not_exist() {
	let vamm_state = VammState::default();
	let base_twap = Some(Decimal::from_inner(10));
	let quote_twap = Some(Decimal::from_inner(10));
	ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
		.build()
		.execute_with(|| {
			assert_noop!(
				TestPallet::update_twap(1, None, None),
				Error::<MockRuntime>::VammDoesNotExist
			);

			assert_noop!(
				TestPallet::update_twap(1, base_twap, quote_twap),
				Error::<MockRuntime>::VammDoesNotExist
			);
		})
}

#[test]
fn update_twap_fails_if_vamm_is_closed() {
	let vamm_state = VammState { closed: Some(Timestamp::MIN), ..Default::default() };
	let base_twap = Some(Decimal::from_inner(10));
	let quote_twap = Some(Decimal::from_inner(10));
	ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
		.build()
		.execute_with(|| {
			// For event emission
			run_for_seconds(vamm_state.closed.unwrap() + 1);

			assert_noop!(
				TestPallet::update_twap(0, None, None),
				Error::<MockRuntime>::VammIsClosed
			);

			assert_noop!(
				TestPallet::update_twap(0, base_twap, quote_twap),
				Error::<MockRuntime>::VammIsClosed
			);
		})
}

#[test]
fn update_twap_fails_if_new_twap_is_zero() {
	let vamm_state = VammState::default();
	let base_twap = Some(Decimal::from_inner(0));
	let base_twap_plus = Some(Decimal::from_inner(1));
	let quote_twap = Some(Decimal::from_inner(0));
	let quote_twap_plus = Some(Decimal::from_inner(1));
	ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
		.build()
		.execute_with(|| {
			assert_noop!(
				TestPallet::update_twap(0, base_twap, quote_twap_plus),
				Error::<MockRuntime>::NewTwapValueIsZero
			);

			assert_noop!(
				TestPallet::update_twap(0, base_twap_plus, quote_twap),
				Error::<MockRuntime>::NewTwapValueIsZero
			);
		})
}

#[test]
fn update_twap_fails_if_twap_timestamp_is_more_recent() {
	let timestamp = Timestamp::MIN;
	let timestamp_greater = Timestamp::MIN + 1;
	let vamm_state = VammState { twap_timestamp: timestamp_greater, ..Default::default() };
	let new_twap = Some(Decimal::from_inner(10));
	ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
		.build()
		.execute_with(|| {
			// For event emission
			run_for_seconds(timestamp);

			assert_noop!(
				TestPallet::update_twap(0, new_twap, new_twap),
				Error::<MockRuntime>::AssetTwapTimestampIsMoreRecent
			);

			assert_noop!(
				TestPallet::update_twap(0, None, new_twap),
				Error::<MockRuntime>::AssetTwapTimestampIsMoreRecent
			);

			assert_noop!(
				TestPallet::update_twap(0, new_twap, None),
				Error::<MockRuntime>::AssetTwapTimestampIsMoreRecent
			);

			assert_noop!(
				TestPallet::update_twap(0, None, None),
				Error::<MockRuntime>::AssetTwapTimestampIsMoreRecent
			);
		})
}

#[test]
fn update_twap_succeeds() {
	let timestamp = Timestamp::MIN;
	let twap = 10_u128.pow(18);
	let new_twap = Some(Decimal::from_inner(10_u128.pow(18) * 5));
	let vamm_state = VammState::<Balance, Timestamp, Decimal> {
		twap_timestamp: timestamp,
		base_asset_twap: twap.into(),
		quote_asset_twap: twap.into(),
		base_asset_reserves: twap,
		quote_asset_reserves: twap,
		twap_period: 3600,
		peg_multiplier: 1,
		..Default::default()
	};
	ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
		.build()
		.execute_with(|| {
			run_for_seconds(1);
			assert_ok!(
				TestPallet::update_twap(0, new_twap, new_twap),
				(new_twap.unwrap(), new_twap.unwrap())
			);

			run_for_seconds(1);
			assert_ok!(TestPallet::update_twap(0, None, new_twap));

			run_for_seconds(1);
			assert_ok!(TestPallet::update_twap(0, new_twap, None));

			run_for_seconds(1);
			assert_ok!(TestPallet::update_twap(0, None, None));
		})
}

#[test]
fn should_update_twap_correctly() {
	ExtBuilder::default().build().execute_with(|| {
		let vamm_creation = TestPallet::create(&VammConfig {
			base_asset_reserves: 10_u128.pow(18) * 2,
			quote_asset_reserves: 10_u128.pow(18) * 50,
			peg_multiplier: 1,
			twap_period: 3600,
		});
		let vamm_id = vamm_creation.unwrap();
		let original_base_twap = TestPallet::get_vamm(vamm_id).unwrap().base_asset_twap;
		let original_quote_twap = TestPallet::get_vamm(vamm_id).unwrap().quote_asset_twap;
		assert_ok!(vamm_creation);

		// For event emission & twap update
		run_to_block(1);
		let new_twap = Some(Decimal::from_inner(10_u128.pow(18) * 100));
		assert_ok!(TestPallet::update_twap(vamm_id, new_twap, new_twap));
		let vamm_state = TestPallet::get_vamm(vamm_id).unwrap();
		assert_eq!(vamm_state.base_asset_twap, new_twap.unwrap());
		assert_eq!(vamm_state.quote_asset_twap, new_twap.unwrap());
		System::assert_last_event(Event::TestPallet(pallet::Event::UpdatedTwap {
			vamm_id,
			base_twap: new_twap.unwrap(),
			quote_twap: new_twap.unwrap(),
		}));

		// Run for long enough in order to approximate to the original twap value.
		run_for_seconds(TestPallet::get_vamm(vamm_id).unwrap().twap_period.saturating_pow(2));
		assert_ok!(TestPallet::update_twap(vamm_id, None, None));
		let vamm_state = TestPallet::get_vamm(vamm_id).unwrap();
		assert_ok!(default_acceptable_computation_error(
			vamm_state.base_asset_twap.into_inner(),
			original_base_twap.into_inner(),
		));
		assert_ok!(default_acceptable_computation_error(
			vamm_state.quote_asset_twap.into_inner(),
			original_quote_twap.into_inner(),
		));
		System::assert_last_event(Event::TestPallet(pallet::Event::UpdatedTwap {
			vamm_id,
			base_twap: vamm_state.base_asset_twap,
			quote_twap: vamm_state.quote_asset_twap,
		}));
	})
}

// -------------------------------------------------------------------------------------------------
//                                           Proptests
// -------------------------------------------------------------------------------------------------

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn update_twap_proptest_succeeds(
		vamm_state in any_vamm_state(),
		(base_twap, quote_twap) in any_new_twap()
	) {
		let now = vamm_state.twap_timestamp
							.saturating_add(1)
							.min(Timestamp::MAX/1000);
		let vamm_state = VammState {
			closed: None,
			twap_timestamp: vamm_state.twap_timestamp
										.min(now).saturating_sub(1),
			twap_period: vamm_state.twap_timestamp
										.saturating_add(1),
			..vamm_state
		};
		ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
			.build()
			.execute_with(|| {
				run_for_seconds(now);
				assert_ok!(
					TestPallet::update_twap(0, base_twap, quote_twap),
				);
			})
	}
}
