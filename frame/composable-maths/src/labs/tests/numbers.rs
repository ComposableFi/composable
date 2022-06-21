use crate::{div_mod_with_acc, multiply_by_rational};
use frame_support::assert_err;
use proptest::prelude::*;
use sp_arithmetic::{FixedI128, FixedI64, FixedPointNumber, FixedU128};
use sp_runtime::ArithmeticError;

fn check_reconstruction(a: u128, b: u128, acc: u128, q: u128, r: u128) {
	let qb = multiply_by_rational(q, b, acc).unwrap();
	assert_eq!(a, qb + r);
}

// -------------------------------------------------------------------------------------------------
//                                          Prop compose
// -------------------------------------------------------------------------------------------------

prop_compose! {
	fn nonzero_quad()(n in 1..=u128::MAX) -> u128 {
		n
	}
}

prop_compose! {
	fn fixed_point_number_accs()(
		acc in prop_oneof![
			Just(FixedI128::DIV as u128),
			Just(FixedI64::DIV as u128),
			Just(FixedU128::DIV)]
	) -> u128 {
		acc
	}
}

prop_compose! {
	fn a_and_acc_for_branch_2()(
		a in 1..=u128::MAX
	)(
		a in Just(a),
		acc in (2_u128 << a.leading_zeros())..=u128::MAX
	) -> (u128, u128) {
		(a, acc)
	}
}

// -------------------------------------------------------------------------------------------------
//                                         Property tests
// -------------------------------------------------------------------------------------------------

proptest! {
	#[test]
	fn div_mod_with_acc_zero_dividend_doesnt_panic(b in nonzero_quad(), acc in nonzero_quad()) {
		div_mod_with_acc(0, b, acc);
	}

	#[test]
	fn div_mod_with_acc_zero_acc_doesnt_panic(a in nonzero_quad(), b in nonzero_quad()) {
		div_mod_with_acc(a, b, 0);
	}

	#[test]
	fn div_mod_with_acc_zero_divisor_returns_error(a in nonzero_quad(), acc in nonzero_quad()) {
		assert_err!(div_mod_with_acc(a, 0, acc), ArithmeticError::DivisionByZero);
	}

	#[test]
	fn works_for_expected_accs(
		a in nonzero_quad(),
		b in nonzero_quad(),
		acc in fixed_point_number_accs())
	{
		if let Ok((q, r)) = div_mod_with_acc(a, b, acc) {
			check_reconstruction(a, b, acc, q, r);
		}
	}

	#[test]
	fn trigger_branch_2((a, acc) in a_and_acc_for_branch_2()) {
		assert!(a.checked_mul(acc).is_none());
	}

	#[test]
	fn div_mod_with_acc_branch_2_output_is_correct(
		(a, acc) in a_and_acc_for_branch_2(),
		b in nonzero_quad(),
	) {
		if let Ok((q, r)) = div_mod_with_acc(a, b, acc) {
			check_reconstruction(a, b, acc, q, r);
		}
	}
}

// -------------------------------------------------------------------------------------------------
//                                           Unit tests
// -------------------------------------------------------------------------------------------------

// Test case caught by proptest
#[test]
fn div_mod_with_acc_branch_2_case1() {
	let a = 42217021644542820961452366325;
	let acc = 184399131818744923007405521160233881;
	let b = 291761325472929851911930257455888011844;

	let (q, r) = div_mod_with_acc(a, b, acc).unwrap();
	check_reconstruction(a, b, acc, q, r);
}
