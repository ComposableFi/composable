use crate::mocks::currency_factory::MockCurrencyId;
use crate::models::VaultInfo;
use crate::*;
use crate::{mocks::*, models::VaultConfig};
use composable_traits::vault::StrategicVault;
use frame_support::{assert_noop, assert_ok};
use orml_traits::MultiCurrency;
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

/* NOTE(hussein-aitlahcen)
   The 1+a is present to avoid the =0 predicate of the vault
*/
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10000))]

    #[test]
    fn vault_single_deposit_withdraw_asset_identity(
        amount in any::<ReasonableBalance>().prop_map(|a| 1 + a as Balance)
    ) {
        let asset_id = MockCurrencyId::A;
        let strategy_account_id = ACCOUNT_FREE_START + 0xCAFEBABE;
        let _ = ExtBuilder::default().build().execute_with(|| {
            let (vault_id, _) = create_vault(strategy_account_id, asset_id);

            prop_assert_eq!(Tokens::total_balance(asset_id, &ALICE), 0);
            assert_ok!(Tokens::deposit(asset_id, &ALICE, amount));

            prop_assert_eq!(Tokens::total_balance(asset_id, &ALICE), amount);

            assert_ok!(Vault::deposit(Origin::signed(ALICE), vault_id, amount));
            assert_ok!(Vault::withdraw(Origin::signed(ALICE), vault_id, amount));

            prop_assert_eq!(Tokens::total_balance(asset_id, &ALICE), amount);
            Ok(())
        });
    }

    #[test]
    fn vault_multi_deposit_withdraw_asset_identity(
        amount1 in any::<ReasonableBalance>().prop_map(|a| 1 + a as Balance),
        amount2 in any::<ReasonableBalance>().prop_map(|a| 1 + a as Balance),
        amount3 in any::<ReasonableBalance>().prop_map(|a| 1 + a as Balance)
    ) {
        let asset_id = MockCurrencyId::A;
        let strategy_account_id = ACCOUNT_FREE_START + 0xCAFEBABE;
        let _ = ExtBuilder::default().build().execute_with(|| {
            let (vault_id, _) = create_vault(strategy_account_id, asset_id);

            prop_assert_eq!(Tokens::total_balance(asset_id, &ALICE), 0);
            prop_assert_eq!(Tokens::total_balance(asset_id, &BOB), 0);
            prop_assert_eq!(Tokens::total_balance(asset_id, &CHARLIE), 0);
            assert_ok!(Tokens::deposit(asset_id, &ALICE, amount1));
            assert_ok!(Tokens::deposit(asset_id, &BOB, amount2));
            assert_ok!(Tokens::deposit(asset_id, &CHARLIE, amount3));

            prop_assert_eq!(Tokens::total_balance(asset_id, &BOB), amount2);
            prop_assert_eq!(Tokens::total_balance(asset_id, &ALICE), amount1);
            prop_assert_eq!(Tokens::total_balance(asset_id, &CHARLIE), amount3);

            assert_ok!(Vault::deposit(Origin::signed(CHARLIE), vault_id, amount3));
            assert_ok!(Vault::deposit(Origin::signed(BOB), vault_id, amount2));
            assert_ok!(Vault::deposit(Origin::signed(ALICE), vault_id, amount1));

            assert_ok!(Vault::withdraw(Origin::signed(ALICE), vault_id, amount1));
            assert_ok!(Vault::withdraw(Origin::signed(CHARLIE), vault_id, amount3));
            assert_ok!(Vault::withdraw(Origin::signed(BOB), vault_id, amount2));

            prop_assert_eq!(Tokens::total_balance(asset_id, &ALICE), amount1);
            prop_assert_eq!(Tokens::total_balance(asset_id, &BOB), amount2);
            prop_assert_eq!(Tokens::total_balance(asset_id, &CHARLIE), amount3);

            Ok(())
        });
    }

    #[test]
    fn vault_single_deposit_lp_ratio_asset_is_one(
        amount in any::<ReasonableBalance>().prop_map(|a| 1 + a as Balance))
    {
        let asset_id = MockCurrencyId::B;
        let strategy_account_id = ACCOUNT_FREE_START + 0xCAFEBABE;
        let _ = ExtBuilder::default().build().execute_with(|| {
            let (vault_id, vault_info) = create_vault(strategy_account_id, asset_id);
            prop_assert_eq!(Tokens::total_balance(asset_id, &ALICE), 0);
            assert_ok!(Tokens::deposit(asset_id, &ALICE, amount));

            prop_assert_eq!(Tokens::total_balance(vault_info.lp_token_id, &ALICE), 0);

            assert_ok!(Vault::deposit(Origin::signed(ALICE), vault_id, amount));

            prop_assert_eq!(Tokens::total_balance(vault_info.lp_token_id, &ALICE), amount);
            Ok(())
        });
    }

    #[test]
    fn vault_withdraw_with_zero_lp_issued_fails_to_burn(
        amount in any::<ReasonableBalance>().prop_map(|a| 1 + a as Balance)
    ) {
        let asset_id = MockCurrencyId::C;
        let strategy_account_id = ACCOUNT_FREE_START + 0xCAFEBABE;
        let _ = ExtBuilder::default().build().execute_with(|| {
            let (vault_id, _) = create_vault(strategy_account_id, asset_id);
            prop_assert_eq!(Tokens::total_balance(asset_id, &ALICE), 0);
            assert_ok!(Tokens::deposit(asset_id, &ALICE, amount));

            assert_noop!(Vault::withdraw(Origin::signed(ALICE), vault_id, amount), Error::<Test>::BurnFailed);
        });
    }

    #[test]
    fn vault_withdraw_without_depositing_fails_to_burn(
        amount in any::<ReasonableBalance>().prop_map(|a| 1 + a as Balance)
    ) {
        let asset_id = MockCurrencyId::D;
        let strategy_account_id = ACCOUNT_FREE_START + 0xCAFEBABE;
        let _ = ExtBuilder::default().build().execute_with(|| {
            let (vault_id, _) = create_vault(strategy_account_id, asset_id);
            prop_assert_eq!(Tokens::total_balance(asset_id, &ALICE), 0);
            prop_assert_eq!(Tokens::total_balance(asset_id, &BOB), 0);
            assert_ok!(Tokens::deposit(asset_id, &ALICE, amount));
            assert_ok!(Tokens::deposit(asset_id, &BOB, amount));

            assert_ok!(Vault::deposit(Origin::signed(ALICE), vault_id, amount));
            assert_noop!(Vault::withdraw(Origin::signed(BOB), vault_id, amount), Error::<Test>::BurnFailed);
        });
    }
}
