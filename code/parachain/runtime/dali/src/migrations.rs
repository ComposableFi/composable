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

	use frame_support::bounded_btree_map;
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

		#[allow(clippy::disallowed_methods)] // BTree size is within bounds
		let assets_weights = bounded_btree_map! {
			first_asset_id => first_asset_weight,
			second_asset_id => first_asset_weight.left_from_one(),
		};

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
			];

			add_initial_pools_to_storage(pools)
		}
	}
}
