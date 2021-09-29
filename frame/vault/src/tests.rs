use crate::{
	mocks::{
		currency_factory::MockCurrencyId,
		tests::{
			AccountId, Balance, BlockNumber, ExtBuilder, Origin, Test, Tokens, Vaults,
			ACCOUNT_FREE_START, ALICE, BOB, CHARLIE, MINIMUM_BALANCE,
		},
	},
	models::VaultInfo,
	*,
};
use composable_traits::vault::{Deposit, FundsAvailability, StrategicVault, Vault, VaultConfig};
use frame_support::{
	assert_noop, assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use proptest::prelude::*;
use sp_runtime::{helpers_128bit::multiply_by_rational, Perquintill};

/// Missing macro, equivalent to `assert_ok!`!
macro_rules! prop_assert_ok {
    ($cond:expr) => {
        prop_assert_ok!($cond, concat!("assertion failed: ", stringify!($cond)))
    };

    ($cond:expr, $($fmt:tt)*) => {
        if let Err(e) = $cond {
            let message = format!($($fmt)*);
            let message = format!("Expected Ok(_), got {:?}, {} at {}:{}", e, message, file!(), line!());
            return ::std::result::Result::Err(
                proptest::test_runner::TestCaseError::fail(message));
        }
    };
}

/// Accept a 'dust' deviation
macro_rules! prop_assert_epsilon {
	($x:expr, $y:expr) => {{
		let precision = 1000;
		let epsilon = 10;
		let upper = precision + epsilon;
		let lower = precision - epsilon;
		let q = multiply_by_rational($x, precision, $y).expect("qed;");
		prop_assert!(
			upper >= q && q >= lower,
			"({}) => {} >= {} * {} / {} >= {}",
			q,
			upper,
			$x,
			precision,
			$y,
			lower
		);
	}};
}

fn create_vault_with_share(
	asset_id: MockCurrencyId,
	strategy_account_id: AccountId,
	strategy_share: Perquintill,
	reserved: Perquintill,
) -> (VaultIndex, VaultInfo<AccountId, Balance, MockCurrencyId, BlockNumber>) {
	let v = Vaults::do_create_vault(
		Deposit::Existential,
		VaultConfig {
			asset_id,
			manager: ALICE,
			reserved,
			strategies: [(strategy_account_id, strategy_share)].iter().cloned().collect(),
		},
	);
	assert_ok!(&v);
	v.expect("unreachable; qed;")
}

fn create_vault(
	strategy_account_id: AccountId,
	asset_id: MockCurrencyId,
) -> (VaultIndex, VaultInfo<AccountId, Balance, MockCurrencyId, BlockNumber>) {
	create_vault_with_share(
		asset_id,
		strategy_account_id,
		Perquintill::from_percent(90),
		Perquintill::from_percent(10),
	)
}

prop_compose! {
	fn valid_amounts_without_overflow_1()
		(x in MINIMUM_BALANCE..Balance::MAX) -> Balance {
		x
	}
}

prop_compose! {
	fn valid_amounts_without_overflow_2()
		(x in MINIMUM_BALANCE..Balance::MAX / 2,
		 y in MINIMUM_BALANCE..Balance::MAX / 2) -> (Balance, Balance) {
			(x, y)
	}
}

prop_compose! {
	fn valid_amounts_without_overflow_3()
		(x in MINIMUM_BALANCE..Balance::MAX / 3,
		 y in MINIMUM_BALANCE..Balance::MAX / 3,
		 z in MINIMUM_BALANCE..Balance::MAX / 3) -> (Balance, Balance, Balance) {
			(x, y, z)
		}
}

prop_compose! {
	fn valid_amounts_without_overflow_k
		(max_accounts: usize, limit: Balance)
		(balances in prop::collection::vec(MINIMUM_BALANCE..limit, 3..max_accounts))
		 -> Vec<(AccountId, Balance)> {
			(ACCOUNT_FREE_START..balances.len() as AccountId)
				.zip(balances)
				.collect()
		}
}

prop_compose! {
	fn valid_amounts_without_overflow_k_with_random_index(max_accounts: usize, limit: Balance)
		(accounts in valid_amounts_without_overflow_k(max_accounts, limit),
		 index in 1..max_accounts) -> (usize, Vec<(AccountId, Balance)>) {
			(usize::max(1, index % usize::max(1, accounts.len())), accounts)
		}
}

prop_compose! {
	fn strategy_account()
		(x in ACCOUNT_FREE_START..AccountId::MAX) -> AccountId {
			x
		}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(10000))]

	#[test]
	fn vault_strategy_withdraw_deposit_identity(
		strategy_account_id in strategy_account(),
		total_funds in valid_amounts_without_overflow_1()
	) {
		let asset_id = MockCurrencyId::A;
		let strategy_share = Perquintill::from_percent(80);
		let reserve_share = Perquintill::from_percent(20);
		ExtBuilder::default().build().execute_with(|| {
			let (vault_id, _) = create_vault_with_share(
				asset_id,
				strategy_account_id,
				strategy_share,
				reserve_share,
			);

			prop_assert_eq!(Tokens::balance(asset_id, &ALICE), 0);
			prop_assert_ok!(Tokens::mint_into(asset_id, &ALICE, total_funds));
			prop_assert_eq!(Tokens::balance(asset_id, &ALICE), total_funds);

			prop_assert_ok!(Vaults::deposit(Origin::signed(ALICE), vault_id, total_funds));

			let expected_strategy_funds =
				strategy_share.mul_floor(total_funds);

			let available_funds = <Vaults as StrategicVault>::available_funds(&vault_id, &strategy_account_id);
			prop_assert!(
				matches!(
					available_funds,
					Ok(FundsAvailability::Withdrawable(strategy_funds))
						if expected_strategy_funds <= strategy_funds
						// && strategy_funds <= expected_strategy_funds + 1
				),
				"Reserve should now be 20% of initial strategy funds, expected: {}, actual: {:?}",
				expected_strategy_funds,
				available_funds
			);

			// Strategy withdraw/deposit full allocation
			prop_assert_eq!(Tokens::balance(asset_id, &strategy_account_id), 0);
			prop_assert_ok!(<Vaults as StrategicVault>::withdraw(&vault_id, &strategy_account_id, expected_strategy_funds));
			prop_assert_eq!(Tokens::balance(asset_id, &strategy_account_id), expected_strategy_funds);
			prop_assert_ok!(<Vaults as StrategicVault>::deposit(&vault_id, &strategy_account_id, expected_strategy_funds));
			prop_assert_eq!(Tokens::balance(asset_id, &strategy_account_id), 0);

			Ok(())
		})?;
	}

	#[test]
	fn vault_reserve_rebalance_ask_strategy_to_deposit(
		strategy_account_id in strategy_account(),
		total_funds in valid_amounts_without_overflow_1()
	) {
		let asset_id = MockCurrencyId::A;
		let strategy_share = Perquintill::from_percent(80);
		let reserve_share = Perquintill::from_percent(20);
		ExtBuilder::default().build().execute_with(|| {
			let (vault_id, _) = create_vault_with_share(
				asset_id,
				strategy_account_id,
				strategy_share,
				reserve_share,
			);

			prop_assert_eq!(Tokens::balance(asset_id, &ALICE), 0);
			prop_assert_ok!(Tokens::mint_into(asset_id, &ALICE, total_funds));
			prop_assert_eq!(Tokens::balance(asset_id, &ALICE), total_funds);

			prop_assert_ok!(Vaults::deposit(Origin::signed(ALICE), vault_id, total_funds));

			let expected_strategy_funds =
				strategy_share.mul_floor(total_funds);

			let available_funds = <Vaults as StrategicVault>::available_funds(&vault_id, &strategy_account_id);
			prop_assert!(
				matches!(
					available_funds,
					Ok(FundsAvailability::Withdrawable(strategy_funds))
						if expected_strategy_funds <= strategy_funds
						&& strategy_funds <= expected_strategy_funds + 1
				),
				"Reserve should now be 20% of initial strategy funds, expected: {}, actual: {:?}",
				expected_strategy_funds,
				available_funds
			);

			// Strategy withdraw full allocation
			prop_assert_eq!(Tokens::balance(asset_id, &strategy_account_id), 0);
			prop_assert_ok!(<Vaults as StrategicVault>::withdraw(&vault_id, &strategy_account_id, expected_strategy_funds));
			prop_assert_eq!(Tokens::balance(asset_id, &strategy_account_id), expected_strategy_funds);

			// User withdraw from the reserve
			let reserve = total_funds - expected_strategy_funds;
			prop_assert_ok!(
				Vaults::withdraw(
					Origin::signed(ALICE),
					vault_id,
					reserve
				)
			);

			let new_expected_reserve =
				reserve_share.mul_floor(expected_strategy_funds);

			// The vault should ask for the strategy to deposit in order to rebalance
			let new_available_funds =
				<Vaults as StrategicVault>::available_funds(&vault_id, &strategy_account_id);

			prop_assert!(
				matches!(
					new_available_funds,
					Ok(FundsAvailability::Depositable(new_reserve))
						if new_expected_reserve <= new_reserve
						// && new_reserve <= new_expected_reserve + 1
				),
				"Reserve should now be 20% of 80% of total funds = 20% of initial strategy funds, expected: {}, actual: {:?}",
				new_expected_reserve,
				new_available_funds
			);

			Ok(())
		})?;
	}

	#[test]
	fn vault_single_deposit_withdraw_asset_identity(
		strategy_account_id in strategy_account(),
		amount in valid_amounts_without_overflow_1()
	) {
		let asset_id = MockCurrencyId::A;
		ExtBuilder::default().build().execute_with(|| {
			let (vault_id, _) = create_vault(strategy_account_id, asset_id);

			prop_assert_eq!(Tokens::balance(asset_id, &ALICE), 0);
			prop_assert_ok!(Tokens::mint_into(asset_id, &ALICE, amount));
			prop_assert_eq!(Tokens::balance(asset_id, &ALICE), amount);

			prop_assert_ok!(Vaults::deposit(Origin::signed(ALICE), vault_id, amount));
			prop_assert_ok!(Vaults::withdraw(Origin::signed(ALICE), vault_id, amount));

			prop_assert_eq!(Tokens::balance(asset_id, &ALICE), amount);
			Ok(())
		})?;
	}

	#[test]
	fn vault_multi_deposit_withdraw_asset_identity(
		strategy_account_id in strategy_account(),
		(amount1, amount2, amount3) in valid_amounts_without_overflow_3()
	) {
		let asset_id = MockCurrencyId::A;
		ExtBuilder::default().build().execute_with(|| {
			let (vault_id, _) = create_vault(strategy_account_id, asset_id);

			prop_assert_eq!(Tokens::balance(asset_id, &ALICE), 0);
			prop_assert_eq!(Tokens::balance(asset_id, &BOB), 0);
			prop_assert_eq!(Tokens::balance(asset_id, &CHARLIE), 0);
			prop_assert_ok!(Tokens::mint_into(asset_id, &ALICE, amount1));
			prop_assert_ok!(Tokens::mint_into(asset_id, &BOB, amount2));
			prop_assert_ok!(Tokens::mint_into(asset_id, &CHARLIE, amount3));

			prop_assert_eq!(Tokens::balance(asset_id, &BOB), amount2);
			prop_assert_eq!(Tokens::balance(asset_id, &ALICE), amount1);
			prop_assert_eq!(Tokens::balance(asset_id, &CHARLIE), amount3);

			prop_assert_ok!(Vaults::deposit(Origin::signed(CHARLIE), vault_id, amount3));
			prop_assert_ok!(Vaults::deposit(Origin::signed(BOB), vault_id, amount2));
			prop_assert_ok!(Vaults::deposit(Origin::signed(ALICE), vault_id, amount1));

			prop_assert_ok!(Vaults::withdraw(Origin::signed(ALICE), vault_id, amount1));
			prop_assert_ok!(Vaults::withdraw(Origin::signed(CHARLIE), vault_id, amount3));
			prop_assert_ok!(Vaults::withdraw(Origin::signed(BOB), vault_id, amount2));

			prop_assert_eq!(Tokens::balance(asset_id, &ALICE), amount1);
			prop_assert_eq!(Tokens::balance(asset_id, &BOB), amount2);
			prop_assert_eq!(Tokens::balance(asset_id, &CHARLIE), amount3);

			Ok(())
		})?;
	}

	#[test]
	fn vault_single_deposit_lp_ratio_asset_is_one(
		strategy_account_id in strategy_account(),
		amount in valid_amounts_without_overflow_1()
	)
	{
		let asset_id = MockCurrencyId::B;
		ExtBuilder::default().build().execute_with(|| {
			let (vault_id, vault_info) = create_vault(strategy_account_id, asset_id);
			prop_assert_eq!(Tokens::balance(asset_id, &ALICE), 0);
			prop_assert_ok!(Tokens::mint_into(asset_id, &ALICE, amount));

			prop_assert_eq!(Tokens::balance(vault_info.lp_token_id, &ALICE),  0);

			prop_assert_ok!(Vaults::deposit(Origin::signed(ALICE), vault_id, amount));

			prop_assert_eq!(Tokens::balance(vault_info.lp_token_id, &ALICE), amount);
			Ok(())
		})?;
	}

	#[test]
	fn vault_withdraw_with_zero_lp_issued_fails_to_burn(
		strategy_account_id in strategy_account(),
		amount in valid_amounts_without_overflow_1()
	) {
		let asset_id = MockCurrencyId::C;
		ExtBuilder::default().build().execute_with(|| {
			let (vault_id, vault) = create_vault(strategy_account_id, asset_id);
			prop_assert_eq!(Tokens::balance(vault.lp_token_id, &ALICE), 0);
			assert_noop!(Vaults::withdraw(Origin::signed(ALICE), vault_id, amount), Error::<Test>::InsufficientLpTokens);
			Ok(())
		})?;
	}

	#[test]
	fn vault_withdraw_without_depositing_fails_to_burn(
		strategy_account_id in strategy_account(),
		amount in valid_amounts_without_overflow_1()
	) {
		let asset_id = MockCurrencyId::D;
		ExtBuilder::default().build().execute_with(|| {
			let (vault_id, vault) = create_vault(strategy_account_id, asset_id);
			prop_assert_eq!(Tokens::balance(asset_id, &ALICE), 0);
			prop_assert_ok!(Tokens::mint_into(asset_id, &ALICE, amount));
			prop_assert_ok!(Vaults::deposit(Origin::signed(ALICE), vault_id, amount));

			prop_assert_eq!(Tokens::balance(vault.lp_token_id, &BOB), 0);
			assert_noop!(Vaults::withdraw(Origin::signed(BOB), vault_id, amount), Error::<Test>::InsufficientLpTokens);
			Ok(())
		})?;
	}

	#[test]
	fn vault_stock_dilution_1(
		strategy_account_id in strategy_account(),
		(amount1, amount2, strategy_profits) in valid_amounts_without_overflow_3()
	) {
		let asset_id = MockCurrencyId::D;
		ExtBuilder::default().build().execute_with(|| {
			let (vault_id, vault) = create_vault(strategy_account_id, asset_id);
			prop_assert_eq!(Tokens::balance(asset_id, &ALICE), 0);
			prop_assert_eq!(Tokens::balance(asset_id, &BOB), 0);
			prop_assert_eq!(Tokens::balance(asset_id, &strategy_account_id), 0);

			prop_assert_ok!(Tokens::mint_into(asset_id, &ALICE, amount1));
			prop_assert_ok!(Tokens::mint_into(asset_id, &BOB, amount2));
			prop_assert_ok!(Tokens::mint_into(asset_id, &strategy_account_id, strategy_profits));

			prop_assert_ok!(Vaults::deposit(Origin::signed(ALICE), vault_id, amount1));
			prop_assert_ok!(<Vaults as StrategicVault>::deposit(&vault_id, &strategy_account_id, strategy_profits));
			prop_assert_ok!(Vaults::deposit(Origin::signed(BOB), vault_id, amount2));

			let alice_lp = Tokens::balance(vault.lp_token_id, &ALICE);
			let bob_lp = Tokens::balance(vault.lp_token_id, &BOB);

			prop_assert_ok!(Vaults::withdraw(Origin::signed(ALICE), vault_id, alice_lp));
			prop_assert_ok!(Vaults::withdraw(Origin::signed(BOB), vault_id, bob_lp));

			let alice_total_balance = Tokens::balance(asset_id, &ALICE);
			let bob_total_balance = Tokens::balance(asset_id, &BOB);
			let strategy_total_balance = Tokens::balance(asset_id, &strategy_account_id);

			prop_assert_epsilon!(alice_total_balance, amount1 + strategy_profits);

			prop_assert_epsilon!(
				alice_total_balance + bob_total_balance + strategy_total_balance,
				amount1 + amount2 + strategy_profits
			);

			Ok(())
		})?;
	}

	#[test]
	fn vault_are_isolated(
		strategy_account_id in strategy_account(),
		(amount1, amount2) in valid_amounts_without_overflow_2()
	) {
		let asset_id = MockCurrencyId::D;
		ExtBuilder::default().build().execute_with(|| {

			// Create two vaults based on the same asset
			let (vault_id1, _) = create_vault(strategy_account_id, asset_id);
			let (vault_id2, _) = create_vault(strategy_account_id, asset_id);

			// Ensure vaults are unique
			prop_assert_ne!(vault_id1, vault_id2);
			prop_assert_ne!(Vaults::account_id(&vault_id1), Vaults::account_id(&vault_id2));

			// Alice deposit an amount in vault 1
			prop_assert_eq!(Tokens::balance(asset_id, &Vaults::account_id(&vault_id1)), 0);
			prop_assert_eq!(Tokens::balance(asset_id, &ALICE), 0);
			prop_assert_ok!(Tokens::mint_into(asset_id, &ALICE, amount1));
			prop_assert_ok!(Vaults::deposit(Origin::signed(ALICE), vault_id1, amount1));

			// Bob deposit an amount in vault 2
			prop_assert_eq!(Tokens::balance(asset_id, &Vaults::account_id(&vault_id2)), 0);
			prop_assert_eq!(Tokens::balance(asset_id, &BOB), 0);
			prop_assert_ok!(Tokens::mint_into(asset_id, &BOB, amount2));
			prop_assert_ok!(Vaults::deposit(Origin::signed(BOB), vault_id2, amount2));

			// The funds should not be shared.
			prop_assert_eq!(Tokens::balance(asset_id, &Vaults::account_id(&vault_id1)), amount1);
			prop_assert_eq!(Tokens::balance(asset_id, &Vaults::account_id(&vault_id2)), amount2);

			Ok(())
		})?;
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(100))]

	#[test]
	fn vault_stock_dilution_k(
		(random_index, created_accounts) in
			valid_amounts_without_overflow_k_with_random_index(500, 1_000_000_000)
				.prop_filter("a minimum of two accounts are required, 1 for the strategy and 1 depositor",
							 |(_, x)| x.len() > 2)
	) {
		let asset_id = MockCurrencyId::D;
		let (strategy_account_id, strategy_profits) = created_accounts[0];
		let strategy_deposit_moment = random_index;
		let account_start = 1;
		let accounts = &created_accounts[account_start..];

		ExtBuilder::default().build().execute_with(|| {
			let (vault_id, vault) = create_vault(strategy_account_id, asset_id);

			prop_assert_eq!(Tokens::balance(asset_id, &strategy_account_id), 0);
			prop_assert_ok!(Tokens::mint_into(asset_id, &strategy_account_id, strategy_profits));

			for (account, balance) in accounts.iter().copied() {
				prop_assert_eq!(Tokens::balance(asset_id, &account), 0);
				prop_assert_ok!(Tokens::mint_into(asset_id, &account, balance));
			}

			// Shareholders
			for (account, balance) in accounts[0..strategy_deposit_moment].iter().copied() {
				prop_assert_ok!(
					Vaults::deposit(Origin::signed(account), vault_id, balance)
				);
			}

			// Profits comming
			prop_assert_ok!(
				<Vaults as StrategicVault>::deposit(
					&vault_id,
					&strategy_account_id,
					strategy_profits
				)
			);

			// Shareholders total LP
			let lp_total = Tokens::total_issuance(vault.lp_token_id);

			// Users depositing later
			for (account, balance) in accounts[strategy_deposit_moment..accounts.len()].iter().copied() {
				prop_assert_ok!(
					Vaults::deposit(Origin::signed(account), vault_id, balance)
				);
			}

			// Withdraw & local check
			for ((account, balance), index) in accounts.iter().copied().zip(account_start..) {
				// Current lp
				let lp = Tokens::balance(vault.lp_token_id, &account);

				// Withdraw all my shares, including profits
				prop_assert_ok!(Vaults::withdraw(Origin::signed(account), vault_id, lp));

				// Balance after having deposited + withdrawn my funds
				let new_balance = Tokens::balance(asset_id, &account);

				// We had shares before the profit, we get a cut of the profit
				if index <= strategy_deposit_moment {
					// Compute my share
					let strategy_profit_share =
						multiply_by_rational(strategy_profits, lp, lp_total).expect("qed;");

					prop_assert_epsilon!(new_balance, balance + strategy_profit_share);
				}
				else {
					// Our balance should be equivalent
					prop_assert_epsilon!(new_balance, balance);
				}
			}

			// Global check
			let shareholders = &accounts[0..strategy_deposit_moment];
			let initial_sum_of_shareholders_balance = shareholders.iter()
				.map(|(_, initial_balance)| initial_balance)
				.sum::<Balance>();
			let current_sum_of_shareholders_balance = shareholders.iter()
				.map(|(account, _)| Tokens::balance(asset_id, account))
				.sum::<Balance>();

			prop_assert_epsilon!(
				current_sum_of_shareholders_balance,
				initial_sum_of_shareholders_balance + strategy_profits
			);

			Ok(())
		})?;
	}
}
