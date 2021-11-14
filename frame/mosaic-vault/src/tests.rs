#![cfg(test)]
use super::*;
use frame_support::{
    assert_ok,
    assert_noop,
    dispatch::{
        DispatchError::BadOrigin,
    }
};
use crate::{mocks::runtime::*}; 
use crate::{mocks::currency_factory::MockCurrencyId};
use std::time::{SystemTime, UNIX_EPOCH};
use sp_runtime::{helpers_128bit::multiply_by_rational, FixedPointNumber, Percent, Perquintill};

#[test]
fn test_set_min_fee(){
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(MosaicVault::set_min_fee(Origin::signed(BOB), 600),  BadOrigin);
        assert_noop!(MosaicVault::set_min_fee(Origin::signed(ALICE), 600), Error::<Test>::MinFeeAboveMaxFee);
        assert_noop!(MosaicVault::set_min_fee(Origin::signed(ALICE), 120), Error::<Test>::MinFeeAboveFeeFactor);
        assert_ok!( MosaicVault::set_min_fee(Origin::signed(ALICE), 10));
        assert_eq!(MosaicVault::min_fee() ,10);
    });
}

#[test]
fn test_set_max_fee() {
    ExtBuilder::default().build().execute_with(||{
        assert_noop!(MosaicVault::set_max_fee(Origin::signed(BOB), 200), BadOrigin);
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
        assert_noop!(MosaicVault::set_asset_max_transfer_size(Origin::signed(BOB), MockCurrencyId::A, 200), BadOrigin);
       assert_ok!(MosaicVault::set_asset_max_transfer_size(Origin::signed(ALICE), MockCurrencyId::A, 200));
       assert_eq!(MosaicVault::max_asset_transfer_size(MockCurrencyId::A), 200 );
    })
}

#[test]
fn test_set_asset_min_transfer_size() {
    ExtBuilder::default().build().execute_with(||{
       assert_noop!(MosaicVault::set_asset_min_transfer_size(Origin::signed(BOB), MockCurrencyId::A, 50), BadOrigin);
       assert_ok!(MosaicVault::set_asset_min_transfer_size(Origin::signed(ALICE), MockCurrencyId::A, 50));
       assert_eq!(MosaicVault::min_asset_transfer_size(MockCurrencyId::A), 50 )
    })
}

#[test]
fn test_set_transfer_lockup_time() {
    ExtBuilder::default().build().execute_with(||{
        assert_noop!(MosaicVault::set_transfer_lockup_time(Origin::signed(BOB), 100), BadOrigin );
        assert_ok!(MosaicVault::set_transfer_lockup_time(Origin::signed(ALICE), 100));
        assert_eq!(MosaicVault::transfer_lockup_time(), 100 )
    })
}

#[test]
fn test_set_max_transfer_delay() {
    ExtBuilder::default().build().execute_with(||{
        assert_noop!(MosaicVault::set_max_transfer_delay(Origin::signed(BOB), 100), BadOrigin );
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
        assert_noop!(MosaicVault::set_min_transfer_delay(Origin::signed(BOB), 100), BadOrigin );
        assert_ok!(MosaicVault::set_min_transfer_delay(Origin::signed(ALICE), 100));
        // 
        assert_eq!(MosaicVault::min_transfer_delay(), 100);
        assert_noop!(MosaicVault::set_min_transfer_delay(Origin::signed(ALICE), 700), Error::<Test>::MinTransferDelayAboveMaximum);
    })
}

#[test]
fn test_set_thresh_hold() {
    ExtBuilder::default().build().execute_with(||{
        MosaicVault::set_thresh_hold(Origin::signed(ALICE), 500).ok();
        assert_noop!(MosaicVault::set_thresh_hold(Origin::signed(BOB), 100), BadOrigin );
        assert_noop!(MosaicVault::set_thresh_hold(Origin::signed(ALICE), 100), Error::<Test>::ThresholdFeeAboveThresholdFactor);

        assert_ok!(MosaicVault::set_thresh_hold(Origin::signed(ALICE), 90));
        assert_eq!(MosaicVault::fee_threshold(), 90);
    })
}

#[test]
fn test_add_supported_token() {
    ExtBuilder::default().build().execute_with(||{
        let remote_network_id: RemoteNetworkId = 100001;
        assert_noop!(MosaicVault::add_supported_token(Origin::signed(BOB), MockCurrencyId::A, MockCurrencyId::A, remote_network_id, 200, 60 ), BadOrigin );
        assert_noop!(MosaicVault::add_supported_token(Origin::signed(ALICE), MockCurrencyId::A, MockCurrencyId::A, remote_network_id, 200,300), Error::<Test>::MaxAssetTransferSizeBelowMinimum );
      
        assert_ok!(MosaicVault::add_supported_token(Origin::signed(ALICE), MockCurrencyId::A, MockCurrencyId::A, remote_network_id,  200, 100 ));
        assert_eq!(MosaicVault::max_asset_transfer_size(MockCurrencyId::A), 200);
        assert_eq!(MosaicVault::min_asset_transfer_size(MockCurrencyId::A), 100);
    })
}

#[test]
fn test_deposit() {
    ExtBuilder::default().build().execute_with(||{
        let remote_network_id: RemoteNetworkId = 1235;
        let transfer_delay: TransferDelay = 60;
        assert_noop!(MosaicVault::deposit(Origin::signed(ALICE), 100, MockCurrencyId::A, BOB, remote_network_id, transfer_delay ), Error::<Test>::UnsupportedToken);
        assert_noop!(MosaicVault::deposit(Origin::signed(ALICE), 0, MockCurrencyId::A, BOB, remote_network_id, transfer_delay ), Error::<Test>::ZeroAmount);

        MosaicVault::add_supported_token(Origin::signed(ALICE), MockCurrencyId::A, MockCurrencyId::A, remote_network_id,  200, 100 );
        assert_noop!(MosaicVault::deposit(Origin::signed(ALICE), 500, MockCurrencyId::A, BOB, remote_network_id, transfer_delay), Error::<Test>::TransferNotPossible);
        Timestamp::set_timestamp(get_epoch_ms());

        MosaicVault::set_max_transfer_delay(Origin::signed(ALICE), 1000).ok();
        MosaicVault::set_min_transfer_delay(Origin::signed(ALICE), 300).ok();

        assert_noop!(MosaicVault::deposit(Origin::signed(ALICE), 500, MockCurrencyId::A, BOB, remote_network_id, transfer_delay), Error::<Test>::TransferDelayBelowMinimum);
        MosaicVault::set_min_transfer_delay(Origin::signed(ALICE), 30).ok();

        assert_noop!(MosaicVault::deposit(Origin::signed(ALICE), 500, MockCurrencyId::A, BOB, remote_network_id, 2000), Error::<Test>::TransferDelayAboveMaximum);

        MosaicVault::set_asset_max_transfer_size(Origin::signed(ALICE), MockCurrencyId::A, 1000);
        assert_noop!(MosaicVault::deposit(Origin::signed(ALICE), 2000, MockCurrencyId::A, BOB, remote_network_id, transfer_delay), Error::<Test>::AmountAboveMaxAssetTransferSize);
        
        MosaicVault::set_asset_min_transfer_size(Origin::signed(ALICE), MockCurrencyId::A, 200);
        assert_noop!(MosaicVault::deposit(Origin::signed(ALICE), 100, MockCurrencyId::A, BOB, remote_network_id, transfer_delay), Error::<Test>::AmountBelowMinAssetTransferSize);
        
        // assert_noop!(MosaicVault::deposit(Origin::signed(ALICE), 500, MockCurrencyId::A, BOB, remote_network_id, transfer_delay), <pallet_vault::Pallet<Test>>::Error::<Test>); - how to use error type from another palet - vault - VaultDoesNotExist
      
        MosaicVault::create_vault(Origin::signed(ALICE), MockCurrencyId::A,Perquintill::from_percent(100));
        assert_ok!(MosaicVault::deposit(Origin::signed(ALICE), 500, MockCurrencyId::A, BOB, remote_network_id, transfer_delay));

        assert_eq!(MosaicVault::deposits(MockCurrencyId::A).asset_id, MockCurrencyId::A );
        assert_eq!(MosaicVault::deposits(MockCurrencyId::A).amount, 500);
        
    })
}

fn get_epoch_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}