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
	fn get_price_quote_asset_returns_overflow_error(
		base_asset_reserves in balance_range_upper_half(),
		quote_asset_reserves in balance_range(),
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
					 invariant: Default::default(),
					 closed: None})]
		}.build().execute_with(|| {
			assert_noop!(
				TestPallet::get_price(0, AssetType::Quote),
				ArithmeticError::Overflow);
		})
	}

	#[test]
	#[allow(clippy::disallowed_methods)]
	fn get_price_quote_asset_returns_division_by_zero_error(
		(base_asset_reserves, peg_multiplier) in asset_times_peg_dont_overflow(),
	) {
		let quote_asset_reserves = 0_u128;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![
				(0,
				 VammState{
					 base_asset_reserves,
					 quote_asset_reserves,
					 peg_multiplier,
					 invariant: Default::default(),
					 closed: None})]
		}.build().execute_with(|| {
			assert_noop!(
				TestPallet::get_price(0, AssetType::Quote),
				ArithmeticError::DivisionByZero);
		})
	}

	#[test]
	#[allow(clippy::disallowed_methods)]
	fn get_price_quote_asset_succeeds(
		quote_asset_reserves in any_sane_asset_amount(),
		(base_asset_reserves, peg_multiplier) in asset_times_peg_dont_overflow(),
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
					 invariant: Default::default(),
					 closed: None})]
		}.build().execute_with(|| {
			let base_peg = base_asset_reserves_decimal.checked_mul(&peg_multiplier_decimal);

			assert_ok!(
				TestPallet::get_price(0, AssetType::Quote),
				base_peg.unwrap().checked_div(&quote_asset_reserves_decimal).unwrap())
		})
	}
}
