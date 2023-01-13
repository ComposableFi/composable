use crate::{prelude::*, runtime::*, Error};
use codec::{Decode, Encode};
use composable_traits::{
	assets::Asset,
	currency::Rational64,
	rational,
	xcm::assets::{RemoteAssetRegistryInspect, XcmAssetLocation},
};
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use xcm::{latest::Junctions, v2::MultiLocation};

#[test]
fn negative_get_metadata() {
	new_test_ext().execute_with(|| {
		assert_eq!(<AssetsRegistry as RemoteAssetRegistryInspect>::asset_to_remote(42), None,);
		assert_eq!(<AssetsRegistry as AssetRatioInspect>::get_ratio(42), None,);
	});
}

#[test]
fn set_metadata() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(AssetsRegistry::register_asset(
			RawOrigin::Root.into(),
			crate::LocalOrForeignAssetId::Foreign(XcmAssetLocation::RELAY_NATIVE),
			rational!(42 / 123),
			b"Kusama".to_vec(),
			b"KSM".to_vec(),
			4
		));
		let asset_id = System::events()
			.iter()
			.find_map(|x| match x.event {
				Event::AssetsRegistry(crate::Event::<Runtime>::AssetRegistered {
					asset_id,
					location: _,
					name: _,
					symbol: _,
					ratio: _,
					decimals: _,
				}) => Some(asset_id),
				_ => None,
			})
			.unwrap();
		assert_eq!(
			<AssetsRegistry as RemoteAssetRegistryInspect>::asset_to_remote(asset_id),
			Some(XcmAssetLocation::RELAY_NATIVE)
		);

		assert_eq!(
			<AssetsRegistry as AssetRatioInspect>::get_ratio(asset_id),
			Some(rational!(42 / 123)),
		);

		assert_eq!(
			<AssetsRegistry as RemoteAssetRegistryInspect>::location_to_asset(
				XcmAssetLocation::RELAY_NATIVE
			),
			Some(asset_id),
		);
	})
}

#[test]
fn register_asset() {
	new_test_ext().execute_with(|| {
		let location = <Runtime as crate::Config>::ForeignAssetId::decode(
			&mut &XcmAssetLocation::RELAY_NATIVE.encode()[..],
		)
		.unwrap();
		let ratio = rational!(42 / 123);
		let decimals = 3;

		assert_eq!(AssetsRegistry::from_foreign_asset(location.clone()), None);

		assert_ok!(AssetsRegistry::register_asset(
			Origin::root(),
			crate::LocalOrForeignAssetId::Foreign(location.clone()),
			ratio,
			b"Kusama".to_vec(),
			b"KSM".to_vec(),
			decimals
		));
		let local_asset_id = AssetsRegistry::from_foreign_asset(location.clone()).unwrap();
		assert_eq!(AssetsRegistry::from_local_asset(local_asset_id), Some(location.clone()));

		assert_noop!(
			AssetsRegistry::register_asset(
				Origin::root(),
				crate::LocalOrForeignAssetId::Foreign(location),
				ratio,
				b"Kusama".to_vec(),
				b"KSM".to_vec(),
				decimals
			),
			Error::<Runtime>::ForeignAssetAlreadyRegistered
		);
	})
}

// TODO(connor): Test various update methods
#[test]
#[ignore = "Must test other update methods"]
fn update_asset() {
	new_test_ext().execute_with(|| {
		let location = <Runtime as crate::Config>::ForeignAssetId::decode(
			&mut &XcmAssetLocation::RELAY_NATIVE.encode()[..],
		)
		.unwrap();
		let ratio = rational!(42 / 123);
		let decimals = 3;

		assert_ok!(AssetsRegistry::register_asset(
			Origin::root(),
			crate::LocalOrForeignAssetId::Foreign(location.clone()),
			ratio,
			b"Kusama".to_vec(),
			b"KSM".to_vec(),
			decimals
		));

		let local_asset_id = AssetsRegistry::from_foreign_asset(location.clone()).unwrap();
		assert_eq!(AssetsRegistry::from_local_asset(local_asset_id), Some(location.clone()));
		assert_eq!(AssetsRegistry::asset_ratio(local_asset_id), Some(ratio));

		let new_decimals = 12;
		let new_ratio = rational!(100500 / 666);
		// assert_ok!(AssetsRegistry::update_asset(
		// 	Origin::root(),
		// 	local_asset_id,
		// 	location.clone(),
		// 	new_ratio,
		// 	Some(new_decimals)
		// ));
		assert_eq!(AssetsRegistry::from_local_asset(local_asset_id), Some(location));
		assert_eq!(AssetsRegistry::asset_ratio(local_asset_id), Some(new_ratio));
	})
}

#[test]
fn set_min_fee() {
	new_test_ext().execute_with(|| {
		let target_parachain_id = 100_u32.into();
		let foreign_asset_id: XcmAssetLocation = Default::default();
		let balance = 100_500_u32.into();

		assert_eq!(
			AssetsRegistry::minimal_amount(target_parachain_id, foreign_asset_id.clone()),
			None
		);

		assert_ok!(AssetsRegistry::set_min_fee(
			Origin::root(),
			target_parachain_id,
			foreign_asset_id.clone(),
			Some(balance)
		));

		assert_eq!(
			AssetsRegistry::minimal_amount(target_parachain_id, foreign_asset_id),
			Some(balance)
		);
	})
}

#[test]
fn get_foreign_assets_list_should_work() {
	new_test_ext().execute_with(|| {
		let location = <Runtime as crate::Config>::ForeignAssetId::decode(
			&mut &XcmAssetLocation::RELAY_NATIVE.encode()[..],
		)
		.unwrap();
		let ratio = rational!(42 / 123);
		let decimals = 3;

		let foreign_assets = AssetsRegistry::get_foreign_assets_list();

		assert_eq!(foreign_assets, vec![]);

		assert_ok!(AssetsRegistry::register_asset(
			Origin::root(),
			crate::LocalOrForeignAssetId::Foreign(location),
			ratio,
			b"Kusama".to_vec(),
			b"KSM".to_vec(),
			decimals
		));

		let foreign_assets = AssetsRegistry::get_foreign_assets_list();

		assert_eq!(
			foreign_assets,
			vec![Asset {
				name: None,
				id: 12884901886,
				decimals: 3,
				ratio: Some(ratio),
				foreign_id: Some(XcmAssetLocation::new(MultiLocation {
					parents: 1,
					interior: Junctions::Here
				})),
				existential_deposit: 0,
			}]
		);
	})
}
