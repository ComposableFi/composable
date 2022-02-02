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

mod ranges;

#[frame_support::pallet]
pub mod pallet {
	use codec::FullCodec;
	use composable_traits::currency::{CurrencyFactory, DynamicCurrencyId, Exponent, LocalAssets};
	use frame_support::{pallet_prelude::*, PalletId};
	use scale_info::TypeInfo;
	use sp_runtime::traits::Saturating;

	use crate::ranges::Ranges;

	pub const PALLET_ID: PalletId = PalletId(*b"pal_curf");

	#[pallet::event]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type AssetId: FullCodec + Copy + TypeInfo + From<u128> + Saturating + Clone + Ord;
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
	pub type AssetIdRanges<T: Config> =
		StorageValue<_, Ranges<T::AssetId>, ValueQuery, RangesOnEmpty<T>>;

	#[pallet::type_value]
	pub fn RangesOnEmpty<T: Config>() -> Ranges<T::AssetId> {
		Ranges::new()
	}

	impl<T: Config> CurrencyFactory<T::AssetId> for Pallet<T> {
		fn create() -> Result<T::AssetId, DispatchError> {
			AssetIdRanges::<T>::mutate(|range| range.increment_tokens())
		}
	}

	impl<T: Config> LocalAssets<T::AssetId> for Pallet<T> {
		fn decimals(_currency_id: T::AssetId) -> Result<Exponent, DispatchError> {
			// All assets are normalized to 12 decimals.
			Ok(12)
		}
	}
}
