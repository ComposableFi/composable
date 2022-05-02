use super::valid_market_config;
use crate::{
	mock::{
		accounts::ALICE,
		runtime::{
			ExtBuilder, Origin, Runtime, System as SystemPallet, TestPallet,
			Timestamp as TimestampPallet,
		},
	},
	tests::run_to_block,
	Error,
};
use composable_traits::{
	clearing_house::ClearingHouse,
	time::{DurationSeconds, ONE_HOUR},
};
use frame_support::{assert_noop, assert_ok, pallet_prelude::Hooks};

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

#[test]
fn enforces_funding_frequency() {
	let funding_frequency = ONE_HOUR;
	let percent = 10;

	ExtBuilder::default().build().execute_with(|| {
		run_to_block(1);
		let mut config = valid_market_config();
		config.funding_frequency = ONE_HOUR;
		let market_id = <TestPallet as ClearingHouse>::create_market(&config).unwrap();

		run_for_seconds((percent * funding_frequency) / 100);
		assert_noop!(
			TestPallet::update_funding(Origin::signed(ALICE), market_id),
			Error::<Runtime>::UpdatingFundingTooEarly
		);

		run_for_seconds(((100 - percent) * funding_frequency) / 100);
		assert_ok!(TestPallet::update_funding(Origin::signed(ALICE), market_id));
	})
}
