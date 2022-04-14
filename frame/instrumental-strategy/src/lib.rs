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
	};

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

		/// The maximum number of vaults that can be associated with this strategy.
		type MaxStrategies: Get<u32>;
	}

	// -------------------------------------------------------------------------------------------
    //                                         Pallet Types                                       
	// -------------------------------------------------------------------------------------------

	// -------------------------------------------------------------------------------------------
    //                                       Runtime  Storage                                     
	// -------------------------------------------------------------------------------------------

	#[pallet::storage]
	#[pallet::getter(fn associated_vaults)]
	pub type WhitelistedStrategies<T: Config> =
		StorageValue<_, BoundedBTreeSet<T::AccountId, T::MaxStrategies> , ValueQuery>;

	// -------------------------------------------------------------------------------------------
    //                                        Runtime Events                                      
	// -------------------------------------------------------------------------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		WhitelistedStrategy { strategy: T::AccountId }
	}

	// -------------------------------------------------------------------------------------------
    //                                        Runtime Errors                                      
	// -------------------------------------------------------------------------------------------

	#[pallet::error]
	pub enum Error<T> {
		StrategyAlreadyWhitelisted,

		TooManyWhitelistedStrategies
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
    //                                     Trait Implementation                                   
	// -------------------------------------------------------------------------------------------

	// TODO: (Nevin)
	//  - create InstrumentalStrategy trait

	impl<T: Config> Pallet<T> {

		pub fn whitelist_strategy(account_id: T::AccountId) -> DispatchResult {
			WhitelistedStrategies::<T>::try_mutate(|strategies| -> DispatchResult {
				ensure!(
					!strategies.contains(&account_id), Error::<T>::StrategyAlreadyWhitelisted
				);

				strategies.try_insert(account_id.clone())
					.map_err(|_| Error::<T>::TooManyWhitelistedStrategies)?;

				Self::deposit_event(Event::WhitelistedStrategy {strategy: account_id} );

				Ok(())
			})
		}
	}
}

// -----------------------------------------------------------------------------------------------
//                                             Unit Tests                                         
// -----------------------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {}