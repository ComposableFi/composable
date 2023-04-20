use crate::{prelude::*, runtime::*, Error};
use codec::{Decode, Encode};
use composable_support::collections::vec::bounded::BiBoundedVec;
use composable_traits::{
	assets::{Asset, AssetInfo, AssetInfoUpdate, GenerateAssetId},
	rational,
	storage::UpdateValue,
	xcm::assets::RemoteAssetRegistryInspect,
};
use frame_support::{assert_noop, assert_ok, error::BadOrigin};
use frame_system::RawOrigin;
use primitives::currency::{ForeignAssetId, VersionedMultiLocation};
use xcm::latest::MultiLocation;

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
			Some(ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::parent()))),
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
			Some(ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::parent())))
		);

		assert_eq!(
			<AssetsRegistry as AssetRatioInspect>::get_ratio(asset_id),
			Some(rational!(42 / 123)),
		);

		assert_eq!(
			<AssetsRegistry as RemoteAssetRegistryInspect>::location_to_asset(ForeignAssetId::Xcm(
				VersionedMultiLocation::V3(MultiLocation::parent())
			)),
			Some(asset_id),
		);
	})
}

#[test]
fn register_asset() {
	new_test_ext().execute_with(|| {
		let location = <Runtime as crate::Config>::ForeignAssetId::decode(
			&mut &ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::parent())).encode()
				[..],
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
			&mut &ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::parent())).encode()
				[..],
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
		let foreign_asset_id =
			ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::here()));
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
fn update_asset_location() {
	new_test_ext().execute_with(|| {
		let location = <Runtime as crate::Config>::ForeignAssetId::decode(
			&mut &ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::parent())).encode()
				[..],
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

		// updating initital location
		let location_new = <Runtime as crate::Config>::ForeignAssetId::decode(
			&mut &ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::here())).encode()[..],
		)
		.expect("Location bytes translate to foreign ID bytes");
		assert_noop!(
			AssetsRegistry::update_asset_location(
				RuntimeOrigin::signed(ALICE),
				local_asset_id,
				Some(location_new.clone()),
			),
			BadOrigin
		);
		let does_not_exist_asset_id: <Runtime as crate::Config>::LocalAssetId = 0;
		assert_noop!(
			AssetsRegistry::update_asset_location(
				RuntimeOrigin::root(),
				does_not_exist_asset_id,
				Some(location_new.clone()),
			),
			Error::<Runtime>::AssetNotFound
		);
		assert_ok!(AssetsRegistry::update_asset_location(
			RuntimeOrigin::root(),
			local_asset_id,
			Some(location_new.clone()),
		));
		assert_eq!(AssetsRegistry::from_local_asset(local_asset_id), Some(location_new.clone()));
		assert_eq!(AssetsRegistry::from_foreign_asset(location_new.clone()), Some(local_asset_id));
		assert_eq!(AssetsRegistry::from_foreign_asset(location.clone()), None);

		// remove location
		assert_ok!(AssetsRegistry::update_asset_location(
			RuntimeOrigin::root(),
			local_asset_id,
			None
		));
		assert_eq!(AssetsRegistry::from_local_asset(local_asset_id), None);
		assert_eq!(AssetsRegistry::from_foreign_asset(location_new.clone()), None);
		assert_eq!(AssetsRegistry::from_foreign_asset(location.clone()), None);

		// add location to the asset without location
		assert_ok!(AssetsRegistry::update_asset_location(
			RuntimeOrigin::root(),
			local_asset_id,
			Some(location.clone()),
		));
		assert_eq!(AssetsRegistry::from_local_asset(local_asset_id), Some(location.clone()));
		assert_eq!(AssetsRegistry::from_foreign_asset(location_new.clone()), None);
		assert_eq!(AssetsRegistry::from_foreign_asset(location.clone()), Some(local_asset_id));
	})
}

#[test]
fn use_same_location_twice() {
	new_test_ext().execute_with(|| {
		let location_1 = <Runtime as crate::Config>::ForeignAssetId::decode(
			&mut &ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::parent())).encode()
				[..],
		)
		.expect("Location bytes translate to foreign ID bytes");
		let location_2 = <Runtime as crate::Config>::ForeignAssetId::decode(
			&mut &ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::here())).encode()[..],
		)
		.expect("Location bytes translate to foreign ID bytes");
		let protocol_id = *b"AssTests";
		let nonce_1 = 1_u64;
		let nonce_2 = 2_u64;
		let nonce_3 = 3_u64;
		let nonce_4 = 4_u64;
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
			nonce_1,
			Some(location_1.clone()),
			asset_info.clone(),
		));

		// shouldn't register because location is used
		assert_noop!(
			AssetsRegistry::register_asset(
				RuntimeOrigin::root(),
				protocol_id,
				nonce_2,
				Some(location_1.clone()),
				asset_info.clone(),
			),
			Error::<Runtime>::LocationIsUsed
		);
		assert_ok!(AssetsRegistry::register_asset(
			RuntimeOrigin::root(),
			protocol_id,
			nonce_2,
			Some(location_2.clone()),
			asset_info.clone(),
		));

		let local_asset_id_1 =
			AssetsRegistry::from_foreign_asset(location_1.clone()).expect("Asset exists");
		let local_asset_id_2 =
			AssetsRegistry::from_foreign_asset(location_2.clone()).expect("Asset exists");
		assert_eq!(AssetsRegistry::generate_asset_id(protocol_id, nonce_1), local_asset_id_1);
		assert_eq!(AssetsRegistry::generate_asset_id(protocol_id, nonce_2), local_asset_id_2);

		// should fail because location_2 is used
		assert_noop!(
			AssetsRegistry::update_asset_location(
				RuntimeOrigin::root(),
				local_asset_id_1,
				Some(location_2.clone()),
			),
			Error::<Runtime>::LocationIsUsed
		);

		// should pass with location None
		assert_ok!(AssetsRegistry::register_asset(
			RuntimeOrigin::root(),
			protocol_id,
			nonce_3,
			None,
			asset_info.clone(),
		));
		assert_ok!(AssetsRegistry::register_asset(
			RuntimeOrigin::root(),
			protocol_id,
			nonce_4,
			None,
			asset_info.clone(),
		));
		assert_ok!(AssetsRegistry::update_asset_location(
			RuntimeOrigin::root(),
			local_asset_id_1,
			None
		));

		let local_asset_id_3 = AssetsRegistry::generate_asset_id(protocol_id, nonce_3);
		let local_asset_id_4 = AssetsRegistry::generate_asset_id(protocol_id, nonce_4);
		assert_eq!(AssetsRegistry::from_local_asset(local_asset_id_1), None);
		assert_eq!(AssetsRegistry::from_local_asset(local_asset_id_2), Some(location_2.clone()));
		assert_eq!(AssetsRegistry::from_local_asset(local_asset_id_3), None);
		assert_eq!(AssetsRegistry::from_local_asset(local_asset_id_4), None);
		assert_eq!(AssetsRegistry::from_foreign_asset(location_1.clone()), None);
		assert_eq!(AssetsRegistry::from_foreign_asset(location_2.clone()), Some(local_asset_id_2));
	})
}

#[test]
fn get_foreign_assets_list_should_work() {
	new_test_ext().execute_with(|| {
		let location = ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::here()));
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
			Some(location.clone()),
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
				foreign_id: Some(location),
				existential_deposit: 0,
			}]
		);
	})
}

#[test]
fn get_all_assets_should_work() {
	new_test_ext().execute_with(|| {
		let location = ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::here()));
		let protocol_id = *b"AssTests";
		let nonce = 1_u64;
		let nonce2 = 2_u64;
		let name = Some(BiBoundedVec::from_vec(b"asset_name".to_vec()).unwrap());
		let asset_info = AssetInfo {
			name: name.clone(),
			symbol: None,
			decimals: Some(4),
			existential_deposit: 0,
			ratio: Some(rational!(42 / 123)),
		};
		let id = AssetsRegistry::generate_asset_id(protocol_id, nonce);
		let id2 = AssetsRegistry::generate_asset_id(protocol_id, nonce2);

		let all_assets = AssetsRegistry::get_all_assets();

		assert_eq!(all_assets, vec![]);

		assert_ok!(AssetsRegistry::register_asset(
			RuntimeOrigin::root(),
			protocol_id,
			nonce,
			Some(location.clone()),
			asset_info.clone(),
		));

		assert_ok!(AssetsRegistry::register_asset(
			RuntimeOrigin::root(),
			protocol_id,
			nonce2,
			None,
			asset_info,
		));

		let all_assets = AssetsRegistry::get_all_assets();

		assert_eq!(
			all_assets,
			vec![
				Asset {
					name: name.clone().map(Into::into),
					id,
					decimals: 4,
					ratio: Some(rational!(42 / 123)),
					foreign_id: Some(location),
					existential_deposit: 0,
				},
				Asset {
					name: name.map(Into::into),
					id: id2,
					decimals: 4,
					ratio: Some(rational!(42 / 123)),
					foreign_id: None,
					existential_deposit: 0,
				}
			]
		);
	})
}
