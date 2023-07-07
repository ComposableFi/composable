use crate::{prelude::*, *};
use frame_support::traits::{GetStorageVersion, StorageVersion};
use migrate_asset_ids::MigratePicassoAssetIds;

pub type Migrations = (
	SchedulerMigrationV1toV4,
	TechCollectiveRenameMigration,
	MigratePicassoAssetIds,
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

pub mod migrate_asset_ids {
	use super::*;
	use frame_support::traits::{GetStorageVersion, StorageVersion};
	use primitives::currency::{ForeignAssetId, PrefixedDenom};

	use common::AccountId;
	use pablo::PoolConfiguration;
	use sp_std::{collections::btree_set::BTreeSet, vec::Vec};

	pub struct MigratePicassoAssetIds;

	fn get_new_asset_id(
		old_asset_id: CurrencyId,
		network_id: [u8; 4],
		index: [u8; 4],
	) -> CurrencyId {
		let zero_array: [u8; 8] = 0_u64.to_be_bytes();
		let nonce = (u128::from_be_bytes(
			<Runtime as pablo::Config>::PalletId::get()
				.0
				.into_iter()
				.chain(zero_array)
				.collect::<Vec<u8>>()
				.try_into()
				.expect("[u8; 8] + bytes(u64) = [u8; 16]"),
		) ^ old_asset_id.0) as u64;

		CurrencyId(u128::from_be_bytes(
			network_id
				.into_iter()
				.chain(index)
				.chain(nonce.to_be_bytes())
				.collect::<Vec<u8>>()
				.try_into()
				.expect("[u8; 8] + bytes(u64) = [u8; 16]"),
		))
	}

	fn get_new_asset_id_picasso(old_asset_id: CurrencyId) -> CurrencyId {
		let pablo_index = (Pablo::index() as u32).to_be_bytes();
		get_new_asset_id(old_asset_id, [0, 0, 0, 0], pablo_index)
	}

	fn get_new_asset_id_composable(old_asset_id: CurrencyId) -> CurrencyId {
		get_new_asset_id(old_asset_id, [0, 0, 0, 1], [0, 0, 0, 0])
	}

	const ASSETS_REGISTRY_V2: StorageVersion = StorageVersion::new(2);
	// ids to change
	const DOT_PICA_LPT: CurrencyId = CurrencyId(149379386384882397174193330044887105538);
	const DOT_USDT_LPT: CurrencyId = CurrencyId(149379386384882397174193330044887105539);
	const DOT_KSM_LPT: CurrencyId = CurrencyId(149379386384882397174193330044887105540);
	const DOT_OSMO_LPT: CurrencyId = CurrencyId(149379386384882397174193330044887105541);
	const KSM_OSMO_LPT: CurrencyId = CurrencyId(149379386384882397174193330044887105542);
	const USDT_OSMO_LPT: CurrencyId = CurrencyId(149379386384882397174193330044887105543);

	/// migrate orml_tokens storage
	fn migrate_orml_tokens(old_asset_ids: Vec<CurrencyId>) -> Weight {
		// orml_tokens TokenIssuance
		for old_asset_id in old_asset_ids.clone() {
			let new_asset_id = get_new_asset_id_picasso(old_asset_id);
			orml_tokens::module::TotalIssuance::<Runtime>::swap(old_asset_id, new_asset_id);
			orml_tokens::module::TotalIssuance::<Runtime>::remove(old_asset_id);
		}

		// orml_tokens Accounts storage
		let mut account_ids: BTreeSet<AccountId> = BTreeSet::new();
		for (account_id, _) in orml_tokens::module::Accounts::<Runtime>::iter_keys() {
			account_ids.insert(account_id);
		}
		for account_id in account_ids {
			for old_asset_id in old_asset_ids.clone() {
				let new_asset_id = get_new_asset_id_picasso(old_asset_id);
				orml_tokens::module::Accounts::<Runtime>::swap(
					account_id.clone(),
					old_asset_id,
					account_id.clone(),
					new_asset_id,
				);
				orml_tokens::module::Accounts::<Runtime>::remove(account_id.clone(), old_asset_id);
			}
		}

		// orml_tokens Locks storage
		let mut account_ids: BTreeSet<AccountId> = BTreeSet::new();
		for (account_id, _) in orml_tokens::module::Locks::<Runtime>::iter_keys() {
			account_ids.insert(account_id);
		}
		for account_id in account_ids {
			for old_asset_id in old_asset_ids.clone() {
				let new_asset_id = get_new_asset_id_picasso(old_asset_id);
				orml_tokens::module::Locks::<Runtime>::swap(
					account_id.clone(),
					old_asset_id,
					account_id.clone(),
					new_asset_id,
				);
				orml_tokens::module::Locks::<Runtime>::remove(account_id.clone(), old_asset_id);
			}
		}

		// orml_tokens Reserves storage
		let mut account_ids: BTreeSet<AccountId> = BTreeSet::new();
		for (account_id, _) in orml_tokens::module::Reserves::<Runtime>::iter_keys() {
			account_ids.insert(account_id);
		}
		for account_id in account_ids {
			for old_asset_id in old_asset_ids.clone() {
				let new_asset_id = get_new_asset_id_picasso(old_asset_id);
				orml_tokens::module::Reserves::<Runtime>::swap(
					account_id.clone(),
					old_asset_id,
					account_id.clone(),
					new_asset_id,
				);
				orml_tokens::module::Reserves::<Runtime>::remove(account_id.clone(), old_asset_id);
			}
		}
		Weight::from_ref_time(100_000)
	}

	/// change some of the pools' lp token ids
	fn migrate_pablo_pools(old_asset_ids: Vec<CurrencyId>) -> Weight {
		pablo::pallet::Pools::<Runtime>::translate_values(|pool| match pool {
			PoolConfiguration::DualAssetConstantProduct(mut config) => {
				if old_asset_ids.contains(&config.lp_token) {
					config.lp_token = get_new_asset_id_picasso(config.lp_token);
				}
				Some(PoolConfiguration::DualAssetConstantProduct(config))
			},
		});
		Weight::from_ref_time(100_000)
	}

	/// change staking storage for staked lp tokens
	fn migrate_staking(old_asset_ids: Vec<CurrencyId>) -> Weight {
		// farming pallet RewardSchedules
		let mut asset_ids: BTreeSet<(CurrencyId, CurrencyId)> = BTreeSet::new();
		for (asset_id, reward_id) in farming::pallet::RewardSchedules::<Runtime>::iter_keys() {
			asset_ids.insert((asset_id, reward_id));
		}
		for (old_asset_id, reward_id) in asset_ids {
			if old_asset_ids.clone().contains(&old_asset_id) {
				let new_asset_id = get_new_asset_id_picasso(old_asset_id);
				farming::pallet::RewardSchedules::<Runtime>::swap(
					old_asset_id.clone(),
					reward_id.clone(),
					new_asset_id.clone(),
					reward_id.clone(),
				);
				farming::pallet::RewardSchedules::<Runtime>::remove(
					old_asset_id.clone(),
					reward_id.clone(),
				);
			}
		}

		// reward pallet RewardCurrencies
		let mut asset_ids: BTreeSet<CurrencyId> = BTreeSet::new();
		for asset_id in
			reward::pallet::RewardCurrencies::<Runtime, FarmingRewardsInstance>::iter_keys()
		{
			asset_ids.insert(asset_id);
		}
		for old_asset_id in asset_ids {
			if old_asset_ids.clone().contains(&old_asset_id) {
				let new_asset_id = get_new_asset_id_picasso(old_asset_id);
				reward::pallet::RewardCurrencies::<Runtime, FarmingRewardsInstance>::swap(
					old_asset_id.clone(),
					new_asset_id.clone(),
				);
				reward::pallet::RewardCurrencies::<Runtime, FarmingRewardsInstance>::remove(
					old_asset_id.clone(),
				);
			}
		}

		// reward pallet TotalStake
		let mut asset_ids: BTreeSet<CurrencyId> = BTreeSet::new();
		for asset_id in reward::pallet::TotalStake::<Runtime, FarmingRewardsInstance>::iter_keys() {
			asset_ids.insert(asset_id);
		}
		for old_asset_id in asset_ids {
			if old_asset_ids.clone().contains(&old_asset_id) {
				let new_asset_id = get_new_asset_id_picasso(old_asset_id);
				reward::pallet::TotalStake::<Runtime, FarmingRewardsInstance>::swap(
					old_asset_id.clone(),
					new_asset_id.clone(),
				);
				reward::pallet::TotalStake::<Runtime, FarmingRewardsInstance>::remove(
					old_asset_id.clone(),
				);
			}
		}

		// reward pallet Stake
		let mut asset_ids: BTreeSet<(CurrencyId, AccountId)> = BTreeSet::new();
		for (asset_id, user_id) in
			reward::pallet::Stake::<Runtime, FarmingRewardsInstance>::iter_keys()
		{
			asset_ids.insert((asset_id, user_id));
		}
		for (old_asset_id, user_id) in asset_ids {
			if old_asset_ids.clone().contains(&old_asset_id) {
				let new_asset_id = get_new_asset_id_picasso(old_asset_id);
				reward::pallet::Stake::<Runtime, FarmingRewardsInstance>::swap(
					(old_asset_id.clone(), user_id.clone()),
					(new_asset_id.clone(), user_id.clone()),
				);
				reward::pallet::Stake::<Runtime, FarmingRewardsInstance>::remove((
					old_asset_id.clone(),
					user_id.clone(),
				));
			}
		}

		// reward pallet RewardPerToken
		let mut keys: BTreeSet<(CurrencyId, CurrencyId)> = BTreeSet::new();
		for (reward_id, asset_id) in
			reward::pallet::RewardPerToken::<Runtime, FarmingRewardsInstance>::iter_keys()
		{
			keys.insert((reward_id, asset_id));
		}
		for (reward_id, old_asset_id) in keys {
			if old_asset_ids.clone().contains(&old_asset_id) {
				let new_asset_id: CurrencyId = get_new_asset_id_picasso(old_asset_id);
				reward::pallet::RewardPerToken::<Runtime, FarmingRewardsInstance>::swap(
					reward_id.clone(),
					old_asset_id.clone(),
					reward_id.clone(),
					new_asset_id.clone(),
				);
				reward::pallet::RewardPerToken::<Runtime, FarmingRewardsInstance>::remove(
					reward_id.clone(),
					old_asset_id.clone(),
				);
			}
		}

		// reward pallet RewardTally
		let mut keys: BTreeSet<(CurrencyId, (CurrencyId, AccountId))> = BTreeSet::new();
		for (reward_id, (asset_id, account_id)) in
			reward::pallet::RewardTally::<Runtime, FarmingRewardsInstance>::iter_keys()
		{
			keys.insert((reward_id, (asset_id, account_id)));
		}
		for (reward_id, (old_asset_id, account_id)) in keys {
			if old_asset_ids.clone().contains(&old_asset_id) {
				let new_asset_id: CurrencyId = get_new_asset_id_picasso(old_asset_id);
				reward::pallet::RewardTally::<Runtime, FarmingRewardsInstance>::swap(
					reward_id.clone(),
					(old_asset_id.clone(), account_id.clone()),
					reward_id.clone(),
					(new_asset_id.clone(), account_id.clone()),
				);
				reward::pallet::RewardTally::<Runtime, FarmingRewardsInstance>::remove(
					reward_id.clone(),
					(old_asset_id.clone(), account_id.clone()),
				);
			}
		}

		Weight::from_ref_time(100_000)
	}

	/// change some of the pools' lp token ids
	fn migrate_assets_registry(old_asset_ids: Vec<CurrencyId>) -> Weight {
		for old_asset_id in old_asset_ids.clone() {
			let new_asset_id = get_new_asset_id_picasso(old_asset_id);
			assets_registry::pallet::AssetRatio::<Runtime>::swap(old_asset_id, new_asset_id);
			assets_registry::pallet::ExistentialDeposit::<Runtime>::swap(
				old_asset_id,
				new_asset_id,
			);
			assets_registry::pallet::AssetName::<Runtime>::swap(old_asset_id, new_asset_id);
			assets_registry::pallet::AssetName::<Runtime>::remove(old_asset_id);
			assets_registry::pallet::AssetSymbol::<Runtime>::swap(old_asset_id, new_asset_id);
			assets_registry::pallet::AssetSymbol::<Runtime>::remove(old_asset_id);
			assets_registry::pallet::AssetDecimals::<Runtime>::swap(old_asset_id, new_asset_id);
			assets_registry::pallet::AssetDecimals::<Runtime>::remove(old_asset_id);
		}
		Weight::from_ref_time(100_000)
	}

	// migration of pallet's storage
	fn migrate_asset_ids(old_asset_ids: Vec<CurrencyId>) -> Weight {
		let mut total_weight = Weight::from_ref_time(0);
		total_weight += migrate_orml_tokens(old_asset_ids.clone());
		total_weight += migrate_pablo_pools(old_asset_ids.clone());
		total_weight += migrate_staking(old_asset_ids.clone());
		total_weight += migrate_assets_registry(old_asset_ids.clone());
		total_weight
	}

	// pica balances of staking accounts migration
	fn migrate_balances(asset_ids: Vec<CurrencyId>) -> Weight {
		for old_asset_id in asset_ids {
			let old_asset_id_balance = farming::Pallet::<Runtime>::pool_account_id(&old_asset_id);
			let new_asset_id = get_new_asset_id_picasso(old_asset_id);
			let new_asset_id_balance = farming::Pallet::<Runtime>::pool_account_id(&new_asset_id);
			system::pallet::Account::<Runtime>::swap(
				old_asset_id_balance.clone(),
				new_asset_id_balance,
			);
			system::pallet::Account::<Runtime>::remove(old_asset_id_balance);
		}
		Weight::from_ref_time(100_000)
	}

	// migrate denoms of composable assets
	fn migrate_composable_denoms(asset_ids: Vec<CurrencyId>) -> Weight {
		let mut locations: Vec<ForeignAssetId> = vec![];
		for (old_location, asset_id) in assets_registry::pallet::ForeignToLocal::<Runtime>::iter() {
			if asset_ids.contains(&asset_id) {
				locations.push(old_location.clone());
			}
		}
		for old_location in locations {
			assets_registry::pallet::ForeignToLocal::<Runtime>::remove(old_location);
		}

		for old_asset_id in asset_ids.clone() {
			let new_asset_id = get_new_asset_id_composable(old_asset_id);
			let old_location_option =
				assets_registry::pallet::LocalToForeign::<Runtime>::take(old_asset_id);

			if let Some(ForeignAssetId::IbcIcs20(location)) = old_location_option {
				if let Ok(mut denom) = PrefixedDenom::from_str(&new_asset_id.to_string()) {
					denom.0.trace_path = location.trace_path.clone();
					let new_location = ForeignAssetId::IbcIcs20(denom);
					assets_registry::pallet::ForeignToLocal::<Runtime>::insert(
						&new_location,
						old_asset_id,
					);
					assets_registry::pallet::LocalToForeign::<Runtime>::insert(
						old_asset_id,
						new_location,
					);
				}
			}
		}

		Weight::from_ref_time(100_000)
	}

	impl OnRuntimeUpgrade for MigratePicassoAssetIds {
		fn on_runtime_upgrade() -> Weight {
			let on_chain_version =
				<AssetsRegistry as GetStorageVersion>::on_chain_storage_version();
			if on_chain_version < ASSETS_REGISTRY_V2 {
				let asset_ids = vec![
					DOT_PICA_LPT,
					DOT_USDT_LPT,
					DOT_KSM_LPT,
					DOT_OSMO_LPT,
					KSM_OSMO_LPT,
					USDT_OSMO_LPT,
				];

				let asset_ids_with_pica_balance = vec![DOT_PICA_LPT, DOT_USDT_LPT, DOT_KSM_LPT];
				let composable_asset_ids = vec![
					CurrencyId(127),
					CurrencyId(33),
					CurrencyId(2006),
					CurrencyId(2011),
					CurrencyId(6),
					CurrencyId(34),
				];
				StorageVersion::new(2).put::<AssetsRegistry>();
				migrate_asset_ids(asset_ids) +
					migrate_balances(asset_ids_with_pica_balance) +
					migrate_composable_denoms(composable_asset_ids)
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

		mod migrate_asset {

			use super::*;

			#[test]
			fn check_ids_correct() {
				assert_eq!(
					get_new_asset_id_picasso(DOT_PICA_LPT),
					CurrencyId(u128::from_be_bytes([
						0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 2
					]))
				);
				assert_eq!(
					get_new_asset_id_picasso(DOT_USDT_LPT),
					CurrencyId(u128::from_be_bytes([
						0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 3
					]))
				);
				assert_eq!(
					get_new_asset_id_picasso(DOT_KSM_LPT),
					CurrencyId(u128::from_be_bytes([
						0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 4
					]))
				);
				assert_eq!(
					get_new_asset_id_picasso(DOT_OSMO_LPT),
					CurrencyId(u128::from_be_bytes([
						0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 5
					]))
				);
				assert_eq!(
					get_new_asset_id_picasso(KSM_OSMO_LPT),
					CurrencyId(u128::from_be_bytes([
						0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 6
					]))
				);
				assert_eq!(
					get_new_asset_id_picasso(USDT_OSMO_LPT),
					CurrencyId(u128::from_be_bytes([
						0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 7
					]))
				);
				assert_eq!(
					get_new_asset_id_composable(CurrencyId(130)),
					CurrencyId(u128::from_be_bytes([
						0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 130
					]))
				);
			}

			#[test]
			fn run_scenarios() {
				new_test_ext().execute_with(|| {
					// todo
					// create & mint asset 1
					// create & mint asset 2
					// create pool
					// add liquidity
					// stake some of lp tokens
					// wait for rewards
					// execute migration functions and check that old values have new keys and old
					// keys are removed
				})
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
