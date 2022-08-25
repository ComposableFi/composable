use crate::{
	mock::{ExtBuilder, MockRuntime, TestPallet},
	pallet::Error,
	tests::{constants::RUN_CASES, helpers::run_for_seconds, types::Timestamp},
	types::VammState,
};
use composable_traits::vamm::Vamm as VammTrait;
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;
use rstest::rstest;
use std::cmp::Ordering::Greater;

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
				assert_eq!(TestPallet::get_vamm(0).unwrap().closed, Some(try_close_at));
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
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn should_return_correct_result(
		current_time in Timestamp::MIN..Timestamp::MAX/1000,
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
			match vamm_close_at {
				Some(closed_at) => {
					match closed_at.cmp(&current_time) {
						Greater => {
							// The caller tried to close a vamm that is already
							// in the closing period, we should deny a new close
							// call with a message warning about the vamm being
							// already in the closing period.
							assert_noop!(TestPallet::close(0, try_close_at),
										 Error::<MockRuntime>::VammIsClosing);
						},
						_ => {
							// The caller tried to close a vamm that is already
							// closed, we deny this with an appropriate message.
							assert_noop!(TestPallet::close(0, try_close_at),
										 Error::<MockRuntime>::VammIsClosed);
						}
					}
				},
				None => {
					match try_close_at.cmp(&current_time) {
						Greater => {
							// To succeed closing a vamm, we need to satisfy the constraints:
							// * Vamm must exist (always true in this test)
							// * Vamm must be open
							// (this is true since `vamm_close_at = None`)
							// * Desired closing time must be in the future
							// (this is true since `try_close_at > vamm_close_at`)
							assert_ok!(TestPallet::close(0, try_close_at));
							assert_eq!(TestPallet::get_vamm(0).unwrap().closed,
									   Some(try_close_at));
						},
						_ => {
							// The caller tried to close a vamm with a time that
							// is not in the future, we should deny it.
							assert_noop!(TestPallet::close(0, try_close_at),
										 Error::<MockRuntime>::ClosingDateIsInThePast);
						}
					}
				},
			};
		})
	}
}
