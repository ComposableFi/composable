use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Signed, Unsigned, Zero};
use sp_arithmetic::{biguint, helpers_128bit::to_big_uint};
use sp_runtime::{
	traits::UniqueSaturatedInto,
	ArithmeticError,
	ArithmeticError::{DivisionByZero, Overflow, Underflow},
	FixedPointNumber, FixedPointOperand,
};

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
		let error = match self.is_negative() ^ other.is_negative() {
			true => Underflow,
			false => Overflow,
		};

		let accuracy = Self::DIV.unique_saturated_into();
		let q = multiply_by_rational(lhs.value, rhs.value, accuracy).map_err(|_| error)?;
		Ok(Self::from_inner(from_i129(I129 { value: q, negative }).ok_or(error)?))
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
		let error = match self.is_negative() ^ other.is_negative() {
			true => Underflow,
			false => Overflow,
		};

		let lhs: I129 = self.into_inner().into();
		let rhs: I129 = other.into_inner().into();
		let negative = lhs.negative != rhs.negative;

		let accuracy = Self::DIV.unique_saturated_into();
		let q = multiply_by_rational(lhs.value, accuracy, rhs.value).map_err(|_| error)?;
		Ok(Self::from_inner(from_i129(I129 { value: q, negative }).ok_or(error)?))
	}

	fn try_div_rem(&self, other: &Self) -> Result<(Self, Self), ArithmeticError> {
		if other.into_inner().is_zero() {
			return Err(DivisionByZero)
		}

		let lhs: I129 = self.into_inner().into();
		let rhs: I129 = other.into_inner().into();
		let negative = lhs.negative != rhs.negative;
		let error = match self.is_negative() ^ other.is_negative() {
			true => Underflow,
			false => Overflow,
		};

		let accuracy = Self::DIV.unique_saturated_into();
		let (q, r) = div_rem_with_acc(lhs.value, rhs.value, accuracy).map_err(|_| error)?;
		Ok((
			Self::from_inner(from_i129(I129 { value: q, negative }).ok_or(error)?),
			Self::from_inner(from_i129(I129 { value: r, negative }).ok_or(error)?),
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

/// Modification of [`sp_arithmetic::helpers_128bits::multiply_by_rational`] that does not modify
/// the quotient of the last division
fn multiply_by_rational(mut a: u128, mut b: u128, mut c: u128) -> Result<u128, &'static str> {
	if a.is_zero() || b.is_zero() {
		return Ok(0)
	}
	c = c.max(1);

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
		let a_num = to_big_uint(a);
		let b_num = to_big_uint(b);
		let c_num = to_big_uint(c);

		let mut ab = a_num * b_num;
		ab.lstrip();
		let mut q = if c_num.len() == 1 {
			// PROOF: if `c_num.len() == 1` then `c` fits in one limb.
			// TODO(0xangelo): verify that the remainder for this type of division is indeed 0
			ab.div_unit(c as biguint::Single)
		} else {
			// PROOF: both `ab` and `c` cannot have leading zero limbs; if length of `c` is 1,
			// the previous branch would handle. Also, if ab for sure has a bigger size than
			// c, because `a.checked_mul(b)` has failed, hence ab must be at least one limb
			// bigger than c. In this case, returning zero is defensive-only and div should
			// always return Some.
			let (q, _) = ab.div(&c_num, false).unwrap_or((Zero::zero(), Zero::zero()));
			q
		};
		q.lstrip();
		Ok(q.try_into().map_err(|_| "result cannot fit in u128")?)
	}
}

/// Fixed point division with remainder using the underlying integers (`a` and `b`) and the
/// precision (`acc`) of the fixed point implementation.
fn div_rem_with_acc(mut a: u128, mut b: u128, mut acc: u128) -> Result<(u128, u128), &'static str> {
	if a.is_zero() || acc.is_zero() {
		return Ok((Zero::zero(), Zero::zero()))
	}
	b = b.max(1);

	// a and acc are interchangeable by definition in this function. It always helps to assume the
	// bigger of which is being multiplied by a `0 < acc/b < 1`. Hence, a should be the bigger and
	// acc the smaller one.
	if acc > a {
		sp_std::mem::swap(&mut a, &mut acc);
	}

	// Attempt to perform the division first
	if a % b == 0 {
		a /= b;
		b = 1;
	} else if acc % b == 0 {
		acc /= b;
		b = 1;
	}

	if let Some(x) = a.checked_mul(acc) {
		// This is the safest way to go. Try it.
		let q = x / b;
		Ok((q, a - (b * q) / acc))
	} else {
		// [`to_big_uint`] strips leading zeroes
		let a_num = to_big_uint(a); // a limbs
		let b_num = to_big_uint(b); // b limbs
		let acc_num = to_big_uint(acc); // c limbs

		let mut aa = a_num.mul(&acc_num); // a + c limbs
		aa.lstrip(); // b,c < aa <= a + c limbs, since a.checked_mul(acc) failed
		let (mut q, r) = if b_num.len() == 1 {
			// PROOF: if `b_num.len() == 1` then `b` fits in one limb.
			// TODO(0xangelo): verify that the remainder for this type of division is indeed 0
			(aa.div_unit(b as biguint::Single), Zero::zero())
		} else {
			// 1 < b limbs
			// PROOF: both `aa` and `b` cannot have leading zero limbs; if length of `b` is 1,
			// the previous branch would handle. Also, if `aa` for sure has a bigger size than
			// `b`, because `a.checked_mul(acc)` has failed, hence `aa` must be at least one limb
			// bigger than `b`. In this case, returning zero is defensive-only and div should
			// always return Some.
			let (q, _) = aa.div(&b_num, false).unwrap_or((Zero::zero(), Zero::zero()));
			//   ^ q = aa - b + 1 >= 2 limbs
			// We can't project the remainder of `aa.div` above back to the original accuracy
			// because:
			// q = (a * acc) // b
			// r = (a * acc) % b
			// a = (q * b + r) // acc >= (q * b) // acc + r // acc
			//
			// So we first compute `b * q // acc` (what we would get by multiplying the underlying
			// fixed point numbers)
			let mut bq = b_num.mul(&q); // bq = b + q = aa + 1 > b + 1, c + 1 limbs
			bq.lstrip(); // ?
			let a_: u128 = if acc_num.len() == 1 {
				bq.div_unit(acc as biguint::Single)
			} else {
				// 1 < c limbs
				bq.div(&acc_num, false)
					.expect(
						"Both `bq` and `acc_num` are stripped. \
							 If `acc_num` is single-limbed, the previous branch would handle it. \
							 If `bq` isn't at least one limb bigger than `acc_num`, then `bq` is \
							 at least one limb smaller than `aa`. But then ",
					)
					.0
			}
			.try_into()
			.expect("multiplication by quotient is less or equal than original value; qed");
			// Then we compute the residual
			let r = a
				.checked_sub(a_)
				.expect("remainder of `a / _` is always less than or equal to a; qed");

			(q, r)
		};
		q.lstrip();
		Ok((q.try_into().map_err(|_| "result cannot fit in u128")?, r))
	}
}
