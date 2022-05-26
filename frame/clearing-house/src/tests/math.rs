use crate::math::{div_rem_with_acc, multiply_by_rational};
use proptest::prelude::*;
use sp_arithmetic::{
	biguint::{Double, Single},
	helpers_128bit::to_big_uint,
	FixedI128, FixedI64, FixedPointNumber, FixedU128,
};

// -------------------------------------------------------------------------------------------------
//                                          Prop compose
// -------------------------------------------------------------------------------------------------

prop_compose! {
	fn nonzero_single()(n in 1..=Single::MAX) -> u128 {
		n as u128
	}
}

prop_compose! {
	fn nonzero_double()(n in (Single::MAX as Double + 1)..=Double::MAX) -> u128 {
		n as u128
	}
}

prop_compose! {
	fn nonzero_quad()(n in (Double::MAX as u128 + 1)..=u128::MAX) -> u128 {
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
	fn a_and_acc_for_branch_2a()(
		a in 1..=u128::MAX
	)(
		a in Just(a),
		acc in (2_u128 << a.leading_zeros())..=(1_u128 << (a.leading_zeros() + 31))
	) -> (u128, u128) {
		(a, acc)
	}
}

fn resulting_mul_max_size(x: u128, y: u128) -> u32 {
	if x == 0 || y == 0 {
		0
	} else {
		let x_len = 128 - x.leading_zeros();
		let y_len = 128 - y.leading_zeros();
		x_len + y_len
	}
}

prop_compose! {
	fn non_overflowing_inputs_for_branch_2a()(
		(a, acc) in a_and_acc_for_branch_2a()
	)(
		(a, acc) in Just((a, acc)),
		// a * acc is guaranteed to overflow by at least 1 bit and at most 31 bits
		b in (1_u128 << (resulting_mul_max_size(a, acc) - 128))..=(Single::MAX as u128)
	) -> (u128, u128, u128) {
		(a, b, acc)
	}
}
// -------------------------------------------------------------------------------------------------
//                                           Unit tests
// -------------------------------------------------------------------------------------------------

#[test]
fn maximal_values_for_branch_1() {
	let a = Double::MAX as u128 + 1;
	let acc = Double::MAX as u128;

	assert!(a.checked_mul(acc).is_some());
	assert!(a.checked_mul(acc + 1).is_none());
}

// -------------------------------------------------------------------------------------------------
//                                         Property tests
// -------------------------------------------------------------------------------------------------

proptest! {
	#[test]
	fn div_rem_with_acc_zero_dividend_doesnt_panic(b in nonzero_quad(), acc in nonzero_quad()) {
		div_rem_with_acc(0, b, acc);
	}

	#[test]
	fn div_rem_with_acc_zero_acc_doesnt_panic(a in nonzero_quad(), b in nonzero_quad()) {
		div_rem_with_acc(a, b, 0);
	}

	#[test]
	fn div_rem_with_acc_zero_divisor_result_equals_unit_divisor_result(
		a in nonzero_quad(),
		acc in nonzero_quad()
	) {
		assert_eq!(div_rem_with_acc(a, 0, acc), div_rem_with_acc(a, 1, acc));
	}

	#[test]
	fn div_rem_with_acc_branch_1_doesnt_panic(
		a in prop_oneof![nonzero_single(), nonzero_double()],
		b in any::<u128>(),
		acc in prop_oneof![nonzero_single(), nonzero_double()],
	) {
		div_rem_with_acc(a, b, acc);
	}

	#[test]
	fn trigger_branch_2((a, acc) in a_and_acc_for_branch_2a()) {
		assert!(a.checked_mul(acc).is_none());
	}

	#[test]
	fn overflow_is_less_than_31_bits((a, acc) in a_and_acc_for_branch_2a()) {
		assert!(resulting_mul_max_size(a, acc) - 128 <= 31);
	}

	#[test]
	fn trigger_branch_2a((a, b, acc) in non_overflowing_inputs_for_branch_2a()) {
		assert!(a.checked_mul(acc).is_none());
		b as u32;
	}

	#[test]
	fn div_rem_with_acc_branch_2a_output_is_correct(
		(a, b, acc) in non_overflowing_inputs_for_branch_2a()
	) {
		let (q, r) = div_rem_with_acc(a, b, acc).unwrap();
		let qb = multiply_by_rational(q, b, acc).unwrap();
		assert_eq!(a - qb, r);
	}
}
