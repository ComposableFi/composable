use num_traits::{Signed, Unsigned};
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

// The following types are markers to disambiguate the two implementations of IntoBalance below.
// Since traits can overlap, the 'where' clauses below are not enough to tell the compiler which
// implementation of the trait it should use for a particular type
pub struct Int;
pub struct FixedPoint;

pub trait IntoBalance<Balance, Marker> {
	fn into_balance(self) -> Result<Balance, ArithmeticError>;
}

impl<B, I> IntoBalance<B, Int> for I
where
	B: Unsigned,
	I: Signed + TryInto<B>,
{
	fn into_balance(self) -> Result<B, ArithmeticError> {
		// Absolute value can still overflow if B's capacity is less than I's
		self.abs().try_into().map_err(|_| Overflow)
	}
}

impl<B, D> IntoBalance<B, FixedPoint> for D
where
	B: Unsigned,
	D: FixedPointNumber,
	D::Inner: IntoBalance<B, Int>,
{
	fn into_balance(self) -> Result<B, ArithmeticError> {
		self.into_inner().into_balance()
	}
}

pub trait FromBalance<Balance>: Sized {
	fn from_balance(value: Balance) -> Result<Self, ArithmeticError>;
}

impl<B, D> FromBalance<B> for D
where
	B: Unsigned,
	D: FixedPointNumber,
	D::Inner: TryFrom<B>,
{
	fn from_balance(value: B) -> Result<Self, ArithmeticError> {
		Ok(Self::from_inner(value.try_into().map_err(|_| Overflow)?))
	}
}
