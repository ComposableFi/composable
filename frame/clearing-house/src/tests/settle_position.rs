use crate::{
	mock::{
		accounts::ALICE,
		runtime::{ExtBuilder, Origin, Runtime, TestPallet},
	},
	tests::run_to_block,
	Error,
};
use frame_support::{assert_noop, assert_ok};

use super::with_market_context;

// ----------------------------------------------------------------------------------------------------
//                                             Unit Tests
// ----------------------------------------------------------------------------------------------------

#[test]
fn should_fail_if_market_does_not_exist() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			TestPallet::settle_position(Origin::signed(ALICE), 0),
			Error::<Runtime>::MarketIdNotFound
		);
	});
}

#[test]
fn should_fail_if_user_has_no_position_in_market() {
	with_market_context(Default::default(), Default::default(), |market_id| {
		// Market is set to close 10 seconds from now.
		assert_ok!(TestPallet::close_market(Origin::root(), market_id, 10));

		run_to_block(10);
		assert_noop!(
			TestPallet::settle_position(Origin::signed(ALICE), market_id),
			Error::<Runtime>::PositionNotFound
		);
	});
}
