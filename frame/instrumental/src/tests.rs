#[cfg(test)]

use crate::mock::{
    AccountId, ADMIN, ALICE, Assets, Event, ExtBuilder, Instrumental, 
    MockRuntime, Origin, System, Vault,
};
use crate::{pallet, pallet::AssetVault, pallet::Error};
use crate::currency::{
    CurrencyId, USDC
};

use pallet_vault::Vaults as VaultInfoStorage;
use composable_traits::vault::VaultConfig;

use frame_support::{
    assert_ok, assert_noop, assert_storage_noop,
    sp_std::collections::btree_map::BTreeMap,
    traits::fungibles::Inspect,
};
use sp_runtime::Perquintill;

// use proptest::prelude::*;

// ----------------------------------------------------------------------------------------------------
//                                           Helper Funcitons                                          
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
            manager: ALICE,
            reserved: Perquintill::zero(),
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
    pub assets: Vec<CurrencyId>,
}

impl Default for VaultBuilder {
    fn default() -> Self {
        VaultBuilder {
            assets: Vec::new(),
        }
    }
}

impl VaultBuilder {
    fn vault(mut self, asset: CurrencyId) -> Self {
        self.assets.push(asset);
        self
    }
    
    fn build(self) -> () {
        self.assets.iter()
            .for_each(|&asset| {
               Instrumental::create(Origin::signed(ADMIN), asset);
            })
    }
}

// ----------------------------------------------------------------------------------------------------
//                                                Create                                               
// ----------------------------------------------------------------------------------------------------

#[test]
fn create_extrinsic_emits_event() {
    ExtBuilder::default().build().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(Instrumental::create(Origin::signed(ALICE), USDC::ID));

        System::assert_last_event(Event::Instrumental(
            pallet::Event::Created { vault_id: 1u64, asset: USDC::ID }
        ));
    });
}

#[test]
fn create_extrinsic_updates_storage() {
    ExtBuilder::default().build().execute_with(|| {
        assert!(!AssetVault::<MockRuntime>::contains_key(USDC::ID));
        assert_ok!(Instrumental::create(Origin::signed(ALICE), USDC::ID));
        assert!(AssetVault::<MockRuntime>::contains_key(USDC::ID));
    });
}

#[test]
fn cannot_create_more_than_one_vault_for_an_asset() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Instrumental::create(Origin::signed(ALICE), USDC::ID));

        assert_noop!(
            Instrumental::create(Origin::signed(ALICE), USDC::ID),
            Error::<MockRuntime>::VaultAlreadyExists
        );
    });
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

        assert_ok!(Instrumental::create(Origin::signed(ALICE), USDC::ID));
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
        assert_ok!(Instrumental::create(Origin::signed(ALICE), USDC::ID));

        assert_storage_noop!(
            Instrumental::add_liquidity(Origin::signed(ALICE), USDC::ID, USDC::units(100))
        );
    });
}

// ----------------------------------------------------------------------------------------------------
//                                           Remove Liquidity                                          
// ----------------------------------------------------------------------------------------------------

#[test]
fn remove_liquidity_extrinsic_emits_event() {
    ExtBuilder::default()
        .initialize_balance(ALICE, USDC::ID, USDC::units(100))
        .build()
        .execute_with(|| {
            System::set_block_number(1);

            assert_ok!(Instrumental::create(Origin::signed(ALICE), USDC::ID));
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
        assert_ok!(Instrumental::create(Origin::signed(ALICE), asset_id));
        
        // Requirement 1) The Instrumental Pallet saves a reference to each created Vault
        assert!(AssetVault::<MockRuntime>::contains_key(asset_id));

        assert!(VaultInfoStorage::<MockRuntime>::contains_key(vault_id));

        let vault_account = 
            <Vault as composable_traits::vault::Vault>::account_id(&vault_id);
        assert_eq!(Assets::balance(USDC::ID, &vault_account), balance);
    });
}