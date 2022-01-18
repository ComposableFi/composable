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
#![warn(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use composable_traits::call_filter::{CallFilter, CallFilterEntry};
pub use pallet::*;
use sp_runtime::DispatchResult;
use sp_std::prelude::*;
use support::{
	dispatch::{CallMetadata, GetCallMetadata},
	pallet_prelude::*,
	traits::{Contains, PalletInfoAccess},
	transactional,
};
use system::pallet_prelude::*;
use weights::WeightInfo;

mod mock;
mod tests;
mod weights;

#[support::pallet]
pub mod pallet {
	use composable_traits::call_filter::CallFilterHook;

	use super::*;

	#[pallet::config]
	pub trait Config: system::Config {
		/// Overarching event type
		type Event: From<Event<Self>> + IsType<<Self as system::Config>::Event>;

		/// The origin which may set filter.
		type UpdateOrigin: EnsureOrigin<Self::Origin>;

		/// A hook that is able to block us from disabling/enabling an extrinsic.
		type Hook: CallFilterHook;

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
		Disabled { entry: CallFilterEntry },
		/// Unpaused transaction
		Enabled { entry: CallFilterEntry },
	}

	/// The list of disabled extrinsics.
	///
	/// CallFilterEntry -> ()
	#[pallet::storage]
	#[pallet::getter(fn disabled_calls)]
	pub type DisabledCalls<T: Config> = StorageMap<_, Twox64Concat, CallFilterEntry, ()>;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Disable a pallet function.
		///
		/// The dispatch origin for this call must be _Signed_ and the sender must be
		/// `UpdateOrigin`.
		///
		/// Possibly emits a `Disabled` event.
		#[pallet::weight(T::WeightInfo::disable())]
		#[transactional]
		pub fn disable(origin: OriginFor<T>, entry: CallFilterEntry) -> DispatchResult {
			T::UpdateOrigin::ensure_origin(origin)?;
			ensure!(entry.valid(), Error::<T>::InvalidString);
			// We are not allowed to disable this pallet.
			ensure!(
				entry.pallet_name != <Self as PalletInfoAccess>::name().as_bytes(),
				Error::<T>::CannotDisable
			);
			Self::do_disable(&entry)?;
			Ok(())
		}

		/// Enable a previously disabled pallet function.
		///
		/// The dispatch origin for this call must be _Signed_ and the sender must be
		/// `UpdateOrigin`.
		///
		/// Possibly emits an `Enabled` event.
		#[pallet::weight(T::WeightInfo::enable())]
		#[transactional]
		pub fn enable(origin: OriginFor<T>, entry: CallFilterEntry) -> DispatchResult {
			T::UpdateOrigin::ensure_origin(origin)?;
			ensure!(entry.valid(), Error::<T>::InvalidString);
			Self::do_enable(&entry)?;
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub(crate) fn disabled(entry: &CallFilterEntry) -> bool {
			DisabledCalls::<T>::contains_key(entry)
		}

		pub(crate) fn do_enable(entry: &CallFilterEntry) -> DispatchResult {
			if Self::disabled(entry) {
				T::Hook::enable_hook(entry)?;
				DisabledCalls::<T>::remove(entry);
				Self::deposit_event(Event::Enabled { entry: entry.clone() });
			}
			Ok(())
		}

		pub(crate) fn do_disable(entry: &CallFilterEntry) -> DispatchResult {
			if !Self::disabled(entry) {
				T::Hook::disable_hook(entry)?;
				DisabledCalls::<T>::insert(entry, ());
				Self::deposit_event(Event::Disabled { entry: entry.clone() });
			}
			Ok(())
		}
	}

	impl<T: Config> CallFilter for Pallet<T> {
		fn disabled(entry: &CallFilterEntry) -> bool {
			Self::disabled(entry)
		}
		fn enable(entry: &CallFilterEntry) -> DispatchResult {
			Self::do_enable(entry)
		}

		fn disable(entry: &CallFilterEntry) -> DispatchResult {
			Self::do_disable(entry)
		}
	}

	impl<T: Config> Contains<T::Call> for Pallet<T>
	where
		<T as system::Config>::Call: GetCallMetadata,
	{
		fn contains(call: &T::Call) -> bool {
			let CallMetadata { function_name, pallet_name } = call.get_call_metadata();
			DisabledCalls::<T>::contains_key(CallFilterEntry {
				pallet_name: pallet_name.as_bytes().to_vec(),
				function_name: function_name.as_bytes().to_vec(),
			})
		}
	}
}
