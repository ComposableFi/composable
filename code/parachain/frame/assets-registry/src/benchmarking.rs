//! Benchmarks and  sanity tests for lending. Only test that action do not error, not that produce
//! positive side effects

use super::*;
use crate::{self as pallet_assets_registry, prelude::*};

#[allow(unused_imports)]
use crate::Pallet as AssetsRegistry;

use composable_traits::{xcm::assets::XcmAssetLocation};
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
		let location = T::ForeignAssetId::decode(&mut &XcmAssetLocation::RELAY_NATIVE.encode()[..]).unwrap();
		let ed = 42_u64.into();
		let ratio = Rational::new(42, 123);
		let decimals = 3;

	}: _(RawOrigin::Root, location, ed, Some(ratio), Some(decimals))

	update_asset {
		let location = T::ForeignAssetId::decode(&mut &XcmAssetLocation::RELAY_NATIVE.encode()[..]).unwrap();
		let ed = 42_u64.into();
		let ratio = Rational::new(42, 123);
		let decimals = 3;

		AssetsRegistry::<T>::register_asset(RawOrigin::Root.into(), location.clone(), ed, Some(ratio), Some(decimals)).unwrap();

		let local_asset_id = AssetsRegistry::<T>::from_foreign_asset(location.clone()).unwrap();

	}: _(RawOrigin::Root, local_asset_id, location, Some(Rational::new(42, 123)), Some(3))

	set_min_fee {
		let target_parachain_id = 100_u32.into();
		let foreign_asset_id = Default::default();
		let balance = 100_500.into();

	}: _(RawOrigin::Root, target_parachain_id, foreign_asset_id, Some(balance))
}

impl_benchmark_test_suite!(AssetsRegistry, crate::runtime::new_test_ext(), crate::runtime::Runtime);
