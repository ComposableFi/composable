use crate::{prelude::*, runtime::*, Error};
use codec::{Decode, Encode};
use composable_traits::{
	assets::{Asset, AssetInfo, AssetInfoUpdate, GenerateAssetId},
	currency::Rational64,
	rational,
	storage::UpdateValue,
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
		let protocol_id = *b"AssTests";
		let nonce = 1_u64;
		let asset_info = AssetInfo {
			name: None,
			symbol: None,
			decimals: Some(4),
			existential_deposit: 0,
			ratio: Some(rational!(42 / 123)),
		};
		System::set_block_number(1);
		assert_ok!(AssetsRegistry::register_asset(
			RawOrigin::Root.into(),
			protocol_id,
			nonce,
			Some(XcmAssetLocation::RELAY_NATIVE),
			asset_info,
		));
		let asset_id = System::events()
			.iter()
			.find_map(|x| match x.event {
				RuntimeEvent::AssetsRegistry(crate::Event::<Runtime>::AssetRegistered {
					asset_id,
					location: _,
					asset_info: _,
				}) => Some(asset_id),
				_ => None,
			})
			.expect("Asset registration event emmited");
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
		.expect("Location bytes translate to foreign ID bytes");
		let protocol_id = *b"AssTests";
		let nonce = 1_u64;
		let asset_info = AssetInfo {
			name: None,
			symbol: None,
			decimals: Some(4),
			existential_deposit: 0,
			ratio: Some(rational!(42 / 123)),
		};

		assert_eq!(AssetsRegistry::from_foreign_asset(location.clone()), None);

		assert_ok!(AssetsRegistry::register_asset(
			RuntimeOrigin::root(),
			protocol_id,
			nonce,
			Some(location.clone()),
			asset_info.clone(),
		));
		let local_asset_id =
			AssetsRegistry::from_foreign_asset(location.clone()).expect("Asset exists");
		assert_eq!(AssetsRegistry::from_local_asset(local_asset_id), Some(location.clone()));

		assert_noop!(
			AssetsRegistry::register_asset(
				RuntimeOrigin::root(),
				protocol_id,
				nonce,
				Some(location),
				asset_info,
			),
			Error::<Runtime>::AssetAlreadyRegistered
		);
	})
}

#[test]
fn update_asset() {
	new_test_ext().execute_with(|| {
		let location = <Runtime as crate::Config>::ForeignAssetId::decode(
			&mut &XcmAssetLocation::RELAY_NATIVE.encode()[..],
		)
		.expect("Location bytes translate to foreign ID bytes");
		let protocol_id = *b"AssTests";
		let nonce = 1_u64;
		let asset_info = AssetInfo {
			name: None,
			symbol: None,
			decimals: Some(4),
			existential_deposit: 0,
			ratio: Some(rational!(42 / 123)),
		};

		assert_ok!(AssetsRegistry::register_asset(
			RuntimeOrigin::root(),
			protocol_id,
			nonce,
			Some(location.clone()),
			asset_info,
		));

		let local_asset_id =
			AssetsRegistry::from_foreign_asset(location.clone()).expect("Asset exists");
		assert_eq!(AssetsRegistry::from_local_asset(local_asset_id), Some(location.clone()));
		assert_eq!(AssetsRegistry::asset_ratio(local_asset_id), Some(rational!(42 / 123)));

		let new_decimals = 12;
		let new_ratio = rational!(100500 / 666);

		let asset_info_update = AssetInfoUpdate {
			name: UpdateValue::Ignore,
			symbol: UpdateValue::Ignore,
			decimals: UpdateValue::Set(Some(new_decimals)),
			ratio: UpdateValue::Set(Some(new_ratio)),
			existential_deposit: UpdateValue::Ignore,
		};

		assert_ok!(AssetsRegistry::update_asset(
			RuntimeOrigin::root(),
			local_asset_id,
			asset_info_update,
		));
		assert_eq!(AssetsRegistry::from_local_asset(local_asset_id), Some(location));
		assert_eq!(AssetsRegistry::asset_ratio(local_asset_id), Some(new_ratio));
	})
}

#[test]
fn update_asset_without_register() {
	new_test_ext().execute_with(|| {
		const NONEXISTING_ASSET_ID: u128 = 1000;
		assert_noop!(
			<AssetsRegistry as AssetExistentialDepositInspect>::existential_deposit(
				NONEXISTING_ASSET_ID
			),
			Error::<Runtime>::AssetNotFound
		);

		let new_decimals = 12;
		let new_ratio = rational!(100500 / 666);

		let asset_info_update = AssetInfoUpdate {
			name: UpdateValue::Ignore,
			symbol: UpdateValue::Ignore,
			decimals: UpdateValue::Set(Some(new_decimals)),
			ratio: UpdateValue::Set(Some(new_ratio)),
			existential_deposit: UpdateValue::Ignore,
		};

		assert_noop!(
			AssetsRegistry::update_asset(
				RuntimeOrigin::root(),
				NONEXISTING_ASSET_ID,
				asset_info_update,
			),
			Error::<Runtime>::AssetNotFound
		);
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
			RuntimeOrigin::root(),
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
		let location =
			XcmAssetLocation::new(MultiLocation { parents: 1, interior: Junctions::Here });
		let protocol_id = *b"AssTests";
		let nonce = 1_u64;
		let asset_info = AssetInfo {
			name: None,
			symbol: None,
			decimals: Some(4),
			existential_deposit: 0,
			ratio: Some(rational!(42 / 123)),
		};
		let id = AssetsRegistry::generate_asset_id(protocol_id, nonce);

		let foreign_assets = AssetsRegistry::get_foreign_assets_list();

		assert_eq!(foreign_assets, vec![]);

		assert_ok!(AssetsRegistry::register_asset(
			RuntimeOrigin::root(),
			protocol_id,
			nonce,
			Some(location),
			asset_info,
		));

		let foreign_assets = AssetsRegistry::get_foreign_assets_list();

		assert_eq!(
			foreign_assets,
			vec![Asset {
				name: None,
				id,
				decimals: 4,
				ratio: Some(rational!(42 / 123)),
				foreign_id: Some(XcmAssetLocation::new(MultiLocation {
					parents: 1,
					interior: Junctions::Here
				})),
				existential_deposit: 0,
			}]
		);
	})
}
