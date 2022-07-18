use crate::{
	abstractions::utils::{decrement::sealed::Sealed, start_at::StartAtValue},
	math::safe::SafeSub,
};

use codec::FullCodec;
use sp_runtime::{traits::One, ArithmeticError};
use sp_std::fmt::Debug;

/// An extension trait for [`StorageValue`]s that can be decreased.
pub trait Decrement<T, D>: Sealed + 'static
where
	T: FullCodec + Clone + Copy + 'static,
	D: Decrementor<T>,
{
	/// See [`Decrementor::Output`].
	type Output;

	/// Decrement the inner value.
	fn decrement() -> Self::Output;
}

/// Something that can decrement a value.
pub trait Decrementor<T: 'static>: Sealed + 'static {
	/// The result of decrementing the provided value `T`.
	///
	/// Since decrementing a value is potentially a fallible operation, the return type of
	/// [`Self::decrement`] is *not* just `T`; allowing for returning a Result, Option, or even a
	/// completely new type.
	type Output;

	fn decrement(value: T) -> Self::Output;
}

/// An [`Decrementor`] that uses [`SafeSub`] to produce the next value.
pub struct SafeDecrement;

impl<T> Decrementor<T> for SafeDecrement
where
	T: Debug + SafeSub + One + 'static,
{
	type Output = Result<T, ArithmeticError>;

	fn decrement(value: T) -> Self::Output {
		value.safe_sub(&T::one())
	}
}

// I'm not sure if this decrementor makes sense, or how it would be used in practice. Leaving it
// here in case we need/ want it. /// An [`Decrementor`] that decrements down to a minimum value.
// pub struct DecrementToMin<Min: 'static, MinError: 'static, PalletError: 'static> {
// 	#[doc(hidden)]
// 	_marker: PhantomData<(Min, MinError, PalletError)>,
// }

// impl<T, Min, MinError, PalletError> Decrementor<T> for DecrementToMin<Min, MinError, PalletError>
// where
// 	T: Debug + Sub<T, Output = T> + One + PartialOrd + 'static,
// 	Min: Get<T> + 'static,
// 	MinError: Debug + Default + Into<PalletError> + 'static,
// 	PalletError: 'static,
// {
// 	type Output = Result<T, MinError>;

// 	/// [`Sub`] is used safely here since `M::get()` must be `>` the lower limit for `T`.
// 	fn decrement(value: T) -> Self::Output {
// 		if value == Min::get() {
// 			Err(MinError::default())
// 		} else {
// 			Ok(value.sub(T::one()))
// 		}
// 	}
// }

mod sealed {
	use frame_support::{pallet_prelude::StorageValue, traits::StorageInstance};

	use crate::abstractions::{
		counter::{Counter, CounterHelperTrait},
		utils::{increment::Incrementor, ValueQuery},
	};

	use super::*;

	/// Sealing trait for [`Decrement`][super::Decrement] and [`Decrementor`][super::Decrementor].
	/// If you want to add a new implementor, be sure to add it here and ensure it's tested.
	pub trait Sealed {}

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

	impl Sealed for SafeDecrement {}

	// See comment on commented-out DecrementToMin
	// impl<Max, MaxError, PalletError> Sealed for DecrementToMin<Max, MaxError, PalletError> {}
}
