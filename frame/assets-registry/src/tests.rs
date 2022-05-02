use crate::runtime::*;
use composable_traits::{
	defi::Ratio,
	xcm::assets::{
		AssetRatioInspect, ForeignMetadata, RemoteAssetRegistryInspect, XcmAssetLocation,
	},
};
use frame_support::{assert_err, assert_noop, assert_ok};
use frame_system::RawOrigin;
use sp_runtime::{traits::BadOrigin, DispatchError, Storage};

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
			XcmAssetLocation::RELAY_NATIVE,
			42,
			Some(Ratio::from_inner(123)),
			Some(4)
		));
		let asset_id = System::events()
			.iter()
			.find_map(|x| match x.event {
				Event::AssetsRegistry(crate::Event::<Runtime>::AssetRegistered {
					asset_id,
					location: _,
				}) => Some(asset_id),
				_ => None,
			})
			.unwrap();
		assert_eq!(
			<AssetsRegistry as RemoteAssetRegistryInspect>::asset_to_remote(asset_id),
			Some(ForeignMetadata { decimals: Some(4), location: XcmAssetLocation::RELAY_NATIVE })
		);

		assert_eq!(
			<AssetsRegistry as AssetRatioInspect>::get_ratio(asset_id),
			Some(Ratio::from_inner(123)),
		);

		assert_eq!(
			<AssetsRegistry as RemoteAssetRegistryInspect>::location_to_asset(
				XcmAssetLocation::RELAY_NATIVE
			),
			Some(asset_id),
		);
	})
}
