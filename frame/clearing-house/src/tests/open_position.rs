use crate::{
	mock::{
		accounts::ALICE,
		assets::USDC,
		runtime::{
			Balance, ExtBuilder, MarketId, Oracle as OraclePallet, Origin, Runtime,
			System as SystemPallet, TestPallet, Vamm as VammPallet,
		},
	},
	pallet::{
		Config,
		Direction::{Long, Short},
		Error, Event,
	},
	tests::{
		any_direction, any_price, as_balance, get_collateral, get_market, get_market_fee_pool,
		get_outstanding_gains, get_position, run_for_seconds, run_to_time, set_fee_pool_depth,
		set_maximum_oracle_mark_divergence, set_oracle_price, set_oracle_twap,
		with_markets_context, with_trading_context, Market, MarketConfig,
	},
};
use composable_maths::labs::numbers::{IntoBalance, IntoSigned};
use composable_traits::{clearing_house::ClearingHouse, time::ONE_HOUR};
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;
use sp_runtime::{FixedI128, FixedPointNumber, FixedU128};

// -------------------------------------------------------------------------------------------------
//                                      Execution Contexts
// -------------------------------------------------------------------------------------------------

fn cross_margin_context<R>(
	configs: Vec<MarketConfig>,
	margin: Balance,
	execute: impl FnOnce(Vec<MarketId>) -> R,
) -> R {
	let ext_builder = ExtBuilder { balances: vec![(ALICE, USDC, margin)], ..Default::default() };

	with_markets_context(ext_builder, configs, |market_ids| {
		TestPallet::deposit_collateral(Origin::signed(ALICE), USDC, margin);

		execute(market_ids)
	})
}

// -------------------------------------------------------------------------------------------------
//                                         Prop Compose
// -------------------------------------------------------------------------------------------------

prop_compose! {
	fn decimal_a_lt_decimal_b(b: FixedU128)(
		a_inner in 0..b.into_inner(),
		b in Just(b),
	) -> (FixedI128, FixedI128) {
		(FixedU128::from_inner(a_inner).into_signed().unwrap(), b.into_signed().unwrap())
	}
}

prop_compose! {
	fn min_trade_size_and_eps(min_size: u128)(
		eps in -(min_size as i128)..=(min_size as i128)
	) -> (FixedI128, i128) {
		// Couldn't find a better way to ensure that min_size is positive, so this will trigger a
		// test error otherwise
		assert!(min_size > 0);
		(FixedI128::from_inner(min_size as i128), eps)
	}
}

prop_compose! {
	fn percentage_fraction()(percent in 1..100_u128) -> FixedU128 {
		FixedU128::from((percent, 100))
	}
}

// -------------------------------------------------------------------------------------------------
//                                          Unit Tests
// -------------------------------------------------------------------------------------------------

#[test]
fn fails_to_open_position_if_market_id_invalid() {
	let quote_amount = as_balance(100);
	let base_amount_limit = as_balance(10);

	with_trading_context(MarketConfig::default(), quote_amount, |market_id| {
		// Current price = quote_amount / base_amount_limit
		VammPallet::set_price(Some((quote_amount, base_amount_limit).into()));

		assert_noop!(
			TestPallet::open_position(
				Origin::signed(ALICE),
				market_id + 1,
				Long,
				quote_amount,
				base_amount_limit
			),
			Error::<Runtime>::MarketIdNotFound,
		);
	});
}

#[test]
fn fails_to_create_new_position_if_violates_maximum_positions_num() {
	let max_positions = <Runtime as Config>::MaxPositions::get() as usize;
	let orders = max_positions + 1;
	let configs = vec![MarketConfig::default(); orders];
	let margin = as_balance(100);

	cross_margin_context(configs, margin, |market_ids| {
		let quote_amount: Balance = margin / (orders as u128);
		let base_amount_limit: Balance = as_balance(10) / (orders as u128);

		// Current price = quote_amount / base_amount_limit
		VammPallet::set_price(Some((quote_amount, base_amount_limit).into()));

		for market_id in market_ids.iter().take(max_positions) {
			assert_ok!(TestPallet::open_position(
				Origin::signed(ALICE),
				*market_id,
				Long,
				quote_amount,
				base_amount_limit,
			));
		}

		assert_noop!(
			TestPallet::open_position(
				Origin::signed(ALICE),
				market_ids[max_positions],
				Long,
				quote_amount,
				base_amount_limit,
			),
			Error::<Runtime>::MaxPositionsExceeded
		);
	});
}

#[test]
fn should_block_risk_decreasing_trade_if_it_pushes_index_mark_divergence_above_threshold() {
	let config = MarketConfig::default();

	with_trading_context(config, as_balance(1_000_000), |market_id| {
		// Set maximum divergence to 10%
		set_maximum_oracle_mark_divergence((10, 100).into());

		let vamm_id = &get_market(&market_id).vamm_id;
		OraclePallet::set_price(Some(100 /* 1 in cents */));
		// HACK: set previous oracle price and TWAP equal to current one to avoid an invalid oracle
		// status
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
		assert_noop!(
			TestPallet::open_position(
				Origin::signed(ALICE),
				market_id,
				Short,
				as_balance(1_000_000),
				as_balance(0)
			),
			Error::<Runtime>::OracleMarkTooDivergent
		);
	});
}

#[test]
fn should_not_block_risk_decreasing_trade_if_index_mark_divergence_was_already_above_threshold() {
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
		assert_ok!(TestPallet::open_position(
			Origin::signed(ALICE),
			market_id,
			Short,
			as_balance(1_000_000),
			as_balance(0)
		));
	});
}

#[test]
fn should_block_risk_increasing_trade_if_it_pushes_index_mark_divergence_above_threshold() {
	let config = MarketConfig::default();

	with_trading_context(config, as_balance(1_000_000), |market_id| {
		// Set maximum divergence to 10%
		set_maximum_oracle_mark_divergence((10, 100).into());

		let vamm_id = &get_market(&market_id).vamm_id;
		OraclePallet::set_price(Some(100 /* 1 in cents */));
		// HACK: set previous oracle price and TWAP equal to current one to avoid an invalid oracle
		// status
		set_oracle_twap(&market_id, 1.into());
		VammPallet::set_price_of(vamm_id, Some(1.into()));

		// Alice tries to open a new long, but it fails because it pushes the mark price too
		// above the index. Opening pumps the mark price to 111% of previous one.
		// Relative index-mark spread:
		// (mark - index) / index = (1.11 - 1.00) / 1.00 = 0.11
		VammPallet::set_price_impact_of(vamm_id, Some((111, 100).into()));
		assert_noop!(
			TestPallet::open_position(
				Origin::signed(ALICE),
				market_id,
				Long,
				as_balance(1_000_000),
				as_balance(1_000_000),
			),
			Error::<Runtime>::OracleMarkTooDivergent
		);
	});
}

// -------------------------------------------------------------------------------------------------
//                                        Property Tests
// -------------------------------------------------------------------------------------------------

proptest! {
	#[test]
	fn open_position_in_new_market_succeeds(direction in any_direction()) {
		let config = MarketConfig { taker_fee: 10 /* 0.1% */, ..Default::default() };
		let quote_amount = as_balance(100);
		let fees = (quote_amount * config.taker_fee) / 10_000;

		// Have enough margin to pay for fees
		with_trading_context(config, quote_amount + fees, |market_id| {
			let positions_before = TestPallet::get_positions(&ALICE).len();

			let base_amount = as_balance(10);
			// Current price = quote_amount / base_amount
			VammPallet::set_price(Some((quote_amount, base_amount).into()));
			assert_ok!(TestPallet::open_position(
				Origin::signed(ALICE),
				market_id,
				direction,
				quote_amount,
				base_amount,
			));

			// Ensure a new position is created
			assert_eq!(TestPallet::get_positions(&ALICE).len(), positions_before + 1);
			let position = get_position(&ALICE, &market_id).unwrap();
			assert!(match direction {
				Long => position.base_asset_amount.is_positive(),
				Short => position.base_asset_amount.is_negative()
			});
			assert!(match direction {
				Long => position.quote_asset_notional_amount.is_positive(),
				Short => position.quote_asset_notional_amount.is_negative()
			});

			// Ensure cumulative funding is initialized to market's current
			let market = get_market(&market_id);
			assert_eq!(position.last_cum_funding, market.cum_funding_rate(direction));

			// Ensure fees are deducted from margin
			assert_eq!(TestPallet::get_collateral(&ALICE), Some(quote_amount));

			// Ensure market state is updated:
			// - net position
			// - fees collected
			assert_eq!(market.base_asset_amount(direction), position.base_asset_amount);
			assert_eq!(get_market_fee_pool(&market_id), fees);

			SystemPallet::assert_last_event(
				Event::TradeExecuted {
					market: market_id,
					direction,
					quote: quote_amount,
					base: base_amount,
				}.into()
			);
		});
	}

	#[test]
	fn fails_to_open_position_if_trade_size_too_small(
		(minimum_trade_size, eps) in min_trade_size_and_eps(as_balance((1, 100)))
	) {
		let market_config = MarketConfig { minimum_trade_size, ..Default::default() };
		let quote_amount = eps.unsigned_abs();

		with_trading_context(market_config, quote_amount, |market_id| {
			VammPallet::set_price(Some(1.into()));

			assert_noop!(
				TestPallet::open_position(
					Origin::signed(ALICE),
					market_id,
					match eps.is_positive() {
						true => Long,
						false => Short,
					},
					quote_amount,
					quote_amount, // price = 1
				),
				Error::<Runtime>::TradeSizeTooSmall
			);
		});
	}

	#[test]
	fn trade_can_close_position_within_tolerance(
		direction in any_direction(),
		(minimum_trade_size, eps) in min_trade_size_and_eps(as_balance((1, 100)))
	) {
		let config = MarketConfig { minimum_trade_size, ..Default::default() };
		let quote_amount = as_balance(100);

		with_trading_context(config, quote_amount, |market_id| {
			let positions_before = TestPallet::get_positions(&ALICE).len();

			let base_amount_limit = as_balance(10);
			// price * base_amount_limit = quote_amount
			VammPallet::set_price(Some((quote_amount, base_amount_limit).into()));
			assert_ok!(TestPallet::open_position(
				Origin::signed(ALICE),
				market_id,
				direction,
				quote_amount,
				base_amount_limit,
			));

			// price' * base_amount_limit = (quote_amount + eps)
			VammPallet::set_price(Some(
				((quote_amount as i128 + eps).unsigned_abs(), base_amount_limit).into()
			));
			assert_ok!(TestPallet::open_position(
				Origin::signed(ALICE),
				market_id,
				direction.opposite(),
				quote_amount,
				base_amount_limit,
			));

			assert_eq!(TestPallet::get_positions(&ALICE).len(), positions_before);

			assert_eq!(get_market(&market_id).base_asset_amount(direction), 0.into());
		});
	}

	#[test]
	fn should_update_oracle_twap(direction in any_direction()) {
		let config = MarketConfig { twap_period: 60, ..Default::default() };
		let collateral = as_balance(100);

		ExtBuilder {
			oracle_price: Some(500), // 5 in cents
			..Default::default()
		}.trading_context(config.clone(), collateral, |market_id| {
			// Ensure market initializes as expected
			let Market { last_oracle_ts, last_oracle_price, last_oracle_twap, .. } = get_market(&market_id);
			assert_eq!(last_oracle_ts, 0);
			assert_eq!(last_oracle_price, 5.into());
			assert_eq!(last_oracle_twap, 5.into());

			// Time passes
			let now = config.twap_period / 2;
			run_to_time(now);

			VammPallet::set_price(Some(5.into()));
			// Index price moved
			set_oracle_price(&market_id, 6.into());

			// Alice opens a position
			assert_ok!(TestPallet::open_position(
				Origin::signed(ALICE),
				market_id,
				direction,
				collateral,
				collateral / 5,
			));

			let market = get_market(&market_id);
			// The last oracle TWAP update timestamp equals the one of the position closing
			assert_eq!(market.last_oracle_ts, now);
			assert_ne!(market.last_oracle_price, last_oracle_price);
			assert_ne!(market.last_oracle_twap, last_oracle_twap);
		});
	}

	#[test]
	fn trading_realizes_position_funding_payments(
		direction in any_direction(),
		rate in -100..=100_i128, // -1% to 1% in basis points
	) {
		let config = MarketConfig {
			funding_frequency: ONE_HOUR,
			funding_period: ONE_HOUR,
			margin_ratio_initial: (1, 10).into(), // allow 10x leverage, more than enough
			taker_fee: 100, // 1%
			..Default::default()
		};
		let quote_amount = as_balance(10_000);
		let fee = (quote_amount * config.taker_fee) / 10_000;

		with_trading_context(config, quote_amount + fee * 2, |market_id| {
			VammPallet::set_price(Some(1.into()));
			let base_amount = quote_amount;
			// Open an initial position, pay fees
			assert_ok!(
				<TestPallet as ClearingHouse>::open_position(
					&ALICE,
					&market_id,
					direction,
					quote_amount,
					base_amount,
				),
				base_amount
			);

			// Update market funding rate
			run_for_seconds(ONE_HOUR);
			set_oracle_twap(&market_id, 1.into());
			// price in basis points
			VammPallet::set_twap(Some(((10_000 + rate) as u128, 10_000).into()));
			// Hack: set Fee Pool depth so as not to worry about capped funding rates
			set_fee_pool_depth(&market_id, quote_amount);
			assert_ok!(<TestPallet as ClearingHouse>::update_funding(&market_id));

			// Increase position, pay fees, and expect funding settlement
			assert_ok!(
				<TestPallet as ClearingHouse>::open_position(
					&ALICE,
					&market_id,
					direction,
					quote_amount,
					base_amount,
				),
				base_amount
			);
			let sign = match direction { Long => -1, _ => 1 };
			let payment = sign * (rate * quote_amount as i128) / 10_000;
			let margin = quote_amount as i128  + payment; // Initial margin minus fees + funding
			assert_eq!(TestPallet::get_collateral(&ALICE), Some(margin as u128));
		});
	}

	#[test]
	fn closing_position_with_trade_realizes_pnl(
		direction in any_direction(),
		new_price in any_price()
	) {
		let quote_amount = as_balance(100);

		with_trading_context(MarketConfig::default(), quote_amount, |market_id| {
			let positions_before = TestPallet::get_positions(&ALICE).len();

			VammPallet::set_price(Some(10.into()));
			let base_amount_limit = quote_amount / 10;
			assert_ok!(
				<TestPallet as ClearingHouse>::open_position(
					&ALICE,
					&market_id,
					direction,
					quote_amount,
					base_amount_limit,
				),
				base_amount_limit,
			);

			VammPallet::set_price(Some(new_price));
			let new_base_value = new_price.saturating_mul_int(base_amount_limit);
			assert_ok!(
				<TestPallet as ClearingHouse>::open_position(
					&ALICE,
					&market_id,
					direction.opposite(),
					new_base_value,
					base_amount_limit,
				),
				base_amount_limit
			);

			assert_eq!(TestPallet::get_positions(&ALICE).len(), positions_before);
			let sign = match direction { Long => 1, _ => -1 };
			let margin = quote_amount as i128;
			let pnl = sign * (new_base_value as i128) - sign * margin;
			// Profits are outstanding since no one realized losses in the market
			if pnl >= 0 {
				assert_eq!(get_collateral(ALICE), margin as u128);
				assert_eq!(get_outstanding_gains(ALICE, &market_id), pnl as u128);
			} else {
				assert_eq!(get_collateral(ALICE), (margin + pnl).max(0) as u128);
			}
		});
	}

	#[test]
	fn reducing_position_partially_realizes_pnl(
		direction in any_direction(),
		new_price in any_price(),
		fraction in percentage_fraction()
	) {
		let market_config = MarketConfig { minimum_trade_size: 0.into(), ..Default::default() };
		let quote_amount = as_balance(100);

		with_trading_context(market_config, quote_amount, |market_id| {
			VammPallet::set_price(Some(10.into()));
			let base_amount = quote_amount / 10;
			assert_ok!(
				<TestPallet as ClearingHouse>::open_position(
					&ALICE,
					&market_id,
					direction,
					quote_amount,
					base_amount,
				),
				base_amount
			);

			let positions_before = TestPallet::get_positions(&ALICE).len();

			VammPallet::set_price(Some(new_price));
			// Reduce (close) position by desired percentage
			let base_amount_to_close = fraction.saturating_mul_int(base_amount);
			let base_value_to_close = new_price.saturating_mul_int(base_amount_to_close);
			assert_ok!(
				<TestPallet as ClearingHouse>::open_position(
					&ALICE,
					&market_id,
					direction.opposite(),
					base_value_to_close,
					base_amount_to_close,
				),
				base_amount_to_close,
			);

			// Position remains open
			assert_eq!(TestPallet::get_positions(&ALICE).len(), positions_before);
			// Fraction of the PnL is realized
			let sign = match direction { Long => 1, _ => -1 };
			let entry_value = fraction.saturating_mul_int(quote_amount);
			let pnl = sign * (base_value_to_close as i128) - sign * (entry_value as i128);
			if pnl >= 0 {
				assert_eq!(get_collateral(ALICE), quote_amount);
				assert_eq!(get_outstanding_gains(ALICE, &market_id), pnl as u128);
			} else {
				assert_eq!(get_collateral(ALICE), (quote_amount as i128 + pnl).max(0) as u128);
			}

			let position = get_position(&ALICE, &market_id).unwrap();
			// Position base asset and quote asset notional are cut by percentage
			assert_eq!(
				position.base_asset_amount.into_inner(),
				sign * ((base_amount - base_amount_to_close) as i128)
			);
			assert_eq!(
				position.quote_asset_notional_amount.into_inner(),
				sign * ((quote_amount - entry_value) as i128)
			);

			assert_eq!(get_market(&market_id).base_asset_amount(direction), position.base_asset_amount);
		});
	}

	#[test]
	fn reversing_position_realizes_pnl(
		direction in any_direction(),
		new_price in any_price()
	) {
		let market_config = MarketConfig { minimum_trade_size: 0.into(), ..Default::default() };
		let quote_amount = as_balance(100);

		with_trading_context(market_config, quote_amount, |market_id| {
			VammPallet::set_price(Some(10.into()));
			let base_amount = quote_amount / 10;
			assert_ok!(
				<TestPallet as ClearingHouse>::open_position(
					&ALICE,
					&market_id,
					direction,
					quote_amount,
					base_amount,
				),
				base_amount
			);

			let positions_before = TestPallet::get_positions(&ALICE).len();

			VammPallet::set_price(Some(new_price));
			let new_base_value = new_price.saturating_mul_int(base_amount);
			// We want to end up with the reverse of the position (in base tokens)
			// Now:
			// base = new_base_value
			// Goal:
			// -base = -new_base_value
			// Delta:
			// base * 2 = new_base_value * 2
			let base_delta = base_amount * 2;
			let quote_delta = new_base_value * 2;
			assert_ok!(
				<TestPallet as ClearingHouse>::open_position(
					&ALICE,
					&market_id,
					direction.opposite(),
					quote_delta,
					base_delta,
				),
				base_delta
			);

			let sign = match direction { Long => 1, _ => -1 };
			// Full PnL is realized
			let margin = quote_amount as i128;
			let pnl = sign * (new_base_value as i128) - sign * margin;
			if pnl >= 0 {
				assert_eq!(get_collateral(ALICE), margin as u128);
				assert_eq!(get_outstanding_gains(ALICE, &market_id), pnl as u128);
			} else {
				assert_eq!(get_collateral(ALICE), (margin + pnl).max(0) as u128);
			}

			// Position remains open
			assert_eq!(TestPallet::get_positions(&ALICE).len(), positions_before);

			let position = get_position(&ALICE, &market_id).unwrap();
			assert_eq!(
				position.base_asset_amount,
				FixedI128::from_inner(- sign * base_amount as i128)
			);
			assert_eq!(
				position.quote_asset_notional_amount,
				FixedI128::from_inner(- sign * new_base_value as i128)
			);

			let market = get_market(&market_id);
			assert_eq!(market.base_asset_amount(direction), 0.into());
			assert_eq!(market.base_asset_amount(direction.opposite()), position.base_asset_amount);
		});
	}

	#[test]
	fn fails_to_create_new_position_without_enough_margin(
		direction in any_direction(),
		excess in 1..as_balance(1_000_000),
	) {
		let market_config = MarketConfig {
			margin_ratio_initial: (1, 10).into(),
			..Default::default()
		};
		let margin = as_balance(10);

		with_trading_context(market_config, margin, |market_id| {
			VammPallet::set_price(Some(10.into()));
			let quote_amount = margin * 10 + excess; // Over 10x margin
			let base_amount_limit = quote_amount / 10;
			assert_noop!(
				TestPallet::open_position(
					Origin::signed(ALICE),
					market_id,
					direction,
					quote_amount,
					base_amount_limit,
				),
				Error::<Runtime>::InsufficientCollateral,
			);
		});
	}

	#[test]
	fn succeeds_in_creating_new_position_with_enough_margin(
		direction in any_direction(),
		max_leverage_percent in 100..2_000_u128,  // Anywhere from 1x to 20x margin
		percentf in percentage_fraction()
	) {
		let market_config = MarketConfig {
			margin_ratio_initial: (100, max_leverage_percent).into(),
			..Default::default()
		};
		let margin = as_balance(10);
		let quote_amount_max = market_config
			.margin_ratio_initial
			.reciprocal()
			.unwrap()
			.saturating_mul_int(margin);
		let quote_amount = percentf.saturating_mul_int(quote_amount_max);

		with_trading_context(market_config, margin, |market_id| {
			VammPallet::set_price(Some(10.into()));
			let base_amount_limit = quote_amount / 10;
			assert_ok!(
				<TestPallet as ClearingHouse>::open_position(
					&ALICE,
					&market_id,
					direction,
					quote_amount,
					base_amount_limit
				),
				base_amount_limit,
			);
		});
	}

	#[test]
	fn can_decrease_position_even_if_below_imr(direction in any_direction()) {
		let market_config = MarketConfig {
			margin_ratio_initial: (1, 10).into(),  // 1/10 IMR, or 10x leverage
			..Default::default()
		};
		let margin = as_balance(10);

		with_trading_context(market_config, margin, |market_id| {
			VammPallet::set_price(Some(10.into()));
			let quote_amount = as_balance(100); // 10x margin => max leverage
			let base_amount_limit = quote_amount / 10;
			assert_ok!(
				<TestPallet as ClearingHouse>::open_position(
					&ALICE,
					&market_id,
					direction,
					quote_amount,
					base_amount_limit
				),
				base_amount_limit,
			);

			let new_price: FixedU128 = match direction {
				Long => 8, // decrease price => negative PnL
				Short => 12, // increase price => negative PnL
			}.into();
			VammPallet::set_price(Some(new_price));
			let new_base_value = new_price.saturating_mul_int(base_amount_limit);
			assert_ok!(
				<TestPallet as ClearingHouse>::open_position(
					&ALICE,
					&market_id,
					direction.opposite(),
					new_base_value / 2,
					base_amount_limit / 2,
				),
				base_amount_limit / 2,
			);
		});
	}

	#[test]
	fn cannot_reverse_position_while_exceeding_max_leverage(
		direction in any_direction()
	) {
		let market_config = MarketConfig {
			margin_ratio_initial: (1, 10).into(),  // 1/10 IMR, or 10x leverage
			..Default::default()
		};
		let margin = as_balance(10);

		with_trading_context(market_config, margin, |market_id| {
			let price = 10;
			VammPallet::set_price(Some(price.into()));
			let quote_amount = 10 * margin; // 10x margin => max leverage
			let base_amount_limit = quote_amount / price;
			assert_ok!(
				<TestPallet as ClearingHouse>::open_position(
					&ALICE,
					&market_id,
					direction,
					quote_amount,
					base_amount_limit
				),
				base_amount_limit,
			);

			// Open trade in the opposite direction while increasing risk
			// This would leaves us with a position with greater size than the current one,
			// but the trade should be blocked because we're already at max leverage
			let quote_amount_delta = 2 * quote_amount + 100;
			let base_amount_delta = quote_amount_delta / price;
			assert_noop!(
				<TestPallet as ClearingHouse>::open_position(
					&ALICE,
					&market_id,
					direction.opposite(),
					quote_amount_delta,
					base_amount_delta,
				),
				Error::<Runtime>::InsufficientCollateral
			);
		});
	}

	#[test]
	fn cannot_reverse_into_dust_position(
		direction in any_direction(),
		(eps, minimum_trade_size) in decimal_a_lt_decimal_b((1, 100).into())
	) {
		let config = MarketConfig { minimum_trade_size, ..Default::default() };
		let quote_amount = as_balance(100);

		with_trading_context(config, quote_amount, |market_id| {
			let positions_before = TestPallet::get_positions(&ALICE).len();

			let base_amount_limit = as_balance(10);
			// price * base_amount_limit = quote_amount
			VammPallet::set_price(Some((quote_amount, base_amount_limit).into()));
			assert_ok!(TestPallet::open_position(
				Origin::signed(ALICE),
				market_id,
				direction,
				quote_amount,
				base_amount_limit,
			));

			// Try reversing while leaving a small resulting position in the opposite direction
			let eps_balance: Balance = eps.into_balance().unwrap();
			assert_ok!(TestPallet::open_position(
				Origin::signed(ALICE),
				market_id,
				direction.opposite(),
				quote_amount + eps_balance,
				base_amount_limit,
			));
			// The position should be closed, rather than leaving a dust position behind
			assert_eq!(TestPallet::get_positions(&ALICE).len(), positions_before);
			assert_eq!(get_market(&market_id).base_asset_amount(direction), 0.into());
		});
	}


	#[test]
	fn margin_ratio_takes_unrealized_funding_into_account(direction in any_direction()) {
		let config = MarketConfig {
			funding_frequency: ONE_HOUR,
			funding_period: ONE_HOUR,
			margin_ratio_initial: 1.into(), // 1x leverage
			..Default::default()
		};
		let margin = as_balance(100);

		cross_margin_context(vec![config.clone(), config], margin, |market_ids| {
			let price_cents = 100;  // price = 1.0
			VammPallet::set_price(Some((price_cents, 100).into()));
			let quote_amount = margin / 2;
			assert_ok!(
				<TestPallet as ClearingHouse>::open_position(
					&ALICE,
					&market_ids[0],
					direction,
					quote_amount,
					quote_amount,
				),
				quote_amount
			);

			// Update funding rate for 1st market
			run_for_seconds(ONE_HOUR);
			set_oracle_twap(&market_ids[0], (price_cents, 100).into());
			VammPallet::set_twap(Some((
				match direction { Long => price_cents + 1, _ => price_cents - 1 },
				100
			).into())); // funding rate = 1%
			assert_ok!(<TestPallet as ClearingHouse>::update_funding(&market_ids[0]));

			// Should fail since 1st market position is more than 0.5x leveraged due to unrealized
			// funding
			assert_noop!(
				<TestPallet as ClearingHouse>::open_position(
					&ALICE,
					&market_ids[1],
					direction,
					quote_amount,
					quote_amount,
				),
				Error::<Runtime>::InsufficientCollateral,
			);

		});
	}

	#[test]
	fn imr_is_combination_of_market_imrs_with_open_positions(direction in any_direction()) {
		let mut configs = Vec::<_>::new();
		let mut market_config = MarketConfig {
			margin_ratio_initial: (1, 10).into(),  // 10x leverage
			..Default::default()
		};
		configs.push(market_config.clone());
		market_config.margin_ratio_initial = (1, 20).into(); // 20x leverage
		configs.push(market_config);
		let margin = as_balance(60);

		cross_margin_context(configs, margin, |market_ids| {
			let price = 10;
			VammPallet::set_price(Some(price.into()));

			// Since the two markets have 10x and 20x max leverage respectively, the first has
			// two times more margin requirement than the second. Thus, it has double the weight
			// in calculating the account's max leverage. By splitting one third of our total
			// exposure in the first market and the rest in the second, we can have 15x max
			// leverage for our account.
			let quote_amount = as_balance(300); // (15 x 60 = 900)
			let base_amount = quote_amount / price;
			assert_ok!(
				<TestPallet as ClearingHouse>::open_position(
					&ALICE,
					&market_ids[0],
					direction,
					quote_amount,
					base_amount,
				),
				base_amount,
			);

			// For second market
			let quote_amount = as_balance(600);
			let base_amount = quote_amount / price;
			// This should exceed the max leverage and fail
			let quote_amount_fail = quote_amount + 100;
			let base_amount_fail = quote_amount_fail / price;
			assert_noop!(
				<TestPallet as ClearingHouse>::open_position(
					&ALICE,
					&market_ids[1],
					direction,
					quote_amount_fail,
					base_amount_fail,
				),
				Error::<Runtime>::InsufficientCollateral
			);

			// This should succeed (max leverage)
			assert_ok!(
				<TestPallet as ClearingHouse>::open_position(
					&ALICE,
					&market_ids[1],
					direction,
					quote_amount,
					base_amount,
				),
				base_amount
			);
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

			// Not enough time passes for a funding update to be possible
			run_for_seconds(config.funding_frequency / 2);
			assert_ok!(TestPallet::open_position(
				Origin::signed(ALICE),
				market_id,
				direction,
				size / 2,
				size / 20,
			));
			// Last funding update should be at time 0
			assert_eq!(get_market(&market_id).funding_rate_ts, 0);

			// Enough time passes for a funding update to be possible
			run_to_time(config.funding_frequency);
			assert_ok!(TestPallet::open_position(
				Origin::signed(ALICE),
				market_id,
				direction,
				size / 2,
				size / 20,
			));
			// Last funding update should be at time 60
			assert_eq!(get_market(&market_id).funding_rate_ts, config.funding_frequency);
		});
	}
}
