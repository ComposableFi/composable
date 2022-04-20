use crate::{
	abstractions::nonce::sealed::Sealed,
	math::{safe::SafeAdd, wrapping_next::WrappingNext},
};

use codec::FullCodec;
use core::fmt::Debug;
use frame_support::{
	pallet_prelude::{StorageValue, ValueQuery},
	traits::{Get, StorageInstance},
};
use sp_runtime::{
	traits::{One, Zero},
	ArithmeticError,
};
use sp_std::marker::PhantomData;

/// A [`Nonce`] that starts at [`<T as Zero>::zero`][Zero::zero].
pub type StartAtZero<T, I> = Nonce<T, ZeroStart, I>;
/// A [`Nonce`] that starts at [`<T as One>::one`][One::one].
pub type StartAtOne<T, I> = Nonce<T, OneStart, I>;
/// A [`Nonce`] that starts at [`<T as One>::default`][Default::default].
pub type StartAtDefault<T, I> = Nonce<T, DefaultStart, I>;

pub trait StorageNonce<T, S, I>: 'static
where
	T: FullCodec + Clone + Copy + 'static,
	S: StartAtValue<T>,
	I: Increment<T>,
{
	type Output;

	fn try_increment() -> Self::Output;

	fn try_increment_with<Err, F>(f: F) -> Result<Self::Output, Err>
	where
		Err: Debug,
		F: FnOnce(T) -> Result<T, Err>;
}

pub trait StorageNonceInner<T, S, I>: 'static
where
	T: FullCodec + Clone + Copy + 'static,
	S: StartAtValue<T>,
	I: Increment<T>,
{
	type Output;

	fn try_increment_inner() -> Self::Output;

	fn try_increment_with_inner<Err, F>(f: F) -> Result<Self::Output, Err>
	where
		Err: Debug,
		F: FnOnce(T) -> Result<T, Err>;
}

// NOTE: https://github.com/rust-lang/rust/issues/20400
// NOTE: https://stackoverflow.com/questions/40392524/conflicting-trait-implementations-even-though-associated-types-differ/40408431#40408431use

impl<P, T, S, I> StorageNonceInner<T, S, I> for (StorageValue<P, T, ValueQuery, Nonce<T, S, I>>, T)
where
	P: StorageInstance + 'static,
	T: FullCodec + Clone + Copy + 'static,
	S: StartAtValue<T>,
	I: Increment<T, Output = T>,
	Nonce<T, S, I>: Get<T>,
{
	type Output = I::Output;

	fn try_increment_inner() -> Self::Output {
		StorageValue::<P, T, ValueQuery, Nonce<T, S, I>>::mutate(|x| {
			let new_x = I::increment(*x);
			*x = new_x;
			new_x
		})
	}

	fn try_increment_with_inner<Err: Debug, F: FnOnce(T) -> Result<T, Err>>(
		f: F,
	) -> Result<Self::Output, Err> {
		StorageValue::<P, T, ValueQuery, Nonce<T, S, I>>::try_mutate(|x| {
			match f(I::increment(*x)) {
				Ok(new_x) => {
					*x = new_x;
					Ok(new_x)
				},
				Err(err) => Err(err),
			}
		})
	}
}

impl<P, T, S, I, IncrementErr> StorageNonceInner<T, S, I>
	for (StorageValue<P, T, ValueQuery, Nonce<T, S, I>>, Result<T, IncrementErr>)
where
	P: StorageInstance + 'static,
	T: FullCodec + Clone + Copy + 'static,
	S: StartAtValue<T>,
	I: Increment<T, Output = Result<T, IncrementErr>>,
	Nonce<T, S, I>: Get<T>,
	IncrementErr: 'static,
{
	type Output = I::Output;

	fn try_increment_inner() -> Self::Output {
		StorageValue::<P, T, ValueQuery, Nonce<T, S, I>>::try_mutate(
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

	fn try_increment_with_inner<
		ClosureCheckErr: Debug,
		F: FnOnce(T) -> Result<T, ClosureCheckErr>,
	>(
		f: F,
	) -> Result<Self::Output, ClosureCheckErr> {
		StorageValue::<P, T, ValueQuery, Nonce<T, S, I>>::try_mutate(
			|x: &mut T| -> Result<Result<T, IncrementErr>, ClosureCheckErr> {
				match I::increment(*x) {
					Ok(new_x) => match f(new_x) {
						Ok(ok) => {
							*x = ok;
							Ok(Ok(ok))
						},
						Err(why) => Err(why),
					},
					Err(e) => Ok(Err(e)),
				}
			},
		)
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

	fn try_increment() -> Self::Output {
		<(TStorage, I::Output) as StorageNonceInner<T, S, I>>::try_increment_inner()
	}

	fn try_increment_with<Err, F>(f: F) -> Result<Self::Output, Err>
	where
		Err: Debug,
		F: FnOnce(T) -> Result<T, Err>,
	{
		<(TStorage, I::Output) as StorageNonceInner<T, S, I>>::try_increment_with_inner(f)
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
// One day...
// pub struct Nonce<T, S<T>, I<T>>
pub struct Nonce<T, S, I>
where
	T: 'static,
	S: StartAtValue<T>,
	I: Increment<T>,
{
	#[doc(hidden)]
	_marker: PhantomData<(S, T, I)>,
}

pub trait StartAtValue<T: 'static>: Sealed + 'static {
	fn value() -> T;
}

// start markers

/// Marker for nonces that should start at `<T as Zero>::zero`.
pub struct ZeroStart;

impl<T: Zero + 'static> StartAtValue<T> for ZeroStart {
	fn value() -> T {
		T::zero()
	}
}

impl<T, I> Get<T> for Nonce<T, ZeroStart, I>
where
	T: Zero + 'static,
	I: Increment<T>,
{
	fn get() -> T {
		<ZeroStart as StartAtValue<T>>::value()
	}
}

/// Marker for nonces that should start at `<T as One>::one`.
pub struct OneStart;

impl<T: One + 'static> StartAtValue<T> for OneStart {
	fn value() -> T {
		T::one()
	}
}

impl<T, I> Get<T> for Nonce<T, OneStart, I>
where
	T: One + 'static,
	I: Increment<T>,
{
	fn get() -> T {
		<OneStart as StartAtValue<T>>::value()
	}
}

/// Marker for nonces that should start at `<T as Default>::default`.
pub struct DefaultStart;

impl<T: Default + 'static> StartAtValue<T> for DefaultStart {
	fn value() -> T {
		T::default()
	}
}

impl<T, I> Get<T> for Nonce<T, DefaultStart, I>
where
	T: Default + 'static,
	I: Increment<T>,
{
	fn get() -> T {
		<DefaultStart as StartAtValue<T>>::value()
	}
}

// Increment trait

pub trait Increment<T: 'static>: 'static {
	type Output;

	fn increment(value: T) -> Self::Output;
}

/// [Increment] with [`WrappingNext`].
pub struct WrappingIncrement<T: 'static> {
	#[doc(hidden)]
	_marker: PhantomData<T>,
}

impl<T> Increment<T> for WrappingIncrement<T>
where
	T: Debug + WrappingNext + 'static,
{
	type Output = T;

	fn increment(value: T) -> Self::Output {
		value.next()
	}
}

/// [Increment] with [`SafeAdd`].
pub struct SafeIncrement<T: 'static> {
	#[doc(hidden)]
	_marker: PhantomData<T>,
}

impl<T> Increment<T> for SafeIncrement<T>
where
	T: Debug + SafeAdd + One + 'static,
{
	type Output = Result<T, ArithmeticError>;

	fn increment(value: T) -> Self::Output {
		value.safe_add(&T::one())
	}
}

/// Sealed trait so that only these types can be used for a [`Nonce`].
mod sealed {
	use crate::abstractions::nonce::{
		DefaultStart, OneStart, SafeIncrement, WrappingIncrement, ZeroStart,
	};

	pub trait Sealed {}

	impl Sealed for ZeroStart {}
	impl Sealed for OneStart {}
	impl Sealed for DefaultStart {}

	impl<T> Sealed for SafeIncrement<T> {}
	impl<T> Sealed for WrappingIncrement<T> {}
}

// pub trait InfallibleIncrement<T>: Increment<T, Output = T> {
// 	fn increment(self) -> T {
// 		<Self as Increment>::increment(self)
// 	}
// }

// impl<_Self, T> InfallibleIncrement<T> for _Self where _Self: Increment<T, Output = T> {}

// pub trait TryIncrement<T>: Increment<T, Output = Result<T, Self::Error>> {
// 	type Error: Debug;

// 	fn increment(self) -> Result<T, Self::Error> {
// 		<Self as Increment>::increment(self)
// 	}
// }

// impl<_Self, T, E> TryIncrement<T> for _Self
// where
// 	_Self: Increment<T, Output = Result<T, Self::Error>>,
// {
// 	type Error = E;
// }
