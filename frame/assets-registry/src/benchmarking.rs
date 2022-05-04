//! Benchmarks and  sanity tests for lending. Only test that action do not error, not that produce
//! positive side effects

use super::*;
use crate::{self as pallet_assets_registry};
use codec::{Decode, Encode};
use composable_traits::{defi::Ratio, xcm::assets::XcmAssetLocation};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_std::prelude::*;

benchmarks! {
	where_clause {
		where
			T: pallet_assets_registry::Config + frame_system::Config,
			<T as pallet_assets_registry::Config>::Balance : From<u64>,
	}
	register_asset {
		let remote = T::ForeignAssetId::decode(&mut &XcmAssetLocation::RELAY_NATIVE.encode()[..]).unwrap();

	}: _(RawOrigin::Root,remote , 42_u64.into(), Some(Ratio::from_inner(123)), Some(3))
}

impl_benchmark_test_suite!(AssetsRegistry, crate::runtime::new_test_ext(), crate::runtime::Runtime);
