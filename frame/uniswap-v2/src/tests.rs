use frame_support::assert_ok;

use crate::mock::*;
use composable_traits::dex::CurveAmm as ConstantProductAmmTrait;
use frame_support::traits::fungibles::{Inspect, Mutate};
use sp_runtime::Permill;

fn create_pool(
	assets: Vec<AssetId>,
	amounts: Vec<Balance>,
	fee: Permill,
	admin_fee: Permill,
) -> PoolId {
	assert_ok!(Tokens::mint_into(assets[0], &ALICE, amounts[0]));
	assert_ok!(Tokens::mint_into(assets[1], &ALICE, amounts[1]));
	assert_ok!(Tokens::mint_into(assets[0], &BOB, amounts[0]));
	assert_ok!(Tokens::mint_into(assets[1], &BOB, amounts[1]));
	let p = ConstantProductAmm::create_pool(&ALICE, assets, fee, admin_fee);
	assert_ok!(&p);
	let pool_id = p.unwrap();
	assert_ok!(ConstantProductAmm::add_liquidity(&ALICE, pool_id, amounts.clone(), 0_u128));
	assert_ok!(ConstantProductAmm::add_liquidity(&BOB, pool_id, amounts.clone(), 0_u128));
	pool_id
}

#[test]
fn test_get_exchange_value() {
	new_test_ext().execute_with(|| {
		let assets = vec![MockCurrencyId::ETH, MockCurrencyId::USDT];
		let fee = Permill::zero();
		let admin_fee = Permill::zero();
		let amounts = vec![10000, 30000000];
		let pool_id = create_pool(assets, amounts, fee, admin_fee);
		let eth_price = ConstantProductAmm::get_exchange_value(pool_id, MockCurrencyId::ETH, 1u128);
		assert_ok!(eth_price);
		let eth_price = eth_price.unwrap();
		assert!(eth_price <= 3000);
		assert!(eth_price > 2990);
		// check price for 3100 USDT (additional 100$ so that it returns 1 ETH)
		let usdt_price =
			ConstantProductAmm::get_exchange_value(pool_id, MockCurrencyId::USDT, 3100u128);
		assert_ok!(usdt_price);
		let usdt_price = usdt_price.unwrap();
		sp_std::if_std! {
			println!("usdt_price {:?}", usdt_price);
		}
		assert!(usdt_price == 1);
	});
}

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
		assert_ok!(ConstantProductAmm::exchange(&CHARLIE, pool_id, MockCurrencyId::USDT, 65000, 0));
		sp_std::if_std! {
			println!("CHARLIE's USDC balance {:?}" , Tokens::balance(   USDC, &CHARLIE));
		}
		assert!(65000 >= Tokens::balance(USDC, &CHARLIE));
	});
}

#[test]
fn buy_test() {
	new_test_ext().execute_with(|| {
		let assets = vec![MockCurrencyId::USDC, MockCurrencyId::USDT];
		let fee = Permill::zero();
		let admin_fee = Permill::zero();

		let amounts = vec![200000u128, 200000u128];
		let pool_id = create_pool(assets, amounts, fee, admin_fee);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &CHARLIE, 200000));
		assert_eq!(Tokens::balance(MockCurrencyId::USDC, &CHARLIE), 0);
		assert_ok!(ConstantProductAmm::buy(&CHARLIE, pool_id, MockCurrencyId::USDC, 1000));
		assert!(1000 - Tokens::balance(MockCurrencyId::USDC, &CHARLIE) < 10);
	});
}
