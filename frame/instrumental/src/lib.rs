//! # Instrumental Pallet
//! 
//! This pallet will house the logic required by [`Intrumental Finance`](https://www.instrumental.finance/);
//! Instrumental will speak to this pallet through the Mosaic Pallet. This pallet will be responsible
//! for sending assets into their associated vaults and specifying which strategies should be set for each
//! vault.
//! 
//! ## Overview
//! 
//! The following API functions will be exposed thorugh this pallet:
//! 
//! - [`create`](Pallet::create)
//! - [`add_liquidity`](Pallet::add_liquidity)
//! - [`remove_liquidity`](Pallet::remove_liquidity)
//! 
//! ### Terminology
//! 
//! ### Goals
//! 
//! ### Actors
//! 
//! - users: Instrumentals users lie in the Ethereum ecosystem and interact (indirectly) with this
//!     pallet through the [`Intrumental Finance`](https://www.instrumental.finance/) frontend.
//! 
//! - Instrumental: The Ethereum-native smart sontracts provide the core funcitoanlity for Instrumental.
//! 
//! - Mosaic Pallet: Instrumental speaks to the Mosaic pallet which then redirects calls to the 
//!     Instrumental pallet.
//! 
//! - [`Vault Pallet`](../pallet_vault/index.html): Each asset supported by this pallet will have an underlying vault.
//!     Each vault will an associated stratgey that will dictate where those assets will go in 
//!     order to earn yeild.
//! 
//! ### Implementations
//! 
//! ## Interface
//! 
//! ### Extrinsics
//! 
//! - [`create`](Pallet::create): Creates a Cubic vault that is responsible for housing the specified asset
//!     and enforcing its strategy.
//! 
//! - [`add_liquidity`](Pallet::add_liquidity): Adds assets to its associated vault.
//! 
//! - [`remove_liquidity`](Pallet::remove_liquidity): Removes assets from its associated vault.
//! 
//! ### Runtime Storage Objects
//! 
//! - [`AssetVault`](AssetVault): Mapping of an `AssetId` to the underlying Cubic Vault's `VaultId` 
//!     that is responsible for enforcing the asset's strategy.
//! 
//! ## Usage
//! 
//! ### Example
//! 
//! ## Related Modules
//! 
//! - [`Vault Pallet`](../pallet_vault/index.html)
//!  

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod currency;

#[cfg(test)]
mod account_id;

mod weights;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	// ----------------------------------------------------------------------------------------------------
	//                                       Imports and Dependencies                                      
	// ----------------------------------------------------------------------------------------------------

	use crate::weights::WeightInfo;

	use frame_support::{
		pallet_prelude::*,
		PalletId,
		transactional,
	};
	use frame_system::{
		pallet_prelude::OriginFor,
		ensure_signed,
	};

	use composable_traits::{
		instrumental::Instrumental,
		vault::{Deposit as Duration, FundsAvailability, StrategicVault, Vault, VaultConfig},
	};

	use sp_runtime::{
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub,
			Zero,
		},
	};
	use sp_std::fmt::Debug;
	use codec::{Codec, FullCodec};
	
	// ----------------------------------------------------------------------------------------------------
	//                                    Declaration Of The Pallet Type                                           
	// ----------------------------------------------------------------------------------------------------

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// ----------------------------------------------------------------------------------------------------
	//                                             Config Trait                                            
	// ----------------------------------------------------------------------------------------------------

	// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type WeightInfo: WeightInfo;

		/// The Balance type used by the pallet for bookkeeping. `Config::Convert` is used for
		/// conversions to `u128`, which are used in the computations.
		type Balance: Default
			+ Parameter
			+ Codec
			+ MaxEncodedLen
			+ Copy
			+ Ord
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ AtLeast32BitUnsigned
			+ Zero;

		/// The `AssetId` used by the pallet. Corresponds to the Ids used by the Currency pallet.
		type AssetId: FullCodec
			+ MaxEncodedLen
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ TypeInfo;

		/// The `VaultId` used by the pallet. Corresponds to the Ids used by the Vault pallet.
		type VaultId: Clone 
		    + Codec 
			+ MaxEncodedLen 
			+ Debug 
			+ PartialEq 
			+ Default 
			+ Parameter;

		type Vault: StrategicVault<
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			VaultId = Self::VaultId,
		>;

		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	// ----------------------------------------------------------------------------------------------------
    //                                             Pallet Types                                           
	// ----------------------------------------------------------------------------------------------------

	// ----------------------------------------------------------------------------------------------------
    //                                            Runtime Storage                                          
	// ----------------------------------------------------------------------------------------------------

	/// Stores the `VaultId` that corresponds to a specific `AssetId`.
	#[pallet::storage]
	#[pallet::getter(fn asset_vault)]
	pub type AssetVault<T: Config> = 
		StorageMap<_, Blake2_128Concat, T::AssetId, T::VaultId>;

	// ----------------------------------------------------------------------------------------------------
    //                                            Runtime Events                                          
	// ----------------------------------------------------------------------------------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Emitted after a successful call to the [`create`](Pallet::create) extrinsic.
		Created {
			vault_id: T::VaultId,
			config: VaultConfig<T::AccountId, T::AssetId>
		},

		/// Emitted after a successful call to the [`add_liquidity`](Pallet::add_liquidity) extrinsic.
		AddedLiquidity {
			asset: T::AssetId,
			amount: T::Balance
		},

		/// Emitted after a successful call to the [`remove_liquidity`](Pallet::remove_liquidity) extrinsic.
		RemovedLiquidity {
			asset: T::AssetId,
			amount: T::Balance
		},
	}

	// ----------------------------------------------------------------------------------------------------
    //                                           Runtime  Errors                                           
	// ----------------------------------------------------------------------------------------------------

	#[pallet::error]
	pub enum Error<T> {
		/// This error is thrown when a vault is trying to be created for an asset that already
		///     has an associated vault.
		VaultAlreadyExists,

		/// This error is thrown when a user tries to call add_liquidity or remove_liquidity on an asset 
		///     that does not have an associated vault (yet).
		AssetDoesNotHaveAnAssociatedVault,

		/// This error is thrown if a user tries to withdraw an amount of assets that is currently not
		///     held in the specified vault.
		NotEnoughLiquidity,
	}

	// ----------------------------------------------------------------------------------------------------
    //                                                Hooks                                                
	// ----------------------------------------------------------------------------------------------------

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	// ----------------------------------------------------------------------------------------------------
    //                                              Extrinsics                                             
	// ----------------------------------------------------------------------------------------------------

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		/// Create an underlying vault and save a reference to its 'VaultId'.
		/// 
		/// # Overview
		/// 
		/// ## Parameters
		/// - `origin`: `Origin` type representing the origin of this dispatch.
		/// - `config`: the `VaultConfig` of the underlying vault to create.
		/// 
		/// ## Requirements
		/// 1. the call must have been signed by the issuer.
		/// 2. 'config.asset_id' must not correspond to a preexisting Instrumental vault.
		/// 
		/// ## Emits 
		/// - [`Event::Created`](Event::Created)
		/// 
		/// ## State Changes
		/// - [`AssetVault`](AssetVault): a mapping between the parameter `asset` and the created vault's
		///     `VaultId` is stored.
		/// 
		/// ## Errors
		/// - `VaultAlreadyExists`: there already exists an underlying vault for `asset`.
		/// 
		/// # Examples
		/// 
		/// # Weight: O(TBD)
		#[pallet::weight(<T as Config>::WeightInfo::create())]
		pub fn create(
			origin: OriginFor<T>,
			config: VaultConfig<T::AccountId, T::AssetId>,
		) -> DispatchResultWithPostInfo {
			// TODO: (Nevin)
			//  - (potentially) enforce that the issuer must have priviledged rights

			// Requirement 1) This extrinsic must be signed 
			let _from = ensure_signed(origin)?;

			let vault_id = <Self as Instrumental>::create(config.clone())?;

			Self::deposit_event(Event::Created { vault_id, config });

			Ok(().into())
		}

		/// Add assets into its underlying vault.
		/// 
		/// # Overview
		/// 
		/// ## Parameters
		/// - `origin`: `Origin` type representing the origin of this dispatch.
		/// - `asset`: the `AssetId` of the asset to deposit.
		/// - `amount`: the amount of `asset` to deposit.
		/// 
		/// ## Requirements
		/// 1. The call must have been signed by the issuer.
		/// 2. There must be a vault associated with `asset`.
		/// 
		/// ## Emits 
		/// - [`Event::AddedLiquidity`](Event::AddedLiquidity)
		/// 
		/// ## Errors
		/// - `AssetDoesNotHaveAnAssociatedVault`: no vault has been created for `asset`.
		/// 
		/// # Examples
		/// 
		/// # Weight: O(TBD)
		#[pallet::weight(<T as Config>::WeightInfo::add_liquidity())]
		pub fn add_liquidity(
			origin: OriginFor<T>,
			asset: T::AssetId,
			amount: T::Balance
		) -> DispatchResultWithPostInfo {
			// Requirement 1) This extrinsic must be signed 
			let issuer = ensure_signed(origin)?;

			<Self as Instrumental>::add_liquidity(&issuer, &asset, amount)?;

			Self::deposit_event(Event::AddedLiquidity {asset, amount});

			Ok(().into())
		}

		/// Remove assets from its underlying vault.
		/// 
		/// # Overview
		/// 
		/// ## Parameters
		/// - `origin`: `Origin` type representing the origin of this dispatch.
		/// - `asset`: the `AssetId` of the asset to withdraw.
		/// - `amount`: the amount of `asset` to withdraw.
		/// 
		/// ## Requirements
		/// 1. The call must have been signed by the issuer.
		/// 2. There must be a vault associated with `asset`.
		/// 
		/// ## Emits 
		/// - [`Event::RemovedLiquidity`](Event::RemovedLiquidity)
		/// 
		/// ## Errors
		/// - `AssetDoesNotHaveAnAssociatedVault`: no vault has been created for `asset`.
		/// 
		/// # Examples
		/// 
		/// # Weight: O(TBD)
		#[pallet::weight(<T as Config>::WeightInfo::add_liquidity())]
		pub fn remove_liquidity(
			origin: OriginFor<T>,
			asset: T::AssetId,
			amount: T::Balance
		) -> DispatchResultWithPostInfo {
			// Requirement 1) This extrinsic must be signed 
			let issuer = ensure_signed(origin)?;

			<Self as Instrumental>::remove_liquidity(&issuer, &asset, amount)?;
			
			Self::deposit_event(Event::RemovedLiquidity {asset, amount});

			Ok(().into())
		}
	}

	// ----------------------------------------------------------------------------------------------------
    //                                          Instrumental Trait                                         
	// ----------------------------------------------------------------------------------------------------

	impl<T: Config> Instrumental for Pallet<T> {
		type AccountId = T::AccountId;
		type AssetId = T::AssetId;
		type Balance = T::Balance;
		type VaultId = T::VaultId;

		/// Create an underlying vault and save a reference to its 'VaultId'.
		/// 
		/// # Overview
		/// 
		/// ## Parameters
		/// - `config`: the `VaultConfig` of the underlying vault to create.
		/// 
		/// ## Requirements
		/// 1. 'config.asset_id' must not correspond to a preexisting Instrumental vault.
		/// 
		/// ## State Changes
		/// - [`AssetVault`](AssetVault): a mapping between the parameter `asset` and the created vault's
		///     `VaultId` is stored.
		/// 
		/// ## Errors
		/// - `VaultAlreadyExists`: their already exists an underlying vault for `asset`.
		/// 
		/// # Runtime: O(TBD)
		fn create(
			config: VaultConfig<Self::AccountId, Self::AssetId>,
		) -> Result<Self::VaultId, DispatchError> {
			// Requirement 1) An asset can only have one vault associated with it
			ensure!(!AssetVault::<T>::contains_key(config.asset_id), Error::<T>::VaultAlreadyExists);
			
			let vault_id = Self::do_create(config)?;

			Ok(vault_id)
		}
	
		/// Add assets into its underlying vault.
		/// 
		/// # Overview
		/// 
		/// ## Parameters
		/// - `issuer`: the 'AccountId' of the user who issued the request
		/// - `asset`: the `AssetId` of the asset to deposit.
		/// - `amount`: the amount of `asset` to deposit.
		/// 
		/// ## Requirements
		/// 1. There must be a vault associated with `asset`.
		/// 
		/// ## Errors
		/// - `AssetDoesNotHaveAnAssociatedVault`: no vault has been created for `asset`.
		/// 
		/// # Runtime: O(TBD)
		fn add_liquidity(
			issuer: &Self::AccountId,
			asset: &Self::AssetId,
			amount: Self::Balance
		) -> Result<(), DispatchError> {
			// Requirement 1) The asset must have an associated vault
			ensure!(
				AssetVault::<T>::contains_key(asset),
				Error::<T>::AssetDoesNotHaveAnAssociatedVault
			);

			Self::do_add_liquidity(issuer, asset, amount)?;

			Ok(())
		}
	
		/// Remove assets from its underlying vault.
		/// 
		/// # Overview
		/// 
		/// ## Parameters
		/// - `issuer`: the 'AccountId' of the user who issued the request
		/// - `asset`: the `AssetId` of the asset to withdraw.
		/// - `amount`: the amount of `asset` to withdraw.
		/// 
		/// ## Requirements
		/// 1. There must be a vault associated with `asset`.
		/// 
		/// ## Errors
		/// - `AssetDoesNotHaveAnAssociatedVault`: no vault has been created for `asset`.
		/// 
		/// # Runtime: O(TBD)
		fn remove_liquidity(
			issuer: &Self::AccountId,
			asset: &Self::AssetId,
			amount: Self::Balance
		) -> Result<(), DispatchError> {
			// Requirement 1) The asset must have an associated vault
			ensure!(
				AssetVault::<T>::contains_key(asset),
				Error::<T>::AssetDoesNotHaveAnAssociatedVault
			);

			Self::do_remove_liquidity(issuer, asset, amount)?;

			Ok(())
		}
	}

	// ----------------------------------------------------------------------------------------------------
    //                                        Low Level Functionality                                      
	// ----------------------------------------------------------------------------------------------------

	impl<T: Config> Pallet<T> {
		// TODO: (Nevin)
		//  - decide if each asset should have an associated account, or if
		//        the pallet itself should have one global account
		fn account_id(asset: &T::AssetId) -> T::AccountId {
			T::PalletId::get().into_sub_account(asset)
		}
		
		#[transactional]
		fn do_create(
			config: VaultConfig<T::AccountId, T::AssetId>,
		) -> Result<T::VaultId, DispatchError> {
			let asset = config.asset_id;
			let account_id = Self::account_id(&asset);

			// TODO: (Nevin)
			//  - decide a better way to input VaultConfig fields (maybe as seperate inputs)
			//  - VaultConfig.manager should be set to account_id
			let vault_id: T::VaultId = T::Vault::create(
				Duration::Existential,
				VaultConfig {
					asset_id: config.asset_id,
					manager: account_id,
					reserved: config.reserved,
					strategies: config.strategies,
				},
			)?;

			AssetVault::<T>::insert(asset, &vault_id);
			
			Ok(vault_id)
		}

		#[transactional]
		fn do_add_liquidity(
			issuer: &T::AccountId,
			asset: &T::AssetId,
			amount: T::Balance
		) -> Result<(), DispatchError> {
			let vault_id: T::VaultId = Self::asset_vault(asset)
				.ok_or(Error::<T>::AssetDoesNotHaveAnAssociatedVault)?;

			<T::Vault as StrategicVault>::deposit(&vault_id, issuer, amount)?;

			Ok(())
		}

		#[transactional]
		fn do_remove_liquidity(
			issuer: &T::AccountId,
			asset: &T::AssetId,
			amount: T::Balance
		) -> Result<(), DispatchError> {
			let vault_id: T::VaultId = Self::asset_vault(asset)
				.ok_or(Error::<T>::AssetDoesNotHaveAnAssociatedVault)?;

			// TODO: (Nevin)
			//  - this can be done in a better way
			let vault_account = T::Vault::account_id(&vault_id);
			match <T::Vault as StrategicVault>::available_funds(&vault_id, &vault_account)? {
				FundsAvailability::Withdrawable(balance) if balance >= amount => {
					<T::Vault as StrategicVault>::withdraw(&vault_id, issuer, amount)
				},
				FundsAvailability::MustLiquidate => {
					<T::Vault as StrategicVault>::withdraw(&vault_id, issuer, amount)
				},
				_ => {
					Err(Error::<T>::NotEnoughLiquidity.into())
				}
			}.map_err(|_| Error::<T>::NotEnoughLiquidity)?;

			Ok(())
		}
	}

}

// ----------------------------------------------------------------------------------------------------
//                                              Unit Tests                                             
// ----------------------------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

}
