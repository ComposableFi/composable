#[allow(unused_imports)]

use crate::mock::{
    ALICE, ExtBuilder, Instrumental, Origin
};

use frame_support::{
    assert_ok, 
};

#[test]
fn call_test_extrinsic() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Instrumental::test(Origin::signed(ALICE)));
    });
}