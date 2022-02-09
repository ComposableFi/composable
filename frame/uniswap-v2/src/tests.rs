use frame_support::assert_ok;

use crate::mock::*;
use composable_traits::dex::CurveAmm as ConstantProductAmmTrait;
use frame_support::traits::fungibles::{Inspect, Mutate};
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
