use crate::{
	mock::{ExtBuilder, MockRuntime, TestPallet},
	pallet::{Error, VammState},
	tests::{any_sane_asset_amount, run_for_seconds, Decimal, Timestamp, RUN_CASES},
};
use composable_traits::vamm::{AssetType, Vamm as VammTrait};
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;

// -------------------------------------------------------------------------------------------------
//                                           Unit Tests
// -------------------------------------------------------------------------------------------------

#[test]
fn should_fail_if_vamm_does_not_exist() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			TestPallet::get_price(0, AssetType::Base),
			Error::<MockRuntime>::VammDoesNotExist
		);
		assert_noop!(
			TestPallet::get_price(0, AssetType::Quote),
			Error::<MockRuntime>::VammDoesNotExist
		);
	})
}

#[test]
fn should_fail_if_vamm_is_closed() {
	let vamm_state = VammState {
		base_asset_reserves: (10_u128.pow(18) * 4), // 4 units in decimal
		quote_asset_reserves: (10_u128.pow(18) * 8), // 8 units in decimal
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
				TestPallet::get_price(0, AssetType::Base),
				Error::<MockRuntime>::VammIsClosed
			);
			assert_noop!(
				TestPallet::get_price(0, AssetType::Quote),
				Error::<MockRuntime>::VammIsClosed
			);
		})
}

#[test]
fn should_succeed_returning_correct_price() {
	let vamm_state = VammState {
		base_asset_reserves: (10_u128.pow(18) * 4), // 4 units in decimal
		quote_asset_reserves: (10_u128.pow(18) * 8), // 8 units in decimal
		peg_multiplier: 1,
		..Default::default()
	};

	ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
		.build()
		.execute_with(|| {
			assert_ok!(
				TestPallet::get_price(0, AssetType::Base),
				Decimal::from_inner(2000000000000000000) // 2 units in decimal
			);
			assert_ok!(
				TestPallet::get_price(0, AssetType::Quote),
				Decimal::from_inner(500000000000000000) // 0.5 unit in decimal
			);
		})
}

// -------------------------------------------------------------------------------------------------
//                                           Proptests
// -------------------------------------------------------------------------------------------------

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn should_succeed_always(
		base_asset_reserves in any_sane_asset_amount(),
		quote_asset_reserves in any_sane_asset_amount(),
		peg_multiplier in 1..=10_u128.pow(6)
	) {
		ExtBuilder {
			vamm_count: 1,
			vamms: vec![
				(0,
				 VammState{
					 base_asset_reserves,
					 quote_asset_reserves,
					 peg_multiplier,
					 ..Default::default()
				 })]
		}.build().execute_with(|| {
			assert_ok!(TestPallet::get_price(0, AssetType::Base));
			assert_ok!(TestPallet::get_price(0, AssetType::Quote));
		})
	}
}
