use super::*;

#[allow(unused)]
use crate::Pallet as Oracle;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{
    assert_ok,
    traits::{Currency, EnsureOrigin, Get},
};
use frame_system::{EventRecord, RawOrigin};
use sp_runtime::{Percent, RuntimeAppPublic};
use sp_std::prelude::*;

pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    let events = frame_system::Pallet::<T>::events();
    let system_event: <T as frame_system::Config>::Event = generic_event.into();
    // compare to the last event record
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

macro_rules! whitelist {
	($acc:ident) => {
		frame_benchmarking::benchmarking::add_to_whitelist(
			frame_system::Account::<T>::hashed_key_for(&$acc).into()
		);
	};
}

benchmarks! {
    add_asset_and_info {
        let caller = T::AddOracle::successful_origin();
        let asset_id = 1;
        let threshold = Percent::from_percent(80);
        let min_answers = 3;
        let max_answers = 5;
    }: {
		assert_ok!(
			<Oracle<T>>::add_asset_and_info(caller, asset_id, threshold, min_answers, max_answers)
		);
	}
    verify {
        assert_last_event::<T>(Event::AssetInfoChange(asset_id, threshold, min_answers, max_answers).into());
    }
}

// benchmarks! {
//     set_signer {
//         let caller: T::AccountId = whitelisted_caller();

//     }: _(RawOrigin::Signed(caller), caller)
//     verify {
// 		assert_eq!(oracl, new_block);
//     }
// }

impl_benchmark_test_suite!(Oracle, crate::mock::new_test_ext(), crate::mock::Test,);
