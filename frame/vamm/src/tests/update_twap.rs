use crate::{
	mock::{Event, ExtBuilder, MockRuntime, System, TestPallet},
	pallet::{self, Error},
	tests::{
		any_sane_asset_amount, any_vamm_state, get_vamm_state, run_for_seconds, run_to_block,
		RUN_CASES,
	},
	VammState,
};
use composable_traits::vamm::{AssetType, Vamm as VammTrait};
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;

// ----------------------------------------------------------------------------------------------------
//                                           Setup
// ----------------------------------------------------------------------------------------------------

type VammTimestamp = <MockRuntime as pallet::Config>::Moment;
type VammDecimal = <MockRuntime as pallet::Config>::Decimal;

// ----------------------------------------------------------------------------------------------------
//                                           Prop Compose
// ----------------------------------------------------------------------------------------------------

prop_compose! {
	fn any_new_twap()(
		twap in any_sane_asset_amount()
	) (
		new_twap in prop_oneof![Just(None), Just(Some(VammDecimal::from_inner(twap)))],
	) -> Option<VammDecimal> {
		new_twap
	}
}

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
	let twap = 10_u128.pow(18);
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
fn update_twap_updates_twap_correctly() {
	let timestamp = VammTimestamp::MIN;
	let mut timestamp_greater = VammTimestamp::MIN + 1;
	let twap = 10_u128.pow(18);
	let new_twap = Some(VammDecimal::from_inner(10_u128.pow(18) * 5));
	let vamm_id = 0;
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
	ExtBuilder { vamm_count: 1, vamms: vec![(vamm_id, vamm_state)] }
		.build()
		.execute_with(|| {
			// For event emission
			run_to_block(timestamp_greater);
			let asset_type = AssetType::Base;
			assert_ok!(TestPallet::update_twap(vamm_id, asset_type, new_twap));
			assert_eq!(
				VammDecimal::from_inner(TestPallet::get_vamm(vamm_id).unwrap().base_asset_twap),
				new_twap.unwrap()
			);
			System::assert_last_event(Event::TestPallet(pallet::Event::UpdatedTwap {
				vamm_id,
				asset_type,
				value: new_twap.unwrap(),
			}));

			timestamp_greater += 1;
			run_to_block(timestamp_greater);
			let asset_type = AssetType::Quote;
			assert_ok!(TestPallet::update_twap(vamm_id, asset_type, new_twap));
			assert_eq!(
				VammDecimal::from_inner(TestPallet::get_vamm(0).unwrap().quote_asset_twap),
				new_twap.unwrap()
			);
			System::assert_last_event(Event::TestPallet(pallet::Event::UpdatedTwap {
				vamm_id,
				asset_type,
				value: new_twap.unwrap(),
			}));

			timestamp_greater += 1;
			run_to_block(timestamp_greater);
			let asset_type = AssetType::Base;
			let value = VammDecimal::from_inner(4997222222222222222);
			assert_ok!(TestPallet::update_twap(vamm_id, asset_type, None));
			assert_eq!(
				VammDecimal::from_inner(TestPallet::get_vamm(vamm_id).unwrap().base_asset_twap),
				value
			);
			System::assert_last_event(Event::TestPallet(pallet::Event::UpdatedTwap {
				vamm_id,
				asset_type,
				value,
			}));

			timestamp_greater += 1;
			run_to_block(timestamp_greater);
			let asset_type = AssetType::Quote;
			let value = VammDecimal::from_inner(4997222222222222222);
			assert_ok!(TestPallet::update_twap(vamm_id, asset_type, None));
			assert_eq!(
				VammDecimal::from_inner(TestPallet::get_vamm(vamm_id).unwrap().quote_asset_twap),
				value
			);
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
		asset_type in prop_oneof![Just(AssetType::Base), Just(AssetType::Quote)],
		new_twap in any_new_twap()
	) {
		let now = vamm_state.base_asset_twap_timestamp
							.max(vamm_state.quote_asset_twap_timestamp)
							.saturating_add(1)
							.min(VammTimestamp::MAX/1000);
		let vamm_state = VammState {
			closed: None,
			base_asset_twap_timestamp: vamm_state.base_asset_twap_timestamp
												 .min(now).saturating_sub(1),
			quote_asset_twap_timestamp: vamm_state.quote_asset_twap_timestamp
												  .min(now).saturating_sub(1),
			funding_period: vamm_state.base_asset_twap_timestamp
									  .max(vamm_state.quote_asset_twap_timestamp)
									  .saturating_add(1),
			..vamm_state
		};
		ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
			.build()
			.execute_with(|| {
				run_for_seconds(now);
				assert_ok!(
					TestPallet::update_twap(0, asset_type, new_twap),
				);
			})
	}
}
