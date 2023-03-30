use crate::{prelude::*, *};
use composable_traits::currency::Rational64;
use hard_coded_assets::HardCodedAssetsMigration;

pub type Migrations = (
	SchedulerMigrationV1toV4,
	HardCodedAssetsMigration,
	preimage::migration::v1::Migration<Runtime>,
	scheduler::migration::v3::MigrateToV4<Runtime>,
	democracy::migrations::v1::Migration<Runtime>,
);

// Migration for scheduler pallet to move from a plain Call to a CallOrHash.
pub struct SchedulerMigrationV1toV4;
impl OnRuntimeUpgrade for SchedulerMigrationV1toV4 {
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		Scheduler::migrate_v1_to_v4()
	}
}

pub mod hard_coded_assets {
	use super::*;
	use assets_registry::WeightInfo;
	use composable_traits::{
		assets::{AssetInfo, AssetInfoUpdate, BiBoundedAssetName, BiBoundedAssetSymbol},
		currency::AssetExistentialDepositInspect,
		rational,
		storage::UpdateValue,
		xcm::assets::{RemoteAssetRegistryInspect, RemoteAssetRegistryMutate},
	};
	use frame_support::traits::{GetStorageVersion, StorageVersion};

	use xcm::latest::prelude::*;

	pub struct HardCodedAssetsMigration;

	const ASSETS_REGISTRY_V1: StorageVersion = StorageVersion::new(1);

	#[derive(Clone)]
	struct AssetCreationInput {
		asset_id: CurrencyId,
		location: Option<XcmAssetLocation>,
		asset_info: AssetInfo<Balance>,
	}

	// in case the asset exists in assets registry but we still want to migrate it
	// new asset info will overwrite old metadata
	fn asset_info_update(asset_info: AssetInfo<Balance>) -> AssetInfoUpdate<Balance> {
		AssetInfoUpdate {
			name: UpdateValue::Set(asset_info.name),
			symbol: UpdateValue::Set(asset_info.symbol),
			decimals: UpdateValue::Set(asset_info.decimals),
			existential_deposit: UpdateValue::Set(asset_info.existential_deposit),
			ratio: UpdateValue::Set(asset_info.ratio),
		}
	}

	fn add_assets_to_storage(assets: Vec<AssetCreationInput>) -> Weight {
		let mut count_created = 0;
		let mut count_updated = 0;
		for AssetCreationInput { asset_id, location, asset_info } in assets {
			// check if there is data stored for foreign asset
			if let Some(foreign_location) = location.clone() {
				// check that new asset_id is the same as old one for the same location
				let is_location_stored =
					<AssetsRegistry as RemoteAssetRegistryInspect>::location_to_asset(
						foreign_location.clone(),
					)
					.map(|prev_asset_id| {
						if prev_asset_id != asset_id {
							panic!("previous and new asset_id for location do not match");
						}
						true
					})
					.unwrap_or(false);

				// check that new location is the same as old one for the same asset_id
				let is_asset_stored =
					<AssetsRegistry as RemoteAssetRegistryInspect>::asset_to_remote(asset_id)
						.map(|prev_location| {
							if prev_location != foreign_location {
								panic!("previous and new location for asset_id do not match");
							}
							true
						})
						.unwrap_or(false);
				// check that either both maps or none map asset_id and location
				if is_location_stored != is_asset_stored {
					panic!("ForeignToLocal and LocalToForeign maps contradict each other");
				}
				if is_location_stored {
					<AssetsRegistry as RemoteAssetRegistryMutate>::update_asset(
						asset_id,
						asset_info_update(asset_info.clone()),
					)
					.expect("asset wasnt updated");
					count_updated += 1;
				} else {
					<AssetsRegistry as RemoteAssetRegistryMutate>::register_asset(
						asset_id,
						location.clone(),
						asset_info.clone(),
					)
					.expect("asset wasnt registered");
					count_created += 1;
				}
				continue
			}
			// check that for local asset there is no location previously stored
			if let Some(_prev_location) =
				<AssetsRegistry as RemoteAssetRegistryInspect>::asset_to_remote(asset_id)
			{
				panic!("location is not None for local asset_id");
			}
			// check if there is local asset with asset_id
			if <AssetsRegistry as AssetExistentialDepositInspect>::existential_deposit(asset_id)
				.is_ok()
			{
				<AssetsRegistry as RemoteAssetRegistryMutate>::update_asset(
					asset_id,
					asset_info_update(asset_info.clone()),
				)
				.expect("asset wasnt updated");
				count_updated += 1;
				continue
			}
			// register new asset if there was no such asset in memory previously
			<AssetsRegistry as RemoteAssetRegistryMutate>::register_asset(
				asset_id, location, asset_info,
			)
			.expect("asset wasnt registered");
			count_created += 1;
		}

		weights::assets_registry::WeightInfo::<Runtime>::register_asset()
			.saturating_mul(count_created as u64)
			.saturating_add(
				weights::assets_registry::WeightInfo::<Runtime>::update_asset()
					.saturating_mul(count_updated as u64),
			)
	}

	impl OnRuntimeUpgrade for HardCodedAssetsMigration {
		fn on_runtime_upgrade() -> Weight {
			let on_chain_version =
				<AssetsRegistry as GetStorageVersion>::on_chain_storage_version();
			if on_chain_version < ASSETS_REGISTRY_V1 {
				let assets = vec![
					AssetCreationInput {
						asset_id: CurrencyId(2),
						location: Some(XcmAssetLocation(MultiLocation::here())),
						asset_info: AssetInfo {
							name: Some(
								BiBoundedAssetName::from_vec(b"Composable Finance".to_vec())
									.expect("String is within bounds"),
							),
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"LAYR".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(12),
							existential_deposit: 100_000_000_000,
							ratio: None,
						},
					},
					AssetCreationInput {
						asset_id: CurrencyId(6),
						location: Some(XcmAssetLocation(MultiLocation::parent())),
						asset_info: AssetInfo {
							name: Some(
								BiBoundedAssetName::from_vec(b"Polkadot".to_vec())
									.expect("String is within bounds"),
							),
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"DOT".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(12),
							existential_deposit: 214_300_000,
							ratio: Some(rational!(2143 / 1_000_000)),
						},
					},
				];

				StorageVersion::new(1).put::<AssetsRegistry>();
				add_assets_to_storage(assets)
			} else {
				<Runtime as system::Config>::DbWeight::get().reads(1)
			}
		}
	}
	#[cfg(test)]
	mod tests {
		use frame_support::sp_io;

		use super::*;
		use composable_traits::assets::InspectRegistryMetadata;

		pub fn new_test_ext() -> sp_io::TestExternalities {
			let storage = frame_system::GenesisConfig::default()
				.build_storage::<Runtime>()
				.expect("in memory test");
			let mut externalities = sp_io::TestExternalities::new(storage);
			externalities.execute_with(|| System::set_block_number(1));
			externalities
		}

		mod migrate_asset {

			use super::*;

			#[test]
			fn should_migrate_all() {
				let assets = vec![
					AssetCreationInput {
						asset_id: CurrencyId(2),
						location: Some(XcmAssetLocation(MultiLocation::here())),
						asset_info: AssetInfo {
							name: Some(
								BiBoundedAssetName::from_vec(b"Composable Finance".to_vec())
									.expect("String is within bounds"),
							),
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"LAYR".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(12),
							existential_deposit: 100_000_000_000,
							ratio: None,
						},
					},
					AssetCreationInput {
						asset_id: CurrencyId(6),
						location: Some(XcmAssetLocation(MultiLocation::parent())),
						asset_info: AssetInfo {
							name: Some(
								BiBoundedAssetName::from_vec(b"Polkadot".to_vec())
									.expect("String is within bounds"),
							),
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"DOT".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(12),
							existential_deposit: 214_300_000,
							ratio: Some(rational!(2143 / 1_000_000)),
						},
					},
				];

				new_test_ext().execute_with(|| {
					add_assets_to_storage(assets.to_owned());
					for AssetCreationInput { asset_id, location, asset_info } in assets {
						assert_eq!(
							<AssetsRegistry as InspectRegistryMetadata>::asset_name(&asset_id),
							asset_info.name.map(|name| name.as_vec().to_owned())
						);
						assert_eq!(
							<AssetsRegistry as AssetExistentialDepositInspect>::existential_deposit(
								asset_id
							),
							Ok(asset_info.existential_deposit)
						);
						assert_eq!(
							<AssetsRegistry as RemoteAssetRegistryInspect>::asset_to_remote(
								asset_id
							),
							location
						);
						if let Some(location_unwrapped) = location {
							assert_eq!(
								<AssetsRegistry as RemoteAssetRegistryInspect>::location_to_asset(
									location_unwrapped
								),
								Some(asset_id)
							);
						}
					}
				})
			}
		}
	}
}
