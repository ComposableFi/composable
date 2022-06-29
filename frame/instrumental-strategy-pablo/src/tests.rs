use composable_traits::instrumental::{AccessRights, InstrumentalProtocolStrategy};
use frame_support::{assert_noop, assert_ok};
use primitives::currency::CurrencyId;

use crate::mock::{
	account_id::ADMIN,
	helpers::{create_pool, create_vault},
	runtime::{
		Balance, Event, ExtBuilder, MockRuntime, Origin, PabloStrategy, System, VaultId,
		MAX_ASSOCIATED_VAULTS,
	},
};
#[allow(unused_imports)]
use crate::{pallet, pallet::Error};

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
			pallet::AdminAccountIds::<MockRuntime>::insert(ADMIN, AccessRights::Full);
			assert_ok!(PabloStrategy::set_pool_id_for_asset(
				Origin::signed(ADMIN),
				base_asset,
				pool_id
			));

			assert_ok!(PabloStrategy::associate_vault(&vault_id));

			assert_ok!(PabloStrategy::rebalance());

			System::assert_last_event(Event::PabloStrategy(pallet::Event::RebalancedVault {
				vault_id,
			}));
		});
	}
}
