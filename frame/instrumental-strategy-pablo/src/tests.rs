#[allow(unused_imports)]

use crate::{pallet, pallet::Error};
use crate::mock::runtime::{
    Event, ExtBuilder, MAX_ASSOCIATED_VAULTS, MockRuntime, PabloStrategy, VaultId, System
};

use frame_support::{assert_ok, assert_noop};

// -----------------------------------------------------------------------------------------------
//                                         Associate Vault                                        
// -----------------------------------------------------------------------------------------------

#[test]
fn test_add_an_associated_vault() {
    ExtBuilder::default().build().execute_with(|| {
        let vault_id: VaultId = 1;

        assert_ok!(PabloStrategy::associate_vault(&vault_id));
    });
}

#[test]
fn test_adding_an_associated_vault_twice_throws_an_error() {
    ExtBuilder::default().build().execute_with(|| {
        let vault_id: VaultId = 1;

        assert_ok!(PabloStrategy::associate_vault(&vault_id));
        assert_noop!(
            PabloStrategy::associate_vault(&vault_id), 
            Error::<MockRuntime>::VaultAlreadyAssociated
        );
    });
}

#[test]
fn test_associating_too_many_vaults_throws_an_error() {
    ExtBuilder::default().build().execute_with(|| {
        for vault_id in 0..MAX_ASSOCIATED_VAULTS {
            assert_ok!(PabloStrategy::associate_vault(&(vault_id as VaultId)));
        }

        let vault_id = MAX_ASSOCIATED_VAULTS as VaultId;
        assert_noop!(
            PabloStrategy::associate_vault(&vault_id), 
            Error::<MockRuntime>::TooManyAssociatedStrategies
        );
    });
}

// -----------------------------------------------------------------------------------------------
//                                            Rebalance                                           
// -----------------------------------------------------------------------------------------------

#[test]
fn test_rebalance_emits_event() {
    ExtBuilder::default().build().execute_with(|| {
        System::set_block_number(1);

        let vault_id: VaultId = 1;
        assert_ok!(PabloStrategy::associate_vault(&(vault_id as VaultId)));

        assert_ok!(PabloStrategy::rebalance());

        System::assert_last_event(Event::PabloStrategy(
            pallet::Event::RebalancedVault { vault_id }
        ));
    });
}