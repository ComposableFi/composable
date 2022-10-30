///! tests that various assets integration scenarios work well
use crate::{helpers::*, kusama_test_net::This, prelude::*};
use composable_traits::{defi::Ratio, xcm::assets::XcmAssetLocation};

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
			Ratio::checked_from_integer::<u128>(10),
			None,
		)
		.unwrap();
		AssetsRegistry::update_asset(
			RawOrigin::Root.into(),
			CurrencyId(123),
			XcmAssetLocation(MultiLocation::new(1, X1(Parachain(4321)))),
			Ratio::checked_from_rational(10u32, 100u32),
			None,
		)
		.unwrap();
		assert_eq!(
			1000,
			<PriceConverter<AssetsRegistry>>::get_price_inverse(CurrencyId(42), 100).unwrap()
		);
		assert_eq!(
			10,
			<PriceConverter<AssetsRegistry>>::get_price_inverse(CurrencyId(123), 100).unwrap()
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
			42,
			Ratio::checked_from_integer::<u128>(10),
			None,
		)
		.unwrap();
		let asset_id = System::events()
			.iter()
			.find_map(|x| match x.event {
				Event::AssetsRegistry(assets_registry::Event::<Runtime>::AssetRegistered {
					asset_id,
					location: _,
					decimals: _,
				}) => Some(asset_id),
				_ => None,
			})
			.unwrap();
		assert_eq!(
			1000,
			<PriceConverter<AssetsRegistry>>::get_price_inverse(asset_id, 100).unwrap()
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
			42,
			Ratio::checked_from_rational(10u32, 100u32),
			None,
		)
		.unwrap();
		let asset_id = System::events()
			.iter()
			.find_map(|x| match x.event {
				Event::AssetsRegistry(assets_registry::Event::<Runtime>::AssetRegistered {
					asset_id,
					location: _,
					decimals: _,
				}) => Some(asset_id),
				_ => None,
			})
			.unwrap();
		assert_eq!(10, <PriceConverter<AssetsRegistry>>::get_price_inverse(asset_id, 100).unwrap());
	});
}
