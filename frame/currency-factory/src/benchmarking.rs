#![allow(clippy::disallowed_methods, clippy::panic)]

use super::*;

#[allow(unused_imports)]
use crate::Pallet as CurrencyRanges;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;

benchmarks! {
	add_range {
	}: _(RawOrigin::Root, 100000000000000)
}

impl_benchmark_test_suite!(CurrencyRanges, crate::mocks::new_test_ext(), crate::mocks::Test,);
