use core::{marker::PhantomData, ops::Deref};
use scale_info::TypeInfo;

/// Black box that embbed the validated value.
#[derive(Default, Copy, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct Validated<T, U> {
	value: T,
	_marker: PhantomData<U>,
}

impl<T: Copy, U> Validated<T, U> {
	#[inline(always)]
	pub fn value(&self) -> T {
		self.value
	}
}

impl<T, U> Deref for Validated<T, U> {
	type Target = T;
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.value
	}
}

impl<T, U> AsRef<T> for Validated<T, U> {
	#[inline(always)]
	fn as_ref(&self) -> &T {
		&self.value
	}
}

pub trait Validate<U>: Sized {
	fn validate(self) -> Result<Self, &'static str>;
}

#[derive(Debug, Eq, PartialEq)]
pub struct QED;

impl<T> Validate<QED> for T {
	#[inline(always)]
	fn validate(self) -> Result<Self, &'static str> {
		Ok(self)
	}
}

impl<T: Validate<U> + Validate<V>, U, V> Validate<(U, V)> for T {
	#[inline(always)]
	fn validate(self) -> Result<Self, &'static str> {
		let value = Validate::<U>::validate(self)?;
		let value = Validate::<V>::validate(value)?;
		Ok(value)
	}
}

// as per substrate pattern and existing macroses for similar purposes, they tend to make things flat
impl<T: Validate<U> + Validate<V> + Validate<W>, U, V, W> Validate<(U, V, W)> for T {
	#[inline(always)]
	fn validate(self) -> Result<Self, &'static str> {
		let value = Validate::<U>::validate(self)?;
		let value = Validate::<V>::validate(value)?;
		let value = Validate::<W>::validate(value)?;
		Ok(value)
	}
}

impl<T: codec::Decode + Validate<(U, V)>, U, V> codec::Decode for Validated<T, (U, V)> {
	fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
		let value = Validate::validate(T::decode(input)?)
			.map_err(|desc| Into::<codec::Error>::into(desc))?;
		Ok(Validated { value, _marker: PhantomData })
	}
	fn skip<I: codec::Input>(input: &mut I) -> Result<(), codec::Error> {
		T::skip(input)
	}
}

impl<T: codec::Encode + codec::Decode + Validate<U>, U> codec::WrapperTypeEncode
	for Validated<T, U>
{
}

#[cfg(test)]
mod test {
	use super::*;
	use codec::{Decode, Encode};

	#[derive(Debug, Eq, PartialEq)]
	struct ValidARange;
	#[derive(Debug, Eq, PartialEq)]
	struct ValidBRange;

	type CheckARange = (ValidARange, QED);
	type CheckBRange = (ValidBRange, QED);
	type CheckABRange = (ValidARange, (ValidBRange, QED));

	#[derive(Debug, Eq, PartialEq, codec::Encode, codec::Decode)]
	struct X {
		a: u32,
		b: u32,
	}

	impl Validate<ValidARange> for X {
		fn validate(self) -> Result<X, &'static str> {
			if self.a > 10 {
				Err("Out of range")
			} else {
				Ok(self)
			}
		}
	}

	impl Validate<ValidBRange> for X {
		fn validate(self) -> Result<X, &'static str> {
			if self.b > 10 {
				Err("Out of range")
			} else {
				Ok(self)
			}
		}
	}

	#[test]
	fn test_valid_a() {
		let valid = X { a: 10, b: 0xCAFEBABE };
		let bytes = valid.encode();
		assert_eq!(
			Ok(Validated { value: valid, _marker: PhantomData }),
			Validated::<X, CheckARange>::decode(&mut &bytes[..])
		);
	}

	#[test]
	fn test_invalid_a() {
		let invalid = X { a: 0xDEADC0DE, b: 0xCAFEBABE };
		let bytes = invalid.encode();
		assert!(Validated::<X, CheckARange>::decode(&mut &bytes[..]).is_err());
	}

	#[test]
	fn test_valid_b() {
		let valid = X { a: 0xCAFEBABE, b: 10 };
		let bytes = valid.encode();
		assert_eq!(
			Ok(Validated { value: valid, _marker: PhantomData }),
			Validated::<X, CheckBRange>::decode(&mut &bytes[..])
		);
	}

	#[test]
	fn test_invalid_b() {
		let invalid = X { a: 0xCAFEBABE, b: 0xDEADC0DE };
		let bytes = invalid.encode();
		assert!(Validated::<X, CheckBRange>::decode(&mut &bytes[..]).is_err());
	}

	#[test]
	fn test_valid_ab() {
		let valid = X { a: 10, b: 10 };
		let bytes = valid.encode();
		assert_eq!(
			Ok(Validated { value: valid, _marker: PhantomData }),
			Validated::<X, CheckABRange>::decode(&mut &bytes[..])
		);
	}

	#[test]
	fn test_invalid_ab() {
		let invalid = X { a: 0xDEADC0DE, b: 0xCAFEBABE };
		let bytes = invalid.encode();
		assert!(Validated::<X, CheckABRange>::decode(&mut &bytes[..]).is_err());
	}

	#[test]
	fn test_invalid_a_ab() {
		let invalid = X { a: 0xDEADC0DE, b: 10 };
		let bytes = invalid.encode();
		assert!(Validated::<X, CheckABRange>::decode(&mut &bytes[..]).is_err());
	}

	#[test]
	fn test_invalid_b_ab() {
		let invalid = X { a: 10, b: 0xDEADC0DE };
		let bytes = invalid.encode();
		assert!(Validated::<X, CheckABRange>::decode(&mut &bytes[..]).is_err());
	}
}
