use composable_traits::instrumental::InstrumentalProtocolStrategy;
use frame_support::{assert_noop, assert_ok};
use primitives::currency::CurrencyId;

use crate::mock::runtime::{
	ExtBuilder, InstrumentalStrategy, MockRuntime, PabloStrategy, VaultId, MAX_ASSOCIATED_VAULTS,
};
#[allow(unused_imports)]
use crate::pallet::Error;

// -------------------------------------------------------------------------------------------------
//                                              Get Apy
// -------------------------------------------------------------------------------------------------

#[test]
fn test_get_apy() {
	ExtBuilder::default().build().execute_with(|| {
		let asset_id = CurrencyId::PICA;

		assert_eq!(InstrumentalStrategy::get_apy(asset_id), PabloStrategy::get_apy(asset_id));
	});
}

// -------------------------------------------------------------------------------------------------
//                                          Associate Vault
// -------------------------------------------------------------------------------------------------

#[test]
fn test_add_an_associated_vault() {
	ExtBuilder::default().build().execute_with(|| {
		let vault_id: VaultId = 1;

		assert_ok!(InstrumentalStrategy::associate_vault(&vault_id));
	});
}

#[test]
fn test_adding_an_associated_vault_twice_throws_an_error() {
	ExtBuilder::default().build().execute_with(|| {
		let vault_id: VaultId = 1;

		assert_ok!(InstrumentalStrategy::associate_vault(&vault_id));
		assert_noop!(
			InstrumentalStrategy::associate_vault(&vault_id),
			Error::<MockRuntime>::VaultAlreadyAssociated
		);
	});
}

#[test]
fn test_associating_too_many_vaults_throws_an_error() {
	ExtBuilder::default().build().execute_with(|| {
		for vault_id in 0..MAX_ASSOCIATED_VAULTS {
			assert_ok!(InstrumentalStrategy::associate_vault(&(vault_id as VaultId)));
		}

		let vault_id = MAX_ASSOCIATED_VAULTS as VaultId;
		assert_noop!(
			InstrumentalStrategy::associate_vault(&vault_id),
			Error::<MockRuntime>::TooManyAssociatedStrategies
		);
	});
}
