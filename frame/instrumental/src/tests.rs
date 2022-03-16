use crate::mock::{
    ALICE, Event, ExtBuilder, Instrumental, MockRuntime, Origin, System
};
use crate::{pallet, pallet::AssetVault, pallet::Error};
use crate::currency::PICA;

use frame_support::{assert_ok, assert_noop};

// ----------------------------------------------------------------------------------------------------
//                                                Create                                               
// ----------------------------------------------------------------------------------------------------

#[test]
fn create_extrinsic_emits_event() {
    ExtBuilder::default().build().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(Instrumental::create(Origin::signed(ALICE), PICA::ID));

        System::assert_last_event(Event::Instrumental(
            pallet::Event::Created { asset: PICA::ID }
        ));
    });
}

#[test]
fn create_extrinsic_updates_storage() {
    ExtBuilder::default().build().execute_with(|| {
        assert!(!AssetVault::<MockRuntime>::contains_key(PICA::ID));
        assert_ok!(Instrumental::create(Origin::signed(ALICE), PICA::ID));
        assert!(AssetVault::<MockRuntime>::contains_key(PICA::ID));
    });
}

#[test]
fn cannot_create_more_than_one_vault_for_an_asset() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Instrumental::create(Origin::signed(ALICE), PICA::ID));

        assert_noop!(
            Instrumental::create(Origin::signed(ALICE), PICA::ID),
            Error::<MockRuntime>::VaultAlreadyExists
        );
    });
}

// ----------------------------------------------------------------------------------------------------
//                                             Add Liquidity                                           
// ----------------------------------------------------------------------------------------------------

#[test]
fn add_liquidity_extrinsic_emits_event() {
    ExtBuilder::default().build().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(Instrumental::create(Origin::signed(ALICE), PICA::ID));
        
        assert_ok!(Instrumental::add_liquidity(Origin::signed(ALICE), PICA::ID, PICA::units(100)));

        System::assert_last_event(Event::Instrumental(
            pallet::Event::AddedLiquidity { asset: PICA::ID , amount: PICA::units(100)}
        ));
    });
}

#[test]
fn add_liquidity_asset_must_have_an_associated_vault() {
    ExtBuilder::default().build().execute_with(|| {
        System::set_block_number(1);
        
        assert_noop!(
            Instrumental::add_liquidity(Origin::signed(ALICE), PICA::ID, PICA::units(100)),
            Error::<MockRuntime>::AssetDoesNotHaveAnAssociatedVault
        );
    });
}
