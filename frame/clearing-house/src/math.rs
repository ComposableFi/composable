use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Signed, Unsigned};
use sp_runtime::{
	ArithmeticError,
	ArithmeticError::{DivisionByZero, Overflow, Underflow},
	FixedPointNumber,
};

pub trait UnsignedMath: CheckedAdd + CheckedDiv + CheckedMul + CheckedSub + Unsigned {
	fn try_add(&self, other: &Self) -> Result<Self, ArithmeticError>;

	fn try_sub(&self, other: &Self) -> Result<Self, ArithmeticError>;

	fn try_mul(&self, other: &Self) -> Result<Self, ArithmeticError>;

	fn try_div(&self, other: &Self) -> Result<Self, ArithmeticError>;

	fn try_add_mut(&mut self, other: &Self) -> Result<(), ArithmeticError> {
		*self = self.try_add(other)?;
		Ok(())
	}

	fn try_sub_mut(&mut self, other: &Self) -> Result<(), ArithmeticError> {
		*self = self.try_sub(other)?;
		Ok(())
	}

	fn try_mul_mut(&mut self, other: &Self) -> Result<(), ArithmeticError> {
		*self = self.try_mul(other)?;
		Ok(())
	}

	fn try_div_mut(&mut self, other: &Self) -> Result<(), ArithmeticError> {
		*self = self.try_div(other)?;
		Ok(())
	}
}

impl<T> UnsignedMath for T
where
	T: CheckedAdd + CheckedDiv + CheckedMul + CheckedSub + Unsigned,
{
	fn try_add(&self, other: &Self) -> Result<Self, ArithmeticError> {
		self.checked_add(other).ok_or(Overflow)
	}

	fn try_sub(&self, other: &Self) -> Result<Self, ArithmeticError> {
		self.checked_sub(other).ok_or(Underflow)
	}

	fn try_mul(&self, other: &Self) -> Result<Self, ArithmeticError> {
		self.checked_mul(other).ok_or(Overflow)
	}

	fn try_div(&self, other: &Self) -> Result<Self, ArithmeticError> {
		self.checked_div(other).ok_or(DivisionByZero)
	}
}

pub trait FixedPointMath: FixedPointNumber {
	fn try_add(&self, other: &Self) -> Result<Self, ArithmeticError>;

	fn try_sub(&self, other: &Self) -> Result<Self, ArithmeticError>;

	fn try_mul(&self, other: &Self) -> Result<Self, ArithmeticError>;

	fn try_div(&self, other: &Self) -> Result<Self, ArithmeticError>;

	fn try_add_mut(&mut self, other: &Self) -> Result<(), ArithmeticError> {
		*self = self.try_add(other)?;
		Ok(())
	}

	fn try_sub_mut(&mut self, other: &Self) -> Result<(), ArithmeticError> {
		*self = self.try_sub(other)?;
		Ok(())
	}

	fn try_mul_mut(&mut self, other: &Self) -> Result<(), ArithmeticError> {
		*self = self.try_mul(other)?;
		Ok(())
	}

	fn try_div_mut(&mut self, other: &Self) -> Result<(), ArithmeticError> {
		*self = self.try_div(other)?;
		Ok(())
	}
}

impl<T: FixedPointNumber> FixedPointMath for T {
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

pub trait IntoBalance<Balance> {
	fn into_balance(self) -> Result<Balance, ArithmeticError>;
}

impl<B, D> IntoBalance<B> for D
where
	B: Unsigned,
	D: FixedPointNumber,
	D::Inner: Signed + TryInto<B>,
{
	fn into_balance(self) -> Result<B, ArithmeticError> {
		self.into_inner().abs().try_into().map_err(|_| Overflow)
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

pub trait IntoDecimal<D> {
	fn into_decimal(self) -> Result<D, ArithmeticError>;
}

impl<B, D> IntoDecimal<D> for B
where
	D: FromBalance<B>,
{
	fn into_decimal(self) -> Result<D, ArithmeticError> {
		D::from_balance(self)
	}
}

pub trait FromUnsigned<U>: Sized {
	fn from_unsigned(value: U) -> Result<Self, ArithmeticError>;
}

pub trait IntoSigned<S> {
	fn into_signed(self) -> Result<S, ArithmeticError>;
}

impl<S, U> FromUnsigned<U> for S
where
	S: FixedPointNumber,
	S::Inner: Signed,
	U: FixedPointNumber,
	U::Inner: TryInto<S::Inner> + Unsigned,
{
	fn from_unsigned(value: U) -> Result<Self, ArithmeticError> {
		Ok(Self::from_inner(value.into_inner().try_into().map_err(|_| Overflow)?))
	}
}

impl<S, U> IntoSigned<S> for U
where
	S: FromUnsigned<U>,
{
	fn into_signed(self) -> Result<S, ArithmeticError> {
		S::from_unsigned(self)
	}
}
