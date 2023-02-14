///! tests that various assets integration scenarios work well
use crate::{helpers::*, kusama_test_net::This, prelude::*};
use composable_traits::xcm::assets::XcmAssetLocation;

use frame_system::RawOrigin;
use primitives::currency::*;
use xcm_emulator::TestExt;

#[test]
fn updated_assets_registry_works_well_for_ratios() {
	simtest();
	This::execute_with(|| {
		use this_runtime::*;
		AssetsRegistry::update_asset(
			RawOrigin::Root.into(),
			CurrencyId(42),
			XcmAssetLocation(MultiLocation::new(1, X1(Parachain(666)))),
			Rational64::from(10, 1),
			None,
		)
		.unwrap();
		AssetsRegistry::update_asset(
			RawOrigin::Root.into(),
			CurrencyId(123),
			XcmAssetLocation(MultiLocation::new(1, X1(Parachain(4321)))),
			Rational64::from(10, 100),
			None,
		)
		.unwrap();
		assert_eq!(
			1000,
			<PriceConverter<AssetsRegistry, this_runtime::WellKnownForeignToNativePriceConverter>>::to_asset_balance(100, CurrencyId(42)).unwrap()
		);
		assert_eq!(
			10,
			<PriceConverter<AssetsRegistry, this_runtime::WellKnownForeignToNativePriceConverter>>::to_asset_balance(100, CurrencyId(123)).unwrap()
		);
	});
}

#[test]
fn registered_assets_with_smaller_than_native_price() {
	simtest();
	This::execute_with(|| {
		use this_runtime::*;
		AssetsRegistry::register_asset(
			RawOrigin::Root.into(),
			XcmAssetLocation(MultiLocation::new(1, X1(Parachain(666)))),
			Rational64::from(10, 1),
			None,
		)
		.unwrap();
		let asset_id = System::events()
			.iter()
			.find_map(|x| match x.event {
				RuntimeEvent::AssetsRegistry(
					assets_registry::Event::<Runtime>::AssetRegistered {
						asset_id,
						location: _,
						decimals: _,
					},
				) => Some(asset_id),
				_ => None,
			})
			.unwrap();
		assert_eq!(
			1000,
			<PriceConverter<AssetsRegistry, this_runtime::WellKnownForeignToNativePriceConverter>>::to_asset_balance(100, asset_id).unwrap()
		);
	});
}

#[test]
fn registered_assets_with_larger_than_native_price() {
	simtest();
	This::execute_with(|| {
		use this_runtime::*;
		AssetsRegistry::register_asset(
			RawOrigin::Root.into(),
			XcmAssetLocation(MultiLocation::new(1, X1(Parachain(666)))),
			Rational64::from(10, 100),
			None,
		)
		.unwrap();
		let asset_id = System::events()
			.iter()
			.find_map(|x| match x.event {
				RuntimeEvent::AssetsRegistry(
					assets_registry::Event::<Runtime>::AssetRegistered {
						asset_id,
						location: _,
						decimals: _,
					},
				) => Some(asset_id),
				_ => None,
			})
			.unwrap();
		assert_eq!(10, <PriceConverter<AssetsRegistry,this_runtime::WellKnownForeignToNativePriceConverter>>::to_asset_balance(100, asset_id).unwrap());
	});
}
