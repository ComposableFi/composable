use super::prelude::*;
use crate::tests::{borrow, process_and_progress_blocks};
use codec::Decode;
use composable_traits::{lending::TotalDebtWithInterest, vault::Vault as VaultTrait};
use frame_support::traits::{fungible::Mutate as FungibleMutateTrait, fungibles::Mutate};
use sp_runtime::{traits::TrailingZeroInput, Perquintill};

#[test]
fn vault_takes_part_of_borrow_so_cannot_withdraw() {
	new_test_ext().execute_with(|| {
		let (market_id, _vault_id) = create_simple_market();
		let initial_total_cash = Lending::total_available_to_be_borrowed(&market_id).unwrap();
		let deposit_usdt = 1_000_000_000;
		let deposit_btc = 10;
		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, deposit_usdt));
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, deposit_btc));

		assert_ok!(Lending::vault_deposit(RuntimeOrigin::signed(*ALICE), market_id, deposit_btc));
		assert_extrinsic_event::<Runtime>(
			Lending::deposit_collateral(
				RuntimeOrigin::signed(*ALICE),
				market_id,
				deposit_usdt,
				false,
			),
			RuntimeEvent::Lending(pallet_lending::Event::<Runtime>::CollateralDeposited {
				sender: *ALICE,
				market_id,
				amount: deposit_usdt,
			}),
		);
		assert_noop!(
			Lending::borrow(
				RuntimeOrigin::signed(*ALICE),
				market_id.clone(),
				deposit_btc + initial_total_cash
			),
			orml_tokens::Error::<Runtime>::BalanceTooLow
		);
		assert_no_event::<Runtime>(RuntimeEvent::Lending(
			pallet_lending::Event::<Runtime>::Borrowed {
				sender: *ALICE,
				market_id,
				amount: deposit_btc + initial_total_cash,
			},
		));
	});
}

#[test]
fn test_vault_market_can_withdraw() {
	new_test_ext().execute_with(|| {
		let (market, _vault_id) = create_simple_market();

		let collateral = 1_000_000_000_000;
		let borrow = 10;
		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, collateral));
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, borrow));

		assert_ok!(Lending::vault_deposit(RuntimeOrigin::signed(*ALICE), market, borrow));

		assert_extrinsic_event::<Runtime>(
			Lending::deposit_collateral(RuntimeOrigin::signed(*ALICE), market, collateral, false),
			RuntimeEvent::Lending(crate::Event::CollateralDeposited {
				sender: *ALICE,
				amount: collateral,
				market_id: market,
			}),
		);
		process_and_progress_blocks::<Lending, Runtime>(1);
		// We waited 1 block, the market should have withdraw the funds
		assert_extrinsic_event::<Runtime>(
			Lending::borrow(RuntimeOrigin::signed(*ALICE), market, borrow - 1),
			RuntimeEvent::Lending(crate::Event::Borrowed {
				sender: *ALICE,
				amount: borrow - 1, // DEFAULT_MARKET_VAULT_RESERVE
				market_id: market,
			}),
		);
	});
}

// Generates well funded accounts
fn generate_accounts(amount: u128) -> Vec<AccountId> {
	let mut accounts = Vec::new();
	let start = 1000;
	let amount = amount.saturating_add(start);
	for num in start..amount {
		let account = AccountId::decode(&mut TrailingZeroInput::new(&num.to_be_bytes())).unwrap();
		Balances::mint_into(&account, PICA::units(1_000_000_000)).unwrap();
		Tokens::mint_into(USDT::ID, &account, USDT::units(100_000)).unwrap();
		accounts.push(account);
	}
	accounts
}

prop_compose! {
	// Generates following inputs:
	// borrowers_amount: amount of borrowers involved in the test,
	// borrowed_amount_per_borrower: personal borrow size,
	// reserved_factor: part of assets which should hold on vault's account
	fn inputs_for_test_vault_balance()
		(   borrowers_amount in 100..501u128,
			borrowed_amount_per_borrower in 100..1001u128,
			reserved_factor in 1..100u128,
			) -> (u128, u128, u128) {
			(borrowers_amount, borrowed_amount_per_borrower, reserved_factor)
		}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(10))]

	#[test]
	fn test_vault_balance((borrowers_amount, borrowed_amount_per_borrower, reserved_factor) in inputs_for_test_vault_balance()) {
	let mut ext = new_test_ext();
		ext.execute_with(|| {
			let manager = *ALICE;
			let lender = *CHARLIE;
			// Individual borrow's part which is going to be returned each block
			let return_factor = FixedU128::saturating_from_rational(25, 100);
			let partial_return_amount:u128 =
				(FixedU128::from_inner(borrowed_amount_per_borrower) * return_factor).into_inner();
			// Total amount which should be minted onto lender account
			let total_amount =
				(FixedU128::from_inner(borrowers_amount * borrowed_amount_per_borrower) /
					FixedU128::saturating_from_rational(100 - reserved_factor, 100u128))
				.into_inner();
			// Creates market with USDT as borrow asset and BTC as collateral asset.
			// 1 BTC = 50_000 USDT, reserved factor is defined from test's inputs.
			let (market_id, vault_id) = create_market::<Runtime, 50_000>(
				USDT::instance(),
				BTC::instance(),
				manager,
				Perquintill::from_percent(reserved_factor as u64),
				MoreThanOneFixedU128::saturating_from_integer(DEFAULT_COLLATERAL_FACTOR),
			);
			let market_account = Lending::account_id(&market_id);
			let vault_account = Vault::account_id(&vault_id);
			// Deposit USDT in the vault.
			prop_assert_ok!(Tokens::mint_into(USDT::ID, &lender, USDT::units(total_amount)));
			prop_assert_ok!(Lending::vault_deposit(RuntimeOrigin::signed(lender), market_id, USDT::units(total_amount)));
			// Process one block to transfer not-reserved assets to the corresponded market.
			process_and_progress_blocks::<Lending, Runtime>(1);
			// Generate a bunch of borrowers' accounts.
			let borrowers = generate_accounts(borrowers_amount);
			for borrower in &borrowers {
			// Deposit 100 BTC collateral from borrower account.
				mint_and_deposit_collateral::<Runtime>(
					*borrower,
					BTC::units(100),
					market_id,
					BTC::ID,
				);
				borrow::<Runtime>(*borrower, market_id, USDT::units(borrowed_amount_per_borrower));
			}
			// For some reason lender needs some of his money back.
			// So, he withdraw all assets from vault's account.
			prop_assert_ok!(Lending::vault_withdraw(
				RuntimeOrigin::signed(lender),
				market_id,
				Assets::balance(USDT::ID, &vault_account)
			));
			process_and_progress_blocks::<Lending, Runtime>(1);
			//Now vault is unbalanced and should restore equilibrium state.
			 while Lending::ensure_can_borrow_from_vault(&vault_id, &market_account).is_err() {
				for borrower in &borrowers {
					// Sometimes partial_return_amount can exceed total debt.
					// In such cases we have to repay actual debt balance.
					let borrower_total_debt = <Lending as LendingTrait>::total_debt_with_interest(&market_id, &borrower).unwrap();
					let partial_return_amount = if let TotalDebtWithInterest::Amount(debt) = borrower_total_debt {
						if debt.div(USDT::ONE) < partial_return_amount {
							debt.div(USDT::ONE)
						} else {
							partial_return_amount
						}
					} else {
						partial_return_amount
					};
					<Lending as LendingTrait>::repay_borrow(
						&market_id,
						borrower,
						borrower,
						RepayStrategy::PartialAmount(USDT::units(partial_return_amount)),
						false,
					)
					.unwrap();
					process_and_progress_blocks::<Lending, Runtime>(1);
				}
			}
			// Vault should be balanced.
			prop_assert!(Lending::ensure_can_borrow_from_vault(&vault_id, &market_account).is_ok());
			// Lender decided withdraw money from the vault again.
			prop_assert_ok!(Lending::vault_withdraw(
						RuntimeOrigin::signed(lender),
						market_id,
						Assets::balance(USDT::ID, &vault_account)
					));
			// Vault is unbalanced
			assert!(Lending::ensure_can_borrow_from_vault(&vault_id, &market_account).is_err());
			// Refresh assets prices
			set_price(USDT::ID, NORMALIZED::ONE);
			set_price(BTC::ID, NORMALIZED::units(50_000));

			// Check that we can not borrow from market related to unbalanced vault
			prop_assert_noop!(Lending::borrow(RuntimeOrigin::signed(*borrowers.get(0).unwrap()), market_id, Assets::balance(USDT::ID, &market_account)),
				Error::<Runtime>::CannotBorrowFromMarketWithUnbalancedVault);
			process_and_progress_blocks::<Lending, Runtime>(1);

			// Lender puts back assets to the vault.
			prop_assert_ok!(Tokens::mint_into(USDT::ID, &lender, USDT::units(total_amount)));
			prop_assert_ok!(Lending::vault_deposit(RuntimeOrigin::signed(lender), market_id, USDT::units(total_amount)));
			process_and_progress_blocks::<Lending, Runtime>(1);
			// Vault is balanced.
			assert!(Lending::ensure_can_borrow_from_vault(&vault_id, &market_account).is_ok());
			Ok(())
		})?;
	}
}
