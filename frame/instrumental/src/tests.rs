#[cfg(test)]

use crate::mock::{
    ALICE, Event, ExtBuilder, Instrumental, MockRuntime, Origin, System,
};
use crate::{pallet, pallet::AssetVault, pallet::Error};
use crate::currency::USDC;

use frame_support::{
    assert_ok, assert_noop, assert_storage_noop,
};

// use proptest::prelude::*;

// ----------------------------------------------------------------------------------------------------
//                                                Create                                               
// ----------------------------------------------------------------------------------------------------

#[test]
fn create_extrinsic_emits_event() {
    ExtBuilder::default().build().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(Instrumental::create(Origin::signed(ALICE), USDC::ID));

        System::assert_last_event(Event::Instrumental(
            pallet::Event::Created { asset: USDC::ID }
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