use crate::{
	math::{FixedPointMath, FromBalance, FromUnsigned, IntoDecimal},
	mock::{
		accounts::{AccountId, ALICE, BOB},
		runtime::{
			ExtBuilder, MarketId, Origin, Runtime, System as SystemPallet, TestPallet,
			Vamm as VammPallet, MINIMUM_PERIOD_SECONDS,
		},
	},
	tests::{
		any_balance, any_price, as_balance, get_market, get_market_fee_pool, run_for_seconds,
		run_to_time, set_fee_pool_depth, set_maximum_oracle_mark_divergence,
		set_maximum_twap_divergence, set_oracle_price, set_oracle_twap,
		traders_in_one_market_context, with_market_context, with_trading_context, Balance, Market,
		MarketConfig, Position,
	},
	Direction, Error, Event,
};
use composable_traits::{
	clearing_house::{ClearingHouse, Instruments},
	time::{DurationSeconds, ONE_HOUR},
	vamm::{AssetType, Vamm},
};
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;
use sp_runtime::{FixedI128, FixedU128};

// -------------------------------------------------------------------------------------------------
//                                             Helpers
// -------------------------------------------------------------------------------------------------

fn get_position(account: &AccountId, market_id: &MarketId) -> Position {
	TestPallet::get_positions(account)
		.into_iter()
		.find(|p| p.market_id == *market_id)
		.unwrap()
}

// -------------------------------------------------------------------------------------------------
//                                           Unit tests
// -------------------------------------------------------------------------------------------------

#[test]
fn should_update_oracle_and_vamm_twaps() {
	let config = MarketConfig {
		funding_frequency: ONE_HOUR,
		funding_period: ONE_HOUR,
		twap_period: ONE_HOUR,
		..Default::default()
	};

	// Set oracle price at genesis
	let ext_builder = ExtBuilder { oracle_price: Some(100) /* 1.0 */, ..Default::default() };
	with_market_context(ext_builder, config, |market_id| {
		// Get variables before update (timestamp should be 0)
		let Market { vamm_id, last_oracle_ts, last_oracle_price, last_oracle_twap, .. } =
			get_market(&market_id);
		assert_eq!(last_oracle_ts, 0);
		assert_eq!(last_oracle_price, 1.into());
		assert_eq!(last_oracle_twap, 1.into());

		// Run to next available update time
		run_to_time(ONE_HOUR);
		set_oracle_price(&market_id, 1.into());
		VammPallet::set_next_twap_of(&vamm_id, Some(1.into()));
		assert_ok!(<TestPallet as ClearingHouse>::update_funding(&market_id));

		let market = get_market(&market_id);
		// Check that oracle price and twap were updated; in this case the price stayed the same
		assert_eq!(market.last_oracle_price, 1.into());
		assert_eq!(market.last_oracle_twap, market.last_oracle_price);
		assert_eq!(market.last_oracle_ts, ONE_HOUR);
		// Assert `Vamm::update_twap` was called
		assert_eq!(VammPallet::get_twap(vamm_id, AssetType::Base).unwrap(), 1.into());
	});
}

#[test]
fn late_update_has_same_weight_as_normal_update() {
	let config = MarketConfig {
		funding_frequency: ONE_HOUR,
		funding_period: ONE_HOUR * 24,
		..Default::default()
	};

	with_market_context(ExtBuilder::default(), config, |market_id| {
		// `with_market_context` creates the market at block 1 with timestamp 0s
		let market_t0 = TestPallet::get_market(&market_id).unwrap();

		// Set mark-index price divergence
		set_oracle_twap(&market_id, 100.into());
		VammPallet::set_twap(Some(150.into()));

		// HACK: set fee pool depth so as not to worry about capped funding rates
		set_fee_pool_depth(&market_id, Balance::MAX);

		// Update after normal interval
		run_for_seconds(ONE_HOUR);
		assert_ok!(TestPallet::update_funding(Origin::signed(ALICE), market_id));

		// Get delta of normal update
		let market_t1 = TestPallet::get_market(&market_id).unwrap();
		let delta_0 = market_t1.cum_funding_rate(Direction::Long) -
			market_t0.cum_funding_rate(Direction::Long);

		// Update after prolonged interval (keeping mark-index divergence the same)
		run_for_seconds(ONE_HOUR * 2);
		assert_ok!(TestPallet::update_funding(Origin::signed(ALICE), market_id));

		// Assert delta is equal to delta of normal update
		let market_t2 = TestPallet::get_market(&market_id).unwrap();
		let delta_1 = market_t2.cum_funding_rate(Direction::Long) -
			market_t1.cum_funding_rate(Direction::Long);
		assert_eq!(delta_0, delta_1);
	});
}

#[test]
fn late_update_may_push_next_update_time_to_later() {
	let config = MarketConfig {
		funding_frequency: ONE_HOUR,
		funding_period: ONE_HOUR * 24,
		..Default::default()
	};

	with_market_context(ExtBuilder::default(), config, |market_id| {
		// Set mark-index price divergence
		set_oracle_twap(&market_id, 100.into());
		VammPallet::set_twap(Some(150.into()));

		// HACK: set fee pool depth so as not to worry about capped funding rates
		set_fee_pool_depth(&market_id, Balance::MAX);

		// Update after more than 1 third of the frequency past the usual frequency time
		run_to_time(ONE_HOUR + ONE_HOUR / 3 + 1);
		assert_ok!(TestPallet::update_funding(Origin::signed(ALICE), market_id));

		// Try updating at next usual frequency time
		run_to_time(ONE_HOUR * 2);
		assert_noop!(
			TestPallet::update_funding(Origin::signed(ALICE), market_id),
			Error::<Runtime>::UpdatingFundingTooEarly
		);

		// Try updating before the frequency time after that
		run_to_time(ONE_HOUR * 2 + ONE_HOUR / 2);
		assert_noop!(
			TestPallet::update_funding(Origin::signed(ALICE), market_id),
			Error::<Runtime>::UpdatingFundingTooEarly
		);

		// Update at the right frequency time
		run_to_time(ONE_HOUR * 3);
		assert_ok!(TestPallet::update_funding(Origin::signed(ALICE), market_id));
	});
}

#[test]
fn late_update_may_not_interfere_with_next_update_time() {
	let config = MarketConfig {
		funding_frequency: ONE_HOUR,
		funding_period: ONE_HOUR * 24,
		..Default::default()
	};

	with_market_context(ExtBuilder::default(), config, |market_id| {
		// Set mark-index price divergence
		set_oracle_twap(&market_id, 100.into());
		VammPallet::set_twap(Some(150.into()));

		// HACK: set fee pool depth so as not to worry about capped funding rates
		set_fee_pool_depth(&market_id, Balance::MAX);

		// Update after less than 1 third of the frequency past the usual frequency time
		run_to_time(ONE_HOUR + ONE_HOUR / 3 - 1);
		assert_ok!(TestPallet::update_funding(Origin::signed(ALICE), market_id));

		// Successfully update at next usual frequency time
		run_to_time(ONE_HOUR * 2);
		assert_ok!(TestPallet::update_funding(Origin::signed(ALICE), market_id));
	});
}

#[test]
fn should_block_update_if_mark_index_too_divergent() {
	let config = MarketConfig {
		funding_frequency: ONE_HOUR,
		funding_period: ONE_HOUR * 24,
		..Default::default()
	};

	with_market_context(ExtBuilder::default(), config, |market_id| {
		// Set max divergence to 10%
		set_maximum_oracle_mark_divergence((1, 10).into());

		// Set mark-index price divergence
		set_oracle_twap(&market_id, 100.into());
		VammPallet::set_twap(Some(111.into()));

		// HACK: set fee pool depth so as not to worry about capped funding rates
		set_fee_pool_depth(&market_id, Balance::MAX);

		// Set mark more than 10% away from oracle
		VammPallet::set_price(Some(111.into()));

		// Update after normal interval
		run_for_seconds(ONE_HOUR);
		assert_noop!(
			TestPallet::update_funding(Origin::signed(ALICE), market_id),
			Error::<Runtime>::OracleMarkTooDivergent
		);
	});
}

#[test]
fn should_clip_twap_divergence_for_update() {
	let config = MarketConfig {
		funding_frequency: ONE_HOUR,
		funding_period: ONE_HOUR,
		..Default::default()
	};
	let net_position = as_balance(100);

	with_trading_context(config.clone(), net_position, |market_id| {
		// Set maximum TWAP divergence for update at 3%
		set_maximum_twap_divergence((3, 100).into());

		VammPallet::set_price(Some(1.into()));

		// Alice opens a position representing the net one of all traders
		// 1st TWAP updates here
		let _ = <TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market_id,
			Direction::Long,
			net_position,
			net_position,
		);

		run_for_seconds(config.funding_frequency);
		set_oracle_twap(&market_id, 1.into());
		VammPallet::set_twap(Some(2.into()));
		// 2nd TWAP updates here
		assert_ok!(<TestPallet as ClearingHouse>::update_funding(&market_id));

		// TWAP_diff is clipped from 100% to 3%
		// funding rate is 3% ( TWAP_diff * freq / period )
		// payment = rate * net_position
		assert_eq!(get_market_fee_pool(&market_id), (net_position * 3) / 100);
	});
}

// -------------------------------------------------------------------------------------------------
//                                          Prop compose
// -------------------------------------------------------------------------------------------------

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

// -------------------------------------------------------------------------------------------------
//                                         Property tests
// -------------------------------------------------------------------------------------------------

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
		let config = MarketConfig { funding_frequency: ONE_HOUR, ..Default::default() };

		with_market_context(ExtBuilder::default(), config, |market_id| {
			// update rate at exact multiple of funding frequency
			run_to_time(ONE_HOUR);
			assert_ok!(TestPallet::update_funding(Origin::signed(ALICE), market_id));

			// try updating before the expected wait time
			run_for_seconds(seconds);
			assert_noop!(
				TestPallet::update_funding(Origin::signed(ALICE), market_id),
				Error::<Runtime>::UpdatingFundingTooEarly
			);

			run_for_seconds(ONE_HOUR - seconds);
			assert_ok!(TestPallet::update_funding(Origin::signed(ALICE), market_id));
		});
	}

	#[test]
	fn updates_market_state(vamm_twap in any_price()) {
		let config = MarketConfig { funding_frequency: ONE_HOUR, ..Default::default() };

		with_market_context(ExtBuilder::default(), config.clone(), |market_id| {
			let old_market = TestPallet::get_market(&market_id).unwrap();

			run_for_seconds(ONE_HOUR);
			// Set new TWAPs
			set_oracle_twap(&market_id, 100.into());
			let oracle_twap: FixedU128 = 100.into();
			VammPallet::set_twap(Some(vamm_twap));
			// HACK: set Fee Pool depth so as not to worry about capped funding rates
			set_fee_pool_depth(&market_id, Balance::MAX);
			assert_ok!(TestPallet::update_funding(Origin::signed(ALICE), market_id));

			let new_market = TestPallet::get_market(&market_id).unwrap();
			let delta = FixedI128::from_unsigned(vamm_twap).unwrap()
				- FixedI128::from_unsigned(oracle_twap).unwrap();
			let update_weight: FixedI128 = (config.funding_frequency, config.funding_period).into();

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
		let config = MarketConfig {
			funding_frequency: ONE_HOUR,
			funding_period: ONE_HOUR,
			..Default::default()
		};

		with_trading_context(config.clone(), net_position, |market_id| {
			VammPallet::set_price(Some(1.into()));

			// Alice opens a position representing the net one of all traders
			// 1st TWAP updates here
			let _ = <TestPallet as ClearingHouse>::open_position(
				&ALICE,
				&market_id,
				Direction::Long,
				net_position,
				net_position,
			);

			run_for_seconds(config.funding_frequency);
			set_oracle_twap(&market_id, 1.into());
			VammPallet::set_twap(Some(2.into()));
			// 2nd TWAP updates here
			assert_ok!(<TestPallet as ClearingHouse>::update_funding(&market_id));

			// funding rate is 1 ( TWAP_diff * freq / period )
			// payment = rate * net_position
			assert_eq!(get_market_fee_pool(&market_id), net_position);
		});
	}

	#[test]
	fn clearing_house_pays_funding_uncapped(net_position in any_balance()) {
		let config = MarketConfig {
			funding_frequency: ONE_HOUR,
			funding_period: ONE_HOUR,
			taker_fee: 100, // 1%
			..Default::default()
		};
		let fee = net_position / 100;

		with_trading_context(config.clone(), net_position + fee, |market_id| {
			VammPallet::set_price(Some(1.into()));

			// Alice opens a position representing the net one of all traders
			// 1st TWAP updates here
			let _ = <TestPallet as ClearingHouse>::open_position(
				&ALICE,
				&market_id,
				Direction::Long,
				net_position,
				net_position,
			);

			// 2st TWAP updates here
			run_for_seconds(config.funding_frequency);
			set_oracle_twap(&market_id, (101, 100).into()); // 1.01
			VammPallet::set_twap(Some(1.into()));
			assert_ok!(<TestPallet as ClearingHouse>::update_funding(&market_id));

			// funding rate is 1% ( TWAP_diff * freq / period )
			// payment = rate * net_position = fee
			// Whole fee pool is paid back in funding
			assert_eq!(get_market_fee_pool(&market_id), 0);
		});
	}

	#[test]
	fn clearing_house_pays_funding_capped(
		(alice_position, bob_position) in long_short_amounts_for_capped_funding()
	) {
		let config = MarketConfig {
			funding_frequency: ONE_HOUR,
			funding_period: ONE_HOUR,
			minimum_trade_size: 0.into(),
			taker_fee: 100, // 1%
			..Default::default()
		};

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

			// Bob opens the smallest position representing the total amount of all short traders
			assert_ok!(<TestPallet as ClearingHouse>::open_position(
				&BOB,
				&market_id,
				Direction::Short,
				bob_position,
				bob_position,
			));

			let initial_fee_pool = fees.0 + fees.1;
			assert_eq!(get_market_fee_pool(&market_id), initial_fee_pool);

			// Time passes and the external market is in favor of Alice's position
			run_for_seconds(config.funding_frequency);
			set_oracle_twap(&market_id, (105, 100).into());
			VammPallet::set_twap(Some(1.into()));
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
			let fee_pool_decimal = get_market_fee_pool(&market_id).into_decimal().unwrap();
			assert_eq!(alice_funding + fee_pool_decimal, bob_and_fees);
		});
	}
}
