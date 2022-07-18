use crate::{
	mock::{ExtBuilder, MockRuntime, TestPallet},
	pallet::Error,
	tests::{
		helpers::{any_sane_asset_amount, as_decimal, as_decimal_from_fraction, run_for_seconds},
		Timestamp, RUN_CASES,
	},
	types::VammState,
};
use composable_traits::vamm::{AssetType, Vamm as VammTrait};
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;
use sp_runtime::FixedPointNumber;

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
		base_asset_reserves: as_decimal(4).into_inner(),
		quote_asset_reserves: as_decimal(8).into_inner(),
		peg_multiplier: 1,
		..Default::default()
	};

	ExtBuilder { vamm_count: 1, vamms: vec![(0, vamm_state)] }
		.build()
		.execute_with(|| {
			assert_ok!(TestPallet::get_price(0, AssetType::Base), as_decimal(2));
			assert_ok!(TestPallet::get_price(0, AssetType::Quote), as_decimal_from_fraction(5, 10));
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
