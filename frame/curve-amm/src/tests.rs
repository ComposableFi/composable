use frame_support::assert_ok;

use crate::mock::*;
use composable_traits::dex::CurveAmm as CurveAmmTrait;
use frame_support::traits::{
	fungibles::{Inspect, Mutate},
	OnInitialize,
};
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

#[test]
fn exchange_test() {
	new_test_ext().execute_with(|| {
		let assets = vec![MockCurrencyId::USDC, MockCurrencyId::USDT];
		let amp_coeff = 2u128;
		let fee = Permill::zero();
		let reserve_factor = Perquintill::from_percent(1);
		CurveAmm::on_initialize(0);
		CurveAmm::on_initialize(1);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &ALICE, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 200000);
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &ALICE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDC, &ALICE, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &ALICE), 200000);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &BOB, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 200000);
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &BOB), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDC, &BOB, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &BOB), 200000);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &CHARLIE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &CHARLIE, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &CHARLIE), 200000);
		let p = CurveAmm::create_pool(&ALICE, assets, amp_coeff, fee, reserve_factor);
		assert_ok!(&p);
		let pool_id = p.unwrap();
		let pool = CurveAmm::pool(pool_id);
		assert!(pool.is_some());
		let pool_info = pool.unwrap();
		let amounts = vec![130000u128, 130000u128];
		assert_ok!(Vault::deposit(
			Origin::signed(ALICE),
			pool_info.assets_vault_ids[0],
			amounts[0]
		));
		assert_ok!(Vault::deposit(
			Origin::signed(ALICE),
			pool_info.assets_vault_ids[1],
			amounts[1]
		));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 200000 - 130000);
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &ALICE), 200000 - 130000);
		assert_ok!(Vault::deposit(Origin::signed(BOB), pool_info.assets_vault_ids[0], amounts[0]));
		assert_ok!(Vault::deposit(Origin::signed(BOB), pool_info.assets_vault_ids[1], amounts[1]));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 200000 - 130000);
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &BOB), 200000 - 130000);
		for i in 2..10 {
			CurveAmm::on_initialize(i);
		}
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &CHARLIE), 0);
		let a = Tokens::balance(MockCurrencyId::USDC, &CHARLIE);
		assert_eq!(a, 0);
		assert_ok!(CurveAmm::exchange(&CHARLIE, pool_id, 1, 0, 100000, 0));
		let b = Tokens::balance(MockCurrencyId::USDC, &CHARLIE);
		assert_eq!(b, 92157);
		assert_eq!(b - a, 92157);
		let a = Tokens::balance(MockCurrencyId::USDT, &CHARLIE);
		assert_eq!(a, 100000);
		assert_ok!(CurveAmm::exchange(&CHARLIE, pool_id, 0, 1, 25000, 0));
		let b = Tokens::balance(MockCurrencyId::USDT, &CHARLIE);
		assert_eq!(b, 128958);
		assert_eq!(b - a, 28958);
		let a = Tokens::balance(MockCurrencyId::USDC, &CHARLIE);
		assert_eq!(a, 67157);
		assert_ok!(CurveAmm::exchange(&CHARLIE, pool_id, 1, 0, 1000, 0));
		let b = Tokens::balance(MockCurrencyId::USDC, &CHARLIE);
		assert_eq!(b, 68045);
		assert_eq!(b - a, 888);
		let a = Tokens::balance(MockCurrencyId::USDC, &CHARLIE);
		assert_eq!(a, 68045);
		assert_ok!(CurveAmm::exchange(&CHARLIE, pool_id, 1, 0, 3000, 0));
		let b = Tokens::balance(MockCurrencyId::USDC, &CHARLIE);
		assert_eq!(b, 70698);
		assert_eq!(b - a, 2653);
		let a = Tokens::balance(MockCurrencyId::USDT, &CHARLIE);
		assert_eq!(a, 124958);
		assert_ok!(CurveAmm::exchange(&CHARLIE, pool_id, 0, 1, 2500, 0));
		let b = Tokens::balance(MockCurrencyId::USDT, &CHARLIE);
		assert_eq!(b, 127784);
		assert_eq!(b - a, 2826);
		let a = Tokens::balance(MockCurrencyId::USDT, &CHARLIE);
		assert_eq!(a, 127784);
		assert_ok!(CurveAmm::exchange(&CHARLIE, pool_id, 0, 1, 300, 0));
		let b = Tokens::balance(MockCurrencyId::USDT, &CHARLIE);
		assert_eq!(b, 128122);
		assert_eq!(b - a, 338);
		let a = Tokens::balance(MockCurrencyId::USDT, &CHARLIE);
		assert_eq!(a, 128122);
		assert_ok!(CurveAmm::exchange(&CHARLIE, pool_id, 0, 1, 3, 0));
		let b = Tokens::balance(MockCurrencyId::USDT, &CHARLIE);
		assert_eq!(b, 128125);
		assert_eq!(b - a, 3);
		let a = Tokens::balance(MockCurrencyId::USDC, &CHARLIE);
		assert_eq!(a, 67895);
		assert_ok!(CurveAmm::exchange(&CHARLIE, pool_id, 1, 0, 1, 0));
		let b = Tokens::balance(MockCurrencyId::USDC, &CHARLIE);
		assert_eq!(b, 67895);
		assert_eq!(b - a, 0);
	});
}

#[test]
fn on_initialize_test() {
	new_test_ext().execute_with(|| {
		let assets = vec![MockCurrencyId::USDC, MockCurrencyId::USDT];
		let amp_coeff = 2u128;
		let fee = Permill::zero();
		let reserve_factor = Perquintill::from_percent(30);
		CurveAmm::on_initialize(0);
		CurveAmm::on_initialize(1);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &ALICE, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 200000);
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &ALICE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDC, &ALICE, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &ALICE), 200000);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &BOB, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 200000);
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &BOB), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDC, &BOB, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &BOB), 200000);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &CHARLIE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &CHARLIE, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &CHARLIE), 200000);
		let p = CurveAmm::create_pool(&ALICE, assets, amp_coeff, fee, reserve_factor);
		assert_ok!(&p);
		let pool_id = p.unwrap();
		let pool_account_id = CurveAmm::account_id(&pool_id);
		let pool = CurveAmm::pool(pool_id);
		assert!(pool.is_some());
		let pool_info = pool.unwrap();
		let amounts = vec![130000u128, 130000u128];
		assert_ok!(Vault::deposit(
			Origin::signed(ALICE),
			pool_info.assets_vault_ids[0],
			amounts[0]
		));
		assert_ok!(Vault::deposit(
			Origin::signed(ALICE),
			pool_info.assets_vault_ids[1],
			amounts[1]
		));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 200000 - 130000);
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &ALICE), 200000 - 130000);
		let bob_usdc_lp_tokens =
			Vault::deposit(Origin::signed(BOB), pool_info.assets_vault_ids[0], amounts[0]);
		let bob_usdt_lp_tokens =
			Vault::deposit(Origin::signed(BOB), pool_info.assets_vault_ids[1], amounts[1]);
		assert_ok!(bob_usdt_lp_tokens);
		assert_ok!(bob_usdc_lp_tokens);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 200000 - 130000);
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &BOB), 200000 - 130000);
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &pool_account_id), 0);
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &pool_account_id), 0);
		// both vaults are withdrwable
		CurveAmm::on_initialize(2);
		let pool_usdt_bal = Tokens::balance(MockCurrencyId::USDT, &pool_account_id);
		let pool_usdc_bal = Tokens::balance(MockCurrencyId::USDC, &pool_account_id);
		assert_ne!(pool_usdt_bal, 0);
		assert_ne!(pool_usdc_bal, 0);
		// BOB withdraws some USDC from vault. So USDC vault must be depositable
		assert_ok!(Vault::withdraw(Origin::signed(BOB), pool_info.assets_vault_ids[0], 5000));
		CurveAmm::on_initialize(3);
		sp_std::if_std! {
			println!("{:?} {:?}", pool_usdc_bal, Tokens::balance(MockCurrencyId::USDC, &pool_account_id));
		}
		CurveAmm::on_initialize(4);
		// BOB again withdraws from USDC pool and ALICE withdraws from USDT pool
		assert_ok!(Vault::withdraw(Origin::signed(BOB), pool_info.assets_vault_ids[0], 3000));
		assert_ok!(Vault::withdraw(Origin::signed(ALICE), pool_info.assets_vault_ids[1], 10000));
		CurveAmm::on_initialize(5);
	});
}
