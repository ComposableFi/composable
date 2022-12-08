use frame_support::traits::{GetStorageVersion, StorageVersion};

use crate::{prelude::*, *};

pub type Migrations = (SchedulerMigrationV3, TechCollectiveRenameMigration);

// Migration for scheduler pallet to move from a plain Call to a CallOrHash.
pub struct SchedulerMigrationV3;
impl OnRuntimeUpgrade for SchedulerMigrationV3 {
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		Scheduler::migrate_v2_to_v3()
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
	if new_pallet_name != OLD_NAME &&
		NewPallet::on_chain_storage_version() < migrated_storage_version
	{
		log::info!(target: "migrations", "move_runtime_pallet from {:?} to  {:?} as {:?}", OLD_NAME, new_pallet_name, migrated_storage_version);
		frame_support::storage::migration::move_pallet(
			OLD_NAME.as_bytes(),
			new_pallet_name.as_bytes(),
		);
		migrated_storage_version.put::<NewPallet>();
		// CAUTION: here is conservative estimate for 6 DB read and writes, for big migration should
		// measure and parametrise (this is not the case now)
		return 100_000_u64
	}

	0_u64
}

impl OnRuntimeUpgrade for TechCollectiveRenameMigration {
	fn on_runtime_upgrade() -> Weight {
		move_runtime_pallet::<"TechnicalCollective", 1, TechnicalCommittee>() +
			move_runtime_pallet::<"TechnicalMembership", 1, TechnicalCommitteeMembership>()
	}
}

mod pablo_picasso_init_pools {

	use super::*;

	use frame_support::BoundedBTreeMap;
	use pablo::pallet::PoolInitConfiguration;
	use sp_runtime::PerThing;

	pub struct PabloPicassoInitialPoolsMigration;

	fn add_initial_pools_to_storage() -> Weight {
		Pablo::do_create_pool(
			create_two_token_pool_config(
				CurrencyId::KSM,
				CurrencyId::USDT,
				Permill::from_percent(50),
			),
			Some(CurrencyId::KSM_USDT_LPT),
		)
		.expect("Is a valid pool config; QED");

		Pablo::do_create_pool(
			create_two_token_pool_config(
				CurrencyId::PICA,
				CurrencyId::USDT,
				Permill::from_percent(50),
			),
			Some(CurrencyId::PICA_USDT_LPT),
		)
		.expect("Is a valid pool config; QED");
		// TODO(connor) calculate weight based of read/write count
		0
	}

	fn create_two_token_pool_config(
		base_asset_id: CurrencyId,
		quote_asset_id: CurrencyId,
		base_asset_weigth: Permill,
	) -> PoolInitConfiguration<AccountId, CurrencyId> {
		let owner = AccountId::from([0; 32]);
		let mut assets_weights = BoundedBTreeMap::new();
		let fee = Permill::from_percent(0);

		assets_weights
			.try_insert(base_asset_id, base_asset_weigth)
			.expect("Within BT Bounds; QED");
		assets_weights
			.try_insert(quote_asset_id, base_asset_weigth.left_from_one())
			.expect("Within BT Bounds; QED");

		PoolInitConfiguration::<AccountId, CurrencyId>::DualAssetConstantProduct {
			owner,
			assets_weights,
			fee,
		}
	}

	// TODO(connor): Add impl methods
	impl OnRuntimeUpgrade for PabloPicassoInitialPoolsMigration {}
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
			assert_ne!(TechCollectiveRenameMigration::on_runtime_upgrade(), 0);
			assert_ne!(hash_root, storage_root(StateVersion::V1));
			let updated = || assert_eq!(TechCollectiveRenameMigration::on_runtime_upgrade(), 0);
			assert_storage_noop!(updated());

			assert!(unhashed::get_raw(&old_prefix).is_none());
		});
	}
}
