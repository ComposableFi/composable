use frame_support::assert_ok;

use crate::mock::*;
use composable_traits::{
	dex::CurveAmm as CurveAmmTrait,
	vault::{Deposit, Vault as VaultTrait, VaultConfig},
};
use frame_support::traits::fungibles::{Inspect, Mutate};
use pallet_vault::models::VaultInfo;
use sp_runtime::{
	traits::{Saturating, Zero},
	FixedPointNumber, FixedU128, Permill, Perquintill,
};
use sp_std::cmp::Ordering;

#[test]
fn compute_d_works() {
	let xp = vec![
		FixedU128::saturating_from_rational(11u128, 10u128),
		FixedU128::saturating_from_rational(88u128, 100u128),
	];
	let amp = FixedU128::saturating_from_rational(292u128, 100u128);
	let ann = CurveAmm::get_ann(amp, xp.len()).unwrap();
	let d = CurveAmm::get_d(&xp, ann);
	// expected d is 1.978195735374521596
	// expected precision is 1e-13
	let delta = d
		.map(|x| {
			x.saturating_sub(FixedU128::saturating_from_rational(
				1978195735374521596u128,
				10_000_000_000_000_000u128,
			))
			.saturating_abs()
		})
		.map(|x| x.cmp(&FixedU128::saturating_from_rational(1u128, 10_000_000_000_000u128)));
	assert_eq!(delta, Some(Ordering::Less));
}

#[test]
fn compute_d_empty() {
	let xp = vec![];
	let amp = FixedU128::saturating_from_rational(292u128, 100u128);
	let ann = CurveAmm::get_ann(amp, xp.len()).unwrap();
	let result = CurveAmm::get_d(&xp, ann);
	assert_eq!(result, Some(FixedU128::zero()));
}

#[test]
fn get_y_successful() {
	let i = 0;
	let j = 1;
	let x = FixedU128::saturating_from_rational(111u128, 100u128);
	let xp = vec![
		FixedU128::saturating_from_rational(11u128, 10u128),
		FixedU128::saturating_from_rational(88u128, 100u128),
	];
	let amp = FixedU128::saturating_from_rational(292u128, 100u128);
	let ann = CurveAmm::get_ann(amp, xp.len()).unwrap();

	let result = CurveAmm::get_y(i, j, x, &xp, ann);
	// expected y is 1.247108067356516682
	// expected precision is 1e-13
	let delta = result
		.map(|x| {
			x.saturating_sub(FixedU128::saturating_from_rational(
				1247108067356516682u128,
				10_000_000_000_000_000u128,
			))
			.saturating_abs()
		})
		.map(|x| x.cmp(&FixedU128::saturating_from_rational(1u128, 10_000_000_000_000u128)));
	assert_eq!(delta, Some(Ordering::Less));
}

#[test]
fn get_y_same_coin() {
	let i = 1;
	let j = 1;
	let x = FixedU128::saturating_from_rational(111u128, 100u128);
	let xp = vec![
		FixedU128::saturating_from_rational(11u128, 10u128),
		FixedU128::saturating_from_rational(88u128, 100u128),
	];
	let amp = FixedU128::saturating_from_rational(292u128, 100u128);
	let ann = CurveAmm::get_ann(amp, xp.len()).unwrap();

	let result = CurveAmm::get_y(i, j, x, &xp, ann);

	assert_eq!(result, None);
}

#[test]
fn get_y_i_greater_than_n() {
	let i = 33;
	let j = 1;
	let x = FixedU128::saturating_from_rational(111u128, 100u128);
	let xp = vec![
		FixedU128::saturating_from_rational(11u128, 10u128),
		FixedU128::saturating_from_rational(88u128, 100u128),
	];
	let amp = FixedU128::saturating_from_rational(292u128, 100u128);
	let ann = CurveAmm::get_ann(amp, xp.len()).unwrap();

	let result = CurveAmm::get_y(i, j, x, &xp, ann);

	assert_eq!(result, None);
}

#[test]
fn get_y_j_greater_than_n() {
	let i = 1;
	let j = 33;
	let x = FixedU128::saturating_from_rational(111u128, 100u128);
	let xp = vec![
		FixedU128::saturating_from_rational(11u128, 10u128),
		FixedU128::saturating_from_rational(88u128, 100u128),
	];
	let amp = FixedU128::saturating_from_rational(292u128, 100u128);
	let ann = CurveAmm::get_ann(amp, xp.len()).unwrap();

	let result = CurveAmm::get_y(i, j, x, &xp, ann);

	assert_eq!(result, None);
}

/// Create a very simple vault for the given currency, 100% is reserved.
fn create_simple_vault(
	asset_id: MockCurrencyId,
) -> (VaultId, VaultInfo<AccountId, Balance, MockCurrencyId, BlockNumber>) {
	let v = Vault::do_create_vault(
		Deposit::Existential,
		VaultConfig {
			asset_id,
			manager: ALICE,
			reserved: Perquintill::from_percent(5),
			strategies: [(CURVE_STRATEGY_ACC_ID, Perquintill::from_percent(95))]
				.iter()
				.cloned()
				.collect(),
		},
	);
	assert_ok!(&v);
	v.expect("unreachable; qed;")
}

#[test]
fn add_remove_liquidity() {
	new_test_ext().execute_with(|| {
		let assets = vec![MockCurrencyId::BTC, MockCurrencyId::USDT];
		let btc_vault_res = create_simple_vault(MockCurrencyId::BTC);
		let usdt_vault_res = create_simple_vault(MockCurrencyId::USDT);
		let assets_vault_ids = vec![btc_vault_res.0, usdt_vault_res.0];
		let amp_coeff = 2u128;
		let fee = Permill::zero();

		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &ALICE, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 200000);
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &ALICE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::BTC, &ALICE, 20));
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &ALICE), 20);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &BOB, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 200000);
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &BOB), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::BTC, &BOB, 20));
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &BOB), 20);
		let p = CurveAmm::create_pool(&ALICE, assets_vault_ids, amp_coeff, fee);
		assert_ok!(&p);
		let pool_id = p.unwrap();
		let pool = CurveAmm::pool(pool_id);
		assert!(pool.is_some());
		// 1 BTC = 65000 USDT
		let amounts = vec![2u128, 130000u128];
		let alice_lp_tokens = CurveAmm::add_liquidity(&ALICE, pool_id, amounts.clone());
		assert!(alice_lp_tokens.is_ok());
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 200000 - 130000);
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &ALICE), 20 - 2);
		let pool = CurveAmm::pool(pool_id);
		assert!(pool.is_some());
		let bob_lp_tokens = CurveAmm::add_liquidity(&BOB, pool_id, amounts.clone());
		assert!(bob_lp_tokens.is_ok());
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 200000 - 130000);
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &BOB), 20 - 2);
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &Vault::account_id(&btc_vault_res.0)), 4);
		assert_eq!(
			Tokens::balance(MockCurrencyId::USDT, &Vault::account_id(&usdt_vault_res.0)),
			260000
		);
		assert_ok!(CurveAmm::remove_liquidity(&ALICE, pool_id, alice_lp_tokens.unwrap()));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 200000);
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &ALICE), 20);
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &Vault::account_id(&btc_vault_res.0)), 2);
		assert_eq!(
			Tokens::balance(MockCurrencyId::USDT, &Vault::account_id(&usdt_vault_res.0)),
			130000
		);
		assert_ok!(CurveAmm::remove_liquidity(&BOB, pool_id, bob_lp_tokens.unwrap()));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 200000);
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &BOB), 20);
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &Vault::account_id(&btc_vault_res.0)), 0);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &Vault::account_id(&usdt_vault_res.0)), 0);
	});
}

#[test]
fn exchange_test() {
	new_test_ext().execute_with(|| {
		let assets = vec![MockCurrencyId::BTC, MockCurrencyId::USDT];
		let amp_coeff = 2u128;
		let fee = Permill::zero();
		let btc_vault_res = create_simple_vault(MockCurrencyId::BTC);
		let usdt_vault_res = create_simple_vault(MockCurrencyId::USDT);
		let assets_vault_ids = vec![btc_vault_res.0, usdt_vault_res.0];

		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &ALICE, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 200000);
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &ALICE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::BTC, &ALICE, 20));
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &ALICE), 20);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &BOB, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 200000);
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &BOB), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::BTC, &BOB, 20));
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &BOB), 20);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &CHARLIE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &CHARLIE, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &CHARLIE), 200000);
		let p = CurveAmm::create_pool(&ALICE, assets_vault_ids, amp_coeff, fee);
		assert_ok!(&p);
		let pool_id = p.unwrap();
		let pool = CurveAmm::pool(pool_id);
		assert!(pool.is_some());
		// 1 BTC = 65000 USDT
		let amounts = vec![2u128, 130000u128];
		let alice_lp_tokens = CurveAmm::add_liquidity(&ALICE, pool_id, amounts.clone());
		assert_ok!(alice_lp_tokens);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 200000 - 130000);
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &ALICE), 20 - 2);
		let pool = CurveAmm::pool(pool_id);
		assert!(pool.is_some());
		let bob_lp_tokens = CurveAmm::add_liquidity(&BOB, pool_id, amounts.clone());
		assert_ok!(bob_lp_tokens);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 200000 - 130000);
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &BOB), 20 - 2);
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &CHARLIE), 0);
		assert_ok!(CurveAmm::exchange(&CHARLIE, pool_id, 1, 0, 65000, 0));
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &CHARLIE), 1);
	});
}
