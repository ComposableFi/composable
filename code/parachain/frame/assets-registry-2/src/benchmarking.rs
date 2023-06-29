//! Benchmarks and  sanity tests for lending. Only test that action do not error, not that produce
//! positive side effects

use super::{prelude::*, *};
use crate::{self as pallet_assets_registry};

#[allow(unused_imports)]
use crate::Pallet as AssetsRegistry;
use composable_traits::{
	assets::{AssetInfo, AssetInfoUpdate, BiBoundedAssetName, BiBoundedAssetSymbol},
	rational,
	storage::UpdateValue,
};

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use primitives::currency::{ForeignAssetId, VersionedMultiLocation};
use sp_runtime::traits::Zero;
use sp_std::prelude::*;
use xcm::{
	latest::MultiLocation,
	v3::{Junction::Parachain, Junctions::X1},
};

benchmarks! {
	where_clause {
		where
			T: pallet_assets_registry::Config + frame_system::Config,
			<T as pallet_assets_registry::Config>::Balance : From<u64>,
	}

	register_asset {
		let location = T::ForeignAssetId::decode(&mut ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::here())).encode().as_ref()).unwrap();
		let protocol_id = *b"benchmar";
		let nonce = 1_u64;
		let asset_info = AssetInfo {
			name: Some(BiBoundedAssetName::from_vec(b"Kusama".to_vec()).expect("String is within bounds")),
			symbol: Some(BiBoundedAssetSymbol::from_vec(b"KSM".to_vec()).expect("String is within bounds")),
			decimals: Some(3),
			existential_deposit: T::Balance::zero(),
			ratio: Some(rational!(42 / 123)),
		};
	}: _(RawOrigin::Root, protocol_id, nonce, Some(location), asset_info)

	update_asset {
		let location =T::ForeignAssetId::decode(&mut ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::here())).encode().as_ref()).unwrap();
		let protocol_id = *b"benchmar";
		let nonce = 1_u64;
		let asset_info = AssetInfo {
			name: Some(BiBoundedAssetName::from_vec(b"Kusama".to_vec()).expect("String is within bounds")),
			symbol: Some(BiBoundedAssetSymbol::from_vec(b"KSM".to_vec()).expect("String is within bounds")),
			decimals: Some(3),
			existential_deposit: T::Balance::zero(),
			ratio: Some(rational!(42 / 123)),
		};

		AssetsRegistry::<T>::register_asset(
			RawOrigin::Root.into(),
			protocol_id,
			nonce,
			Some(location.clone()),
			asset_info,
		)
		.expect("Asset details are non-duplicate and valid");

		let asset_info_update = AssetInfoUpdate {
			name: UpdateValue::Set(Some(BiBoundedAssetName::from_vec(b"Cooler Kusama".to_vec()).expect("String is within bounds"))),
			symbol: UpdateValue::Set(Some(BiBoundedAssetSymbol::from_vec(b"CKSM".to_vec()).expect("String is within bounds"))),
			decimals: UpdateValue::Set(Some(12)),
			existential_deposit: UpdateValue::Set(T::Balance::zero()),
			ratio: UpdateValue::Set(None),
		};

		let local_asset_id = AssetsRegistry::<T>::from_foreign_asset(location)
			.expect("Asset exists");
	}: _(RawOrigin::Root, local_asset_id, asset_info_update)

	set_min_fee {
		let target_parachain_id = 100_u32.into();
		let foreign_asset_id =T::ForeignAssetId::decode(&mut ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::here())).encode().as_ref()).unwrap();
		let balance = 100_500.into();

	}: _(RawOrigin::Root, target_parachain_id, foreign_asset_id, Some(balance))

	update_asset_location {
		let location =T::ForeignAssetId::decode(&mut ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::here())).encode().as_ref()).unwrap();
		let protocol_id = *b"benchmar";
		let nonce = 1_u64;
		let asset_info = AssetInfo {
			name: Some(BiBoundedAssetName::from_vec(b"Kusama".to_vec()).expect("String is within bounds")),
			symbol: Some(BiBoundedAssetSymbol::from_vec(b"KSM".to_vec()).expect("String is within bounds")),
			decimals: Some(3),
			existential_deposit: T::Balance::zero(),
			ratio: Some(rational!(42 / 123)),
		};

		AssetsRegistry::<T>::register_asset(
			RawOrigin::Root.into(),
			protocol_id,
			nonce,
			Some(location.clone()),
			asset_info,
		)
		.expect("Asset details are non-duplicate and valid");

		let local_asset_id = AssetsRegistry::<T>::from_foreign_asset(location.clone())
			.expect("Asset exists");
		let location_new =T::ForeignAssetId::decode(&mut ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::new(1, X1(Parachain(4321))))).encode().as_ref()).unwrap();
	}: _(RawOrigin::Root, local_asset_id, Some(location_new))
}

impl_benchmark_test_suite!(AssetsRegistry, crate::runtime::new_test_ext(), crate::runtime::Runtime);
