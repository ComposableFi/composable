use crate::{mock::*, Error};
use frame_support::{
	assert_noop, assert_ok,
	traits::{Currency},
};use sp_runtime::traits::BadOrigin;
use orml_tokens::{AccountData};

#[test]
fn initiate() {
	new_test_ext().execute_with(|| {
		assert_ok!(LiquidCrowdloan::initiate(Origin::signed(2), 1, 100));
		let token_id = LiquidCrowdloan::token_id();
		assert_eq!(token_id, Some(1));
		let balance = AccountData {
			free: 100,
			reserved: 0,
			frozen: 0
		};
		assert_eq!(Tokens::accounts(1, token_id.unwrap()), balance);

		assert_noop!(LiquidCrowdloan::initiate(Origin::signed(1), 1, 100), BadOrigin);
		assert_noop!(LiquidCrowdloan::initiate(Origin::signed(2), 1, 100), Error::<Test>::AlreadyInitiated);
	});
}

#[test]
fn make_claimable() {
	new_test_ext().execute_with(|| {
		assert_ok!(LiquidCrowdloan::make_claimable(Origin::signed(2)));
		assert_eq!(LiquidCrowdloan::is_claimable(), Some(true));
		assert_noop!(LiquidCrowdloan::make_claimable(Origin::signed(1)), BadOrigin);
	});
}

#[test]
fn claim() {
	new_test_ext().execute_with(|| {
		assert_ok!(LiquidCrowdloan::initiate(Origin::signed(2), 1, 200));
		assert_noop!(LiquidCrowdloan::claim(Origin::signed(1), 100), Error::<Test>::NotClaimable);
		assert_ok!(LiquidCrowdloan::make_claimable(Origin::signed(2)));
		NativeBalances::make_free_balance_be(&LiquidCrowdloan::account_id(), 100);
		assert_eq!(NativeBalances::free_balance(LiquidCrowdloan::account_id()), 100);


		assert_noop!(LiquidCrowdloan::claim(Origin::signed(1), 300), Error::<Test>::InsufficientTokens);

		assert_ok!(LiquidCrowdloan::claim(Origin::signed(1), 100));


		// user claims half there stash twice
		let balance = AccountData {
			free: 100,
			reserved: 0,
			frozen: 0
		};
		let token_id = LiquidCrowdloan::token_id();

		assert_eq!(Tokens::accounts(1, token_id.unwrap()), balance);

		assert_eq!(NativeBalances::free_balance(LiquidCrowdloan::account_id()), 50);
		assert_eq!(NativeBalances::free_balance(1), 50);


		assert_ok!(LiquidCrowdloan::claim(Origin::signed(1), 100));

		let balance = AccountData {
			free: 0,
			reserved: 0,
			frozen: 0
		};
		let token_id = LiquidCrowdloan::token_id();

		assert_eq!(Tokens::accounts(1, token_id.unwrap()), balance);

		assert_eq!(NativeBalances::free_balance(LiquidCrowdloan::account_id()), 0);
		assert_eq!(NativeBalances::free_balance(1), 100);

		assert_noop!(LiquidCrowdloan::claim(Origin::signed(1), 100), Error::<Test>::ConversionError);

	});
}
