use crate::{prelude::*, *};

use hard_coded_assets::HardCodedAssetsMigration;
use pablo_picasso_init_pools::PabloPicassoInitialPoolsMigration;

pub type Migrations = (
	PabloPicassoInitialPoolsMigration,
	SchedulerMigrationV1toV4,
	HardCodedAssetsMigration,
	preimage::migration::v1::Migration<Runtime>,
	scheduler::migration::v3::MigrateToV4<Runtime>,
	democracy::migrations::v1::Migration<Runtime>,
	multisig::migrations::v1::MigrateToV1<Runtime>,
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
		currency::{AssetExistentialDepositInspect, Rational64},
		rational,
		storage::UpdateValue,
		xcm::assets::RemoteAssetRegistryInspect,
	};
	use frame_support::{
		traits::{GetStorageVersion, StorageVersion},
		WeakBoundedVec,
	};

	use primitives::topology;
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
						asset_id: CurrencyId(1),
						location: None,
						asset_info: AssetInfo {
							name: Some(
								BiBoundedAssetName::from_vec(b"Picasso".to_vec())
									.expect("String is within bounds"),
							),
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"PICA".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(12),
							existential_deposit: 100_000_000_000,
							ratio: None,
						},
					},
					AssetCreationInput {
						asset_id: CurrencyId(4),
						location: Some(XcmAssetLocation(MultiLocation::parent())),
						asset_info: AssetInfo {
							name: Some(
								BiBoundedAssetName::from_vec(b"Kusama".to_vec())
									.expect("String is within bounds"),
							),
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"KSM".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(12),
							existential_deposit: 37_500_000,
							ratio: Some(rational!(375 / 1_000_000)),
						},
					},
					AssetCreationInput {
						asset_id: CurrencyId(105),
						location: None,
						asset_info: AssetInfo {
							name: Some(
								BiBoundedAssetName::from_vec(b"KSM USDT LPT".to_vec())
									.expect("String is within bounds"),
							),
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"KSM_USDT_LPT".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(12),
							existential_deposit: 100,
							ratio: None,
						},
					},
					AssetCreationInput {
						asset_id: CurrencyId(106),
						location: None,
						asset_info: AssetInfo {
							name: Some(
								BiBoundedAssetName::from_vec(b"PICA USDT LPT".to_vec())
									.expect("String is within bounds"),
							),
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"PICA_USDT_LPT".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(12),
							existential_deposit: 100,
							ratio: None,
						},
					},
					AssetCreationInput {
						asset_id: CurrencyId(107),
						location: None,
						asset_info: AssetInfo {
							name: Some(
								BiBoundedAssetName::from_vec(b"PICA KSM LPT".to_vec())
									.expect("String is within bounds"),
							),
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"PICA_KSM_LPT".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(12),
							existential_deposit: 100,
							ratio: None,
						},
					},
					AssetCreationInput {
						asset_id: CurrencyId(129),
						location: Some(XcmAssetLocation(MultiLocation {
							parents: 1,
							interior: X2(
								Parachain(topology::karura::ID),
								GeneralKey(WeakBoundedVec::force_from(
									topology::karura::AUSD_KEY.to_vec(),
									None,
								)),
							),
						})),
						asset_info: AssetInfo {
							name: Some(
								BiBoundedAssetName::from_vec(b"Karura Dollar".to_vec())
									.expect("String is within bounds"),
							),
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"kUSD".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(12),
							existential_deposit: 100_000_000,
							ratio: Some(rational!(15 / 1_000)),
						},
					},
					AssetCreationInput {
						asset_id: CurrencyId(130),
						location: Some(XcmAssetLocation(MultiLocation {
							parents: 1,
							interior: X3(
								Parachain(topology::common_good_assets::ID),
								PalletInstance(topology::common_good_assets::ASSETS),
								GeneralIndex(topology::common_good_assets::USDT),
							),
						})),
						asset_info: AssetInfo {
							name: Some(
								BiBoundedAssetName::from_vec(b"Tether".to_vec())
									.expect("String is within bounds"),
							),
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"USDT".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(6),
							existential_deposit: 100,
							ratio: Some(rational!(15 / 1_000_000_000)),
						},
					},
					AssetCreationInput {
						asset_id: CurrencyId(5),
						location: Some(XcmAssetLocation(MultiLocation {
							parents: 0,
							interior: X1(GeneralIndex(5)),
						})),
						asset_info: AssetInfo {
							name: None,
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"PBLO".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(12),
							existential_deposit: 100_000_000_000,
							ratio: Some(rational!(1 / 1)),
						},
					},
					AssetCreationInput {
						asset_id: CurrencyId(6),
						location: None,
						asset_info: AssetInfo {
							name: None,
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"ibcDOT".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(12),
							existential_deposit: 214_300_000,
							ratio: None,
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
			fn should_migrate_local_asset() {
				let assets = vec![AssetCreationInput {
					asset_id: CurrencyId(1),
					location: None,
					asset_info: AssetInfo {
						name: Some(
							BiBoundedAssetName::from_vec(b"Picasso".to_vec())
								.expect("String is within bounds"),
						),
						symbol: Some(
							BiBoundedAssetSymbol::from_vec(b"PICA".to_vec())
								.expect("String is within bounds"),
						),
						decimals: Some(12),
						existential_deposit: 100_000_000_000,
						ratio: None,
					},
				}];

				new_test_ext().execute_with(|| {
					assert_eq!(
						<AssetsRegistry as InspectRegistryMetadata>::asset_name(&CurrencyId(1)),
						None
					);
					add_assets_to_storage(assets);
					assert_eq!(
						<AssetsRegistry as InspectRegistryMetadata>::asset_name(&CurrencyId(1)),
						Some(b"Picasso".to_vec())
					);
				})
			}
			#[test]
			fn should_migrate_foreign_asset() {
				let assets = vec![AssetCreationInput {
					asset_id: CurrencyId(4),
					location: Some(XcmAssetLocation(MultiLocation::parent())),
					asset_info: AssetInfo {
						name: Some(
							BiBoundedAssetName::from_vec(b"Kusama".to_vec())
								.expect("String is within bounds"),
						),
						symbol: Some(
							BiBoundedAssetSymbol::from_vec(b"KSM".to_vec())
								.expect("String is within bounds"),
						),
						decimals: Some(12),
						existential_deposit: 37_500_000,
						ratio: Some(rational!(375 / 1_000_000)),
					},
				}];

				new_test_ext().execute_with(|| {
					add_assets_to_storage(assets);
					assert_eq!(
						<AssetsRegistry as InspectRegistryMetadata>::asset_name(&CurrencyId(4)),
						Some(b"Kusama".to_vec())
					);
					assert_eq!(
						<AssetsRegistry as RemoteAssetRegistryInspect>::location_to_asset(
							XcmAssetLocation(MultiLocation::parent())
						),
						Some(CurrencyId(4))
					);

					assert_eq!(
						<AssetsRegistry as RemoteAssetRegistryInspect>::asset_to_remote(
							CurrencyId(4)
						),
						Some(XcmAssetLocation(MultiLocation::parent()))
					);
				})
			}
			#[test]
			fn should_migrate_all() {
				let assets = vec![
					AssetCreationInput {
						asset_id: CurrencyId(1),
						location: None,
						asset_info: AssetInfo {
							name: Some(
								BiBoundedAssetName::from_vec(b"Picasso".to_vec())
									.expect("String is within bounds"),
							),
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"PICA".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(12),
							existential_deposit: 100_000_000_000,
							ratio: None,
						},
					},
					AssetCreationInput {
						asset_id: CurrencyId(4),
						location: Some(XcmAssetLocation(MultiLocation::parent())),
						asset_info: AssetInfo {
							name: Some(
								BiBoundedAssetName::from_vec(b"Kusama".to_vec())
									.expect("String is within bounds"),
							),
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"KSM".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(12),
							existential_deposit: 37_500_000,
							ratio: Some(rational!(375 / 1_000_000)),
						},
					},
					AssetCreationInput {
						asset_id: CurrencyId(105),
						location: None,
						asset_info: AssetInfo {
							name: Some(
								BiBoundedAssetName::from_vec(b"KSM USDT LPT".to_vec())
									.expect("String is within bounds"),
							),
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"KSM_USDT_LPT".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(12),
							existential_deposit: 100,
							ratio: None,
						},
					},
					AssetCreationInput {
						asset_id: CurrencyId(106),
						location: None,
						asset_info: AssetInfo {
							name: Some(
								BiBoundedAssetName::from_vec(b"PICA USDT LPT".to_vec())
									.expect("String is within bounds"),
							),
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"PICA_USDT_LPT".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(12),
							existential_deposit: 100,
							ratio: None,
						},
					},
					AssetCreationInput {
						asset_id: CurrencyId(107),
						location: None,
						asset_info: AssetInfo {
							name: Some(
								BiBoundedAssetName::from_vec(b"PICA KSM LPT".to_vec())
									.expect("String is within bounds"),
							),
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"PICA_KSM_LPT".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(12),
							existential_deposit: 100,
							ratio: None,
						},
					},
					AssetCreationInput {
						asset_id: CurrencyId(129),
						location: Some(XcmAssetLocation(MultiLocation {
							parents: 1,
							interior: X2(
								Parachain(topology::karura::ID),
								GeneralKey(WeakBoundedVec::force_from(
									topology::karura::AUSD_KEY.to_vec(),
									None,
								)),
							),
						})),
						asset_info: AssetInfo {
							name: Some(
								BiBoundedAssetName::from_vec(b"Karura Dollar".to_vec())
									.expect("String is within bounds"),
							),
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"kUSD".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(12),
							existential_deposit: 100_000_000,
							ratio: Some(rational!(15 / 1_000)),
						},
					},
					AssetCreationInput {
						asset_id: CurrencyId(130),
						location: Some(XcmAssetLocation(MultiLocation {
							parents: 1,
							interior: X3(
								Parachain(topology::common_good_assets::ID),
								PalletInstance(topology::common_good_assets::ASSETS),
								GeneralIndex(topology::common_good_assets::USDT),
							),
						})),
						asset_info: AssetInfo {
							name: Some(
								BiBoundedAssetName::from_vec(b"Tether".to_vec())
									.expect("String is within bounds"),
							),
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"USDT".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(6),
							existential_deposit: 100,
							ratio: Some(rational!(15 / 1_000_000_000)),
						},
					},
					AssetCreationInput {
						asset_id: CurrencyId(5),
						location: Some(XcmAssetLocation(MultiLocation {
							parents: 0,
							interior: X1(GeneralIndex(5)),
						})),
						asset_info: AssetInfo {
							name: None,
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"PBLO".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(12),
							existential_deposit: 100_000_000_000,
							ratio: Some(rational!(1 / 1)),
						},
					},
					AssetCreationInput {
						asset_id: CurrencyId(6),
						location: None,
						asset_info: AssetInfo {
							name: None,
							symbol: Some(
								BiBoundedAssetSymbol::from_vec(b"ibcDOT".to_vec())
									.expect("String is within bounds"),
							),
							decimals: Some(12),
							existential_deposit: 214_300_000,
							ratio: None,
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
					}
				})
			}
		}
	}
}

pub mod pablo_picasso_init_pools {

	use super::*;

	use frame_support::BoundedBTreeMap;
	use pablo::{pallet::PoolInitConfiguration, WeightInfo};
	use sp_runtime::PerThing;

	pub struct PabloPicassoInitialPoolsMigration;

	#[derive(Clone)]
	struct PoolCreationInput {
		/// Initial Configuration for the Pool
		init_config: PoolInitConfiguration<AccountId, CurrencyId>,
		/// LP Token for pool to mint
		lp_token: CurrencyId,
	}

	impl PoolCreationInput {
		fn new_two_token_pool(
			first_asset_id: CurrencyId,
			first_asset_weight: Permill,
			second_asset_id: CurrencyId,
			lp_asset: CurrencyId,
			fee: Permill,
		) -> Self {
			Self {
				init_config: create_two_token_pool_config(
					first_asset_id,
					second_asset_id,
					first_asset_weight,
					fee,
				),
				lp_token: lp_asset,
			}
		}
	}

	/// Adds pools to Pablo Storage
	///
	/// Expects a vec of (pool_init_config, pool_lp_token_id)
	fn add_initial_pools_to_storage(pools: Vec<PoolCreationInput>) -> Weight {
		if !Pablo::pool_count().is_zero() {
			return Weight::zero()
		}

		pools.iter().for_each(|pool_creation_input| {
			Pablo::do_create_pool(
				pool_creation_input.init_config.to_owned(),
				Some(pool_creation_input.lp_token),
			)
			.expect("Pool config is valid; QED");
		});

		weights::pablo::WeightInfo::<Runtime>::do_create_pool().saturating_mul(pools.len() as u64)
	}

	fn create_two_token_pool_config(
		first_asset_id: CurrencyId,
		second_asset_id: CurrencyId,
		first_asset_weight: Permill,
		fee: Permill,
	) -> PoolInitConfiguration<AccountId, CurrencyId> {
		let owner = AccountId::from([0; 32]);

		let mut assets_weights = BoundedBTreeMap::new();

		assets_weights
			.try_insert(first_asset_id, first_asset_weight)
			.expect("Map is within bounds; QED");
		assets_weights
			.try_insert(second_asset_id, first_asset_weight.left_from_one())
			.expect("Map is within bounds; QED");

		PoolInitConfiguration::<AccountId, CurrencyId>::DualAssetConstantProduct {
			owner,
			assets_weights,
			fee,
		}
	}

	impl OnRuntimeUpgrade for PabloPicassoInitialPoolsMigration {
		fn on_runtime_upgrade() -> Weight {
			let pools = vec![
				PoolCreationInput::new_two_token_pool(
					CurrencyId::KSM,
					Permill::from_percent(50),
					CurrencyId::USDT,
					CurrencyId::KSM_USDT_LPT,
					Permill::from_rational::<u32>(3, 1000),
				),
				PoolCreationInput::new_two_token_pool(
					CurrencyId::PICA,
					Permill::from_percent(50),
					CurrencyId::USDT,
					CurrencyId::PICA_USDT_LPT,
					Permill::from_rational::<u32>(3, 1000),
				),
				PoolCreationInput::new_two_token_pool(
					CurrencyId::PICA,
					Permill::from_percent(50),
					CurrencyId::KSM,
					CurrencyId::PICA_KSM_LPT,
					Permill::from_rational::<u32>(3, 1000),
				),
			];

			add_initial_pools_to_storage(pools)
		}
	}

	#[cfg(test)]
	mod tests {
		use frame_support::sp_io;

		use super::*;

		pub fn new_test_ext() -> sp_io::TestExternalities {
			let storage = frame_system::GenesisConfig::default()
				.build_storage::<Runtime>()
				.expect("in memory test");
			let mut externalities = sp_io::TestExternalities::new(storage);
			externalities.execute_with(|| System::set_block_number(1));
			externalities
		}

		mod add_initial_pools_to_storage {
			use pablo::pallet::PoolConfiguration;

			use super::*;

			#[test]
			fn should_update_the_pool_count() {
				let pools = vec![
					PoolCreationInput::new_two_token_pool(
						CurrencyId::KSM,
						Permill::from_percent(50),
						CurrencyId::USDT,
						CurrencyId::KSM_USDT_LPT,
						Permill::from_rational::<u32>(3, 1000),
					),
					PoolCreationInput::new_two_token_pool(
						CurrencyId::PICA,
						Permill::from_percent(50),
						CurrencyId::USDT,
						CurrencyId::PICA_USDT_LPT,
						Permill::from_rational::<u32>(3, 1000),
					),
				];

				new_test_ext().execute_with(|| {
					assert_eq!(Pablo::pool_count(), 0);
					add_initial_pools_to_storage(pools);
					assert_eq!(Pablo::pool_count(), 2);
				})
			}

			#[test]
			fn should_create_pools_with_given_data() {
				let pools = vec![
					PoolCreationInput::new_two_token_pool(
						CurrencyId::KSM,
						Permill::from_percent(50),
						CurrencyId::USDT,
						CurrencyId::KSM_USDT_LPT,
						Permill::from_rational::<u32>(3, 1000),
					),
					PoolCreationInput::new_two_token_pool(
						CurrencyId::PICA,
						Permill::from_percent(50),
						CurrencyId::USDT,
						CurrencyId::PICA_USDT_LPT,
						Permill::from_rational::<u32>(3, 1000),
					),
				];

				new_test_ext().execute_with(|| {
					assert_eq!(Pablo::pools(0), None);
					assert_eq!(Pablo::pools(1), None);
					add_initial_pools_to_storage(pools);
					assert_eq!(Pablo::pool_count(), 2);
					let ksm_usdt_pool = Pablo::pools(0).expect("Pool is some; QED");
					let pica_usdt_pool = Pablo::pools(1).expect("Pool is some; QED");

					match ksm_usdt_pool {
						PoolConfiguration::DualAssetConstantProduct(pool_info) => {
							assert_eq!(pool_info.lp_token, CurrencyId::KSM_USDT_LPT);
							assert_eq!(
								pool_info.fee_config.fee_rate,
								Permill::from_rational::<u32>(3, 1000)
							);
						},
					}

					match pica_usdt_pool {
						PoolConfiguration::DualAssetConstantProduct(pool_info) => {
							assert_eq!(pool_info.lp_token, CurrencyId::PICA_USDT_LPT);
							assert_eq!(
								pool_info.fee_config.fee_rate,
								Permill::from_rational::<u32>(3, 1000)
							);
						},
					}
				})
			}
		}
	}
}
