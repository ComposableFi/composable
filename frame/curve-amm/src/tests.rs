use frame_support::assert_ok;

use crate::mock::*;
use composable_traits::dex::CurveAmm as CurveAmmTrait;
use frame_support::traits::fungibles::{Inspect, Mutate};
use sp_runtime::{
	traits::{Saturating, Zero},
	FixedPointNumber, FixedU128,
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

#[test]
fn add_remove_liquidity() {
	new_test_ext().execute_with(|| {
		let assets = vec![MockCurrencyId::BTC, MockCurrencyId::USDT];
		let amp_coeff = 2u128;

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
		let p = CurveAmm::create_pool(&ALICE, assets, amp_coeff);
		assert_ok!(&p);
		let pool_id = p.unwrap();
		let pool = CurveAmm::pool(pool_id);
		assert!(pool.is_some());
		let pool_info = pool.unwrap();
		// 1 BTC = 65000 USDT
		let amounts = vec![2u128, 130000u128];
		assert_ok!(CurveAmm::add_liquidity(&ALICE, pool_id, amounts.clone(), 0u128));
		let alice_balance = Tokens::balance(pool_info.pool_asset, &ALICE);
		assert_ne!(alice_balance, 0);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 200000 - 130000);
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &ALICE), 20 - 2);
		let pool = CurveAmm::pool(pool_id);
		assert!(pool.is_some());
		let pool_info = pool.unwrap();
		assert_ok!(CurveAmm::add_liquidity(&BOB, pool_id, amounts.clone(), 0u128));
		let bob_balance = Tokens::balance(pool_info.pool_asset, &BOB);
		assert_ne!(bob_balance, 0);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 200000 - 130000);
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &BOB), 20 - 2);
		let min_amt = vec![0u128, 0u128];
		assert_ok!(CurveAmm::remove_liquidity(&ALICE, pool_id, alice_balance, min_amt.clone()));
		assert_eq!(Tokens::balance(pool_info.pool_asset, &ALICE), 0);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 200000);
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &ALICE), 20);
		assert_ok!(CurveAmm::remove_liquidity(&BOB, pool_id, bob_balance, min_amt.clone()));
		assert_eq!(Tokens::balance(pool_info.pool_asset, &BOB), 0);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 200000);
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &BOB), 20);
	});
}
