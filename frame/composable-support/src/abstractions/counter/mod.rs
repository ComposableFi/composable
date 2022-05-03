use crate::abstractions::utils::{
	decrement::{Decrement, Decrementor},
	increment::{Increment, Incrementor},
	start_at::StartAtValue,
};

use codec::FullCodec;
use core::marker::PhantomData;
use frame_support::{
	pallet_prelude::{StorageValue, ValueQuery},
	traits::{Get, StorageInstance},
};

#[cfg(test)]
mod test_storage_counter;

/// Defines what a nonce should start at and how it should be incremented.
///
/// # Example
///
/// A nonce that starts at zero, incrementing using [`SafeAdd`]:
///
/// ```rust,ignore
/// #[pallet::storage]
/// pub type SomeNonce<T: Config> = StorageValue<
///     _,
///     T::Something,
///     ValueQuery,
///     Counter<ZeroInit, SafeNext, SafePrevious>
/// >;
/// ```
///
/// Increment the nonce in an extrinsic:
///
/// ```rust,ignore
/// #[pallet::call]
/// impl<T: Config> Pallet<T> {
///     pub fn extrinsic(
///         origin: OriginFor<T>,
///     ) -> DispatchResultWithPostInfo {
///         // notice the double ?; since SafeNext is fallible, increment() is also fallible
///         let nonce_next = SomeNonce::increment()??;
///     }
/// }
/// ```
pub struct Counter<S, I, D> {
	#[doc(hidden)]
	_marker: PhantomData<(S, I, D)>,
}

impl<T, S, I, D> Get<T> for Counter<S, I, D>
where
	T: 'static,
	S: StartAtValue<T>,
	I: Incrementor<T>,
{
	fn get() -> T {
		S::value()
	}
}

// NOTE: Once chalk gets integrated and this limitation is fixed, this trait can be removed:
// https://github.com/rust-lang/rust/issues/20400
// https://stackoverflow.com/questions/40392524/conflicting-trait-implementations-even-though-associated-types-differ/40408431#40408431use
pub trait CounterHelperTrait<T, I, D>: 'static
where
	T: FullCodec + Clone + Copy + 'static,
	I: Incrementor<T>,
	D: Decrementor<T>,
{
	type IOutput;

	type DOutput;

	fn increment_inner() -> Self::IOutput;

	fn decrement_inner() -> Self::DOutput;
}

#[allow(clippy::disallowed_types)]
impl<P, T, S, I, D> CounterHelperTrait<T, I, D>
	for (StorageValue<P, T, ValueQuery, Counter<S, I, D>>, T, T, S)
where
	P: StorageInstance + 'static,
	T: FullCodec + Clone + Copy + 'static,
	S: StartAtValue<T>,
	I: Incrementor<T, Output = T>,
	D: Decrementor<T, Output = T>,
	Counter<S, I, D>: Get<T>,
{
	type IOutput = I::Output;
	type DOutput = D::Output;

	fn increment_inner() -> Self::IOutput {
		#[allow(clippy::disallowed_types)]
		StorageValue::<P, T, ValueQuery, Counter<S, I, D>>::mutate(|x| {
			let new_x = I::increment(*x);
			*x = new_x;
			new_x
		})
	}

	fn decrement_inner() -> Self::DOutput {
		#[allow(clippy::disallowed_types)]
		StorageValue::<P, T, ValueQuery, Counter<S, I, D>>::mutate(|x| {
			let new_x = D::decrement(*x);
			*x = new_x;
			new_x
		})
	}
}

#[allow(clippy::disallowed_types)]
impl<P, T, S, I, D, IncrementErr, DecrementErr> CounterHelperTrait<T, I, D>
	for (
		StorageValue<P, T, ValueQuery, Counter<S, I, D>>,
		Result<T, IncrementErr>,
		Result<T, DecrementErr>,
		S,
	) where
	P: StorageInstance + 'static,
	T: FullCodec + Clone + Copy + 'static,
	S: StartAtValue<T>,
	I: Incrementor<T, Output = Result<T, IncrementErr>>,
	D: Decrementor<T, Output = Result<T, DecrementErr>>,
	Counter<S, I, D>: Get<T>,
	IncrementErr: 'static,
	DecrementErr: 'static,
{
	type IOutput = I::Output;
	type DOutput = D::Output;

	fn increment_inner() -> Self::IOutput {
		#[allow(clippy::disallowed_types)]
		StorageValue::<P, T, ValueQuery, Counter<S, I, D>>::try_mutate(
			|x| -> Result<T, IncrementErr> {
				match I::increment(*x) {
					Ok(new_x) => {
						*x = new_x;
						Ok(new_x)
					},
					Err(err) => Err(err),
				}
			},
		)
	}

	fn decrement_inner() -> Self::DOutput {
		#[allow(clippy::disallowed_types)]
		StorageValue::<P, T, ValueQuery, Counter<S, I, D>>::try_mutate(
			|x| -> Result<T, DecrementErr> {
				match D::decrement(*x) {
					Ok(new_x) => {
						*x = new_x;
						Ok(new_x)
					},
					Err(err) => Err(err),
				}
			},
		)
	}
}

impl<P, T, S, I, D> Increment<T, I> for StorageValue<P, T, ValueQuery, Counter<S, I, D>>
where
	P: StorageInstance + 'static,
	T: FullCodec + Clone + Copy + 'static,
	S: StartAtValue<T>,
	I: Incrementor<T>,
	D: Decrementor<T>,
	(StorageValue<P, T, ValueQuery, Counter<S, I, D>>, I::Output, D::Output, S):
		CounterHelperTrait<T, I, D>,
{
	type Output =
		<(StorageValue<P, T, ValueQuery, Counter<S, I, D>>, I::Output, D::Output, S) as CounterHelperTrait<
			T,
			I,
			D,
		>>::IOutput;

	fn increment() -> Self::Output {
		<(StorageValue<P, T, ValueQuery, Counter<S, I, D>>, I::Output, D::Output, S) as CounterHelperTrait<
			T,
			I,
			D,
		>>::increment_inner()
	}
}

impl<P, T, S, I, D> Decrement<T, D> for StorageValue<P, T, ValueQuery, Counter<S, I, D>>
where
	P: StorageInstance + 'static,
	T: FullCodec + Clone + Copy + 'static,
	S: StartAtValue<T>,
	I: Incrementor<T>,
	D: Decrementor<T>,
	(StorageValue<P, T, ValueQuery, Counter<S, I, D>>, I::Output, D::Output, S):
		CounterHelperTrait<T, I, D>,
{
	type Output =
		<(StorageValue<P, T, ValueQuery, Counter<S, I, D>>, I::Output, D::Output, S) as CounterHelperTrait<
			T,
			I,
			D,
		>>::DOutput;

	fn decrement() -> Self::Output {
		<(StorageValue<P, T, ValueQuery, Counter<S, I, D>>, I::Output, D::Output, S) as CounterHelperTrait<
			T,
			I,
			D,
		>>::decrement_inner()
	}
}
