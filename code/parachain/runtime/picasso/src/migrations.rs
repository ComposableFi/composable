use crate::{prelude::*, *};
use frame_support::traits::{GetStorageVersion, StorageVersion};
use migrate_oracle::MigrateOracle;

pub type Migrations = (
	SchedulerMigrationV1toV4,
	TechCollectiveRenameMigration,
	MigrateOracle,
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

pub mod migrate_oracle {
	use super::*;
	use frame_support::traits::{GetStorageVersion, StorageVersion};

	use sp_std::vec::Vec;

	pub struct MigrateOracle;

	const ORACLE_V2: StorageVersion = StorageVersion::new(2);

	/// change some of the pools' lp token ids
	fn migrate_oracle() -> Weight {
		let mut answers_accounts: Vec<_> = Vec::new();
		for account in oracle::pallet::AnswerInTransit::<Runtime>::iter_keys() {
			answers_accounts.push(account);
		}

		let mut preprices_assets: Vec<_> = Vec::new();
		for asset in oracle::pallet::PrePrices::<Runtime>::iter_keys() {
			preprices_assets.push(asset);
		}

		for account in answers_accounts {
			oracle::pallet::AnswerInTransit::<Runtime>::remove(account.clone());
		}

		for asset in preprices_assets {
			oracle::pallet::PrePrices::<Runtime>::remove(asset.clone());
		}

		Weight::from_ref_time(100_000)
	}

	impl OnRuntimeUpgrade for MigrateOracle {
		fn on_runtime_upgrade() -> Weight {
			let on_chain_version = <Oracle as GetStorageVersion>::on_chain_storage_version();
			if on_chain_version < ORACLE_V2 {
				StorageVersion::new(2).put::<Oracle>();
				migrate_oracle()
			} else {
				<Runtime as system::Config>::DbWeight::get().reads(1)
			}
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

		mod migrate_oracle {

			use super::*;
			use common::AccountId;
			use oracle::PrePrice;
			use sp_core::crypto::AccountId32;
			#[test]
			fn test_migrate_oracle() {
				new_test_ext().execute_with(|| {
					let alice: AccountId = AccountId32::new([
						0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
						0, 0, 0, 0, 0, 0, 1,
					]);
					let bob: AccountId = AccountId32::new([
						0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
						0, 0, 0, 0, 0, 0, 2,
					]);
					let usdt = CurrencyId(130);
					let usdc = CurrencyId(131);
					oracle::pallet::AnswerInTransit::<Runtime>::mutate(&alice, |transit| {
						*transit = Some(2000000000000);
					});
					oracle::pallet::AnswerInTransit::<Runtime>::mutate(&bob, |transit| {
						*transit = Some(20000000000000);
					});
					assert_eq!(
						oracle::pallet::AnswerInTransit::<Runtime>::get(&alice),
						Some(2000000000000)
					);
					assert_eq!(
						oracle::pallet::AnswerInTransit::<Runtime>::get(&bob),
						Some(20000000000000)
					);

					oracle::pallet::PrePrices::<Runtime>::try_mutate(
						&usdt,
						|current_prices| -> Result<(), DispatchError> {
							current_prices
								.try_push(PrePrice { price: 1, who: bob.clone(), block: 0 })
								.unwrap();
							current_prices
								.try_push(PrePrice { price: 2, who: alice.clone(), block: 1 })
								.unwrap();

							Ok(())
						},
					)
					.unwrap();

					oracle::pallet::PrePrices::<Runtime>::try_mutate(
						&usdc,
						|current_prices| -> Result<(), DispatchError> {
							current_prices
								.try_push(PrePrice { price: 20, who: bob.clone(), block: 1 })
								.unwrap();
							current_prices
								.try_push(PrePrice { price: 10, who: alice.clone(), block: 0 })
								.unwrap();

							Ok(())
						},
					)
					.unwrap();

					assert_eq!(
						oracle::pallet::PrePrices::<Runtime>::get(&usdt)
							.into_iter()
							.collect::<Vec<PrePrice<Balance, BlockNumber, AccountId>>>(),
						vec![
							PrePrice { price: 1, block: 0, who: bob.clone() },
							PrePrice { price: 2, block: 1, who: alice.clone() }
						]
					);
					assert_eq!(
						oracle::pallet::PrePrices::<Runtime>::get(&usdc)
							.into_iter()
							.collect::<Vec<PrePrice<Balance, BlockNumber, AccountId>>>(),
						vec![
							PrePrice { price: 20, block: 1, who: bob.clone() },
							PrePrice { price: 10, block: 0, who: alice.clone() }
						]
					);
					migrate_oracle();
					assert_eq!(oracle::pallet::AnswerInTransit::<Runtime>::get(&alice), None);
					assert_eq!(oracle::pallet::AnswerInTransit::<Runtime>::get(&bob), None);
					assert_eq!(
						oracle::pallet::PrePrices::<Runtime>::get(&usdt)
							.into_iter()
							.collect::<Vec<PrePrice<Balance, BlockNumber, AccountId>>>(),
						vec![]
					);
					assert_eq!(
						oracle::pallet::PrePrices::<Runtime>::get(&usdc)
							.into_iter()
							.collect::<Vec<PrePrice<Balance, BlockNumber, AccountId>>>(),
						vec![]
					);
				});
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
