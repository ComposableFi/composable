use crate::{
	mock::{ExtBuilder, MockRuntime, TestPallet},
	pallet::{Error, VammState},
	tests::{any_sane_asset_amount, balance_range, balance_range_upper_half, Decimal, RUN_CASES},
};
use composable_traits::vamm::{AssetType, Vamm as VammTrait};
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;
use sp_runtime::{
	traits::{CheckedDiv, CheckedMul},
	ArithmeticError,
};

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	#[allow(clippy::disallowed_methods)]
	fn get_price_base_asset(
		base_asset_reserves in any_sane_asset_amount(),
		quote_asset_reserves in any_sane_asset_amount(),
		peg_multiplier in any_sane_asset_amount(),
	) {

		let base_asset_reserves_decimal =
			Decimal::from_inner(base_asset_reserves);
		let quote_asset_reserves_decimal =
			Decimal::from_inner(quote_asset_reserves);
		let peg_multiplier_decimal = Decimal::from_inner(peg_multiplier);

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
			let quote_peg = quote_asset_reserves_decimal.checked_mul(&peg_multiplier_decimal);

			if quote_peg.is_none() {
				assert_noop!(
					TestPallet::get_price(0, AssetType::Base),
					ArithmeticError::Overflow);
			} else if quote_peg.unwrap().checked_div(&base_asset_reserves_decimal).is_none() {
				assert_noop!(
					TestPallet::get_price(0, AssetType::Base),
					ArithmeticError::DivisionByZero);
			} else {
				assert_ok!(
					TestPallet::get_price(0, AssetType::Base),
					quote_peg.unwrap().checked_div(&base_asset_reserves_decimal).unwrap())
			}
		})
	}

	#[test]
	#[allow(clippy::disallowed_methods)]
	fn get_price_quote_asset(
		base_asset_reserves in any_sane_asset_amount(),
		quote_asset_reserves in any_sane_asset_amount(),
		peg_multiplier in any_sane_asset_amount(),
	) {

		let base_asset_reserves_decimal =
			Decimal::from_inner(base_asset_reserves);
		let quote_asset_reserves_decimal =
			Decimal::from_inner(quote_asset_reserves);
		let peg_multiplier_decimal = Decimal::from_inner(peg_multiplier);

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
			let base_peg = base_asset_reserves_decimal.checked_mul(&peg_multiplier_decimal);

			if base_peg.is_none() {
				assert_noop!(
					TestPallet::get_price(0, AssetType::Quote),
					ArithmeticError::Overflow);
			} else if base_peg.unwrap().checked_div(&quote_asset_reserves_decimal).is_none() {
				assert_noop!(
					TestPallet::get_price(0, AssetType::Quote),
					ArithmeticError::DivisionByZero);
			} else {
				assert_ok!(
					TestPallet::get_price(0, AssetType::Quote),
					base_peg.unwrap().checked_div(&quote_asset_reserves_decimal).unwrap())
			}
		})
	}
}
