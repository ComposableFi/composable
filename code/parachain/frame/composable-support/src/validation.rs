//! Module for validating extrinsic inputs
//!
//! This module is made of two main parts that are needed to validate an
//! extrinsic input, the `Validated` struct and the `Validate` trait.
//!
//! # Example
//! ## Single Validation
//! ```
//! use composable_support::validation::{self, Validate, Validated};
//! use scale_info::TypeInfo;
//!
//! pub struct SomeInput;
//!
//! #[derive(Clone, Copy, Debug, PartialEq, TypeInfo)]
//! pub struct ValidateSomeInput;
//!
//! impl Validate<SomeInput, ValidateSomeInput>
//!     for ValidateSomeInput {
//!     fn validate(input: SomeInput) -> Result<SomeInput, &'static str> {
//!         // ... validation code
//!         Ok(input)
//!     }
//! }
//!
//! pub type CheckSomeCondition = (ValidateSomeInput, validation::Valid);
//!
//! pub fn someExtrinsic(input: Validated<SomeInput, CheckSomeCondition>) {
//!     // ... extrinsic code
//! }
//! ```
//!
//! ## Multiple Validations (Up to 3)
//! ```
//! use composable_support::validation::{self, Validate, Validated};
//! use scale_info::TypeInfo;
//!
//! pub struct SomeInput;
//!
//! #[derive(Clone, Copy, Debug, PartialEq, TypeInfo)]
//! pub struct ValidateSomeInput;
//!
//! #[derive(Clone, Copy, Debug, PartialEq, TypeInfo)]
//! pub struct ValidateAnotherCondition;
//!
//! impl Validate<SomeInput, ValidateSomeInput>
//!     for ValidateSomeInput {
//!     fn validate(input: SomeInput) -> Result<SomeInput, &'static str> {
//!         // ... validation code
//!         return Ok(input)
//!     }
//! }
//!
//! impl Validate<SomeInput, ValidateAnotherCondition>
//!     for ValidateAnotherCondition {
//!     fn validate(input: SomeInput) -> Result<SomeInput, &'static str> {
//!         // ... validation code
//!         return Ok(input)
//!     }
//! }
//!
//! pub type CheckSomeConditions = (ValidateSomeInput, ValidateAnotherCondition, validation::Valid);
//!
//! pub fn someExtrinsic(input: Validated<SomeInput, CheckSomeConditions>) {
//!     // ... extrinsic code
//! }
//! ```

use core::{fmt, marker::PhantomData};

use frame_support::log;
use scale_info::TypeInfo;
use sp_runtime::DispatchError;
use sp_std::ops::Deref;

pub mod validators;

/// Black box that embeds the validated value.
/// Validated during construction or serde.
#[derive(Default)]
pub struct Validated<T, U> {
	value: T,
	_marker: PhantomData<U>,
}

impl<T: Copy, U> Copy for Validated<T, U> {}

impl<T: Clone, U> Clone for Validated<T, U> {
	fn clone(&self) -> Self {
		Self { value: self.value.clone(), _marker: PhantomData }
	}
}

impl<T, U> TypeInfo for Validated<T, U>
where
	T: TypeInfo,
{
	type Identity = <T as TypeInfo>::Identity;

	fn type_info() -> scale_info::Type {
		T::type_info()
	}
}

impl<T, U> PartialEq for Validated<T, U>
where
	T: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.value == other.value
	}
}

impl<T, U> Eq for Validated<T, U> where T: PartialEq + Eq {}

impl<T, U> fmt::Debug for Validated<T, U>
where
	T: core::fmt::Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.value.fmt(f)
	}
}

impl<T, U> Validated<T, U>
where
	Validated<T, U>: Validate<T, U>,
	U: Validate<T, U>,
{
	pub fn new(value: T) -> Result<Self, &'static str> {
		match <U as Validate<T, U>>::validate(value) {
			Ok(value) => Ok(Self { value, _marker: PhantomData }),
			Err(e) => Err(e),
		}
	}
}

pub trait ValidateDispatch<U>: Sized {
	fn validate(self) -> Result<Self, DispatchError>;
}

pub trait Validate<T, U> {
	// use string here because in serde layer there is not dispatch
	fn validate(input: T) -> Result<T, &'static str>;
}

pub trait TryIntoValidated<T> {
	fn try_into_validated<U: Validate<T, U>>(self) -> Result<Validated<T, U>, &'static str>;
}

impl<T> TryIntoValidated<T> for T {
	fn try_into_validated<U: Validate<T, U>>(self) -> Result<Validated<T, U>, &'static str> {
		Validated::new(self)
	}
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct Valid;

#[derive(Debug, Eq, PartialEq, Default)]
pub struct Invalid;

impl<T> Validate<T, Invalid> for Invalid {
	#[inline(always)]
	fn validate(_input: T) -> Result<T, &'static str> {
		Err("not valid")
	}
}

impl<T> Validate<T, Valid> for Valid {
	#[inline(always)]
	fn validate(input: T) -> Result<T, &'static str> {
		Ok(input)
	}
}

impl<T, U, V> Validate<T, (U, V)> for (U, V)
where
	U: Validate<T, U>,
	V: Validate<T, V>,
{
	#[inline(always)]
	fn validate(input: T) -> Result<T, &'static str> {
		let value = U::validate(input)?;
		let value = V::validate(value)?;
		Ok(value)
	}
}

// as per substrate pattern and existing macros for similar purposes, they tend to make things
// flat like `#[impl_trait_for_tuples::impl_for_tuples(30)]`
// so if we will need more than 3, can consider it
impl<T, U, V, W> Validate<T, (U, V, W)> for (U, V, W)
where
	U: Validate<T, U>,
	V: Validate<T, V>,
	W: Validate<T, W>,
{
	#[inline(always)]
	fn validate(input: T) -> Result<T, &'static str> {
		let value = U::validate(input)?;
		let value = V::validate(value)?;
		let value = W::validate(value)?;
		Ok(value)
	}
}

impl<T, U, V, W, Z> Validate<T, (U, V, W, Z)> for (U, V, W, Z)
where
	U: Validate<T, U>,
	V: Validate<T, V>,
	W: Validate<T, W>,
	Z: Validate<T, Z>,
{
	#[inline(always)]
	fn validate(input: T) -> Result<T, &'static str> {
		let value = U::validate(input)?;
		let value = V::validate(value)?;
		let value = W::validate(value)?;
		let value = Z::validate(value)?;
		Ok(value)
	}
}

impl<T, U: Validate<T, U>> Validated<T, U> {
	pub fn value(self) -> T {
		self.value
	}
}

impl<T: codec::Decode, U: Validate<T, U>> codec::Decode for Validated<T, U> {
	fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
		// If validation has failed we'll log the error, and continue as usual.
		let value = <U as Validate<T, U>>::validate(T::decode(input)?).map_err(|err| {
			log::warn!("validation error: {:?}", err);
			err
		})?;
		Ok(Validated { value, _marker: PhantomData })
	}
	fn skip<I: codec::Input>(input: &mut I) -> Result<(), codec::Error> {
		T::skip(input)
	}
}

/// Originally there to have `WrapperTypeEncode` work, but now also used in order to prevent
/// .value() calls everywhere
impl<T, U> Deref for Validated<T, U> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.value
	}
}

impl<T: codec::Encode + codec::Decode, U: Validate<T, U>> codec::WrapperTypeEncode
	for Validated<T, U>
{
}

impl<T, U: Validate<T, U>> Validate<T, U> for Validated<T, U> {
	fn validate(input: T) -> Result<T, &'static str> {
		<U as Validate<T, U>>::validate(input)
	}
}

#[cfg(test)]
mod test {
	use codec::{Decode, Encode};
	use frame_support::assert_ok;

	use super::*;

	#[derive(Debug, Eq, PartialEq, Default)]
	struct ValidARange;
	#[derive(Debug, Eq, PartialEq, Default)]
	struct ValidBRange;

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

	impl Validate<X, ValidARange> for ValidARange {
		fn validate(input: X) -> Result<X, &'static str> {
			if input.a > 10 {
				Err("Out of range")
			} else {
				Ok(input)
			}
		}
	}

	impl Validate<X, ValidBRange> for ValidBRange {
		fn validate(input: X) -> Result<X, &'static str> {
			if input.b > 10 {
				Err("Out of range")
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
		>>::validate(valid)
		.is_err());

		let valid = X { a: 10, b: 10 };
		assert_ok!(
			<ManyValidatorsTagsNestedValid as Validate<X, ManyValidatorsTagsNestedValid>>::validate(
				valid
			)
		);
	}

	#[test]
	fn either_nested_or_flat() {
		let valid = X { a: 10, b: 0xCAFEBABE };
		assert_eq!(
			<ManyValidatorsTagsNestedInvalid as Validate<X, ManyValidatorsTagsNestedInvalid>>::validate(
				valid.clone()
			),
			<ManyValidatorsTagsFlatInvalid as Validate<X, ManyValidatorsTagsFlatInvalid>>::validate(valid)
		);
	}

	#[test]
	fn flat_validator_multiple_invalid() {
		let value = X { a: 10, b: 0xCAFEBABE };

		assert!(
			<ManyValidatorsTagsFlatInvalid as Validate<X, ManyValidatorsTagsFlatInvalid>>::validate(value).is_err()
		);
	}

	#[test]
	fn flat_validator_multiple_valid() {
		let value = X { a: 10, b: 0xCAFEBABE };

		assert!(
			<ManyValidatorsTagsFlatValid as Validate<X, ManyValidatorsTagsFlatValid>>::validate(
				value
			)
			.is_err()
		);
	}

	#[test]
	fn value() {
		let value = Validated::<_, Valid>::new(42);
		assert_ok!(value);
		let value = Validated::<_, Invalid>::new(42);
		assert!(value.is_err());
	}

	#[test]
	fn test_valid_a() {
		let valid = X { a: 10, b: 0xCAFEBABE };
		let bytes = valid.encode();

		assert_eq!(
			Ok(Validated { value: valid, _marker: PhantomData }),
			Validated::<X, CheckARangeTag>::decode(&mut &bytes[..])
		);
	}

	#[test]
	fn test_invalid_a() {
		let invalid = X { a: 0xDEADC0DE, b: 0xCAFEBABE };
		let bytes = invalid.encode();
		let invalid = Validated::<X, CheckARangeTag>::decode(&mut &bytes[..]);
		assert!(invalid.is_err());
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
			Validated::<X, CheckBRangeTag>::decode(&mut &bytes[..])
		);
	}

	#[test]
	fn test_invalid_b() {
		let invalid = X { a: 0xCAFEBABE, b: 0xDEADC0DE };
		let bytes = invalid.encode();
		let invalid = Validated::<X, CheckBRangeTag>::decode(&mut &bytes[..]);
		assert!(invalid.is_err());
	}

	#[test]
	fn test_valid_ab() {
		let valid = X { a: 10, b: 10 };
		let bytes = valid.encode();
		assert_eq!(
			Ok(Validated { value: valid, _marker: PhantomData }),
			Validated::<X, CheckABRangeTag>::decode(&mut &bytes[..])
		);
	}

	#[test]
	fn test_invalid_ab() {
		let invalid = X { a: 0xDEADC0DE, b: 0xCAFEBABE };
		let bytes = invalid.encode();
		let invalid = Validated::<X, CheckABRangeTag>::decode(&mut &bytes[..]);
		assert!(invalid.is_err());
	}

	#[test]
	fn test_invalid_a_ab() {
		let invalid = X { a: 0xDEADC0DE, b: 10 };
		let bytes = invalid.encode();
		let invalid = Validated::<X, CheckABRangeTag>::decode(&mut &bytes[..]);
		assert!(invalid.is_err());
	}

	#[test]
	fn test_invalid_b_ab() {
		let invalid = X { a: 10, b: 0xDEADC0DE };
		let bytes = invalid.encode();
		let invalid = Validated::<X, CheckABRangeTag>::decode(&mut &bytes[..]);
		assert!(invalid.is_err());
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

		let invalid = Validated::<X, (Valid, Invalid, Valid)>::decode(&mut &bytes[..]);
		assert!(invalid.is_err());
	}

	#[test]
	fn try_into_valid() {
		let value = 42_u32.try_into_validated::<Valid>().unwrap();

		assert_eq!(value, Validated { value: 42, _marker: PhantomData });
	}

	#[test]
	fn try_into_invalid() {
		let value = 42_u32.try_into_validated::<Invalid>();

		assert!(value.is_err());
	}
}
