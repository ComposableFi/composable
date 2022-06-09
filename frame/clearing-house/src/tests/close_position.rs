use crate::{
	mock::{
		accounts::{AccountId, ALICE},
		runtime::{
			Balance, MarketId, Oracle as OraclePallet, Origin, Runtime, System as SystemPallet,
			TestPallet, Vamm as VammPallet,
		},
	},
	pallet::{
		Direction::{Long, Short},
		Error, Event,
	},
	tests::{any_direction, as_balance, run_for_seconds, with_trading_context, MarketConfig},
};
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;

// -------------------------------------------------------------------------------------------------
//                                           Helpers
// -------------------------------------------------------------------------------------------------

fn get_collateral(account_id: AccountId) -> Balance {
	TestPallet::get_collateral(&account_id).unwrap()
}

fn get_market_fee_pool(market_id: &MarketId) -> Balance {
	TestPallet::get_market(market_id).unwrap().fee_pool
}

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
	let config = MarketConfig { taker_fee: 0, ..Default::default() };
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
		assert_eq!(get_collateral(ALICE), collateral_0 * 2);

		SystemPallet::assert_last_event(
			Event::PositionClosed { user: ALICE, market: market_id, direction: Long, base }.into(),
		);
	});
}

#[test]
fn should_realize_long_position_losses() {
	let config = MarketConfig { taker_fee: 0, ..Default::default() };
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
	let config = MarketConfig {
		funding_frequency: 60,
		funding_period: 60,
		taker_fee: 0,
		..Default::default()
	};
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
		OraclePallet::set_twap(Some(1_800)); // 18 in cents
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
	let config = MarketConfig { taker_fee: 0, ..Default::default() };
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
		assert_eq!(get_collateral(ALICE), collateral_0 + as_balance(50));

		SystemPallet::assert_last_event(
			Event::PositionClosed { user: ALICE, market: market_id, direction: Short, base }.into(),
		);
	});
}

#[test]
fn should_realize_short_position_losses() {
	let config = MarketConfig { taker_fee: 0, ..Default::default() };
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
	let config = MarketConfig {
		funding_frequency: 60,
		funding_period: 60,
		taker_fee: 0,
		..Default::default()
	};
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
		OraclePallet::set_twap(Some(550)); // 5.5 in cents
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

	// TODO(0xangelo): should_remove_position_from_storage
}
