
use frame_support::{
    assert_ok,
    assert_noop,
};

use crate::{mocks::runtime::*, Error};
use crate::{mocks::currency_factory::MockCurrencyId};

#[test]
fn test_set_min_fee(){
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(MosaicVault::set_min_fee(Origin::signed(ALICE as u64), 600), Error::<Test>::MinFeeAboveMaxFee);
        assert_noop!(MosaicVault::set_min_fee(Origin::signed(ALICE as u64), 120), Error::<Test>::MinFeeAboveFeeFactor);
        assert_ok!( MosaicVault::set_min_fee(Origin::signed(ALICE as u64), 10));
        assert_eq!(MosaicVault::min_fee() ,10);
    });
}

#[test]
fn test_set_max_fee() {
    ExtBuilder::default().build().execute_with(||{
        MosaicVault::set_min_fee(Origin::signed(ALICE as u64), 10).ok();
        assert_noop!(MosaicVault::set_max_fee(Origin::signed(ALICE as u64), 200), Error::<Test>::MaxFeeAboveFeeFactor);
        assert_noop!(MosaicVault::set_max_fee(Origin::signed(ALICE as u64), 10), Error::<Test>::MaxFeeBelowMinFee);
        assert_ok!(MosaicVault::set_max_fee(Origin::signed(ALICE as u64), 15));
        assert_eq!(MosaicVault::max_fee(), 15);
    })
}

#[test]
fn test_set_asset_max_transfer_size() {
    ExtBuilder::default().build().execute_with(||{
       assert_ok!(MosaicVault::set_asset_max_transfer_size(Origin::signed(ALICE as u64), MockCurrencyId::A, 200));
       assert_eq!(MosaicVault::max_asset_transfer_size(MockCurrencyId::A), 200 );
    })
}

#[test]
fn test_set_asset_min_transfer_size() {
    ExtBuilder::default().build().execute_with(||{
       assert_ok!(MosaicVault::set_asset_min_transfer_size(Origin::signed(ALICE as u64), MockCurrencyId::A, 50));
       assert_eq!(MosaicVault::min_asset_transfer_size(MockCurrencyId::A), 50 )
    })
}