use crate::{
	mock::{
		accounts::ALICE,
		assets::USDC,
		runtime::{
			Balance, ExtBuilder, MarketId, Oracle as OraclePallet, Origin, Runtime,
			System as SystemPallet, TestPallet, Vamm as VammPallet,
		},
	},
	pallet::{Config, Direction, Error, Event},
	tests::{
		any_price, as_balance, run_for_seconds, set_fee_pool_depth, with_markets_context,
		with_trading_context, MarketConfig,
	},
};
use composable_traits::{clearing_house::ClearingHouse, time::ONE_HOUR};
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;
use sp_runtime::{FixedI128, FixedPointNumber, FixedU128};

// ----------------------------------------------------------------------------------------------------
//                                        Execution Contexts
// ----------------------------------------------------------------------------------------------------

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

// ----------------------------------------------------------------------------------------------------
//                                          Valid Inputs
// ----------------------------------------------------------------------------------------------------

fn valid_quote_asset_amount() -> Balance {
	as_balance(100)
}

fn valid_base_asset_amount_limit() -> Balance {
	as_balance(10)
}

fn valid_market_config() -> MarketConfig {
	MarketConfig { taker_fee: 0, ..Default::default() }
}

// ----------------------------------------------------------------------------------------------------
//                                             Prop Compose
// ----------------------------------------------------------------------------------------------------

fn any_direction() -> impl Strategy<Value = Direction> {
	prop_oneof![Just(Direction::Long), Just(Direction::Short)]
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

// ----------------------------------------------------------------------------------------------------
//                                            Open Position
// ----------------------------------------------------------------------------------------------------

#[test]
fn fails_to_open_position_if_market_id_invalid() {
	let quote_amount = valid_quote_asset_amount();
	let base_amount_limit = valid_base_asset_amount_limit();

	with_trading_context(MarketConfig::default(), quote_amount, |market_id| {
		// Current price = quote_amount / base_amount_limit
		VammPallet::set_price(Some((quote_amount, base_amount_limit).into()));

		assert_noop!(
			TestPallet::open_position(
				Origin::signed(ALICE),
				market_id + 1,
				Direction::Long,
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
	let configs = vec![valid_market_config(); orders];
	let margin = valid_quote_asset_amount();

	cross_margin_context(configs, margin, |market_ids| {
		let quote_amount: Balance = margin / (orders as u128);
		let base_amount_limit: Balance = valid_base_asset_amount_limit() / (orders as u128);

		// Current price = quote_amount / base_amount_limit
		VammPallet::set_price(Some((quote_amount, base_amount_limit).into()));

		for market_id in market_ids.iter().take(max_positions) {
			assert_ok!(TestPallet::open_position(
				Origin::signed(ALICE),
				*market_id,
				Direction::Long,
				quote_amount,
				base_amount_limit,
			));
		}

		assert_noop!(
			TestPallet::open_position(
				Origin::signed(ALICE),
				market_ids[max_positions],
				Direction::Long,
				quote_amount,
				base_amount_limit,
			),
			Error::<Runtime>::MaxPositionsExceeded
		);
	});
}

proptest! {
	#[test]
	fn open_position_in_new_market_succeeds(direction in any_direction()) {
		let mut config = valid_market_config();
		config.taker_fee = 10; // 0.1%
		let quote_amount = valid_quote_asset_amount();
		let fees = (quote_amount * config.taker_fee) / 10_000;

		// Have enough margin to pay for fees
		with_trading_context(config, quote_amount + fees, |market_id| {
			let positions_before = TestPallet::get_positions(&ALICE).len();

			let base_amount = valid_base_asset_amount_limit();
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
			let positions = TestPallet::get_positions(&ALICE);
			assert_eq!(positions.len(), positions_before + 1);
			let position = positions.iter().find(|p| p.market_id == market_id).unwrap();
			assert!(match direction {
				Direction::Long => position.base_asset_amount.is_positive(),
				Direction::Short => position.base_asset_amount.is_negative()
			});
			assert!(match direction {
				Direction::Long => position.quote_asset_notional_amount.is_positive(),
				Direction::Short => position.quote_asset_notional_amount.is_negative()
			});

			// Ensure cumulative funding is initialized to market's current
			let market = TestPallet::get_market(&market_id).unwrap();
			assert_eq!(position.last_cum_funding, market.cum_funding_rate(direction));

			// Ensure fees are deducted from margin
			assert_eq!(TestPallet::get_collateral(&ALICE), Some(quote_amount));

			// Ensure market state is updated:
			// - net position
			// - fees collected
			assert_eq!(market.base_asset_amount(direction), position.base_asset_amount);
			assert_eq!(market.fee_pool, fees);

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
		let mut market_config = valid_market_config();
		market_config.minimum_trade_size = minimum_trade_size;
		let quote_amount = eps.unsigned_abs();

		with_trading_context(market_config, quote_amount, |market_id| {
			VammPallet::set_price(Some(1.into()));

			assert_noop!(
				TestPallet::open_position(
					Origin::signed(ALICE),
					market_id,
					match eps.is_positive() {
						true => Direction::Long,
						false => Direction::Short,
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
		let mut config = valid_market_config();
		config.minimum_trade_size = minimum_trade_size;
		let quote_amount = valid_quote_asset_amount();

		with_trading_context(config, quote_amount, |market_id| {
			let positions_before = TestPallet::get_positions(&ALICE).len();

			let base_amount_limit = valid_base_asset_amount_limit();
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

			let market = TestPallet::get_market(&market_id).unwrap();
			assert_eq!(market.base_asset_amount(direction), 0.into());
		});
	}

	#[test]
	fn trading_realizes_position_funding_payments(
		direction in any_direction(),
		rate in -100..=100_i128, // -1% to 1% in basis points
	) {
		let mut config = valid_market_config();
		config.funding_frequency = ONE_HOUR;
		config.funding_period = ONE_HOUR;
		config.margin_ratio_initial = (1, 10).into(); // allow 10x leverage, more than enough
		config.taker_fee = 100; // 1%
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
			OraclePallet::set_twap(Some(100)); // 1.0 in cents
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
			let sign = match direction { Direction::Long => -1, _ => 1 };
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

		with_trading_context(valid_market_config(), quote_amount, |market_id| {
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
			let sign = match direction { Direction::Long => 1, _ => -1 };
			let margin = quote_amount as i128;
			let pnl = sign * (new_base_value as i128) - sign * margin;
			assert_eq!(
				TestPallet::get_collateral(&ALICE).unwrap(),
				(margin + pnl).max(0) as u128
			);
		});
	}

	#[test]
	fn reducing_position_partially_realizes_pnl(
		direction in any_direction(),
		new_price in any_price(),
		percentf in percentage_fraction()
	) {
		let mut market_config = valid_market_config();
		market_config.minimum_trade_size = 0.into();
		let quote_amount = as_balance(100);

		with_trading_context(market_config, quote_amount, |market_id| {
			let positions_before = TestPallet::get_positions(&ALICE).len();

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


			VammPallet::set_price(Some(new_price));
			// Reduce (close) position by desired percentage
			let base_amount_to_close = percentf.saturating_mul_int(base_amount);
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

			let positions = TestPallet::get_positions(&ALICE);
			// Positions remains open
			assert_eq!(positions.len(), positions_before + 1);

			// Fraction of the PnL is realized
			let sign = match direction { Direction::Long => 1, _ => -1 };
			let entry_value = percentf.saturating_mul_int(quote_amount);
			let pnl = sign * (base_value_to_close as i128) - sign * (entry_value as i128);
			assert_eq!(
				TestPallet::get_collateral(&ALICE).unwrap(),
				(quote_amount as i128 + pnl).max(0) as u128
			);

			let position = positions.iter().find(|p| p.market_id == market_id).unwrap();
			// Position base asset and quote asset notional are cut by percentage
			assert_eq!(
				position.base_asset_amount.into_inner(),
				sign * ((base_amount - base_amount_to_close) as i128)
			);
			assert_eq!(
				position.quote_asset_notional_amount.into_inner(),
				sign * ((quote_amount - entry_value) as i128)
			);

			let market = TestPallet::get_market(&market_id).unwrap();
			assert_eq!(market.base_asset_amount(direction), position.base_asset_amount);
		});
	}

	#[test]
	fn reversing_position_realizes_pnl(
		direction in any_direction(),
		new_price in any_price()
	) {
		let mut market_config = valid_market_config();
		market_config.minimum_trade_size = 0.into();
		let quote_amount = as_balance(100);

		with_trading_context(market_config, quote_amount, |market_id| {
			let positions_before = TestPallet::get_positions(&ALICE).len();

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

			let sign = match direction { Direction::Long => 1, _ => -1 };
			// Full PnL is realized
			let margin = quote_amount as i128;
			let pnl = sign * (new_base_value as i128) - sign * margin;
			assert_eq!(
				TestPallet::get_collateral(&ALICE).unwrap(),
				(margin + pnl).max(0) as u128
			);

			// Position remains open
			let positions = TestPallet::get_positions(&ALICE);
			assert_eq!(positions.len(), positions_before + 1);

			let position = positions.iter().find(|p| p.market_id == market_id).unwrap();
			assert_eq!(
				position.base_asset_amount,
				FixedI128::from_inner(- sign * base_amount as i128)
			);
			assert_eq!(
				position.quote_asset_notional_amount,
				FixedI128::from_inner(- sign * new_base_value as i128)
			);

			let market = TestPallet::get_market(&market_id).unwrap();
			assert_eq!(market.base_asset_amount(direction), 0.into());
			assert_eq!(market.base_asset_amount(direction.opposite()), position.base_asset_amount);
		});
	}

	#[test]
	fn fails_to_create_new_position_without_enough_margin(
		direction in any_direction(),
		excess in 1..as_balance(1_000_000),
	) {
		let mut market_config = valid_market_config();
		market_config.margin_ratio_initial = (1, 10).into();  // 1/10 IMR, or 10x leverage
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
		let mut market_config = valid_market_config();
		market_config.margin_ratio_initial = (100, max_leverage_percent).into();
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
		let mut market_config = valid_market_config();
		market_config.margin_ratio_initial = (1, 10).into();  // 1/10 IMR, or 10x leverage
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
				Direction::Long => 8, // decrease price => negative PnL
				Direction::Short => 12, // increase price => negative PnL
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
		let mut market_config = valid_market_config();
		market_config.margin_ratio_initial = (1, 10).into();  // 1/10 IMR, or 10x leverage
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

	// TODO(0xangelo): cannot reverse into dust position (< min_trade_size)

	#[test]
	fn margin_ratio_takes_unrealized_funding_into_account(direction in any_direction()) {
		let mut config = valid_market_config();
		config.funding_frequency = ONE_HOUR;
		config.funding_period = ONE_HOUR;
		config.margin_ratio_initial = 1.into(); // 1x leverage
		config.taker_fee = 0;
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
			OraclePallet::set_twap(Some(price_cents));
			VammPallet::set_twap(Some((
				match direction { Direction::Long => price_cents + 1, _ => price_cents - 1 },
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
		let mut market_config = valid_market_config();
		market_config.margin_ratio_initial = (1, 10).into(); // 10x leverage
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
}
