use crate::mocks::currency_factory::MockCurrencyId;
use crate::models::VaultInfo;
use crate::*;
use crate::{mocks::*, models::VaultConfig};
use composable_traits::vault::StrategicVault;
use frame_support::traits::fungibles::{Inspect, Mutate};
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;
use sp_runtime::Perquintill;

fn create_vault(
    strategy_account_id: AccountId,
    asset_id: MockCurrencyId,
) -> (VaultIndex, VaultInfo<MockCurrencyId>) {
    let v = Vault::do_create_vault(VaultConfig {
        asset_id,
        reserved: Perquintill::from_percent(10),
        strategies: [(strategy_account_id, Perquintill::from_percent(90))]
            .iter()
            .cloned()
            .collect(),
    });
    assert_ok!(&v);
    v.expect("unreachable; qed;")
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
        (max_accounts: usize)
        (balances in prop::collection::vec(MINIMUM_BALANCE..Balance::MAX / max_accounts as Balance, 3..max_accounts))
         -> Vec<(AccountId, Balance)> {
            (ACCOUNT_FREE_START..balances.len() as AccountId)
                .zip(balances)
                .collect()
        }
}

prop_compose! {
    fn valid_amounts_without_overflow_k_with_random_index(max_accounts: usize)
        (accounts in valid_amounts_without_overflow_k(max_accounts),
         index in 1..max_accounts) -> (usize, Vec<(AccountId, Balance)>) {
            (index, accounts)
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
    fn vault_single_deposit_withdraw_asset_identity(
        strategy_account_id in strategy_account(),
        amount in valid_amounts_without_overflow_1()
    ) {
        let asset_id = MockCurrencyId::A;
        let _ = ExtBuilder::default().build().execute_with(|| {
            let (vault_id, _) = create_vault(strategy_account_id, asset_id);

            prop_assert!(Tokens::balance(asset_id, &ALICE) == 0);
            assert_ok!(Tokens::mint_into(asset_id, &ALICE, amount));

            prop_assert_eq!(Tokens::balance(asset_id, &ALICE), amount);

            assert_ok!(Vault::deposit(Origin::signed(ALICE), vault_id, amount));
            assert_ok!(Vault::withdraw(Origin::signed(ALICE), vault_id, amount));

            prop_assert!(Tokens::balance(asset_id, &ALICE) == amount);
            Ok(())
        });
    }

    #[test]
    fn vault_multi_deposit_withdraw_asset_identity(
        strategy_account_id in strategy_account(),
        (amount1, amount2, amount3) in valid_amounts_without_overflow_3()
    ) {
        let asset_id = MockCurrencyId::A;
        let _ = ExtBuilder::default().build().execute_with(|| {
            let (vault_id, _) = create_vault(strategy_account_id, asset_id);

            prop_assert!(Tokens::balance(asset_id, &ALICE) == 0);
            prop_assert!(Tokens::balance(asset_id, &BOB) == 0);
            prop_assert!(Tokens::balance(asset_id, &CHARLIE) == 0);
            assert_ok!(Tokens::mint_into(asset_id, &ALICE, amount1));
            assert_ok!(Tokens::mint_into(asset_id, &BOB, amount2));
            assert_ok!(Tokens::mint_into(asset_id, &CHARLIE, amount3));

            prop_assert!(Tokens::balance(asset_id, &BOB) == amount2);
            prop_assert!(Tokens::balance(asset_id, &ALICE) == amount1);
            prop_assert!(Tokens::balance(asset_id, &CHARLIE) == amount3);

            assert_ok!(Vault::deposit(Origin::signed(CHARLIE), vault_id, amount3));
            assert_ok!(Vault::deposit(Origin::signed(BOB), vault_id, amount2));
            assert_ok!(Vault::deposit(Origin::signed(ALICE), vault_id, amount1));

            assert_ok!(Vault::withdraw(Origin::signed(ALICE), vault_id, amount1));
            assert_ok!(Vault::withdraw(Origin::signed(CHARLIE), vault_id, amount3));
            assert_ok!(Vault::withdraw(Origin::signed(BOB), vault_id, amount2));

            prop_assert!(Tokens::balance(asset_id, &ALICE) == amount1);
            prop_assert!(Tokens::balance(asset_id, &BOB) == amount2);
            prop_assert!(Tokens::balance(asset_id, &CHARLIE) == amount3);

            Ok(())
        });
    }

    #[test]
    fn vault_single_deposit_lp_ratio_asset_is_one(
        strategy_account_id in strategy_account(),
        amount in valid_amounts_without_overflow_1()
    )
    {
        let asset_id = MockCurrencyId::B;
        let _ = ExtBuilder::default().build().execute_with(|| {
            let (vault_id, vault_info) = create_vault(strategy_account_id, asset_id);
            prop_assert!(Tokens::balance(asset_id, &ALICE) == 0);
            assert_ok!(Tokens::mint_into(asset_id, &ALICE, amount));

            prop_assert_eq!(Tokens::balance(vault_info.lp_token_id, &ALICE), 0);

            assert_ok!(Vault::deposit(Origin::signed(ALICE), vault_id, amount));

            prop_assert_eq!(Tokens::balance(vault_info.lp_token_id, &ALICE), amount);
            Ok(())
        });
    }

    #[test]
    fn vault_withdraw_with_zero_lp_issued_fails_to_burn(
        strategy_account_id in strategy_account(),
        amount in valid_amounts_without_overflow_1()
    ) {
        let asset_id = MockCurrencyId::C;
        let _ = ExtBuilder::default().build().execute_with(|| {
            let (vault_id, vault) = create_vault(strategy_account_id, asset_id);
            prop_assert!(Tokens::balance(vault.lp_token_id, &ALICE) == 0);
            assert_noop!(Vault::withdraw(Origin::signed(ALICE), vault_id, amount), Error::<Test>::InsufficientLpTokens);
            Ok(())
        });
    }

    #[test]
    fn vault_withdraw_without_depositing_fails_to_burn(
        strategy_account_id in strategy_account(),
        amount in valid_amounts_without_overflow_1()
    ) {
        let asset_id = MockCurrencyId::D;
        let _ = ExtBuilder::default().build().execute_with(|| {
            let (vault_id, vault) = create_vault(strategy_account_id, asset_id);
            prop_assert!(Tokens::balance(asset_id, &ALICE) == 0);
            assert_ok!(Tokens::mint_into(asset_id, &ALICE, amount));
            assert_ok!(Vault::deposit(Origin::signed(ALICE), vault_id, amount));

            prop_assert!(Tokens::balance(vault.lp_token_id, &BOB) == 0);
            assert_noop!(Vault::withdraw(Origin::signed(BOB), vault_id, amount), Error::<Test>::InsufficientLpTokens);
            Ok(())
        });
    }

    #[test]
    fn vault_stock_dilution_1(
        strategy_account_id in strategy_account(),
        (amount1, amount2, strategy_profits) in valid_amounts_without_overflow_3()
    ) {
        let asset_id = MockCurrencyId::D;
        let _ = ExtBuilder::default().build().execute_with(|| {
            let (vault_id, vault) = create_vault(strategy_account_id, asset_id);
            prop_assert!(Tokens::balance(asset_id, &ALICE) == 0);
            prop_assert!(Tokens::balance(asset_id, &BOB) == 0);
            prop_assert!(Tokens::balance(asset_id, &strategy_account_id) == 0);

            assert_ok!(Tokens::mint_into(asset_id, &ALICE, amount1));
            assert_ok!(Tokens::mint_into(asset_id, &BOB, amount2));
            assert_ok!(Tokens::mint_into(asset_id, &strategy_account_id, strategy_profits));

            assert_ok!(Vault::deposit(Origin::signed(ALICE), vault_id, amount1));
            assert_ok!(<Vault as StrategicVault>::deposit(&vault_id, &strategy_account_id, strategy_profits));
            assert_ok!(Vault::deposit(Origin::signed(BOB), vault_id, amount2));

            let alice_lp = Tokens::balance(vault.lp_token_id, &ALICE);
            let bob_lp = Tokens::balance(vault.lp_token_id, &BOB);

            assert_ok!(Vault::withdraw(Origin::signed(ALICE), vault_id, alice_lp));
            assert_ok!(Vault::withdraw(Origin::signed(BOB), vault_id, bob_lp));

            let alice_total_balance = Tokens::balance(asset_id, &ALICE);
            let bob_total_balance = Tokens::balance(asset_id, &BOB);
            let strategy_total_balance = Tokens::balance(asset_id, &strategy_account_id);

            prop_assert!(alice_total_balance == amount1 + strategy_profits);
            prop_assert!(alice_total_balance + bob_total_balance + strategy_total_balance
                         == amount1 + amount2 + strategy_profits);

            Ok(())
        });
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn vault_stock_dilution_k(
        (random_index, created_accounts) in
            valid_amounts_without_overflow_k_with_random_index(1000)
                .prop_filter("a minimum of two accounts are required, 1 for the strategy and 1 depositor",
                             |(_, x)| x.len() > 1)
    ) {
        let asset_id = MockCurrencyId::D;
        let (strategy_account_id, strategy_profits) = created_accounts[0];
        let strategy_deposit_moment = random_index;
        let accounts = &created_accounts[1..];
        let _ = ExtBuilder::default().build().execute_with(|| {
            let (vault_id, vault) = create_vault(strategy_account_id, asset_id);

            prop_assert!(Tokens::balance(asset_id, &strategy_account_id) == 0);
            assert_ok!(Tokens::mint_into(asset_id, &strategy_account_id, strategy_profits));

            for (account, balance) in accounts.iter().copied() {
                prop_assert!(Tokens::balance(asset_id, &account) == 0);
                assert_ok!(Tokens::mint_into(asset_id, &account, balance));
            }

            let strategy_profit_share =
                strategy_profits.checked_div(strategy_deposit_moment as Balance).expect(">= MINIMUM_BALANCE; qed;");

            for ((account, balance), index) in accounts.iter().copied().zip(0..accounts.len()) {
                if index == strategy_deposit_moment {
                    assert_ok!(<Vault as StrategicVault>::deposit(&vault_id, &strategy_account_id, strategy_profits));
                }
                assert_ok!(Vault::deposit(Origin::signed(account), vault_id, balance));
            }

            for ((account, balance), index) in accounts.iter().copied().zip(0..accounts.len()) {
                let lp = Tokens::balance(vault.lp_token_id, &account);
                assert_ok!(Vault::withdraw(Origin::signed(account), vault_id, lp));

                let new_balance = Tokens::balance(asset_id, &account);
                // We had shares before the profit, we get a cut of the profit
                if index < strategy_deposit_moment {
                    prop_assert!(new_balance == balance + strategy_profit_share);
                }
                else {
                    prop_assert!(new_balance == balance);
                }
            }

            let shareholders = &accounts[0..strategy_deposit_moment];
            let initial_sum_of_shareholders_balance = shareholders.iter()
                .map(|(_, initial_balance)| initial_balance)
                .sum::<Balance>();
            let current_sum_of_shareholders_balance = shareholders.iter()
                .map(|(account, _)| Tokens::balance(asset_id, &account))
                .sum::<Balance>();

            prop_assert!(current_sum_of_shareholders_balance
                         == initial_sum_of_shareholders_balance + strategy_profits);

            Ok(())
        });
    }
}
