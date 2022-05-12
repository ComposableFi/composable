use crate::{
	mock::{MockRuntime, TestPallet},
	pallet::Error,
	tests::{balance_range, RUN_CASES},
};
use frame_support::{assert_err, assert_ok};
use proptest::prelude::*;
use sp_core::U256;

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn compute_invariant_suceeds(
		base in balance_range(),
		quote in balance_range(),
	) {
		prop_assume!(base != 0);
		prop_assume!(quote != 0);

		let base_u256 = U256::from(base);
		let quote_u256 = U256::from(quote);
		let expected_invariant = base_u256 * quote_u256;

		assert_ok!(
			TestPallet::compute_invariant(base, quote),
			expected_invariant
		);
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn compute_invariant_fails_if_base_is_zero(
		quote in balance_range(),
	) {
		prop_assume!(quote != 0);
		let base = quote * 0;

		assert_err!(
			TestPallet::compute_invariant(base, quote),
			Error::<MockRuntime>::BaseAssetReserveIsZero
		);
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn compute_invariant_fails_if_quote_is_zero(
		base in balance_range(),
	) {
		prop_assume!(base != 0);
		let quote = base * 0;

		assert_err!(
			TestPallet::compute_invariant(base, quote),
			Error::<MockRuntime>::QuoteAssetReserveIsZero
		);
	}
}
