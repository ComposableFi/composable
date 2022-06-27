use composable_traits::instrumental::{
	AccessRights, Instrumental as InstrumentalTrait, InstrumentalProtocolStrategy,
	InstrumentalVaultConfig, State,
};
use frame_support::{assert_noop, assert_ok};
use primitives::currency::CurrencyId;
use sp_runtime::Perquintill;

use crate::mock::{
	account_id::{ADMIN, ALICE},
	helpers::{create_layr_crowd_loan_pool, set_admin_account_with_full_access},
	runtime::{
		Event, ExtBuilder, Instrumental, MockRuntime, Origin, PabloStrategy, System, VaultId,
		MAX_ASSOCIATED_VAULTS,
	},
};
#[allow(unused_imports)]
use crate::{pallet, pallet::Error};

// -------------------------------------------------------------------------------------------------
//                                          Associate Vault
// -------------------------------------------------------------------------------------------------

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

// -------------------------------------------------------------------------------------------------
//                                             Rebalance
// -------------------------------------------------------------------------------------------------

#[test]
fn rebalance_emits_event() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		let asset_id = CurrencyId::LAYR;
		// Create Vault (LAYR)
		let config = InstrumentalVaultConfig { asset_id, percent_deployable: Perquintill::zero() };
		let vault_id = <Instrumental as InstrumentalTrait>::create(config);
		assert_ok!(vault_id);
		let vault_id = vault_id.unwrap() as VaultId;

		// Create Pool (LAYR/CROWD_LOAN)
		let pool_id = create_layr_crowd_loan_pool();

		pallet::AdminAccountIds::<MockRuntime>::insert(ADMIN, AccessRights::Full);
		assert_ok!(PabloStrategy::set_pool_id_for_asset(Origin::signed(ADMIN), asset_id, pool_id));

		assert_ok!(PabloStrategy::associate_vault(&vault_id));

		assert_ok!(PabloStrategy::rebalance());

		System::assert_last_event(Event::PabloStrategy(pallet::Event::RebalancedVault {
			vault_id,
		}));
	});
}

// -------------------------------------------------------------------------------------------------
//                                             Set pool_id for asset
// -------------------------------------------------------------------------------------------------

#[test]
fn test_caller_is_persmissoned() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		let asset_id = CurrencyId::LAYR;
		// Create Pool (LAYR/CROWD_LOAN)
		let pool_id = create_layr_crowd_loan_pool();

		pallet::AdminAccountIds::<MockRuntime>::insert(ADMIN, AccessRights::Rebalance);
		assert_noop!(
			PabloStrategy::set_pool_id_for_asset(Origin::signed(ADMIN), asset_id, pool_id),
			Error::<MockRuntime>::NotEnoughAccessRights
		);

		set_admin_account_with_full_access();
		assert_ok!(PabloStrategy::set_pool_id_for_asset(Origin::signed(ADMIN), asset_id, pool_id));

		assert_noop!(
			PabloStrategy::set_pool_id_for_asset(Origin::signed(ALICE), asset_id, pool_id),
			Error::<MockRuntime>::NotAdminAccount
		);
	})
}

#[test]
fn test_pool_id_must_be_valid() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		let asset_id = CurrencyId::LAYR;
		// Create Pool (LAYR/CROWD_LOAN)
		let not_valid_pool_id = 1;
		set_admin_account_with_full_access();

		assert_noop!(
			PabloStrategy::set_pool_id_for_asset(
				Origin::signed(ADMIN),
				asset_id,
				not_valid_pool_id
			),
			Error::<MockRuntime>::PoolIsNotValidated
		);
	})
}

#[test]
fn test_setting_pool_id_for_the_first_time_succeeds() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		let asset_id = CurrencyId::LAYR;
		// Create Pool (LAYR/CROWD_LOAN)
		let pool_id = create_layr_crowd_loan_pool();
		set_admin_account_with_full_access();
		assert_ok!(PabloStrategy::set_pool_id_for_asset(Origin::signed(ADMIN), asset_id, pool_id));
		PabloStrategy::pools(asset_id);
		assert_eq!(
			PabloStrategy::pools(asset_id).unwrap(),
			pallet::PoolState { pool_id, state: State::Normal }
		);
		System::assert_last_event(Event::PabloStrategy(pallet::Event::AssociatedPoolWithAsset {
			asset_id,
			pool_id,
		}));
	})
}
