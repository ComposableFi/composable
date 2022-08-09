use crate::labs::numbers::SignedMath;
use frame_support::{assert_err, assert_ok};
use proptest::prelude::*;
use proptest_derive::Arbitrary;
use rstest::rstest;
use sp_runtime::ArithmeticError::{self, DivisionByZero, Overflow, Underflow};

// -------------------------------------------------------------------------------------------------
//                                              Helpers
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Arbitrary)]
enum TryOp {
	Add,
	Sub,
	Mul,
	Div,
}

// -------------------------------------------------------------------------------------------------
//                                         Prop Compose
// -------------------------------------------------------------------------------------------------
prop_compose! {
	fn x_lt(y: i128)(
		x in i128::MIN..y,
		y in Just(y)
	) -> (i128, i128) {
		(x, y)
	}
}

prop_compose! {
	fn x_lt_y()(
		y in i128::MIN+1..=i128::MAX,
	)(
		(x, y) in x_lt(y),
	) -> (i128, i128) {
		(x, y)
	}
}

// -------------------------------------------------------------------------------------------------
//                                           Unit tests
// -------------------------------------------------------------------------------------------------

#[rstest]
#[case(TryOp::Add, 42, 1337, 1379)]
#[case(TryOp::Sub, 42, 1337, -1295)]
#[case(TryOp::Mul, 42, 1337, 56154)]
#[case(TryOp::Div, 42, 1337, 0)]
fn should_succeed_try_operation_cases(
	#[case] op: TryOp,
	#[case] a: i128,
	#[case] b: i128,
	#[case] eq: i128,
) {
	assert_ok!(
		match op {
			TryOp::Add => a.try_add(&b),
			TryOp::Sub => a.try_sub(&b),
			TryOp::Mul => a.try_mul(&b),
			TryOp::Div => a.try_div(&b),
		},
		eq
	);
}

#[rstest]
#[case(TryOp::Add, i128::MAX, i128::MAX, Overflow)]
#[case(TryOp::Add, i128::MIN, i128::MIN, Underflow)]
#[case(TryOp::Sub, i128::MAX, i128::MIN, Overflow)]
#[case(TryOp::Sub, i128::MIN, i128::MAX, Underflow)]
#[case(TryOp::Mul, i128::MAX, i128::MAX, Overflow)]
#[case(TryOp::Mul, i128::MIN, i128::MIN, Overflow)]
#[case(TryOp::Mul, i128::MAX, i128::MIN, Underflow)]
#[case(TryOp::Mul, i128::MIN, i128::MAX, Underflow)]
#[case(TryOp::Div, i128::MAX, 0, DivisionByZero)]
fn should_fail_with_correct_values_try_operation_cases(
	#[case] op: TryOp,
	#[case] a: i128,
	#[case] b: i128,
	#[case] eq: ArithmeticError,
) {
	assert_err!(
		match op {
			TryOp::Add => a.try_add(&b),
			TryOp::Sub => a.try_sub(&b),
			TryOp::Mul => a.try_mul(&b),
			TryOp::Div => a.try_div(&b),
		},
		eq
	);
}

// -------------------------------------------------------------------------------------------------
//                                         Property tests
// -------------------------------------------------------------------------------------------------

proptest! {
	#[test]
	fn should_succeed_try_operations(
		op in any::<TryOp>(),
		(x, y) in x_lt_y(),
	) {
		match match op {
			TryOp::Add => (x.try_add(&y), x.checked_add(y)),
			TryOp::Sub => (x.try_sub(&y), x.checked_sub(y)),
			TryOp::Mul => (x.try_mul(&y), x.checked_mul(y)),
			TryOp::Div => (x.try_div(&y), x.checked_div(y)),
		} {
			(Ok(a), Some(b)) => {
				assert_eq!(a, b);
			},
			(Err(_), None) => (),
			_ => {
				panic!("This combination should never happen.");
			},
		}
	}
}
