use core::marker::PhantomData;
use scale_info::TypeInfo;
use sp_runtime::DispatchError;

/// Black box that embbed the validated value.
#[derive(Default, Copy, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct Validated<T, U> {
	value: T,
	_marker: PhantomData<U>,
}

impl<T, U> Validated<T, U>
where
	Validated<T, U>: Validate<T, U>,
	U: Validate<T, U>,
{
	pub fn new(value: T, _validator_tag: U) -> Result<Self, &'static str> {
		// Validate::<U>::validate(Self { value, _marker: PhantomData })
		// Ok(Self { value: Validate::<T, U>::validate(value), _marker: PhantomData })
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
	fn validate(extrinsic_input: T) -> Result<T, &'static str>;
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct Valid;

#[derive(Debug, Eq, PartialEq, Default)]
pub struct Invalid;

impl<T> Validate<T, Invalid> for Invalid {
	#[inline(always)]
	fn validate(_extinsic_input: T) -> Result<T, &'static str> {
		Err("not valid")
	}
}

impl<T> Validate<T, Valid> for Valid {
	#[inline(always)]
	fn validate(extrinsic_input: T) -> Result<T, &'static str> {
		Ok(extrinsic_input)
	}
}

impl<T, U, V> Validate<T, (U, V)> for U
where
	U: Validate<T, U>,
	V: Validate<T, V>,
{
	#[inline(always)]
	fn validate(extrinsic_input: T) -> Result<T, &'static str> {
		let value = <U as Validate<T, U>>::validate(extrinsic_input)?;
		let value = <V as Validate<T, V>>::validate(value)?;
		Ok(value)
	}
}

// as per substrate pattern and existing macroses for similar purposes, they tend to make things
// flat like `#[impl_trait_for_tuples::impl_for_tuples(30)]`
// so if we will need more than 3, can consider it
impl<T, U, V, W> Validate<T, (U, V, W)> for U
where
	U: Validate<T, U>,
	V: Validate<T, V>,
	W: Validate<T, W>,
{
	#[inline(always)]
	fn validate(extrinsic_input: T) -> Result<T, &'static str> {
		let value = <U as Validate<T, U>>::validate(extrinsic_input)?;
		let value = <V as Validate<T, V>>::validate(value)?;
		let value = <W as Validate<T, W>>::validate(value)?;
		Ok(value)
	}
}

impl<T, U, V, W, Z> Validate<T, (U, V, W, Z)> for U
where
	U: Validate<T, U>,
	V: Validate<T, V>,
	W: Validate<T, W>,
	Z: Validate<T, Z>,
{
	#[inline(always)]
	fn validate(extrinsic_input: T) -> Result<T, &'static str> {
		let value = <U as Validate<T, U>>::validate(extrinsic_input)?;
		let value = <V as Validate<T, V>>::validate(value)?;
		let value = <W as Validate<T, W>>::validate(value)?;
		let value = <Z as Validate<T, Z>>::validate(value)?;
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
		let value = <U as Validate<T, U>>::validate(T::decode(input)?)?;
		Ok(Validated { value, _marker: PhantomData })
	}
	fn skip<I: codec::Input>(input: &mut I) -> Result<(), codec::Error> {
		T::skip(input)
	}
}

pub(crate) mod private {
	use sp_std::ops::Deref;

	use super::Validated;

	/// just to have `WrapperTypeEncode` work
	impl<T, U> Deref for Validated<T, U> {
		type Target = T;
		#[doc(hidden)]
		fn deref(&self) -> &Self::Target {
			&self.value
		}
	}
}

impl<T: codec::Encode + codec::Decode, U: Validate<T, U>> codec::WrapperTypeEncode
	for Validated<T, U>
{
}

// impl<T: codec::Encode + codec::Decode, U: Validate<T, U>> codec::WrapperTypeDecode
// 	for Validated<T, U>
// {
// }

impl<T, U: Validate<T, U>> Validate<T, U> for Validated<T, U> {
	fn validate(extrinsic_input: T) -> Result<T, &'static str> {
		<U as Validate<T, U>>::validate(extrinsic_input)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use codec::{Decode, Encode};
	use frame_support::assert_ok;

	#[derive(Debug, Eq, PartialEq, Default)]
	struct ValidARange;
	#[derive(Debug, Eq, PartialEq, Default)]
	struct ValidBRange;

	type CheckARangeTag = (ValidARange, Valid);
	type CheckBRangeTag = (ValidBRange, Valid);
	type CheckABRangeTag = (ValidARange, (ValidBRange, Valid));
	// type ManyValidatorsTagsNested = (ValidARange, (ValidBRange, (Invalid, Valid)));
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
		fn validate(extrinsic_input: X) -> Result<X, &'static str> {
			if extrinsic_input.a > 10 {
				Err("Out of range")
			} else {
				Ok(extrinsic_input)
			}
		}
	}

	impl Validate<X, ValidBRange> for ValidBRange {
		fn validate(extrinsic_input: X) -> Result<X, &'static str> {
			if extrinsic_input.b > 10 {
				Err("Out of range")
			} else {
				Ok(extrinsic_input)
			}
		}
	}

	// #[test]
	// fn nested_validator() {
	// 	let valid = X { a: 10, b: 0xCAFEBABE };
	//
	// 	assert!(Validate::<X, ManyValidatorsTagsNested>::validate(valid).is_err());
	// }
	//
	// #[test]
	// fn either_nested_or_flat() {
	// 	let valid = X { a: 10, b: 0xCAFEBABE };
	// 	assert_eq!(
	// 		<ManyValidatorsTagsNested as Validate<X, ManyValidatorsTagsNested>>::validate(
	// 			valid.clone()
	// 		),
	// 		Validate::<X, ManyValidatorsTagsFlat>::validate(valid)
	// 	);
	// }

	#[test]
	fn flat_validator_multiple_invalid() {
		let value = X { a: 10, b: 0xCAFEBABE };

		assert!(
			<ValidARange as Validate<X, ManyValidatorsTagsFlatInvalid>>::validate(value).is_err()
		);
	}

	#[test]
	fn flat_validator_multiple_valid() {
		let value = X { a: 10, b: 0xCAFEBABE };

		assert!(<ValidARange as Validate<X, ManyValidatorsTagsFlatValid>>::validate(value).is_err());
	}

	#[test]
	fn value() {
		let value = Validated::new(42, Valid);
		assert_ok!(value);
		let value = Validated::new(42, Invalid);
		assert!(value.is_err());
		let some_x = X { a: 10, b: 0xCAFEBABE };
		let value = Validated::new(some_x, ValidARange);
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
}
