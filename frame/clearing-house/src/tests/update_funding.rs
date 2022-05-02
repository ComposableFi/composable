use super::valid_market_config;
use crate::{
	mock::{
		accounts::ALICE,
		runtime::{ExtBuilder, Origin, Runtime, TestPallet},
	},
	tests::run_to_block,
	Error,
};
use composable_traits::{
	clearing_house::ClearingHouse,
	time::{DurationSeconds, ONE_HOUR},
};
use frame_support::{assert_noop, assert_ok};

fn run_for_seconds(seconds: DurationSeconds) {
	todo!()
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
