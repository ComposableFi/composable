use super::prelude::*;
use crate::tests::{process_and_progress_blocks};
use frame_support::traits::{ fungibles::Mutate};
use composable_traits::{vault::Vault as VaultTrait};

#[test]
fn test_pausing_vault_deposit() {
	new_test_ext().execute_with(|| {
		let (market, _vault_id) = create_simple_market();

		let collateral = 1_000_000_000_000;
		let borrow = 10;
		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, collateral));
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, borrow));

        // only manager can pause/unpause
		assert_noop!(
			Lending::update_market_functionality(Origin::signed(*BOB), market, vec![(0, true)]),
			crate::Error::<Runtime>::Unauthorized
		);

        // pausing should go through
		assert_extrinsic_event::<Runtime>(
			Lending::update_market_functionality(Origin::signed(*ALICE), market, vec![(0, true)]),
			Event::Lending(crate::Event::FunctionalityChanged {
				market_id: market,
				changed_functionalities: vec![(0, true)]
			}),
		);

		let (_, market_state) = Lending::get_market_state(&market).unwrap();
		// market state should have changed
		assert_eq!(market_state.is_paused_functionalities, vec![true, false, false, false, false, false, false]);
        // vault deposit should fail because it is paused
        assert_noop!(
            Lending::vault_deposit(Origin::signed(*ALICE), market, borrow),
            crate::Error::<Runtime>::DepositVaultPaused
        );

		process_and_progress_blocks::<Lending, Runtime>(1);
		// vault deposit should still fail because it is paused
		assert_noop!(
            Lending::vault_deposit(Origin::signed(*ALICE), market, borrow),
            crate::Error::<Runtime>::DepositVaultPaused
        );
		// try unpausing by not a manager
        assert_noop!(
			Lending::update_market_functionality(Origin::signed(*BOB), market, vec![(0, false)]),
			crate::Error::<Runtime>::Unauthorized
		);
        // manager unpauses
        assert_ok!(
			Lending::update_market_functionality(Origin::signed(*ALICE), market, vec![(0, false)])
		);
        let (_, market_state) = Lending::get_market_state(&market).unwrap();
		// market state should have changed
		assert_eq!(market_state.is_paused_functionalities, vec![false; 7]);
        // vault withdraw should succeed
		assert_ok!(
			Lending::vault_deposit(Origin::signed(*ALICE), market, borrow),
		);
	});
}


#[test]
fn test_pausing_vault_withdraw() {
	new_test_ext().execute_with(|| {
		let (market, vault_id) = create_simple_market();
        let vault_account = Vault::account_id(&vault_id);

		let collateral = 1_000_000_000_000;
		let borrow = 10;
		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, collateral));
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, borrow));

		assert_ok!(Lending::vault_deposit(Origin::signed(*ALICE), market, borrow));

		// only manager can pause/unpause
		assert_noop!(
			Lending::update_market_functionality(Origin::signed(*BOB), market, vec![(1, true)]),
			crate::Error::<Runtime>::Unauthorized
		);
		// pausing should go through
		assert_extrinsic_event::<Runtime>(
			Lending::update_market_functionality(Origin::signed(*ALICE), market, vec![(1, true)]),
			Event::Lending(crate::Event::FunctionalityChanged {
				market_id: market,
				changed_functionalities: vec![(1, true)]
			}),
		);

		let (_, market_state) = Lending::get_market_state(&market).unwrap();
		// market state should have changed
		assert_eq!(market_state.is_paused_functionalities, vec![false, true, false, false, false, false, false]);
		// vault withdraw should fail because it is paused
		assert_noop!(
			Lending::vault_withdraw(Origin::signed(*ALICE), market, borrow),
			crate::Error::<Runtime>::WithdrawVaultPaused
		);
		process_and_progress_blocks::<Lending, Runtime>(1);
		// vault withdraw should still fail because it is paused
		assert_noop!(
			Lending::vault_withdraw(Origin::signed(*ALICE), market, borrow),
			crate::Error::<Runtime>::WithdrawVaultPaused
		);
		// try unpausing by not a manager
        assert_noop!(
			Lending::update_market_functionality(Origin::signed(*BOB), market, vec![(1, false)]),
			crate::Error::<Runtime>::Unauthorized
		);
        // manager unpauses
        assert_ok!(
			Lending::update_market_functionality(Origin::signed(*ALICE), market, vec![(1, false)])
		);
        let (_, market_state) = Lending::get_market_state(&market).unwrap();
		// market state should have changed
		assert_eq!(market_state.is_paused_functionalities, vec![false; 7]);
        // vault withdraw should succeed
		assert_ok!(
			Lending::vault_withdraw(Origin::signed(*ALICE), market, Assets::balance(USDT::ID, &vault_account))
		);
	});
}

#[test]
fn test_pausing_collateral_deposit() {
	new_test_ext().execute_with(|| {
		let (market_id, _vault_id) = create_simple_market();
		let deposit_usdt = 1_000_000_000;
		let deposit_btc = 10;
		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, deposit_usdt));
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, deposit_btc));


        

		// assert_ok!(Lending::vault_deposit(Origin::signed(*ALICE), market_id, deposit_btc));
		assert_extrinsic_event::<Runtime>(
			Lending::deposit_collateral(Origin::signed(*ALICE), market_id, deposit_usdt, false),
			Event::Lending(pallet_lending::Event::<Runtime>::CollateralDeposited {
				sender: *ALICE,
				market_id,
				amount: deposit_usdt,
			}),
		);
	});
}