use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;
use orml_tokens::{AccountData};

#[test]
fn initiate() {
	new_test_ext().execute_with(|| {
		assert_ok!(LiquidCrowdloan::initiate(Origin::root(), 1, 100));
		let token_id = LiquidCrowdloan::token_id();
		assert_eq!(token_id, Some(1));
		let balance = AccountData {
			free: 100,
			reserved: 0,
			frozen: 0
		};
		assert_eq!(Tokens::accounts(1, token_id.unwrap()), balance);

		assert_noop!(LiquidCrowdloan::initiate(Origin::signed(1), 1, 100), BadOrigin);
		assert_noop!(LiquidCrowdloan::initiate(Origin::root(), 1, 100), Error::<Test>::AlreadyInitiated);
	});
}

#[test]
fn make_claimable() {
	new_test_ext().execute_with(|| {
		assert_ok!(LiquidCrowdloan::make_claimable(Origin::root()));
		assert_eq!(LiquidCrowdloan::is_claimable(), Some(true));
		assert_noop!(LiquidCrowdloan::make_claimable(Origin::signed(1)), BadOrigin);
	});
}
