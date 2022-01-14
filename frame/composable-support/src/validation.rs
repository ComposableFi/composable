use core::marker::PhantomData;
use scale_info::TypeInfo;

/// Black box that embbed the validated value.
#[derive(Eq, PartialEq, Debug, TypeInfo, codec::Encode)]
pub struct Validated<T, U> {
	pub value: T,
	_marker: PhantomData<U>,
}

pub trait Validate<U> {
	fn validate(&self) -> Result<(), &'static str>;
}

impl<T: codec::Decode + Validate<U>, U> codec::Decode for Validated<T, U> {
	fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
		let value = T::decode(input)?;
		Validate::<U>::validate(&value).map_err(|desc| Into::<codec::Error>::into(desc))?;
		Ok(Validated { value, _marker: PhantomData })
	}
	fn skip<I: codec::Input>(input: &mut I) -> Result<(), codec::Error> {
		T::skip(input)
	}
}

impl<T: codec::Encode + codec::Decode + Validate<U>, U> codec::EncodeLike<T> for Validated<T, U> {}

#[cfg(test)]
mod test {
	use super::*;
	use codec::{Decode, Encode};

	#[derive(Debug, Eq, PartialEq, codec::Encode, codec::Decode)]
	struct X {
		a: u32,
		b: u32,
	}

	#[derive(Debug, Eq, PartialEq)]
	struct ValidateARange;

	impl Validate<ValidateARange> for X {
		fn validate(&self) -> Result<(), &'static str> {
			if self.a > 10 {
				Err("Out of range")
			} else {
				Ok(())
			}
		}
	}

	#[derive(Debug, Eq, PartialEq)]
	struct ValidateBRange;

	impl Validate<ValidateBRange> for X {
		fn validate(&self) -> Result<(), &'static str> {
			if self.b > 10 {
				Err("Out of range")
			} else {
				Ok(())
			}
		}
	}

	#[test]
	fn test_valid_a() {
		let valid = X { a: 10, b: 0xCAFEBABE };
		let bytes = valid.encode();
		assert_eq!(
			Ok(Validated { value: valid, _marker: PhantomData }),
			Validated::<X, ValidateARange>::decode(&mut &bytes[..])
		);
	}

	#[test]
	fn test_invalid_a() {
		let invalid = X { a: 0xDEADC0DE, b: 0xCAFEBABE };
		let bytes = invalid.encode();
		assert!(Validated::<X, ValidateARange>::decode(&mut &bytes[..]).is_err());
	}

	#[test]
	fn test_valid_b() {
		let valid = X { a: 0xCAFEBABE, b: 10 };
		let bytes = valid.encode();
		assert_eq!(
			Ok(Validated { value: valid, _marker: PhantomData }),
			Validated::<X, ValidateBRange>::decode(&mut &bytes[..])
		);
	}

	#[test]
	fn test_invalid_b() {
		let invalid = X { a: 0xCAFEBABE, b: 0xDEADC0DE };
		let bytes = invalid.encode();
		assert!(Validated::<X, ValidateBRange>::decode(&mut &bytes[..]).is_err());
	}
}
