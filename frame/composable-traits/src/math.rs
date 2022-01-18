use sp_runtime::{
	traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, One, Saturating, Zero},
	ArithmeticError, FixedU128,
};

/// Number like of higher bits, so that amount and balance calculations are done it it with higher
/// precision via fixed point.
/// While this is 128 bit, cannot support u128 because 18 bits are for of mantissa (so maximal
/// integer is 110 bit). Can support u128 if lift upper to use FixedU256 analog.
pub type LiftedFixedBalance = FixedU128;

/// little bit slower than maximizing performance by knowing constraints.
/// Example, you sum to negative numbers, can get underflow, so need to check on each add; but if
/// you have positive number only, you cannot have underflow. Same for other constrains, like non
/// zero divisor.
pub trait SafeArithmetic: Sized {
	fn safe_add(&self, rhs: &Self) -> Result<Self, ArithmeticError>;
	fn safe_div(&self, rhs: &Self) -> Result<Self, ArithmeticError>;
	fn safe_mul(&self, rhs: &Self) -> Result<Self, ArithmeticError>;
	fn safe_sub(&self, rhs: &Self) -> Result<Self, ArithmeticError>;
}

impl<T: CheckedAdd + CheckedMul + CheckedDiv + CheckedSub + Zero> SafeArithmetic for T {
	#[inline(always)]
	fn safe_add(&self, rhs: &Self) -> Result<Self, ArithmeticError> {
		self.checked_add(rhs).ok_or(ArithmeticError::Overflow)
	}
	#[inline(always)]
	fn safe_div(&self, rhs: &Self) -> Result<Self, ArithmeticError> {
		if rhs.is_zero() {
			return Err(ArithmeticError::DivisionByZero);
		}

		self.checked_div(rhs).ok_or(ArithmeticError::Overflow)
	}

	#[inline(always)]
	fn safe_mul(&self, rhs: &Self) -> Result<Self, ArithmeticError> {
		self.checked_mul(rhs).ok_or(ArithmeticError::Overflow)
	}

	#[inline(always)]
	fn safe_sub(&self, rhs: &Self) -> Result<Self, ArithmeticError> {
		self.checked_sub(rhs).ok_or(ArithmeticError::Underflow)
	}
}

/// An object from which we can derive a second object of the same type.
/// This function cannot fail and might return the same object if a boundary is about to be crossed.
// This kind of function is usually called an Endomorphism. But let's keep it simple.
pub trait WrappingNext {
	/// pallet must be coded that way that wrapping around does not do harm except of error
	/// so additional check should be check on pallet level
	#[must_use]
	fn next(&self) -> Self;
}

impl<T> WrappingNext for T
where
	T: Copy + One + Saturating,
{
	fn next(&self) -> Self {
		// bug: this should WrappingAdd from num_traits.
		self.saturating_add(T::one())
	}
}
