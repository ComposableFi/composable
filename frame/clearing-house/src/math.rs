use num_integer::Integer;
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Signed, Unsigned, Zero};
use sp_core::U256;
use sp_runtime::{
	traits::UniqueSaturatedInto,
	ArithmeticError,
	ArithmeticError::{DivisionByZero, Overflow, Underflow},
	FixedPointNumber, FixedPointOperand,
};
use sp_std::cmp;

// -------------------------------------------------------------------------------------------------
//                                          Functions
// -------------------------------------------------------------------------------------------------

/// Clips `value` between `lower` and `upper` (inclusive).
pub fn clip<T: FixedPointNumber>(value: T, lower: T, upper: T) -> T {
	cmp::min(cmp::max(lower, value), upper)
}

/// Computes a weighted average as `(a * wa + b * wb) / (wa + wb)`.
pub fn weighted_average<T>(a: &T, b: &T, weight_a: &T, weight_b: &T) -> Result<T, ArithmeticError>
where
	T: FixedPointNumber,
{
	a.try_mul(weight_a)?
		.try_add(&b.try_mul(weight_b)?)?
		.try_div(&weight_a.try_add(weight_b)?)
}

// -------------------------------------------------------------------------------------------------
//                                             Traits
// -------------------------------------------------------------------------------------------------

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

pub trait FixedPointMath: FixedPointNumber {
	/// Like [`FixedPointNumber::checked_add`], but returning a [`Result`] with [`ArithmeticError`]
	/// in case of failures
	fn try_add(&self, other: &Self) -> Result<Self, ArithmeticError>;

	/// Like [`FixedPointNumber::checked_sub`], but returning a [`Result`] with [`ArithmeticError`]
	/// in case of failures
	fn try_sub(&self, other: &Self) -> Result<Self, ArithmeticError>;

	/// Like [`FixedPointNumber::checked_mul`], but:
	/// - with flooring instead of rounding in the final division by accuracy
	/// - returning a [`Result`] with [`ArithmeticError`] in case of failures
	fn try_mul(&self, other: &Self) -> Result<Self, ArithmeticError>;

	/// Like [`FixedPointNumber::checked_div`], but:
	/// - with flooring instead of rounding of the quotient
	/// - returning a [`Result`] with [`ArithmeticError`] in case of failures
	fn try_div(&self, other: &Self) -> Result<Self, ArithmeticError>;

	/// Like [`FixedPointMath::try_div`], but returning a tuple with both the quotient and remainder
	/// respectively.
	fn try_div_rem(&self, other: &Self) -> Result<(Self, Self), ArithmeticError>;

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

pub trait IntoBalance<Balance> {
	fn into_balance(self) -> Result<Balance, ArithmeticError>;
}

pub trait FromBalance<Balance>: Sized {
	fn from_balance(value: Balance) -> Result<Self, ArithmeticError>;
}

pub trait IntoDecimal<D> {
	fn into_decimal(self) -> Result<D, ArithmeticError>;
}

pub trait FromUnsigned<U>: Sized {
	fn from_unsigned(value: U) -> Result<Self, ArithmeticError>;
}

pub trait IntoSigned<S> {
	fn into_signed(self) -> Result<S, ArithmeticError>;
}

// -------------------------------------------------------------------------------------------------
//                                              Impls
// -------------------------------------------------------------------------------------------------

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

impl<T: FixedPointNumber> FixedPointMath for T {
	fn try_add(&self, other: &Self) -> Result<Self, ArithmeticError> {
		// sign(a) sign(other) | CheckedAdd
		// --------------------------------
		//      -1          -1 | Underflow
		//      -1           1 | Ok
		//       1          -1 | Ok
		//       1           1 | Overflow
		self.checked_add(other).ok_or_else(|| match self.is_positive() {
			true => Overflow,
			false => Underflow,
		})
	}

	fn try_sub(&self, other: &Self) -> Result<Self, ArithmeticError> {
		// sign(a) sign(other) | CheckedSub
		// --------------------------------
		//      -1          -1 | Ok
		//      -1           1 | Underflow
		//       1          -1 | Overflow
		//       1           1 | Ok
		self.checked_sub(other).ok_or_else(|| match self.is_positive() {
			true => Overflow,
			false => Underflow,
		})
	}

	fn try_mul(&self, other: &Self) -> Result<Self, ArithmeticError> {
		let lhs: I129 = self.into_inner().into();
		let rhs: I129 = other.into_inner().into();
		let negative = lhs.negative != rhs.negative;
		// sign(a) sign(other) | CheckedMul
		// --------------------------------
		//      -1          -1 | Overflow
		//      -1           1 | Underflow
		//       1          -1 | Underflow
		//       1           1 | Overflow
		let error = || match self.is_negative() ^ other.is_negative() {
			true => Underflow,
			false => Overflow,
		};

		let accuracy = Self::DIV.unique_saturated_into();
		let q = multiply_by_rational(lhs.value, rhs.value, accuracy).map_err(|_| error())?;
		Ok(Self::from_inner(from_i129(I129 { value: q, negative }).ok_or_else(error)?))
	}

	fn try_div(&self, other: &Self) -> Result<Self, ArithmeticError> {
		// sign(a) sign(other) | CheckedDiv
		// --------------------------------
		//      -1          -1 | Overflow
		//      -1           1 | Underflow
		//       1          -1 | Underflow
		//       1           1 | Overflow
		//    -1/1           0 | DivisionByZero
		if other.into_inner().is_zero() {
			return Err(DivisionByZero)
		}
		let error = || match self.is_negative() ^ other.is_negative() {
			true => Underflow,
			false => Overflow,
		};

		let lhs: I129 = self.into_inner().into();
		let rhs: I129 = other.into_inner().into();
		let negative = lhs.negative != rhs.negative;

		let accuracy = Self::DIV.unique_saturated_into();
		let q = multiply_by_rational(lhs.value, accuracy, rhs.value).map_err(|_| error())?;
		Ok(Self::from_inner(from_i129(I129 { value: q, negative }).ok_or_else(error)?))
	}

	fn try_div_rem(&self, other: &Self) -> Result<(Self, Self), ArithmeticError> {
		if other.into_inner().is_zero() {
			return Err(DivisionByZero)
		}

		let lhs: I129 = self.into_inner().into();
		let rhs: I129 = other.into_inner().into();
		let negative = lhs.negative != rhs.negative;
		let error = || match self.is_negative() ^ other.is_negative() {
			true => Underflow,
			false => Overflow,
		};

		let accuracy = Self::DIV.unique_saturated_into();
		let (q, r) = div_mod_with_acc(lhs.value, rhs.value, accuracy).map_err(|_| error())?;
		Ok((
			Self::from_inner(from_i129(I129 { value: q, negative }).ok_or_else(error)?),
			Self::from_inner(from_i129(I129 { value: r, negative }).ok_or_else(error)?),
		))
	}
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

impl<B, D> IntoDecimal<D> for B
where
	D: FromBalance<B>,
{
	fn into_decimal(self) -> Result<D, ArithmeticError> {
		D::from_balance(self)
	}
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

// -------------------------------------------------------------------------------------------------
//                                             Helpers
// -------------------------------------------------------------------------------------------------

/// Copied from [`sp_arithmetic::fixed_point`]:
///
/// Data type used as intermediate storage in some computations to avoid overflow.
struct I129 {
	value: u128,
	negative: bool,
}

impl<N: FixedPointOperand> From<N> for I129 {
	fn from(n: N) -> I129 {
		if n < N::zero() {
			let value: u128 = n
				.checked_neg()
				.map(|n| n.unique_saturated_into())
				.unwrap_or_else(|| N::max_value().unique_saturated_into().saturating_add(1));
			I129 { value, negative: true }
		} else {
			I129 { value: n.unique_saturated_into(), negative: false }
		}
	}
}

/// Copied from [`sp_arithmetic::fixed_point`]:
///
/// Transforms an `I129` to `N` if it is possible.
fn from_i129<N: FixedPointOperand>(n: I129) -> Option<N> {
	let max_plus_one: u128 = N::max_value().unique_saturated_into().saturating_add(1);
	if n.negative && N::min_value() < N::zero() && n.value == max_plus_one {
		Some(N::min_value())
	} else {
		let unsigned_inner: N = n.value.try_into().ok()?;
		let inner = if n.negative { unsigned_inner.checked_neg()? } else { unsigned_inner };
		Some(inner)
	}
}

/// Alterative to [`sp_arithmetic::helpers_128bits::multiply_by_rational`] that does not modify
/// the quotient of the last division and uses U256 as a backend if necessary
pub fn multiply_by_rational(
	mut a: u128,
	mut b: u128,
	mut c: u128,
) -> Result<u128, ArithmeticError> {
	if a.is_zero() || b.is_zero() {
		return Ok(0)
	}
	if c.is_zero() {
		return Err(DivisionByZero)
	}

	// a and b are interchangeable by definition in this function. It always helps to assume the
	// bigger of which is being multiplied by a `0 < b/c < 1`. Hence, a should be the bigger and
	// b the smaller one.
	if b > a {
		sp_std::mem::swap(&mut a, &mut b);
	}

	// Attempt to perform the division first
	if a % c == 0 {
		a /= c;
		c = 1;
	} else if b % c == 0 {
		b /= c;
		c = 1;
	}

	if let Some(x) = a.checked_mul(b) {
		// This is the safest way to go. Try it.
		Ok(x / c)
	} else {
		let a_256: U256 = a.into();
		let b_256: U256 = b.into();
		let c_256: U256 = c.into();

		let ab = a_256 * b_256;
		let q = ab / c_256; // Safe because `c` isnt zero
		Ok(q.try_into().map_err(|_| Overflow)?)
	}
}

/// Fixed point division with remainder using the underlying integers (`a` and `b`) and the
/// precision (`acc`) of the fixed point implementation. Uses [`U256`] in case intermediate
/// operations overflow [`u128`]
pub fn div_mod_with_acc(
	mut a: u128,
	mut b: u128,
	mut acc: u128,
) -> Result<(u128, u128), ArithmeticError> {
	if a.is_zero() || acc.is_zero() {
		return Ok((Zero::zero(), Zero::zero()))
	}
	if b.is_zero() {
		return Err(DivisionByZero)
	}

	// Attempt to perform the division first
	if a % b == 0 {
		a /= b;
		b = 1;
	} else if acc % b == 0 {
		acc /= b;
		b = 1;
	}

	// We can't project the remainder of `aa / b` below back to the original accuracy because:
	// q = (a * acc) // b
	// r = (a * acc) % b
	// a = (q * b + r) // acc >= (q * b) // acc + r // acc
	//
	// So we first compute `a_ = b * q // acc` (what we would get by multiplying the
	// underlying fixed point numbers) then compute the residual `r = a - a_`
	if let Some(aa) = a.checked_mul(acc) {
		// This is the safest way to go. Try it.
		let (q, r) = aa.div_rem(&b);
		Ok((q, a - (aa - r) / acc))
	} else {
		let a_256: U256 = a.into();
		let b_256: U256 = b.into();
		let acc_256: U256 = acc.into();

		let aa = a_256 * acc_256; // Safe since `a` and `acc` were originally `u128`
		let (q, r) = aa.div_mod(b_256); // Safe because `b` is not zero
		let bq = aa - r; // Safe because remainder is less than or equal to the dividend
		let a_: u128 = (bq / acc_256)
			.try_into()
			.expect("quotient times divisor is less than or equal to the dividend; qed");
		Ok((
			q.try_into().map_err(|_| Overflow)?,
			a - a_, // Safe because a = (q * b + r) // acc >= (q * b) // acc
		))
	}
}
