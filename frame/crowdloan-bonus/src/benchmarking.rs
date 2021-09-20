//! Benchmarking setup for pallet-crowdloan-bonus

use super::*;

#[allow(unused)]
use crate::Pallet as LiquidCrowdloan;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{
	assert_ok, traits::{Currency, EnsureOrigin},
};
use frame_system::{EventRecord, RawOrigin};

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::Event = generic_event.into();
	// compare to the last event record
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

benchmarks! {
	make_claimable {
		let caller = T::JumpStart::successful_origin();

	}:  {
		assert_ok!(
			<LiquidCrowdloan<T>>::make_claimable(caller)
		);
	}
	verify {
		assert!(<LiquidCrowdloan<T>>::is_claimable() == Some(true));
	}
	claim {
		let successful = T::JumpStart::successful_origin();
		let caller: T::AccountId = whitelisted_caller();
		let pot_address = <LiquidCrowdloan<T>>::account_id();
		// let _ = <LiquidCrowdloan<T>>::initiate(successful.clone(), caller.clone(), 200u32.into());
		let _ = <LiquidCrowdloan<T>>::make_claimable(successful);
		T::NativeCurrency::make_free_balance_be(&pot_address, T::NativeCurrency::minimum_balance() * 2u32.into());

	}: _(RawOrigin::Signed(caller.clone()), 100)
	verify {
		assert_last_event::<T>(Event::Claimed(caller, 100u32.into()).into());
	}
}

impl_benchmark_test_suite!(LiquidCrowdloan, crate::mock::new_test_ext(), crate::mock::Test);
