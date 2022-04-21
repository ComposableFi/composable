use crate::Config;
use num_traits::Signed;
use sp_runtime::{
	ArithmeticError,
	ArithmeticError::{DivisionByZero, Overflow, Underflow},
	FixedPointNumber,
};

pub trait TryMath: Sized {
	fn try_add(&self, other: &Self) -> Result<Self, ArithmeticError>;

	fn try_sub(&self, other: &Self) -> Result<Self, ArithmeticError>;

	fn try_mul(&self, other: &Self) -> Result<Self, ArithmeticError>;

	fn try_div(&self, other: &Self) -> Result<Self, ArithmeticError>;

	fn try_add_(&mut self, other: &Self) -> Result<(), ArithmeticError> {
		*self = self.try_add(other)?;
		Ok(())
	}

	fn try_sub_(&mut self, other: &Self) -> Result<(), ArithmeticError> {
		*self = self.try_sub(other)?;
		Ok(())
	}

	fn try_mul_(&mut self, other: &Self) -> Result<(), ArithmeticError> {
		*self = self.try_mul(other)?;
		Ok(())
	}

	fn try_div_(&mut self, other: &Self) -> Result<(), ArithmeticError> {
		*self = self.try_div(other)?;
		Ok(())
	}
}

impl<T> TryMath for T
where
	T: FixedPointNumber,
{
	fn try_add(&self, other: &Self) -> Result<Self, ArithmeticError> {
		// sign(a) sign(other) | CheckedAdd
		// ----------------------------
		//      -1      -1 | Underflow
		//      -1       1 | Ok
		//       1      -1 | Ok
		//       1       1 | Overflow
		self.checked_add(other).ok_or_else(|| match self.is_positive() {
			true => Overflow,
			false => Underflow,
		})
	}

	fn try_sub(&self, other: &Self) -> Result<Self, ArithmeticError> {
		// sign(a) sign(other) | CheckedSub
		// ----------------------------
		//      -1      -1 | Ok
		//      -1       1 | Underflow
		//       1      -1 | Overflow
		//       1       1 | Ok
		self.checked_sub(other).ok_or_else(|| match self.is_positive() {
			true => Overflow,
			false => Underflow,
		})
	}

	fn try_mul(&self, other: &Self) -> Result<Self, ArithmeticError> {
		// sign(a) sign(other) | CheckedMul
		// ----------------------------
		//      -1      -1 | Overflow
		//      -1       1 | Underflow
		//       1      -1 | Underflow
		//       1       1 | Overflow
		self.checked_mul(other)
			.ok_or_else(|| match self.is_negative() ^ other.is_negative() {
				true => Underflow,
				false => Overflow,
			})
	}

	fn try_div(&self, other: &Self) -> Result<Self, ArithmeticError> {
		// sign(a) sign(other) | CheckedDiv
		// ----------------------------
		//      -1      -1 | Overflow
		//      -1       1 | Underflow
		//       1      -1 | Underflow
		//       1       1 | Overflow
		self.checked_div(other).ok_or_else(|| {
			if other.is_zero() {
				DivisionByZero
			} else {
				match self.is_negative() ^ other.is_negative() {
					true => Underflow,
					false => Overflow,
				}
			}
		})
	}
}

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
