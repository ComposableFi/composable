#![cfg(test)]
use super::*;
use frame_support::{
    assert_ok,
    assert_noop,
    dispatch::{
        DispatchError
    }
};
// use frame_support::dispatch::DispatchError;

use crate::{mocks::runtime::*};
// use frame_system::pallet::Error;
// use crate::{mocks::runtime::*, Error};
use crate::{mocks::currency_factory::MockCurrencyId};

#[test]
fn test_set_min_fee(){
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(MosaicVault::set_min_fee(Origin::signed(BOB), 600),  DispatchError::BadOrigin);
        assert_noop!(MosaicVault::set_min_fee(Origin::signed(ALICE), 600), Error::<Test>::MinFeeAboveMaxFee);
        assert_noop!(MosaicVault::set_min_fee(Origin::signed(ALICE), 120), Error::<Test>::MinFeeAboveFeeFactor);
        assert_ok!( MosaicVault::set_min_fee(Origin::signed(ALICE), 10));
        assert_eq!(MosaicVault::min_fee() ,10);
    });
}

#[test]
fn test_set_max_fee() {
    ExtBuilder::default().build().execute_with(||{
        assert_noop!(MosaicVault::set_max_fee(Origin::signed(BOB), 200), DispatchError::BadOrigin);
        MosaicVault::set_min_fee(Origin::signed(ALICE), 10).ok();
        assert_noop!(MosaicVault::set_max_fee(Origin::signed(ALICE), 200), Error::<Test>::MaxFeeAboveFeeFactor);
        assert_noop!(MosaicVault::set_max_fee(Origin::signed(ALICE), 10), Error::<Test>::MaxFeeBelowMinFee);
        assert_ok!(MosaicVault::set_max_fee(Origin::signed(ALICE), 15));
        assert_eq!(MosaicVault::max_fee(), 15);
    })
}

#[test]
fn test_set_asset_max_transfer_size() {
    ExtBuilder::default().build().execute_with(||{
        assert_noop!(MosaicVault::set_asset_max_transfer_size(Origin::signed(BOB), MockCurrencyId::A, 200), DispatchError::BadOrigin);
       assert_ok!(MosaicVault::set_asset_max_transfer_size(Origin::signed(ALICE), MockCurrencyId::A, 200));
       assert_eq!(MosaicVault::max_asset_transfer_size(MockCurrencyId::A), 200 );
    })
}

#[test]
fn test_set_asset_min_transfer_size() {
    ExtBuilder::default().build().execute_with(||{
       assert_noop!(MosaicVault::set_asset_min_transfer_size(Origin::signed(BOB), MockCurrencyId::A, 50), DispatchError::BadOrigin);
       assert_ok!(MosaicVault::set_asset_min_transfer_size(Origin::signed(ALICE), MockCurrencyId::A, 50));
       assert_eq!(MosaicVault::min_asset_transfer_size(MockCurrencyId::A), 50 )
    })
}

#[test]
fn test_set_transfer_lockup_time() {
    ExtBuilder::default().build().execute_with(||{
        assert_noop!(MosaicVault::set_transfer_lockup_time(Origin::signed(BOB), 100), DispatchError::BadOrigin );
        assert_ok!(MosaicVault::set_transfer_lockup_time(Origin::signed(ALICE), 100));
        assert_eq!(MosaicVault::transfer_lockup_time(), 100 )
    })
}

#[test]
fn test_set_max_transfer_delay() {
    ExtBuilder::default().build().execute_with(||{
        assert_noop!(MosaicVault::set_max_transfer_delay(Origin::signed(BOB), 100), DispatchError::BadOrigin );
        assert_ok!(MosaicVault::set_max_transfer_delay(Origin::signed(ALICE), 100));
        assert_eq!(MosaicVault::max_transfer_delay(), 100);
        //
        MosaicVault::set_min_transfer_delay(Origin::signed(ALICE), 90).ok();
        assert_noop!(MosaicVault::set_max_transfer_delay(Origin::signed(ALICE), 80), Error::<Test>::MaxTransferDelayBelowMinimum);
    })
}

#[test]
fn test_set_min_transfer_delay() {
    ExtBuilder::default().build().execute_with(||{
        MosaicVault::set_max_transfer_delay(Origin::signed(ALICE), 500).ok();
        assert_noop!(MosaicVault::set_min_transfer_delay(Origin::signed(BOB), 100), DispatchError::BadOrigin );
        assert_ok!(MosaicVault::set_min_transfer_delay(Origin::signed(ALICE), 100));
        // 
        assert_eq!(MosaicVault::min_transfer_delay(), 100);
        assert_noop!(MosaicVault::set_min_transfer_delay(Origin::signed(ALICE), 700), Error::<Test>::MinTransferDelayAboveMaximum);
    })
}