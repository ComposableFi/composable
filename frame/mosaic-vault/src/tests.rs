
use frame_support::{
    assert_ok,
};

use crate::{
    mocks::{
        currency_factory::MockCurrencyId,
        runtime::{
			AccountId, Balance, BlockNumber, ExtBuilder, Origin, Test, Tokens, Vault, MosaicVault, 
			ACCOUNT_FREE_START, ALICE, BOB, CHARLIE, MINIMUM_BALANCE,
		},
    }
};

#[test]
fn test_set_min_fee(){
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!( 
            MosaicVault::set_min_fee(Origin::signed(ALICE as u64), 12)
         );
    })
}