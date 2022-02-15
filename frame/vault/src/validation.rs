use composable_support::validation::Validate;
use core::marker::PhantomData;
use frame_support::traits::Get;
use crate::pallet::{BalanceOf, Config};

#[derive(Clone, Copy)]
pub struct ValidateCreationDeposit<T> {
    _marker: PhantomData<T>,
}

impl<T: Config> Validate<BalanceOf<T>, ValidateCreationDeposit<T>> for ValidateCreationDeposit<T> {
    fn validate(input: BalanceOf<T>) -> Result<BalanceOf<T>, &'static str> {
       if input < T::CreationDeposit::get() {
            return Err("Insufficent Creation Deposit")
        }

        Ok(input)
    }
}
