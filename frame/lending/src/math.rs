use num_traits::{CheckedDiv, SaturatingSub};
use sp_runtime::{
	traits::{
		AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, One,
		Saturating, Zero,
	},
	ArithmeticError, FixedPointNumber, FixedPointOperand, FixedU128, Perquintill,
};

/// Number like of higher bits, so that amount and balance calculations are done it it with higher
/// precision via fixed point.
/// While this is 128 bit, cannot support u128 because 18 bits are for of mantissa.
/// Can support u128 it lifter to use FixedU256
pub type LiftedFixedBalance = FixedU128;

/// little bit slower than maximizing performance by knowing constraints.
/// Example, you sum to negative numbers, can get underflow, so need to check on each add; but if you have positive number only, you cannot have underflow.
/// Same for other constrains, like non zero divisor.
pub trait ErrorArithmetic: Sized {
	fn error_add(&self, rhs: &Self) -> Result<Self, ArithmeticError>;
	fn error_div(&self, rhs: &Self) -> Result<Self, ArithmeticError>;
	fn error_mul(&self, rhs: &Self) -> Result<Self, ArithmeticError>;
	fn error_sub(&self, rhs: &Self) -> Result<Self, ArithmeticError>;
}

impl ErrorArithmetic for LiftedFixedBalance {
	#[inline(always)]
	fn error_add(&self, rhs: &Self) -> Result<Self, ArithmeticError> {
		self.checked_add(rhs).ok_or(ArithmeticError::Overflow)
	}
	#[inline(always)]
	fn error_div(&self, rhs: &Self) -> Result<Self, ArithmeticError> {
		if rhs.is_zero() {
			return Err(ArithmeticError::DivisionByZero);
		}

		self.checked_div(rhs).ok_or(ArithmeticError::Overflow)
	}

	#[inline(always)]
	fn error_mul(&self, rhs: &Self) -> Result<Self, ArithmeticError> {
		self.checked_mul(rhs).ok_or(ArithmeticError::Overflow)
	}

	#[inline(always)]
	fn error_sub(&self, rhs: &Self) -> Result<Self, ArithmeticError> {
		self.checked_sub(rhs).ok_or(ArithmeticError::Underflow)
	}
}
