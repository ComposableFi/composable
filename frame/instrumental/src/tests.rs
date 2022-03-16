#[allow(unused_imports)]

use crate::mock::{
    ALICE, Event, ExtBuilder, Instrumental, Origin, System
};
use crate::pallet;
use crate::currency::PICA;

use frame_support::assert_ok;

#[test]
fn call_create_extrinsic() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Instrumental::create(Origin::signed(ALICE), PICA::ID));
    });
}

#[test]
fn test_create_emits_event() {
    ExtBuilder::default().build().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(Instrumental::create(Origin::signed(ALICE), PICA::ID));

        System::assert_last_event(Event::Instrumental(
            pallet::Event::Create { asset: PICA::ID }
        ));
    });
}