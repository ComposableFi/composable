use crate::{
	migrations::pablo_picasso_init_pools::PabloPicassoInitialPoolsMigration, prelude::*, *,
};
use frame_support::traits::{GetStorageVersion, StorageVersion};

pub type Migrations = (
	SchedulerMigrationV1toV4,
	TechCollectiveRenameMigration,
	PabloPicassoInitialPoolsMigration,
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

pub struct TechCollectiveRenameMigration;

pub fn move_runtime_pallet<
	const OLD_NAME: &'static str,
	const MIGRATED_STORAGE_VERSION: u16,
	NewPallet: PalletInfoAccess + GetStorageVersion,
>() -> Weight {
	let new_pallet_name = <NewPallet as PalletInfoAccess>::name();
	let migrated_storage_version = StorageVersion::new(MIGRATED_STORAGE_VERSION);
	Weight::from_ref_time(
		if new_pallet_name != OLD_NAME &&
			NewPallet::on_chain_storage_version() < migrated_storage_version
		{
			log::info!(target: "migrations", "move_runtime_pallet from {:?} to  {:?} as {:?}", OLD_NAME, new_pallet_name, migrated_storage_version);
			frame_support::storage::migration::move_pallet(
				OLD_NAME.as_bytes(),
				new_pallet_name.as_bytes(),
			);
			migrated_storage_version.put::<NewPallet>();
			// CAUTION: here is conservative estimate for 6 DB read and writes, for big migration
			// should measure and parametrise (this is not the case now)
			100_000_u64
		} else {
			0_u64
		},
	)
}

impl OnRuntimeUpgrade for TechCollectiveRenameMigration {
	fn on_runtime_upgrade() -> Weight {
		move_runtime_pallet::<"TechnicalCollective", 1, TechnicalCommittee>() +
			move_runtime_pallet::<"TechnicalMembership", 1, TechnicalCommitteeMembership>()
	}
}

pub mod pablo_picasso_init_pools {
	use super::*;

	use frame_support::BoundedBTreeMap;
	use pablo::{pallet::PoolInitConfiguration, WeightInfo};
	use sp_runtime::PerThing;

	pub struct PabloPicassoInitialPoolsMigration;

	#[derive(Clone)]
	pub struct PoolCreationInput {
		/// Initial Configuration for the Pool
		init_config: PoolInitConfiguration<AccountId, CurrencyId>,
		/// LP Token for pool to mint
		lp_token: CurrencyId,
	}

	impl PoolCreationInput {
		pub(crate) fn new_two_token_pool(
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
	pub fn add_initial_pools_to_storage(pools: Vec<PoolCreationInput>) -> Weight {
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
		let owner = PabloPalletId::get().into_account_truncating();

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
}

#[cfg(test)]
mod tests {
	use frame_support::{
		assert_storage_noop, sp_io, storage::unhashed, storage_root, StateVersion, StorageHasher,
		Twox128,
	};

	use super::*;

	pub fn new_test_ext() -> sp_io::TestExternalities {
		let storage = frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.expect("in memory test");
		let mut externalities = sp_io::TestExternalities::new(storage);
		externalities.execute_with(|| System::set_block_number(1));
		externalities
	}

	#[test]
	fn migration_v1() {
		new_test_ext().execute_with(|| {
			let mut old_prefix = Twox128::hash(b"TechnicalCollective").to_vec();
			old_prefix.append(&mut Twox128::hash(b"whatever").to_vec());
			unhashed::put_raw(&old_prefix, &[42]);

			let hash_root = storage_root(StateVersion::V1);
			assert_ne!(
				TechCollectiveRenameMigration::on_runtime_upgrade(),
				Weight::from_ref_time(0)
			);
			assert_ne!(hash_root, storage_root(StateVersion::V1));
			let updated = || {
				assert_eq!(
					TechCollectiveRenameMigration::on_runtime_upgrade(),
					Weight::from_ref_time(0)
				)
			};
			assert_storage_noop!(updated());

			assert!(unhashed::get_raw(&old_prefix).is_none());
		});
	}

	mod add_initial_pools_to_storage {
		use crate::migrations::pablo_picasso_init_pools::{
			add_initial_pools_to_storage, PoolCreationInput,
		};
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
