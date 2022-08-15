use crate::{
	mock::{
		accounts::{ALICE, BOB},
		runtime::{ExtBuilder, Origin, Runtime, TestPallet, Vamm as VammPallet},
	},
	tests::{
		as_balance, get_collateral, get_market, get_position, run_to_time,
		traders_in_one_market_context, with_market_context, with_trading_context,
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

#[test]
fn should_handle_both_longs() {
	traders_in_one_market_context(
		Default::default(),
		vec![(ALICE, as_balance(100)), (BOB, as_balance(100))],
		|market_id| {
			let market = get_market(&market_id);
			VammPallet::set_price_of(&market.vamm_id, Some((101, 100).into()));
			assert_ok!(TestPallet::open_position(
				Origin::signed(ALICE),
				market_id,
				Long,
				as_balance(100),
				0
			));
			// Simulate the price move caused by Alice buying
			VammPallet::set_price_of(&market.vamm_id, Some((103, 100).into()));
			assert_ok!(TestPallet::open_position(
				Origin::signed(BOB),
				market_id,
				Long,
				as_balance(100),
				0
			));

			// Market closes
			assert_ok!(TestPallet::close_market(Origin::root(), market_id, 10));
			run_to_time(10);

			// The settlement price is 102, so Alice should be in profit and Bob should be in loss.
			VammPallet::set_settlement_price_of(&market.vamm_id, Some((102, 100).into()));
			assert_ok!(TestPallet::settle_position(Origin::signed(ALICE), market_id));
			assert_ok!(TestPallet::settle_position(Origin::signed(BOB), market_id));
			let (alice_col, bob_col) = (get_collateral(ALICE), get_collateral(BOB));
			dbg!((alice_col, bob_col));
			assert!(alice_col > as_balance(100));
			assert!(bob_col < as_balance(100));

			// System is airtight
			assert_eq!(alice_col + bob_col, as_balance(200));
		},
	)
}

#[test]
fn should_handle_equivalent_long_and_short() {
	traders_in_one_market_context(
		Default::default(),
		vec![(ALICE, as_balance(100)), (BOB, as_balance(100))],
		|market_id| {
			let market = get_market(&market_id);
			// Simulate price impact caused by Alice buying
			VammPallet::set_price_of(&market.vamm_id, Some((101, 100).into()));
			assert_ok!(TestPallet::open_position(
				Origin::signed(ALICE),
				market_id,
				Long,
				as_balance(100),
				0
			));
			// Simulate price impact caused by Bob selling
			VammPallet::set_price_of(&market.vamm_id, Some((101, 100).into()));
			assert_ok!(TestPallet::open_position(
				Origin::signed(BOB),
				market_id,
				Short,
				as_balance(100),
				0
			));

			// Market closes
			assert_ok!(TestPallet::close_market(Origin::root(), market_id, 10));
			run_to_time(10);

			// The settlement price is 0 (the vAMM is back at equilibrium), so everyone should get
			// back their collateral
			VammPallet::set_settlement_price_of(&market.vamm_id, Some(0.into()));
			assert_ok!(TestPallet::settle_position(Origin::signed(ALICE), market_id));
			assert_ok!(TestPallet::settle_position(Origin::signed(BOB), market_id));
			let (alice_col, bob_col) = (get_collateral(ALICE), get_collateral(BOB));
			assert_eq!(alice_col, as_balance(100));
			assert_eq!(bob_col, as_balance(100));
		},
	)
}
