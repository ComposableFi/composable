use crate::{
	mock::{
		accounts::ALICE,
		runtime::{MarketId, Origin, Runtime, TestPallet, Vamm as VammPallet},
	},
	tests::{
		as_balance, get_market, run_to_time, with_market_context, with_trading_context, ExtBuilder,
		MarketConfig,
	},
	Direction::Long,
	Error,
};
use composable_traits::time::DurationSeconds;
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;

// ----------------------------------------------------------------------------------------------------
//                                             Prop Compose
// ----------------------------------------------------------------------------------------------------

const MAX_DURATION_SECONDS: DurationSeconds = 1_000_000_000;

prop_compose! {
	fn valid_close_times()(close_market in 2..MAX_DURATION_SECONDS)(
		close_market in Just(close_market),
		close_position in 1..close_market,
	) -> (DurationSeconds, DurationSeconds) {
		(close_position, close_market)
	}
}

prop_compose! {
	fn invalid_close_times()(close_market in 1..(MAX_DURATION_SECONDS / 2))(
		close_market in Just(close_market),
		close_position in (close_market + 1)..=MAX_DURATION_SECONDS,
	) -> (DurationSeconds, DurationSeconds) {
		(close_position, close_market)
	}
}

// --------------------------------------------------------------------------------------------------
//                                         Property Tests
// --------------------------------------------------------------------------------------------------

proptest! {
	#[test]
	fn updates_market_state(time in any::<DurationSeconds>()) {
		let config = MarketConfig::default();

		with_market_context(ExtBuilder::default(), config, |market_id| {
			let market_before = get_market(&market_id);

			assert_ok!(TestPallet::close_market(Origin::signed(ALICE), market_id, time));

			let market_after = get_market(&market_id);

			assert_ne!(market_before, market_after);
		});
	}

	#[test]
	fn should_fail_if_market_does_not_exist(
		market_id in any::<MarketId>(),
		time in any::<DurationSeconds>()
	) {
		ExtBuilder::default().build().execute_with(|| {
			assert_noop!(
			TestPallet::close_market(Origin::signed(ALICE), market_id, time),
				Error::<Runtime>::MarketIdNotFound
			);
		});
	}

	#[test]
	fn should_allow_closing_positions_before_market_close(
		(position_time, market_time) in valid_close_times(),
	) {
		let config = MarketConfig::default();
		let collateral = as_balance(100);

		with_trading_context(config, collateral, |market_id| {
			// Alice opens a position while market is open
			VammPallet::set_price(Some(10.into()));

			assert_ok!(TestPallet::open_position(
				Origin::signed(ALICE),
				market_id,
				Long,
				as_balance(100),
				as_balance(10)
			));

			// In the same block, market close is programmed to a future time
			assert_ok!(TestPallet::close_market(Origin::signed(ALICE), market_id, market_time));

			// Time passes, but it's still earlier than the market close
			run_to_time(position_time);

			// Alice closes her position
			assert_ok!(TestPallet::close_position(Origin::signed(ALICE), market_id));
		});
	}

	#[test]
	fn should_not_allow_closing_positions_after_market_close(
		(position_time, market_time) in invalid_close_times(),
	) {
		let config = MarketConfig::default();
		let collateral = as_balance(100);

		with_trading_context(config, collateral, |market_id| {
			// Alice opens a position while market is open
			VammPallet::set_price(Some(10.into()));

			assert_ok!(TestPallet::open_position(
				Origin::signed(ALICE),
				market_id,
				Long,
				as_balance(100),
				as_balance(10)
			));

			// In the same block, market close is programmed to a future time
			assert_ok!(TestPallet::close_market(Origin::signed(ALICE), market_id, market_time));

			// Time passes and now it's after the market close
			run_to_time(position_time);

			// Alice closes her position, but it should fail
			assert_noop!(
				TestPallet::close_position(Origin::signed(ALICE), market_id),
				Error::<Runtime>::MarketClosed,
			);
		});
	}
}
