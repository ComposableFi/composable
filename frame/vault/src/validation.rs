use crate::pallet::{BalanceOf, Config};
use composable_support::validation::Validate;
use composable_traits::vault::VaultConfig;
use core::marker::PhantomData;
use frame_support::traits::Get;

#[derive(Clone, Copy)]
pub struct ValidateCreationDeposit<T> {
	_marker: PhantomData<T>,
}

#[derive(Clone, Copy)]
pub struct ValidateMaxStrategies<T> {
	_marker: PhantomData<T>,
}

impl<T: Config> Validate<BalanceOf<T>, ValidateCreationDeposit<T>> for ValidateCreationDeposit<T> {
	fn validate(input: BalanceOf<T>) -> Result<BalanceOf<T>, &'static str> {
		if input < T::CreationDeposit::get() {
			return Err("Insufficient Creation Deposit")
		}

		Ok(input)
	}
}

impl<T: Config> Validate<VaultConfig<T::AccountId, T::AssetId>, ValidateMaxStrategies<T>>
	for ValidateMaxStrategies<T>
{
	fn validate(
		input: VaultConfig<T::AccountId, T::AssetId>,
	) -> Result<VaultConfig<T::AccountId, T::AssetId>, &'static str> {
		if input.strategies.len() > T::MaxStrategies::get() {
			return Err("Too Many Strategies")
		}

		Ok(input)
	}
}
