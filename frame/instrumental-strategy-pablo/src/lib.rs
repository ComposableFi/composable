#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

mod weights;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	// -------------------------------------------------------------------------------------------
	//                                   Imports and Dependencies                                
	// -------------------------------------------------------------------------------------------
	use crate::weights::WeightInfo;

	use frame_support::{
		pallet_prelude::*,
		storage::bounded_btree_set::BoundedBTreeSet,
		transactional, dispatch::DispatchResult
	};

	use sp_runtime::{
		traits::{
			AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, Zero,
		},
	};

	use composable_traits::vault::StrategicVault;

	use storage::require_transaction;

	use sp_std::fmt::Debug;
	use codec::{Codec, FullCodec};

	// -------------------------------------------------------------------------------------------
	//                                Declaration Of The Pallet Type                              
	// -------------------------------------------------------------------------------------------

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// -------------------------------------------------------------------------------------------
	//                                         Config Trait                                       
	// -------------------------------------------------------------------------------------------

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
		type VaultId: FullCodec
			+ MaxEncodedLen
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ Ord
			+ TypeInfo
			+ Into<u128>;

		type Vault: StrategicVault<
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			VaultId = Self::VaultId
		> ;

		/// The maximum number of vaults that can be associated with this strategy.
		type MaxAssociatedVaults: Get<u32>;
	}

	// -------------------------------------------------------------------------------------------
    //                                         Pallet Types                                       
	// -------------------------------------------------------------------------------------------

	// -------------------------------------------------------------------------------------------
    //                                       Runtime  Storage                                     
	// -------------------------------------------------------------------------------------------

	#[pallet::storage]
	#[pallet::getter(fn associated_vaults)]
	pub type AssociatedVaults<T: Config> =
		StorageValue<_, BoundedBTreeSet<T::VaultId, T::MaxAssociatedVaults>, ValueQuery>;

	// -------------------------------------------------------------------------------------------
    //                                        Runtime Events                                      
	// -------------------------------------------------------------------------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AssociatedVault { vault_id: T::VaultId },

		RebalancedVault { vault_id: T::VaultId },

		UnableToRebalanceVault { vault_id: T::VaultId },
	}

	// -------------------------------------------------------------------------------------------
    //                                        Runtime Errors                                      
	// -------------------------------------------------------------------------------------------

	#[pallet::error]
	pub enum Error<T> {
		VaultAlreadyAssociated,

		TooManyAssociatedStrategies,
	}

	// -------------------------------------------------------------------------------------------
    //                                            Hooks                                                
	// -------------------------------------------------------------------------------------------

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	// -------------------------------------------------------------------------------------------
    //                                          Extrinsics                                         
	// -------------------------------------------------------------------------------------------

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	// -------------------------------------------------------------------------------------------
    //                                      Protocol Strategy                                     
	// -------------------------------------------------------------------------------------------

	// TODO: (Nevin)
	//  - create InstrumentalProtocolStrategyTrait

	impl<T: Config> Pallet<T> {
		
		#[transactional]
		pub fn associate_vault(vault_id: &T::VaultId) -> Result<T::VaultId, DispatchError> {
			AssociatedVaults::<T>::try_mutate(|vaults| -> Result<T::VaultId, DispatchError> {
				ensure!(!vaults.contains(&vault_id), Error::<T>::VaultAlreadyAssociated);

				vaults.try_insert(*vault_id)
					.map_err(|_| Error::<T>::TooManyAssociatedStrategies)?;

				Self::deposit_event(Event::AssociatedVault{ vault_id: *vault_id });

				Ok(*vault_id)
			})
		}
		
		pub fn rebalance() -> DispatchResult {
			AssociatedVaults::<T>::try_mutate(|vaults| -> DispatchResult {
				vaults.iter().for_each(|vault_id| {
					if let Ok(_) = Self::do_rebalance(vault_id) {
						Self::deposit_event(Event::RebalancedVault{ vault_id: *vault_id });
					} else {
						Self::deposit_event(Event::UnableToRebalanceVault{ vault_id: *vault_id });
					}
				});

				Ok(())
			})
		}

	}

	// -------------------------------------------------------------------------------------------
    //                                   Low Level Functionality                                  
	// -------------------------------------------------------------------------------------------
	
	impl<T: Config> Pallet<T> {
		
		#[transactional]
		fn do_rebalance(_vault_id: &T::VaultId) -> DispatchResult {
			Ok(())
		}

	}
}

// -----------------------------------------------------------------------------------------------
//                                             Unit Tests                                         
// -----------------------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {}