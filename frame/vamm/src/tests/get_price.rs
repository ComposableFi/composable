use crate::{
	mock::{ExtBuilder, TestPallet},
	pallet::VammState,
	tests::{min_max_reserve, Decimal, RUN_CASES},
};
use composable_traits::vamm::{AssetType, Vamm as VammTrait};
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;
use sp_runtime::ArithmeticError;

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	#[allow(clippy::disallowed_methods)]
	fn get_price_base_asset(
		(base_asset_reserves, quote_asset_reserves, peg_multiplier) in min_max_reserve(),
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
			let quote_peg = quote_asset_reserves.checked_mul(peg_multiplier);

			if quote_peg.is_none() {
				assert_noop!(
					TestPallet::get_price(0, AssetType::Base),
					ArithmeticError::Overflow);
			} else if quote_peg.unwrap().checked_div(base_asset_reserves).is_none() {
				assert_noop!(
					TestPallet::get_price(0, AssetType::Base),
					ArithmeticError::DivisionByZero);
			} else {
				assert_ok!(
					TestPallet::get_price(0, AssetType::Base),
					Decimal::from_inner(quote_peg.unwrap().checked_div(base_asset_reserves).unwrap()))
			}
		})
	}

	#[test]
	#[allow(clippy::disallowed_methods)]
	fn get_price_quote_asset(
		(base_asset_reserves, quote_asset_reserves, peg_multiplier) in min_max_reserve(),
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
			let quote_peg = quote_asset_reserves.checked_mul(peg_multiplier);

			if quote_peg.is_none() {
				assert_noop!(
					TestPallet::get_price(0, AssetType::Quote),
					ArithmeticError::Overflow);
			} else if quote_peg.unwrap().checked_div(base_asset_reserves).is_none() {
				assert_noop!(
					TestPallet::get_price(0, AssetType::Quote),
					ArithmeticError::DivisionByZero);
			} else {
				assert_ok!(
					TestPallet::get_price(0, AssetType::Quote),
					Decimal::from_inner(quote_peg.unwrap().checked_div(base_asset_reserves).unwrap()))
			}
		})
	}
}
