use crate::{prelude::*, *};
use frame_support::traits::{GetStorageVersion, StorageVersion};

pub type Migrations = (
	SchedulerMigrationV1toV4,
	TechCollectiveRenameMigration,
	preimage::migration::v1::Migration<Runtime>,
	scheduler::migration::v3::MigrateToV4<Runtime>,
	democracy::migrations::v1::Migration<Runtime>,
	multisig::migrations::v1::MigrateToV1<Runtime>,
	vesting::migrations::VestingV0ToV1<Runtime>,
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
}
