use crate::*;
use migrate_asset_ids::MigrateComposableAssetIds;

pub type Migrations = (
	SchedulerMigrationV1toV4,
	MigrateComposableAssetIds,
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

pub mod migrate_asset_ids {
	use super::*;
	use frame_support::traits::{GetStorageVersion, StorageVersion};

	use common::AccountId;
	use sp_std::{collections::btree_set::BTreeSet, vec::Vec};

	pub struct MigrateComposableAssetIds;

	fn get_new_asset_id(old_asset_id: CurrencyId) -> CurrencyId {
		let zero_array: [u8; 8] = 0_u64.to_be_bytes();
		let nonce = (u128::from_be_bytes(
			[0; 8]
				.into_iter()
				.chain(zero_array)
				.collect::<Vec<u8>>()
				.try_into()
				.expect("[u8; 8] + bytes(u64) = [u8; 16]"),
		) ^ old_asset_id.0) as u64;

		CurrencyId(u128::from_be_bytes(
			[0, 0, 0, 1]
				.into_iter()
				.chain([0, 0, 0, 0])
				.chain(nonce.to_be_bytes())
				.collect::<Vec<u8>>()
				.try_into()
				.expect("[u8; 8] + bytes(u64) = [u8; 16]"),
		))
	}

	const ASSETS_REGISTRY_V2: StorageVersion = StorageVersion::new(2);
	// ids to change
	const USDT: CurrencyId = CurrencyId(130);
	const PICA: CurrencyId = CurrencyId(1);
	const EQD: CurrencyId = CurrencyId(127);
	const BNC_PLK: CurrencyId = CurrencyId(33);
	const ASTR: CurrencyId = CurrencyId(2006);
	const LAYR: CurrencyId = CurrencyId(2);
	const EQ: CurrencyId = CurrencyId(2011);
	const DOT: CurrencyId = CurrencyId(6);
	const KSM: CurrencyId = CurrencyId(4);
	const V_DOT: CurrencyId = CurrencyId(34);

	/// change asset ids in AR
	fn migrate_assets_registry(old_asset_ids: Vec<CurrencyId>) -> Weight {
		for old_asset_id in old_asset_ids.clone() {
			let new_asset_id = get_new_asset_id(old_asset_id);
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

	fn migrate_orml_tokens(old_asset_ids: Vec<CurrencyId>) -> Weight {
		// orml_tokens TokenIssuance
		for old_asset_id in old_asset_ids.clone() {
			let new_asset_id = get_new_asset_id(old_asset_id);
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
				let new_asset_id = get_new_asset_id(old_asset_id);
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
				let new_asset_id = get_new_asset_id(old_asset_id);
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
				let new_asset_id = get_new_asset_id(old_asset_id);
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

	/// change tx payment asset ids
	fn migrate_tx_payment(old_asset_ids: Vec<CurrencyId>) -> Weight {
		// orml_tokens Accounts storage
		let mut account_ids: Vec<(AccountId, CurrencyId)> = vec![];
		for (account_id, (asset_id, _amount)) in
			asset_tx_payment::pallet::PaymentAssets::<Runtime>::iter()
		{
			if old_asset_ids.contains(&asset_id) {
				account_ids.push((account_id, asset_id));
			}
		}
		for (account_id, asset_id) in account_ids {
			asset_tx_payment::pallet::PaymentAssets::<Runtime>::mutate(account_id, |val| {
				if let Some((asset_id_in, _)) = val {
					*asset_id_in = get_new_asset_id(asset_id)
				}
			});
		}
		Weight::from_ref_time(100_000)
	}

	impl OnRuntimeUpgrade for MigrateComposableAssetIds {
		fn on_runtime_upgrade() -> Weight {
			let on_chain_version =
				<AssetsRegistry as GetStorageVersion>::on_chain_storage_version();
			if on_chain_version < ASSETS_REGISTRY_V2 {
				let asset_ids = vec![USDT, PICA, EQD, LAYR, BNC_PLK, ASTR, EQ, DOT, KSM, V_DOT];
				StorageVersion::new(2).put::<AssetsRegistry>();
				// 4 pallets are affected, system isnt migrated because only native token id needs
				// to be changed
				migrate_orml_tokens(asset_ids.clone()) +
					migrate_assets_registry(asset_ids.clone()) +
					migrate_tx_payment(asset_ids.clone())
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
					get_new_asset_id(LAYR),
					CurrencyId(u128::from_be_bytes([
						0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2
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
