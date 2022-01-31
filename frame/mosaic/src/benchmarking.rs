use super::*;

use crate::Pallet as Mosaic;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::{EventRecord, RawOrigin};


benchmarks! {

    set_relayer {
        let relayer = whitelisted_caller();
    }: _ (RawOrigin::Root, relayer)
}

impl_benchmark_test_suite!(Mosaic, crate::mock::new_test_ext(), crate::mock::Test,);
