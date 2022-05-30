use crate::{
	mock::{ExtBuilder, TestPallet},
	pallet::VammState,
	tests::{
		any_sane_asset_amount, asset_times_peg_dont_overflow, balance_range,
		balance_range_upper_half, Decimal, RUN_CASES,
	},
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
	fn get_price_base_asset_returns_overflow_error(
		base_asset_reserves in balance_range(),
		quote_asset_reserves in balance_range_upper_half(),
		peg_multiplier in balance_range_upper_half(),
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
			assert_noop!(
				TestPallet::get_price(0, AssetType::Base),
				ArithmeticError::Overflow);
		})
	}

	#[test]
	#[allow(clippy::disallowed_methods)]
	fn get_price_base_asset_returns_division_by_zero_error(
		(quote_asset_reserves, peg_multiplier) in asset_times_peg_dont_overflow(),
	) {
		let base_asset_reserves = 0_u128;

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
			assert_noop!(
				TestPallet::get_price(0, AssetType::Base),
				ArithmeticError::DivisionByZero);
		})
	}

	#[test]
	#[allow(clippy::disallowed_methods)]
	fn get_price_base_asset_succeeds(
		base_asset_reserves in any_sane_asset_amount(),
		(quote_asset_reserves, peg_multiplier) in asset_times_peg_dont_overflow(),
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

			assert_ok!(
				TestPallet::get_price(0, AssetType::Base),
				quote_peg.unwrap().checked_div(&base_asset_reserves_decimal).unwrap())
		})
	}
}
