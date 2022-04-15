use crate::pallet::{AssetVault, Config, InstrumentalVaultConfigFor};

use composable_traits::instrumental::InstrumentalVaultConfig;
use composable_support::validation::Validate;

use core::marker::PhantomData;
use sp_runtime::Perquintill;

// #[derive(Clone, Copy)]
// pub struct ValidateInstrumentalVaultConfig<T> {
// 	_marker: PhantomData<T>,
// }

// impl<T: Config> Validate<InstrumentalVaultConfig<T::AssetId, Perquintill>, ValidateInstrumentalVaultConfig<T>>
// 	for ValidateInstrumentalVaultConfig<T>
// {
// 	fn validate(
// 		input: InstrumentalVaultConfig<T::AssetId, Perquintill>,
// 	) -> Result<InstrumentalVaultConfig<T::AssetId, Perquintill>, &'static str> {
// 		if AssetVault::<T>::contains_key(input.asset_id) {
//             return Err("Vault Already Exists")
//         }

//         if input.percent_deployable < Perquintill::zero() || Perquintill::one() < input.percent_deployable {
//             return Err("Invalid Deployable Percent")
//         }

// 		Ok(input)
// 	}
// }

// -----------------------------------------------------------------------------------------------
//                                    ValidateVaultDoesNotExist                                   
// -----------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct ValidateVaultDoesNotExist<T> {
	_marker: PhantomData<T>,
}

impl<T: Config> Validate<InstrumentalVaultConfigFor<T>, ValidateVaultDoesNotExist<T>>
	for ValidateVaultDoesNotExist<T>
{
	fn validate(
		input: InstrumentalVaultConfig<T::AssetId, Perquintill>,
	) -> Result<InstrumentalVaultConfig<T::AssetId, Perquintill>, &'static str> {
		if AssetVault::<T>::contains_key(input.asset_id) {
            return Err("Vault Already Exists")
        }

		Ok(input)
	}
}

// -----------------------------------------------------------------------------------------------
//                                    ValidatePercentDeployable                                   
// -----------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct ValidatePercentDeployable<T> {
	_marker: PhantomData<T>,
}

impl<T: Config> Validate<InstrumentalVaultConfigFor<T>, ValidatePercentDeployable<T>>
	for ValidatePercentDeployable<T>
{
	fn validate(
		input: InstrumentalVaultConfig<T::AssetId, Perquintill>,
	) -> Result<InstrumentalVaultConfig<T::AssetId, Perquintill>, &'static str> {
		if input.percent_deployable < Perquintill::zero() || 
            Perquintill::one() < input.percent_deployable {
                return Err("Invalid Deployable Percent")
        }

		Ok(input)
	}
}