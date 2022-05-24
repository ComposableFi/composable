#![cfg(feature = "runtime-benchmarks")]

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {}

impl_benchmark_test_suite! {
	YourPallet,
	crate::mocks::ExtBuilder::default().build(),
	crate::mocks::Test,
}
