#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![deny(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

mod prelude;
mod types;
pub use pallet::*;
use support::{
	dispatch::{CallMetadata, GetCallMetadata},
	pallet_prelude::*,
	traits::{Contains, PalletInfoAccess},
	transactional,
};
use system::pallet_prelude::*;
pub use types::*;
use weights::WeightInfo;
mod mock;
mod tests;
mod weights;

#[support::pallet]
pub mod pallet {
	use super::*;
	use prelude::*;

	type CallFilterEntryOf<T> = CallFilterEntry<<T as Config>::MaxStringSize>;

	#[pallet::config]
	pub trait Config: system::Config {
		/// Overarching event type
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as system::Config>::RuntimeEvent>;

		type DisableOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		type EnableOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		// NOTE: can match by binary prefix which is much more efficient than string comparison.
		#[pallet::constant]
		type MaxStringSize: Get<u32>
			+ TypeInfo
			+ core::fmt::Debug
			+ MaxEncodedLen
			+ Copy
			+ Clone
			+ PartialEq
			+ Eq;

		/// A hook that is able to block us from disabling/enabling an extrinsic.
		type Hook: CallFilterHook<Self::MaxStringSize>;

		/// Weight information for the extrinsics in this module.
		type WeightInfo: WeightInfo;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// We tried to disable an extrinsic that cannot be disabled.
		CannotDisable,
		/// The pallet name is not a valid UTF8 string.
		InvalidString,
	}

	#[pallet::event]
	#[pallet::generate_deposit(fn deposit_event)]
	pub enum Event<T: Config> {
		/// Paused transaction
		Disabled { entry: CallFilterEntryOf<T> },
		/// Unpaused transaction
		Enabled { entry: CallFilterEntryOf<T> },
	}

	/// The list of disabled extrinsics.
	#[pallet::storage]
	#[pallet::getter(fn disabled_calls)]
	pub type DisabledCalls<T: Config> = StorageMap<_, Twox64Concat, CallFilterEntryOf<T>, ()>;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Disable a pallet function.
		///
		/// The dispatch origin for this call must be _Signed_ and the sender must be
		/// `DisableOrigin`.
		///
		/// Possibly emits a `Disabled` event.
		#[pallet::weight(T::WeightInfo::disable())]
		#[transactional]
		pub fn disable(origin: OriginFor<T>, entry: CallFilterEntryOf<T>) -> DispatchResult {
			T::DisableOrigin::ensure_origin(origin)?;
			ensure!(entry.valid(), Error::<T>::InvalidString);
			// We are not allowed to disable this pallet.
			ensure!(
				entry.pallet_name.to_vec() != <Self as PalletInfoAccess>::name().as_bytes(),
				Error::<T>::CannotDisable
			);
			Self::do_disable(&entry)?;
			Ok(())
		}

		/// Enable a previously disabled pallet function.
		///
		/// The dispatch origin for this call must be _Signed_ and the sender must be
		/// `EnableOrigin`.
		///
		/// Possibly emits an `Enabled` event.
		#[pallet::weight(T::WeightInfo::enable())]
		#[transactional]
		pub fn enable(origin: OriginFor<T>, entry: CallFilterEntryOf<T>) -> DispatchResult {
			T::EnableOrigin::ensure_origin(origin)?;
			ensure!(entry.valid(), Error::<T>::InvalidString);
			Self::do_enable(&entry)?;
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub(crate) fn disabled(entry: &CallFilterEntryOf<T>) -> bool {
			DisabledCalls::<T>::contains_key(entry)
		}

		pub(crate) fn do_enable(entry: &CallFilterEntryOf<T>) -> DispatchResult {
			if Self::disabled(entry) {
				T::Hook::enable_hook(entry)?;
				DisabledCalls::<T>::remove(entry);
				Self::deposit_event(Event::Enabled { entry: entry.clone() });
			}
			Ok(())
		}

		pub(crate) fn do_disable(entry: &CallFilterEntryOf<T>) -> DispatchResult {
			if entry.pallet_name ==
				BoundedVec::<_, T::MaxStringSize>::try_from(Self::name().as_bytes().to_vec())
					.expect("static pallet name cannot be too long")
			{
				return Err(Error::<T>::CannotDisable.into())
			}
			if !Self::disabled(entry) {
				T::Hook::disable_hook(entry)?;
				DisabledCalls::<T>::insert(entry, ());
				Self::deposit_event(Event::Disabled { entry: entry.clone() });
			}
			Ok(())
		}
	}

	impl<T: Config> CallFilter<T::MaxStringSize> for Pallet<T> {
		fn disabled(entry: &CallFilterEntryOf<T>) -> bool {
			Self::disabled(entry)
		}
		fn enable(entry: &CallFilterEntryOf<T>) -> DispatchResult {
			Self::do_enable(entry)
		}

		fn disable(entry: &CallFilterEntryOf<T>) -> DispatchResult {
			Self::do_disable(entry)
		}
	}

	impl<T: Config> Contains<T::RuntimeCall> for Pallet<T>
	where
		<T as system::Config>::RuntimeCall: GetCallMetadata,
	{
		fn contains(call: &T::RuntimeCall) -> bool {
			let CallMetadata { function_name, pallet_name } = call.get_call_metadata();
			match (
				BoundedVec::try_from(pallet_name.as_bytes().to_vec()),
				BoundedVec::try_from(function_name.as_bytes().to_vec()),
			) {
				(Ok(pallet_name), Ok(function_name)) =>
					DisabledCalls::<T>::contains_key(CallFilterEntry { pallet_name, function_name }),
				_ => false,
			}
		}
	}
}
