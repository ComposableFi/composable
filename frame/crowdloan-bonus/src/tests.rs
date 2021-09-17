use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, traits::Currency};
use orml_tokens::AccountData;
use sp_runtime::traits::BadOrigin;
use primitives::currency::{CurrencyId, TokenSymbol};

#[test]
fn initiate() {
	new_test_ext().execute_with(|| {
		assert_ok!(LiquidCrowdloan::initialize());
		let balance = AccountData { free: 200, reserved: 0, frozen: 0 };
		assert_eq!(Tokens::accounts(Sudo::key(), CurrencyId::Token(TokenSymbol::Crowdloan)), balance);
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
		let owner = Sudo::key();
		assert_ok!(LiquidCrowdloan::initialize());
		assert_noop!(LiquidCrowdloan::claim(Origin::signed(1), 100), Error::<Test>::NotClaimable);
		assert_ok!(LiquidCrowdloan::make_claimable(Origin::signed(2)));
		NativeBalances::make_free_balance_be(&LiquidCrowdloan::account_id(), 100);
		assert_eq!(NativeBalances::free_balance(LiquidCrowdloan::account_id()), 100);

		assert_noop!(
			LiquidCrowdloan::claim(Origin::signed(1), 300),
			Error::<Test>::InsufficientTokens
		);

		assert_ok!(LiquidCrowdloan::claim(Origin::signed(owner), 100));

		// owner claims half there stash twice
		let balance = AccountData { free: 100, reserved: 0, frozen: 0 };
		let token_id = CurrencyId::Token(TokenSymbol::Crowdloan);

		assert_eq!(Tokens::accounts(owner, token_id), balance);

		assert_eq!(NativeBalances::free_balance(LiquidCrowdloan::account_id()), 50);
		assert_eq!(NativeBalances::free_balance(owner), 50);

		assert_ok!(LiquidCrowdloan::claim(Origin::signed(owner), 100));

		let balance = AccountData { free: 0, reserved: 0, frozen: 0 };
		let token_id = CurrencyId::Token(TokenSymbol::Crowdloan);

		assert_eq!(Tokens::accounts(1, token_id), balance);

		assert_eq!(NativeBalances::free_balance(LiquidCrowdloan::account_id()), 0);
		assert_eq!(NativeBalances::free_balance(owner), 100);

		assert_noop!(
			LiquidCrowdloan::claim(Origin::signed(1), 100),
			Error::<Test>::EmptyPot
		);
	});
}
