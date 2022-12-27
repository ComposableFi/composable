use super::prelude::*;
use crate::{tests::process_and_progress_blocks, Functionality};
use composable_traits::vault::Vault as VaultTrait;
use frame_support::traits::fungibles::Mutate;
use num_traits::Pow;

#[test]
fn test_pausing_vault_deposit() {
	new_test_ext().execute_with(|| {
		let (market_id, _vault_id) = create_simple_market();

		let collateral = 1_000_000_000_000;
		let borrow = 10;
		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, collateral));
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, borrow));
		let vault_deposit_index = Lending::get_functionality_index(Functionality::DepositVault)
			.try_into()
			.unwrap();
		// only manager can pause/unpause
		assert_noop!(
			Lending::update_market_functionality(
				Origin::signed(*BOB),
				market_id,
				vec![(vault_deposit_index, true)]
			),
			crate::Error::<Runtime>::Unauthorized
		);

		// pausing should go through
		assert_extrinsic_event::<Runtime>(
			Lending::update_market_functionality(
				Origin::signed(*ALICE),
				market_id,
				vec![(vault_deposit_index, true)],
			),
			Event::Lending(crate::Event::FunctionalityChanged {
				market_id,
				changed_functionalities: vec![(0, true)],
			}),
		);

		let (_, market_state) = Lending::get_market_state(&market_id).unwrap();
		// market state should have changed
		assert_eq!(
			market_state.is_paused_functionalities,
			vec![true, false, false, false, false, false, false]
		);
		// vault deposit should fail because it is paused
		assert_noop!(
			Lending::vault_deposit(Origin::signed(*ALICE), market_id, borrow),
			crate::Error::<Runtime>::DepositVaultPaused
		);

		process_and_progress_blocks::<Lending, Runtime>(1);
		// vault deposit should still fail because it is paused
		assert_noop!(
			Lending::vault_deposit(Origin::signed(*ALICE), market_id, borrow),
			crate::Error::<Runtime>::DepositVaultPaused
		);
		// try unpausing by not a manager
		assert_noop!(
			Lending::update_market_functionality(
				Origin::signed(*BOB),
				market_id,
				vec![(vault_deposit_index, false)]
			),
			crate::Error::<Runtime>::Unauthorized
		);
		// manager unpauses
		assert_ok!(Lending::update_market_functionality(
			Origin::signed(*ALICE),
			market_id,
			vec![(vault_deposit_index, false)]
		));
		let (_, market_state) = Lending::get_market_state(&market_id).unwrap();
		// market state should have changed
		assert_eq!(market_state.is_paused_functionalities, vec![false; 7]);
		// vault withdraw should succeed
		assert_ok!(Lending::vault_deposit(Origin::signed(*ALICE), market_id, borrow),);
	});
}

#[test]
fn test_pausing_vault_withdraw() {
	new_test_ext().execute_with(|| {
		let (market_id, vault_id) = create_simple_market();
		let vault_account = Vault::account_id(&vault_id);

		let collateral = 1_000_000_000_000;
		let borrow = 10;
		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, collateral));
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, borrow));

		assert_ok!(Lending::vault_deposit(Origin::signed(*ALICE), market_id, borrow));
		let vault_withdraw_index = Lending::get_functionality_index(Functionality::WithdrawVault)
			.try_into()
			.unwrap();
		// only manager can pause/unpause
		assert_noop!(
			Lending::update_market_functionality(
				Origin::signed(*BOB),
				market_id,
				vec![(vault_withdraw_index, true)]
			),
			crate::Error::<Runtime>::Unauthorized
		);
		// pausing should go through
		assert_extrinsic_event::<Runtime>(
			Lending::update_market_functionality(
				Origin::signed(*ALICE),
				market_id,
				vec![(vault_withdraw_index, true)],
			),
			Event::Lending(crate::Event::FunctionalityChanged {
				market_id,
				changed_functionalities: vec![(1, true)],
			}),
		);

		let (_, market_state) = Lending::get_market_state(&market_id).unwrap();
		// market state should have changed
		assert_eq!(
			market_state.is_paused_functionalities,
			vec![false, true, false, false, false, false, false]
		);
		// vault withdraw should fail because it is paused
		assert_noop!(
			Lending::vault_withdraw(Origin::signed(*ALICE), market_id, borrow),
			crate::Error::<Runtime>::WithdrawVaultPaused
		);
		process_and_progress_blocks::<Lending, Runtime>(1);
		// vault withdraw should still fail because it is paused
		assert_noop!(
			Lending::vault_withdraw(Origin::signed(*ALICE), market_id, borrow),
			crate::Error::<Runtime>::WithdrawVaultPaused
		);
		// try unpausing by not a manager
		assert_noop!(
			Lending::update_market_functionality(
				Origin::signed(*BOB),
				market_id,
				vec![(vault_withdraw_index, false)]
			),
			crate::Error::<Runtime>::Unauthorized
		);
		// manager unpauses
		assert_ok!(Lending::update_market_functionality(
			Origin::signed(*ALICE),
			market_id,
			vec![(vault_withdraw_index, false)]
		));
		let (_, market_state) = Lending::get_market_state(&market_id).unwrap();
		// market state should have changed
		assert_eq!(market_state.is_paused_functionalities, vec![false; 7]);
		// vault withdraw should succeed
		assert_ok!(Lending::vault_withdraw(
			Origin::signed(*ALICE),
			market_id,
			Assets::balance(USDT::ID, &vault_account)
		));
	});
}

#[test]
// took from test_vault_market_can_withdraw
fn test_pausing_collateral_deposit() {
	new_test_ext().execute_with(|| {
		let (market_id, _vault_id) = create_simple_market();
		let collateral = 1_000_000_000_000;
		let borrow = 10;
		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, collateral));
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, borrow));

		assert_ok!(Lending::vault_deposit(Origin::signed(*ALICE), market_id, borrow));
		let collateral_deposit_index =
			Lending::get_functionality_index(Functionality::DepositCollateral)
				.try_into()
				.unwrap();
		// pausing should go through
		assert_extrinsic_event::<Runtime>(
			Lending::update_market_functionality(
				Origin::signed(*ALICE),
				market_id,
				vec![(collateral_deposit_index, true)],
			),
			Event::Lending(crate::Event::FunctionalityChanged {
				market_id,
				changed_functionalities: vec![(2, true)],
			}),
		);
		let (_, market_state) = Lending::get_market_state(&market_id).unwrap();
		// market state should have changed
		assert_eq!(
			market_state.is_paused_functionalities,
			vec![false, false, true, false, false, false, false]
		);

		assert_noop!(
			Lending::deposit_collateral(Origin::signed(*ALICE), market_id, collateral, false),
			crate::Error::<Runtime>::DepositCollateralPaused
		);
		process_and_progress_blocks::<Lending, Runtime>(1);
		// collateral deposit should still fail because it is paused
		assert_noop!(
			Lending::deposit_collateral(Origin::signed(*ALICE), market_id, collateral, false),
			crate::Error::<Runtime>::DepositCollateralPaused
		);
		// manager unpauses
		assert_ok!(Lending::update_market_functionality(
			Origin::signed(*ALICE),
			market_id,
			vec![(collateral_deposit_index, false)]
		));
		let (_, market_state) = Lending::get_market_state(&market_id).unwrap();
		// market state should have changed
		assert_eq!(market_state.is_paused_functionalities, vec![false; 7]);
		assert_extrinsic_event::<Runtime>(
			Lending::deposit_collateral(Origin::signed(*ALICE), market_id, collateral, false),
			Event::Lending(crate::Event::CollateralDeposited {
				sender: *ALICE,
				amount: collateral,
				market_id,
			}),
		);
		process_and_progress_blocks::<Lending, Runtime>(1);
		// We waited 1 block, the market should have withdraw the funds
		assert_extrinsic_event::<Runtime>(
			Lending::borrow(Origin::signed(*ALICE), market_id, borrow - 1),
			Event::Lending(crate::Event::Borrowed {
				sender: *ALICE,
				amount: borrow - 1, // DEFAULT_MARKET_VAULT_RESERVE
				market_id,
			}),
		);
	});
}

#[test]
fn test_pausing_borrow() {
	new_test_ext().execute_with(|| {
		let (market_id, _vault_id) = create_simple_market();
		let collateral = 1_000_000_000_000;
		let borrow = 10;
		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, collateral));
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, borrow));

		assert_ok!(Lending::vault_deposit(Origin::signed(*ALICE), market_id, borrow));
		let borrow_index =
			Lending::get_functionality_index(Functionality::Borrow).try_into().unwrap();
		// pausing should go through
		assert_extrinsic_event::<Runtime>(
			Lending::update_market_functionality(
				Origin::signed(*ALICE),
				market_id,
				vec![(borrow_index, true)],
			),
			Event::Lending(crate::Event::FunctionalityChanged {
				market_id,
				changed_functionalities: vec![(4, true)],
			}),
		);
		let (_, market_state) = Lending::get_market_state(&market_id).unwrap();
		// market state should have changed
		assert_eq!(
			market_state.is_paused_functionalities,
			vec![false, false, false, false, true, false, false]
		);

		assert_extrinsic_event::<Runtime>(
			Lending::deposit_collateral(Origin::signed(*ALICE), market_id, collateral, false),
			Event::Lending(crate::Event::CollateralDeposited {
				sender: *ALICE,
				amount: collateral,
				market_id,
			}),
		);
		process_and_progress_blocks::<Lending, Runtime>(1);

		// We waited 1 block, the market should have withdraw the funds
		// now we should fail to borrow
		assert_noop!(
			Lending::borrow(Origin::signed(*ALICE), market_id, borrow - 1),
			crate::Error::<Runtime>::BorrowPaused
		);

		// manager unpauses
		assert_ok!(Lending::update_market_functionality(
			Origin::signed(*ALICE),
			market_id,
			vec![(borrow_index, false)]
		));
		let (_, market_state) = Lending::get_market_state(&market_id).unwrap();
		// market state should have changed
		assert_eq!(market_state.is_paused_functionalities, vec![false; 7]);
		// now can borrow
		assert_extrinsic_event::<Runtime>(
			Lending::borrow(Origin::signed(*ALICE), market_id, borrow - 1),
			Event::Lending(crate::Event::Borrowed {
				sender: *ALICE,
				amount: borrow - 1, // DEFAULT_MARKET_VAULT_RESERVE
				market_id,
			}),
		);
	});
}

#[test]
fn test_pausing_repay_borrow() {
	new_test_ext().execute_with(|| {
		// accounts have 1 unit of collateral
		let alice_balance = BTC::ONE;

		let (market_id, _vault_id) = create_simple_market();

		mint_and_deposit_collateral::<Runtime>(*ALICE, alice_balance, market_id, BTC::ID);

		let borrow_asset_deposit = USDT::units(1_000_000);
		assert_ok!(Tokens::mint_into(USDT::ID, &CHARLIE, borrow_asset_deposit));
		assert_extrinsic_event::<Runtime>(
			Lending::vault_deposit(Origin::signed(*CHARLIE), market_id, borrow_asset_deposit),
			Event::Lending(crate::Event::AssetDeposited {
				sender: *CHARLIE,
				market_id,
				amount: borrow_asset_deposit,
			}),
		);

		process_and_progress_blocks::<Lending, Runtime>(1_000);

		let get_collateral_borrow_limit_for_account = |account| {
			// `limit_normalized` is the limit in USDT
			// `limit` is the limit in BTC
			// BTC is worth 50_000 times more than USDT (see `create_simple_market()`)

			// borrow_limit * COLLATERAL::ONE / price_of(COLLATERAL::ONE)
			// REVIEW: I'm still not sure if this makes sense
			let limit_normalized = Lending::get_borrow_limit(&market_id, &account).unwrap();
			let limit = limit_normalized.mul(BTC::ONE).div(get_price(BTC::ID, BTC::ONE));
			limit
		};

		let alice_limit = get_collateral_borrow_limit_for_account(*ALICE);
		assert_extrinsic_event::<Runtime>(
			// partial borrow
			Lending::borrow(Origin::signed(*ALICE), market_id, alice_limit / 2),
			Event::Lending(crate::Event::<Runtime>::Borrowed {
				sender: *ALICE,
				market_id,
				amount: alice_limit / 2,
			}),
		);

		let alice_total_debt_with_interest_initial =
			Lending::total_debt_with_interest(&market_id, &ALICE).unwrap().unwrap_amount();

		process_and_progress_blocks::<Lending, Runtime>(1_000);

		let alice_total_debt_with_interest_before_repay_pause =
			Lending::total_debt_with_interest(&market_id, &ALICE).unwrap().unwrap_amount();

		// interest accrued
		assert!(
			alice_total_debt_with_interest_initial <
				alice_total_debt_with_interest_before_repay_pause
		);

		let repay_borrow_index =
			Lending::get_functionality_index(Functionality::RepayBorrow).try_into().unwrap();
		// pausing should go through
		assert_extrinsic_event::<Runtime>(
			Lending::update_market_functionality(
				Origin::signed(*ALICE),
				market_id,
				vec![(repay_borrow_index, true)],
			),
			Event::Lending(crate::Event::FunctionalityChanged {
				market_id,
				changed_functionalities: vec![(5, true)],
			}),
		);
		let (_, market_state) = Lending::get_market_state(&market_id).unwrap();
		// market state should have changed
		assert_eq!(
			market_state.is_paused_functionalities,
			vec![false, false, false, false, false, true, false]
		);

		// cant repay borrow
		assert_noop!(
			Lending::repay_borrow(
				Origin::signed(*ALICE),
				market_id,
				*ALICE,
				RepayStrategy::PartialAmount(USDT::units(1) / 10_000),
				false,
			),
			crate::Error::<Runtime>::RepayBorrowPaused
		);

		// no new interest should accrued due to repay pause
		process_and_progress_blocks::<Lending, Runtime>(1_000);

		// new debt plus interest
		let alice_total_debt_with_interest_after_repay_pause =
			Lending::total_debt_with_interest(&market_id, &ALICE).unwrap().unwrap_amount();
		// should be equal
		assert_eq!(
			alice_total_debt_with_interest_after_repay_pause,
			alice_total_debt_with_interest_before_repay_pause
		);

		// manager unpauses
		assert_ok!(Lending::update_market_functionality(
			Origin::signed(*ALICE),
			market_id,
			vec![(repay_borrow_index, false)]
		));
		let (_, market_state) = Lending::get_market_state(&market_id).unwrap();
		// market state should have changed
		assert_eq!(market_state.is_paused_functionalities, vec![false; 7]);

		// pay off a small amount
		assert_extrinsic_event::<Runtime>(
			Lending::repay_borrow(
				Origin::signed(*ALICE),
				market_id,
				*ALICE,
				RepayStrategy::PartialAmount(USDT::units(1) / 10_000),
				false,
			),
			Event::Lending(crate::Event::<Runtime>::BorrowRepaid {
				sender: *ALICE,
				market_id,
				beneficiary: *ALICE,
				amount: USDT::units(1) / 10_000,
			}),
		);

		let alice_total_debt_with_interest_after_repay_unpause =
			Lending::total_debt_with_interest(&market_id, &ALICE).unwrap().unwrap_amount();
		assert!(
			alice_total_debt_with_interest_after_repay_pause >
				alice_total_debt_with_interest_after_repay_unpause
		);
		process_and_progress_blocks::<Lending, Runtime>(1_000);
		// interest should grow
		let alice_total_debt_with_interest_after_repay_unpause_next =
			Lending::total_debt_with_interest(&market_id, &ALICE).unwrap().unwrap_amount();
		assert!(
			alice_total_debt_with_interest_after_repay_unpause_next >
				alice_total_debt_with_interest_after_repay_unpause
		);
	});
}

#[test]
fn test_pausing_liquidation() {
	new_test_ext().execute_with(|| {
		let (market_id, vault_id) = create_simple_market();
		let collateral = BTC::units(100);
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, collateral));

		assert_extrinsic_event::<Runtime>(
			Lending::deposit_collateral(Origin::signed(*ALICE), market_id, collateral, false),
			Event::Lending(crate::Event::CollateralDeposited {
				sender: *ALICE,
				amount: collateral,
				market_id,
			}),
		);

		let usdt_amt = 2 * DEFAULT_COLLATERAL_FACTOR * USDT::ONE * get_price(BTC::ID, collateral) /
			get_price(NORMALIZED::ID, NORMALIZED::ONE);
		assert_ok!(Tokens::mint_into(USDT::ID, &CHARLIE, usdt_amt));
		assert_ok!(Vault::deposit(Origin::signed(*CHARLIE), vault_id, usdt_amt));

		// Allow the market to initialize it's account by withdrawing
		// from the vault
		process_and_progress_blocks::<Lending, Runtime>(1);

		let borrow_limit = Lending::get_borrow_limit(&market_id, &ALICE).expect("impossible");
		assert!(borrow_limit > 0);

		assert_extrinsic_event::<Runtime>(
			Lending::borrow(Origin::signed(*ALICE), market_id, borrow_limit),
			Event::Lending(crate::Event::Borrowed {
				sender: *ALICE,
				amount: borrow_limit,
				market_id,
			}),
		);

		let liquidate_index =
			Lending::get_functionality_index(Functionality::Liquidate).try_into().unwrap();
		// pausing should go through
		assert_extrinsic_event::<Runtime>(
			Lending::update_market_functionality(
				Origin::signed(*ALICE),
				market_id,
				vec![(liquidate_index, true)],
			),
			Event::Lending(crate::Event::FunctionalityChanged {
				market_id,
				changed_functionalities: vec![(6, true)],
			}),
		);
		let (_, market_state) = Lending::get_market_state(&market_id).unwrap();
		// market state should have changed
		assert_eq!(
			market_state.is_paused_functionalities,
			vec![false, false, false, false, false, false, true]
		);

		process_and_progress_blocks::<Lending, Runtime>(10_000);

		assert_noop!(
			Lending::liquidate(
				Origin::signed(*ALICE),
				market_id.clone(),
				TestBoundedVec::try_from(vec![*ALICE]).unwrap(),
			),
			crate::Error::<Runtime>::LiquidatePaused
		);

		process_and_progress_blocks::<Lending, Runtime>(10_000);

		assert_noop!(
			Lending::liquidate(
				Origin::signed(*ALICE),
				market_id.clone(),
				TestBoundedVec::try_from(vec![*ALICE]).unwrap(),
			),
			crate::Error::<Runtime>::LiquidatePaused
		);

		// manager unpauses
		assert_ok!(Lending::update_market_functionality(
			Origin::signed(*ALICE),
			market_id,
			vec![(liquidate_index, false)]
		));
		let (_, market_state) = Lending::get_market_state(&market_id).unwrap();
		// market state should have changed
		assert_eq!(market_state.is_paused_functionalities, vec![false; 7]);

		assert_extrinsic_event::<Runtime>(
			Lending::liquidate(
				Origin::signed(*ALICE),
				market_id.clone(),
				TestBoundedVec::try_from(vec![*ALICE]).unwrap(),
			),
			Event::Lending(crate::Event::LiquidationInitiated {
				market_id,
				borrowers: vec![*ALICE],
			}),
		);
		// Check if cleanup was done correctly
		assert!(!crate::DebtIndex::<Runtime>::contains_key(market_id, *ALICE));
		assert!(!crate::BorrowTimestamp::<Runtime>::contains_key(market_id, *ALICE));
	});
}

// return all combinations of vec with length desired_len where items are booleans
fn get_bool_vec(num: i32, desired_len: usize) -> (Vec<(u8, bool)>, Vec<bool>) {
	let mut single_position = 1;
	let mut index = 0;
	let mut changed_functionalities = Vec::new();
	let mut result_functionalities = Vec::new();
	while single_position <= num {
		if single_position & num != 0 {
			changed_functionalities.push((index, true));
			result_functionalities.push(true);
		} else {
			result_functionalities.push(false);
		}
		single_position = single_position << 1;
		index += 1;
	}
	while desired_len > result_functionalities.len() {
		result_functionalities.push(false);
	}
	(changed_functionalities, result_functionalities)
}

#[test]
fn test_get_bool_vec() {
	new_test_ext().execute_with(|| {
		assert_eq!(get_bool_vec(0, 4), (vec![], vec![false; 4]));
		assert_eq!(
			get_bool_vec(7, 4),
			(vec![(0, true), (1, true), (2, true)], vec![true, true, true, false])
		);
		assert_eq!(
			get_bool_vec(9, 4),
			(vec![(0, true), (3, true)], vec![true, false, false, true])
		);
	});
}

#[test]
fn test_pausing_global() {
	new_test_ext().execute_with(|| {
		let (market_id_1, _vault_id_1) = create_simple_market();
		let (market_id_2, _vault_id_2) = create_simple_market();
		assert!(market_id_1 != market_id_2);
		let (_, market_state_1) = Lending::get_market_state(&market_id_1).unwrap();
		let (_, market_state_2) = Lending::get_market_state(&market_id_2).unwrap();
		let functionality_len_1 = market_state_1.is_paused_functionalities.len();
		let functionality_len_2 = market_state_2.is_paused_functionalities.len();
		assert_eq!(functionality_len_1, functionality_len_2);

		for i in 0..2.pow(functionality_len_1) {
			let (changed_functionalities, result_functionalities) =
				get_bool_vec(i, functionality_len_1);
			assert_noop!(
				Lending::update_global_market_functionality(
					Origin::signed(*BOB),
					changed_functionalities.clone()
				),
				crate::Error::<Runtime>::Unauthorized
			);
			// pausing should go through
			assert_extrinsic_event::<Runtime>(
				Lending::update_global_market_functionality(
					Origin::signed(*ALICE),
					changed_functionalities.clone(),
				),
				Event::Lending(crate::Event::GlobalFunctionalityChanged {
					changed_functionalities: changed_functionalities.clone(),
				}),
			);
			let (_, market_state_1) = Lending::get_market_state(&market_id_1).unwrap();
			let (_, market_state_2) = Lending::get_market_state(&market_id_2).unwrap();
			assert_eq!(market_state_1.is_paused_functionalities, result_functionalities.clone());
			assert_eq!(market_state_2.is_paused_functionalities, result_functionalities.clone());
			let undo_changes: Vec<(u8, bool)> = changed_functionalities
				.iter()
				.map(|(i, change)| -> (u8, bool) { (*i, !*change) })
				.collect();
			assert_extrinsic_event::<Runtime>(
				Lending::update_global_market_functionality(
					Origin::signed(*ALICE),
					undo_changes.clone(),
				),
				Event::Lending(crate::Event::GlobalFunctionalityChanged {
					changed_functionalities: undo_changes.clone(),
				}),
			);
			let (_, market_state_1) = Lending::get_market_state(&market_id_1).unwrap();
			let (_, market_state_2) = Lending::get_market_state(&market_id_2).unwrap();
			assert_eq!(market_state_1.is_paused_functionalities, vec![false; functionality_len_1]);
			assert_eq!(market_state_2.is_paused_functionalities, vec![false; functionality_len_2]);
		}
	});
}

prop_compose! {
	fn valid_amount_without_overflow()
		(x in MINIMUM_BALANCE..u64::MAX as Balance) -> Balance {
		x
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(10_000))]

	#[test]
	// used market_collateral_deposit_withdraw_identity test
	fn test_pausing_collateral_withdraw(amount in valid_amount_without_overflow()) {
		new_test_ext().execute_with(|| {
			let (market_id, _) = create_simple_market();
			let before = Tokens::balance( BTC::ID, &ALICE);
			prop_assert_ok!(Tokens::mint_into( BTC::ID, &ALICE, amount));

			assert_extrinsic_event::<Runtime>(
				Lending::deposit_collateral(Origin::signed(*ALICE), market_id, amount, false),
				Event::Lending(crate::Event::CollateralDeposited {
					sender: *ALICE,
					amount,
					market_id,
				}),
			);

			let collateral_withdraw_index = Lending::get_functionality_index(Functionality::WithdrawCollateral).try_into().unwrap();
			// pausing should go through
			assert_extrinsic_event::<Runtime>(
				Lending::update_market_functionality(Origin::signed(*ALICE), market_id, vec![(collateral_withdraw_index, true)]),
				Event::Lending(crate::Event::FunctionalityChanged {
					market_id,
					changed_functionalities: vec![(3, true)]
				}),
			);
			let (_, market_state) = Lending::get_market_state(&market_id).unwrap();
			// market state should have changed
			assert_eq!(market_state.is_paused_functionalities, vec![false, false, false, true, false, false, false]);

			assert_noop!(
				Lending::withdraw_collateral(Origin::signed(*ALICE), market_id, amount),
				crate::Error::<Runtime>::WithdrawCollateralPaused
			);

			// manager unpauses
			assert_ok!(
				Lending::update_market_functionality(Origin::signed(*ALICE), market_id, vec![(collateral_withdraw_index, false)])
			);
			let (_, market_state) = Lending::get_market_state(&market_id).unwrap();
			// market state should have changed
			assert_eq!(market_state.is_paused_functionalities, vec![false; 7]);

			assert_extrinsic_event::<Runtime>(
				Lending::withdraw_collateral(Origin::signed(*ALICE), market_id, amount),
				Event::Lending(crate::Event::CollateralWithdrawn {
					sender: *ALICE,
					amount,
					market_id,
				}),
			);
			prop_assert_eq!(Tokens::balance( BTC::ID, &ALICE) - before, amount);

			Ok(())
		})?;
	}
}
