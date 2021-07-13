//! A simple customizable pallet
// useful links: https://github.com/paritytech/substrate/, https://substrate.dev

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{debug, decl_error, decl_storage, traits::Get};
use frame_support::{decl_event, decl_module, dispatch::DispatchResult};
use frame_system::ensure_signed;
use sp_runtime::print; //substrate-print | https://github.com/paritytech/substrate/blob/77dcc4f90917f2215ee40efeacd68be9ce85db14/primitives/runtime/src/lib.rs#L626

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

// define key-value storage

decl_storage! {
    // A unique name is used to ensure that the pallet's storage items are isolated.
    // This name may be updated, but each pallet in the runtime must use a unique name.
    trait Store for Module<T: Config> as TemplateModule {
        // Learn more about declaring storage items:
        // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
        MyStorage get(fn something): Option<u32>;
    }
}

// Add Errors that informs users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Config> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
    }
}

// define functions
decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// A simple call that does little more than emit an event
        #[weight = 10_000]
        fn do_something(origin, input: u32) -> DispatchResult {
            let user = ensure_signed(origin)?;

            // could do something with the input here instead
            let new_number = input;

            Self::deposit_event(RawEvent::EmitInput(user, new_number));
            Ok(())
        }

/// Test function that prints
        #[weight = 10_000]
        fn print_test(origin) -> DispatchResult {
            let user = ensure_signed(origin)?;
	    debug::info!("testprint function activated by: {:?}", user);// debug message
            print("print_test is working");// Print a message
            Ok(())
        }


/// An example dispatchable that may throw a custom error.
        #[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
        pub fn cause_error(origin) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // Read a value from storage.
            match MyStorage::get() {
                // Return an error if the value has not been set.
                None => Err(Error::<T>::NoneValue)?,
                Some(old) => {
                    // Increment the value read from storage; will error in the event of overflow.
                    let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                    // Update the value in storage with the incremented result.
                    MyStorage::put(new);
                    Ok(())
                },
            }
        }


        // add more functions here
//		#[weight = 10_000]
//		pub fn new_function(origin) - > DispatchResult {
//			unimplemented!()
//		}

    }
}

// AccountId, u32 both are inputs `=>` declaration with `<T>`
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
    {
        /// Some input was sent
        EmitInput(AccountId, u32),
    }
);
