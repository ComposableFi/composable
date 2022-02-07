use crate::{mock::*, Error};
use composable_traits::{
	defi::CurrencyPair,
	dex::{CurveAmm as CurveAmmTrait, DexRouteNode, DexRouter as DexRouterTrait},
};
use frame_support::{assert_noop, assert_ok, traits::fungibles::Mutate};
use sp_runtime::{FixedPointNumber, FixedU128, Permill};

// Create CurveAmm pool with given amounts added as liquidity to the pool.
fn create_curve_amm_pool(
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
	// 1 USDC = 1 USDT
	assert_ok!(CurveAmm::add_liquidity(&ALICE, pool_id, amounts.clone(), 0_u128));
	assert_ok!(CurveAmm::add_liquidity(&BOB, pool_id, amounts, 0_u128));
	pool_id
}

// Create ConstantProductAmm pool with given amounts added as liquidity to the pool.
fn create_constant_product_amm_pool(
	assets: Vec<AssetId>,
	amounts: Vec<Balance>,
	fee: Permill,
	admin_fee: Permill,
) -> PoolId {
	assert_ok!(Tokens::mint_into(assets[0], &ALICE, amounts[0]));
	assert_ok!(Tokens::mint_into(assets[1], &ALICE, amounts[1]));
	assert_ok!(Tokens::mint_into(assets[0], &BOB, amounts[0]));
	assert_ok!(Tokens::mint_into(assets[1], &BOB, amounts[1]));

	// Create ConstantProductAmm pool
	let p = ConstantProductAmm::create_pool(&ALICE, assets, fee, admin_fee);
	assert_ok!(&p);
	let pool_id = p.unwrap();
	// Add liquidity from ALICE's account to pool
	assert_ok!(ConstantProductAmm::add_liquidity(&ALICE, pool_id, amounts.clone(), 0_u128));
	// Add liquidity from BOB's account to pool
	assert_ok!(ConstantProductAmm::add_liquidity(&BOB, pool_id, amounts, 0_u128));
	pool_id
}

fn create_usdc_usdt_pool() -> PoolId {
	let amp_coeff = FixedU128::saturating_from_rational(1000_i128, 1_i128);
	let fee = Permill::zero();
	let admin_fee = Permill::zero();
	let assets = vec![USDC, USDT];
	let amounts = vec![100000, 100000];
	create_curve_amm_pool(assets, amounts, amp_coeff, fee, admin_fee)
}

fn create_eth_usdc_pool() -> PoolId {
	let fee = Permill::zero();
	let admin_fee = Permill::zero();
	let assets = vec![ETH, USDC];
	let amounts = vec![1000, 3000000];
	create_constant_product_amm_pool(assets, amounts, fee, admin_fee)
}

#[test]
fn get_route_tests() {
	new_test_ext().execute_with(|| {
		let currency_pair = CurrencyPair { base: ETH, quote: USDT };
		assert_eq!(DexRouter::get_route(currency_pair), None);

		let dex_route = vec![
			DexRouteNode::Uniswap(create_eth_usdc_pool()),
			DexRouteNode::Curve(create_usdc_usdt_pool()),
		];
		assert_ok!(DexRouter::update_route(
			&ALICE,
			currency_pair,
			Some(dex_route.clone().try_into().unwrap())
		));
		assert_eq!(DexRouter::get_route(currency_pair), Some(dex_route));
	});
}

#[test]
fn update_route_tests() {
	new_test_ext().execute_with(|| {
		let currency_pair = CurrencyPair { base: ETH, quote: USDT };
		assert_eq!(DexRouter::get_route(currency_pair), None);

		// insert
		let dex_route = vec![
			DexRouteNode::Uniswap(create_eth_usdc_pool()),
			DexRouteNode::Curve(create_usdc_usdt_pool()),
		];
		assert_ok!(DexRouter::update_route(
			&ALICE,
			currency_pair,
			Some(dex_route.clone().try_into().unwrap())
		));
		assert_eq!(DexRouter::get_route(currency_pair), Some(dex_route));

		// update
		let dex_route = vec![
			DexRouteNode::Curve(create_usdc_usdt_pool()),
			DexRouteNode::Uniswap(create_eth_usdc_pool()),
		];
		assert_ok!(DexRouter::update_route(
			&ALICE,
			currency_pair,
			Some(dex_route.clone().try_into().unwrap())
		));
		assert_eq!(DexRouter::get_route(currency_pair), Some(dex_route));

		// delete
		assert_ok!(DexRouter::update_route(&ALICE, currency_pair, None));
		assert_eq!(DexRouter::get_route(currency_pair), None);

		// invalid route, case #1
		let dex_route = vec![
			DexRouteNode::Curve(create_usdc_usdt_pool()),
			DexRouteNode::Curve(42), // fake route
			DexRouteNode::Uniswap(create_eth_usdc_pool()),
		];
		assert_noop!(
			DexRouter::update_route(&ALICE, currency_pair, Some(dex_route.try_into().unwrap())),
			Error::<Test>::PoolDoesNotExist,
		);

		// invalid route, case #2
		let dex_route = vec![
			DexRouteNode::Curve(create_usdc_usdt_pool()),
			DexRouteNode::Uniswap(create_eth_usdc_pool()),
			DexRouteNode::Uniswap(42), // fake route
		];
		assert_noop!(
			DexRouter::update_route(&ALICE, currency_pair, Some(dex_route.try_into().unwrap())),
			Error::<Test>::PoolDoesNotExist,
		);
	});
}

#[test]
fn exchange_tests() {
	new_test_ext().execute_with(|| {
		let currency_pair = CurrencyPair { base: ETH, quote: USDT };
		let dex_route = vec![
			DexRouteNode::Uniswap(create_eth_usdc_pool()),
			DexRouteNode::Curve(create_usdc_usdt_pool()),
		];
		assert_ok!(DexRouter::update_route(
			&ALICE,
			currency_pair,
			Some(dex_route.try_into().unwrap())
		));
		assert_ok!(Tokens::mint_into(ETH, &CHARLIE, 10_u128));
		let dy = DexRouter::exchange(&CHARLIE, currency_pair, 1_u128);
		assert_ok!(dy);
		let dy = dy.unwrap();
		assert!(3000 >= dy);
		assert!(2995 < dy);
	});
}

#[test]
fn buy_test() {
	new_test_ext().execute_with(|| {
		let currency_pair = CurrencyPair { base: MockCurrencyId::ETH, quote: MockCurrencyId::USDT };
		let dex_route = vec![
			DexRouteNode::Uniswap(create_eth_usdc_pool()),
			DexRouteNode::Curve(create_usdc_usdt_pool()),
		];
		assert_ok!(DexRouter::update_route(
			&ALICE,
			currency_pair,
			Some(dex_route.try_into().unwrap())
		));
		assert_ok!(Tokens::mint_into(MockCurrencyId::ETH, &CHARLIE, 10u128));
		let dy = DexRouter::buy(&CHARLIE, currency_pair, 3100u128);
		assert_ok!(dy);
		let dy = dy.unwrap();
		assert!(3000 >= dy);
		assert!(2995 < dy);
	});
}
