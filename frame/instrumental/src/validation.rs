use crate::pallet::{AssetVault, Config, InstrumentalVaultConfigFor};

use composable_traits::instrumental::InstrumentalVaultConfig;
use composable_support::validation::Validate;

use core::marker::PhantomData;
use sp_runtime::Perquintill;

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
