use super::valid_market_config;
use crate::{
	math::FromUnsigned,
	mock::{
		accounts::ALICE,
		runtime::{
			ExtBuilder, MarketId, Oracle as OraclePallet, Origin, Runtime, System as SystemPallet,
			TestPallet, Timestamp as TimestampPallet, Vamm as VammPallet, MINIMUM_PERIOD_SECONDS,
		},
	},
	tests::{any_price, run_to_block},
	Error, Event,
};
use composable_traits::{
	clearing_house::ClearingHouse,
	time::{DurationSeconds, ONE_HOUR},
};
use frame_support::{assert_noop, assert_ok, pallet_prelude::Hooks};
use proptest::prelude::*;
use sp_runtime::FixedI128;

fn run_for_seconds(seconds: DurationSeconds) {
	if SystemPallet::block_number() > 0 {
		TimestampPallet::on_finalize(SystemPallet::block_number());
		SystemPallet::on_finalize(SystemPallet::block_number());
	}
	SystemPallet::set_block_number(SystemPallet::block_number() + 1);
	// Time is set in milliseconds, so we multiply the seconds by 1_000
	let _ = TimestampPallet::set(Origin::none(), TimestampPallet::now() + 1_000 * seconds);
	SystemPallet::on_initialize(SystemPallet::block_number());
	TimestampPallet::on_initialize(SystemPallet::block_number());
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
		let funding_frequency = ONE_HOUR;

		ExtBuilder::default().build().execute_with(|| {
			run_to_block(1);
			let mut config = valid_market_config();
			config.funding_frequency = funding_frequency;
			let market_id = <TestPallet as ClearingHouse>::create_market(&config).unwrap();

			run_for_seconds(seconds);
			assert_noop!(
				TestPallet::update_funding(Origin::signed(ALICE), market_id),
				Error::<Runtime>::UpdatingFundingTooEarly
			);

			run_for_seconds(ONE_HOUR - seconds);
			assert_ok!(TestPallet::update_funding(Origin::signed(ALICE), market_id));
		})
	}

	// TODO(0xangelo): what to expect if a lot of time has passed since the last update?

	#[test]
	fn updates_market_state(new_mark_price in any_price()) {
		let funding_frequency = ONE_HOUR;
		let oracle_price = 10_000; // 100 in cents
		let vamm_price = 100.into();

		ExtBuilder::default().build().execute_with(|| {
			run_to_block(1);
			// Set oracle and vamm TWAPs
			OraclePallet::set_twap(Some(oracle_price));
			VammPallet::set_twap(Some(vamm_price));

			let mut config = valid_market_config();
			config.funding_frequency = funding_frequency;
			let market_id = <TestPallet as ClearingHouse>::create_market(&config).unwrap();
			let old_market = TestPallet::get_market(&market_id).unwrap();

			run_for_seconds(ONE_HOUR);
			// Set new oracle and vamm TWAPs
			OraclePallet::set_twap(Some(oracle_price));
			VammPallet::set_twap(Some(new_mark_price));

			assert_ok!(TestPallet::update_funding(Origin::signed(ALICE), market_id));

			let new_market = TestPallet::get_market(&market_id).unwrap();
			let delta = FixedI128::from_unsigned(new_mark_price).unwrap()
				- FixedI128::from_unsigned(vamm_price).unwrap();
			assert_eq!(new_market.funding_rate_ts, old_market.funding_rate_ts + ONE_HOUR);
			assert_eq!(
				new_market.cum_funding_rate,
				old_market.cum_funding_rate +
					delta *
					FixedI128::from((old_market.funding_frequency, old_market.funding_period))
			);

			SystemPallet::assert_last_event(
				Event::FundingUpdated {
					market: market_id, time: new_market.funding_rate_ts
				}.into(),
			)
		})
	}
}
