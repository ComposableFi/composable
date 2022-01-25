use frame_support::assert_ok;

use crate::mock::*;
use composable_traits::dex::CurveAmm as CurveAmmTrait;
use frame_support::traits::fungibles::{Inspect, Mutate};
use proptest::prelude::*;
use sp_runtime::{
	helpers_128bit::multiply_by_rational,
	traits::{Saturating, Zero},
	FixedPointNumber, FixedU128, Permill,
};
use sp_std::cmp::Ordering;

/// Accepts -2, -1, 0, 1, 2
macro_rules! prop_assert_zero_epsilon {
	($x:expr) => {{
		let epsilon = 2;
		let upper = 0 + epsilon;
		let lower = 0;
		prop_assert!(upper >= $x && $x >= lower, "{} => {} >= {}", upper, $x, lower);
	}};
}

/// Accept a 'dust' deviation
macro_rules! prop_assert_epsilon {
	($x:expr, $y:expr) => {{
		let precision = 1000;
		let epsilon = 5;
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

fn create_pool(
	assets: Vec<AssetId>,
	amounts: Vec<Balance>,
	amp_coeff: FixedU128,
	fee: Permill,
	admin_fee: Permill,
) -> PoolId {
	assert_ok!(Tokens::mint_into(assets[0], &ALICE, amounts[0]));
	assert_ok!(Tokens::mint_into(assets[1], &ALICE, amounts[1]));
	assert_ok!(Tokens::mint_into(assets[0], &BOB, amounts[0]));
	assert_ok!(Tokens::mint_into(assets[1], &BOB, amounts[1]));
	let p = CurveAmm::create_pool(&ALICE, assets, amp_coeff, fee, admin_fee);
	assert_ok!(&p);
	let pool_id = p.unwrap();
	assert_ok!(CurveAmm::add_liquidity(&ALICE, pool_id, amounts.clone(), 0_u128));
	assert_ok!(CurveAmm::add_liquidity(&BOB, pool_id, amounts.clone(), 0_u128));
	pool_id
}

#[test]
fn compute_d_works() {
	let xp = vec![
		FixedU128::saturating_from_rational(11_u128, 10_u128),
		FixedU128::saturating_from_rational(88_u128, 100_u128),
	];
	let amp = FixedU128::saturating_from_rational(292_u128, 100_u128);
	let ann = CurveAmm::get_ann(amp, xp.len()).unwrap();
	let d = CurveAmm::get_d(&xp, ann);
	// expected d is 1.978195735374521596
	// expected precision is 1e-13
	let delta = d
		.map(|x| {
			x.saturating_sub(FixedU128::saturating_from_rational(
				1978195735374521596_u128,
				10_000_000_000_000_000_u128,
			))
			.saturating_abs()
		})
		.map(|x| x.cmp(&FixedU128::saturating_from_rational(1_u128, 10_000_000_000_000_u128)));
	assert_eq!(delta, Some(Ordering::Less));
}

#[test]
fn compute_d_empty() {
	let xp = vec![];
	let amp = FixedU128::saturating_from_rational(292_u128, 100_u128);
	let ann = CurveAmm::get_ann(amp, xp.len()).unwrap();
	let result = CurveAmm::get_d(&xp, ann);
	assert_eq!(result, Some(FixedU128::zero()));
}

#[test]
fn get_y_successful() {
	let i = 0;
	let j = 1;
	let x = FixedU128::saturating_from_rational(111_u128, 100_u128);
	let xp = vec![
		FixedU128::saturating_from_rational(11_u128, 10_u128),
		FixedU128::saturating_from_rational(88_u128, 100_u128),
	];
	let amp = FixedU128::saturating_from_rational(292_u128, 100_u128);
	let ann = CurveAmm::get_ann(amp, xp.len()).unwrap();

	let result = CurveAmm::get_y(i, j, x, &xp, ann);
	// expected y is 1.247108067356516682
	// expected precision is 1e-13
	let delta = result
		.map(|x| {
			x.saturating_sub(FixedU128::saturating_from_rational(
				1247108067356516682_u128,
				10_000_000_000_000_000_u128,
			))
			.saturating_abs()
		})
		.map(|x| x.cmp(&FixedU128::saturating_from_rational(1_u128, 10_000_000_000_000_u128)));
	assert_eq!(delta, Some(Ordering::Less));
}

#[test]
fn get_y_same_coin() {
	let i = 1;
	let j = 1;
	let x = FixedU128::saturating_from_rational(111_u128, 100_u128);
	let xp = vec![
		FixedU128::saturating_from_rational(11_u128, 10_u128),
		FixedU128::saturating_from_rational(88_u128, 100_u128),
	];
	let amp = FixedU128::saturating_from_rational(292_u128, 100_u128);
	let ann = CurveAmm::get_ann(amp, xp.len()).unwrap();

	let result = CurveAmm::get_y(i, j, x, &xp, ann);

	assert_eq!(result, None);
}

#[test]
fn get_y_i_greater_than_n() {
	let i = 33;
	let j = 1;
	let x = FixedU128::saturating_from_rational(111_u128, 100_u128);
	let xp = vec![
		FixedU128::saturating_from_rational(11_u128, 10_u128),
		FixedU128::saturating_from_rational(88_u128, 100_u128),
	];
	let amp = FixedU128::saturating_from_rational(292_u128, 100_u128);
	let ann = CurveAmm::get_ann(amp, xp.len()).unwrap();

	let result = CurveAmm::get_y(i, j, x, &xp, ann);

	assert_eq!(result, None);
}

#[test]
fn get_y_j_greater_than_n() {
	let i = 1;
	let j = 33;
	let x = FixedU128::saturating_from_rational(111_u128, 100_u128);
	let xp = vec![
		FixedU128::saturating_from_rational(11_u128, 10_u128),
		FixedU128::saturating_from_rational(88_u128, 100_u128),
	];
	let amp = FixedU128::saturating_from_rational(292_u128, 100_u128);
	let ann = CurveAmm::get_ann(amp, xp.len()).unwrap();

	let result = CurveAmm::get_y(i, j, x, &xp, ann);

	assert_eq!(result, None);
}

#[test]
fn test_get_exchange_value() {
	new_test_ext().execute_with(|| {
		let assets = vec![MockCurrencyId::USDC, MockCurrencyId::USDT];
		let amp_coeff = FixedU128::saturating_from_integer(1_000_i128);
		let fee = Permill::zero();
		let admin_fee = Permill::zero();
		let amounts = vec![999999999u128, 999999999u128];
		let pool_id = create_pool(assets, amounts, amp_coeff, fee, admin_fee);
		let balance = 100u128;
		let price_usdc = CurveAmm::get_exchange_value(pool_id, MockCurrencyId::USDC, balance);
		assert_ok!(price_usdc);
		let price_usdc = price_usdc.unwrap();
		let price_usdt = CurveAmm::get_exchange_value(pool_id, MockCurrencyId::USDT, balance);
		assert_ok!(price_usdt);
		let price_usdt = price_usdt.unwrap();
		sp_std::if_std! {
			println!("usdc_price {:?}", price_usdc);
			println!("usdt_price {:?}", price_usdt);
		}
		assert!(price_usdc == price_usdt);
	});
}

#[test]
fn add_remove_liquidity() {
	new_test_ext().execute_with(|| {
		let assets = vec![USDC, USDT];
		let amp_coeff = FixedU128::saturating_from_rational(1_000_i128, 1_i128);
		let fee = Permill::zero();
		let admin_fee = Permill::zero();

		assert_eq!(Tokens::balance(USDT, &ALICE), 0);
		assert_ok!(Tokens::mint_into(USDT, &ALICE, 200_000));
		assert_eq!(Tokens::balance(USDT, &ALICE), 200_000);

		assert_eq!(Tokens::balance(USDC, &ALICE), 0);
		assert_ok!(Tokens::mint_into(USDC, &ALICE, 200_000));
		assert_eq!(Tokens::balance(USDC, &ALICE), 200_000);

		assert_eq!(Tokens::balance(USDT, &BOB), 0);
		assert_ok!(Tokens::mint_into(USDT, &BOB, 200_000));
		assert_eq!(Tokens::balance(USDT, &BOB), 200_000);

		assert_eq!(Tokens::balance(USDC, &BOB), 0);
		assert_ok!(Tokens::mint_into(USDC, &BOB, 200_000));
		assert_eq!(Tokens::balance(USDC, &BOB), 200_000);

		let p = CurveAmm::create_pool(&ALICE, assets, amp_coeff, fee, admin_fee);
		assert_ok!(&p);
		let pool_id = p.unwrap();
		let pool = CurveAmm::get_pool_info(pool_id);
		assert!(pool.is_some());
		let pool = pool.unwrap();

		let pool_lp_asset = pool.lp_token;

		// 1 USDC = 1 USDT
		let amounts = vec![130_000_u128, 130_000_u128];
		assert_ok!(CurveAmm::add_liquidity(&ALICE, pool_id, amounts.clone(), 0_u128));
		let alice_lp_asset_balance = Tokens::balance(pool_lp_asset, &ALICE);
		assert_ne!(alice_lp_asset_balance, 0);
		assert_eq!(Tokens::balance(USDT, &ALICE), 200_000 - 1300_00);
		assert_eq!(Tokens::balance(USDC, &ALICE), 200_000 - 1300_00);
		let pool = CurveAmm::get_pool_info(pool_id);
		assert!(pool.is_some());
		assert_ok!(CurveAmm::add_liquidity(&BOB, pool_id, amounts.clone(), 0_u128));

		let bob_lp_asset_balance = Tokens::balance(pool_lp_asset, &BOB);
		assert_ne!(bob_lp_asset_balance, 0);
		assert_eq!(Tokens::balance(USDT, &BOB), 200_000 - 130_000);
		assert_eq!(Tokens::balance(USDC, &BOB), 200_000 - 130_000);

		let min_amt = vec![0_u128, 0_u128];
		assert_eq!(Tokens::balance(USDC, &CurveAmm::account_id(&pool_id)), 260_000);
		assert_eq!(Tokens::balance(USDT, &CurveAmm::account_id(&pool_id)), 260_000);

		assert_ok!(CurveAmm::remove_liquidity(
			&ALICE,
			pool_id,
			alice_lp_asset_balance,
			min_amt.clone()
		));
		assert_eq!(Tokens::balance(pool_lp_asset, &ALICE), 0);
		assert_eq!(Tokens::balance(USDT, &ALICE), 200_000);
		assert_eq!(Tokens::balance(USDC, &ALICE), 200_000);
		assert_eq!(Tokens::balance(USDC, &CurveAmm::account_id(&pool_id)), 130_000);
		assert_eq!(Tokens::balance(USDT, &CurveAmm::account_id(&pool_id)), 130_000);

		assert_ok!(CurveAmm::remove_liquidity(
			&BOB,
			pool_id,
			bob_lp_asset_balance,
			min_amt.clone()
		));
		assert_eq!(Tokens::balance(pool_lp_asset, &BOB), 0);
		assert_eq!(Tokens::balance(USDT, &BOB), 200_000);
		assert_eq!(Tokens::balance(USDC, &BOB), 200_000);
		assert_eq!(Tokens::balance(USDC, &CurveAmm::account_id(&pool_id)), 0);
		assert_eq!(Tokens::balance(USDT, &CurveAmm::account_id(&pool_id)), 0);
	});
}

#[test]
fn exchange_test() {
	new_test_ext().execute_with(|| {
		let assets = vec![USDC, USDT];
		let amp_coeff = FixedU128::saturating_from_rational(1000_i128, 1_i128);
		let fee = Permill::zero();
		let admin_fee = Permill::zero();
		let amounts = vec![999999999u128, 999999999u128];
		let pool_id = create_pool(assets, amounts, amp_coeff, fee, admin_fee);

		assert_eq!(Tokens::balance(USDT, &ALICE), 0);
		assert_ok!(Tokens::mint_into(USDT, &ALICE, 200000));
		assert_eq!(Tokens::balance(USDT, &ALICE), 200000);
		assert_eq!(Tokens::balance(USDC, &ALICE), 0);
		assert_ok!(Tokens::mint_into(USDC, &ALICE, 200000));
		assert_eq!(Tokens::balance(USDC, &ALICE), 200000);
		assert_eq!(Tokens::balance(USDT, &BOB), 0);
		assert_ok!(Tokens::mint_into(USDT, &BOB, 200000));
		assert_eq!(Tokens::balance(USDT, &BOB), 200000);
		assert_eq!(Tokens::balance(USDC, &BOB), 0);
		assert_ok!(Tokens::mint_into(USDC, &BOB, 200000));
		assert_eq!(Tokens::balance(USDC, &BOB), 200000);
		assert_eq!(Tokens::balance(USDT, &CHARLIE), 0);
		assert_ok!(Tokens::mint_into(USDT, &CHARLIE, 200000));
		assert_eq!(Tokens::balance(USDT, &CHARLIE), 200000);
		let p = CurveAmm::create_pool(&ALICE, assets, amp_coeff, fee, admin_fee);
		assert_ok!(&p);
		let pool_id = p.unwrap();
		let pool = CurveAmm::get_pool_info(pool_id);
		assert!(pool.is_some());
		let pool = pool.unwrap();
		let pool_lp_asset = pool.lp_token;
		// 1 USDC = 1 USDT
		let amounts = vec![130000_u128, 130000_u128];
		assert_ok!(CurveAmm::add_liquidity(&ALICE, pool_id, amounts.clone(), 0_u128));
		let alice_balance = Tokens::balance(pool_lp_asset, &ALICE);
		assert_ne!(alice_balance, 0);
		assert_eq!(Tokens::balance(USDT, &ALICE), 200000 - 130000);
		assert_eq!(Tokens::balance(USDC, &ALICE), 200000 - 130000);
		let pool = CurveAmm::get_pool_info(pool_id);
		assert!(pool.is_some());
		assert_ok!(CurveAmm::add_liquidity(&BOB, pool_id, amounts.clone(), 0_u128));
		let bob_balance = Tokens::balance(pool_lp_asset, &BOB);
		assert_ne!(bob_balance, 0);
		assert_eq!(Tokens::balance(USDT, &BOB), 200000 - 130000);
		assert_eq!(Tokens::balance(USDC, &BOB), 200000 - 130000);
		assert_eq!(Tokens::balance(USDC, &CHARLIE), 0);
		assert_ok!(CurveAmm::exchange(&CHARLIE, pool_id, 1, 0, 65000, 0));
		assert!(65000 - Tokens::balance(USDC, &CHARLIE) < 10);
	});
}

#[test]
fn buy_test() {
	new_test_ext().execute_with(|| {
		let assets = vec![MockCurrencyId::USDC, MockCurrencyId::USDT];
		let amp_coeff = FixedU128::saturating_from_integer(1000_i128);
		let fee = Permill::zero();
		let admin_fee = Permill::zero();
		let amounts = vec![999999999u128, 999999999u128];
		let pool_id = create_pool(assets, amounts, amp_coeff, fee, admin_fee);

		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &CHARLIE, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &CHARLIE), 0);
		assert_ok!(CurveAmm::buy(&CHARLIE, pool_id, MockCurrencyId::USDC, 1000));
		assert!(1000 - Tokens::balance(MockCurrencyId::USDC, &CHARLIE) < 10);
	});
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(10000))]

	#[test]
	fn proptest_exchange(alice_balance in 1..u32::MAX,
						 bob_balance in 1..u32::MAX) {

	new_test_ext().execute_with(|| {
		// configuration for DEX Pool
		let assets = vec![USDC, USDT];
		let amp_coeff = FixedU128::saturating_from_rational(10000_i128, 1_i128);
		let fee = Permill::zero();
		let admin_fee = Permill::zero();

		// Add funds to ALICE's account.
		assert_eq!(Tokens::balance(USDT, &ALICE), 0);
		assert_ok!(Tokens::mint_into(USDT, &ALICE, alice_balance.into()));
		assert_eq!(Tokens::balance(USDT, &ALICE), alice_balance.into());
		assert_eq!(Tokens::balance(USDC, &ALICE), 0);
		assert_ok!(Tokens::mint_into(USDC, &ALICE, alice_balance.into()));
		assert_eq!(Tokens::balance(USDC, &ALICE), alice_balance.into());

		// Add funds to BOB's account.
		assert_eq!(Tokens::balance(USDT, &BOB), 0);
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_balance.into()));
		assert_eq!(Tokens::balance(USDT, &BOB), bob_balance.into());
		assert_eq!(Tokens::balance(USDC, &BOB), 0);
		assert_ok!(Tokens::mint_into(USDC, &BOB, bob_balance.into()));
		assert_eq!(Tokens::balance(USDC, &BOB), bob_balance.into());

		// Add funds to CHARLIE's account.
		assert_eq!(Tokens::balance(USDT, &CHARLIE), 0);
		assert_ok!(Tokens::mint_into(USDT, &CHARLIE, alice_balance.into()));
		assert_eq!(Tokens::balance(USDT, &CHARLIE), alice_balance.into());


		// create DEX pool for 1 USDC = 1 USDT
		let p = CurveAmm::create_pool(&ALICE, assets, amp_coeff, fee, admin_fee);
		assert_ok!(&p);
		let pool_id = p.unwrap();
		let pool = CurveAmm::get_pool_info(pool_id);
		assert!(pool.is_some());
		let pool = pool.unwrap();
		let pool_lp_asset = pool.lp_token;

		// ALICE adds liquidity to DEX pool.
		let alice_amounts = vec![alice_balance as u128, alice_balance as u128];
		assert_ok!(CurveAmm::add_liquidity(&ALICE, pool_id, alice_amounts.clone(), 0_u128));
		let alice_lp_balance = Tokens::balance(pool_lp_asset, &ALICE);
		assert_ne!(alice_lp_balance, 0);
		assert_eq!(Tokens::balance(USDT, &ALICE), 0);
		assert_eq!(Tokens::balance(USDC, &ALICE), 0);

		// BOB adds liquidity to DEX pool.
		let bob_amounts = vec![bob_balance as u128, bob_balance as u128];
		assert_ok!(CurveAmm::add_liquidity(&BOB, pool_id, bob_amounts.clone(), 0_u128));
		let bob_lp_balance = Tokens::balance(pool_lp_asset, &BOB);
		assert_ne!(bob_lp_balance, 0);
		assert_eq!(Tokens::balance(USDT, &BOB), 0);
		assert_eq!(Tokens::balance(USDC, &BOB), 0);

		// CHARLIE exchanges USDT for USDC, CHARLIE has same balance of USDT as of ALICE.
		assert_eq!(Tokens::balance(USDC, &CHARLIE), 0);
		assert_ok!(CurveAmm::exchange(&CHARLIE, pool_id, 1, 0, alice_balance as u128, 0));
		prop_assert_epsilon!(alice_balance as u128, Tokens::balance(USDC, &CHARLIE));
		Ok(())
	}).unwrap();
	}

	#[test]
	fn proptest_add_remove_liquidity(alice_balance in 0..u32::MAX,
									 bob_balance in 0..u32::MAX) {
	new_test_ext().execute_with(|| {
		// configuration for DEX Pool
		let assets = vec![USDC, USDT];
		let amp_coeff = FixedU128::saturating_from_rational(1000_i128, 1_i128);
		let fee = Permill::zero();
		let admin_fee = Permill::zero();

		// Add funds to ALICE's account.
		assert_eq!(Tokens::balance(USDT, &ALICE), 0);
		assert_ok!(Tokens::mint_into(USDT, &ALICE, alice_balance.into()));
		assert_eq!(Tokens::balance(USDT, &ALICE), alice_balance.into());
		assert_eq!(Tokens::balance(USDC, &ALICE), 0);
		assert_ok!(Tokens::mint_into(USDC, &ALICE, alice_balance.into()));
		assert_eq!(Tokens::balance(USDC, &ALICE), alice_balance.into());

		// Add funds to BOB's account.
		assert_eq!(Tokens::balance(USDT, &BOB), 0);
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_balance.into()));
		assert_eq!(Tokens::balance(USDT, &BOB), bob_balance.into());
		assert_eq!(Tokens::balance(USDC, &BOB), 0);
		assert_ok!(Tokens::mint_into(USDC, &BOB, bob_balance.into()));
		assert_eq!(Tokens::balance(USDC, &BOB), bob_balance.into());

		// create DEX pool for 1 USDC = 1 USDT
		let p = CurveAmm::create_pool(&ALICE, assets, amp_coeff, fee, admin_fee);
		assert_ok!(&p);
		let pool_id = p.unwrap();
		let pool = CurveAmm::get_pool_info(pool_id);
		assert!(pool.is_some());
		let pool = pool.unwrap();
		let pool_lp_asset = pool.lp_token;

		// ALICE adds liquidity to DEX pool.
		let alice_amounts = vec![alice_balance as u128, alice_balance as u128];
		assert_ok!(CurveAmm::add_liquidity(&ALICE, pool_id, alice_amounts.clone(), 0_u128));
		let alice_lp_balance = Tokens::balance(pool_lp_asset, &ALICE);
		assert_ne!(alice_lp_balance, 0);
		assert_eq!(Tokens::balance(USDT, &ALICE), 0);
		assert_eq!(Tokens::balance(USDC, &ALICE), 0);

		// BOB adds liquidity to DEX pool.
		let bob_amounts = vec![bob_balance as u128, bob_balance as u128];
		assert_ok!(CurveAmm::add_liquidity(&BOB, pool_id, bob_amounts.clone(), 0_u128));
		let bob_lp_balance = Tokens::balance(pool_lp_asset, &BOB);
		assert_ne!(bob_balance, 0);
		assert_eq!(Tokens::balance(USDT, &BOB), 0);
		assert_eq!(Tokens::balance(USDC, &BOB), 0);

		let min_amt = vec![0_u128, 0_u128];
		assert_eq!(Tokens::balance(USDC, &CurveAmm::account_id(&pool_id)), alice_balance as u128 + bob_balance as u128);
		assert_eq!(Tokens::balance(USDT, &CurveAmm::account_id(&pool_id)), alice_balance as u128 + bob_balance as u128);

		// ALICE removes liquidity from DEX pool.
		assert_ok!(CurveAmm::remove_liquidity(&ALICE, pool_id, alice_lp_balance, min_amt.clone()));
		prop_assert_zero_epsilon!(Tokens::balance(pool_lp_asset, &ALICE));
		prop_assert_epsilon!(Tokens::balance(USDT, &ALICE), alice_balance as u128);
		prop_assert_epsilon!(Tokens::balance(USDC, &ALICE), alice_balance as u128);
		prop_assert_epsilon!(Tokens::balance(USDC, &CurveAmm::account_id(&pool_id)), bob_balance as u128);
		prop_assert_epsilon!(Tokens::balance(USDT, &CurveAmm::account_id(&pool_id)), bob_balance as u128);

		// BOB removes liquidity from DEX pool.
		assert_ok!(CurveAmm::remove_liquidity(&BOB, pool_id, bob_lp_balance, min_amt.clone()));
		prop_assert_zero_epsilon!(Tokens::balance(pool_lp_asset, &BOB));
		prop_assert_zero_epsilon!(Tokens::balance(USDC, &CurveAmm::account_id(&pool_id)));
		prop_assert_zero_epsilon!(Tokens::balance(USDT, &CurveAmm::account_id(&pool_id)));
		prop_assert_epsilon!(Tokens::balance(USDT, &BOB), bob_balance as u128);
		prop_assert_epsilon!(Tokens::balance(USDC, &BOB), bob_balance as u128);
		Ok(())
	}).unwrap();
	}
}
