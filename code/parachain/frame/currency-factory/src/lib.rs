#![cfg_attr(
	not(test),
	deny(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![deny(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
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
#![doc = include_str!("../README.md")]

pub use pallet::*;
pub use weights::{SubstrateWeight, WeightInfo};

mod ranges;
mod weights;

#[cfg(any(feature = "runtime-benchmarks", test))]
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
	use composable_traits::{
		assets::BasicAssetMetadata,
		currency::{
			AssetExistentialDepositInspect, AssetIdLike, BalanceLike, CurrencyFactory, Exponent,
			LocalAssets,
		},
	};
	use frame_support::{pallet_prelude::*, traits::EnsureOrigin, transactional, PalletId};
	use frame_system::pallet_prelude::*;
	use sp_runtime::{
		traits::{CheckedAdd, Saturating},
		DispatchError,
	};

	// cspell:disable-next
	pub const PALLET_ID: PalletId = PalletId(*b"pal_curf");

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		RangeCreated { range: Range<T::AssetId> },
	}

	#[pallet::error]
	pub enum Error<T> {
		AssetNotFound,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The currency which can be created from thin air.
		type AssetId: AssetIdLike
			+ From<u128>
			+ Into<u128>
			+ Saturating
			+ Ord
			+ CheckedAdd
			+ core::fmt::Debug
			+ MaxEncodedLen;

		type Balance: BalanceLike;

		///  can add new ranges or assign metadata
		type AddOrigin: EnsureOrigin<Self::Origin>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn asset_id_rages)]
	// Storage is initialized using RangesOnEmpty, so ValueQuery is allowed.
	#[allow(clippy::disallowed_types)]
	pub type AssetIdRanges<T: Config> =
		StorageValue<_, Ranges<T::AssetId>, ValueQuery, RangesOnEmpty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn get_assets_ed)]
	pub type AssetEd<T: Config> = StorageMap<_, Twox128, T::AssetId, T::Balance, OptionQuery>;

	// technically that can be stored offchain, but other parachains do int on chain too (and some
	// other blockchains) also may do routing for approved symbols based, not on ids, too
	#[pallet::storage]
	#[pallet::getter(fn get_assets_metadata)]
	pub type AssetMetadata<T: Config> = StorageMap<
		_,
		Twox128,
		T::AssetId,
		composable_traits::assets::BasicAssetMetadata,
		OptionQuery,
	>;

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

		/// Sets metadata
		#[pallet::weight(T::WeightInfo::set_metadata())]
		pub fn set_metadata(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			metadata: BasicAssetMetadata,
		) -> DispatchResultWithPostInfo {
			T::AddOrigin::ensure_origin(origin)?;
			if AssetEd::<T>::get(asset_id).is_some() {
				// note: if will decide to build route on symbol, than better to make second map
				// from symbol to asset to check unique
				AssetMetadata::<T>::insert(asset_id, metadata);
				Ok(().into())
			} else {
				Err(Error::<T>::AssetNotFound.into())
			}
		}
	}

	impl<T: Config> CurrencyFactory for Pallet<T> {
		type AssetId = T::AssetId;
		type Balance = T::Balance;

		#[transactional]
		fn create(id: RangeId, ed: Self::Balance) -> Result<Self::AssetId, DispatchError> {
			let asset_id = AssetIdRanges::<T>::mutate(|range| range.increment(id))?;
			AssetEd::<T>::insert(asset_id, ed);
			Ok(asset_id)
		}

		fn protocol_asset_id_to_unique_asset_id(
			protocol_asset_id: u32,
			range_id: RangeId,
		) -> Result<Self::AssetId, DispatchError> {
			if range_id.inner() > 5 {
				Err(DispatchError::from("RangeId outside of preconfigured ranges!"))
			} else {
				Ok(((u32::MAX as u128).saturating_mul(range_id.inner() as u128 + 1) +
					protocol_asset_id as u128)
					.into())
			}
		}

		fn unique_asset_id_to_protocol_asset_id(unique_asset_id: Self::AssetId) -> u32 {
			u32::try_from(unique_asset_id.into() % u32::MAX as u128)
				.expect("u128 is made of u32 chunks")
		}
	}

	impl<T: Config> AssetExistentialDepositInspect for Pallet<T> {
		type AssetId = T::AssetId;
		type Balance = T::Balance;

		fn existential_deposit(asset_id: Self::AssetId) -> Result<Self::Balance, DispatchError> {
			AssetEd::<T>::get(asset_id).ok_or_else(|| Error::<T>::AssetNotFound.into())
		}
	}

	impl<T: Config> LocalAssets<T::AssetId> for Pallet<T> {
		fn decimals(_currency_id: T::AssetId) -> Result<Exponent, DispatchError> {
			// All assets are normalized to 12 decimals.
			Ok(12)
		}
	}
}
