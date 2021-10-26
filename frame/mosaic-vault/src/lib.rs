#![cfg_attr(not(feature = "std"), no_std)]

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	use frame_support::{
		ensure,
		pallet_prelude::*,
		traits::fungibles::{Inspect, Mutate, Transfer},
		PalletId,
	};

	use frame_system::pallet_prelude::*;
 	use scale_info::TypeInfo;
	use sp_std::{fmt::Debug, vec::Vec}; 
	use codec::{Codec, FullCodec};
	use sp_runtime::{
         traits::{
			AtLeast32BitUnsigned, Convert, AccountIdConversion, Saturating
		 },
		  Perquintill,
	};

	use composable_traits::{loans::Timestamp, vault::{Deposit, StrategicVault, Vault, VaultConfig }};

	// use sp_runtime::traits::AccountIdConversion;
	
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);
	
	#[pallet::config]
    pub trait Config: frame_system::Config {

        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
		     + Mutate<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>;

		type Convert: Convert<Self::Balance, u128> + Convert<u128, Self::Balance>;

		type Balance: Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy + MaybeSerializeDeserialize + Debug + MaxEncodedLen + TypeInfo;

		type TransferDelay:  Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy + MaybeSerializeDeserialize + Debug + MaxEncodedLen + TypeInfo;

		type VaultId: Clone + Codec + Debug + PartialEq + Default + Parameter;

		type Vault: StrategicVault<
			VaultId = Self::VaultId,
			AssetId = <Self as Config>::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,>;
		// type Moment: Moment;

		type AssetId: FullCodec
		     + Eq
			 + PartialEq
			 + Copy
			 + MaybeSerializeDeserialize
			 + Debug
			 + Default
			 + TypeInfo;
		
		type RemoteAssetId: FullCodec
			 + Eq
			 + PartialEq
			 + Copy
			 + MaybeSerializeDeserialize
			 + Debug
			 + Default
			 + TypeInfo;
		
		type RemoteNetworkId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ TypeInfo;

		#[pallet::constant]
		type FeeFactor: Get<Self::Balance>;

		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}
	
	#[pallet::storage]
	#[pallet::getter(fn remote_asset_id)]
    pub(super) type RemoteAssetId<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::RemoteNetworkId, Blake2_128Concat, T::AssetId, T::RemoteAssetId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn max_asset_transfer_size)]
	pub(super) type MaxAssetTransferSize<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, T::Balance, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn min_asset_transfer_size)]
	pub(super) type MinAssetTransferSize<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, T::Balance, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn max_transfer_delay)]
	pub(super) type MaxTransferDelay<T: Config> = StorageValue<_, T::TransferDelay, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn min_transfer_delay)]
	pub(super) type MinTransferDelay<T: Config> =  StorageValue<_, T::TransferDelay, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn last_transfer)]
	pub(super) type LastTransfer<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Timestamp, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn transfer_lockup_time)]
	pub(super) type TransferLockupTime<T: Config> = StorageValue<_, Timestamp, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn max_fee)]
	pub(super) type MaxFee<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn min_fee)]
	pub(super) type MinFee<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn asset_vault)]
	pub(super) type AssetVault<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, T::VaultId, ValueQuery>;
	
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
		),

		MaxTransferDelayChanged(
			T::TransferDelay,
		),

		MinTransferDelayChanged(
			T::TransferDelay,
		),

		AssetMaxTransferSizeChanged(
			T::AssetId,
			T::Balance,
		),

		AssetMinTransferSizeChanged(
			T::AssetId,
			T::Balance,
		),

		LockupTimeChanged(
			T::AccountId,
			Timestamp, // old lockup time
			Timestamp, // new lockup time
			Vec<u8>, //
		),

		MinFeeChanged(
			T::Balance,
		),

		MaxFeeChanged(
			T::Balance,
		),

		VaultCreated(
			T:: AccountId, // asset manager
			T::AssetId, // asset id
			T::VaultId, // vault id
			Perquintill, // reserved factor
		)

	}

	#[allow(missing_docs)]
	#[pallet::error]
	pub enum Error<T> {
		/// Minting failures result in `MintFailed`. In general this should never occur.
		MintFailed,
		/// 
		BurnFailed,
		///
		MaxAssetTransferSizeBelowMinimum,
		///
		TransferDelayAboveMaximum,
		///
		TransferDelayBelowMinimum,
		/// max_asset_transfer_size
		AmountAboveMaxAssetTransferSize,
		/// 
		AmountBelowMaxAssetTransferSize,
		///
		MaxTransferDelayBelowMinimum,
		///
		MinTransferDelayAboveMaximum,
		///
	    MinFeeAboveFeeFactor,
		///
		MaxFeeAboveFeeFactor,
		/// 
		MinFeeAboveMaxFee,
		/// 
		MaxFeeBelowMinFee,
	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000)]
		 pub fn add_supported_token(origin: OriginFor<T>, 
			asset_id: T::AssetId, 
			remote_asset_id: T::RemoteAssetId, 
			remote_network_id: T::RemoteNetworkId, 
			max_asset_transfer_size: T::Balance,
			min_asset_transfer_size: T::Balance,) -> DispatchResultWithPostInfo {
           
		   ensure_signed(origin)?; // -todo check admin permission 

		   ensure!(max_asset_transfer_size > min_asset_transfer_size, Error::<T>::MaxAssetTransferSizeBelowMinimum);

		   <RemoteAssetId<T>>::insert(remote_network_id, asset_id, remote_asset_id);	
		   
		   <MaxAssetTransferSize<T>>::insert(asset_id, max_asset_transfer_size);

		   <MinAssetTransferSize<T>>::insert(asset_id, min_asset_transfer_size);

		   Self::deposit_event(Event::TokenAdded(asset_id, remote_asset_id, remote_network_id));

		   Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn remove_supported_token(origin: OriginFor<T>, asset_id: T::AssetId, remote_network_id: T::RemoteNetworkId) -> DispatchResultWithPostInfo {

			ensure_signed(origin)?; // -todo  check admin permission 
          
 		    let remote_asset_id = RemoteAssetId::<T>::get(remote_network_id, asset_id);

		     <RemoteAssetId<T>>::remove(remote_network_id, asset_id);

			 <MaxAssetTransferSize<T>>::remove(asset_id);

			 <MinAssetTransferSize<T>>::remove(asset_id);

             Self::deposit_event(Event::TokenRemoved(asset_id, remote_asset_id,  remote_network_id));

			 Ok(().into())

		 }

		 #[pallet::weight(10_000)]
		 pub fn set_asset_max_transfer_size(origin: OriginFor<T>, asset_id: T::AssetId, size: T::Balance) -> DispatchResultWithPostInfo {

		     ensure_signed(origin);

			 <MaxAssetTransferSize<T>>::insert(asset_id, size);

			 Self::deposit_event(Event::AssetMaxTransferSizeChanged(asset_id, size));

			 Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn set_asset_min_transfer_size(origin: OriginFor<T>, asset_id: T::AssetId, size: T::Balance) -> DispatchResultWithPostInfo {

		     ensure_signed(origin);

			 <MinAssetTransferSize<T>>::insert(asset_id, size);

			 Self::deposit_event(Event::AssetMinTransferSizeChanged(asset_id, size));

			 Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn set_transfer_lockup_time(origin: OriginFor<T>, lockup_time: Timestamp) -> DispatchResultWithPostInfo {

		     let sender = ensure_signed(origin)?;

			 let old_lockup_time = <TransferLockupTime<T>>::get();

			 <TransferLockupTime<T>>::put(lockup_time);

			 Self::deposit_event(Event::LockupTimeChanged(sender, old_lockup_time, lockup_time, "Transfer".as_bytes().to_vec()));

			 Ok(().into())
		 }	

		 #[pallet::weight(10_000)]
		 pub fn set_max_transfer_delay(origin: OriginFor<T>, new_max_transfer_delay: T::TransferDelay) -> DispatchResultWithPostInfo {
            
			ensure_signed(origin);

			let min_transfer_delay = Self::min_transfer_delay();

			ensure!(new_max_transfer_delay >= min_transfer_delay, Error::<T>::MaxTransferDelayBelowMinimum);

			<MaxTransferDelay<T>>::put(new_max_transfer_delay);

			Self::deposit_event(Event::MaxTransferDelayChanged(new_max_transfer_delay));

			Ok(().into())
		 }
		 
		 #[pallet::weight(10_000)]
		 pub fn set_min_transfer_delay(origin: OriginFor<T>, new_min_transfer_delay: T::TransferDelay) -> DispatchResultWithPostInfo {
            
			ensure_signed(origin);

			let max_transfer_delay = Self::max_transfer_delay();

			ensure!(new_min_transfer_delay <= max_transfer_delay, Error::<T>::MinTransferDelayAboveMaximum);
            
			<MinTransferDelay<T>>::put(new_min_transfer_delay);

			Self::deposit_event(Event::MinTransferDelayChanged(new_min_transfer_delay));

			Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn set_max_fee(origin: OriginFor<T>, max_fee: T::Balance) -> DispatchResultWithPostInfo {
			
			ensure_signed(origin);
            
			ensure!(max_fee < T::FeeFactor::get(), Error::<T>::MaxFeeAboveFeeFactor);

			ensure!(max_fee > Self::min_fee(), Error::<T>::MaxFeeBelowMinFee);

            <MaxFee<T>>::put(max_fee);

			Self::deposit_event(Event::MaxFeeChanged(max_fee));

			Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn set_min_fee(origin: OriginFor<T>, min_fee: T::Balance) -> DispatchResultWithPostInfo {
			
			ensure_signed(origin);
            
			ensure!(min_fee < T::FeeFactor::get(), Error::<T>::MinFeeAboveFeeFactor);

			ensure!(min_fee < Self::max_fee(), Error::<T>::MinFeeAboveMaxFee);

            <MinFee<T>>::put(min_fee);

			Self::deposit_event(Event::MinFeeChanged(min_fee));

			Ok(().into())
		 }

		 /**
		  * todo 
		  * setFeeAddress
		  *  getCurrentTokenLiquidity
		  calculateFeePercentage
		  withdrawTo
		  */

		 #[pallet::weight(10_000)]
		 pub fn deposit(
			 origin: OriginFor<T>, 
			 amount: T:: Balance,
			 asset_id: T::AssetId, 
			 destination_address: T::AccountId, 
			 remote_network_id: T::RemoteNetworkId,
		 	 transfer_delay: T::TransferDelay,
			) -> DispatchResultWithPostInfo {

			let sender = ensure_signed(origin)?;
			// ensure!(
			// 	config.strategies.len() <= T::MaxStrategies::get(),
			// 	Error::<T>::TooManyStrategies
			// );

			// ensure!(LastTransfer::<T>::)
			// todo - add lastTransfer check, ? how to get block.timespamp

			ensure!(transfer_delay <= <MaxTransferDelay<T>>::get(), Error::<T>::TransferDelayAboveMaximum);

			ensure!(transfer_delay >= <MinTransferDelay<T>>::get(), Error::<T>::TransferDelayBelowMinimum);

			ensure!(amount <= Self::max_asset_transfer_size(asset_id), Error::<T>::AmountAboveMaxAssetTransferSize);

			ensure!(amount >= Self::min_asset_transfer_size(asset_id), Error::<T>::AmountBelowMaxAssetTransferSize);

			T::Currency::burn_from(asset_id, &destination_address,  amount).map_err(|_| Error::<T>::BurnFailed)?;

			

			///- toddo store deposit info info 
			/// 
			/// - send event 
 			/// // question how to generate the hash id used in the solidity version , hash vs incremented uint

			Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn create_vault(
			 origin: OriginFor<T>,
			 asset_id: <T as Config>::AssetId,
			 reserved: Perquintill,
		 ) -> DispatchResultWithPostInfo {
 
			 let sender = ensure_signed(origin)?;
 
			 let account = Self::account_id();
 
			 let vault_id = T::Vault::create(
				 Deposit::Existential,
				 VaultConfig {
					 asset_id: asset_id,
					 reserved: reserved,
					 manager: sender,
					 strategies:[(account, Perquintill::one().saturating_sub(reserved))]
					 .iter()
					 .cloned()
					 .collect(),
				 },
			 )?;
 
		 	<AssetVault<T>>::insert(asset_id, vault_id);
 
		 	Self::deposit_event(Event::VaultCreated(sender, asset_id, vault_id, reserved));
 
			 Ok(().into())
		 }
 	}

	impl<T: Config> Pallet<T> {
		fn account_id() -> T::AccountId {
			T::PalletId::get().into_account()
		}
	}

 }

 
 
//  #[derive(Clone, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
//  pub struct VaultConfig<AccountId, CurrencyId>
//  where
// 	 AccountId: core::cmp::Ord,
//  {
// 	 pub asset_id: CurrencyId,
// 	 pub reserved: Perquintill,
// 	 pub manager: AccountId,
// 	 pub strategies: BTreeMap<AccountId, Perquintill>,
//  }