use crate::{prelude::*, *};
use frame_support::traits::{GetStorageVersion, StorageVersion};
use migrate_gov::MigrateGov;

parameter_types! {
	pub const DemocracyPalletName: &'static str = "Democracy";
	pub const OpenGovPalletName: &'static str = "OpenGovBalances";
}

pub type Migrations = (
	SchedulerMigrationV1toV4,
	TechCollectiveRenameMigration,
	MigrateGov,
	preimage::migration::v1::Migration<Runtime>,
	scheduler::migration::v3::MigrateToV4<Runtime>,
	multisig::migrations::v1::MigrateToV1<Runtime>,
	vesting::migrations::VestingV0ToV1<Runtime>,
	frame_support::migrations::RemovePallet<
		DemocracyPalletName,
		<Runtime as frame_system::Config>::DbWeight,
	>,
	frame_support::migrations::RemovePallet<
		OpenGovPalletName,
		<Runtime as frame_system::Config>::DbWeight,
	>,
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
	Weight::from_parts(
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
		0,
	)
}

impl OnRuntimeUpgrade for TechCollectiveRenameMigration {
	fn on_runtime_upgrade() -> Weight {
		move_runtime_pallet::<"TechnicalCollective", 1, TechnicalCommittee>() +
			move_runtime_pallet::<"TechnicalMembership", 1, TechnicalCommitteeMembership>()
	}
}

pub mod migrate_gov {
	use super::*;
	use frame_support::traits::{GetStorageVersion, StorageVersion};

	use frame_support::traits::LockableCurrency;
	use hex_literal::hex;
	use sp_runtime::AccountId32;
	pub struct MigrateGov;

	const REFERENDA_V2: StorageVersion = StorageVersion::new(2);
	const DEMOCRACY_ID: LockIdentifier = *b"democrac";

	fn migrate() -> Weight {
		// set key for relayer committee
		let relayer_address = sp_runtime::MultiAddress::Id(AccountId32::from(hex!(
			"868232e15789eaae263d655db7d222fcf5ffa5f6f8da4e46d32609312fcf6e60"
		)));
		let _ = membership::pallet::Pallet::<Runtime, NativeRelayerMembership>::add_member(
			frame_system::RawOrigin::Root.into(),
			relayer_address.clone(),
		);
		let _ = membership::pallet::Pallet::<Runtime, NativeTechnicalMembership>::remove_member(
			frame_system::RawOrigin::Root.into(),
			relayer_address,
		);
		let accounts = balances::pallet::Locks::<Runtime>::iter()
			.filter(|(_key, locks)| locks.iter().any(|a| a.id == DEMOCRACY_ID))
			.map(|(key, _locks)| key)
			.collect::<Vec<_>>();

		for account in accounts {
			<Balances as LockableCurrency<AccountId>>::remove_lock(DEMOCRACY_ID, &account);
		}
		Weight::from_parts(100_000, 0)
	}

	impl OnRuntimeUpgrade for MigrateGov {
		fn on_runtime_upgrade() -> Weight {
			let on_chain_version = <Referenda as GetStorageVersion>::on_chain_storage_version();
			if on_chain_version < REFERENDA_V2 {
				StorageVersion::new(2).put::<Referenda>();
				migrate()
			} else {
				<Runtime as system::Config>::DbWeight::get().reads(1)
			}
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
				Weight::from_parts(0, 0)
			);
			assert_ne!(hash_root, storage_root(StateVersion::V1));
			let updated = || {
				assert_eq!(
					TechCollectiveRenameMigration::on_runtime_upgrade(),
					Weight::from_parts(0, 0)
				)
			};
			assert_storage_noop!(updated());

			assert!(unhashed::get_raw(&old_prefix).is_none());
		});
	}
}
