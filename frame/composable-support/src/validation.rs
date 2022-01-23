use core::{marker::PhantomData, ops::Deref};
use scale_info::TypeInfo;
use sp_runtime::DispatchError;

/// Black box that embbed the validated value.
#[derive(Default, Copy, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct Validated<T, U> {
	value: T,
	_marker: PhantomData<U>,
}

impl<T, U> Validated<T, U> {
	#[inline(always)]
	pub fn value(self) -> T {
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


pub trait ValidateDispatch<U> : Sized {
	fn validate(self) -> Result<Self, DispatchError>;
}

pub trait Validate<U>: Sized {
	// use string here because in serde layer there is not dispatch
	fn validate(self) -> Result<Self, &'static str>;
}

#[derive(Debug, Eq, PartialEq)]
pub enum Valid {}

#[derive(Debug, Eq, PartialEq)]
pub enum Invalid {}


impl<T> Validate<Invalid> for T {
	#[inline(always)]
	fn validate(self) -> Result<Self, &'static str> {
		Err("not valid")
	}
}

impl<T> Validate<Valid> for T {
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
// like `#[impl_trait_for_tuples::impl_for_tuples(30)]`
// so if we will need more than 3, can consider it
impl<T: Validate<U> + Validate<V> + Validate<W>, U, V, W> Validate<(U, V, W)> for T {
	#[inline(always)]
	fn validate(self) -> Result<Self, &'static str> {
		let value = Validate::<U>::validate(self)?;
		let value = Validate::<V>::validate(value)?;
		let value = Validate::<W>::validate(value)?;
		Ok(value)
	}
}

impl<T: codec::Decode + Validate<U>, U> codec::Decode for Validated<T, U> {
	fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
		let value = Validate::validate(T::decode(input)?).map_err(Into::<codec::Error>::into)?;
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

	type CheckARange = (ValidARange, Valid);
	type CheckBRange = (ValidBRange, Valid);
	type CheckABRange = (ValidARange, (ValidBRange, Valid));

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
	fn encode_decode_validated_encode_decode() {
		let original = X { a: 0xDEADC0DE, b: 0xCAFEBABE };
		let bytes = original.encode();
		let wrapped = Validated::<X, Valid>::decode(&mut &bytes[..]).unwrap();
		let bytes = wrapped.encode();
		let reencoded = X::decode(&mut &bytes[..]).unwrap();
		assert_eq!(reencoded, original);
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

	#[test]
	fn valid_triple() {
		let value = X { a: 10, b: 0xDEADC0DE };
		let bytes = value.encode();
		assert_eq!(
			Ok(Validated { value, _marker: PhantomData }),
			Validated::<X, (Valid, Valid, Valid)>::decode(&mut &bytes[..])
		);
	}

	#[test]
	fn valid_invalid_valid() {
		let value = X { a: 10, b: 0xDEADC0DE };
		let bytes = value.encode();
		assert!(Validated::<X, (Valid, Invalid, Valid)>::decode(&mut &bytes[..]).is_err());
	}
}
