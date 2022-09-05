use composable_traits::{instrumental::InstrumentalProtocolStrategy, vault::Vault as VaultTrait};
use frame_support::{
	assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use primitives::currency::CurrencyId;
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, Hash},
	Percent, Permill, Perquintill,
};
use sp_std::collections::btree_set::BTreeSet;

use crate::{
	mock::{
		account_id::{ADMIN, ALICE, BOB},
		helpers::{assert_has_event, create_pool, create_vault, make_proposal, set_admin_members},
		runtime::{
			Balance, Call, Event, ExtBuilder, MockRuntime, PabloStrategy, System, Tokens, Vault,
			VaultId, MAX_ASSOCIATED_VAULTS,
		},
	},
	pallet,
};

// -------------------------------------------------------------------------------------------------
//                                          Associate Vault
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod associate_vault {
	use super::*;

	#[test]
	fn add_an_associated_vault() {
		ExtBuilder::default().build().execute_with(|| {
			System::set_block_number(1);
			let base_asset = CurrencyId::LAYR;
			let vault_id = create_vault(base_asset, None);
			set_admin_members(vec![ALICE], 5);
			let proposal = Call::PabloStrategy(crate::Call::associate_vault { vault_id });
			make_proposal(proposal, ALICE, 1, 0, None);
			System::assert_has_event(Event::PabloStrategy(pallet::Event::AssociatedVault {
				vault_id,
			}));
		});
	}

	#[test]
	fn adding_an_associated_vault_twice_throws_an_error() {
		ExtBuilder::default().build().execute_with(|| {
			System::set_block_number(1);
			let base_asset = CurrencyId::LAYR;
			let vault_id = create_vault(base_asset, None);
			set_admin_members(vec![ALICE], 5);
			let proposal_1 = Call::PabloStrategy(crate::Call::associate_vault { vault_id });
			make_proposal(proposal_1, ALICE, 1, 0, None);

			let proposal_2 = Call::PabloStrategy(crate::Call::associate_vault { vault_id });
			let hash: H256 = BlakeTwo256::hash_of(&proposal_2);
			make_proposal(proposal_2, ALICE, 1, 1, None);
			// Check that last proposal completed with error, since we are trying to add the same Vault
			assert_has_event::<MockRuntime, _>(
				|e| matches!(
					e.event,
					Event::CollectiveInstrumental(pallet_collective::Event::Executed { proposal_hash, .. }) if proposal_hash == hash),
			);
			let mut correct_associated_vaults_storage = BTreeSet::new();
			correct_associated_vaults_storage.insert(vault_id);
			assert_eq!(
				BTreeSet::from(PabloStrategy::associated_vaults()),
				correct_associated_vaults_storage
			);
		});
	}

	#[test]
	fn associating_too_many_vaults_throws_an_error() {
		ExtBuilder::default().build().execute_with(|| {
			System::set_block_number(1);
			set_admin_members(vec![ALICE], 5);
			for vault_id in 0..MAX_ASSOCIATED_VAULTS {
				let proposal =
					Call::PabloStrategy(crate::Call::associate_vault { vault_id: vault_id as u64 });
				make_proposal(proposal, ALICE, 1, 0, None);
			}

			let vault_id = MAX_ASSOCIATED_VAULTS as VaultId;
			let proposal = Call::PabloStrategy(crate::Call::associate_vault { vault_id });
			let hash: H256 = BlakeTwo256::hash_of(&proposal);
			make_proposal(proposal, ALICE, 1, 0, None);
			// Check that last proposal completed with error, since we are trying to add more Vaults than allowed
			assert_has_event::<MockRuntime, _>(
				|e| matches!(
					e.event,
					Event::CollectiveInstrumental(pallet_collective::Event::Executed { proposal_hash, .. }) if proposal_hash == hash),
			);
			assert!(!BTreeSet::from(PabloStrategy::associated_vaults()).contains(&vault_id));
		});
	}
}

// -------------------------------------------------------------------------------------------------
//                                             Rebalance
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod rebalance {
	use super::*;

	#[test]
	fn rebalance_emits_event() {
		ExtBuilder::default().build().execute_with(|| {
			System::set_block_number(1);
			let base_asset = CurrencyId::LAYR;
			let quote_asset = CurrencyId::CROWD_LOAN;
			let amount = 1_000_000_000 * CurrencyId::unit::<Balance>();

			// Create Vault (LAYR)
			let vault_id = create_vault(base_asset, None);

			// Create Pool (LAYR/CROWD_LOAN)
			let pool_id = create_pool(base_asset, amount, quote_asset, amount, None, None);

			let proposal = Call::PabloStrategy(crate::Call::set_pool_id_for_asset {
				asset_id: base_asset,
				pool_id,
			});
			set_admin_members(vec![ALICE], 5);
			make_proposal(proposal, ALICE, 1, 0, None);

			let proposal = Call::PabloStrategy(crate::Call::associate_vault { vault_id });
			make_proposal(proposal, ALICE, 1, 0, None);

			assert_ok!(PabloStrategy::rebalance());

			System::assert_last_event(Event::PabloStrategy(pallet::Event::RebalancedVault {
				vault_id,
			}));
		});
	}
}

// -------------------------------------------------------------------------------------------------
//                                             Set pool_id for asset_id
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod set_pool_id_for_asset {
	use super::*;

	#[test]
	fn set_pool_id_for_asset_emits_event() {
		ExtBuilder::default().build().execute_with(|| {
			System::set_block_number(1);
			let base_asset = CurrencyId::LAYR;
			let quote_asset = CurrencyId::CROWD_LOAN;
			let amount = 1_000_000_000 * CurrencyId::unit::<Balance>();

			// Create Pool (LAYR/CROWD_LOAN)
			let pool_id = create_pool(base_asset, amount, quote_asset, amount, None, None);
			set_admin_members(vec![ADMIN, ALICE, BOB], 5);
			let proposal = Call::PabloStrategy(crate::Call::set_pool_id_for_asset {
				asset_id: base_asset,
				pool_id,
			});
			make_proposal(proposal, ALICE, 2, 0, Some(&[ALICE, BOB]));
			System::assert_has_event(Event::PabloStrategy(
				pallet::Event::AssociatedPoolWithAsset { asset_id: base_asset, pool_id },
			));
		});
	}

	#[test]
	fn setting_pool_id_for_asset_with_not_enough_number_of_votes_throw_error() {
		ExtBuilder::default().build().execute_with(|| {
			System::set_block_number(1);
			let base_asset = CurrencyId::LAYR;
			let quote_asset = CurrencyId::CROWD_LOAN;
			let amount = 1_000_000_000 * CurrencyId::unit::<Balance>();

			// Create Pool (LAYR/CROWD_LOAN)
			let pool_id = create_pool(base_asset, amount, quote_asset, amount, None, None);
			set_admin_members(vec![ADMIN, ALICE, BOB], 5);
			let proposal = Call::PabloStrategy(crate::Call::set_pool_id_for_asset {
				asset_id: base_asset,
				pool_id,
			});
			make_proposal(proposal, ALICE, 2, 0, Some(&[ALICE]));
		});
	}
}

// -------------------------------------------------------------------------------------------------
//                                             Transferring funds
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod transferring_funds {
	use super::*;

	#[test]
	fn transferring_funds_success() {
		ExtBuilder::default().build().execute_with(|| {
			System::set_block_number(1);
			let base_asset = CurrencyId::LAYR;
			let quote_asset = CurrencyId::CROWD_LOAN;
			let amount = 1_000_000_000 * CurrencyId::unit::<Balance>();

			// Create Vault (LAYR)
			let vault_id = create_vault(base_asset, Perquintill::from_percent(50));
			let pool_id = create_pool(base_asset, amount, quote_asset, amount, None, None);
			set_admin_members(vec![ALICE], 5);
			let associate_vault_proposal =
				Call::PabloStrategy(crate::Call::associate_vault { vault_id });
			make_proposal(associate_vault_proposal, ALICE, 1, 0, None);
			let vault_account = Vault::account_id(&vault_id);
			assert_ok!(Tokens::mint_into(base_asset, &vault_account, 1_000_000));
			// set pool_id for asset
			let set_pool_id_for_asset_proposal =
				Call::PabloStrategy(crate::Call::set_pool_id_for_asset {
					asset_id: base_asset,
					pool_id,
				});
			make_proposal(set_pool_id_for_asset_proposal, ALICE, 1, 0, None);
			// liquidity rebalance
			let liquidity_rebalance_proposal =
				Call::PabloStrategy(crate::Call::liquidity_rebalance {});
			make_proposal(liquidity_rebalance_proposal, ALICE, 1, 0, None);
			// tranferring funds
			let new_quote_asset = CurrencyId::USDT;
			let new_pool_id = create_pool(base_asset, amount, new_quote_asset, amount, None, None);
			let percentage_of_funds = Percent::from_percent(10);
			let transferring_funds_proposal =
				Call::PabloStrategy(crate::Call::transferring_funds {
					vault_id,
					asset_id: base_asset,
					new_pool_id,
					percentage_of_funds,
				});
			make_proposal(transferring_funds_proposal, ALICE, 1, 0, None);
			let proposal = Call::PabloStrategy(crate::Call::liquidity_rebalance {});
			assert_eq!(PabloStrategy::pools(base_asset).unwrap().pool_id, new_pool_id);
		});
	}
}
