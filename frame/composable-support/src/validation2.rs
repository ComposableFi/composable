use core::{fmt, marker::PhantomData, ops::Deref};
use scale_info::TypeInfo;
use sp_runtime::DispatchError;

/// Black box that embbed the validated value.
/// Validated during construction or serde.
#[derive(Default)]
pub struct Validated<T, U, E> {
    value: T,
	_marker: PhantomData<(U, E)>,
}

impl<T, U, E> Clone for Validated<T, U, E> 
    where T: Clone,
{
    fn clone(&self) -> Self {
        Self { 
           value: self.value.clone(), 
           _marker: PhantomData::<(U,E)>, 
       } 
    }
}

impl<T, U, E> TypeInfo for Validated<T, U, E>
where
	T: TypeInfo,
{
	type Identity = <T as TypeInfo>::Identity;

	fn type_info() -> scale_info::Type {
		T::type_info()
	}
}

impl<T, U, E> PartialEq for Validated<T, U, E>
where
	T: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.value == other.value
	}
}

impl<T, U, E> Eq for Validated<T, U, E> where T: PartialEq + Eq {}

impl<T, U, E> fmt::Debug for Validated<T, U, E>
where
	T: core::fmt::Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.value.fmt(f)
	}
}

impl<T, U, E> Validated<T, U, E> 
where
    Validated<T, U, E>: Validate<T, U, E>,
	U: Validate<T, U, E>,
{
	pub fn new(value: T) -> Result<Self, E> {
		match <U as Validate<T, U, E>>::validate(value) {
			Ok(value) => Ok(Self { value, _marker: PhantomData}),
			Err(e) => Err(e),
		}
	}
}

impl<T, U, E> Validated<T, U, E> {
	pub fn value(self) -> T {
		self.value
	}
}
pub trait ValidateDispatch<U>: Sized {
	fn validate(self) -> Result<Self, DispatchError>;
}

pub trait Validate<T, U, E> {
	// use string here because in serde layer there is not dispatch
	fn validate(input: T) -> Result<T, E>;
}

pub trait TryIntoValidated<T, E> {
	fn try_into_validated<U: Validate<T, U, E>>(self) -> Result<Validated<T, U, E>, E>;
}

impl<T, E> TryIntoValidated<T, E> for T {
	fn try_into_validated<U: Validate<T, U, E>>(self) -> Result<Validated<T, U, E>, E> {
		Validated::new(self)
	}
}

impl<T, E, U: Validate<T, U, E>> Validate<T, U, E> for Validated<T, U, E> {
	fn validate(input: T) -> Result<T, E> {
		<U as Validate<T, U, E>>::validate(input)
	}
}



impl<T, U, V, E> Validate<T, (U, V), E> for (U, V)
where
	U: Validate<T, U, E>,
	V: Validate<T, V, E>,
{
	#[inline(always)]
	fn validate(input: T) -> Result<T, E> {
		let value = U::validate(input)?;
		let value = V::validate(value)?;
		Ok(value)
	}
}

// as per substrate pattern and existing macroses for similar purposes, they tend to make things
// flat like `#[impl_trait_for_tuples::impl_for_tuples(30)]`
// so if we will need more than 3, can consider it
impl<T, U, V, W, E> Validate<T, (U, V, W), E> for (U, V, W)
where
	U: Validate<T, U, E>,
    V: Validate<T, V, E>,
	W: Validate<T, W, E>,
{
	#[inline(always)]
	fn validate(input: T) -> Result<T, E> {
		let value = U::validate(input)?;
		let value = V::validate(value)?;
		let value = W::validate(value)?;
		Ok(value)
	}
}

impl<T, U, V, W, Z, E> Validate<T, (U, V, W, Z), E> for (U, V, W, Z)
where
	U: Validate<T, U, E>,
	V: Validate<T, V, E>,
	W: Validate<T, W, E>,
	Z: Validate<T, Z, E>,
{
	#[inline(always)]
	fn validate(input: T) -> Result<T, E> {
		let value = U::validate(input)?;
		let value = V::validate(value)?;
		let value = W::validate(value)?;
		let value = Z::validate(value)?;
		Ok(value)
	}
}


impl<T: codec::Decode, U: Validate<T, U, E>, E> codec::Decode for Validated<T, U, E> 
    where E: Into<&'static str>, 
{
	fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        let value = match  <U as Validate<T, U, E>>::validate(T::decode(input)?) {
            Ok(value) => value, 
            Err(error) => return Err(codec::Error::from(error.into())),
        };
		Ok(Validated { value, _marker: PhantomData})
	}
	fn skip<I: codec::Input>(input: &mut I) -> Result<(), codec::Error> {
		T::skip(input)
	}
}


/// Originally there to have `WrapperTypeEncode` work, but now also used in order to prevent
/// .value() calls everywhere
impl<T, U, E> Deref for Validated<T, U, E> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		&self.value
	}
}

impl<T: codec::Encode + codec::Decode, E, U: Validate<T, U, E>> codec::WrapperTypeEncode
	for Validated<T, U, E>
{
}
/*	impl<T,U,E> codec::WrapperTypeDecode for Validated<T,U,E> {
		type Wrapped = T;
	}
*/


#[cfg(test)]
mod test {
	use super::*;
	use codec::{Decode, Encode};
	use frame_support::assert_ok;

	#[derive(Debug, Eq, PartialEq, Default)]
	struct ValidARange;
	#[derive(Debug, Eq, PartialEq, Default)]
	struct ValidBRange;
    #[derive(Debug, Eq, PartialEq, Default, Decode, Encode)]
    pub struct Valid;

    #[derive(Debug, Eq, PartialEq, Default, Decode, Encode)]
    pub struct Invalid;

    impl<T> Validate<T, Invalid, Error> for Invalid {
	#[inline(always)]
	fn validate(_input: T) -> Result<T, Error> {
		Err(Error::NotValid)
	    }
    }

impl<T> Validate<T, Valid, Error> for Valid {
	#[inline(always)]
	fn validate(input: T) -> Result<T, Error> {
		Ok(input)
	}
}

	type CheckARangeTag = (ValidARange, Valid);
	type CheckBRangeTag = (ValidBRange, Valid);
	type CheckABRangeTag = (ValidARange, (ValidBRange, Valid));
	type ManyValidatorsTagsNestedInvalid = (ValidARange, (ValidBRange, (Invalid, Valid)));
	type ManyValidatorsTagsNestedValid = (ValidARange, (ValidBRange, Valid));
	type ManyValidatorsTagsFlatInvalid = (ValidARange, ValidBRange, Invalid, Valid);
	type ManyValidatorsTagsFlatValid = (ValidARange, ValidBRange, Valid);
	// note: next seems is not supported yet
	// type NestedValidated = (Validated<X, Valid>, Validated<Y,  Valid>);
	// #[derive(Debug, Eq, PartialEq, codec::Encode, codec::Decode, Default)]
	// struct Y {
	// }

	#[derive(Debug, Eq, PartialEq, codec::Encode, codec::Decode, Default, Clone)]
	struct X {
		a: u32,
		b: u32,
	}
   #[derive(frame_support::codec::Encode,frame_support::codec::Decode,frame_support::scale_info::TypeInfo,frame_support::PalletError, Debug, PartialEq)]
    pub enum Error {
        NotValid,
        OutOfRange,
    }
   
impl Error {

   pub fn as_str(&self) ->  & 'static str {
      match&self {
       Self::NotValid => "NotValid",
        Self::OutOfRange => "OutOfRange",
        }
    }
 } 
 
    impl From<Error> for &'static str {
    fn from(err:Error) -> &'static str {
           err.as_str()      
        }
    }
 

	impl Validate<X, ValidARange, Error> for ValidARange {
		fn validate(input: X) -> Result<X, Error> {
			if input.a > 10 {
				Err(Error::OutOfRange)
			} else {
				Ok(input)
			}
		}
	}

	impl Validate<X, ValidBRange, Error> for ValidBRange {
		fn validate(input: X) -> Result<X, Error> {
			if input.b > 10 {
				Err(Error::OutOfRange)
			} else {
				Ok(input)
			}
		}
	}

	#[test]
	fn nested_validator() {
		let valid = X { a: 10, b: 0xCAFEBABE };
		assert!(<ManyValidatorsTagsNestedInvalid as Validate<
			X,
			ManyValidatorsTagsNestedInvalid,
            Error
		>>::validate(valid)
		.is_err());

		let valid = X { a: 10, b: 10 };
		assert_ok!(
			<ManyValidatorsTagsNestedValid as Validate<X, ManyValidatorsTagsNestedValid, Error>>::validate(
				valid
			)
		);
	}

	#[test]
	fn either_nested_or_flat() {
		let valid = X { a: 10, b: 0xCAFEBABE };
		assert_eq!(
			<ManyValidatorsTagsNestedInvalid as Validate<X, ManyValidatorsTagsNestedInvalid, Error>>::validate(
				valid.clone()
			),
			<ManyValidatorsTagsFlatInvalid as Validate<X, ManyValidatorsTagsFlatInvalid, Error>>::validate(valid)
		);
	}

	#[test]
	fn flat_validator_multiple_invalid() {
		let value = X { a: 10, b: 0xCAFEBABE };

		assert!(
			<ManyValidatorsTagsFlatInvalid as Validate<X, ManyValidatorsTagsFlatInvalid, Error>>::validate(value).is_err()
		);
	}

	#[test]
	fn flat_validator_multiple_valid() {
		let value = X { a: 10, b: 0xCAFEBABE };

		assert!(
			<ManyValidatorsTagsFlatValid as Validate<X, ManyValidatorsTagsFlatValid, Error>>::validate(
				value
			)
			.is_err()
		);
	}

	#[test]
	fn value() {
		let value = Validated::<_, Valid, Error>::new(42);
		assert_ok!(value);
		let value = Validated::<_, Invalid, Error>::new(42);
		assert!(value.is_err());
	}

	#[test]
	fn test_valid_a() {
		let valid = X { a: 10, b: 0xCAFEBABE };
		let bytes = valid.encode();

		assert_eq!(
			Ok(Validated { value: valid, _marker: PhantomData}),
			Validated::<X, CheckARangeTag, Error>::decode(&mut &bytes[..])
		);
	}

	#[test]
	fn test_invalid_a() {
		let invalid = X { a: 0xDEADC0DE, b: 0xCAFEBABE };
		let bytes = invalid.encode();
		let invalid = Validated::<X, CheckARangeTag, Error>::decode(&mut &bytes[..]);
		assert!(invalid.is_err());
	}

	#[test]
	fn encode_decode_validated_encode_decode() {
		let original = X { a: 0xDEADC0DE, b: 0xCAFEBABE };
		let bytes = original.encode();
		let wrapped = Validated::<X, Valid, Error>::decode(&mut &bytes[..]).unwrap();

		let bytes = wrapped.encode();
		let reencoded = X::decode(&mut &bytes[..]).unwrap();
		assert_eq!(reencoded, original);
	}

	#[test]
	fn test_valid_b() {
		let valid = X { a: 0xCAFEBABE, b: 10 };
		let bytes = valid.encode();
		assert_eq!(
			Ok(Validated { value: valid, _marker: PhantomData}),
			Validated::<X, CheckBRangeTag, Error>::decode(&mut &bytes[..])
		);
	}

	#[test]
	fn test_invalid_b() {
		let invalid = X { a: 0xCAFEBABE, b: 0xDEADC0DE };
		let bytes = invalid.encode();
		let invalid = Validated::<X, CheckBRangeTag, Error>::decode(&mut &bytes[..]);
		assert!(invalid.is_err());
	}

	#[test]
	fn test_valid_ab() {
		let valid = X { a: 10, b: 10 };
		let bytes = valid.encode();
		assert_eq!(
			Ok(Validated { value: valid, _marker: PhantomData}),
			Validated::<X, CheckABRangeTag, Error>::decode(&mut &bytes[..])
		);
	}

	#[test]
	fn test_invalid_ab() {
		let invalid = X { a: 0xDEADC0DE, b: 0xCAFEBABE };
		let bytes = invalid.encode();
		let invalid = Validated::<X, CheckABRangeTag, Error>::decode(&mut &bytes[..]);
		assert!(invalid.is_err());
	}

	#[test]
	fn test_invalid_a_ab() {
		let invalid = X { a: 0xDEADC0DE, b: 10 };
		let bytes = invalid.encode();
		let invalid = Validated::<X, CheckABRangeTag, Error>::decode(&mut &bytes[..]);
		assert!(invalid.is_err());
	}

	#[test]
	fn test_invalid_b_ab() {
		let invalid = X { a: 10, b: 0xDEADC0DE };
		let bytes = invalid.encode();
		let invalid = Validated::<X, CheckABRangeTag, Error>::decode(&mut &bytes[..]);
		assert!(invalid.is_err());
	}

	#[test]
	fn valid_triple() {
		let value = X { a: 10, b: 0xDEADC0DE };
		let bytes = value.encode();
		assert_eq!(
			Ok(Validated { value, _marker: PhantomData}),
			Validated::<X, (Valid, Valid, Valid), Error>::decode(&mut &bytes[..])
		);
	}

	#[test]
	fn valid_invalid_valid() {
		let value = X { a: 10, b: 0xDEADC0DE };
		let bytes = value.encode();

		let invalid = Validated::<X, (Valid, Invalid, Valid), Error>::decode(&mut &bytes[..]);
		assert!(invalid.is_err());
	}

	#[test]
	fn try_into_valid() {
		let value = 42_u32.try_into_validated::<Valid>().unwrap();
		assert_eq!(value, Validated { value: 42, _marker: PhantomData});
	}

	#[test]
	fn try_into_invalid() {
		let value = 42_u32.try_into_validated::<Invalid>();

		assert!(value.is_err());
	}
}

