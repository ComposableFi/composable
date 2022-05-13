#[cfg(test)]

use crate::{pallet, pallet::AssetVault, pallet::Error};

use crate::mock::runtime::{
    Assets, Balance, Event, ExtBuilder, Instrumental, 
    MockRuntime, Origin, System, Vault
};
use crate::mock::currency::{
    CurrencyId, pick_currency, USDC
};
use crate::mock::account_id::{AccountId, ADMIN, pick_account};
use crate::mock::helpers::*;

use composable_traits::vault::Vault as VaultTrait;

use frame_support::{
    assert_ok, assert_noop, assert_storage_noop,
    traits::fungibles::Inspect,
};

use proptest::prelude::*;
use itertools::Itertools;

// ----------------------------------------------------------------------------------------------------
//                                             Prop_compose                                            
// ----------------------------------------------------------------------------------------------------

#[allow(dead_code)]
const TOTAL_NUM_OF_ASSETS: usize = 6;
#[allow(dead_code)]
const MINIMUM_RESERVE: Balance = 1_000;
#[allow(dead_code)]
const MAXIMUM_RESERVE: Balance = 1_000_000_000;

#[allow(dead_code)]
const TOTAL_NUM_OF_ACCOUNTS: usize = 5;

#[allow(dead_code)]
const NUMBER_OF_PROPTEST_CASES: u32 = 3u32 * TOTAL_NUM_OF_ASSETS as u32 * TOTAL_NUM_OF_ACCOUNTS as u32;

prop_compose! {
    fn generate_assets()(
        assets in prop::collection::vec(pick_currency(), 1..=TOTAL_NUM_OF_ASSETS),
    ) -> Vec<CurrencyId>{
        assets
   }
}

prop_compose! {
    fn generate_balances()(
        balances in prop::collection::vec(MINIMUM_RESERVE..MAXIMUM_RESERVE, 1..=TOTAL_NUM_OF_ASSETS),
    ) -> Vec<Balance>{
        balances
   }
}

prop_compose! {
    fn generate_accounts()(
        accounts in prop::collection::vec(pick_account(), 1..=TOTAL_NUM_OF_ACCOUNTS),
    ) -> Vec<AccountId>{
        accounts
   }
}

prop_compose! {
    fn generate_reserves()(
        assets in generate_assets(),
        balances in generate_balances(),
    ) -> Vec<(CurrencyId, Balance)>{
        assets.into_iter().unique().zip(balances.into_iter()).collect()
   }
}

prop_compose! {
    fn generate_deposits()(
        accounts in generate_accounts(),
        assets in generate_assets(),
        balances in generate_balances(),
    ) -> Vec<(AccountId, CurrencyId, Balance)>{
        accounts.into_iter()
            .zip(assets.into_iter())
            .unique()
            .zip(balances.into_iter())
            .map(|((account, asset), balance)| (account, asset, balance))
            .collect()
   }
}

// ----------------------------------------------------------------------------------------------------
//                                                Create                                               
// ----------------------------------------------------------------------------------------------------

mod create {
    use super::*;

    #[test]
    fn create_extrinsic_emits_event() {
        ExtBuilder::default().build().execute_with(|| {
            System::set_block_number(1);

            let config = InstrumentalVaultConfigBuilder::default().build();
            assert_ok!(Instrumental::create(Origin::signed(ADMIN), config));

            System::assert_last_event(Event::Instrumental(
                pallet::Event::Created { vault_id: 1u64, config }
            ));
        });
    }

    #[test]
    fn create_extrinsic_enforces_you_cannot_create_more_than_one_vault_for_an_asset() {
        ExtBuilder::default().build().execute_with(|| {
            let config = InstrumentalVaultConfigBuilder::default().build();
            assert_ok!(Instrumental::create(Origin::signed(ADMIN), config));

            assert_noop!(
                Instrumental::create(Origin::signed(ADMIN), config),
                Error::<MockRuntime>::VaultAlreadyExists
            );
        });
    }

    #[test]
    fn create_extrinsic_updates_storage() {
        ExtBuilder::default().build().execute_with(|| {
            assert!(!AssetVault::<MockRuntime>::contains_key(USDC::ID));

            let config = InstrumentalVaultConfigBuilder::default().build();
            assert_ok!(Instrumental::create(Origin::signed(ADMIN), config));

            assert!(AssetVault::<MockRuntime>::contains_key(USDC::ID));
        });
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn create_extrinsic(assets in generate_assets()) {
            ExtBuilder::default().build().execute_with(|| {
                assets.iter().for_each(|&asset| {
                    let config = InstrumentalVaultConfigBuilder::default().asset_id(asset).build();

                    if !AssetVault::<MockRuntime>::contains_key(asset) {
                        assert_ok!(Instrumental::create(Origin::signed(ADMIN), config));
                        assert!(AssetVault::<MockRuntime>::contains_key(asset));
                    } else {
                        assert_noop!(
                            Instrumental::create(Origin::signed(ADMIN), config),
                            Error::<MockRuntime>::VaultAlreadyExists
                        );
                    }
                });
            });
        }
    }
}

// ----------------------------------------------------------------------------------------------------
//                                             Add Liquidity                                           
// ----------------------------------------------------------------------------------------------------

mod add_liquidity {
    use super::*;

    #[test]
    fn add_liquidity_extrinsic_emits_event() {
        ExtBuilder::default().initialize_balance(
            ADMIN, USDC::ID, USDC::units(100)
        ).build().execute_with(|| {
            System::set_block_number(1);

            let config = InstrumentalVaultConfigBuilder::default().build();
            assert_ok!(Instrumental::create(Origin::signed(ADMIN), config));

            assert_ok!(Instrumental::add_liquidity(Origin::signed(ADMIN), USDC::ID, USDC::units(100)));

            System::assert_last_event(Event::Instrumental(
                pallet::Event::AddedLiquidity { asset: USDC::ID , amount: USDC::units(100)}
            ));
        });
    }

    #[test]
    fn add_liquidity_asset_must_have_an_associated_vault() {
        ExtBuilder::default().build().execute_with(|| {        
            assert_noop!(
                Instrumental::add_liquidity(Origin::signed(ADMIN), USDC::ID, USDC::units(100)),
                Error::<MockRuntime>::AssetDoesNotHaveAnAssociatedVault
            );
        });
    }

    #[test]
    #[allow(unused_must_use)]
    fn add_liquidity_does_not_update_storage_if_user_does_not_have_balance() {
        ExtBuilder::default().build().execute_with(|| {
            let config = InstrumentalVaultConfigBuilder::default().build();
            assert_ok!(Instrumental::create(Origin::signed(ADMIN), config));
            
            assert_storage_noop!(
                Instrumental::add_liquidity(Origin::signed(ADMIN), USDC::ID, USDC::units(100))
            );
        });
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(NUMBER_OF_PROPTEST_CASES))]

        #[test]
        fn add_liquidity_extrinsic(
            assets in generate_assets(),
            deposits in generate_deposits()
        ) {
            // Create a VaultConfig object for each asset in assets
            let configs = assets.iter().map(|&asset| {
                InstrumentalVaultConfigBuilder::default().asset_id(asset).build()
            }).collect();

            ExtBuilder::default().initialize_balances(deposits.clone()).build()
                .initialize_vaults(configs).execute_with(|| {                   
                    // Have each account try to deposit an asset balance into an Instrumental vault
                    deposits.into_iter().for_each(|(account, asset, balance)| {
                        if AssetVault::<MockRuntime>::contains_key(asset) {
                            assert_ok!(Instrumental::add_liquidity(Origin::signed(account), asset, balance));
                        } else {
                            assert_noop!(
                                Instrumental::add_liquidity(Origin::signed(account), asset, balance),
                                Error::<MockRuntime>::AssetDoesNotHaveAnAssociatedVault
                            );
                        }
                    });
            });
        }

        #[test]
        fn add_liquidity_extrinsic_transfers_liquidity(
            deposits in generate_deposits()
        ) {
            // Create a VaultConfig object for each asset in deposits
            let configs = deposits.iter().map(|&(_, asset, _)| {
                InstrumentalVaultConfigBuilder::default().asset_id(asset).build()
            }).collect();

            ExtBuilder::default().initialize_balances(deposits.clone()).build()
                .initialize_vaults(configs).execute_with(|| {
                    // Have each account try to deposit an asset balance into an Instrumental vault
                    deposits.into_iter().for_each(|(account, asset, balance)| {
                        let vault_id = Instrumental::asset_vault(asset).unwrap();
                        let vault_account = Vault::account_id(&vault_id);
                        let vault_balance_before_deposit = Assets::balance(asset, &vault_account);

                        assert_ok!(Instrumental::add_liquidity(Origin::signed(account), asset, balance));

                        // Requirement 1: user transferred their balance
                        assert_eq!(Assets::balance(asset, &account), 0);

                        // Requirement 2: the vault holds the transferred balance
                        assert_eq!(Assets::balance(asset, &vault_account), vault_balance_before_deposit + balance);
                    });
            });
        }
    }
}

// ----------------------------------------------------------------------------------------------------
//                                           Remove Liquidity                                          
// ----------------------------------------------------------------------------------------------------

mod remove_liquidity {
    use super::*;

    #[test]
    fn remove_liquidity_extrinsic_emits_event() {
        let config = InstrumentalVaultConfigBuilder::default().build();

        ExtBuilder::default()
            .initialize_balance(ADMIN, USDC::ID, USDC::units(100))
            .build()
            .initialize_vault(config)
            .execute_with(|| {
                System::set_block_number(1);

                assert_ok!(Instrumental::add_liquidity(Origin::signed(ADMIN), USDC::ID, USDC::units(100)));
                assert_ok!(Instrumental::remove_liquidity(Origin::signed(ADMIN), USDC::ID, USDC::units(100)));

                System::assert_last_event(Event::Instrumental(
                    pallet::Event::RemovedLiquidity { asset: USDC::ID , amount: USDC::units(100)}
                ));
        });
    }

    #[test]
    fn remove_liquidity_asset_must_have_an_associated_vault() {
        ExtBuilder::default().build().execute_with(|| {
            System::set_block_number(1);
            
            assert_noop!(
                Instrumental::remove_liquidity(Origin::signed(ADMIN), USDC::ID, USDC::units(100)),
                Error::<MockRuntime>::AssetDoesNotHaveAnAssociatedVault
            );
        });
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(NUMBER_OF_PROPTEST_CASES))]

        #[test]
        fn remove_liquidity_extrinsic(
            reserves in generate_reserves(),
            withdraws in generate_deposits()
        ) {
            // Create a VaultConfig object for each asset in reserves
            let configs = reserves.iter().map(|&(asset, _)| {
                InstrumentalVaultConfigBuilder::default().asset_id(asset).build()
            }).collect();

            ExtBuilder::default().build().initialize_vaults_with_reserves(configs, reserves).execute_with(|| {        
                // Have each account try to withdraw an asset balance from an Instrumental vault
                withdraws.into_iter().for_each(|(account, asset, balance)| {
                    if !AssetVault::<MockRuntime>::contains_key(asset) {
                        assert_noop!(
                            Instrumental::remove_liquidity(Origin::signed(account), asset, balance),
                            Error::<MockRuntime>::AssetDoesNotHaveAnAssociatedVault
                        );
                    } else {
                        let vault_id = Instrumental::asset_vault(asset).unwrap();
                        let vault_account = Vault::account_id(&vault_id);
                        
                        if Assets::balance(asset, &vault_account) >= balance {
                            assert_ok!(Instrumental::remove_liquidity(Origin::signed(account), asset, balance));
                        } else {
                            assert_noop!(
                                Instrumental::remove_liquidity(Origin::signed(account), asset, balance),
                                Error::<MockRuntime>::NotEnoughLiquidity
                            );
                        }
                    }
                });
            });
        }

        #[test]
        fn remove_liquidity_extrinsic_transfers_liquidity(
            deposits in generate_deposits()
        ) {
            // Create a VaultConfig object for each asset in deposits
            let configs = deposits.iter().map(|&(_, asset, _)| {
                InstrumentalVaultConfigBuilder::default().asset_id(asset).build()
            }).collect();
            
            ExtBuilder::default().initialize_balances(deposits.clone()).build().initialize_vaults(configs).execute_with(|| {                    
                // Have each account try to deposit an asset balance into an Instrumental vault
                deposits.iter().for_each(|(account, asset, balance)| {
                    assert_ok!(Instrumental::add_liquidity(Origin::signed(*account), *asset, *balance));
                });

                deposits.into_iter().for_each(|(account, asset, balance)| {
                    // Requirement 1: user has no balance of the asset
                    assert_eq!(Assets::balance(asset, &account), 0);

                    let vault_id = Instrumental::asset_vault(asset).unwrap();
                    let vault_account = Vault::account_id(&vault_id);
                    let vault_balance_before_withdraw = Assets::balance(asset, &vault_account);

                    assert_ok!(Instrumental::remove_liquidity(Origin::signed(account), asset, balance));

                    // Requirement 2: user has some balance of the asset
                    assert_eq!(Assets::balance(asset, &account), balance);

                    // Requirement 3: the vault holds the transferred balance
                    assert_eq!(Assets::balance(asset, &vault_account), vault_balance_before_withdraw - balance);
                });
            });
        }
    }
}

// ----------------------------------------------------------------------------------------------------
//                                              ExtBuilder                                             
// ----------------------------------------------------------------------------------------------------

mod ext_builder {
    use super::*;

    #[test]
    fn ext_builder_initialize_balance() {
        let user = ADMIN;
        let (asset, balance) = (USDC::ID, USDC::units(100));

        ExtBuilder::default().initialize_balance(user, asset, balance).build().execute_with(|| {                    
            assert_eq!(Assets::balance(asset, &user), balance);
        });
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(NUMBER_OF_PROPTEST_CASES))]

        #[test]
        fn ext_builder_initialize_balances(
            deposits in generate_deposits()
        ) {         
            ExtBuilder::default().initialize_balances(deposits.clone()).build().execute_with(|| {                    
                deposits.into_iter().for_each(|(user, asset, balance)| {
                    assert_eq!(Assets::balance(asset, &user), balance);
                });
            });
        }
    }
}

// ----------------------------------------------------------------------------------------------------
//                                       spi_io::TestExternalities                                     
// ----------------------------------------------------------------------------------------------------

mod spi_io_test_externalities {
    use super::*;

    #[test]
    fn test_externalities_initialize_vault() {
        let asset = USDC::ID;
        let config = InstrumentalVaultConfigBuilder::default().asset_id(asset).build();
        
        ExtBuilder::default().build().initialize_vault(config).execute_with(|| {                    
            assert!(AssetVault::<MockRuntime>::contains_key(asset));
        });
    }

    #[test]
    fn test_externalities_initialize_reserve() {
        let asset = USDC::ID;
        let config = InstrumentalVaultConfigBuilder::default().asset_id(asset).build();
        
        let balance = USDC::units(1_000);
        ExtBuilder::default().build().initialize_vault(config).initialize_reserve(asset, balance).execute_with(|| {
            assert!(AssetVault::<MockRuntime>::contains_key(asset));
                    
            let vault_id = Instrumental::asset_vault(asset).unwrap();
            let vault_account = Vault::account_id(&vault_id);
            assert_eq!(Assets::balance(asset, &vault_account), balance);
        });
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(NUMBER_OF_PROPTEST_CASES))]

        #[test]
        fn test_externalities_initialize_vaults(
            assets in generate_assets()
        ) {
            let configs = assets.iter().map(|&asset| {
                InstrumentalVaultConfigBuilder::default().asset_id(asset).build()
            }).collect();

            ExtBuilder::default().build().initialize_vaults(configs).execute_with(|| {                    
                assets.iter().for_each(|&asset| assert!(AssetVault::<MockRuntime>::contains_key(asset)));
            });
        }

        #[test]
        fn test_externalities_initialize_vaults_with_reserves(
            reserves in generate_reserves()
        ) {
            let configs = reserves.iter().map(|&(asset, _)| {
                InstrumentalVaultConfigBuilder::default().asset_id(asset).build()
            }).collect();

            ExtBuilder::default().build().initialize_vaults_with_reserves(configs, reserves.clone()).execute_with(|| {                    
                reserves.iter().for_each(|&(asset, balance)| {
                    assert!(AssetVault::<MockRuntime>::contains_key(asset));
                    
                    let vault_id = Instrumental::asset_vault(asset).unwrap();
                    let vault_account = Vault::account_id(&vault_id);
                    assert_eq!(Assets::balance(asset, &vault_account), balance);
                });
            });
        }

    }
}