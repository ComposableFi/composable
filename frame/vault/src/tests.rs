use crate::{
	mocks::{
		currency_factory::MockCurrencyId,
		tests::{
			AccountId, Balance, Balances, BlockNumber, CreationDeposit, Event, ExistentialDeposit,
			ExtBuilder, Origin, System, Test, Tokens, TombstoneDuration, Vaults,
			ACCOUNT_FREE_START, ALICE, BOB, CHARLIE, MINIMUM_BALANCE,
		},
	},
	models::VaultInfo,
	*,
};
use composable_support::{math::safe::safe_multiply_by_rational, validation::Validated};
use composable_tests_helpers::{
	prop_assert_acceptable_computation_error, prop_assert_ok,
	test::helper::default_acceptable_computation_error,
};
use composable_traits::{
	defi::Rate,
	vault::{
		Deposit, FundsAvailability, ReportableStrategicVault, StrategicVault, Vault, VaultConfig,
	},
};
use frame_support::{
	assert_noop, assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use proptest::prelude::*;
use sp_runtime::{ArithmeticError, FixedPointNumber, Perbill, Perquintill};

const DEFAULT_STRATEGY_SHARE: Perquintill = Perquintill::from_percent(90);
// dependent on the previous value, both should be changed
const DEFAULT_RESERVE: Perquintill = Perquintill::from_percent(10);

fn create_vault_with_share(
	asset_id: MockCurrencyId,
	strategy_account_id: AccountId,
	strategy_share: Perquintill,
	reserved: Perquintill,
) -> (u64, VaultInfo<AccountId, Balance, MockCurrencyId, BlockNumber>) {
	let config = VaultConfig {
		asset_id,
		manager: ALICE,
		reserved,
		strategies: [(strategy_account_id, strategy_share)].iter().cloned().collect(),
	};

	let v = Vaults::do_create_vault(Deposit::Existential, Validated::new(config).unwrap());

	assert_ok!(&v);
	v.expect("unreachable; qed;")
}

fn create_vault_with_deposit(asset_id: MockCurrencyId, deposit: Balance) -> u64 {
	let v = Vaults::create(
		Origin::signed(ALICE),
		VaultConfig {
			asset_id,
			manager: ALICE,
			reserved: DEFAULT_RESERVE,
			strategies: [(ALICE, DEFAULT_STRATEGY_SHARE)].iter().cloned().collect(),
		},
		deposit,
	);
	assert_ok!(&v);
	v.expect("unreachable; qed;");
	1
}

fn create_vault(
	strategy_account_id: AccountId,
	asset_id: MockCurrencyId,
) -> (u64, VaultInfo<AccountId, Balance, MockCurrencyId, BlockNumber>) {
	create_vault_with_share(asset_id, strategy_account_id, DEFAULT_STRATEGY_SHARE, DEFAULT_RESERVE)
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

	/*
		Create an empty vault with 80% allocated for a single strategy, 20% as reserve.
		A single user and a single strategy are interacting.
		The user deposit an arbitrary amount.
		The strategy withdraw it's full allocation, a.k.a. 80% of the user deposit.
		The strategy deposit everything back, a.k.a. 80% of the user deposit.
		The vault funds is at it's initial state.

		V is vaults, S is strategies, U is users

		```math
		∀v ∈ V, ∀s ∈ S, ∀u ∈ U, ∀a ∈ ℕ, alloc x = 0.8 * funds x

		let v₁ = user_deposit u a v
			alloc_v₁ = alloc v₁
		in (strategy_deposit s alloc_v₁ ∘ strategy_withdraw s alloc_v₁) v₁ = identity v₁
		```
	*/
	#[test]
	fn liquidate_strategy_successfully_liquidates_a_strategy_account(
		strategy_account_id in strategy_account(),
		total_funds in valid_amounts_without_overflow_1()
	) {
		do_liquidate_strategy_successfully_liquidates_a_strategy_account(
			strategy_account_id,
			total_funds
		)
	}

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

			let expected_strategy_funds = strategy_share.mul_floor(total_funds);

			let available_funds = <Vaults as StrategicVault>::available_funds(&vault_id, &strategy_account_id);
			prop_assert!(
				matches!(
					available_funds,
					Ok(FundsAvailability::Withdrawable(strategy_funds))
						if expected_strategy_funds == strategy_funds
				),
				"Reserve should be 20% of initial strategy funds, expected: {}, actual: {:?}",
				expected_strategy_funds,
				available_funds
			);

			//  deposit . withdraw
			prop_assert_eq!(Tokens::balance(asset_id, &strategy_account_id), 0);
			prop_assert_ok!(<Vaults as StrategicVault>::withdraw(&vault_id, &strategy_account_id, expected_strategy_funds));
			prop_assert_eq!(Tokens::balance(asset_id, &strategy_account_id), expected_strategy_funds);
			prop_assert_ok!(<Vaults as StrategicVault>::deposit(&vault_id, &strategy_account_id, expected_strategy_funds));
			prop_assert_eq!(Tokens::balance(asset_id, &strategy_account_id), 0);

			// check that vault is back to its initial state
			let available_funds = <Vaults as StrategicVault>::available_funds(&vault_id, &strategy_account_id);
			prop_assert!(
				matches!(
					available_funds,
					Ok(FundsAvailability::Withdrawable(strategy_funds))
						if expected_strategy_funds == strategy_funds
				),
				"Reserve should be 20% of initial strategy funds, expected: {}, actual: {:?}",
				expected_strategy_funds,
				available_funds
			);

			Ok(())
		})?;
	}

	/*
		Create an empty vault with 80% allocated for a single strategy, 20% as reserve.
		A single user and a single strategy are interacting.
		The user deposit an arbitrary amount.
		The strategy withdraw it's full allocation, a.k.a. 80% of the user deposit.
		The user withdraw the whole reserve, a.k.a. 20% of it's initial deposit.
		The vault is unbalanced.
		The strategy questions the vault for the funds available, the vault answer that a 20% deposit is required.
		The vault is balanced.

		V is vaults, S is strategies, U is users

		```math
		∀v ∈ V, ∀s ∈ S, ∀u ∈ U, ∀a ∈ ℕ, alloc x = 0.8 * funds x, reserve x = 0.2 * funds x

		let v₁ = user_deposit u a v
			v₂ = strategy_withdraw s (alloc v₁) v₁
			v₃ = user_withdraw u (reserve v₂) v₂
			v₄ = strategy_deposit (reserve v₃) v₃
		in unbalanced v₃ and balanced v₄
		```
	 */
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

			let expected_strategy_funds = strategy_share.mul_floor(total_funds);

			let available_funds = <Vaults as StrategicVault>::available_funds(&vault_id, &strategy_account_id);
			prop_assert!(
				matches!(
					available_funds,
					Ok(FundsAvailability::Withdrawable(strategy_funds))
						if strategy_funds == expected_strategy_funds
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
						if default_acceptable_computation_error(new_expected_reserve, new_reserve).is_ok()
				),
				"Reserve should now be 20% of 80% of total funds = 20% of initial strategy funds, expected: {}, actual: {:?}",
				new_expected_reserve,
				new_available_funds
			);

			Ok(())
		})?;
	}

	/*
		Create an empty vault.
		A single user is interacting.
		The user deposit an arbitrary amount.
		The user withdraw everything it can.
		The vault funds is at it's initial state.

		V is vaults, U is users
		```math
		∀v ∈ V, ∀u ∈ U, ∀a ∈ ℕ

		(user_withdraw u a ∘ user_deposit u a) v = identity v
		```
	 */
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

	/// Similar to the previous single user version, but with three distinct users.
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

	/*
		Create an empty vault.
		A single user is interacting.
		The user deposit an arbitrary amount.
		The vault mint a 1:1 amount of lp tokens.

		V is vaults, U is users
		```math
		∀v ∈ V, ∀u ∈ U, ∀a ∈ ℕ

		let v₁ = user_deposit u a v
		in issued (lp_id v) v₁ = balance (lp_id v) u = a
		```
	 */
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

	/*
		Impossible to withdraw without holding lp tokens.
	 */
	#[test]
	fn vault_withdraw_with_zero_lp_issued_fails_to_burn(
		strategy_account_id in strategy_account(),
		amount in valid_amounts_without_overflow_1()
	) {
		let asset_id = MockCurrencyId::C;
		ExtBuilder::default().build().execute_with(|| {
			let (vault_id, vault) = create_vault(strategy_account_id, asset_id);
			prop_assert_eq!(Tokens::balance(vault.lp_token_id, &ALICE), 0);
			assert_noop!(Vaults::withdraw(Origin::signed(ALICE), vault_id, amount), ArithmeticError::Overflow);
			Ok(())
		})?;
	}

	/// Impossible to withdraw without holding lp tokens. Two users version.
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

	/*
		Create an empty vault.
		Two distinct users A and B and a single strategy are interacting.
		The user A deposit an arbitrary amount X.
		The strategy deposit an arbitrary profit, this profit should belong to previous shareholders.
		The user B deposit an arbitrary amount Y.
		The user A withdraw everything it can, which should be X + profit.
		The user B withdraw everything it can, which should be Y.

		V is vaults, U is users, S is strategies

		```math
		∀v ∈ V, ∀s ∈ S, ∀u₁ ∈ U, ∀u₂ ∈ U, u₁ != u₂, ∀a₁ ∈ ℕ, ∀a₂ ∈ ℕ, ∀p ∈ ℕ

		let v₁ = user_deposit u₁ a₁ v
			v₂ = strategy_deposit s p v₁
			v₃ = user_deposit u₂ a₂ v₂
			v₄ = user_withdraw u₁ (balance (lp_id v) u₁) v₃
			v₅ = user_withdraw u₂ (balance (lp_id v) u₂) v₄
		in balance (asset_id v) u₁  = a₁ + p and balance (asset_id v) u₂ = a₂
		```
	 */
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

			prop_assert_acceptable_computation_error!(alice_total_balance, amount1 + strategy_profits);

			prop_assert_acceptable_computation_error!(
				alice_total_balance + bob_total_balance + strategy_total_balance,
				amount1 + amount2 + strategy_profits
			);

			Ok(())
		})?;
	}

	/// Make sure that two distinct vault have their account isolated.
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

	/// Make sure that two distinct vault have their account isolated.
	#[test]
	fn vault_stock_dilution_rate(
		strategy_account_id in strategy_account(),
		(amount1, amount2, profits) in valid_amounts_without_overflow_3()
	) {
		ExtBuilder::default().build().execute_with(|| {
			let asset_id = MockCurrencyId::A;
			let (vault_id, VaultInfo { lp_token_id, .. }) = create_vault(strategy_account_id, asset_id);

			prop_assert_eq!(Tokens::balance(asset_id, &ALICE), 0);
			prop_assert_ok!(Tokens::mint_into(asset_id, &ALICE, amount1));
			prop_assert_ok!(Vaults::deposit(Origin::signed(ALICE), vault_id, amount1));

			// Rate unchanged
			prop_assert_eq!(Vaults::stock_dilution_rate(&vault_id), Ok(Rate::from(1)));

			prop_assert_eq!(Tokens::balance(asset_id, &BOB), 0);
			prop_assert_ok!(Tokens::mint_into(asset_id, &BOB, amount2));
			prop_assert_ok!(Vaults::deposit(Origin::signed(BOB), vault_id, amount2));

			// Rate unchanged
			prop_assert_eq!(Vaults::stock_dilution_rate(&vault_id), Ok(Rate::from(1)));

			let total_funds = amount1 + amount2;
			let expected_strategy_funds =
				DEFAULT_STRATEGY_SHARE.mul_floor(total_funds);
			let available_funds = <Vaults as StrategicVault>::available_funds(&vault_id, &strategy_account_id);
			prop_assert!(
				matches!(
					available_funds,
					Ok(FundsAvailability::Withdrawable(strategy_funds))
						if strategy_funds == expected_strategy_funds
				),
				"Strategy funds should be 90%, expected: {}, actual: {:?}",
				expected_strategy_funds,
				available_funds
			);

			// Strategy withdraw full allocation
			prop_assert_eq!(Tokens::balance(asset_id, &strategy_account_id), 0);
			prop_assert_ok!(<Vaults as StrategicVault>::withdraw(&vault_id, &strategy_account_id, expected_strategy_funds));
			prop_assert_eq!(Tokens::balance(asset_id, &strategy_account_id), expected_strategy_funds);

			// Rate unchanged
			prop_assert_eq!(Vaults::stock_dilution_rate(&vault_id), Ok(Rate::from(1)));

			let current_strategy_balance = expected_strategy_funds + profits;
			prop_assert_ok!(<Vaults as ReportableStrategicVault>::update_strategy_report(
				&vault_id,
				&strategy_account_id,
				&current_strategy_balance
			));

			let total_vault_balance = Tokens::balance(asset_id, &Vaults::account_id(&vault_id));
			let total_vault_value = total_vault_balance + current_strategy_balance;
			let total_lp_issued = Tokens::total_issuance(lp_token_id);

			let expected_dilution_rate =
				Rate::saturating_from_rational(total_vault_value, total_lp_issued);

			// Rate moved
			prop_assert_eq!(Vaults::stock_dilution_rate(&vault_id), Ok(expected_dilution_rate));

			Ok(())
		})?;
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(100))]

	// The strategy withdraw is calculated multiplying the deposited value by
	// (accounts.len() % 100) and then the strategy balance representing a loss is calculated
	// using the same percentage meaning that `deposit` > `withdraw` > `strategy balance`.
	//
	// `strategy_moment` is when the strategy generates profits or losses. Before or after that
	// Liquidity Providers deposit native tokens into the vault.
	#[test]
	fn vault_stock_dilution_k(
		(strategy_moment, accounts) in
			valid_amounts_without_overflow_k_with_random_index(500, 1_000_000_000)
				.prop_filter("a minimum of two accounts are required, 1 for the strategy and 1 depositor",
							 |(_, x)| x.len() > 2)
	) {
		let lps_start = 1;
		let asset_id = MockCurrencyId::D;

		let (strategy_account_id, strategy_deposit) = accounts[0];
		let strategy_withdraw_pbl = Perbill::from_percent(accounts.len() as u32 % 100);
		let strategy_withdraw = strategy_withdraw_pbl.mul_floor(strategy_deposit);
		let strategy_vault_balance = strategy_withdraw - strategy_withdraw_pbl.mul_floor(strategy_withdraw);
		let strategy_withdraw_diff = strategy_withdraw - strategy_vault_balance;
		let lps = &accounts[lps_start..];

		//let strategy_diff = strategy_profits - strategy_losses;
		let before_moment_lps = || lps.iter().take(strategy_moment).copied();
		let after_moment_lps = || lps.iter().skip(strategy_moment).copied();

		ExtBuilder::default().build().execute_with(|| {
			let (vault_id, vault) = create_vault(strategy_account_id, asset_id);

			// Mints native tokens for all accounts
			for (account, initial_native_tokens) in accounts.iter().copied() {
				prop_assert_eq!(Tokens::balance(asset_id, &account), 0);
				prop_assert_ok!(Tokens::mint_into(asset_id, &account, initial_native_tokens));
			}

			// Liquidity providers deposit all their native tokens to receive LP tokens
			// BEFORE losses and profits
			for (account, initial_native_tokens) in before_moment_lps() {
				let origin = Origin::signed(account);
				prop_assert_ok!(Vaults::deposit(origin, vault_id, initial_native_tokens));
			}

			prop_assert_ok!(<Vaults as StrategicVault>::deposit(
				&vault_id,
				&strategy_account_id,
				strategy_deposit
			));
			prop_assert_ok!(<Vaults as StrategicVault>::withdraw(
				&vault_id,
				&strategy_account_id,
				strategy_withdraw
			));
			prop_assert_ok!(<Vaults as ReportableStrategicVault>::update_strategy_report(
				&vault_id,
				&strategy_account_id,
				&strategy_vault_balance
			));

			let lp_tokens_total = Tokens::total_issuance(vault.lp_token_id);

			// Liquidity providers deposit all their native tokens to receive LP tokens
			// AFTER losses and profits
			for (account, initial_native_tokens) in after_moment_lps() {
				let origin = Origin::signed(account);
				prop_assert_ok!(Vaults::deposit(origin, vault_id, initial_native_tokens));
			}

			for (idx, (account, initial_native_tokens)) in lps.iter().copied().enumerate() {
				//  Contains half of LP balances minus LP profits
				let half_initial_native_tokens = initial_native_tokens / 2;

				let lp_tokens = Tokens::balance(vault.lp_token_id, &account);
				// Because of `<Vaults as StrategicVault>::withdraw`, the vault does not own 100% of
				// the funds. Therefore, a full withdraw is not possible.
				let withdrawn_lp_tokens = lp_tokens / 2;

				// Withdraws all LP tokens
				prop_assert_ok!(Vaults::withdraw(Origin::signed(account), vault_id, withdrawn_lp_tokens));

				// New balance that includes losses and profits
				let new_native_tokens = Tokens::balance(asset_id, &account);

				let curr_lp_deposited_before_moment = lps_start + idx <= strategy_moment;

				if curr_lp_deposited_before_moment {
					let strategy_native_tokens_deposit = safe_multiply_by_rational(
						strategy_deposit / 2,
						lp_tokens,
						lp_tokens_total,
					)
					.expect("qed;");
					let strategy_native_tokens_withdraw = safe_multiply_by_rational(
						strategy_withdraw_diff / 2,
						lp_tokens,
						lp_tokens_total,
					)
					.expect("qed;");

					let diff = strategy_native_tokens_deposit - strategy_native_tokens_withdraw;

					prop_assert_acceptable_computation_error!(new_native_tokens, half_initial_native_tokens + diff);
				} else {
					// Our balance should be equivalent
		  let precision = 1000;
		  let epsilon = 5;
				  prop_assert_acceptable_computation_error!(new_native_tokens, half_initial_native_tokens, precision, epsilon);
				}
			}

			// Global check
			let initial_sum_of_native_tokens = before_moment_lps()
				.map(|(_, initial_native_tokens)| initial_native_tokens)
				.sum::<Balance>();
			let current_sum_of_native_tokens = before_moment_lps()
				.map(|(account, _)| Tokens::balance(asset_id, &account))
				.sum::<Balance>();

			prop_assert_acceptable_computation_error!(
					current_sum_of_native_tokens,
					initial_sum_of_native_tokens / 2 + strategy_deposit / 2 - strategy_withdraw_diff / 2
			);

			Ok(())
		})?;
	}
}

use frame_support::traits::fungible::{Inspect as _, Mutate as _};

#[test]
fn test_vault_claim_surcharge_existential() {
	ExtBuilder::default().build().execute_with(|| {
		Balances::mint_into(&ALICE, ExistentialDeposit::get() * 3).unwrap();
		let id = create_vault_with_deposit(MockCurrencyId::A, ExistentialDeposit::get());
		System::set_block_number(10000000000);
		Vaults::claim_surcharge(Origin::none(), id, Some(ALICE))
			.expect("claiming surcharge for existential should always ok");
	})
}

#[test]
fn test_vault_claim_surcharge_rent_exempt() {
	ExtBuilder::default().build().execute_with(|| {
		Balances::mint_into(&ALICE, ExistentialDeposit::get() * 3).unwrap();
		assert_eq!(Balances::balance(&CHARLIE), 0);
		System::set_block_number(1);
		let id = create_vault_with_deposit(MockCurrencyId::A, ExistentialDeposit::get() / 2);
		System::set_block_number(1);
		Vaults::claim_surcharge(Origin::none(), id, Some(CHARLIE))
			.expect("claiming surcharge for rent should work");
		assert!(Balances::balance(&CHARLIE) == 0);
		let vault = Vaults::vault_data(id).unwrap();
		assert!(!vault.capabilities.is_tombstoned());
	})
}

#[test]
fn test_vault_claim_surcharge_rent_charge() {
	ExtBuilder::default().build().execute_with(|| {
		Balances::mint_into(&ALICE, ExistentialDeposit::get() * 3).unwrap();
		assert_eq!(Balances::balance(&CHARLIE), 0);
		System::set_block_number(1);
		let id = create_vault_with_deposit(MockCurrencyId::A, ExistentialDeposit::get() / 2);
		let duration = 20;
		System::set_block_number(duration);
		Vaults::claim_surcharge(Origin::none(), id, Some(CHARLIE))
			.expect("claiming surcharge for rent should work");
		assert!(Balances::balance(&CHARLIE) > 0);
		let vault = Vaults::vault_data(id).unwrap();
		assert!(!vault.capabilities.is_tombstoned());
	})
}

#[test]
fn test_vault_claim_surcharge_rent_evict() {
	ExtBuilder::default().build().execute_with(|| {
		Balances::mint_into(&ALICE, ExistentialDeposit::get() * 3).unwrap();
		assert_eq!(Balances::balance(&CHARLIE), 0);
		System::set_block_number(1);
		let id = create_vault_with_deposit(MockCurrencyId::A, ExistentialDeposit::get() / 2);
		let duration = 100000;
		System::set_block_number(duration);
		Vaults::claim_surcharge(Origin::none(), id, Some(CHARLIE))
			.expect("claiming surcharge for rent should work");
		assert!(Balances::balance(&CHARLIE) > 0);
		let vault = Vaults::vault_data(id).unwrap();
		assert!(vault.capabilities.is_tombstoned());
	})
}

#[test]
fn test_vault_add_surcharge() {
	ExtBuilder::default().build().execute_with(|| {
		Balances::mint_into(&ALICE, ExistentialDeposit::get() * 3).unwrap();
		assert_eq!(Balances::balance(&CHARLIE), 0);
		System::set_block_number(1);
		let id = create_vault_with_deposit(MockCurrencyId::A, ExistentialDeposit::get() / 2);
		let duration = 100000;
		System::set_block_number(duration);
		Vaults::claim_surcharge(Origin::none(), id, Some(CHARLIE))
			.expect("claiming surcharge for rent should work");
		assert!(Balances::balance(&CHARLIE) > 0);
		let vault = Vaults::vault_data(id).unwrap();
		assert!(vault.capabilities.is_tombstoned());
		Vaults::add_surcharge(
			Origin::signed(ALICE),
			id,
			Validated::new(CreationDeposit::get()).unwrap(),
		)
		.unwrap();
		let vault = Vaults::vault_data(id).unwrap();
		assert!(!vault.capabilities.is_tombstoned());
	})
}

#[test]
fn test_vault_delete_tombstoned() {
	ExtBuilder::default().build().execute_with(|| {
		Balances::mint_into(&ALICE, ExistentialDeposit::get() * 3).unwrap();
		assert_eq!(Balances::balance(&CHARLIE), 0);
		System::set_block_number(0);
		let id = create_vault_with_deposit(MockCurrencyId::A, ExistentialDeposit::get() - 1);
		System::set_block_number(1000000);
		Vaults::claim_surcharge(Origin::none(), id, Some(CHARLIE))
			.expect("claiming surcharge for rent should work");
		let after_surcharge_balance = Balances::balance(&CHARLIE);
		assert!(after_surcharge_balance > 0);
		let vault = Vaults::vault_data(id).unwrap();
		assert!(vault.capabilities.is_tombstoned());
		System::set_block_number(1000000 + TombstoneDuration::get());
		Vaults::delete_tombstoned(Origin::signed(CHARLIE), id, None).unwrap();
		let after_delete_balance = Balances::balance(&CHARLIE);
		assert!(after_delete_balance > after_surcharge_balance);
		// second time should error, as the vault is not deleted.
		Vaults::delete_tombstoned(Origin::signed(CHARLIE), id, None).unwrap_err();
	})
}

#[test]
fn test_vault_delete_tombstoned_insufficient_time_fails() {
	ExtBuilder::default().build().execute_with(|| {
		Balances::mint_into(&ALICE, ExistentialDeposit::get() * 3).unwrap();
		assert_eq!(Balances::balance(&CHARLIE), 0);
		System::set_block_number(0);
		let id = create_vault_with_deposit(MockCurrencyId::A, ExistentialDeposit::get() - 1);
		System::set_block_number(1000000);
		Vaults::claim_surcharge(Origin::none(), id, Some(CHARLIE))
			.expect("claiming surcharge for rent should work");
		assert!(Balances::balance(&CHARLIE) > 0);
		let vault = Vaults::vault_data(id).unwrap();
		assert!(vault.capabilities.is_tombstoned());
		System::set_block_number(1000000 + TombstoneDuration::get() - 1);
		Vaults::delete_tombstoned(Origin::signed(ALICE), id, None).unwrap_err();
	})
}

#[test]
fn test_vault_delete_tombstoned_non_tombstoned_fails() {
	ExtBuilder::default().build().execute_with(|| {
		Balances::mint_into(&ALICE, ExistentialDeposit::get() - 1).unwrap();
		assert_eq!(Balances::balance(&CHARLIE), 0);
		let id = create_vault_with_deposit(MockCurrencyId::A, CreationDeposit::get());
		Vaults::delete_tombstoned(Origin::signed(ALICE), id, None).unwrap_err();
	})
}

#[test]
fn test_vault_emergency_shutdown_origin() {
	ExtBuilder::default().build().execute_with(|| {
		let (id, _) = create_vault(ALICE, MockCurrencyId::A);
		Vaults::emergency_shutdown(Origin::signed(ALICE), id)
			.expect_err("only root may emergency_shutdown");
		Vaults::emergency_shutdown(Origin::none(), id)
			.expect_err("only root may emergency_shutdown");
	})
}

#[test]
fn test_vault_emergency_shutdown() {
	ExtBuilder::default().build().execute_with(|| {
		let (id, _) = create_vault(ALICE, MockCurrencyId::A);
		// Setting up the vault and depositing to ensure it is working correctly, and later to
		// ensure that the specific deposit cannot be withdrawn if the vault is stopped.
		Tokens::mint_into(MockCurrencyId::A, &ALICE, 1000)
			.expect("minting for ALICE should succeed");
		Vaults::deposit(Origin::signed(ALICE), id, 100)
			.expect("depositing in active vault should succeed");

		// Shutdown the vault, and ensure that the deposited funds cannot be withdrawn.
		Vaults::emergency_shutdown(Origin::root(), id)
			.expect("root should be able to emergency shutdown");
		Vaults::deposit(Origin::signed(ALICE), id, 100)
			.expect_err("depositing in stopped vault should fail");
		Vaults::withdraw(Origin::signed(ALICE), id, 100)
			.expect_err("withdrawing from stopped vault should fail");

		// Restart the vault, and ensure that funds can be withdrawn and deposited
		Vaults::start(Origin::root(), id).expect("root can restart the vault");
		Vaults::deposit(Origin::signed(ALICE), id, 100)
			.expect("depositing in restarted vault should succeed");
		Vaults::withdraw(Origin::signed(ALICE), id, 100)
			.expect("withdrawing from restarted vault should succeed");
	});
}

#[test]
fn liquidate_strategy_can_not_be_executed_by_non_manager_accounts() {
	ExtBuilder::default().build().execute_with(|| {
		let (id, _) = create_vault(ALICE, MockCurrencyId::A);
		assert_noop!(
			Vaults::liquidate_strategy(Origin::signed(BOB), id, 100),
			Error::<Test>::AccountIsNotManager
		);
	});
}

fn do_liquidate_strategy_successfully_liquidates_a_strategy_account(
	strategy_account_id: AccountId,
	total_funds: Balance,
) {
	let currency_id = MockCurrencyId::A;
	let strategy_share = Perquintill::from_percent(20);

	let strategy_vault = strategy_share.mul_floor(total_funds);

	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		Tokens::mint_into(currency_id, &ALICE, total_funds).unwrap();

		let (id, _) = create_vault(strategy_account_id, currency_id);

		Vaults::deposit(Origin::signed(ALICE), id, total_funds).unwrap();
		assert_eq!(Tokens::balance(currency_id, &strategy_account_id), 0);

		<Vaults as StrategicVault>::withdraw(&id, &strategy_account_id, strategy_vault).unwrap();
		assert!(CapitalStructure::<Test>::try_get(id, strategy_account_id).is_ok());
		assert_eq!(Tokens::balance(currency_id, &strategy_account_id), strategy_vault);

		Vaults::liquidate_strategy(Origin::signed(ALICE), id, strategy_account_id).unwrap();
		assert!(CapitalStructure::<Test>::try_get(id, strategy_account_id).is_err());
		assert_eq!(
			<Vaults as StrategicVault>::available_funds(&id, &strategy_account_id),
			Ok(FundsAvailability::MustLiquidate)
		);
		System::assert_has_event(Event::Vaults(crate::Event::LiquidateStrategy {
			account: strategy_account_id,
			amount: strategy_vault,
		}));

		<Vaults as StrategicVault>::deposit(&id, &strategy_account_id, strategy_vault).unwrap();
		assert_eq!(Tokens::balance(currency_id, &strategy_account_id), 0);
	});
}
