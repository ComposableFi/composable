#![allow(clippy::expect_fun_call)] // dumb lint for tests

use core::{clone::Clone, cmp::PartialEq, fmt::Debug, iter::FlatMap};

use frame_support::{assert_ok, pallet_prelude::Member, traits::OriginTrait, Parameter};
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

type EventRecordOf<T> = EventRecord<<T as Config>::RuntimeEvent, <T as Config>::Hash>;

// NOTE/FIXME(benluelo): These trait bounds can be simplified quite a bit once this issue is
// resolved: https://github.com/rust-lang/rust/issues/20671#issuecomment-529752828
pub trait RuntimeTrait<PalletEvent>:
	Config<
	RuntimeEvent = <Self as RuntimeTrait<PalletEvent>>::Event,
	RuntimeOrigin = <Self as RuntimeTrait<PalletEvent>>::Origin,
>
where
	PalletEvent: Clone + Debug + PartialEq,
{
	type Event: Parameter
		+ Member
		+ Debug
		+ Clone
		+ TryInto<PalletEvent, Error = Self::EventTryIntoPalletEventError>
		+ From<PalletEvent>;
	type EventTryIntoPalletEventError: Debug;
	type Origin: OriginTrait<AccountId = <Self as Config>::AccountId>;

	/// Asserts that the last event in the runtime is the expected event.
	fn assert_last_event(pallet_event: PalletEvent) {
		let events = frame_system::Pallet::<Self>::events();
		// compare to the last event record
		let EventRecord { event, .. } = &events.last().expect("No events present!");
		assert_eq!(event, &pallet_event.into());
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
	fn assert_last_event_with<R>(f: impl FnOnce(PalletEvent) -> Option<R>) -> R {
		// compare to the last event record
		let EventRecord { event, .. } =
			frame_system::Pallet::<Self>::events().pop().expect("No events present!");

		match event.clone().try_into() {
			Ok(pallet_event) => match f(pallet_event.clone()) {
				Some(r) => r,
				None => panic!("expected event was not found; found {pallet_event:#?}"),
			},
			Err(_) => panic!(
				r#"
last event was not from this pallet
found {event:#?}"#
			),
		}
	}

	/// Asserts the event wasn't dispatched.
	fn assert_no_event(event: PalletEvent) {
		assert!(
			Self::pallet_events().all(|e| e != event),
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
	fn assert_extrinsic_event<T, E>(result: sp_std::result::Result<T, E>, event: PalletEvent)
	where
		T: Debug,
		E: Into<DispatchError> + Debug,
	{
		assert_ok!(result);

		let events = frame_system::Pallet::<Self>::events();
		let event_record = events.last().expect("No events present!");

		assert_eq!(event_record.event, event.into());
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
	fn assert_extrinsic_event_with<T, E, R>(
		result: sp_std::result::Result<T, E>,
		f: impl FnOnce(PalletEvent) -> Option<R>,
	) -> R
	where
		T: Debug,
		E: Into<DispatchError> + Debug,
	{
		assert_ok!(result);
		Self::assert_last_event_with::<R>(f)
	}

	/// Iterates over all of the events currently in the runtime and calls the provided function on
	/// all of the `PalletEvent` events, returning an iterator over the the returned values of all
	/// of the found events.
	fn assert_event(pallet_event: PalletEvent) {
		match Self::pallet_events().find(|e| e == &pallet_event) {
			Some(_) => {},
			None => panic!(
				r#"
expected event wasn't emitted
event checked: {pallet_event:#?}
"#
			),
		}
	}

	/// Iterates over all of the events currently in the runtime and calls the provided function on
	/// all of the `PalletEvent` events, returning an iterator over the the returned values of all
	/// of the found events.
	fn assert_event_with<R, F: FnMut(PalletEvent) -> Option<R>>(
		f: F,
	) -> FlatMap<
		FlatMap<
			sp_std::vec::IntoIter<EventRecordOf<Self>>,
			Option<PalletEvent>,
			fn(EventRecordOf<Self>) -> Option<PalletEvent>,
		>,
		Option<R>,
		F,
	> {
		Self::pallet_events().flat_map(f)
	}

	fn pallet_events() -> FlatMap<
		sp_std::vec::IntoIter<EventRecordOf<Self>>,
		Option<PalletEvent>,
		fn(EventRecordOf<Self>) -> Option<PalletEvent>,
	> {
		frame_system::Pallet::<Self>::events()
			.into_iter()
			.flat_map(|EventRecord { event, .. }| event.try_into().ok())
	}
}

impl<Runtime, PalletEvent> RuntimeTrait<PalletEvent> for Runtime
where
	Runtime: Config,
	<Runtime as Config>::RuntimeEvent:
		Parameter + Member + Debug + Clone + TryInto<PalletEvent> + From<PalletEvent>,
	<<Runtime as Config>::RuntimeEvent as TryInto<PalletEvent>>::Error: Debug,
	<Runtime as Config>::RuntimeOrigin: OriginTrait<AccountId = <Runtime as Config>::AccountId>,
	PalletEvent: Clone + Debug + PartialEq,
{
	type Event = <Runtime as Config>::RuntimeEvent;
	type EventTryIntoPalletEventError =
		<<Runtime as Config>::RuntimeEvent as TryInto<PalletEvent>>::Error;
	type Origin = <Runtime as Config>::RuntimeOrigin;
}
