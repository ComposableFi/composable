use crate::{prelude::*, *};

use pablo_picasso_init_pools::PabloPicassoInitialPoolsMigration;

pub type Migrations = (PabloPicassoInitialPoolsMigration, SchedulerMigrationV3);

// Migration for scheduler pallet to move from a plain Call to a CallOrHash.
pub struct SchedulerMigrationV3;
impl OnRuntimeUpgrade for SchedulerMigrationV3 {
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		Scheduler::migrate_v2_to_v3()
	}
}

pub mod pablo_picasso_init_pools {

	use super::*;

	use frame_support::BoundedBTreeMap;
	use frame_support::traits::OnGenesis;
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
			return 0
		}

		pools.iter().for_each(|pool_creation_input| {
			Pablo::do_create_pool(
				pool_creation_input.init_config.to_owned(),
				Some(pool_creation_input.lp_token),
			)
			.expect("Pool config is valid; QED");
		});

		weights::pablo::WeightInfo::<Runtime>::do_create_pool() * pools.len() as Weight
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

	// This is only for testing purposes for Pablo as otherwise
	// pools are not created without a runtime upgrade.
	impl OnGenesis for PabloPicassoInitialPoolsMigration {
		fn on_genesis() {
			Self::initial_pools();
		}
	}

	impl OnRuntimeUpgrade for PabloPicassoInitialPoolsMigration {
		fn on_runtime_upgrade() -> Weight {
			Self::initial_pools()
		}
	}

	impl PabloPicassoInitialPoolsMigration {
		fn initial_pools() -> Weight {
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
