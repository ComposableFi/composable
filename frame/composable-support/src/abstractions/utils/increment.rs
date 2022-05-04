use crate::{
	abstractions::utils::{increment::sealed::Sealed, start_at::StartAtValue},
	math::{safe::SafeAdd, wrapping_next::WrappingNext},
};

use codec::FullCodec;
use frame_support::pallet_prelude::Get;
use sp_runtime::{traits::One, ArithmeticError};
use sp_std::{fmt::Debug, marker::PhantomData, ops::Add};

/// An extension trait for [`StorageValue`]s that are used as a [nonce](nonce).
///
/// [nonce]: <https://www.investopedia.com/terms/n/nonce.asp>
pub trait Increment<T, I>: Sealed + 'static
where
	T: FullCodec + Clone + Copy + 'static,
	I: Incrementor<T>,
{
	/// See [`Incrementor::Output`].
	type Output;

	/// Increment the inner value.
	fn increment() -> Self::Output;
}

/// Something that can increment a value.
pub trait Incrementor<T: 'static>: Sealed + 'static {
	/// The result of incrementing the provided value `T`.
	///
	/// Since incrementing a value is potentially a fallible operation, the return type of
	/// [`Self::increment`] is *not* just `T`; allowing for returning a Result, Option, or even a
	/// completely new type.
	type Output;

	fn increment(value: T) -> Self::Output;
}

/// An [`Incrementor`] that uses [`WrappingNext`] to produce the next value.
pub struct WrappingIncrement;

impl<T> Incrementor<T> for WrappingIncrement
where
	T: Debug + WrappingNext + 'static,
{
	type Output = T;

	fn increment(value: T) -> Self::Output {
		value.next()
	}
}

/// An [`Incrementor`] that uses [`SafeAdd`] to produce the next value.
pub struct SafeIncrement;

impl<T> Incrementor<T> for SafeIncrement
where
	T: Debug + SafeAdd + One + 'static,
{
	type Output = Result<T, ArithmeticError>;

	fn increment(value: T) -> Self::Output {
		value.safe_add(&T::one())
	}
}

/// An [`Incrementor`] that increments up to a maximum value.
pub struct IncrementToMax<Max: 'static, MaxError: 'static, PalletError: 'static> {
	#[doc(hidden)]
	_marker: PhantomData<(Max, MaxError, PalletError)>,
}

impl<T, Max, MaxError, PalletError> Incrementor<T> for IncrementToMax<Max, MaxError, PalletError>
where
	T: Debug + Add<T, Output = T> + One + PartialOrd + 'static,
	Max: Get<T> + 'static,
	MaxError: Debug + Default + Into<PalletError> + 'static,
	PalletError: 'static,
{
	type Output = Result<T, PalletError>;

	/// [`Add`] is used safely here since `M::get()` must be `<=` the upper limit for `T`.
	fn increment(value: T) -> Self::Output {
		if value < Max::get() {
			let new_value = value.add(T::one());
			Ok(new_value)
		} else {
			Err(MaxError::default().into())
		}
	}
}

mod sealed {
	use frame_support::{pallet_prelude::StorageValue, traits::StorageInstance};

	use crate::abstractions::{
		counter::{Counter, CounterHelperTrait},
		nonce::{Nonce, NonceHelperTrait},
		utils::{decrement::Decrementor, ValueQuery},
	};

	use super::*;

	/// Sealing trait for [`Increment`][super::Increment] and [`Incrementor`][super::Incrementor].
	/// If you want to add a new implementor, be sure to add it here and ensure it's tested.
	pub trait Sealed {}

	impl<P, T, S, I> Sealed for StorageValue<P, T, ValueQuery, Nonce<S, I>>
	where
		P: StorageInstance + 'static,
		T: FullCodec + Clone + Copy + 'static,
		S: StartAtValue<T>,
		I: Incrementor<T>,
		(StorageValue<P, T, ValueQuery, Nonce<S, I>>, I::Output, S): NonceHelperTrait<T, S, I>,
	{
	}

	impl<P, T, S, I, D> Sealed for StorageValue<P, T, ValueQuery, Counter<S, I, D>>
	where
		P: StorageInstance + 'static,
		T: FullCodec + Clone + Copy + 'static,
		S: StartAtValue<T>,
		I: Incrementor<T>,
		D: Decrementor<T>,
		(StorageValue<P, T, ValueQuery, Counter<S, I, D>>, I::Output, D::Output, S):
			CounterHelperTrait<T, I, D>,
	{
	}

	impl Sealed for SafeIncrement {}
	impl Sealed for WrappingIncrement {}
	impl<Max, MaxError, PalletError> Sealed for IncrementToMax<Max, MaxError, PalletError> {}
}
