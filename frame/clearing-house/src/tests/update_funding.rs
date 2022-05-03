use super::{
	any_balance, any_price, valid_market_config, MarketConfig, MarketInitializer, RunToBlock,
};
use crate::{
	math::FromUnsigned,
	mock::{
		accounts::ALICE,
		assets::USDC,
		runtime::{
			ExtBuilder, MarketId, Origin, Runtime, System as SystemPallet, TestPallet,
			Timestamp as TimestampPallet, Vamm as VammPallet, MINIMUM_PERIOD_SECONDS,
		},
	},
	Error, Event,
};
use composable_traits::time::{DurationSeconds, ONE_HOUR};
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
//                                                Setup
// ----------------------------------------------------------------------------------------------------

fn with_market_context<R>(
	ext_builder: ExtBuilder,
	config: MarketConfig,
	execute: impl FnOnce(MarketId) -> R,
) -> R {
	let mut ext = ext_builder.build().run_to_block(1);
	let market_id = ext.execute_with(|| {
		<sp_io::TestExternalities as MarketInitializer>::create_market_helper(Some(config))
	});

	ext.execute_with(|| execute(market_id))
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
		let mut config = valid_market_config();
		config.funding_frequency = ONE_HOUR;

		with_market_context(
			ExtBuilder::default(),
			config,
			|market_id| {
				run_for_seconds(seconds);
				assert_noop!(
					TestPallet::update_funding(Origin::signed(ALICE), market_id),
					Error::<Runtime>::UpdatingFundingTooEarly
				);

				run_for_seconds(ONE_HOUR - seconds);
				assert_ok!(TestPallet::update_funding(Origin::signed(ALICE), market_id));
			}
		);
	}

	// TODO(0xangelo): what to expect if a lot of time has passed since the last update?

	#[test]
	fn updates_market_state(new_vamm_twap in any_price()) {
		let mut config = valid_market_config();
		config.funding_frequency = ONE_HOUR;
		let vamm_twap = 100.into();

		with_market_context(
			ExtBuilder {
				oracle_twap: Some(10_000), // 100 in cents
				vamm_twap: Some(vamm_twap),
				..Default::default()
			},
			config,
			|market_id| {
				let old_market = TestPallet::get_market(&market_id).unwrap();

				run_for_seconds(ONE_HOUR);
				// Set new vamm TWAP, leave oracle unchanged
				VammPallet::set_twap(Some(new_vamm_twap));

				assert_ok!(TestPallet::update_funding(Origin::signed(ALICE), market_id));

				let new_market = TestPallet::get_market(&market_id).unwrap();
				let delta = FixedI128::from_unsigned(new_vamm_twap).unwrap()
					- FixedI128::from_unsigned(vamm_twap).unwrap();
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
			}
		);

	}
}
