use composable_support::validation::Validate;
use support::{pallet_prelude::*};
use crate::pallet::{Config, CallFilterEntryOf};

#[derive(Debug, Decode, Copy, Clone)]
pub struct ValidEntry<T>{
    _marker: PhantomData<T>,
}

impl<T: Config> Validate<CallFilterEntryOf<T>, ValidEntry<T>> for ValidEntry<T> {

    fn validate(input: CallFilterEntryOf<T>) -> Result<CallFilterEntryOf<T>, &'static str>{

        if !input.valid() {
            return Err("INVALID_STRING")
        }

        Ok(input)
    }
}



