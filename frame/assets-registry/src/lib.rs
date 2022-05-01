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
#![deny(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

pub use pallet::*;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	pub use crate::weights::WeightInfo;
	use codec::{EncodeLike, FullCodec};
	use composable_traits::{ currency::{Exponent, BalanceLike, CurrencyFactory, RangeId}, xcm::assets::{RemoteAssetRegistryInspect, ForeignMetadata, XcmAssetLocation, RemoteAssetRegistryMutate}, defi::Ratio};
	use frame_support::{
		dispatch::DispatchResultWithPostInfo, pallet_prelude::*, traits::EnsureOrigin,
	};

	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_std::{fmt::Debug, marker::PhantomData, str};

	/// The module configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Identifier for the class of local asset.
		type LocalAssetId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ From<u128>
			+ Into<u128>
			+ Debug
			+ Default
			+ TypeInfo;

		/// Identifier for the class of foreign asset.
		type ForeignAssetId: FullCodec
			+ Eq
			+ PartialEq
			+ MaybeSerializeDeserialize
			+ Debug
			+ Clone
			+ Default
			+ TypeInfo;

		/// The origin which may set local and foreign admins.
		type UpdateAdminOrigin: EnsureOrigin<Self::Origin>;
		type WeightInfo: WeightInfo;
		type Balance : BalanceLike;
		type CurrencyFactory: CurrencyFactory<Self::LocalAssetId, Self::Balance>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Mapping local asset to foreign asset.
	#[pallet::storage]
	#[pallet::getter(fn from_local_asset)]
	pub type LocalToForeign<T: Config> =
		StorageMap<_, Twox128, T::LocalAssetId, ForeignMetadata<T::ForeignAssetId>, OptionQuery>;

	/// Mapping foreign asset to local asset.
	#[pallet::storage]
	#[pallet::getter(fn from_foreign_asset)]
	pub type ForeignToLocal<T: Config> =
		StorageMap<_, Blake2_128Concat, T::ForeignAssetId, T::LocalAssetId, OptionQuery>;

	/// Mapping (local asset, foreign asset) to a candidate status.
	#[pallet::storage]
	#[pallet::getter(fn asset_ratio)]
	pub type AssetRatio<T: Config> =
			StorageMap<_, Twox128, T::LocalAssetId,  Ratio, OptionQuery>;
	
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config>(PhantomData<T>);

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self(<_>::default())
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T>
	where
		XcmAssetLocation: EncodeLike<<T as Config>::ForeignAssetId>,
	{
		fn build(&self) {
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AssetRegistered {
			asset_id : T::LocalAssetId,
			location: T::ForeignAssetId,
		},
		AssetUpdated {
			asset_id : T::LocalAssetId,
			location: T::ForeignAssetId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		AssetNotFound,
		ForeignAssetAlreadyRegistered,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {		

		/// creates asset using `CurrencyFactory`,
		/// raises `AssetRegistered` event
		#[pallet::weight(<T as Config>::WeightInfo::register_asset())]
		pub fn register_asset(
			origin: OriginFor<T>,
			location: T::ForeignAssetId, 
			ed: T::Balance,
			ratio: Option<Ratio>, decimals: Option<Exponent>,
		) -> DispatchResultWithPostInfo {
			T::UpdateAdminOrigin::ensure_origin(origin)?;
			if ForeignToLocal::<T>::contains_key(&location){
				return Err(Error::<T>::ForeignAssetAlreadyRegistered.into())
			}
			let asset_id = T::CurrencyFactory::create(RangeId::FOREIGN_ASSETS, ed)?;
			Self::set_reserve_location(asset_id, location.clone(), ratio, decimals)?;		
			Self::deposit_event(Event::<T>::AssetRegistered { asset_id, location });
			Ok(().into())
		}

		/// given well existing asset, update its remote information
		/// use with caution
		#[pallet::weight(<T as Config>::WeightInfo::update_asset())]
		pub fn update_asset(
			origin: OriginFor<T>,
			asset_id: T::LocalAssetId, 
			location: T::ForeignAssetId, 
			ratio: Option<Ratio>, decimals: Option<Exponent>,
		) -> DispatchResultWithPostInfo {
			T::UpdateAdminOrigin::ensure_origin(origin)?;
			// note: does not validates if assets exists, not clear what is expected in this case
			Self::set_reserve_location(asset_id, location.clone(), ratio, decimals)?;		
			Self::deposit_event(Event::<T>::AssetUpdated { asset_id, location });
			Ok(().into())
		}
	}

	impl <T:Config> RemoteAssetRegistryMutate for Pallet<T> {
    type AssetId = T::LocalAssetId;

    type AssetNativeLocation = T::ForeignAssetId;

    type Balance = T::Balance;

    fn set_reserve_location(asset_id: Self::AssetId, location: Self::AssetNativeLocation, ratio: Option<Ratio>, decimals: Option<Exponent>) 
		-> DispatchResult {
			ForeignToLocal::<T>::insert(&location, asset_id );
			LocalToForeign::<T>::insert(asset_id, ForeignMetadata { decimals, location});
			AssetRatio::<T>::mutate_exists(asset_id, |x| *x = ratio);
			Ok(())
    }

    fn update_ratio(location: Self::AssetNativeLocation, ratio: Option<Ratio>) -> DispatchResult {
        if let Some(asset_id) = ForeignToLocal::<T>::get(location) { 
				AssetRatio::<T>::mutate_exists(asset_id, |x| *x = ratio);
				Ok(())
			}
			else {
				Err(Error::<T>::AssetNotFound.into())
			}
    	}
	}

	impl<T: Config> RemoteAssetRegistryInspect for Pallet<T> {
		type AssetId = T::LocalAssetId;
		type AssetNativeLocation = T::ForeignAssetId;

		fn asset_to_remote(asset_id: Self::AssetId) -> Option<composable_traits::xcm::assets::ForeignMetadata<Self::AssetNativeLocation>> {
				LocalToForeign::<T>::get(asset_id)
		}

		fn get_ratio(asset_id: Self::AssetId) -> Option<composable_traits::defi::Ratio> {
				AssetRatio::<T>::get(asset_id)
			}

		fn location_to_asset(location: Self::AssetNativeLocation) -> Option<Self::AssetId> {
				ForeignToLocal::<T>::get(location)
		}
	}
}
