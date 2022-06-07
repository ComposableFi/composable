use frame_support::{assert_ok, dispatch::DispatchResultWithPostInfo};
use frame_system::{Config, EventRecord};
use sp_runtime::{FixedPointNumber, FixedU128};

/// Default is percent
pub const DEFAULT_PRECISION: u128 = 1000;

/// Per mill
pub const DEFAULT_EPSILON: u128 = 1;

/// This function should be used in context of approximation.
/// It is extensively used in conjunction with proptest because of random input generation.
pub fn acceptable_computation_error(
	x: u128,
	y: u128,
	precision: u128,
	epsilon: u128,
) -> Result<(), FixedU128> {
	let delta = i128::abs(x as i128 - y as i128);
	if delta > 1 {
		let lower =
			FixedU128::saturating_from_rational(precision, precision.saturating_add(epsilon));
		let upper =
			FixedU128::saturating_from_rational(precision, precision.saturating_sub(epsilon));
		let q = FixedU128::checked_from_rational(x, y).expect("values too big; qed;");
		if lower <= q && q <= upper {
			Ok(())
		} else {
			Err(q)
		}
	} else {
		Ok(())
	}
}

pub fn default_acceptable_computation_error(x: u128, y: u128) -> Result<(), FixedU128> {
	acceptable_computation_error(x, y, DEFAULT_PRECISION, DEFAULT_EPSILON)
}

/// Asserts that the last event in the runtime is the expected event.
pub fn assert_last_event<Runtime: Config>(generic_event: <Runtime as Config>::Event) {
	let events = frame_system::Pallet::<Runtime>::events();
	let system_event: <Runtime as frame_system::Config>::Event = generic_event.into();
	// compare to the last event record
	let EventRecord { event, .. } = &events.last().expect("No events present!");
	assert_eq!(event, &system_event);
}

/// Asserts the event wasn't dispatched.
pub fn assert_no_event<Runtime: Config>(event: <Runtime as Config>::Event) {
	assert!(frame_system::Pallet::<Runtime>::events()
		.iter()
		.all(|record| record.event != event));
}

/// Asserts that the outcome of an extrinsic is `Ok`, and that the last event is the specified
/// event.
///
/// # Example
///
/// ```rust,ignore
/// assert_extrinsic_event::<Runtime>(
///     Pallet::extrinsic(..),
///     Event::Pallet(pallet::Event::<Runtime>::SomethingHappened {
///         ..
///     }),
/// );
pub fn assert_extrinsic_event<
	Runtime: Config,
	Event: Into<<Runtime as frame_system::Config>::Event>,
>(
	result: DispatchResultWithPostInfo,
	event: Event,
) {
	assert_ok!(result);
	assert_last_event::<Runtime>(event.into());
}
