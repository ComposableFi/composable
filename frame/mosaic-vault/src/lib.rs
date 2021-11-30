#![cfg_attr(not(feature = "std"), no_std)]

pub mod mocks;
pub mod traits;

pub use pallet::*;

// #[cfg(test)]
// mod mock;

#[cfg(test)]
mod tests;
#[frame_support::pallet]
pub mod pallet {

	use frame_support::{
		ensure,
		pallet_prelude::*,
		traits::{
			UnixTime,
			fungibles::{Mutate, Transfer}
		},
		PalletId,
	};
	use sp_core::hashing::keccak_256;
	use frame_system::pallet_prelude::*;
 	use scale_info::TypeInfo;
	use sp_std::{fmt::Debug, vec::Vec}; 
	use codec::{Codec, FullCodec};
	use sp_runtime::{
         traits::{
			AtLeast32BitUnsigned, Convert, AccountIdConversion, 
			Saturating, CheckedSub, CheckedAdd, CheckedMul, CheckedDiv, Zero,
			
		 },
		  Perquintill,
	};
	use composable_traits::{loans::Timestamp, vault::{Deposit, FundsAvailability, StrategicVault, Vault, VaultConfig }};

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

		type Balance: Parameter 
		    + Member 
			+ AtLeast32BitUnsigned 
			+ Codec 
			+ Default 
			+ Copy 
			+ MaybeSerializeDeserialize 
			+ Debug 
			+ MaxEncodedLen 
			+ TypeInfo 
			+ CheckedSub 
			+ CheckedAdd 
			+ Zero 
			+ PartialOrd;

		type Nonce:  Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy + MaybeSerializeDeserialize + Debug + MaxEncodedLen + TypeInfo + CheckedSub + CheckedAdd;//+ From<u8>;

		type TransferDelay:  Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy + MaybeSerializeDeserialize + Debug + MaxEncodedLen + TypeInfo;

		type VaultId: Clone 
		    + Codec 
			+ Debug 
			+ PartialEq 
			+ Default 
			+ Parameter;

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

		type DepositId: FullCodec
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
		type ThresholdFactor: Get<Self::Balance>;

		#[pallet::constant]
		type PalletId: Get<PalletId>;

		type FeeAddress: Get<Self::AccountId>;

		type BlockTimestamp: UnixTime;

		type MaxFeeDefault: Get<Self::Balance>;

		type MinFeeDefault: Get<Self::Balance>;
	}
	#[derive(Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
	pub struct DepositInfo<AssetId, Balance > {
        pub asset_id: AssetId,
		pub amount: Balance,
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

	#[pallet::type_value]
	pub(super) fn MaxFeeDefault<T: Config>() -> T::Balance {
        T::MaxFeeDefault::get()
	}

	#[pallet::storage]
	#[pallet::getter(fn max_fee)]
	pub(super) type MaxFee<T: Config> = StorageValue<_, T::Balance, ValueQuery, MaxFeeDefault<T>>;

	#[pallet::type_value]
	pub(super) fn MinFeeDefault<T: Config>() -> T::Balance {
        T::MinFeeDefault::get()
	}

	#[pallet::storage]
	#[pallet::getter(fn min_fee)]
	pub(super) type MinFee<T: Config> = StorageValue<_, T::Balance, ValueQuery, MinFeeDefault<T>>;

	#[pallet::storage]
	#[pallet::getter(fn asset_vault)]
	pub(super) type AssetVault<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, T::VaultId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn has_been_withdrawn)]
	pub(super) type HasBeenWithdrawn<T: Config> = StorageMap<_, Blake2_128Concat, T::DepositId, bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn has_been_unlocked)]
	pub(super) type HasBeenUnlocked<T: Config> = StorageMap<_, Blake2_128Concat, T::DepositId, bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn has_been_completed)]
	pub(super) type HasBeenCompleted<T: Config> = StorageMap<_, Blake2_128Concat, T::DepositId, bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn in_transfer_funds)]
	pub(super) type InTransferFunds<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, T::Balance, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn deposits)]
	pub(super) type Deposits<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, DepositInfo<T::AssetId, T::Balance>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn nonce)]
	pub(super) type Nonce<T: Config> = StorageValue<_, T::Nonce, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn fee_threshold)]
	pub(super) type FeeThreshold<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn last_withdraw_id)]
	pub(super) type LastWithdrawID<T: Config> = StorageValue<_, T::DepositId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn last_unlocked_id)]
	pub(super) type LastUnlockedID<T: Config> = StorageValue<_, T::DepositId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn pause_status)]
	pub(super) type PauseStatus<T :Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
 	pub enum Event<T: Config> {

		DepositCompleted(
   			T::AccountId, // sender
   			T::AssetId,   // assetId
			T::RemoteAssetId, // remoteAssetId
			T::RemoteNetworkId, // remoteNetworkId
			T::AccountId, // receiver
 			T::Balance, // value
			[u8; 32], // uniqueId
			T::TransferDelay // transferDelay
		),

		WithdrawalCompleted(
		   T::AccountId, // receiver
           T::Balance, // amount
		   T::Balance, // receivedAmount
		   T::Balance, // feeAmount
		   T::AssetId, // assetId
		   T::DepositId, // uniqueId
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
		),

		TransferFundsUnlocked(
			T::AssetId, 
			T::Balance, 
			T::DepositId
		),

		FeeTaken(
            T::AccountId, // sender
			T::AccountId, // receiver
			T::AssetId,
			T::Balance, // amount
			T::Balance, // fee
			T::DepositId,
		),

		FeeThresholdChanged(
           T::Balance,
		),

		Pause(
			T::AccountId,
		),

		UnPause(
			T::AccountId,
		),

		FundsUnlocked(
			T::AssetId,
			T::AccountId,
			T::Balance,
			T::DepositId,
		),

		LiquidityMoved(
          T::AccountId,
		  T::AccountId,
		  T::Balance,
		),
	}

	#[allow(missing_docs)]
	#[pallet::error]
	pub enum Error<T> {
		/// Minting failures result in `MintFailed`. In general this should never occur.
		MintFailed,
		/// 
		DepositFailed,
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

		AlreadCompleted,

		InsufficientFunds,

		InsufficientAssetBalance,

		ThresholdFeeAboveThresholdFactor,

		AlreadyWithdrawn,

		TransferNotPossible,

		AssetUnlreadyUnlocked,

		TransferFromFailed,

		WithdrawFailed,

		ZeroAmount,

		DivisionError,

		ContractPaused,

		ContractNotPaused,

		NoTransferableBalance,

		UnsupportedToken,

		Underflow,

		Overflow,
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

			Self::only_supported_remote_token(remote_network_id.clone(), asset_id.clone())?;
          
 		    let remote_asset_id = RemoteAssetId::<T>::get(remote_network_id, asset_id);

		     <RemoteAssetId<T>>::remove(remote_network_id, asset_id);

			 <MaxAssetTransferSize<T>>::remove(asset_id);

			 <MinAssetTransferSize<T>>::remove(asset_id);

             Self::deposit_event(Event::TokenRemoved(asset_id, remote_asset_id,  remote_network_id));

			 Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn set_asset_max_transfer_size(origin: OriginFor<T>, asset_id: T::AssetId, size: T::Balance) -> DispatchResultWithPostInfo {

		     ensure_signed(origin)?;

			 <MaxAssetTransferSize<T>>::insert(asset_id, size);

			 Self::deposit_event(Event::AssetMaxTransferSizeChanged(asset_id, size));

			 Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn set_asset_min_transfer_size(origin: OriginFor<T>, asset_id: T::AssetId, size: T::Balance) -> DispatchResultWithPostInfo {

		     ensure_signed(origin)?;

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
            
			ensure_signed(origin)?;

			let min_transfer_delay = Self::min_transfer_delay();

			ensure!(new_max_transfer_delay >= min_transfer_delay, Error::<T>::MaxTransferDelayBelowMinimum);

			<MaxTransferDelay<T>>::put(new_max_transfer_delay);

			Self::deposit_event(Event::MaxTransferDelayChanged(new_max_transfer_delay));

			Ok(().into())
		 }
		 
		 #[pallet::weight(10_000)]
		 pub fn set_min_transfer_delay(origin: OriginFor<T>, new_min_transfer_delay: T::TransferDelay) -> DispatchResultWithPostInfo {
            
			ensure_signed(origin)?;

			let max_transfer_delay = Self::max_transfer_delay();

			ensure!(new_min_transfer_delay <= max_transfer_delay, Error::<T>::MinTransferDelayAboveMaximum);
            
			<MinTransferDelay<T>>::put(new_min_transfer_delay);

			Self::deposit_event(Event::MinTransferDelayChanged(new_min_transfer_delay));

			Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn set_max_fee(origin: OriginFor<T>, max_fee: T::Balance) -> DispatchResultWithPostInfo {
			
			ensure_signed(origin)?;
            
			ensure!(max_fee < T::FeeFactor::get(), Error::<T>::MaxFeeAboveFeeFactor);

			ensure!(max_fee > Self::min_fee(), Error::<T>::MaxFeeBelowMinFee);

            <MaxFee<T>>::put(max_fee);

			Self::deposit_event(Event::MaxFeeChanged(max_fee));

			Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn set_min_fee(origin: OriginFor<T>, min_fee: T::Balance) -> DispatchResultWithPostInfo {
			
			ensure_signed(origin)?;
            
			ensure!(min_fee < T::FeeFactor::get(), Error::<T>::MinFeeAboveFeeFactor);

			ensure!(min_fee < Self::max_fee(), Error::<T>::MinFeeAboveMaxFee);

            <MinFee<T>>::put(min_fee);

			Self::deposit_event(Event::MinFeeChanged(min_fee));

			Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn set_thresh_hold(origin: OriginFor<T>, new_fee_threshold: T::Balance) -> DispatchResultWithPostInfo {
			 
			ensure_signed(origin)?;

			ensure!(new_fee_threshold < T::ThresholdFactor::get(), Error::<T>::ThresholdFeeAboveThresholdFactor);

			<FeeThreshold<T>>::put(new_fee_threshold);

			Self::deposit_event(Event::FeeThresholdChanged(new_fee_threshold));

			Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn deposit(
			 origin: OriginFor<T>, 
			 amount: T::Balance,
			 asset_id: T::AssetId, 
			 destination_address: T::AccountId, 
			 remote_network_id: T::RemoteNetworkId,
		 	 transfer_delay: T::TransferDelay,
			) -> DispatchResultWithPostInfo {

			ensure!(Self::pause_status() == false, Error::<T>::ContractPaused);

			Self::only_supported_remote_token(remote_network_id.clone(), asset_id.clone())?;

			let sender = ensure_signed(origin)?;

			ensure!(amount != T::Balance::zero(), Error::<T>::ZeroAmount);

			ensure!(Self::last_transfer(&sender).checked_add(Self::transfer_lockup_time()).ok_or(Error::<T>::Overflow)? < T::BlockTimestamp::now().as_secs(), Error::<T>::TransferNotPossible);
           
			ensure!(transfer_delay >= <MinTransferDelay<T>>::get(), Error::<T>::TransferDelayBelowMinimum);
		
			ensure!(transfer_delay <= <MaxTransferDelay<T>>::get(), Error::<T>::TransferDelayAboveMaximum);

			ensure!(amount <= Self::max_asset_transfer_size(asset_id), Error::<T>::AmountAboveMaxAssetTransferSize);

			ensure!(amount >= Self::min_asset_transfer_size(asset_id), Error::<T>::AmountBelowMaxAssetTransferSize);

			// update in_transfer_funds
			let in_transfer_funds = Self::in_transfer_funds(asset_id);
			let new_in_transfer_funds = in_transfer_funds.checked_add(&amount).ok_or(Error::<T>::Overflow)?;
			<InTransferFunds<T>>::insert(asset_id, new_in_transfer_funds);
			// 
			let pallet_account_id = Self::account_id();            
            // move funds to pallet amount
			T::Currency::transfer(asset_id, &sender, &pallet_account_id, amount, true).map_err(|_|Error::<T>::TransferFromFailed)?;
            // deposit to valut
			let vault_id = <AssetVault<T>>::get(asset_id);
			<T::Vault as StrategicVault>::deposit(&vault_id, &pallet_account_id, amount).map_err(|_| Error::<T>::DepositFailed)?;
           
			<LastTransfer<T>>::insert(&sender, T::BlockTimestamp::now().as_secs());

			let deposit_id = Self::generate_deposit_id(remote_network_id, &destination_address, pallet_account_id);
            <Deposits<T>>::insert(asset_id, DepositInfo{asset_id, amount});

			Self::deposit_event(Event::DepositCompleted(
				sender,
				asset_id,
				Self::remote_asset_id(remote_network_id, asset_id),
				remote_network_id,
				destination_address,
				amount, 
				deposit_id,
				transfer_delay
			));

			Ok(().into())
		 }


		 #[pallet::weight(10_000)]
		 pub fn withdraw(
			origin: OriginFor<T>, 
			destination_account: T::AccountId,
			amount: T::Balance,
			asset_id: T::AssetId, 
			remote_network_id: T::RemoteNetworkId,
	        deposit_id: T::DepositId,
		 ) -> DispatchResultWithPostInfo {
         
			 ensure!(Self::pause_status() == false, Error::<T>::ContractPaused);

			  Self::only_supported_remote_token(remote_network_id.clone(), asset_id.clone())?;

			  let sender = ensure_signed(origin)?;
             
			  ensure!(Self::has_been_withdrawn(deposit_id) == false, Error::<T>::AlreadyWithdrawn);

			  <HasBeenWithdrawn<T>>::insert(deposit_id, true);

			  <LastWithdrawID<T>>::put(deposit_id);

			  let pallet_account_id = Self::account_id(); 

			  let vault_id = <AssetVault<T>>::get(asset_id);

			  <T::Vault as StrategicVault>::withdraw(&vault_id, &pallet_account_id, amount).map_err(|_| Error::<T>::WithdrawFailed)?;
              
              let fee = Self::calculate_fee_percentage(asset_id, amount)?;
			  
			  let fee_absolute = amount.checked_mul(&fee)
			     .and_then(|x|x.checked_div(&T::FeeFactor::get()))
				 .ok_or(Error::<T>::Overflow)?;
	
			  let withdraw_amount = amount.checked_sub(&fee_absolute).ok_or(Error::<T>::Underflow)?;

			  ensure!(Self::get_current_token_liquidity(asset_id)? >= amount, Error::<T>::InsufficientAssetBalance);    

			  T::Currency::transfer(asset_id, &pallet_account_id, &sender, withdraw_amount, true).map_err(|_|Error::<T>::TransferFromFailed)?;

			 if fee_absolute > T::Balance::zero() {  
			   
				T::Currency::transfer(asset_id, &pallet_account_id, &Self::get_fee_address(), fee_absolute, true).map_err(|_|Error::<T>::TransferFromFailed)?;
				
				Self::deposit_event(Event::FeeTaken(
					sender, 
					destination_account.clone(), // receiver
					asset_id,
					amount, // amount
					fee_absolute, // fee
					deposit_id,
				));
			 }

			 Self::deposit_event(Event::WithdrawalCompleted(
				destination_account,
				amount,
				withdraw_amount,
				fee_absolute,
				asset_id,
				deposit_id
			 ));

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
					 manager: sender.clone(),
					 strategies:[(account, Perquintill::one().saturating_sub(reserved))]
					 .iter()
					 .cloned()
					 .collect(),
				 },
			 )?;
 
		 	<AssetVault<T>>::insert(asset_id, &vault_id);
 
		 	Self::deposit_event(Event::VaultCreated(sender, asset_id, vault_id, reserved));
 
			 Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn unlock_in_transfer_funds(
			origin: OriginFor<T>,
			asset_id: T:: AssetId,
			amount: T::Balance,
			deposit_id: T::DepositId,
		 ) ->DispatchResultWithPostInfo {

			ensure!(Self::pause_status() == false, Error::<T>::ContractPaused);
			
			ensure_signed(origin)?;
            
			ensure!(Self::has_been_completed(deposit_id) == false, Error::<T>::AlreadCompleted);

			ensure!(Self::in_transfer_funds(asset_id) >= amount, Error::<T>::InsufficientFunds);

			let deposit = Self::deposits(asset_id);

			ensure!(deposit.asset_id == asset_id && deposit.amount == amount, Error::<T>::InsufficientFunds);

			<HasBeenCompleted<T>>::insert(deposit_id, true);

	       let new_intransfer_funds = Self::in_transfer_funds(asset_id).checked_sub(&amount).ok_or(Error::<T>::Underflow)?;

		   <InTransferFunds<T>>::insert(asset_id, new_intransfer_funds);

		   Self::deposit_event(Event::TransferFundsUnlocked(asset_id, amount, deposit_id));
	
			Ok(().into())
		 }

		 #[pallet::weight(10_000)]
		 pub fn unlock_funds(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			user_account_id: T::AccountId,
			amount: T::Balance,
			deposit_id: T::DepositId,
		 ) ->DispatchResultWithPostInfo {
            
			ensure_signed(origin.clone())?;
          
			 ensure!(Self::has_been_unlocked(deposit_id) == false, Error::<T>::AssetUnlreadyUnlocked);

			 <HasBeenUnlocked<T>>::insert(deposit_id, true);

			 <LastUnlockedID<T>>::put(deposit_id);

			 let pallet_account_id = Self::account_id(); 

			 let vault_id = <AssetVault<T>>::get(asset_id);

			<T::Vault as StrategicVault>::withdraw(&vault_id, &pallet_account_id, amount).map_err(|_| Error::<T>::WithdrawFailed)?;

			T::Currency::transfer(asset_id, &pallet_account_id, &user_account_id, amount, true).map_err(|_|Error::<T>::TransferFromFailed)?;
             
			Self::deposit_event(Event::FundsUnlocked(asset_id,user_account_id, amount, deposit_id));

			if Self::has_been_completed(deposit_id) == false {
				Self::unlock_in_transfer_funds(origin, asset_id, amount, deposit_id)?;
		    }

			Ok(().into())

		 }

		 #[pallet::weight(10_000)]
		 pub fn save_funds(
			 origin: OriginFor<T>,
			 asset_id: T::AssetId,
			 to: T::AccountId,
		 ) -> DispatchResultWithPostInfo {

			let sender = ensure_signed(origin)?;

			ensure!(Self::pause_status() == true, Error::<T>::ContractNotPaused);

			let withdrawable_balance = Self::get_withdrawable_balance(asset_id)?;

			ensure!(withdrawable_balance > T::Balance::zero(), Error::<T>::NoTransferableBalance);

			let pallet_account_id = Self::account_id(); 

			let vault_id = <AssetVault<T>>::get(asset_id);

			<T::Vault as StrategicVault>::withdraw(&vault_id, &pallet_account_id, withdrawable_balance).map_err(|_| Error::<T>::WithdrawFailed)?;

			T::Currency::transfer(asset_id, &pallet_account_id, &to, withdrawable_balance, true).map_err(|_|Error::<T>::TransferFromFailed)?;
             
		    Self::deposit_event(Event::LiquidityMoved(sender, to, withdrawable_balance));

			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn pause(origin: OriginFor<T>) -> DispatchResultWithPostInfo {

			ensure!(Self::pause_status() == false, Error::<T>::ContractPaused);
			
            let sender = ensure_signed(origin)?;

			 <PauseStatus<T>>::put(true);
			 Self::deposit_event(Event::Pause(sender));

			 Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn un_pause(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

			 <PauseStatus<T>>::put(false);
			 Self::deposit_event(Event::UnPause(sender));

			 Ok(().into())
		}
 	}

	impl<T: Config> Pallet<T> {
		fn account_id() -> T::AccountId {
			T::PalletId::get().into_account()
		}

		fn get_fee_address() -> T::AccountId {
			T::FeeAddress::get()
		}

		fn increment_nonce() -> T::Nonce {

			let mut nonce = Self::nonce();

			nonce += 1u8.into();

			<Nonce<T>>::put(nonce);

			nonce
		}

		fn calculate_fee_percentage(asset_id: T::AssetId, amount: T::Balance) -> Result<T::Balance, DispatchError> {

			let token_liquidity = Self::get_current_token_liquidity(asset_id)?;

			if token_liquidity == T::Balance::zero() {
				return Ok(Self::max_fee());
			}

           let fee_threshold = Self::fee_threshold();

		   let multiplier: T::Balance = 100u8.into();

			if  amount.checked_mul(&multiplier).and_then(|x| x.checked_div(&token_liquidity)).ok_or(Error::<T>::Overflow)? > fee_threshold {
                  return Ok(Self::max_fee());
			}

			let max_transfer = (token_liquidity.checked_mul(&fee_threshold).and_then(|x| x.checked_div(&T::ThresholdFactor::get())).ok_or(Error::<T>::Overflow))?;
            let percent_transfer = (amount.checked_mul(&multiplier).and_then(|x| x.checked_div(&max_transfer)).ok_or(Error::<T>::Overflow))?;

			let fee_percentage = percent_transfer.checked_mul(
				&(max_transfer.checked_sub(&Self::max_fee()).ok_or(Error::<T>::Underflow)?)
			).ok_or(Error::<T>::Overflow)?.checked_add(&(
				(Self::min_fee()).checked_mul(&multiplier).ok_or(Error::<T>::Overflow)?
			)).ok_or(Error::<T>::Overflow)?.checked_div(
				&multiplier
			).ok_or(Error::<T>::DivisionError)?;
         
	     	Ok(fee_percentage)
		}

		fn get_current_token_liquidity(asset_id: T::AssetId) -> Result<T::Balance, DispatchError> {
		
			let available_funds = Self::get_withdrawable_balance(asset_id)?;

			let liquidity = available_funds.checked_sub(&Self::in_transfer_funds(asset_id)).ok_or(Error::<T>::Underflow)?;

			Ok(liquidity)
		}

		fn get_withdrawable_balance(asset_id: T::AssetId) -> Result<T::Balance, DispatchError> {

			let vault_id = <AssetVault<T>>::get(asset_id);
			
			let available_funds = match <T::Vault as StrategicVault>::available_funds(&vault_id, &Self::account_id())? {
				FundsAvailability::Withdrawable(balance) => balance,
				_ => T::Balance::zero(),
			};

			Ok(available_funds)
		}


		fn only_supported_remote_token(remote_network_id: T::RemoteNetworkId, asset_id:T::AssetId) -> Result<T::RemoteAssetId, DispatchError> {
			
			let remote_asset_id = <RemoteAssetId<T>>::try_get(remote_network_id, asset_id).map_err(|_|Error::<T>::UnsupportedToken)?;

			Ok(remote_asset_id)
		}

		fn generate_deposit_id(
			remote_network_id: T::RemoteNetworkId,
			destination_address: &T::AccountId,
			pallet_account_id: T::AccountId,
		) -> [u8; 32] {

			let mut encoded_remote_network_id = Encode::encode(&remote_network_id);

			let mut encoded_block_number = Encode::encode(&<frame_system::Pallet<T>>::block_number());

            let mut encoded_destination_address = Encode::encode(&destination_address);

			let mut encoded_pallet_account_id = Encode::encode(&pallet_account_id);

			let mut encoded_nonce = Encode::encode(&Self::increment_nonce());

			let mut encoded_data = Vec::new();
			encoded_data.append(& mut encoded_remote_network_id);
			encoded_data.append(& mut encoded_block_number);
			encoded_data.append(& mut encoded_destination_address);
			encoded_data.append(& mut encoded_pallet_account_id);
			encoded_data.append(& mut encoded_nonce);

			let deposit_id = keccak_256(&encoded_data);

			deposit_id
		}
	}
 }