use composable_traits::{
	instrumental::InstrumentalProtocolStrategy,
	vault::{CapabilityVault, StrategicVault as StrategicVaultTrait, Vault as VaultTrait},
};
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
		helpers::{
			assert_has_event, associate_vault, create_pool, create_vault, liquidity_rebalance,
			make_proposal, set_admin_members, set_pool_id_for_asset,
		},
		runtime::{
			Balance, Call, Event, ExtBuilder, MockRuntime, Origin, Pablo, PabloStrategy, System,
			Tokens, Vault, VaultId, MAX_ASSOCIATED_VAULTS,
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
	use crate::mock::runtime::Assets;

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

	#[test]
	fn funds_availability_withdrawable() {
		ExtBuilder::default().build().execute_with(|| {
			System::set_block_number(1);
			let base_asset = CurrencyId::LAYR;
			let quote_asset = CurrencyId::CROWD_LOAN;
			let amount = 1_000_000_000 * CurrencyId::unit::<Balance>();
			// Create Vault (LAYR)
			let vault_id = create_vault(base_asset, Perquintill::from_percent(50));
			let pool_id = create_pool(base_asset, amount, quote_asset, amount, None, None);
			set_admin_members(vec![ALICE], 5);
			associate_vault(vault_id);
			// mint funds to Alice
			assert_ok!(Tokens::mint_into(base_asset, &ALICE, amount));
			// deposit funds to Vault
			assert_ok!(Vault::deposit(Origin::signed(ALICE), vault_id, 100_000 as Balance));
			// set pool_id for asset
			set_pool_id_for_asset(base_asset, pool_id);
			// liquidity rebalance
			liquidity_rebalance();
			System::assert_has_event(Event::PabloStrategy(
				pallet::Event::WithdrawFunctionalityOccuredDuringRebalance { vault_id },
			));
		});
	}

	#[test]
	fn funds_availability_depositable() {
		ExtBuilder::default().build().execute_with(|| {
			System::set_block_number(1);
			let base_asset = CurrencyId::LAYR;
			let quote_asset = CurrencyId::CROWD_LOAN;
			let amount = 1_000_000_000 * CurrencyId::unit::<Balance>();
			// Create Vault (LAYR)
			let vault_id = create_vault(base_asset, Perquintill::from_percent(50));
			let pool_id = create_pool(base_asset, amount, quote_asset, amount, None, None);
			set_admin_members(vec![ALICE], 5);
			associate_vault(vault_id);
			let vault_data = Vault::vault_data(vault_id).unwrap();
			// set pool_id for asset
			set_pool_id_for_asset(base_asset, pool_id);
			// mint funds for Alice
			assert_ok!(Tokens::mint_into(base_asset, &ALICE, 1_000_000_000));
			// deposit to Vault
			assert_ok!(Vault::deposit(Origin::signed(ALICE), vault_id, 1_000_000));
			// first rebalance
			liquidity_rebalance();
			// withdraw funds from Vault
			assert_ok!(Vault::withdraw(Origin::signed(ALICE), vault_id, 1_000));
			// second rebalance
			liquidity_rebalance();
			System::assert_has_event(Event::PabloStrategy(
				pallet::Event::DepositFunctionalityOccuredDuringRebalance { vault_id },
			));
		});
	}

	#[test]
	fn funds_availability_none() {
		ExtBuilder::default().build().execute_with(|| {
			System::set_block_number(1);
			let base_asset = CurrencyId::LAYR;
			let quote_asset = CurrencyId::CROWD_LOAN;
			let amount = 1_000_000_000 * CurrencyId::unit::<Balance>();
			// Create Vault (LAYR)
			let vault_id = create_vault(base_asset, Perquintill::from_percent(50));
			let pool_id = create_pool(base_asset, amount, quote_asset, amount, None, None);
			set_admin_members(vec![ALICE], 5);
			associate_vault(vault_id);
			let vault_data = Vault::vault_data(vault_id).unwrap();
			// set pool_id for asset
			set_pool_id_for_asset(base_asset, pool_id);
			// mint funds for Alice
			assert_ok!(Tokens::mint_into(base_asset, &ALICE, 1_000_000_000));
			// deposit to Vault
			assert_ok!(Vault::deposit(Origin::signed(ALICE), vault_id, 1_000_000));
			// first rebalance
			liquidity_rebalance();
			// second rebalance
			liquidity_rebalance();
			System::assert_has_event(Event::PabloStrategy(
				pallet::Event::NoneFunctionalityOccuredDuringRebalance { vault_id },
			));
		});
	}

	#[test]
	fn funds_availability_liquidate() {
		ExtBuilder::default().build().execute_with(|| {
			System::set_block_number(1);
			let base_asset = CurrencyId::LAYR;
			let quote_asset = CurrencyId::CROWD_LOAN;
			let amount = 1_000_000_000 * CurrencyId::unit::<Balance>();
			// Create Vault (LAYR)
			let vault_id = create_vault(base_asset, Perquintill::from_percent(50));
			let pool_id = create_pool(base_asset, amount, quote_asset, amount, None, None);
			set_admin_members(vec![ALICE], 5);
			associate_vault(vault_id);
			let vault_data = Vault::vault_data(vault_id).unwrap();
			// set pool_id for asset
			set_pool_id_for_asset(base_asset, pool_id);
			// mint funds for Alice
			assert_ok!(Tokens::mint_into(base_asset, &ALICE, 1_000_000_000));
			// deposit to Vault
			assert_ok!(Vault::deposit(Origin::signed(ALICE), vault_id, 1_000_000));
			// first rebalance
			liquidity_rebalance();
			// stop Vault
			assert_ok!(Vault::stop(&vault_id));
			// second rebalance
			liquidity_rebalance();
			System::assert_has_event(Event::PabloStrategy(
				pallet::Event::LiquidateFunctionalityOccuredDuringRebalance { vault_id },
			));
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
			associate_vault(vault_id);
			// set pool_id for asset
			set_pool_id_for_asset(base_asset, pool_id);
			// liquidity rebalance
			liquidity_rebalance();
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
