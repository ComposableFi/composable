use crate::{
	mock::{
		accounts::{ALICE, BOB},
		runtime::{
			Oracle as OraclePallet, Origin, Runtime, System as SystemPallet, TestPallet,
			Vamm as VammPallet,
		},
	},
	pallet::{
		Direction::{Long, Short},
		Error, Event,
	},
	tests::{
		any_direction, as_balance, get_collateral, get_market, get_market_fee_pool,
		get_outstanding_profits, get_position, run_for_seconds, run_to_time,
		set_maximum_oracle_mark_divergence, set_oracle_price, set_oracle_twap,
		traders_in_one_market_context, with_trading_context, Market, MarketConfig,
	},
};

use frame_support::{assert_err, assert_noop, assert_ok};
use proptest::prelude::*;

// -------------------------------------------------------------------------------------------------
//                                          Unit Tests
// -------------------------------------------------------------------------------------------------

#[test]
fn should_fail_if_market_does_not_exist() {
	// Create dummy market
	let config = MarketConfig::default();

	with_trading_context(config, as_balance(100), |market_id| {
		assert_noop!(
			TestPallet::close_position(Origin::signed(ALICE), market_id + 1),
			Error::<Runtime>::MarketIdNotFound
		);
	});
}

#[test]
fn should_fail_if_there_is_no_position_in_market() {
	let config = MarketConfig::default();

	with_trading_context(config, as_balance(100), |market_id| {
		assert_noop!(
			TestPallet::close_position(Origin::signed(ALICE), market_id),
			Error::<Runtime>::PositionNotFound
		);
	});
}

#[test]
fn should_realize_long_position_gains() {
	let config = MarketConfig::default();
	let collateral_0 = as_balance(100);

	with_trading_context(config, collateral_0, |market_id| {
		VammPallet::set_price(Some(10.into()));

		let base = as_balance(10);
		assert_ok!(TestPallet::open_position(
			Origin::signed(ALICE),
			market_id,
			Long,
			as_balance(100),
			base,
		));

		VammPallet::set_price(Some(20.into()));
		assert_ok!(TestPallet::close_position(Origin::signed(ALICE), market_id));
		assert_eq!(get_outstanding_profits(ALICE), collateral_0);

		SystemPallet::assert_last_event(
			Event::PositionClosed { user: ALICE, market: market_id, direction: Long, base }.into(),
		);
	});
}

#[test]
fn should_realize_long_position_losses() {
	let config = MarketConfig::default();
	let collateral_0 = as_balance(100);

	with_trading_context(config, collateral_0, |market_id| {
		VammPallet::set_price(Some(20.into()));

		let base = as_balance(5);
		assert_ok!(TestPallet::open_position(
			Origin::signed(ALICE),
			market_id,
			Long,
			as_balance(100),
			base,
		));

		VammPallet::set_price(Some(10.into()));
		assert_ok!(TestPallet::close_position(Origin::signed(ALICE), market_id));
		assert_eq!(get_collateral(ALICE), collateral_0 / 2);

		SystemPallet::assert_last_event(
			Event::PositionClosed { user: ALICE, market: market_id, direction: Long, base }.into(),
		);
	});
}

#[test]
fn should_realize_long_position_funding() {
	let config = MarketConfig { funding_frequency: 60, funding_period: 60, ..Default::default() };
	let collateral_0 = as_balance(100);

	with_trading_context(config.clone(), collateral_0, |market_id| {
		// Alice opens a position
		VammPallet::set_price(Some(20.into()));
		let base = as_balance(5);
		assert_ok!(TestPallet::open_position(
			Origin::signed(ALICE),
			market_id,
			Long,
			as_balance(100),
			base,
		));

		// Time passes and the index moves against Alice's position by 10%
		run_for_seconds(config.funding_frequency);
		set_oracle_twap(&market_id, 18.into());
		VammPallet::set_twap(Some(20.into()));
		assert_ok!(TestPallet::update_funding(Origin::signed(ALICE), market_id));

		// Alice closes her position and pays 10% of the collateral in funding (price stays the
		// same)
		assert_ok!(TestPallet::close_position(Origin::signed(ALICE), market_id));
		assert_eq!(get_collateral(ALICE), collateral_0 - collateral_0 / 10);

		SystemPallet::assert_last_event(
			Event::PositionClosed { user: ALICE, market: market_id, direction: Long, base }.into(),
		);
	});
}

#[test]
fn should_realize_short_position_gains() {
	let config = MarketConfig::default();
	let collateral_0 = as_balance(100);

	with_trading_context(config, collateral_0, |market_id| {
		VammPallet::set_price(Some(10.into()));

		let base = as_balance(10);
		assert_ok!(TestPallet::open_position(
			Origin::signed(ALICE),
			market_id,
			Short,
			as_balance(100),
			base,
		));

		VammPallet::set_price(Some(5.into()));
		assert_ok!(TestPallet::close_position(Origin::signed(ALICE), market_id));
		assert_eq!(get_outstanding_profits(ALICE), as_balance(50));

		SystemPallet::assert_last_event(
			Event::PositionClosed { user: ALICE, market: market_id, direction: Short, base }.into(),
		);
	});
}

#[test]
fn should_realize_short_position_losses() {
	let config = MarketConfig::default();
	let collateral_0 = as_balance(100);

	with_trading_context(config, collateral_0, |market_id| {
		VammPallet::set_price(Some(5.into()));

		let base = as_balance(20);
		assert_ok!(TestPallet::open_position(
			Origin::signed(ALICE),
			market_id,
			Short,
			as_balance(100),
			base,
		));

		VammPallet::set_price(Some(10.into()));
		assert_ok!(TestPallet::close_position(Origin::signed(ALICE), market_id));
		assert_eq!(get_collateral(ALICE), 0);

		SystemPallet::assert_last_event(
			Event::PositionClosed { user: ALICE, market: market_id, direction: Short, base }.into(),
		);
	});
}

#[test]
fn should_realize_short_position_funding() {
	let config = MarketConfig { funding_frequency: 60, funding_period: 60, ..Default::default() };
	let collateral_0 = as_balance(100);

	with_trading_context(config.clone(), collateral_0, |market_id| {
		// Alice opens a position
		VammPallet::set_price(Some(5.into()));
		let base = as_balance(20);
		assert_ok!(TestPallet::open_position(
			Origin::signed(ALICE),
			market_id,
			Short,
			as_balance(100),
			base,
		));

		// Time passes and the index moves against Alice's position by 10%
		run_for_seconds(config.funding_frequency);
		set_oracle_twap(&market_id, (55, 10).into()); // 5.5
		VammPallet::set_twap(Some(5.into()));
		assert_ok!(TestPallet::update_funding(Origin::signed(ALICE), market_id));

		// Alice closes her position and pays 10% of the collateral in funding (price stays the
		// same)
		assert_ok!(TestPallet::close_position(Origin::signed(ALICE), market_id));
		assert_eq!(get_collateral(ALICE), collateral_0 - collateral_0 / 10);

		SystemPallet::assert_last_event(
			Event::PositionClosed { user: ALICE, market: market_id, direction: Short, base }.into(),
		);
	});
}

#[test]
fn should_fail_if_pushes_index_mark_divergence_above_threshold() {
	let config = MarketConfig::default();

	with_trading_context(config, as_balance(1_000_000), |market_id| {
		// Set maximum divergence to 10%
		set_maximum_oracle_mark_divergence((10, 100).into());

		let vamm_id = &get_market(&market_id).vamm_id;
		OraclePallet::set_price(Some(100 /* 1 in cents */));
		// HACK: set the last oracle price and TWAP equal to the current one to avoid an invalid
		// oracle status
		set_oracle_twap(&market_id, 1.into());
		VammPallet::set_price_of(vamm_id, Some(1.into()));

		// Alice opens a position (no price impact)
		assert_ok!(TestPallet::open_position(
			Origin::signed(ALICE),
			market_id,
			Long,
			as_balance(1_000_000),
			as_balance(1_000_000),
		));

		// Alice tries to close her position, but it fails because it pushes the mark price too
		// below the index. Closing tanks the mark price to 89% of the previous one.
		// Relative index-mark spread:
		// (mark - index) / index = (0.89 - 1.00) / 1.00 = -0.11
		VammPallet::set_price_impact_of(vamm_id, Some((89, 100).into()));
		assert_err!(
			TestPallet::close_position(Origin::signed(ALICE), market_id),
			Error::<Runtime>::OracleMarkTooDivergent
		);
	});
}

#[test]
fn should_not_fail_if_index_mark_divergence_was_already_above_threshold() {
	let config = MarketConfig::default();

	with_trading_context(config, as_balance(1_000_000), |market_id| {
		// Set maximum divergence to 10%
		set_maximum_oracle_mark_divergence((10, 100).into());

		let vamm_id = &get_market(&market_id).vamm_id;
		OraclePallet::set_price(Some(100)); // 1 in cents
		VammPallet::set_price_of(vamm_id, Some(1.into()));

		// Alice opens a position (no price impact)
		assert_ok!(TestPallet::open_position(
			Origin::signed(ALICE),
			market_id,
			Long,
			as_balance(1_000_000),
			as_balance(1_000_000),
		));

		// Due to external market conditions, index-mark spread rises to >10%
		// Relative index-mark spread:
		// (mark - index) / index = (1.00 - 1.12) / 1.12 = -0.1071428571
		OraclePallet::set_price(Some(112));

		// Alice closes her position causing mark price to drop by 1%
		VammPallet::set_price_impact_of(vamm_id, Some((99, 100).into()));
		assert_ok!(TestPallet::close_position(Origin::signed(ALICE), market_id));
	});
}

#[test]
fn should_only_update_collateral_with_available_gains() {
	let config = MarketConfig::default();
	let collateral = as_balance(100);
	let margins = vec![(ALICE, collateral), (BOB, collateral / 2)];

	traders_in_one_market_context(config, margins, |market_id| {
		// Make sure no funding is incurred
		set_oracle_twap(&market_id, 10.into());
		VammPallet::set_twap(Some(10.into()));

		VammPallet::set_price(Some(10.into()));

		let base = as_balance(10);
		assert_ok!(TestPallet::open_position(
			Origin::signed(ALICE),
			market_id,
			Long,
			collateral,
			base,
		));

		let base = as_balance(10);
		assert_ok!(TestPallet::open_position(
			Origin::signed(BOB),
			market_id,
			Short,
			collateral / 2,
			base,
		));

		// Price moves so that Alice is up 100% and Bob is down 100%
		VammPallet::set_price(Some(20.into()));
		assert_ok!(TestPallet::close_position(Origin::signed(BOB), market_id));
		assert_ok!(TestPallet::close_position(Origin::signed(ALICE), market_id));

		// Since Bob's size was lower than Alice's, even his whole collateral cannot cover Alice's
		// gains
		assert_eq!(get_collateral(BOB), 0);
		assert_eq!(get_collateral(ALICE), collateral + collateral / 2);
		assert_eq!(get_outstanding_profits(ALICE), collateral / 2);
	});
}

// -------------------------------------------------------------------------------------------------
//                                        Property Tests
// -------------------------------------------------------------------------------------------------

proptest! {
	#[test]
	fn should_succeed_if_position_exists(direction in any_direction()) {
		let config = MarketConfig::default();

		with_trading_context(config, as_balance(100), |market_id| {
			VammPallet::set_price(Some(10.into()));

			assert_ok!(TestPallet::open_position(
				Origin::signed(ALICE),
				market_id,
				direction,
				as_balance(100),
				as_balance(10),
			));

			assert_ok!(TestPallet::close_position(Origin::signed(ALICE), market_id));
		});
	}

	#[test]
	fn should_update_oracle_twap(direction in any_direction()) {
		let config = MarketConfig { twap_period: 60, ..Default::default() };
		let collateral = as_balance(100);

		with_trading_context(config.clone(), collateral, |market_id| {
			set_oracle_twap(&market_id, 5.into());

			// Alice opens a position
			VammPallet::set_price(Some(5.into()));
			assert_ok!(TestPallet::open_position(
				Origin::signed(ALICE),
				market_id,
				direction,
				collateral,
				as_balance(20),
			));

			let Market { last_oracle_price, last_oracle_twap, .. } = get_market(&market_id);

			// Time passes, the index price moves, and ALICE closes her position
			let now = config.twap_period / 2;
			run_to_time(now);
			set_oracle_price(&market_id, 6.into());
			assert_ok!(TestPallet::close_position(Origin::signed(ALICE), market_id));

			let market = get_market(&market_id);
			// The last oracle TWAP update timestamp equals the one of the position closing
			assert_eq!(market.last_oracle_ts, now);
			assert_ne!(market.last_oracle_price, last_oracle_price);
			assert_ne!(market.last_oracle_twap, last_oracle_twap);
		});
	}

		#[test]
	fn should_charge_fees_upon_closing(direction in any_direction(), taker_fee in 1..=1_000_u128) {
		let config = MarketConfig {
			taker_fee,
			..Default::default()
		};
		let size = as_balance(100);
		let collateral_0 = size * 2; // have excess for fees

		with_trading_context(config, collateral_0, |market_id| {
			VammPallet::set_price(Some(10.into()));

			assert_ok!(TestPallet::open_position(
				Origin::signed(ALICE),
				market_id,
				direction,
				as_balance(100),
				as_balance(10),
			));
			let collateral_1 = get_collateral(ALICE);
			assert!(collateral_0 > collateral_1); // collateral should be reduced by fees

			assert_ok!(TestPallet::close_position(Origin::signed(ALICE), market_id));
			let collateral_2 = get_collateral(ALICE);
			assert!(collateral_1 > collateral_2); // collateral should be reduced by fees

			// Market Fee Pool is increased by the difference between initial and final collateral values
			assert_eq!(get_market_fee_pool(&market_id), collateral_0 - collateral_2);
		});
	}

	#[test]
	fn should_remove_position_from_storage(direction in any_direction()) {
		let config = MarketConfig::default();

		with_trading_context(config, as_balance(100), |market_id| {
			VammPallet::set_price(Some(10.into()));

			assert_ok!(TestPallet::open_position(
				Origin::signed(ALICE),
				market_id,
				direction,
				as_balance(100),
				as_balance(10),
			));

			assert_ok!(TestPallet::close_position(Origin::signed(ALICE), market_id));
			assert!(matches!(get_position(&ALICE, &market_id), None));
		});
	}

	#[test]
	fn should_update_market_funding_if_possible(direction in any_direction()) {
		let config = MarketConfig {
			funding_frequency: 60,
			funding_period: 60,
			..Default::default()
		};
		let size = as_balance(100);

		with_trading_context(config.clone(), size, |market_id| {
			// Ensure last funding update is at time 0
			assert_eq!(get_market(&market_id).funding_rate_ts, 0);

			VammPallet::set_price(Some(10.into()));

			assert_ok!(TestPallet::open_position(
				Origin::signed(ALICE),
				market_id,
				direction,
				size,
				size / 10,
			));

			// Enough time passes for a funding update to be possible
			run_for_seconds(config.funding_frequency);
			assert_ok!(TestPallet::close_position(Origin::signed(ALICE), market_id));
			// Last funding update should be at time 60
			assert_eq!(get_market(&market_id).funding_rate_ts, config.funding_frequency);
		});
	}

	#[test]
	fn should_not_update_market_funding_if_too_early(
		direction in any_direction(),
		offset in 1..=60_u64
	) {
		let config = MarketConfig {
			funding_frequency: 60,
			funding_period: 60,
			..Default::default()
		};
		let size = as_balance(100);

		with_trading_context(config, size, |market_id| {
			// Ensure last funding update is at time 0
			assert_eq!(get_market(&market_id).funding_rate_ts, 0);

			VammPallet::set_price(Some(10.into()));

			assert_ok!(TestPallet::open_position(
				Origin::signed(ALICE),
				market_id,
				direction,
				size,
				size / 10,
			));

			// Not enough time passes for a funding update to be possible
			run_for_seconds(60 - offset);
			assert_ok!(TestPallet::close_position(Origin::signed(ALICE), market_id));
			// Last funding update should be at time 0
			assert_eq!(get_market(&market_id).funding_rate_ts, 0);
		});
	}
}
