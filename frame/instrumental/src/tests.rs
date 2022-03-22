#[cfg(test)]

use crate::mock::{
    AccountId, ADMIN, ALICE, Assets, Balance, Event, ExtBuilder, Instrumental, 
    MockRuntime, Origin, System, Vault,
};
use crate::{pallet, pallet::AssetVault, pallet::Error};
use crate::currency::{
    CurrencyId, pick_currency, USDC
};

use pallet_vault::Vaults as VaultInfoStorage;
use composable_traits::vault::VaultConfig;

use frame_support::{
    assert_ok, assert_noop, assert_storage_noop,
    sp_std::collections::btree_map::BTreeMap,
    traits::fungibles::Inspect,
};
use sp_runtime::Perquintill;

use proptest::prelude::*;

// ----------------------------------------------------------------------------------------------------
//                                           Helper Functions                                          
// ----------------------------------------------------------------------------------------------------

struct VaultConfigBuilder {
    pub asset_id: CurrencyId,
    pub manager: AccountId,
    pub reserved: Perquintill,
    pub strategies: BTreeMap<AccountId, Perquintill>,
}

impl Default for VaultConfigBuilder {
    fn default() -> Self {
        VaultConfigBuilder {
            asset_id: USDC::ID,
            manager: ADMIN,
            reserved: Perquintill::one(),
            strategies: BTreeMap::new(),
        }
    }
}

#[allow(dead_code)]
impl VaultConfigBuilder {

    fn asset_id(mut self, asset: CurrencyId) -> Self {
        self.asset_id = asset;
        self
    }

    fn reserved(mut self, reserved: Perquintill) -> Self {
        self.reserved = reserved;
        self
    }

    fn manager(mut self, manager: AccountId) -> Self {
        self.manager = manager;
        self
    }

    fn strategy(mut self, account: AccountId, strategy: Perquintill) -> Self {
        self.strategies.insert(account, strategy);
        self
    }
    
    fn build(self) -> VaultConfig<AccountId, CurrencyId> {
        VaultConfig {
            asset_id: self.asset_id,
            reserved: self.reserved,
            manager: self.manager,
            strategies: self.strategies,
        }
    }
}

struct VaultBuilder {
    pub configs: Vec<VaultConfig<AccountId, CurrencyId>>,
}

impl Default for VaultBuilder {
    fn default() -> Self {
        VaultBuilder {
            configs: Vec::new(),
        }
    }
}

#[allow(dead_code)]
impl VaultBuilder {
    fn vault(mut self, config: VaultConfig<AccountId, CurrencyId>) -> Self {
        self.configs.push(config);
        self
    }
    
    fn build(self) -> () {
        // TODO: (Nevin)
        //  - remove duplicate assets
        self.configs.iter()
            .for_each(|config| {
               Instrumental::create(
                   Origin::signed(ADMIN), 
                   config.clone()
                ).ok();
            })
    }
}

// ----------------------------------------------------------------------------------------------------
//                                             Prop_compose                                            
// ----------------------------------------------------------------------------------------------------

prop_compose! {
    fn generate_centre()(centre in -1000..1000) -> i32 {
        centre
    }
}

// initial_assets in prop::collection::vec(strategy_pick_random_mock_currency(), 2usize..26usize),

#[allow(dead_code)]
const TOTAL_NUM_OF_ASSETS: usize = 6;
#[allow(dead_code)]
const MINIMUM_RESERVE: Balance = 1_000;
#[allow(dead_code)]
const MAXIMUM_RESERVE: Balance = 1_000_000_000;

prop_compose! {
    fn generate_assets()
        (
            assets in prop::collection::vec(pick_currency(), 1..=TOTAL_NUM_OF_ASSETS),
        ) -> Vec<CurrencyId>{

            assets
   }
}

prop_compose! {
    fn generate_reserves()
        (
            assets in prop::collection::vec(pick_currency(), 1..=TOTAL_NUM_OF_ASSETS),
            balances in prop::collection::vec(MINIMUM_RESERVE..MAXIMUM_RESERVE, 1..=TOTAL_NUM_OF_ASSETS),
        ) -> Vec<(CurrencyId, Balance)>{

            assets.into_iter().zip(balances.into_iter()).collect()
   }
}

// ----------------------------------------------------------------------------------------------------
//                                                Create                                               
// ----------------------------------------------------------------------------------------------------

#[test]
fn create_extrinsic_emits_event() {
    ExtBuilder::default().build().execute_with(|| {
        System::set_block_number(1);

        let config = VaultConfigBuilder::default().build();
        assert_ok!(Instrumental::create(Origin::signed(ADMIN), config.clone()));

        System::assert_last_event(Event::Instrumental(
            pallet::Event::Created { vault_id: 1u64, config: config}
        ));
    });
}

#[test]
fn cannot_create_more_than_one_vault_for_an_asset() {
    ExtBuilder::default().build().execute_with(|| {
        let config = VaultConfigBuilder::default().build();
        assert_ok!(Instrumental::create(Origin::signed(ADMIN), config.clone()));

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

        let config = VaultConfigBuilder::default().build();
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
                let config = VaultConfigBuilder::default().asset_id(asset).build();

                if AssetVault::<MockRuntime>::contains_key(asset) {
                    assert_noop!(
                        Instrumental::create(Origin::signed(ADMIN), config),
                        Error::<MockRuntime>::VaultAlreadyExists
                    );
                } else {
                    assert_ok!(Instrumental::create(Origin::signed(ADMIN), config));
                    assert!(AssetVault::<MockRuntime>::contains_key(asset));
                }
            });
        });
    }
}

// ----------------------------------------------------------------------------------------------------
//                                             Add Liquidity                                           
// ----------------------------------------------------------------------------------------------------

#[test]
fn add_liquidity_extrinsic_emits_event() {
    ExtBuilder::default().initialize_balance(
        ALICE, USDC::ID, USDC::units(100)
    ).build().execute_with(|| {
        System::set_block_number(1);

        let config = VaultConfigBuilder::default().build();
        assert_ok!(Instrumental::create(Origin::signed(ADMIN), config));

        assert_ok!(Instrumental::add_liquidity(Origin::signed(ALICE), USDC::ID, USDC::units(100)));

        System::assert_last_event(Event::Instrumental(
            pallet::Event::AddedLiquidity { asset: USDC::ID , amount: USDC::units(100)}
        ));
    });
}

#[test]
fn add_liquidity_asset_must_have_an_associated_vault() {
    ExtBuilder::default().build().execute_with(|| {
        System::set_block_number(1);
        
        assert_noop!(
            Instrumental::add_liquidity(Origin::signed(ALICE), USDC::ID, USDC::units(100)),
            Error::<MockRuntime>::AssetDoesNotHaveAnAssociatedVault
        );
    });
}

#[test]
#[allow(unused_must_use)]
fn add_liquidity_does_not_update_storage_if_user_does_not_have_balance() {
    ExtBuilder::default().build().execute_with(|| {
        let config = VaultConfigBuilder::default().build();
        assert_ok!(Instrumental::create(Origin::signed(ADMIN), config));

        assert_storage_noop!(
            Instrumental::add_liquidity(Origin::signed(ALICE), USDC::ID, USDC::units(100))
        );
    });
}

// // ----------------------------------------------------------------------------------------------------
// //                                           Remove Liquidity                                          
// // ----------------------------------------------------------------------------------------------------

#[test]
fn remove_liquidity_extrinsic_emits_event() {
    ExtBuilder::default()
        .initialize_balance(ALICE, USDC::ID, USDC::units(100))
        .build()
        .execute_with(|| {
            System::set_block_number(1);

            let config = VaultConfigBuilder::default().build();
            assert_ok!(Instrumental::create(Origin::signed(ADMIN), config));

            assert_ok!(Instrumental::add_liquidity(Origin::signed(ALICE), USDC::ID, USDC::units(100)));
            assert_ok!(Instrumental::remove_liquidity(Origin::signed(ALICE), USDC::ID, USDC::units(100)));

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
            Instrumental::remove_liquidity(Origin::signed(ALICE), USDC::ID, USDC::units(100)),
            Error::<MockRuntime>::AssetDoesNotHaveAnAssociatedVault
        );
    });
}

// ----------------------------------------------------------------------------------------------------
//                                              ExtBuilder                                             
// ----------------------------------------------------------------------------------------------------

#[test]
fn initialize_vault() {
    let vault_id = 1u64;
    let (asset_id, balance) = (USDC::ID, USDC::units(100));

    ExtBuilder::default().initialize_vault(asset_id, balance).build().execute_with(|| {
        let config = VaultConfigBuilder::default().asset_id(asset_id).build();
        assert_ok!(Instrumental::create(Origin::signed(ADMIN), config));
        
        // Requirement 1) The Instrumental Pallet saves a reference to each created Vault
        assert!(AssetVault::<MockRuntime>::contains_key(asset_id));

        assert!(VaultInfoStorage::<MockRuntime>::contains_key(vault_id));

        let vault_account = 
            <Vault as composable_traits::vault::Vault>::account_id(&vault_id);
        assert_eq!(Assets::balance(USDC::ID, &vault_account), balance);
    });
}