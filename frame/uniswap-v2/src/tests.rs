use frame_support::assert_ok;

use crate::mock::*;
use composable_tests_helpers::{
	prop_assert_acceptable_computation_error, prop_assert_zero_epsilon,
};
use composable_traits::dex::CurveAmm as ConstantProductAmmTrait;
use frame_support::traits::fungibles::{Inspect, Mutate};
use proptest::prelude::*;
use sp_runtime::Permill;

#[test]
fn add_remove_liquidity() {
	new_test_ext().execute_with(|| {
		// ConstantProductAmm configurations
		let assets = vec![USDC, USDT];
		let fee = Permill::zero();
		let admin_fee = Permill::zero();

		// Mint USDT for ALICE
		assert_eq!(Tokens::balance(USDT, &ALICE), 0);
		assert_ok!(Tokens::mint_into(USDT, &ALICE, 200000));
		assert_eq!(Tokens::balance(USDT, &ALICE), 200000);
		// Mint USDC for ALICE
		assert_eq!(Tokens::balance(USDC, &ALICE), 0);
		assert_ok!(Tokens::mint_into(USDC, &ALICE, 200000));
		assert_eq!(Tokens::balance(USDC, &ALICE), 200000);
		// Mint USDT for BOB
		assert_eq!(Tokens::balance(USDT, &BOB), 0);
		assert_ok!(Tokens::mint_into(USDT, &BOB, 200000));
		assert_eq!(Tokens::balance(USDT, &BOB), 200000);
		// Mint USDC for BOB
		assert_eq!(Tokens::balance(USDC, &BOB), 0);
		assert_ok!(Tokens::mint_into(USDC, &BOB, 200000));
		assert_eq!(Tokens::balance(USDC, &BOB), 200000);

		// Create ConstantProductAmm pool
		let p = ConstantProductAmm::create_pool(&ALICE, assets, fee, admin_fee);
		assert_ok!(&p);
		let pool_id = p.unwrap();
		let pool = ConstantProductAmm::get_pool_info(pool_id);
		assert!(pool.is_some());
		let pool = pool.unwrap();
		let pool_lp_asset = pool.lp_token;
		// 1 USDC = 1 USDT

		// Add liquidity from ALICE's account to pool
		let amounts = vec![130000u128, 130000u128];
		assert_ok!(ConstantProductAmm::add_liquidity(&ALICE, pool_id, amounts.clone(), 0u128));
		let alice_balance = Tokens::balance(pool_lp_asset, &ALICE);
		assert_ne!(alice_balance, 0);
		assert_eq!(Tokens::balance(USDT, &ALICE), 200000 - 130000);
		assert_eq!(Tokens::balance(USDC, &ALICE), 200000 - 130000);
		let pool = ConstantProductAmm::get_pool_info(pool_id);
		assert!(pool.is_some());

		// Add liquidity from BOB's account to pool
		assert_ok!(ConstantProductAmm::add_liquidity(&BOB, pool_id, amounts.clone(), 0u128));
		let bob_balance = Tokens::balance(pool_lp_asset, &BOB);
		assert_ne!(bob_balance, 0);
		assert_eq!(Tokens::balance(USDT, &BOB), 200000 - 130000);
		assert_eq!(Tokens::balance(USDC, &BOB), 200000 - 130000);
		let min_amt = vec![0u128, 0u128];

		// Check that pool has USDT and USDC transferred from ALICE and BOB
		assert_eq!(Tokens::balance(USDC, &ConstantProductAmm::account_id(&pool_id)), 260000);
		assert_eq!(Tokens::balance(USDT, &ConstantProductAmm::account_id(&pool_id)), 260000);

		// Withdraw ALICE"s fund from pool.
		assert_ok!(ConstantProductAmm::remove_liquidity(
			&ALICE,
			pool_id,
			alice_balance,
			min_amt.clone()
		));
		// Check balances which should be impacted.
		assert_eq!(Tokens::balance(pool_lp_asset, &ALICE), 0);
		assert_eq!(Tokens::balance(USDT, &ALICE), 200000);
		assert_eq!(Tokens::balance(USDC, &ALICE), 200000);
		assert_eq!(Tokens::balance(USDC, &ConstantProductAmm::account_id(&pool_id)), 130000);
		assert_eq!(Tokens::balance(USDT, &ConstantProductAmm::account_id(&pool_id)), 130000);

		// Withdraw BOB"s fund from pool.
		assert_ok!(ConstantProductAmm::remove_liquidity(
			&BOB,
			pool_id,
			bob_balance,
			min_amt.clone()
		));
		// Check balances which should be impacted.
		assert_eq!(Tokens::balance(pool_lp_asset, &BOB), 0);
		assert_eq!(Tokens::balance(USDT, &BOB), 200000);
		assert_eq!(Tokens::balance(USDC, &BOB), 200000);
		assert_eq!(Tokens::balance(USDC, &ConstantProductAmm::account_id(&pool_id)), 0);
		assert_eq!(Tokens::balance(USDT, &ConstantProductAmm::account_id(&pool_id)), 0);
	});
}

#[test]
fn exchange_test() {
	new_test_ext().execute_with(|| {
		// ConstantProductAmm configurations
		let assets = vec![USDC, USDT];
		let fee = Permill::zero();
		let admin_fee = Permill::zero();

		// Mint USDT for ALICE
		assert_eq!(Tokens::balance(USDT, &ALICE), 0);
		assert_ok!(Tokens::mint_into(USDT, &ALICE, 200000));
		assert_eq!(Tokens::balance(USDT, &ALICE), 200000);
		// Mint USDC for ALICE
		assert_eq!(Tokens::balance(USDC, &ALICE), 0);
		assert_ok!(Tokens::mint_into(USDC, &ALICE, 200000));
		assert_eq!(Tokens::balance(USDC, &ALICE), 200000);
		// Mint USDT for BOB
		assert_eq!(Tokens::balance(USDT, &BOB), 0);
		assert_ok!(Tokens::mint_into(USDT, &BOB, 200000));
		assert_eq!(Tokens::balance(USDT, &BOB), 200000);
		// Mint USDC for BOB
		assert_eq!(Tokens::balance(USDC, &BOB), 0);
		assert_ok!(Tokens::mint_into(USDC, &BOB, 200000));
		assert_eq!(Tokens::balance(USDC, &BOB), 200000);
		// Mint USDT for CHARLIE
		assert_eq!(Tokens::balance(USDT, &CHARLIE), 0);
		assert_ok!(Tokens::mint_into(USDT, &CHARLIE, 200000));
		assert_eq!(Tokens::balance(USDT, &CHARLIE), 200000);

		// Create ConstantProductAmm pool
		let p = ConstantProductAmm::create_pool(&ALICE, assets, fee, admin_fee);
		assert_ok!(&p);
		let pool_id = p.unwrap();
		let pool = ConstantProductAmm::get_pool_info(pool_id);
		assert!(pool.is_some());
		let pool = pool.unwrap();
		let pool_lp_asset = pool.lp_token;
		// 1 USDC = 1 USDT
		// Add liquidity from ALICE's account to pool
		let amounts = vec![130000u128, 130000u128];
		assert_ok!(ConstantProductAmm::add_liquidity(&ALICE, pool_id, amounts.clone(), 0u128));
		let alice_balance = Tokens::balance(pool_lp_asset, &ALICE);
		assert_ne!(alice_balance, 0);
		assert_eq!(Tokens::balance(USDT, &ALICE), 200000 - 130000);
		assert_eq!(Tokens::balance(USDC, &ALICE), 200000 - 130000);
		let pool = ConstantProductAmm::get_pool_info(pool_id);
		assert!(pool.is_some());
		// Add liquidity from BOB's account to pool
		assert_ok!(ConstantProductAmm::add_liquidity(&BOB, pool_id, amounts.clone(), 0u128));
		let bob_balance = Tokens::balance(pool_lp_asset, &BOB);
		assert_ne!(bob_balance, 0);
		assert_eq!(Tokens::balance(USDT, &BOB), 200000 - 130000);
		assert_eq!(Tokens::balance(USDC, &BOB), 200000 - 130000);
		assert_eq!(Tokens::balance(USDC, &CHARLIE), 0);
		// CHARLIE exchanges USDT for USDC
		assert_ok!(ConstantProductAmm::exchange(&CHARLIE, pool_id, 1, 0, 65000, 0));
		sp_std::if_std! {
			println!("CHARLIE's USDC balance {:?}" , Tokens::balance(   USDC, &CHARLIE));
		}
		assert!(65000 >= Tokens::balance(USDC, &CHARLIE));
	});
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(10000))]
	#[test]
	fn proptest_add_remove_liquidity(
		alice_balance in 0..u32::MAX,
		bob_balance in 0..u32::MAX
	) {
	new_test_ext().execute_with(|| {
		// configuration for DEX Pool
		let assets = vec![USDC, USDT];
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
		let p = ConstantProductAmm::create_pool(&ALICE, assets, fee, admin_fee);
		assert_ok!(&p);
		let pool_id = p.unwrap();
		let pool = ConstantProductAmm::get_pool_info(pool_id);
		assert!(pool.is_some());
		let pool = pool.unwrap();
		let pool_lp_asset = pool.lp_token;

		// ALICE adds liquidity to DEX pool.
		let alice_amounts = vec![alice_balance as u128, alice_balance as u128];
		assert_ok!(ConstantProductAmm::add_liquidity(&ALICE, pool_id, alice_amounts.clone(), 0u128));
		let alice_lp_balance = Tokens::balance(pool_lp_asset, &ALICE);
		assert_ne!(alice_lp_balance, 0);
		assert_eq!(Tokens::balance(USDT, &ALICE), 0);
		assert_eq!(Tokens::balance(USDC, &ALICE), 0);

		// BOB adds liquidity to DEX pool.
		let bob_amounts = vec![bob_balance as u128, bob_balance as u128];
		assert_ok!(ConstantProductAmm::add_liquidity(&BOB, pool_id, bob_amounts.clone(), 0u128));
		let bob_lp_balance = Tokens::balance(pool_lp_asset, &BOB);
		assert_ne!(bob_balance, 0);
		assert_eq!(Tokens::balance(USDT, &BOB), 0);
		assert_eq!(Tokens::balance(USDC, &BOB), 0);

		let min_amt = vec![0u128, 0u128];
		assert_eq!(Tokens::balance(USDC, &ConstantProductAmm::account_id(&pool_id)), alice_balance as u128 + bob_balance as u128);
		assert_eq!(Tokens::balance(USDT, &ConstantProductAmm::account_id(&pool_id)), alice_balance as u128 + bob_balance as u128);

		// ALICE removes liquidity from DEX pool.
		assert_ok!(ConstantProductAmm::remove_liquidity(&ALICE, pool_id, alice_lp_balance, min_amt.clone()));
		prop_assert_zero_epsilon!(Tokens::balance(pool_lp_asset, &ALICE));
		prop_assert_acceptable_computation_error!(Tokens::balance(USDT, &ALICE), alice_balance as u128);
		prop_assert_acceptable_computation_error!(Tokens::balance(USDC, &ALICE), alice_balance as u128);
		prop_assert_acceptable_computation_error!(Tokens::balance(USDC, &ConstantProductAmm::account_id(&pool_id)), bob_balance as u128);
		prop_assert_acceptable_computation_error!(Tokens::balance(USDT, &ConstantProductAmm::account_id(&pool_id)), bob_balance as u128);

		// BOB removes liquidity from DEX pool.
		assert_ok!(ConstantProductAmm::remove_liquidity(&BOB, pool_id, bob_lp_balance, min_amt.clone()));
		prop_assert_zero_epsilon!(Tokens::balance(pool_lp_asset, &BOB));
		prop_assert_zero_epsilon!(Tokens::balance(USDC, &ConstantProductAmm::account_id(&pool_id)));
		prop_assert_zero_epsilon!(Tokens::balance(USDT, &ConstantProductAmm::account_id(&pool_id)));
		prop_assert_acceptable_computation_error!(Tokens::balance(USDT, &BOB), bob_balance as u128);
		prop_assert_acceptable_computation_error!(Tokens::balance(USDC, &BOB), bob_balance as u128);
		Ok(())
	}).unwrap();
	}
	#[test]
	fn proptest_exchange(
		alice_balance in 1..u32::MAX,
		bob_balance in 1..u32::MAX
	) {

	new_test_ext().execute_with(|| {
		// configuration for DEX Pool
		let assets = vec![USDC, USDT];
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
		let p = ConstantProductAmm::create_pool(&ALICE, assets, fee, admin_fee);
		assert_ok!(&p);
		let pool_id = p.unwrap();
		let pool = ConstantProductAmm::get_pool_info(pool_id);
		assert!(pool.is_some());
		let pool = pool.unwrap();
		let pool_lp_asset = pool.lp_token;

		// ALICE adds liquidity to DEX pool.
		let alice_amounts = vec![alice_balance as u128, alice_balance as u128];
		assert_ok!(ConstantProductAmm::add_liquidity(&ALICE, pool_id, alice_amounts.clone(), 0u128));
		let alice_lp_balance = Tokens::balance(pool_lp_asset, &ALICE);
		assert_ne!(alice_lp_balance, 0);
		assert_eq!(Tokens::balance(USDT, &ALICE), 0);
		assert_eq!(Tokens::balance(USDC, &ALICE), 0);

		// BOB adds liquidity to DEX pool.
		let bob_amounts = vec![bob_balance as u128, bob_balance as u128];
		assert_ok!(ConstantProductAmm::add_liquidity(&BOB, pool_id, bob_amounts.clone(), 0u128));
		let bob_lp_balance = Tokens::balance(pool_lp_asset, &BOB);
		assert_ne!(bob_lp_balance, 0);
		assert_eq!(Tokens::balance(USDT, &BOB), 0);
		assert_eq!(Tokens::balance(USDC, &BOB), 0);

		// CHARLIE exchanges USDT for USDC, CHARLIE has same balance of USDT as of ALICE.
		assert_eq!(Tokens::balance(USDC, &CHARLIE), 0);
		assert_ok!(ConstantProductAmm::exchange(&CHARLIE, pool_id, 1, 0, alice_balance as u128, 0));
		assert_ne!(Tokens::balance(USDC, &CHARLIE), 0);
	});
	}
}
