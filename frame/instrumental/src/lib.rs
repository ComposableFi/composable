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

mod weights;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	// ----------------------------------------------------------------------------------------------------
	//                                       Imports and Dependencies                                      
	// ----------------------------------------------------------------------------------------------------

	use crate::weights::WeightInfo;

	use frame_support::{
		PalletId,
		pallet_prelude::*,
		transactional,
	};
	use frame_system::{
		pallet_prelude::OriginFor,
		ensure_signed,
	};

	use composable_traits::{
		vault::{Deposit as Duration, StrategicVault, Vault, VaultConfig},
	};

	use sp_runtime::{
		Perquintill,
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
		Created {
			asset: T::AssetId
		},

		AddedLiquidity {
			asset: T::AssetId,
			amount: T::Balance
		},

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
		
		#[pallet::weight(<T as Config>::WeightInfo::create())]
		pub fn create(
			origin: OriginFor<T>,
			asset: T::AssetId,
		) -> DispatchResultWithPostInfo {
			// TODO: (Nevin)
			//  - (potentially) enforce that the issuer must have priviledged rights

			// Requirement 0) This extrinsic must be signed 
			let from = ensure_signed(origin)?;

			Self::do_create(&from, &asset)?;

			Self::deposit_event(Event::Created { asset });

			Ok(().into())
		}

		#[pallet::weight(<T as Config>::WeightInfo::add_liquidity())]
		pub fn add_liquidity(
			origin: OriginFor<T>,
			asset: T::AssetId,
			amount: T::Balance
		) -> DispatchResultWithPostInfo {
			// Requirement 0) This extrinsic must be signed 
			let issuer = ensure_signed(origin)?;

			// Requirement 1) The asset must have an associated vault
			ensure!(
				AssetVault::<T>::contains_key(asset),
				Error::<T>::AssetDoesNotHaveAnAssociatedVault
			);

			Self::do_add_liquidity(&issuer, &asset, amount)?;
			Self::deposit_event(Event::AddedLiquidity {asset, amount});

			Ok(().into())
		}

		#[pallet::weight(<T as Config>::WeightInfo::add_liquidity())]
		pub fn remove_liquidity(
			origin: OriginFor<T>,
			asset: T::AssetId,
			amount: T::Balance
		) -> DispatchResultWithPostInfo {
			// Requirement 0) This extrinsic must be signed 
			let issuer = ensure_signed(origin)?;

			Self::do_remove_liquidity(&issuer, &asset, amount)?;
			Self::deposit_event(Event::RemovedLiquidity {asset, amount});

			Ok(().into())
		}
	}

	// ----------------------------------------------------------------------------------------------------
    //                                        Low Level Functionality                                      
	// ----------------------------------------------------------------------------------------------------

	impl<T: Config> Pallet<T> {
		fn account_id(asset: &T::AssetId) -> T::AccountId {
			T::PalletId::get().into_sub_account(asset)
		}
		
		#[transactional]
		fn do_create(
			_issuer: &T::AccountId,
			asset: &T::AssetId,
		) -> Result<(), DispatchError> {
			// Requirement 0) An asset can only have one vault associated with it
			ensure!(!AssetVault::<T>::contains_key(asset), Error::<T>::VaultAlreadyExists);

			// TODO: (Nevin)
			//  - decide if each assey should have an associated account, or if
			//        the pallet itself should have one global account
			let account_id = Self::account_id(asset);

			let vault_id: T::VaultId = T::Vault::create(
				Duration::Existential,
				VaultConfig:: <T::AccountId ,T::AssetId > {
					asset_id: *asset,
					reserved: Perquintill::from_percent(100),
					manager: account_id,
					strategies: [].iter().cloned().collect(),
				},
			)?;

			AssetVault::<T>::insert(asset, vault_id);
			
			Ok(())
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

			<T::Vault as StrategicVault>::withdraw(&vault_id, issuer, amount)?;

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
