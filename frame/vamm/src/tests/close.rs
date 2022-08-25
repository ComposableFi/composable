use crate::{
	mock::{Balance, ExtBuilder, MockRuntime, System, TestPallet, VammId},
	pallet::{Error, Event, VammMap},
	tests::{
		constants::{DEFAULT_OUTPUT_ADDING_BASE, DEFAULT_OUTPUT_REMOVING_BASE, RUN_CASES},
		helpers::{
			create_vamm, run_for_seconds, run_to_block, swap_config,
			with_swap_context_checking_limit,
		},
		helpers_propcompose::{
			any_swap_config, any_vamm_state, balance_range_lower_half, balance_range_upper_half,
			multiple_swaps, then_and_now,
		},
		types::{TestSwapConfig, TestVammConfig, Timestamp},
	},
	types::VammState,
};
use composable_traits::vamm::{
	AssetType, Direction, SwapConfig, SwapOutput, Vamm as VammTrait, MINIMUM_TWAP_PERIOD,
};
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;
use rstest::rstest;
use sp_core::U256;
use sp_runtime::traits::Zero;

// -------------------------------------------------------------------------------------------------
//                                            Unit Tests
// -------------------------------------------------------------------------------------------------

#[test]
fn should_fail_if_vamm_does_not_exist() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(TestPallet::close(0, 0), Error::<MockRuntime>::VammDoesNotExist);
	});
}

#[rstest]
#[case(0, Some(0), 0, Err(Error::<MockRuntime>::VammIsClosed))]
#[case(1, Some(0), 0, Err(Error::<MockRuntime>::VammIsClosed))]
#[case(1, Some(1), 0, Err(Error::<MockRuntime>::VammIsClosed))]
#[case(1, Some(1), 1, Err(Error::<MockRuntime>::VammIsClosed))]
#[case(1, Some(2), 1, Err(Error::<MockRuntime>::VammIsClosing))]
#[case(1, Some(2), 2, Err(Error::<MockRuntime>::VammIsClosing))]
#[case(1, Some(3), 2, Err(Error::<MockRuntime>::VammIsClosing))]
#[case(0, None, 1, Ok(()))]
#[case(0, None, 2, Ok(()))]
#[case(2, None, 3, Ok(()))]
#[case(0, None, 0, Err(Error::<MockRuntime>::ClosingDateIsInThePast))]
#[case(1, None, 0, Err(Error::<MockRuntime>::ClosingDateIsInThePast))]
#[case(2, None, 0, Err(Error::<MockRuntime>::ClosingDateIsInThePast))]
#[case(2, None, 1, Err(Error::<MockRuntime>::ClosingDateIsInThePast))]
#[case(2, None, 2, Err(Error::<MockRuntime>::ClosingDateIsInThePast))]
// Cases caught by proptest
// TODO(Cardosaum): Enable this test.
// #[case(6419055347932622256, None, 18446744073709552,
// Err(Error::<MockRuntime>::ClosingDateIsInThePast))]
fn should_fail_if_vamm_is_not_open_otherwise_succeed(
	#[case] current_time: Timestamp,
	#[case] vamm_close_at: Option<Timestamp>,
	#[case] try_close_at: Timestamp,
	#[case] error: Result<(), Error<MockRuntime>>,
) {
	ExtBuilder {
		vamm_count: 1,
		vamms: vec![(0, VammState { closed: vamm_close_at, ..Default::default() })],
	}
	.build()
	.execute_with(|| {
		run_for_seconds(current_time);
		match error {
			Ok(_) => {
				assert_ok!(TestPallet::close(0, try_close_at));
			},
			Err(e) => {
				assert_noop!(TestPallet::close(0, try_close_at), e);
			},
		};
	});
}

// -------------------------------------------------------------------------------------------------
//                                             Proptests
// -------------------------------------------------------------------------------------------------

proptest! {
	#![proptest_config(ProptestConfig::with_cases(u32::MAX))]
	#[test]
	#[ignore = "The `run_for_seconds` function has a cap, it seems it only goes until `u32::MAX`\
				but it should support `u64::MAX`."]
	fn should_return_correct_result(
		current_time in any::<Timestamp>(),
		vamm_close_at in any::<Option<Timestamp>>(),
		try_close_at in any::<Timestamp>(),
	) {
		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, VammState { closed: vamm_close_at, ..Default::default() })],
		}
		.build()
		.execute_with(|| {
			run_for_seconds(current_time);
			// To succeed closing a vamm, we need to satisfy the constraints:
			// * Vamm must exist (always true in this test)
			// * Vamm must be open
			// * Desired closing time must be in the future
			if current_time < try_close_at && vamm_close_at.is_none() {
				assert_ok!(TestPallet::close(0, try_close_at));
			} else if current_time >= try_close_at && vamm_close_at.is_none() {
				// If we try to close a open vamm with a time that is less than
				// or equal to the  current time, we should deny it.
				assert_noop!(TestPallet::close(0, try_close_at), Error::<MockRuntime>::ClosingDateIsInThePast);
			} else if current_time < try_close_at && current_time < vamm_close_at.unwrap() {
				// If we try to close a vamm in the future, but it's already
				// scheduled to close in the future, we deny it with a message
				// warning about the vamm being in the closing period.
				assert_noop!(TestPallet::close(0, try_close_at), Error::<MockRuntime>::VammIsClosing);
			} else if current_time < try_close_at && current_time >= vamm_close_at.unwrap() {
				// If we try to close a vamm but it's already closed, we deny
				// the operation with an appropriate message.
				assert_noop!(TestPallet::close(0, try_close_at), Error::<MockRuntime>::VammIsClosed);
			} else {
				dbg!(current_time, vamm_close_at, try_close_at);
				panic!("We should never find a case like this.");
			}
		})
	}
}
