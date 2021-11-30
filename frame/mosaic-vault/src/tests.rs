
use frame_support::{
    assert_ok,
    assert_noop,
};

use crate::{mocks::runtime::*, Error};

#[test]
fn test_set_min_fee(){
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(MosaicVault::set_min_fee(Origin::signed(ALICE as u64), 600), Error::<Test>::MinFeeAboveMaxFee);
        assert_noop!(MosaicVault::set_min_fee(Origin::signed(ALICE as u64), 120), Error::<Test>::MinFeeAboveFeeFactor);
        assert_ok!( MosaicVault::set_min_fee(Origin::signed(ALICE as u64), 2));
    });
}