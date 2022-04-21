use sp_std::marker::PhantomData;

use crate::abstractions::utils::{increment::Increment, start_at::StartAtValue};

use codec::FullCodec;
use frame_support::{
	pallet_prelude::{StorageValue, ValueQuery},
	traits::{Get, StorageInstance},
};

#[cfg(test)]
mod test_storage_nonce;

/// An extension trait for [`StorageValue`]s that are used as a [nonce](nonce).
///
/// [nonce]: <https://www.investopedia.com/terms/n/nonce.asp>
pub trait StorageNonce<T, S, I>: 'static
where
	T: FullCodec + Clone + Copy + 'static,
	S: StartAtValue<T>,
	I: Increment<T>,
{
	type Output;

	fn increment() -> Self::Output;
}

// NOTE: Once chalk gets integrated and this limitation is fixed, this trait can be removed:
// https://github.com/rust-lang/rust/issues/20400
// https://stackoverflow.com/questions/40392524/conflicting-trait-implementations-even-though-associated-types-differ/40408431#40408431use
pub trait StorageNonceInner<T, S, I>: 'static
where
	T: FullCodec + Clone + Copy + 'static,
	S: StartAtValue<T>,
	I: Increment<T>,
{
	type Output;

	fn increment_inner() -> Self::Output;
}

#[allow(clippy::disallowed_types)]
impl<P, T, S, I> StorageNonceInner<T, S, I> for (StorageValue<P, T, ValueQuery, Nonce<S, I>>, T)
where
	P: StorageInstance + 'static,
	T: FullCodec + Clone + Copy + 'static,
	S: StartAtValue<T>,
	I: Increment<T, Output = T>,
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
impl<P, T, S, I, IncrementErr> StorageNonceInner<T, S, I>
	for (StorageValue<P, T, ValueQuery, Nonce<S, I>>, Result<T, IncrementErr>)
where
	P: StorageInstance + 'static,
	T: FullCodec + Clone + Copy + 'static,
	S: StartAtValue<T>,
	I: Increment<T, Output = Result<T, IncrementErr>>,
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

impl<TStorage, T, S, I> StorageNonce<T, S, I> for TStorage
where
	T: FullCodec + Clone + Copy + 'static,
	S: StartAtValue<T>,
	I: Increment<T>,
	(TStorage, I::Output): StorageNonceInner<T, S, I>,
{
	type Output = <(TStorage, I::Output) as StorageNonceInner<T, S, I>>::Output;

	fn increment() -> Self::Output {
		<(TStorage, I::Output) as StorageNonceInner<T, S, I>>::increment_inner()
	}
}

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
///     Nonce<T, ZeroStart, SafeNext>
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
///         let nonce_next = SomeNonce::try_increment()??;
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
	I: Increment<T>,
{
	fn get() -> T {
		S::value()
	}
}
