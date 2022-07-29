use crate::{
	mock::{MockRuntime, TestPallet},
	pallet::Error,
	tests::{constants::RUN_CASES, helpers::as_decimal, helpers_propcompose::balance_range},
};
use frame_support::{assert_err, assert_ok};
use proptest::prelude::*;
use sp_core::U256;
use sp_runtime::FixedPointNumber;

// -------------------------------------------------------------------------------------------------
//                                            Unit Tests
// -------------------------------------------------------------------------------------------------

#[test]
fn should_succeed_computing_correct_invariant() {
	assert_ok!(
		TestPallet::compute_invariant(as_decimal(2).into_inner(), as_decimal(50).into_inner()),
		U256::from(as_decimal(1).into_inner().pow(2) * 100)
	);
}

// -------------------------------------------------------------------------------------------------
//                                             Proptests
// -------------------------------------------------------------------------------------------------

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn should_succeed_computing_invariant(
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

	#[test]
	fn should_fail_if_base_is_zero(
		quote in balance_range(),
	) {
		prop_assume!(quote != 0);
		let base = 0;

		assert_err!(
			TestPallet::compute_invariant(base, quote),
			Error::<MockRuntime>::BaseAssetReserveIsZero
		);
	}

	#[test]
	fn should_fail_if_quote_is_zero(
		base in balance_range(),
	) {
		prop_assume!(base != 0);
		let quote = 0;

		assert_err!(
			TestPallet::compute_invariant(base, quote),
			Error::<MockRuntime>::QuoteAssetReserveIsZero
		);
	}
}
