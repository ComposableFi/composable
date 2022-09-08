use sp_arithmetic::Rounding;
use sp_runtime::{
	helpers_128bit::multiply_by_rational_with_rounding,
	traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Zero},
	ArithmeticError,
};

/// Thin wrapper around [`multiply_by_rational`], transforming any errors into [`ArithmeticError`]
/// for easier use in pallets and Dispatch-related contexts. See [`multiply_by_rational`] for more
/// information.
///
/// Note: [`multiply_by_rational`] clamps `c` at a minimum of `1`, which can be confusing.
/// [`safe_multiply_by_rational`] instead returns a divide by zero error if `c == 0`.
pub fn safe_multiply_by_rational(a: u128, b: u128, c: u128) -> Result<u128, ArithmeticError> {
	if c == 0 {
		Err(ArithmeticError::DivisionByZero)
	} else {
		multiply_by_rational_with_rounding(a, b, c, Rounding::Down).ok_or(ArithmeticError::Overflow)
	}
}

// little bit slower than maximizing performance by knowing constraints.
// Example, you sum to negative numbers, can get underflow, so need to check on each add; but if
// you have positive number only, you cannot have underflow. Same for other constrains, like non
// zero divisor.

pub trait SafeAdd: Sized {
	fn safe_add(&self, rhs: &Self) -> Result<Self, ArithmeticError>;
}

pub trait SafeDiv: Sized {
	fn safe_div(&self, rhs: &Self) -> Result<Self, ArithmeticError>;
}

pub trait SafeMul: Sized {
	fn safe_mul(&self, rhs: &Self) -> Result<Self, ArithmeticError>;
}

pub trait SafeSub: Sized {
	fn safe_sub(&self, rhs: &Self) -> Result<Self, ArithmeticError>;
}

pub trait SafeArithmetic: Sized + SafeAdd + SafeDiv + SafeMul + SafeSub {}

impl<T: CheckedAdd> SafeAdd for T {
	#[inline(always)]
	fn safe_add(&self, rhs: &Self) -> Result<Self, ArithmeticError> {
		self.checked_add(rhs).ok_or(ArithmeticError::Overflow)
	}
}

impl<T: CheckedDiv + Zero> SafeDiv for T {
	#[inline(always)]
	fn safe_div(&self, rhs: &Self) -> Result<Self, ArithmeticError> {
		if rhs.is_zero() {
			return Err(ArithmeticError::DivisionByZero)
		}
		self.checked_div(rhs).ok_or(ArithmeticError::Overflow)
	}
}

impl<T: CheckedMul + Zero> SafeMul for T {
	#[inline(always)]
	fn safe_mul(&self, rhs: &Self) -> Result<Self, ArithmeticError> {
		self.checked_mul(rhs).ok_or(ArithmeticError::Overflow)
	}
}

impl<T: CheckedSub + Zero> SafeSub for T {
	#[inline(always)]
	fn safe_sub(&self, rhs: &Self) -> Result<Self, ArithmeticError> {
		self.checked_sub(rhs).ok_or(ArithmeticError::Underflow)
	}
}

impl<T: Sized + SafeAdd + SafeDiv + SafeMul + SafeSub> SafeArithmetic for T {}
