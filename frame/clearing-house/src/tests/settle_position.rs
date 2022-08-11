use crate::{
	mock::{
		accounts::ALICE,
		runtime::{ExtBuilder, Origin, Runtime, TestPallet, Vamm as VammPallet},
	},
	tests::{
		as_balance, get_market, get_position, run_to_time, with_market_context,
		with_trading_context,
	},
	Direction::*,
	Error,
};
use frame_support::{assert_noop, assert_ok};

// ----------------------------------------------------------------------------------------------------
//                                             Unit Tests
// ----------------------------------------------------------------------------------------------------

#[test]
fn should_fail_if_market_does_not_exist() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			TestPallet::settle_position(Origin::signed(ALICE), 0),
			Error::<Runtime>::MarketIdNotFound
		);
	});
}

#[test]
fn should_fail_if_user_has_no_position_in_market() {
	with_market_context(Default::default(), Default::default(), |market_id| {
		// Market is set to close 10 seconds from now.
		assert_ok!(TestPallet::close_market(Origin::root(), market_id, 10));

		run_to_time(10);
		assert_noop!(
			TestPallet::settle_position(Origin::signed(ALICE), market_id),
			Error::<Runtime>::PositionNotFound
		);
	});
}

#[test]
fn should_fail_if_market_is_not_closed() {
	with_trading_context(Default::default(), as_balance(100), |market_id| {
		assert_ok!(TestPallet::open_position(
			Origin::signed(ALICE),
			market_id,
			Long,
			as_balance(100),
			0
		));

		VammPallet::set_settlement_price_of(&get_market(&market_id).vamm_id, Some(0.into()));

		assert_noop!(
			TestPallet::settle_position(Origin::signed(ALICE), market_id),
			Error::<Runtime>::MarketNotClosed
		);

		assert_ok!(TestPallet::close_market(Origin::root(), market_id, 10));
		run_to_time(5);
		assert_noop!(
			TestPallet::settle_position(Origin::signed(ALICE), market_id),
			Error::<Runtime>::MarketNotClosed
		);

		run_to_time(10);
		assert_ok!(TestPallet::settle_position(Origin::signed(ALICE), market_id));
	})
}

#[test]
fn should_close_user_position_in_market() {
	with_trading_context(Default::default(), as_balance(100), |market_id| {
		assert_ok!(TestPallet::open_position(
			Origin::signed(ALICE),
			market_id,
			Long,
			as_balance(100),
			0
		));

		assert_ok!(TestPallet::close_market(Origin::root(), market_id, 10));
		let market = get_market(&market_id);
		VammPallet::set_settlement_price_of(&market.vamm_id, Some(0.into()));
		run_to_time(10);

		assert_ok!(TestPallet::settle_position(Origin::signed(ALICE), market_id));
		assert!(get_position(&ALICE, &market_id).is_none());
	});
}
