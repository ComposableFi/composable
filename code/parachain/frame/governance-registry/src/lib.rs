// TODO: move this pallet stuff into currency-factory (check if used on picasso and may need to
// route from this pallet to factory)
// TODO: prove it supports `collective`(CallerOrigin) (which allows Council and other more
// complicated custom) https://app.clickup.com/t/37h4edu
//! # Overview
//! Allows root (or entity acting as root) to set origin for relevant token.
//! The origin can be used to enact preimages if voted using specific token or update some asset
//! parameters.

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

pub use pallet::*;

pub mod weights;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use composable_traits::{
		currency::AssetIdLike,
		governance::{GovernanceRegistry, SignedRawOrigin},
	};
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::{ensure_root, pallet_prelude::OriginFor};

	use crate::weights::WeightInfo;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type AssetId: AssetIdLike + Decode + MaxEncodedLen + Clone + core::fmt::Debug + Default;
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	type OriginsByAssetId<T: Config> =
		StorageMap<_, Twox64Concat, T::AssetId, SignedRawOrigin<T::AccountId>, OptionQuery>;

	#[pallet::error]
	pub enum Error<T> {
		/// Not found
		NoneError,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Set { asset_id: T::AssetId, value: T::AccountId },
		GrantRoot { asset_id: T::AssetId },
		Remove { asset_id: T::AssetId },
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Sets the value of an `asset_id` to the signed account id. Only callable by root.
		#[pallet::weight(T::WeightInfo::set())]
		pub fn set(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			value: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			OriginsByAssetId::<T>::insert(asset_id, SignedRawOrigin::Signed(value.clone()));
			Self::deposit_event(Event::<T>::Set { asset_id, value });
			Ok(().into())
		}

		/// Sets the value of an `asset_id` to root. Only callable by root.
		#[pallet::weight(T::WeightInfo::grant_root())]
		pub fn grant_root(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			OriginsByAssetId::<T>::insert(asset_id, SignedRawOrigin::Root);
			Self::deposit_event(Event::<T>::GrantRoot { asset_id });
			Ok(().into())
		}

		/// Removes mapping of an `asset_id`. Only callable by root.
		#[pallet::weight(T::WeightInfo::remove())]
		pub fn remove(origin: OriginFor<T>, asset_id: T::AssetId) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			OriginsByAssetId::<T>::remove(asset_id);
			Self::deposit_event(Event::<T>::Remove { asset_id });
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Gets the origin associated with the asset.
		///
		/// # Errors
		///  - When the `asset_id` has no associated mapping
		pub fn get(asset_id: &T::AssetId) -> Result<SignedRawOrigin<T::AccountId>, Error<T>> {
			OriginsByAssetId::<T>::get(asset_id).ok_or(Error::<T>::NoneError)
		}
	}

	impl<T: Config>
		orml_traits::GetByKey<T::AssetId, Result<SignedRawOrigin<T::AccountId>, DispatchError>>
		for Pallet<T>
	{
		fn get(k: &T::AssetId) -> Result<SignedRawOrigin<T::AccountId>, DispatchError> {
			Self::get(k).map_err(Into::into)
		}
	}

	impl<T: Config> GovernanceRegistry<T::AssetId, T::AccountId> for Pallet<T> {
		fn set(k: T::AssetId, v: SignedRawOrigin<T::AccountId>) {
			OriginsByAssetId::<T>::insert(k, v);
		}
	}
}
