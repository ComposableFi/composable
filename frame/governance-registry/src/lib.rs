#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod weights;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use codec::FullCodec;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::{ensure_root, pallet_prelude::OriginFor};
	use scale_info::TypeInfo;

	use crate::weights::WeightInfo;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type AssetId: Encode
			+ Decode
			+ Clone
			+ core::fmt::Debug
			+ Eq
			+ Default
			+ TypeInfo
			+ FullCodec;
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	type OriginsByAssetId<T: Config> =
		StorageMap<_, Twox64Concat, T::AssetId, StorageOrigin<T::AccountId>, ValueQuery>;

	#[pallet::error]
	pub enum Error<T> {
		NoneError,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Set { asset: T::AssetId, value: T::AccountId },
		GrantRoot { asset: T::AssetId },
		Remove { asset: T::AssetId },
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Sets the value of an asset to the signed account id. Only callable by root.
		#[pallet::weight(T::WeightInfo::set())]
		pub fn set(
			origin: OriginFor<T>,
			asset: T::AssetId,
			value: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			OriginsByAssetId::<T>::insert(asset.clone(), StorageOrigin::Signed(value.clone()));
			Self::deposit_event(Event::<T>::Set { asset, value });
			Ok(().into())
		}

		/// Sets the value of an asset to root. Only callable by root.
		#[pallet::weight(T::WeightInfo::grant_root())]
		pub fn grant_root(origin: OriginFor<T>, asset: T::AssetId) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			OriginsByAssetId::<T>::insert(asset.clone(), StorageOrigin::Root);
			Self::deposit_event(Event::<T>::GrantRoot { asset });
			Ok(().into())
		}

		/// Removes mapping of an asset_id. Only callable by root.
		#[pallet::weight(T::WeightInfo::remove())]
		pub fn remove(origin: OriginFor<T>, asset: T::AssetId) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			OriginsByAssetId::<T>::remove(asset.clone());
			Self::deposit_event(Event::<T>::Remove { asset });
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Gets the orgin associated with the asset. 
		/// 
		/// # Errors
		///  - When the asset has no associated mapping
		pub fn get(asset: &T::AssetId) -> Result<frame_system::RawOrigin<T::AccountId>, Error<T>> {
			let res = OriginsByAssetId::<T>::try_get(asset)
				.map(|s| match s {
					StorageOrigin::Root => Some(frame_system::RawOrigin::Root),
					StorageOrigin::Signed(account) =>
						Some(frame_system::RawOrigin::Signed(account)),
					_ => {
						debug_assert!(false, "invalid storage, storing StorageOrigin::Invalid");
						None
					},
				})
				.map_err(|_| Error::<T>::NoneError)?;

			match res {
				None => Err(Error::<T>::NoneError),
				Some(res) => Ok(res),
			}
		}
	}

	#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
	enum StorageOrigin<AccountId> {
		#[doc(hidden)]
		Invalid,
		Root,
		Signed(AccountId),
	}

	// WARNING: only used since the Default bound is needed for `StorageMap`.
	impl<T> Default for StorageOrigin<T> {
		fn default() -> Self {
			Self::Invalid
		}
	}

	impl<T: Config>
		orml_traits::GetByKey<T::AssetId, Result<frame_system::RawOrigin<T::AccountId>, Error<T>>>
		for Pallet<T>
	{
		fn get(k: &T::AssetId) -> Result<frame_system::RawOrigin<T::AccountId>, Error<T>> {
			Self::get(k)
		}
	}

	impl<T: Config> composable_traits::SetByKey<T::AssetId, frame_system::RawOrigin<T::AccountId>>
		for Pallet<T>
	{
		fn set(
			k: &T::AssetId,
			v: frame_system::RawOrigin<T::AccountId>,
		) -> Result<(), frame_system::RawOrigin<T::AccountId>> {
			let value = match v {
				frame_system::RawOrigin::None => return Err(v),
				frame_system::RawOrigin::Root => StorageOrigin::Root,
				frame_system::RawOrigin::Signed(acc) => StorageOrigin::Signed(acc),
			};
			OriginsByAssetId::<T>::insert(k, value);
			Ok(())
		}
	}
}
