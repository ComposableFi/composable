use crate::{
	mocks::{
		new_test_ext, AccountId, Balance, Lending, MockCurrencyId, Oracle, Test, Tokens, Vault,
		VaultId, ALICE,
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
	traits::fungibles::{Inspect, Mutate},
};
use hex_literal::hex;
use proptest::prelude::*;
use sp_runtime::{traits::Zero, FixedPointNumber, Perquintill};
use sp_std::collections::btree_map::BTreeMap;

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
