use super::*;

use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::Config;

benchmarks! {
	upload {
		let caller = whitelisted_caller();

	}: _(RawOrigin::Signed(caller), [].into())

}

impl_benchmark_test_suite!(Cosmwasm, crate::mocks::new_test_ext(), crate::mocks::Test,);
