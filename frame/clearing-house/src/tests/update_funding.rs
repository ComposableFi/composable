use super::{
	any_balance, any_price, run_for_seconds, set_fee_pool_depth, traders_in_one_market_context,
	valid_market_config, with_market_context, with_trading_context, Balance, Position,
};
use crate::{
	math::{FixedPointMath, FromBalance, FromUnsigned, IntoDecimal},
	mock::{
		accounts::{AccountId, ALICE, BOB},
		runtime::{
			ExtBuilder, MarketId, Oracle as OraclePallet, Origin, Runtime, System as SystemPallet,
			TestPallet, Vamm as VammPallet, MINIMUM_PERIOD_SECONDS,
		},
	},
	Direction, Error, Event,
};
use composable_traits::{
	clearing_house::{ClearingHouse, Instruments},
	time::{DurationSeconds, ONE_HOUR},
};
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;
use sp_runtime::{FixedI128, FixedU128};

// ----------------------------------------------------------------------------------------------------
//                                             Helpers
// ----------------------------------------------------------------------------------------------------

fn get_position(account: &AccountId, market_id: &MarketId) -> Position {
	TestPallet::get_positions(account)
		.into_iter()
		.find(|p| p.market_id == *market_id)
		.unwrap()
}

// ----------------------------------------------------------------------------------------------------
//                                             Prop Compose
// ----------------------------------------------------------------------------------------------------

prop_compose! {
	fn seconds_lt(upper_bound: DurationSeconds)(
		s in MINIMUM_PERIOD_SECONDS..upper_bound
	) -> DurationSeconds {
		s
	}
}

prop_compose! {
	fn balance_in_open_interval(lower_bound: Balance, upper_bound: Balance)(
		balance in (lower_bound + 1)..upper_bound
	) -> Balance {
		balance
	}
}

prop_compose! {
	fn long_short_amounts_for_capped_funding()(
		alice_position in any_balance()
	)(
		alice_position in Just(alice_position),
		net_position in balance_in_open_interval(alice_position / 3, alice_position),
	) -> (Balance, Balance) {
		// Assuming a taker fee of 1% and a funding rate of 5% in favor of Alice's position, the net
		// position should be at least one third of Alice's position for funding to be capped
		// (for Alice). Derivation:
		// alice_position = bob_position + net_position
		// fees = (alice_position + bob_position) * 1%
		//      = 2 * bob_position * 1% + net_position * 1%
		// bob_funding = (alice_position - net_position) * 5%
		// alice_funding = alice_position * 5%
		//               > bob_funding + fees
		//               = 2 * (alice_position - net_position) * 1% + net_position * 1%
		//                 + (alice_position - net_position) * 5%
		//               = alice_position * 7% - net_position * 6%
		//               <=> net_position > alice_position / 3
		(alice_position, alice_position - net_position)
	}
}

// ----------------------------------------------------------------------------------------------------
//                                            Update Funding
// ----------------------------------------------------------------------------------------------------

proptest! {
	#[test]
	fn cannot_update_for_nonexistent_market(market_id in any::<MarketId>()) {
		ExtBuilder::default().build().execute_with(|| {
			assert_noop!(
				TestPallet::update_funding(Origin::signed(ALICE), market_id),
				Error::<Runtime>::MarketIdNotFound
			);
		})
	}

	#[test]
	fn enforces_funding_frequency(seconds in seconds_lt(ONE_HOUR)) {
		let mut config = valid_market_config();
		config.funding_frequency = ONE_HOUR;

		with_market_context(ExtBuilder::default(), config, |market_id| {
			run_for_seconds(seconds);
			assert_noop!(
				TestPallet::update_funding(Origin::signed(ALICE), market_id),
				Error::<Runtime>::UpdatingFundingTooEarly
			);

			run_for_seconds(ONE_HOUR - seconds);
			assert_ok!(TestPallet::update_funding(Origin::signed(ALICE), market_id));
		});
	}

	// TODO(0xangelo): what to expect if a lot of time has passed since the last update?

	#[test]
	fn updates_market_state(vamm_twap in any_price()) {
		let mut config = valid_market_config();
		config.funding_frequency = ONE_HOUR;

		with_market_context(ExtBuilder::default(), config, |market_id| {
			let old_market = TestPallet::get_market(&market_id).unwrap();

			run_for_seconds(ONE_HOUR);
			// Set new TWAPs
			OraclePallet::set_twap(Some(10_000)); // 100 in cents
			let oracle_twap: FixedU128 = 100.into();
			VammPallet::set_twap(Some(vamm_twap));
			// Hack: set Fee Pool depth so as not to worry about capped funding rates
			set_fee_pool_depth(&market_id, Balance::MAX);
			assert_ok!(TestPallet::update_funding(Origin::signed(ALICE), market_id));

			let new_market = TestPallet::get_market(&market_id).unwrap();
			let delta = FixedI128::from_unsigned(vamm_twap).unwrap()
				- FixedI128::from_unsigned(oracle_twap).unwrap();
			let update_weight: FixedI128 =
				(old_market.funding_frequency, old_market.funding_period).into();

			assert_eq!(new_market.funding_rate_ts, old_market.funding_rate_ts + ONE_HOUR);
			for direction in [Direction::Long, Direction::Short] {
				assert_eq!(
					new_market.cum_funding_rate(direction),
					old_market.cum_funding_rate(direction) + delta.try_mul(&update_weight).unwrap()
				);
			}

			SystemPallet::assert_last_event(
				Event::FundingUpdated {
					market: market_id, time: new_market.funding_rate_ts
				}.into(),
			)
		});
	}

	#[test]
	fn clearing_house_receives_funding(net_position in any_balance()) {
		let mut config = valid_market_config();
		config.funding_frequency = ONE_HOUR;
		config.funding_period = ONE_HOUR;
		config.taker_fee = 0;

		with_trading_context(config, net_position, |market_id| {
			VammPallet::set_price(Some(1.into()));
			// Alice opens a position representing the net one of all traders
			let _ = <TestPallet as ClearingHouse>::open_position(
				&ALICE,
				&market_id,
				Direction::Long,
				net_position,
				net_position,
			);

			let market = TestPallet::get_market(&market_id).unwrap();
			run_for_seconds(market.funding_frequency);
			OraclePallet::set_twap(Some(100)); // 1 in cents
			VammPallet::set_twap(Some(2.into()));
			assert_ok!(<TestPallet as ClearingHouse>::update_funding(&market_id));

			// funding rate is 1 ( TWAP_diff * freq / period )
			// payment = rate * net_position
			let payment = net_position;
			let market = TestPallet::get_market(&market_id).unwrap();
			assert_eq!(market.fee_pool, payment);
		});
	}

	#[test]
	fn clearing_house_pays_funding_uncapped(net_position in any_balance()) {
		let mut config = valid_market_config();
		config.funding_frequency = ONE_HOUR;
		config.funding_period = ONE_HOUR;
		config.taker_fee = 100; // 1%
		let fee = net_position / 100;

		with_trading_context(config.clone(), net_position + fee, |market_id| {
			VammPallet::set_price(Some(1.into()));
			// Alice opens a position representing the net one of all traders
			let _ = <TestPallet as ClearingHouse>::open_position(
				&ALICE,
				&market_id,
				Direction::Long,
				net_position,
				net_position,
			);

			run_for_seconds(config.funding_frequency);
			OraclePallet::set_twap(Some(101)); // 1.01 in cents
			VammPallet::set_twap(Some(1.into()));
			assert_ok!(<TestPallet as ClearingHouse>::update_funding(&market_id));

			// funding rate is 1% ( TWAP_diff * freq / period )
			// payment = rate * net_position = fee
			let market = TestPallet::get_market(&market_id).unwrap();
			// Whole fee pool is paid back in funding
			assert_eq!(market.fee_pool, 0);
		});
	}

	#[test]
	fn clearing_house_pays_funding_capped(
		(alice_position, bob_position) in long_short_amounts_for_capped_funding()
	) {
		let mut config = valid_market_config();
		config.funding_frequency = ONE_HOUR;
		config.funding_period = ONE_HOUR;
		config.minimum_trade_size = 0.into();
		config.taker_fee = 100; // 1%

		let fees = (alice_position / 100, bob_position / 100);
		let margins = vec![(ALICE, alice_position + fees.0), (BOB, bob_position + fees.1)];
		traders_in_one_market_context(config.clone(), margins, |market_id| {
			VammPallet::set_price(Some(1.into()));

			// Alice opens the bigger position representing the total amount of all long traders
			assert_ok!(<TestPallet as ClearingHouse>::open_position(
				&ALICE,
				&market_id,
				Direction::Long,
				alice_position,
				alice_position,
			));
			// Bob opens the smallet position representing the total amount of all short traders
			assert_ok!(<TestPallet as ClearingHouse>::open_position(
				&BOB,
				&market_id,
				Direction::Short,
				bob_position,
				bob_position,
			));
			let initial_fee_pool = fees.0 + fees.1;
			assert_eq!(TestPallet::get_market(&market_id).unwrap().fee_pool, initial_fee_pool);

			// Time passes and the external market moves in favor of Alice's position
			run_for_seconds(config.funding_frequency);
			OraclePallet::set_twap(Some(105)); // 0.95 in cents, a 5% move
			VammPallet::set_twap(Some(1.into())); // On-chain market hasn't changed
			assert_ok!(<TestPallet as ClearingHouse>::update_funding(&market_id));

			let market = TestPallet::get_market(&market_id).unwrap();
			// Bob owes 5% of his position in funding
			let bob_funding = FixedI128::from_balance(bob_position / 20).unwrap();
			assert_eq!(
				<TestPallet as Instruments>::unrealized_funding(
					&market,
					&get_position(&BOB, &market_id)
				).unwrap(),
				-bob_funding
			);
			// Alice can't realize her PnL based on the index price, but she should be owed the same
			// amount in funding payments. However, Bob's position + the Fee Pool cannot cover the
			// whole amount, so she is paid less
			let bob_and_fees = bob_funding + initial_fee_pool.into_decimal().unwrap();
			let alice_funding = <TestPallet as Instruments>::unrealized_funding(
				&market,
				&get_position(&ALICE, &market_id)
			).unwrap();
			assert!(alice_funding > bob_funding);
			assert!(alice_funding <= bob_and_fees);

			// System is airtight: all transfers are accounted for and no funds are left without a
			// destination
			let fee_pool_decimal = market.fee_pool.into_decimal().unwrap();
			assert_eq!(alice_funding + fee_pool_decimal, bob_and_fees);
		});
	}
}
