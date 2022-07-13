use crate::{
	mock::{
		accounts::ALICE,
		runtime::{MarketId, Origin, Runtime, TestPallet},
	},
	tests::ExtBuilder,
	Error,
};
use composable_traits::time::DurationSeconds;
use frame_support::assert_noop;
use proptest::prelude::*;

proptest! {
	#[test]
	fn should_fail_if_market_does_not_exist(
		market_id in any::<MarketId>(),
		time in any::<DurationSeconds>()
	) {
		ExtBuilder::default().build().execute_with(|| {
			assert_noop!(
			TestPallet::close_market(Origin::signed(ALICE), market_id, time),
				Error::<Runtime>::MarketIdNotFound
			);
		});
	}
}
