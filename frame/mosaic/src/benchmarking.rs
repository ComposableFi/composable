use super::*;

use crate::Pallet as Mosaic;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::{EventRecord, RawOrigin};
use crate::mock::Origin;




benchmarks! {
    where_clause {
        where T::BlockNumber: From<u32>
    }

    set_relayer {
        let relayer = whitelisted_caller();
    }: _ (RawOrigin::Root, relayer)

    rotate_relayer {
        let relayer: T::AccountId = whitelisted_caller();
        // let  = whitelisted_caller();
        Mosaic::<T>::set_relayer(RawOrigin::Root.into(), relayer.clone());

        let new_relayer = account("new_relayer", 0, 0);
    }: _ (RawOrigin::Signed(relayer), new_relayer, 42.into())


}

impl_benchmark_test_suite!(Mosaic, crate::mock::new_test_ext(), crate::mock::Test,);
