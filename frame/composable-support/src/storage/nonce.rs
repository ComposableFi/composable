use crate::math::safe::SafeAdd;

use codec::FullCodec;
use frame_support::{
	pallet_prelude::{StorageValue, ValueQuery},
	traits::StorageInstance,
};
use sp_runtime::{
	traits::{One, Zero},
	ArithmeticError,
};

use start_at::{StartAt, StartAtValue, ZeroStart};

/// Extension trait for storage nonces.
pub trait StorageNonce<T, S: StartAtValue<T>> {
	/// Attempt to increment the nonce. Returns the new value if successful.
	fn try_increment() -> Result<T, ArithmeticError>;
}

impl<Prefix, T, S: StartAtValue<T>> StorageNonce<T, S>
	for StorageValue<Prefix, T, ValueQuery, start_at::StartAt<S, T>>
where
	Prefix: StorageInstance,
	T: FullCodec + SafeAdd + Copy + 'static + Zero + One,
{
	fn try_increment() -> Result<T, ArithmeticError> {
		Self::try_mutate(|x| {
			*x = x.safe_add(&T::one())?;
			Ok(*x)
		})
	}
}

pub type StartAtZero<T> = StartAt<start_at::ZeroStart, T>;
pub type StartAtOne<T> = StartAt<start_at::OneStart, T>;

mod start_at {
	use sp_std::marker::PhantomData;

	use frame_support::traits::Get;
	use sp_runtime::traits::{One, Zero};

	/// Defines what a nonce should start at.
	///
	/// # Example
	///
	/// A nonce that starts at zero:
	///
	/// ```rust,skip
	/// #[pallet::storage]
	/// pub type SomeNonce<T: Config> = StorageValue<_, T::Something, ValueQuery, StartAt<ZeroStart>;
	/// ```
	///
	/// Increment the nonce in an extrinsic:
	///
	/// ```rust,skip
	/// #[pallet::call]
	/// impl<T: Config> Pallet<T> {
	/// 	pub fn extrinsic(
	/// 		origin: OriginFor<T>,
	/// 	) -> DispatchResultWithPostInfo {
	/// 		let nonce_next = SomeNonce::try_increment()?;
	/// 	}
	/// }
	/// ```
	pub struct StartAt<S: StartAtValue<T>, T> {
		#[doc(hidden)]
		_marker: PhantomData<(S, T)>,
	}

	impl<S: StartAtValue<T>, T: Zero + One> Get<T> for StartAt<S, T> {
		fn get() -> T {
			S::value()
		}
	}

	pub trait StartAtValue<T>: private::Sealed + 'static {
		fn value() -> T;
	}

	/// Storage starts at `0`.
	pub struct ZeroStart;

	impl<T: Zero> StartAtValue<T> for ZeroStart {
		fn value() -> T {
			T::zero()
		}
	}

	/// Storage starts at `1`.
	pub struct OneStart;

	impl<T: One> StartAtValue<T> for OneStart {
		fn value() -> T {
			T::one()
		}
	}

	mod private {
		use super::{OneStart, ZeroStart};

		pub trait Sealed {}

		impl Sealed for OneStart {}
		impl Sealed for ZeroStart {}
	}
}
