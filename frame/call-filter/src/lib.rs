#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

pub use pallet::*;
use sp_runtime::DispatchResult;
use sp_std::{prelude::*, vec::Vec};
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
	use super::*;

	#[pallet::config]
	pub trait Config: system::Config {
		/// Overarching event type
		type Event: From<Event<Self>> + IsType<<Self as system::Config>::Event>;

		/// The origin which may set filter.
		type UpdateOrigin: EnsureOrigin<Self::Origin>;

		// Weight information for the extrinsics in this module.
		type WeightInfo: WeightInfo;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// can not pause
		CannotPause,
		/// invalid character encoding
		InvalidCharacter,
	}

	#[pallet::event]
	#[pallet::generate_deposit(fn deposit_event)]
	pub enum Event<T: Config> {
		/// Paused transaction . \[pallet_name_bytes, function_name_bytes\]
		Disabled(Vec<u8>, Vec<u8>),
		/// Unpaused transaction . \[pallet_name_bytes, function_name_bytes\]
		TransactionUnpaused(Vec<u8>, Vec<u8>),
	}

	/// The paused transaction map
	///
	/// map (PalletNameBytes, FunctionNameBytes) => Option<()>
	#[pallet::storage]
	#[pallet::getter(fn disabled_calls)]
	pub type PausedTransactions<T: Config> =
		StorageMap<_, Twox64Concat, (Vec<u8>, Vec<u8>), (), OptionQuery>;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::disable())]
		#[transactional]
		pub fn disable(
			origin: OriginFor<T>,
			pallet_name: Vec<u8>,
			function_name: Vec<u8>,
		) -> DispatchResult {
			T::UpdateOrigin::ensure_origin(origin)?;

			// not allowed to pause calls of this pallet to ensure safe
			let pallet_name_string =
				sp_std::str::from_utf8(&pallet_name).map_err(|_| Error::<T>::InvalidCharacter)?;
			ensure!(
				pallet_name_string != <Self as PalletInfoAccess>::name(),
				Error::<T>::CannotPause
			);

			PausedTransactions::<T>::mutate_exists(
				(pallet_name.clone(), function_name.clone()),
				|maybe_paused| {
					if maybe_paused.is_none() {
						*maybe_paused = Some(());
						Self::deposit_event(Event::Disabled(pallet_name, function_name));
					}
				},
			);
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::enable())]
		#[transactional]
		pub fn enable(
			origin: OriginFor<T>,
			pallet_name: Vec<u8>,
			function_name: Vec<u8>,
		) -> DispatchResult {
			T::UpdateOrigin::ensure_origin(origin)?;
			if PausedTransactions::<T>::take((&pallet_name, &function_name)).is_some() {
				Self::deposit_event(Event::TransactionUnpaused(pallet_name, function_name));
			};
			Ok(())
		}
	}
}

pub struct CallFilter<T>(sp_std::marker::PhantomData<T>);
impl<T: Config> Contains<T::Call> for CallFilter<T>
where
	<T as system::Config>::Call: GetCallMetadata,
{
	fn contains(call: &T::Call) -> bool {
		let CallMetadata { function_name, pallet_name } = call.get_call_metadata();
		PausedTransactions::<T>::contains_key((pallet_name.as_bytes(), function_name.as_bytes()))
	}
}
