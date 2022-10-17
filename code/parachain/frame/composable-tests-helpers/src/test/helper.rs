#![allow(clippy::expect_fun_call)] // dumb lint for tests

use core::fmt::Debug;

use frame_support::{assert_ok, pallet_prelude::Member, Parameter};
use frame_system::{Config, EventRecord};
use sp_runtime::{DispatchError, FixedPointNumber, FixedU128};

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
	let system_event: <Runtime as frame_system::Config>::Event = generic_event;
	// compare to the last event record
	let EventRecord { event, .. } = &events.last().expect("No events present!");
	assert_eq!(event, &system_event);
}

/// Asserts that the last event in the runtime is the expected event.
///
/// Useful if not all of the information in the event needs to be checked:
///
/// ```rust,ignore
/// assert_last_event_with::<Runtime, _>(
///     Pallet::extrinsic(),
///     |event| matches!(
///         event,
///         pallet::Event::<Runtime>::SomethingHappened {
///             field,
///             ..
///         } if field == expected_field
///     ).then_some(())
/// )
/// ```
///
/// It is also possible to return a value from the provided function, for example to retrieve a
/// generated id for later use:
///
/// ```rust,ignore
/// assert_last_event_with::<Runtime, _>(
///     Pallet::extrinsic(),
///     |event| if let pallet::Event::<Runtime>::SomethingHappened {
///         field,
///         generated_id,
///     } = event {
///         assert!(field);
///         Some(generated_id)
///     } else {
///         None
///     },
/// )
/// ```
pub fn assert_last_event_with<Runtime, RuntimeEvent, PalletEvent, R>(
	f: impl FnOnce(PalletEvent) -> Option<R>,
) -> R
where
	Runtime: Config<Event = RuntimeEvent>,
	PalletEvent: sp_std::fmt::Debug + Clone,
	RuntimeEvent: TryInto<PalletEvent> + Parameter + Member + Debug + Clone,
	<RuntimeEvent as TryInto<PalletEvent>>::Error: sp_std::fmt::Debug,
{
	// compare to the last event record
	let EventRecord { event, .. } =
		frame_system::Pallet::<Runtime>::events().pop().expect("No events present!");

	match event.clone().try_into() {
		Ok(pallet_event) => match f(pallet_event.clone()) {
			Some(r) => r,
			None => panic!("expected event was not found; found {pallet_event:#?}"),
		},
		Err(_) => panic!(
			r#"last event was not from this pallet
found {event:#?}"#
		),
	}
}

/// Asserts the event wasn't dispatched.
pub fn assert_no_event<Runtime: Config>(event: <Runtime as Config>::Event) {
	assert!(
		frame_system::Pallet::<Runtime>::events()
			.iter()
			.all(|record| record.event != event),
		"Provided event was dispatched unexpectedly!\n\nEvent checked: {event:#?}"
	);
}

/// Asserts that the outcome of an extrinsic is `Ok`, and that the last event is the specified
/// event.
///
/// # Example
///
/// ```rust,ignore
/// assert_extrinsic_event::<Runtime>(
///     Pallet::extrinsic(),
///     pallet::Event::<Runtime>::SomethingHappened {
///         ..
///     },
/// );
pub fn assert_extrinsic_event<Runtime, RuntimeEvent, PalletEvent, T, E>(
	result: sp_std::result::Result<T, E>,
	event: PalletEvent,
) where
	Runtime: Config<Event = RuntimeEvent>,
	PalletEvent: sp_std::fmt::Debug + Clone,
	RuntimeEvent: Parameter + Member + Debug + Clone + From<PalletEvent>,
	RuntimeEvent: TryInto<PalletEvent>,
	<RuntimeEvent as TryInto<PalletEvent>>::Error: sp_std::fmt::Debug,
	T: Debug,
	E: Into<DispatchError> + Debug,
{
	assert_ok!(result);
	assert_last_event::<Runtime>(event.into());
}

/// Asserts that the outcome of an extrinsic is `Ok`, and that the last event is the specified
/// event.
///
/// # Example
///
/// ```rust,ignore
/// assert_extrinsic_event::<Runtime>(
///     Pallet::extrinsic(),
///     pallet::Event::<Runtime>::SomethingHappened {
///         ..
///     },
/// );
pub fn assert_extrinsic_event_with<Runtime, RuntimeEvent, PalletEvent, T, E, R>(
	result: sp_std::result::Result<T, E>,
	f: impl FnOnce(PalletEvent) -> Option<R>,
) -> R
where
	Runtime: Config<Event = RuntimeEvent>,
	PalletEvent: sp_std::fmt::Debug + Clone,
	RuntimeEvent: Parameter + Member + Debug + Clone,
	RuntimeEvent: TryInto<PalletEvent>,
	<RuntimeEvent as TryInto<PalletEvent>>::Error: sp_std::fmt::Debug,
	T: Debug,
	E: Into<DispatchError> + Debug,
{
	assert_ok!(result);
	assert_last_event_with::<Runtime, RuntimeEvent, PalletEvent, R>(f)
}

/// Iterates over all of the events currently in the runtime and calls the provided function on all
/// of the `PalletEvent` events, returning an iterator over the the returned values of all of the
/// found events.
pub fn assert_event_with<Runtime, RuntimeEvent, PalletEvent, R>(
	f: impl FnMut(PalletEvent) -> Option<R>,
) -> impl Iterator<Item = R>
where
	Runtime: Config<Event = RuntimeEvent>,
	RuntimeEvent: Parameter + Member + Debug + Clone,
	RuntimeEvent: TryInto<PalletEvent>,
	<RuntimeEvent as TryInto<PalletEvent>>::Error: sp_std::fmt::Debug,
{
	pallet_events::<Runtime, RuntimeEvent, PalletEvent>().flat_map(f)
}

pub fn pallet_events<Runtime, RuntimeEvent, PalletEvent>() -> impl Iterator<Item = PalletEvent>
where
	Runtime: Config<Event = RuntimeEvent>,
	RuntimeEvent: Parameter + Member + Debug + Clone,
	RuntimeEvent: TryInto<PalletEvent>,
	<RuntimeEvent as TryInto<PalletEvent>>::Error: sp_std::fmt::Debug,
{
	frame_system::Pallet::<Runtime>::events()
		.into_iter()
		.flat_map(|EventRecord { event, .. }| event.try_into().ok())
}

/// Iterates over all of the events currently in the runtime and calls the provided function on all
/// of the `PalletEvent` events, returning an iterator over the the returned values of all of the
/// found events.
pub fn assert_event<Runtime, RuntimeEvent, PalletEvent>(pallet_event: PalletEvent)
where
	Runtime: Config<Event = RuntimeEvent>,
	RuntimeEvent: Parameter + Member + Debug + Clone,
	RuntimeEvent: TryInto<PalletEvent>,
	<RuntimeEvent as TryInto<PalletEvent>>::Error: sp_std::fmt::Debug,
	PalletEvent: PartialEq + sp_std::fmt::Debug,
{
	match pallet_events::<Runtime, RuntimeEvent, PalletEvent>().find(|e| e == &pallet_event) {
		Some(_) => {},
		None => panic!(
			r#"
expected event wasn't emitted
event checked: {pallet_event:#?}
"#
		),
	}
}
