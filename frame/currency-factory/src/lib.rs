//! Overview
//! Allows to add new assets internally. User facing mutating API is provided by other pallets.
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
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
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
pub use weights::{SubstrateWeight, WeightInfo};

mod ranges;
mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod mocks;
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use crate::{
		ranges::{Range, RangeId, Ranges},
		weights::WeightInfo,
	};
	use codec::FullCodec;
	use composable_traits::currency::{CurrencyFactory, Exponent, LocalAssets};
	use frame_support::{pallet_prelude::*, traits::EnsureOrigin, PalletId};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_runtime::{
		traits::{CheckedAdd, Saturating},
		DispatchError,
	};

	pub const PALLET_ID: PalletId = PalletId(*b"pal_curf");

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		RangeCreated { range: Range<T::AssetId> },
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The currency which can be created from thin air.
		type AssetId: FullCodec
			+ Copy
			+ TypeInfo
			+ From<u128>
			+ Saturating
			+ Clone
			+ Ord
			+ CheckedAdd
			+ core::fmt::Debug
			+ MaxEncodedLen;

		type AddOrigin: EnsureOrigin<Self::Origin>;
		type ReserveOrigin: EnsureOrigin<Self::Origin>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn asset_id_rages)]
	// Storage is intialized using RangesOnEmpty, so ValueQuery is allowed.
	#[allow(clippy::disallowed_types)]
	pub type AssetIdRanges<T: Config> =
		StorageValue<_, Ranges<T::AssetId>, ValueQuery, RangesOnEmpty<T>>;

	#[pallet::type_value]
	pub fn RangesOnEmpty<T: Config>() -> Ranges<T::AssetId> {
		Ranges::new()
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::add_range())]
		pub fn add_range(origin: OriginFor<T>, length: u64) -> DispatchResultWithPostInfo {
			T::AddOrigin::ensure_origin(origin)?;
			if let Some(range) = AssetIdRanges::<T>::try_mutate(
				|range| -> Result<Option<Range<T::AssetId>>, DispatchError> {
					range.append(length.into())?;
					Ok(range.last().cloned())
				},
			)? {
				Self::deposit_event(Event::<T>::RangeCreated { range })
			}

			Ok(().into())
		}
	}

	impl<T: Config> CurrencyFactory<T::AssetId> for Pallet<T> {
		fn create(id: RangeId) -> Result<T::AssetId, DispatchError> {
			AssetIdRanges::<T>::mutate(|range| range.increment(id))
		}
	}

	impl<T: Config> LocalAssets<T::AssetId> for Pallet<T> {
		fn decimals(_currency_id: T::AssetId) -> Result<Exponent, DispatchError> {
			// All assets are normalized to 12 decimals.
			Ok(12)
		}
	}
}
