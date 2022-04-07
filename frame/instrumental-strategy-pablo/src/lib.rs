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
		transactional
	};
	use frame_system::{
		ensure_signed,
		pallet_prelude::*,
	};

	use sp_std::fmt::Debug;
	use codec::FullCodec;

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
		// TODO: consider the tradeoff of using BoundedBTreeSet
		StorageValue<_, BoundedBTreeSet<T::VaultId,T::MaxAssociatedVaults>, ValueQuery>;

	// -------------------------------------------------------------------------------------------
    //                                        Runtime Events                                      
	// -------------------------------------------------------------------------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Test {
			issuer: T::AccountId
		},
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
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {

	}

	// -------------------------------------------------------------------------------------------
    //                                          Extrinsics                                         
	// -------------------------------------------------------------------------------------------

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(0)]
		pub fn test(
			origin: OriginFor<T>,
		) -> DispatchResultWithPostInfo {
			// Requirement 0) This extrinsic must be signed 
			let from = ensure_signed(origin)?;

			Self::deposit_event(Event::Test { issuer: from });

			Ok(().into())
		}
	}

	// -------------------------------------------------------------------------------------------
    //                                  Instrumental Strategy                                     
	// -------------------------------------------------------------------------------------------

	impl<T: Config> Pallet<T> {
		
		#[transactional]
		pub fn associate_vault(vault_id: &T::VaultId) -> DispatchResult {
			AssociatedVaults::<T>::try_mutate(|vaults| -> Result<(), DispatchError> {
				ensure!(!vaults.contains(&vault_id), Error::<T>::VaultAlreadyAssociated);

				vaults.try_insert(*vault_id)
					.map_err(|_| Error::<T>::TooManyAssociatedStrategies)?;

				Ok(())
			})?;

			Ok(())
		}
	}
}

// -----------------------------------------------------------------------------------------------
//                                             Unit Tests                                         
// -----------------------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {
}