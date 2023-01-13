//! Benchmarks and  sanity tests for lending. Only test that action do not error, not that produce
//! positive side effects

use super::*;
use crate::{self as pallet_assets_registry};

#[allow(unused_imports)]
use crate::Pallet as AssetsRegistry;
use composable_traits::{currency::Rational64, rational, xcm::assets::XcmAssetLocation};
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
		let location = LocalOrForeignAssetId::Foreign(location);
		let ratio = rational!(42 / 123);
		let name = b"Kusama".to_vec();
		let symbol = b"KSM".to_vec();
		let decimals = 3;
	}: _(RawOrigin::Root, location, ratio, name, symbol, decimals)

	update_asset_location {
		let location_base = T::ForeignAssetId::decode(&mut &XcmAssetLocation::RELAY_NATIVE.encode()[..]).unwrap();
		let location = LocalOrForeignAssetId::Foreign(location_base.clone());
		let ratio = rational!(42 / 123);
		let name = b"Kusama".to_vec();
		let symbol = b"KSM".to_vec();
		let decimals = 3;

		AssetsRegistry::<T>::register_asset(RawOrigin::Root.into(), location, ratio, name, symbol, decimals).unwrap();

		let local_asset_id = AssetsRegistry::<T>::from_foreign_asset(location_base.clone()).unwrap();
	}: _(RawOrigin::Root, local_asset_id, location_base)

	update_asset_ratio {
		let location_base = T::ForeignAssetId::decode(&mut &XcmAssetLocation::RELAY_NATIVE.encode()[..]).unwrap();
		let location = LocalOrForeignAssetId::Foreign(location_base.clone());
		let ratio = rational!(42 / 123);
		let name = b"Kusama".to_vec();
		let symbol = b"KSM".to_vec();
		let decimals = 3;

		AssetsRegistry::<T>::register_asset(RawOrigin::Root.into(), location, ratio, name, symbol, decimals).unwrap();

		let local_asset_id = AssetsRegistry::<T>::from_foreign_asset(location_base).unwrap();
	}: _(RawOrigin::Root, local_asset_id, Some(rational!(420 / 8008)))

	update_asset_metadata {
		let location_base = T::ForeignAssetId::decode(&mut &XcmAssetLocation::RELAY_NATIVE.encode()[..]).unwrap();
		let location = LocalOrForeignAssetId::Foreign(location_base.clone());
		let ratio = rational!(42 / 123);
		let name = b"Kusama".to_vec();
		let symbol = b"KSM".to_vec();
		let decimals = 3;

		AssetsRegistry::<T>::register_asset(RawOrigin::Root.into(), location, ratio, name, symbol, decimals).unwrap();

		let local_asset_id = AssetsRegistry::<T>::from_foreign_asset(location_base).unwrap();
		let name = b"Cooler Kusama".to_vec();
		let symbol = b"CKSM".to_vec();
	}: _(RawOrigin::Root, local_asset_id, Some(name), Some(symbol), Some(18))

	set_min_fee {
		let target_parachain_id = 100_u32.into();
		let foreign_asset_id = Default::default();
		let balance = 100_500.into();

	}: _(RawOrigin::Root, target_parachain_id, foreign_asset_id, Some(balance))
}

impl_benchmark_test_suite!(AssetsRegistry, crate::runtime::new_test_ext(), crate::runtime::Runtime);
