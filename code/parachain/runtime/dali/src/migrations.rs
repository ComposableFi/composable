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
		assets::{AssetInfo, AssetInfoUpdate, InspectRegistryMetadata},
		currency::Rational64,
		rational,
	};
	use frame_support::WeakBoundedVec;

	use primitives::topology;
	use xcm::latest::prelude::*;

	pub struct HardCodedAssetsMigration;

	#[derive(Clone)]
	struct AssetCreationInput {
		asset_id: CurrencyId,
		location: Option<XcmAssetLocation>,
		asset_info: AssetInfo<Balance>,
	}

	impl AssetCreationInput {
		fn new_asset(
			asset_id: CurrencyId,
			location: Option<XcmAssetLocation>,
			name: Vec<u8>,
			symbol: Vec<u8>,
			decimals: u8,
			existential_deposit: Balance,
			ratio: Option<Rational64>,
		) -> Self {
			Self {
				asset_id,
				location,
				asset_info: AssetInfo { name, symbol, decimals, existential_deposit, ratio },
			}
		}
	}

	fn asset_info_update(asset_info: AssetInfo<Balance>) -> AssetInfoUpdate<Balance> {
		AssetInfoUpdate {
			name: Some(asset_info.name),
			symbol: Some(asset_info.symbol),
			decimals: Some(asset_info.decimals),
			existential_deposit: Some(asset_info.existential_deposit),
			ratio: Some(asset_info.ratio),
		}
	}

	fn add_assets_to_storage(assets: Vec<AssetCreationInput>) -> Weight {
		let (mut count_created, mut count_updated) = (0, 0);
		for asset_input in assets {
			let AssetCreationInput { asset_id, location, asset_info } = asset_input;
			// check if there is data stored for foreign asset
			if let Some(foreign_location) = location.clone() {
				// check that new asset_id is the same as old one for the same location
				let mut location_stored = false;
				if let Some(prev_asset_id) =
					<AssetsRegistry as RemoteAssetRegistryInspect>::location_to_asset(
						foreign_location.clone(),
					) {
					if prev_asset_id != asset_id {
						panic!("previous and new asset_id for location do not match");
					}
					location_stored = true;
				}
				// check that new location is the same as old one for the same asset_id
				let mut asset_stored = false;
				if let Some(prev_location) =
					<AssetsRegistry as RemoteAssetRegistryInspect>::asset_to_remote(asset_id)
				{
					if prev_location != foreign_location {
						panic!("previous and new location for asset_id do not match");
					}
					asset_stored = true;
				}
				// check that either both maps or none map asset_id and location
				if location_stored != asset_stored {
					panic!("ForeignToLocal and LocalToForeign maps contradict each other");
				}
				if location_stored {
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
			if <AssetsRegistry as InspectRegistryMetadata>::asset_name(&asset_id).is_some() {
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
			let assets = vec![
				AssetCreationInput::new_asset(
					CurrencyId(1),
					None,
					"Picasso".as_bytes().to_vec(),
					"PICA".as_bytes().to_vec(),
					12,
					100_000_000_000,
					None,
				),
				AssetCreationInput::new_asset(
					CurrencyId(4),
					Some(XcmAssetLocation(MultiLocation::parent())),
					"Kusama".as_bytes().to_vec(),
					"KSM".as_bytes().to_vec(),
					12,
					37_500_000,
					Some(rational!(375 / 1_000_000)),
				),
				AssetCreationInput::new_asset(
					CurrencyId(105),
					None,
					"Kusama Tether LPT".as_bytes().to_vec(),
					"KSM_USDT_LPT".as_bytes().to_vec(),
					12,
					100,
					None,
				),
				AssetCreationInput::new_asset(
					CurrencyId(106),
					None,
					"Picasso Tether LPT".as_bytes().to_vec(),
					"PICA_USDT_LPT".as_bytes().to_vec(),
					12,
					100,
					None,
				),
				AssetCreationInput::new_asset(
					CurrencyId(107),
					None,
					"Picasso Kusama LPT".as_bytes().to_vec(),
					"PICA_KSM_LPT".as_bytes().to_vec(),
					12,
					100,
					None,
				),
				AssetCreationInput::new_asset(
					CurrencyId(129),
					Some(XcmAssetLocation(MultiLocation {
						parents: 1,
						interior: X2(
							Parachain(topology::karura::ID),
							GeneralKey(WeakBoundedVec::force_from(
								topology::karura::AUSD_KEY.to_vec(),
								None,
							)),
						),
					})),
					"Karura Dollar".as_bytes().to_vec(),
					"kUSD".as_bytes().to_vec(),
					12,
					100_000_000,
					Some(rational!(15 / 1_000)),
				),
				AssetCreationInput::new_asset(
					CurrencyId(130),
					Some(XcmAssetLocation(MultiLocation {
						parents: 1,
						interior: X3(
							Parachain(topology::common_good_assets::ID),
							PalletInstance(topology::common_good_assets::ASSETS),
							GeneralIndex(topology::common_good_assets::USDT),
						),
					})),
					"Tether".as_bytes().to_vec(),
					"USDT".as_bytes().to_vec(),
					6,
					100,
					Some(rational!(15 / 1_000_000_000)),
				),
				AssetCreationInput::new_asset(
					CurrencyId(5),
					Some(XcmAssetLocation(MultiLocation {
						parents: 0,
						interior: X1(GeneralIndex(5)),
					})),
					"Pablo Token".as_bytes().to_vec(),
					"PBLO".as_bytes().to_vec(),
					12,
					100_000_000_000,
					Some(rational!(1 / 1)),
				),
				AssetCreationInput::new_asset(
					CurrencyId(6),
					None,
					"IBC DOT".as_bytes().to_vec(),
					"ibcDOT".as_bytes().to_vec(),
					12,
					214_300_000,
					None,
				),
			];

			add_assets_to_storage(assets)
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

		mod migrate_asset {

			use super::*;

			#[test]
			fn should_migrate_local_asset() {
				let assets = vec![AssetCreationInput::new_asset(
					CurrencyId(1),
					None,
					"Picasso".as_bytes().to_vec(),
					"PICA".as_bytes().to_vec(),
					12,
					100_000_000_000,
					None,
				)];

				new_test_ext().execute_with(|| {
					add_assets_to_storage(assets);
					assert_eq!(
						<AssetsRegistry as InspectRegistryMetadata>::asset_name(&CurrencyId(1)),
						Some("Picasso".as_bytes().to_vec())
					)
				})
			}
			#[test]
			fn should_migrate_foreign_asset() {
				let assets = vec![AssetCreationInput::new_asset(
					CurrencyId(4),
					Some(XcmAssetLocation(MultiLocation::parent())),
					"Kusama".as_bytes().to_vec(),
					"KSM".as_bytes().to_vec(),
					12,
					37_500_000,
					Some(rational!(375 / 1_000_000)),
				)];

				new_test_ext().execute_with(|| {
					add_assets_to_storage(assets);
					assert_eq!(
						<AssetsRegistry as InspectRegistryMetadata>::asset_name(&CurrencyId(4)),
						Some("Kusama".as_bytes().to_vec())
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
					AssetCreationInput::new_asset(
						CurrencyId(1),
						None,
						"Picasso".as_bytes().to_vec(),
						"PICA".as_bytes().to_vec(),
						12,
						100_000_000_000,
						None,
					),
					AssetCreationInput::new_asset(
						CurrencyId(4),
						Some(XcmAssetLocation(MultiLocation::parent())),
						"Kusama".as_bytes().to_vec(),
						"KSM".as_bytes().to_vec(),
						12,
						37_500_000,
						Some(rational!(375 / 1_000_000)),
					),
					AssetCreationInput::new_asset(
						CurrencyId(105),
						None,
						"Kusama Tether LPT".as_bytes().to_vec(),
						"KSM_USDT_LPT".as_bytes().to_vec(),
						12,
						100,
						None,
					),
					AssetCreationInput::new_asset(
						CurrencyId(106),
						None,
						"Picasso Tether LPT".as_bytes().to_vec(),
						"PICA_USDT_LPT".as_bytes().to_vec(),
						12,
						100,
						None,
					),
					AssetCreationInput::new_asset(
						CurrencyId(107),
						None,
						"Picasso Kusama LPT".as_bytes().to_vec(),
						"PICA_KSM_LPT".as_bytes().to_vec(),
						12,
						100,
						None,
					),
					AssetCreationInput::new_asset(
						CurrencyId(129),
						Some(XcmAssetLocation(MultiLocation {
							parents: 1,
							interior: X2(
								Parachain(topology::karura::ID),
								GeneralKey(WeakBoundedVec::force_from(
									topology::karura::AUSD_KEY.to_vec(),
									None,
								)),
							),
						})),
						"Karura Dollar".as_bytes().to_vec(),
						"kUSD".as_bytes().to_vec(),
						12,
						100_000_000,
						Some(rational!(15 / 1_000)),
					),
					AssetCreationInput::new_asset(
						CurrencyId(130),
						Some(XcmAssetLocation(MultiLocation {
							parents: 1,
							interior: X3(
								Parachain(topology::common_good_assets::ID),
								PalletInstance(topology::common_good_assets::ASSETS),
								GeneralIndex(topology::common_good_assets::USDT),
							),
						})),
						"Tether".as_bytes().to_vec(),
						"USDT".as_bytes().to_vec(),
						6,
						100,
						Some(rational!(15 / 1_000_000_000)),
					),
					AssetCreationInput::new_asset(
						CurrencyId(5),
						Some(XcmAssetLocation(MultiLocation {
							parents: 0,
							interior: X1(GeneralIndex(5)),
						})),
						"Pablo Token".as_bytes().to_vec(),
						"PBLO".as_bytes().to_vec(),
						12,
						100_000_000_000,
						Some(rational!(1 / 1)),
					),
					AssetCreationInput::new_asset(
						CurrencyId(6),
						None,
						"IBC DOT".as_bytes().to_vec(),
						"ibcDOT".as_bytes().to_vec(),
						12,
						214_300_000,
						None,
					),
				];

				new_test_ext().execute_with(|| {
					add_assets_to_storage(assets);
					assert_eq!(
						<AssetsRegistry as InspectRegistryMetadata>::asset_name(&CurrencyId(1)),
						Some("Picasso".as_bytes().to_vec())
					);
					assert_eq!(
						<AssetsRegistry as InspectRegistryMetadata>::asset_name(&CurrencyId(4)),
						Some("Kusama".as_bytes().to_vec())
					);
					assert_eq!(
						<AssetsRegistry as InspectRegistryMetadata>::asset_name(&CurrencyId(105)),
						Some("Kusama Tether LPT".as_bytes().to_vec())
					);
					assert_eq!(
						<AssetsRegistry as InspectRegistryMetadata>::asset_name(&CurrencyId(106)),
						Some("Picasso Tether LPT".as_bytes().to_vec())
					);
					assert_eq!(
						<AssetsRegistry as InspectRegistryMetadata>::asset_name(&CurrencyId(107)),
						Some("Picasso Kusama LPT".as_bytes().to_vec())
					);
					assert_eq!(
						<AssetsRegistry as InspectRegistryMetadata>::asset_name(&CurrencyId(129)),
						Some("Karura Dollar".as_bytes().to_vec())
					);
					assert_eq!(
						<AssetsRegistry as InspectRegistryMetadata>::asset_name(&CurrencyId(130)),
						Some("Tether".as_bytes().to_vec())
					);
					assert_eq!(
						<AssetsRegistry as InspectRegistryMetadata>::asset_name(&CurrencyId(5)),
						Some("Pablo Token".as_bytes().to_vec())
					);
					assert_eq!(
						<AssetsRegistry as InspectRegistryMetadata>::asset_name(&CurrencyId(6)),
						Some("IBC DOT".as_bytes().to_vec())
					);
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
