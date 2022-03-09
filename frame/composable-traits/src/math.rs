use sp_runtime::{
	helpers_128bit::multiply_by_rational,
	traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, One, Saturating, Zero},
	ArithmeticError,
};

/// Thin convenient wrapper to lift the Result in a Dispatch context.
pub fn safe_multiply_by_rational(a: u128, b: u128, c: u128) -> Result<u128, ArithmeticError> {
	multiply_by_rational(a, b, c).map_err(|_| ArithmeticError::Overflow)
}

/// little bit slower than maximizing performance by knowing constraints.
/// Example, you sum to negative numbers, can get underflow, so need to check on each add; but if
/// you have positive number only, you cannot have underflow. Same for other constrains, like non
/// zero divisor.

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

impl<T: SafeAdd + SafeDiv + SafeMul + SafeSub> SafeArithmetic for T {}

/// An object from which we can derive a second object of the same type.
/// This function cannot fail and might return the same object if a boundary is about to be crossed.
// This kind of function is usually called an Endomorphism. But let's keep it simple.
pub trait WrappingNext {
	/// pallet must be coded that way that wrapping around does not do harm except of error
	/// so additional check should be check on pallet level
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
