//! Benchmarks and  sanity tests for lending. Only test that action do not error, not that produce
//! positive side effects

use super::*;
use crate::{self as pallet_assets_registry};

#[allow(unused_imports)]
use crate::Pallet as AssetsRegistry;
use codec::{Decode, Encode};
use composable_traits::{
	assets::{
		AssetInfo, AssetInfoUpdate, BiBoundedAssetName, BiBoundedAssetSymbol, LocalOrForeignAssetId,
	},
	currency::Rational64,
	rational,
	xcm::assets::XcmAssetLocation,
};
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
		let location = T::ForeignAssetId::decode(&mut &XcmAssetLocation::RELAY_NATIVE.encode()[..])
			.expect("Asset location is foreign ID");
		let location = LocalOrForeignAssetId::Foreign(location);
		let asset_info = AssetInfo {
			name: Some(BiBoundedAssetName::from_vec(b"Kusama".to_vec()).expect("String is within bounds")),
			symbol: Some(BiBoundedAssetSymbol::from_vec(b"KSM".to_vec()).expect("String is within bounds")),
			decimals: 3,
			existential_deposit: T::Balance::from(0),
			ratio: Some(rational!(42 / 123)),
		};
	}: _(RawOrigin::Root, location, asset_info)

	update_asset_location {
		let location_base = T::ForeignAssetId::decode(&mut &XcmAssetLocation::RELAY_NATIVE.encode()[..])
			.expect("Asset location is foreign ID");
		let location = LocalOrForeignAssetId::Foreign(location_base.clone());
		let asset_info = AssetInfo {
			name: Some(BiBoundedAssetName::from_vec(b"Kusama".to_vec()).expect("String is within bounds")),
			symbol: Some(BiBoundedAssetSymbol::from_vec(b"KSM".to_vec()).expect("String is within bounds")),
			decimals: 3,
			existential_deposit: T::Balance::from(0),
			ratio: Some(rational!(42 / 123)),
		};

		AssetsRegistry::<T>::register_asset(
			RawOrigin::Root.into(),
			location,
			asset_info,
		)
		.expect("Asset details are non-duplicate and valid");

		let local_asset_id = AssetsRegistry::<T>::from_foreign_asset(location_base.clone())
			.expect("Asset exists");
	}: _(RawOrigin::Root, local_asset_id, location_base)

	update_asset {
		let location_base = T::ForeignAssetId::decode(&mut &XcmAssetLocation::RELAY_NATIVE.encode()[..])
			.expect("Asset location is foreign ID");
		let location = LocalOrForeignAssetId::Foreign(location_base.clone());
		let asset_info = AssetInfo {
			name: Some(BiBoundedAssetName::from_vec(b"Kusama".to_vec()).expect("String is within bounds")),
			symbol: Some(BiBoundedAssetSymbol::from_vec(b"KSM".to_vec()).expect("String is within bounds")),
			decimals: 3,
			existential_deposit: T::Balance::from(0),
			ratio: Some(rational!(42 / 123)),
		};

		AssetsRegistry::<T>::register_asset(
			RawOrigin::Root.into(),
			location,
			asset_info,
		)
		.expect("Asset details are non-duplicate and valid");

		let asset_info_update = AssetInfoUpdate {
			name: Some(Some(BiBoundedAssetName::from_vec(b"Cooler Kusama".to_vec()).expect("String is within bounds"))),
			symbol: Some(Some(BiBoundedAssetSymbol::from_vec(b"CKSM".to_vec()).expect("String is within bounds"))),
			decimals: Some(12),
			existential_deposit: Some(T::Balance::from(0)),
			ratio: Some(None),
		};

		let local_asset_id = AssetsRegistry::<T>::from_foreign_asset(location_base)
			.expect("Asset exists");
	}: _(RawOrigin::Root, local_asset_id, asset_info_update)

	set_min_fee {
		let target_parachain_id = 100_u32.into();
		let foreign_asset_id = Default::default();
		let balance = 100_500.into();

	}: _(RawOrigin::Root, target_parachain_id, foreign_asset_id, Some(balance))
}

impl_benchmark_test_suite!(AssetsRegistry, crate::runtime::new_test_ext(), crate::runtime::Runtime);
