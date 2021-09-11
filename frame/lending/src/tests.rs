use crate::{
	mocks::{
		new_test_ext, process_block, AccountId, Balance, Lending, MockCurrencyId, Origin, Tokens,
		Vault, VaultId, ALICE, BOB, CHARLIE,
	},
	MarketIndex,
};
use composable_traits::{
	lending::{MarketConfigInput, NormalizedCollateralFactor},
	rate_model::*,
	vault::{Deposit, VaultConfig},
};
use frame_support::{
	assert_ok,
	traits::{
		fungibles::{Inspect, Mutate},
		OnInitialize,
	},
};
use proptest::prelude::*;
use sp_runtime::{traits::Zero, FixedPointNumber, Perquintill};

type BorrowAssetVault = VaultId;

type CollateralAsset = MockCurrencyId;

fn create_market(
	borrow_asset: MockCurrencyId,
	collateral_asset: MockCurrencyId,
	manager: AccountId,
	reserved: Perquintill,
	collateral_factor: NormalizedCollateralFactor,
) -> (MarketIndex, BorrowAssetVault) {
	let market_config = MarketConfigInput { manager, reserved, collateral_factor };
	let market = Lending::create(borrow_asset, collateral_asset, market_config);
	assert_ok!(market);
	market.expect("unreachable; qed;")
}

fn create_simple_vaulted_market() -> ((MarketIndex, BorrowAssetVault), CollateralAsset) {
	let collateral_vault = Vault::do_create_vault(
		Deposit::Existential,
		VaultConfig {
			asset_id: MockCurrencyId::USDT,
			reserved: Perquintill::from_percent(100),
			manager: ALICE,
			strategies: [].iter().cloned().collect(),
		},
	);
	assert_ok!(collateral_vault);
	let (collateral_vault, collateral_vault_config) = collateral_vault.expect("unreachable; qed;");
	(
		create_market(
			MockCurrencyId::BTC,
			collateral_vault_config.lp_token_id,
			ALICE,
			Perquintill::from_percent(10),
			NormalizedCollateralFactor::saturating_from_rational(200, 100),
		),
		collateral_vault_config.lp_token_id,
	)
}

fn create_simple_market() -> (MarketIndex, BorrowAssetVault) {
	create_market(
		MockCurrencyId::BTC,
		MockCurrencyId::USDT,
		ALICE,
		Perquintill::from_percent(10),
		NormalizedCollateralFactor::saturating_from_rational(200, 100),
	)
}

#[test]
fn test_calc_utilization_ratio() {
	// 50% borrow
	assert_eq!(
		Lending::calc_utilization_ratio(&1, &1, &0).unwrap(),
		Ratio::saturating_from_rational(50, 100)
	);
	assert_eq!(
		Lending::calc_utilization_ratio(&100, &100, &0).unwrap(),
		Ratio::saturating_from_rational(50, 100)
	);
	// no borrow
	assert_eq!(Lending::calc_utilization_ratio(&1, &0, &0).unwrap(), Ratio::zero());
	// full borrow
	assert_eq!(
		Lending::calc_utilization_ratio(&0, &1, &0).unwrap(),
		Ratio::saturating_from_rational(100, 100)
	);
}

#[test]
fn test_borrow() {
	new_test_ext().execute_with(|| {
		let amount = 900000;
		let (market, vault) = create_simple_market();
		// Balance for ALICE
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &ALICE, amount));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), amount);

		assert_ok!(Lending::deposit_collateral(&market, &ALICE, amount));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 0);
		// Balance for BOB
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &BOB, amount));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), amount);

		// Balance of BTC for CHARLIE
		// CHARLIE is only lender of BTC
		let btc_amt = amount * 100;
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &CHARLIE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::BTC, &CHARLIE, btc_amt));
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &CHARLIE), btc_amt);
		Vault::deposit(Origin::signed(CHARLIE), vault, btc_amt);
		let mut total_cash = btc_amt;
		assert_ok!(Lending::deposit_collateral(&market, &BOB, amount));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 0);

		assert_eq!(Lending::borrow_balance_current(&market, &ALICE), Ok(Some(0)));
		let alice_limit = Lending::get_borrow_limit(&market, &ALICE).unwrap();
		assert_eq!(Lending::total_cash(&market), Ok(total_cash));
		assert_ok!(Lending::borrow(&market, &ALICE, alice_limit / 4));
		total_cash -= alice_limit / 4;
		let mut total_borrows = alice_limit / 4;
		assert_eq!(Lending::total_cash(&market), Ok(total_cash));
		assert_eq!(Lending::total_borrows(&market), Ok(total_borrows));
		let mut total_interest: u128 = 0;
		// Interest rate model, should be same as defined in InterestRateModel
		let base_rate = Rate::saturating_from_rational(2, 100);
		let jump_rate = Rate::saturating_from_rational(10, 100);
		let jump_utilization = Ratio::saturating_from_rational(80, 100);
		for i in 1..50 {
			process_block(i);
			// utilizationRatio = totalBorrows / (totalCash + totalBorrows)
			let util_ratio =
				Ratio::saturating_from_rational(total_borrows, total_cash + total_borrows);

			let delta_time = 6; // in seconds
			let borrow_rate =
				(jump_rate - base_rate) * util_ratio.into() / jump_utilization.into() + base_rate;
			let interest_accumulated: u128 = borrow_rate
				.saturating_mul_int(total_borrows)
				.saturating_mul(delta_time)
				.checked_div(SECONDS_PER_YEAR.into())
				.unwrap();
			//FIXME: interest_accumulated gets converted to 0 (FixedU128 -> u64)
			//       for value which is less than 1
			total_interest = interest_accumulated + total_interest;
			assert_eq!(Lending::total_interest(&market), Ok(total_interest));
		}
	});
}

macro_rules! prop_assert_ok {
    ($cond:expr) => {
        prop_assert_ok!($cond, concat!("assertion failed: ", stringify!($cond)))
    };

    ($cond:expr, $($fmt:tt)*) => {
        if let Err(e) = $cond {
            let message = format!($($fmt)*);
            let message = format!("{} unexpected {:?} at {}:{}", message, e, file!(), line!());
            return ::std::result::Result::Err(
                proptest::test_runner::TestCaseError::fail(message));
        }
    };
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(10000))]

	#[test]
	fn market_collateral_deposit_withdraw_identity(amount in 0..u32::MAX as Balance) {
		new_test_ext().execute_with(|| {
			let (market, vault) = create_simple_market();
			prop_assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 0);
			prop_assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &ALICE, amount));
			prop_assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), amount);

			prop_assert_ok!(Lending::deposit_collateral(&market, &ALICE, amount));
			prop_assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 0);
			prop_assert_ok!(Lending::withdraw_collateral(&market, &ALICE, amount));
			prop_assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), amount);

			Ok(())
		})?;
	}

	#[test]
	fn market_collateral_vaulted_deposit_withdraw_identity(amount in 0..u32::MAX as Balance) {
		new_test_ext().execute_with(|| {
			let ((market, borrow_vault), collateral_asset) = create_simple_vaulted_market();

			prop_assert_eq!(Tokens::balance(collateral_asset, &ALICE), 0);
			prop_assert_ok!(Tokens::mint_into(collateral_asset, &ALICE, amount));
			prop_assert_eq!(Tokens::balance(collateral_asset, &ALICE), amount);

			prop_assert_ok!(Lending::deposit_collateral(&market, &ALICE, amount));
			prop_assert_eq!(Tokens::balance(collateral_asset, &ALICE), 0);
			prop_assert_ok!(Lending::withdraw_collateral(&market, &ALICE, amount));
			prop_assert_eq!(Tokens::balance(collateral_asset, &ALICE), amount);

			Ok(())
		})?;
	}
}
