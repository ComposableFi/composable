use super::{any_balance, any_price, valid_market_config, with_market_context};
use crate::{
	math::FromUnsigned,
	mock::{
		accounts::{AccountId, ALICE},
		assets::USDC,
		runtime::{
			Assets as AssetsPallet, ExtBuilder, MarketId, Oracle as OraclePallet, Origin, Runtime,
			System as SystemPallet, TestPallet, TestPalletId, Timestamp as TimestampPallet,
			Vamm as VammPallet, MINIMUM_PERIOD_SECONDS,
		},
	},
	Direction, Error, Event,
};
use composable_traits::{
	clearing_house::ClearingHouse,
	time::{DurationSeconds, ONE_HOUR},
};
use frame_support::{assert_noop, assert_ok, pallet_prelude::Hooks, traits::fungibles::Inspect};
use proptest::prelude::*;
use sp_runtime::{traits::AccountIdConversion, FixedI128, FixedU128};

// ----------------------------------------------------------------------------------------------------
//                                               Helpers
// ----------------------------------------------------------------------------------------------------

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
		let ext_builder = ExtBuilder {
			..Default::default()
		};

		with_market_context(ext_builder, config, |market_id| {
			let old_market = TestPallet::get_market(&market_id).unwrap();

			run_for_seconds(ONE_HOUR);
			// Set new TWAPs
			OraclePallet::set_twap(Some(10_000)); // 100 in cents
			let oracle_twap: FixedU128 = 100.into();
			VammPallet::set_twap(Some(vamm_twap));

			assert_ok!(TestPallet::update_funding(Origin::signed(ALICE), market_id));

			let new_market = TestPallet::get_market(&market_id).unwrap();
			let delta = FixedI128::from_unsigned(vamm_twap).unwrap()
				- FixedI128::from_unsigned(oracle_twap).unwrap();
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
		});
	}

	#[test]
	fn clearing_house_receives_funding(net_position in any_balance()) {
		let mut config = valid_market_config();
		config.funding_frequency = ONE_HOUR;
		config.funding_period = ONE_HOUR;
		let ext_builder = ExtBuilder {
			balances: vec![(ALICE, USDC, net_position)],
			..Default::default()
		};

		with_market_context(ext_builder, config, |market_id| {
			let _ = <TestPallet as ClearingHouse>::add_margin(&ALICE, USDC, net_position);
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

			let insurance_acc = TestPalletId::get().into_sub_account("Insurance");
			assert_eq!(<AssetsPallet as Inspect<AccountId>>::balance(USDC, &insurance_acc), payment);
		});
	}
}
