use crate::{mock::*, *};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;

#[test]
fn set_local_admin_tests() {
	new_test_ext().execute_with(|| {
		assert_noop!(AssetsRegistry::set_local_admin(Origin::signed(ALICE), ALICE), BadOrigin,);

		assert_ok!(AssetsRegistry::set_local_admin(Origin::signed(ROOT), ALICE));
	})
}

#[test]
fn set_foreign_admin_tests() {
	new_test_ext().execute_with(|| {
		assert_noop!(AssetsRegistry::set_foreign_admin(Origin::signed(BOB), BOB), BadOrigin,);

		assert_ok!(AssetsRegistry::set_foreign_admin(Origin::signed(ROOT), BOB));
	})
}

#[test]
fn approve_assets_mapping_candidate_tests() {
	new_test_ext().execute_with(|| {
		let (local_asset_id, foreign_asset_id) = (0, 100);
		assert_eq!(AssetsRegistry::from_local_asset(local_asset_id), None);
		assert_eq!(AssetsRegistry::from_local_asset(foreign_asset_id), None);
		assert_ok!(AssetsRegistry::set_local_admin(Origin::signed(ROOT), ALICE));
		assert_ok!(AssetsRegistry::set_foreign_admin(Origin::signed(ROOT), BOB));
		assert_ok!(AssetsRegistry::approve_assets_mapping_candidate(
			Origin::signed(ALICE),
			local_asset_id,
			foreign_asset_id
		));
		assert_eq!(
			<AssetsMappingCandidates<Test>>::get((local_asset_id, foreign_asset_id)),
			Some(CandidateStatus::LocalAdminApproved),
		);
		assert_ok!(AssetsRegistry::approve_assets_mapping_candidate(
			Origin::signed(BOB),
			local_asset_id,
			foreign_asset_id
		));
		assert_eq!(AssetsRegistry::from_local_asset(local_asset_id), Some(foreign_asset_id));
		assert_eq!(AssetsRegistry::from_foreign_asset(foreign_asset_id), Some(local_asset_id));

		let (other_local_asset_id, other_foreign_asset_id) = (1, 101);
		assert_noop!(
			AssetsRegistry::approve_assets_mapping_candidate(
				Origin::signed(ALICE),
				other_local_asset_id,
				foreign_asset_id
			),
			Error::<Test>::ForeignAssetIdAlreadyUsed,
		);
		assert_noop!(
			AssetsRegistry::approve_assets_mapping_candidate(
				Origin::signed(ALICE),
				local_asset_id,
				other_foreign_asset_id
			),
			Error::<Test>::LocalAssetIdAlreadyUsed,
		);

		assert_noop!(
			AssetsRegistry::approve_assets_mapping_candidate(
				Origin::signed(CHARLIE),
				other_local_asset_id,
				other_foreign_asset_id
			),
			Error::<Test>::OnlyAllowedForAdmins,
		);

		assert_eq!(AssetsRegistry::from_local_asset(other_local_asset_id), None);
		assert_eq!(AssetsRegistry::from_foreign_asset(other_foreign_asset_id), None);
		assert_ok!(AssetsRegistry::approve_assets_mapping_candidate(
			Origin::signed(BOB),
			other_local_asset_id,
			other_foreign_asset_id
		));
		assert_eq!(
			<AssetsMappingCandidates<Test>>::get((other_local_asset_id, other_foreign_asset_id)),
			Some(CandidateStatus::ForeignAdminApproved),
		);
		assert_ok!(AssetsRegistry::approve_assets_mapping_candidate(
			Origin::signed(ALICE),
			other_local_asset_id,
			other_foreign_asset_id
		));
		assert_eq!(
			AssetsRegistry::from_local_asset(other_local_asset_id),
			Some(other_foreign_asset_id)
		);
		assert_eq!(
			AssetsRegistry::from_foreign_asset(other_foreign_asset_id),
			Some(other_local_asset_id)
		);
	})
}
