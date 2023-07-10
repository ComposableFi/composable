use super::*;
use crate::{runtime::*, Error};

use composable_traits::{
	assets::{BiBoundedAssetName, BiBoundedAssetSymbol, GenerateAssetId, InspectRegistryMetadata},
	storage::UpdateValue,
	xcm::assets::RemoteAssetRegistryInspect,
};
use frame_support::{assert_err, assert_ok};
use frame_system::RawOrigin;
use primitives::currency::{ForeignAssetId, VersionedMultiLocation};
use sp_arithmetic::ArithmeticError;
use xcm::latest::MultiLocation;

#[allow(unused_imports)]
use crate::Pallet as AssetsInterface;

use runtime::AssetsRegistry;
use sp_std::prelude::*;

use frame_support::traits::{fungible::Mutate, PalletInfoAccess};
#[test]
fn test_set_creation_fee() {
	new_test_ext().execute_with(|| {
		assert_err!(
			AssetsInterface::<Test>::set_creation_fee(
				RuntimeOrigin::signed(ALICE),
				100_u128.into()
			),
			Error::<Test>::BadOrigin
		);
		assert_ok!(AssetsInterface::<Test>::set_creation_fee(
			RawOrigin::Root.into(),
			100_u128.into()
		));
	})
}

#[test]
fn test_cosmwasm_asset() {
	new_test_ext().execute_with(|| {
		let owner: AccountId = ALICE;
		let name = Some(
			BiBoundedAssetName::from_vec(b"Kusama".to_vec()).expect("String is within bounds"),
		);
		let symbol =
			Some(BiBoundedAssetSymbol::from_vec(b"KSM".to_vec()).expect("Stringis within bounds"));
		let decimals = 3;
		assert_ok!(Balances::mint_into(&AccountId::into(owner.clone()), 200));
		assert_ok!(AssetsInterface::<Test>::set_creation_fee(RawOrigin::Root.into(), 100));
		assert_ok!(AssetsInterface::<Test>::register_cosmwasm_asset(
			RuntimeOrigin::signed(ALICE),
			name.clone(),
			symbol,
			decimals,
		));
		let asset_id: AssetId = <AssetsRegistry as GenerateAssetId>::generate_asset_id(
			(AssetsInterface::<Test>::index() as u32).to_be_bytes(),
			AssetNonce::<Test>::get(),
		)
		.into();
		assert_eq!(
			<AssetsRegistry as InspectRegistryMetadata>::asset_name(&asset_id),
			name.map(|name| name.as_vec().to_owned())
		);

		let name = Some(
			BiBoundedAssetName::from_vec(b"CKusama".to_vec()).expect("String is within bounds"),
		);
		let symbol =
			Some(BiBoundedAssetSymbol::from_vec(b"CKSM".to_vec()).expect("Stringis within bounds"));
		let decimals = 4;

		assert_ok!(AssetsInterface::<Test>::update_asset(
			RuntimeOrigin::signed(ALICE),
			asset_id,
			UpdateValue::Set(name.clone()),
			UpdateValue::Set(symbol.clone()),
			UpdateValue::Set(decimals.clone())
		));

		assert_eq!(
			<AssetsRegistry as InspectRegistryMetadata>::asset_name(&asset_id),
			name.map(|name| name.as_vec().to_owned())
		);
		assert_eq!(
			<AssetsRegistry as InspectRegistryMetadata>::symbol(&asset_id),
			symbol.map(|symbol| symbol.as_vec().to_owned())
		);
		assert_eq!(
			<AssetsRegistry as InspectRegistryMetadata>::decimals(&asset_id),
			Some(decimals)
		);
		let location = ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::here()));
		assert_err!(
			AssetsInterface::<Test>::update_asset_location(
				RuntimeOrigin::signed(ALICE),
				asset_id,
				Some(location)
			),
			Error::<Test>::CosmWasmAsset
		);
		assert_err!(
			AssetsInterface::<Test>::update_asset_location(
				RuntimeOrigin::signed(ALICE),
				asset_id,
				None
			),
			Error::<Test>::CosmWasmAsset
		);

		assert_ok!(AssetsInterface::<Test>::mint_cosmwasm(
			RuntimeOrigin::signed(ALICE),
			asset_id,
			ALICE.clone(),
			200_000,
		));

		assert_err!(
			AssetsInterface::<Test>::burn_cosmwasm(
				RuntimeOrigin::signed(ALICE),
				asset_id,
				ALICE.clone(),
				300_000,
			),
			ArithmeticError::Underflow
		);

		assert_ok!(AssetsInterface::<Test>::burn_cosmwasm(
			RuntimeOrigin::signed(ALICE),
			asset_id,
			ALICE.clone(),
			200_000,
		));
	})
}

#[test]
fn test_other_asset() {
	new_test_ext().execute_with(|| {
		let owner: AccountId = ALICE;
		let name = Some(
			BiBoundedAssetName::from_vec(b"Kusama".to_vec()).expect("String is within bounds"),
		);
		let symbol =
			Some(BiBoundedAssetSymbol::from_vec(b"KSM".to_vec()).expect("Stringis within bounds"));
		let decimals = 3;
		assert_ok!(Balances::mint_into(&AccountId::into(owner.clone()), 200));
		assert_ok!(AssetsInterface::<Test>::set_creation_fee(RawOrigin::Root.into(), 100));
		assert_ok!(AssetsInterface::<Test>::register_asset(
			RuntimeOrigin::signed(ALICE),
			None,
			name.clone(),
			symbol,
			decimals,
		));
		let asset_id: AssetId = <AssetsRegistry as GenerateAssetId>::generate_asset_id(
			(AssetsInterface::<Test>::index() as u32).to_be_bytes(),
			AssetNonce::<Test>::get(),
		)
		.into();
		assert_eq!(
			<AssetsRegistry as InspectRegistryMetadata>::asset_name(&asset_id),
			name.map(|name| name.as_vec().to_owned())
		);

		let name = Some(
			BiBoundedAssetName::from_vec(b"CKusama".to_vec()).expect("String is within bounds"),
		);
		let symbol =
			Some(BiBoundedAssetSymbol::from_vec(b"CKSM".to_vec()).expect("Stringis within bounds"));
		let decimals = 4;

		assert_ok!(AssetsInterface::<Test>::update_asset(
			RuntimeOrigin::signed(ALICE),
			asset_id,
			UpdateValue::Set(name.clone()),
			UpdateValue::Set(symbol.clone()),
			UpdateValue::Set(decimals.clone())
		));

		assert_eq!(
			<AssetsRegistry as InspectRegistryMetadata>::asset_name(&asset_id),
			name.map(|name| name.as_vec().to_owned())
		);
		assert_eq!(
			<AssetsRegistry as InspectRegistryMetadata>::symbol(&asset_id),
			symbol.map(|symbol| symbol.as_vec().to_owned())
		);
		assert_eq!(
			<AssetsRegistry as InspectRegistryMetadata>::decimals(&asset_id),
			Some(decimals)
		);
		let location = ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::here()));
		assert_ok!(AssetsInterface::<Test>::update_asset_location(
			RuntimeOrigin::signed(ALICE),
			asset_id,
			Some(location.clone())
		));

		assert_eq!(
			<AssetsRegistry as RemoteAssetRegistryInspect>::asset_to_remote(asset_id.clone()),
			Some(location.clone())
		);
		assert_eq!(
			<AssetsRegistry as RemoteAssetRegistryInspect>::location_to_asset(location),
			Some(asset_id)
		);

		assert_err!(
			AssetsInterface::<Test>::mint_cosmwasm(
				RuntimeOrigin::signed(ALICE),
				asset_id,
				ALICE.clone(),
				200_000,
			),
			Error::<Test>::NotCosmWasmAsset
		);

		assert_err!(
			AssetsInterface::<Test>::burn_cosmwasm(
				RuntimeOrigin::signed(ALICE),
				asset_id,
				ALICE.clone(),
				300_000,
			),
			Error::<Test>::NotCosmWasmAsset
		);
	})
}
