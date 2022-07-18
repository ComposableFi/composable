use crate::mock::{
	account_id::{ADMIN, ALICE, BOB},
	helpers::{create_pool, create_vault},
	runtime::{
		Balance, Call, CollectiveInstrumental, Event, ExtBuilder, MockRuntime, Origin,
		PabloStrategy, System, VaultId, MAX_ASSOCIATED_VAULTS,
	},
};
use composable_traits::instrumental::InstrumentalProtocolStrategy;
use frame_support::{assert_noop, assert_ok, weights::GetDispatchInfo};
use primitives::currency::CurrencyId;
use sp_core::{Encode, H256};
use sp_runtime::traits::{BlakeTwo256, Hash};

#[allow(unused_imports)]
use crate::{pallet, pallet::Error};
use pallet_collective::{Error as CollectiveError, Instance1, MemberCount};

// -------------------------------------------------------------------------------------------------
//                                          Associate Vault
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod associate_vault {
	use super::*;

	#[test]
	fn add_an_associated_vault() {
		ExtBuilder::default().build().execute_with(|| {
			let vault_id: VaultId = 1;

			assert_ok!(PabloStrategy::associate_vault(&vault_id));
		});
	}

	#[test]
	fn adding_an_associated_vault_twice_throws_an_error() {
		ExtBuilder::default().build().execute_with(|| {
			let vault_id: VaultId = 1;

			assert_ok!(PabloStrategy::associate_vault(&vault_id));
			assert_noop!(
				PabloStrategy::associate_vault(&vault_id),
				Error::<MockRuntime>::VaultAlreadyAssociated
			);
		});
	}

	#[test]
	fn associating_too_many_vaults_throws_an_error() {
		ExtBuilder::default().build().execute_with(|| {
			for vault_id in 0..MAX_ASSOCIATED_VAULTS {
				assert_ok!(PabloStrategy::associate_vault(&(vault_id as VaultId)));
			}

			let vault_id = MAX_ASSOCIATED_VAULTS as VaultId;
			assert_noop!(
				PabloStrategy::associate_vault(&vault_id),
				Error::<MockRuntime>::TooManyAssociatedStrategies
			);
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

			let members_count: MemberCount = 5;
			assert_ok!(CollectiveInstrumental::set_members(
				Origin::root(),
				vec![ALICE],
				None,
				members_count,
			));
			let proposal = Call::PabloStrategy(crate::Call::set_pool_id_for_asset {
				asset_id: base_asset,
				pool_id,
			});
			let proposal_len: u32 = proposal.using_encoded(|p| p.len() as u32);
			let proposal_weight = proposal.get_dispatch_info().weight;
			let hash: H256 = BlakeTwo256::hash_of(&proposal);
			assert_ok!(CollectiveInstrumental::propose(
				Origin::signed(ALICE),
				1,
				Box::new(proposal),
				proposal_len
			));

			assert_ok!(PabloStrategy::associate_vault(&vault_id));

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
			let members_count: MemberCount = 5;
			assert_ok!(CollectiveInstrumental::set_members(
				Origin::root(),
				vec![ADMIN, ALICE, BOB],
				None,
				members_count,
			));
			let proposal = Call::PabloStrategy(crate::Call::set_pool_id_for_asset {
				asset_id: base_asset,
				pool_id,
			});
			let proposal_len: u32 = proposal.using_encoded(|p| p.len() as u32);
			let proposal_weight = proposal.get_dispatch_info().weight;
			let hash: H256 = BlakeTwo256::hash_of(&proposal);
			assert_ok!(CollectiveInstrumental::propose(
				Origin::signed(ALICE),
				2,
				Box::new(proposal),
				proposal_len
			));
			assert_ok!(CollectiveInstrumental::vote(Origin::signed(ALICE), hash, 0, true));
			assert_ok!(CollectiveInstrumental::vote(Origin::signed(BOB), hash, 0, true));
			assert_ok!(CollectiveInstrumental::close(
				Origin::signed(ALICE),
				hash,
				0,
				proposal_weight,
				proposal_len
			));
			let system_events = frame_system::Pallet::<MockRuntime>::events();
			let mut occured_associated_pool_with_asset = false;
			system_events.iter().for_each(|event_record| {
				if event_record.event ==
					Event::PabloStrategy(pallet::Event::AssociatedPoolWithAsset {
						asset_id: base_asset,
						pool_id,
					}) {
					occured_associated_pool_with_asset = true;
				}
			});
			assert!(occured_associated_pool_with_asset);
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
			let members_count: MemberCount = 5;
			assert_ok!(CollectiveInstrumental::set_members(
				Origin::root(),
				vec![ADMIN, ALICE, BOB],
				None,
				members_count,
			));
			let proposal = Call::PabloStrategy(crate::Call::set_pool_id_for_asset {
				asset_id: base_asset,
				pool_id,
			});
			let proposal_len: u32 = proposal.using_encoded(|p| p.len() as u32);
			let proposal_weight = proposal.get_dispatch_info().weight;
			let hash: H256 = BlakeTwo256::hash_of(&proposal);
			assert_ok!(CollectiveInstrumental::propose(
				Origin::signed(ALICE),
				2,
				Box::new(proposal),
				proposal_len
			));
			assert_ok!(CollectiveInstrumental::vote(Origin::signed(ALICE), hash, 0, true));
			assert_noop!(
				CollectiveInstrumental::close(
					Origin::signed(ALICE),
					hash,
					0,
					proposal_weight,
					proposal_len
				),
				CollectiveError::<MockRuntime, Instance1>::TooEarly
			);
		});
	}
}
