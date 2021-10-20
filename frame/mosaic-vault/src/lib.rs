#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {

	use frame_support::{pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_std::{fmt::Debug, vec::Vec}; 
	use codec::{Codec, FullCodec};
	use sp_runtime::{
         traits::{
			AtLeast32BitUnsigned, Convert
		 }
	};


	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);
	
	#[pallet::config]
    pub trait Config: frame_system::Config {

        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Converts the `Balance` type to `u128`, which internally is used in calculations.
		type Convert: Convert<Self::Balance, u128> + Convert<u128, Self::Balance>;

		type Balance: Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy + MaybeSerializeDeserialize + Debug + MaxEncodedLen + TypeInfo;

		// type Moment: Moment;

		type AssetId: FullCodec
		     + Eq
			 + PartialEq
			 + Copy
			 + MaybeSerializeDeserialize
			 + Debug
			 + Default;
		
		type RemoteAssetId: FullCodec
		    + Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default;
		
		type RemoteNetworkId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default;
	}

	
	#[pallet::storage]
	#[pallet::getter(fn remote_asset_id)]
    pub(super) type RemoteAssetId<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::RemoteNetworkId, Blake2_128Concat, T::AssetId, T::RemoteAssetId, ValueQuery >;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
 	pub enum Event<T: Config> {

		DepositCompleted(
   			T::AccountId, // sender
   			T::AssetId,   // assetId
			// T::RemoteAssetId, // remoteAssetId
			// T::RemoteNetworkId, // remoteNetworkId
			T::AccountId, // receiver
 			u128, // value
			Vec<u8>, // uniqueId
			u128 // transferDelay
		),

		WithdrawalCompleted(
		   T::AccountId, // receiver
           T::Balance, // amount
		   T::Balance, // receivedAmount
		   T::Balance, // feeAmount
		   T::AssetId, // assetId
		   Vec<u8>, // uniqueId
		),
 
        TokenAdded(
           T::AssetId, // asset
		   T::RemoteAssetId, // remoteAssetId
		   T::RemoteNetworkId // remoteNetworkId
		),

		TokenRemoved(
			T::AssetId, // assetId
			T::RemoteAssetId, // remoteAssetId
			T::RemoteNetworkId //remoteNetworkId
		)

	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000)]
		 pub fn add_supported_token(origin: OriginFor<T>, asset_id: T::AssetId, remote_asset_id: T::RemoteAssetId, remote_network_id: T::RemoteNetworkId ) -> DispatchResultWithPostInfo {
           
		   ensure_signed(origin)?; // -todo check admin permission 

		   <RemoteAssetId<T>>::insert(remote_network_id, asset_id, remote_asset_id);		

		  Self::deposit_event(Event::TokenAdded(asset_id, remote_asset_id, remote_network_id));

		   Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn remove_supported_token(origin: OriginFor<T>, asset_id: T::AssetId, remote_network_id: T::RemoteNetworkId) -> DispatchResultWithPostInfo {

			ensure_signed(origin)?; // -todo  check admin permission 
          
 		    let remote_asset_id = RemoteAssetId::<T>::get(remote_network_id, asset_id);

		    <RemoteAssetId<T>>::remove(remote_network_id, asset_id);

             Self::deposit_event(Event::TokenRemoved(asset_id, remote_asset_id,  remote_network_id));

			 Ok(().into())

		 }

	}

 }

 
 
