#[allow(unused_imports)]

use crate::mock::{
    ALICE, Event, ExtBuilder, Instrumental, Origin, System
};
use crate::pallet;

use frame_support::assert_ok;

#[test]
fn call_test_extrinsic() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Instrumental::test(Origin::signed(ALICE)));
    });
}

#[test]
fn test_extrinsic_emits_event() {
    ExtBuilder::default().build().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(Instrumental::test(Origin::signed(ALICE)));

        System::assert_last_event(Event::Instrumental(
            pallet::Event::Test { account: ALICE}
        ));
    });
}