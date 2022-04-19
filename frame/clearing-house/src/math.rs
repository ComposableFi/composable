use crate::Config;
use num_traits::{CheckedMul, CheckedSub};
use sp_runtime::{ArithmeticError, FixedPointNumber};

pub fn decimal_abs_to_balance<T: Config>(decimal: &T::Decimal) -> T::Balance {
	decimal
		.saturating_abs()
		.into_inner()
		.try_into()
		.map_err(|_| ArithmeticError::Underflow)
		.expect("An absolute of Integer can always be converted to Balance")
}

pub fn decimal_checked_mul<T: Config>(
	a: &T::Decimal,
	b: &T::Decimal,
) -> Result<T::Decimal, ArithmeticError> {
	a.checked_mul(b).ok_or_else(|| match a.is_negative() ^ b.is_negative() {
		true => ArithmeticError::Underflow,
		false => ArithmeticError::Overflow,
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
		true => ArithmeticError::Overflow,
		false => ArithmeticError::Underflow,
	})
}
