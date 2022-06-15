use crate::{
	mock::{Balance, Event, ExtBuilder, MockRuntime, System, TestPallet},
	pallet::{self, Error},
	tests::{
		any_sane_asset_amount, any_vamm_state, get_vamm_state, run_for_seconds, run_to_block,
		Decimal, Timestamp, RUN_CASES,
	},
	VammState,
};
use composable_traits::vamm::{AssetType, Vamm as VammTrait, VammConfig};
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;

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
	let timestamp = Timestamp::MIN;
	let mut timestamp_greater = Timestamp::MIN + 1;
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
fn should_update_twap_correctly() {
	let timestamp = Timestamp::MIN;
	let mut timestamp_greater = Timestamp::MIN + 1;
	let twap = 10_u128.pow(18);
	let new_twap = Some(Decimal::from_inner(10_u128.pow(18) * 5));
	ExtBuilder::default().build().execute_with(|| {
		let vamm_creation = TestPallet::create(&VammConfig {
			base_asset_reserves: twap,
			quote_asset_reserves: twap,
			peg_multiplier: 1,
			twap_period: 3600,
		});
		let vamm_id = vamm_creation.unwrap();
		assert_ok!(vamm_creation);

		// For event emission
		run_to_block(timestamp_greater);
		let asset_type = AssetType::Base;
		assert_ok!(TestPallet::update_twap(vamm_id, asset_type, new_twap));
		assert_eq!(TestPallet::get_vamm(vamm_id).unwrap().base_asset_twap, new_twap.unwrap());
		System::assert_last_event(Event::TestPallet(pallet::Event::UpdatedTwap {
			vamm_id,
			asset_type,
			value: new_twap.unwrap(),
		}));

		timestamp_greater += 1;
		run_to_block(timestamp_greater);
		let asset_type = AssetType::Quote;
		assert_ok!(TestPallet::update_twap(vamm_id, asset_type, new_twap));
		assert_eq!(TestPallet::get_vamm(0).unwrap().quote_asset_twap, new_twap.unwrap());
		System::assert_last_event(Event::TestPallet(pallet::Event::UpdatedTwap {
			vamm_id,
			asset_type,
			value: new_twap.unwrap(),
		}));

		timestamp_greater += 1;
		run_to_block(timestamp_greater);
		let asset_type = AssetType::Base;
		let value = Decimal::from_inner(4997777777777777777);
		assert_ok!(TestPallet::update_twap(vamm_id, asset_type, None));
		assert_eq!(TestPallet::get_vamm(vamm_id).unwrap().base_asset_twap, value);
		System::assert_last_event(Event::TestPallet(pallet::Event::UpdatedTwap {
			vamm_id,
			asset_type,
			value,
		}));

		timestamp_greater += 1;
		run_to_block(timestamp_greater);
		let asset_type = AssetType::Quote;
		let value = Decimal::from_inner(4997777777777777777);
		assert_ok!(TestPallet::update_twap(vamm_id, asset_type, None));
		assert_eq!(TestPallet::get_vamm(vamm_id).unwrap().quote_asset_twap, value);
		System::assert_last_event(Event::TestPallet(pallet::Event::UpdatedTwap {
			vamm_id,
			asset_type,
			value,
		}));
	})
}

// -------------------------------------------------------------------------------------------------
//                                           Proptests
// -------------------------------------------------------------------------------------------------

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	// #![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
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
