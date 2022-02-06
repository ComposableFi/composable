//! Overview
//! Allows to add new assets internally. User facing mutating API is provided by other pallets.
#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_method,
		clippy::disallowed_type,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_type)]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
	bad_style,
	bare_trait_objects,
	const_err,
	improper_ctypes,
	non_shorthand_field_patterns,
	no_mangle_generic_items,
	overflowing_literals,
	path_statements,
	patterns_in_fns_without_body,
	private_in_public,
	unconditional_recursion,
	unused_allocation,
	unused_comparisons,
	unused_parens,
	while_true,
	trivial_casts,
	trivial_numeric_casts,
	unused_extern_crates
)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use codec::FullCodec;
	use composable_traits::currency::{CurrencyFactory, DynamicCurrencyId, Exponent, LocalAssets};
	use frame_support::{pallet_prelude::*, PalletId};
	use scale_info::TypeInfo;

	pub const PALLET_ID: PalletId = PalletId(*b"pal_curf");

	#[pallet::event]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The currency which can be created from thin air.
		type DynamicCurrencyId: FullCodec + MaxEncodedLen + Copy + DynamicCurrencyId + TypeInfo;

		/// The initial currency id from which we are able to generate the next.
		#[pallet::constant]
		type DynamicCurrencyIdInitial: Get<Self::DynamicCurrencyId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// The counter that track the latest generated currency id.
	#[pallet::storage]
	#[pallet::getter(fn currency_latest)]
	// Absense of a set `CurrencyCounter` means we default to `T::DynamicCurrencyIdInitial`, so
	// `ValueQuery` is allowed
	#[allow(clippy::disallowed_type)]
	pub type CurrencyCounter<T: Config> =
		StorageValue<_, T::DynamicCurrencyId, ValueQuery, T::DynamicCurrencyIdInitial>;

	impl<T: Config> CurrencyFactory<T::DynamicCurrencyId> for Pallet<T> {
		fn create() -> Result<T::DynamicCurrencyId, DispatchError> {
			CurrencyCounter::<T>::mutate(|c| {
				let c_current = *c;
				let c_next = c_current.next()?;
				*c = c_next;
				Ok(c_next)
			})
		}
	}

	impl<T: Config> LocalAssets<T::DynamicCurrencyId> for Pallet<T> {
		fn decimals(_currency_id: T::DynamicCurrencyId) -> Result<Exponent, DispatchError> {
			// All assets are normalized to 12 decimals.
			Ok(12)
		}
	}
}
