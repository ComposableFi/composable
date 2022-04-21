use crate::Config;
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Signed};
use sp_runtime::{
	traits::Zero,
	ArithmeticError,
	ArithmeticError::{DivisionByZero, Overflow, Underflow},
	FixedPointNumber,
};

pub fn integer_to_balance<T: Config>(int: &T::Integer) -> T::Balance {
	int.abs()
		.try_into()
		.map_err(|_| Underflow)
		.expect("An absolute of Integer can always be converted to Balance")
}

pub fn decimal_abs_to_balance<T: Config>(decimal: &T::Decimal) -> T::Balance {
	integer_to_balance::<T>(&decimal.into_inner())
}

pub fn decimal_from_balance<T: Config>(
	balance: &T::Balance,
) -> Result<T::Decimal, ArithmeticError> {
	Ok(T::Decimal::from_inner((*balance).try_into().map_err(|_| ArithmeticError::Overflow)?))
}

pub fn decimal_checked_add<T: Config>(
	a: &T::Decimal,
	b: &T::Decimal,
) -> Result<T::Decimal, ArithmeticError> {
	// sign(a) sign(b) | CheckedAdd
	// ----------------------------
	//      -1      -1 | Underflow
	//      -1       1 | Ok
	//       1      -1 | Ok
	//       1       1 | Overflow
	a.checked_add(b).ok_or_else(|| match a.is_positive() {
		true => Overflow,
		false => Underflow,
	})
}

pub fn decimal_checked_sub<T: Config>(
	a: &T::Decimal,
	b: &T::Decimal,
) -> Result<T::Decimal, ArithmeticError> {
	// sign(a) sign(b) | CheckedSub
	// ----------------------------
	//      -1      -1 | Ok
	//      -1       1 | Underflow
	//       1      -1 | Overflow
	//       1       1 | Ok
	a.checked_sub(b).ok_or_else(|| match a.is_positive() {
		true => Overflow,
		false => Underflow,
	})
}

pub fn decimal_checked_mul<T: Config>(
	a: &T::Decimal,
	b: &T::Decimal,
) -> Result<T::Decimal, ArithmeticError> {
	// sign(a) sign(b) | CheckedMul
	// ----------------------------
	//      -1      -1 | Overflow
	//      -1       1 | Underflow
	//       1      -1 | Underflow
	//       1       1 | Overflow
	a.checked_mul(b).ok_or_else(|| match a.is_negative() ^ b.is_negative() {
		true => Underflow,
		false => Overflow,
	})
}

pub fn decimal_checked_div<T: Config>(
	a: &T::Decimal,
	b: &T::Decimal,
) -> Result<T::Decimal, ArithmeticError> {
	// sign(a) sign(b) | CheckedDiv
	// ----------------------------
	//      -1      -1 | Overflow
	//      -1       1 | Underflow
	//       1      -1 | Underflow
	//       1       1 | Overflow
	a.checked_div(b).ok_or_else(|| {
		if b.is_zero() {
			DivisionByZero
		} else {
			match a.is_negative() ^ b.is_negative() {
				true => Underflow,
				false => Overflow,
			}
		}
	})
}
