use crate::abstractions::utils::{
	increment::{Increment, Incrementor},
	start_at::StartAtValue,
	ValueQuery,
};

use codec::FullCodec;
use frame_support::{
	pallet_prelude::StorageValue,
	traits::{Get, StorageInstance},
};
use sp_std::marker::PhantomData;

#[cfg(test)]
mod test_storage_nonce;

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
///     Nonce<ZeroInit, SafeNext>
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
pub struct Nonce<S, I> {
	#[doc(hidden)]
	_marker: PhantomData<(S, I)>,
}

impl<T, S, I> Get<T> for Nonce<S, I>
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
pub trait NonceHelperTrait<T, S, I>: 'static
where
	T: FullCodec + Clone + Copy + 'static,
	S: StartAtValue<T>,
	I: Incrementor<T>,
{
	type Output;

	fn increment_inner() -> Self::Output;
}

#[allow(clippy::disallowed_types)]
impl<P, T, S, I> NonceHelperTrait<T, S, I> for (StorageValue<P, T, ValueQuery, Nonce<S, I>>, T, S)
where
	P: StorageInstance + 'static,
	T: FullCodec + Clone + Copy + 'static,
	S: StartAtValue<T>,
	I: Incrementor<T, Output = T>,
	Nonce<S, I>: Get<T>,
{
	type Output = I::Output;

	fn increment_inner() -> Self::Output {
		#[allow(clippy::disallowed_types)]
		StorageValue::<P, T, ValueQuery, Nonce<S, I>>::mutate(|x| {
			let new_x = I::increment(*x);
			*x = new_x;
			new_x
		})
	}
}

#[allow(clippy::disallowed_types)]
impl<P, T, S, I, IncrementErr> NonceHelperTrait<T, S, I>
	for (StorageValue<P, T, ValueQuery, Nonce<S, I>>, Result<T, IncrementErr>, S)
where
	P: StorageInstance + 'static,
	T: FullCodec + Clone + Copy + 'static,
	S: StartAtValue<T>,
	I: Incrementor<T, Output = Result<T, IncrementErr>>,
	Nonce<S, I>: Get<T>,
	IncrementErr: 'static,
{
	type Output = I::Output;

	fn increment_inner() -> Self::Output {
		#[allow(clippy::disallowed_types)]
		StorageValue::<P, T, ValueQuery, Nonce<S, I>>::try_mutate(|x| -> Result<T, IncrementErr> {
			match I::increment(*x) {
				Ok(new_x) => {
					*x = new_x;
					Ok(new_x)
				},
				Err(err) => Err(err),
			}
		})
	}
}

impl<P, T, S, I> Increment<T, I> for StorageValue<P, T, ValueQuery, Nonce<S, I>>
where
	P: StorageInstance + 'static,
	T: FullCodec + Clone + Copy + 'static,
	S: StartAtValue<T>,
	I: Incrementor<T>,
	(StorageValue<P, T, ValueQuery, Nonce<S, I>>, I::Output, S): NonceHelperTrait<T, S, I>,
{
	type Output =
		<(StorageValue<P, T, ValueQuery, Nonce<S, I>>, I::Output, S) as NonceHelperTrait<
			T,
			S,
			I,
		>>::Output;

	fn increment() -> Self::Output {
		<(StorageValue<P, T, ValueQuery, Nonce<S, I>>, I::Output, S) as NonceHelperTrait<
			T,
			S,
			I,
		>>::increment_inner()
	}
}
