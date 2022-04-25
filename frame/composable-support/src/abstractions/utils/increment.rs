use crate::{
	abstractions::utils::increment::sealed::Sealed,
	math::{safe::SafeAdd, wrapping_next::WrappingNext},
};
use frame_support::pallet_prelude::Get;
use sp_runtime::{traits::One, ArithmeticError};
use sp_std::{fmt::Debug, marker::PhantomData, ops::Add};

/// A trait defining something that has a potential "next" value..
pub trait Incrementor<T: 'static>: Sealed + 'static {
	type Output;

	fn increment(value: T) -> Self::Output;
}

/// [Increment] with [`WrappingNext`].
pub struct WrappingIncrement {}

impl<T> Incrementor<T> for WrappingIncrement
where
	T: Debug + WrappingNext + 'static,
{
	type Output = T;

	fn increment(value: T) -> Self::Output {
		value.next()
	}
}

/// [Increment] with [`SafeAdd`].
pub struct SafeIncrement {}

impl<T> Incrementor<T> for SafeIncrement
where
	T: Debug + SafeAdd + One + 'static,
{
	type Output = Result<T, ArithmeticError>;

	fn increment(value: T) -> Self::Output {
		value.safe_add(&T::one())
	}
}

/// [Increment] up to a maximum value.
pub struct IncrementToMax<M: 'static, E: 'static, PE: 'static> {
	#[doc(hidden)]
	_marker: PhantomData<(M, E, PE)>,
}

impl<T, Max, MaxError, PalletError> Incrementor<T> for IncrementToMax<Max, MaxError, PalletError>
where
	T: Debug + Add<T, Output = T> + One + PartialOrd + 'static,
	Max: Get<T> + 'static,
	MaxError: Debug + Default + Into<PalletError> + 'static,
	PalletError: 'static,
{
	type Output = Result<T, MaxError>;

	/// [`Add`] is used safely here since `M::get()` must be `<=` the upper limit for `T`.
	fn increment(value: T) -> Self::Output {
		let new_value = value.add(T::one());
		if new_value <= Max::get() {
			Ok(new_value)
		} else {
			Err(MaxError::default())
		}
	}
}

/// Helper macro to create a type that can be used in [`IncrementToMax`].
///
/// # Usage
///
/// ```rust,ignore
/// error_to_pallet_error!(
///     SomeErrorType -> PalletErrorVariant;
/// );
/// ```
///
/// Note that this assumes that the pallet's `Error` and `Config` types are in scope and not
/// renamed.
#[macro_export]
macro_rules! error_to_pallet_error {
	($($name:ident -> $to:ident;)+) => {
		$(
			#[derive(core::fmt::Debug, core::default::Default, core::cmp::PartialEq)]
			pub struct $name;

			impl<T: Config> From<$name> for Error<T> {
				fn from(_: $name) -> Error<T> {
					Error::<T>::$to
				}
			}
		)+
	};
}

mod sealed {
	use super::*;

	/// Sealing trait for [`Increment`][super::Increment]. If you want to add a new implementor, be
	/// sure to add it here and ensure it's tested.
	pub trait Sealed {}

	impl Sealed for SafeIncrement {}
	impl Sealed for WrappingIncrement {}
	impl<Max, MaxError, PalletError> Sealed for IncrementToMax<Max, MaxError, PalletError> {}
}
