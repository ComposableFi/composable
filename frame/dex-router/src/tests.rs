use crate::{mock::*, Error};
use composable_tests_helpers::test::helper::acceptable_computation_error;
use composable_traits::{
	defi::CurrencyPair,
	dex::{Amm as AmmTrait, DexRouteNode, DexRouter as DexRouterTrait},
};
use frame_support::{assert_noop, assert_ok, traits::fungibles::Mutate};
use pallet_pablo::PoolInitConfiguration;
use sp_runtime::Permill;

// Create Amm pool with given amounts added as liquidity to the pool.
fn create_curve_amm_pool(
	assets: CurrencyPair<AssetId>,
	amounts: Vec<Balance>,
	amp_coeff: u16,
	fee: Permill,
	admin_fee: Permill,
) -> PoolId {
	let base = assets.base;
	let quote = assets.quote;
	assert_ok!(Tokens::mint_into(base, &ALICE, amounts[0]));
	assert_ok!(Tokens::mint_into(quote, &ALICE, amounts[1]));
	assert_ok!(Tokens::mint_into(base, &BOB, amounts[0]));
	assert_ok!(Tokens::mint_into(quote, &BOB, amounts[1]));

	let init_config = PoolInitConfiguration::StableSwap {
		owner: ALICE,
		pair: assets,
		amplification_coefficient: amp_coeff,
		fee,
		owner_fee: admin_fee,
	};
	let p = Pablo::do_create_pool(init_config);
	assert_ok!(&p);
	let pool_id = p.unwrap();
	// 1 USDC = 1 USDT
	assert_ok!(<Pablo as AmmTrait>::add_liquidity(
		&ALICE, pool_id, amounts[0], amounts[1], 0_u128, true
	));
	assert_ok!(<Pablo as AmmTrait>::add_liquidity(
		&BOB, pool_id, amounts[0], amounts[1], 0_u128, true
	));
	pool_id
}

// Create Pablo pool with given amounts added as liquidity to the pool.
fn create_constant_product_amm_pool(
	assets: CurrencyPair<AssetId>,
	amounts: Vec<Balance>,
	fee: Permill,
	admin_fee: Permill,
) -> PoolId {
	let base = assets.base;
	let quote = assets.quote;
	assert_ok!(Tokens::mint_into(base, &ALICE, amounts[0]));
	assert_ok!(Tokens::mint_into(quote, &ALICE, amounts[1]));
	assert_ok!(Tokens::mint_into(base, &BOB, amounts[0]));
	assert_ok!(Tokens::mint_into(quote, &BOB, amounts[1]));

	let init_config = PoolInitConfiguration::ConstantProduct {
		owner: ALICE,
		pair: assets,
		fee,
		owner_fee: admin_fee,
	};
	// Create Pablo pool
	let p = Pablo::do_create_pool(init_config);
	assert_ok!(&p);
	let pool_id = p.unwrap();
	// Add liquidity from ALICE's account to pool
	assert_ok!(<Pablo as AmmTrait>::add_liquidity(
		&ALICE, pool_id, amounts[0], amounts[1], 0_u128, true
	));
	// Add liquidity from BOB's account to pool
	assert_ok!(<Pablo as AmmTrait>::add_liquidity(
		&BOB, pool_id, amounts[0], amounts[1], 0_u128, true
	));
	pool_id
}

fn create_usdt_usdc_pool() -> PoolId {
	let unit = 1_000_000_000_000_u128;
	// usdc usdt have same price which is 1 USD
	let initial_usdc = 1_000_000_000 * unit;
	let initial_usdt = 1_000_000_000 * unit;
	let amp_coeff = 100;
	let fee = Permill::zero();
	let admin_fee = Permill::zero();
	let assets = CurrencyPair::new(USDT, USDC);
	let amounts = vec![initial_usdt, initial_usdc];
	create_curve_amm_pool(assets, amounts, amp_coeff, fee, admin_fee)
}

fn create_usdc_eth_pool() -> PoolId {
	let unit = 1_000_000_000_000_u128;
	let eth_price = 3000_u128;
	let eth_balance = 1_000_000_000 * unit;
	let usdc_balance = eth_price * eth_balance;
	let fee = Permill::zero();
	let admin_fee = Permill::zero();
	let assets = CurrencyPair::new(USDC, ETH);
	let amounts = vec![usdc_balance, eth_balance];
	create_constant_product_amm_pool(assets, amounts, fee, admin_fee)
}

#[test]
fn get_route_tests() {
	new_test_ext().execute_with(|| {
		let currency_pair = CurrencyPair { base: ETH, quote: USDT };
		assert_eq!(DexRouter::get_route(currency_pair), None);

		let dex_route = vec![
			DexRouteNode::Pablo(create_usdc_eth_pool()),
			DexRouteNode::Pablo(create_usdt_usdc_pool()),
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
			DexRouteNode::Pablo(create_usdc_eth_pool()),
			DexRouteNode::Pablo(create_usdt_usdc_pool()),
		];
		assert_ok!(DexRouter::update_route(
			&ALICE,
			currency_pair,
			Some(dex_route.clone().try_into().unwrap())
		));
		assert_eq!(DexRouter::get_route(currency_pair), Some(dex_route));

		// update
		let dex_route = vec![
			DexRouteNode::Pablo(create_usdt_usdc_pool()),
			DexRouteNode::Pablo(create_usdc_eth_pool()),
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
			DexRouteNode::Pablo(create_usdt_usdc_pool()),
			DexRouteNode::Pablo(42), // fake route
			DexRouteNode::Pablo(create_usdc_eth_pool()),
		];
		assert_noop!(
			DexRouter::update_route(&ALICE, currency_pair, Some(dex_route.try_into().unwrap())),
			Error::<Test>::PoolDoesNotExist,
		);

		// invalid route, case #2
		let dex_route = vec![
			DexRouteNode::Pablo(create_usdt_usdc_pool()),
			DexRouteNode::Pablo(create_usdc_eth_pool()),
			DexRouteNode::Pablo(42), // fake route
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
		let unit = 1_000_000_000_000_u128;
		let currency_pair = CurrencyPair { base: USDT, quote: ETH };
		let dex_route = vec![
			DexRouteNode::Pablo(create_usdc_eth_pool()),
			DexRouteNode::Pablo(create_usdt_usdc_pool()),
		];
		assert_ok!(DexRouter::update_route(
			&ALICE,
			currency_pair,
			Some(dex_route.try_into().unwrap())
		));
		assert_ok!(Tokens::mint_into(ETH, &CHARLIE, 10_u128 * unit));
		let dy = DexRouter::exchange(&CHARLIE, currency_pair, 1_u128 * unit);
		assert_ok!(dy);
		let dy = dy.unwrap();
		let expected_value = 3000 * unit;
		let precision = 100;
		let epsilon = 1;
		assert_ok!(acceptable_computation_error(dy, expected_value, precision, epsilon));
	});
}

#[test]
fn buy_test() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let currency_pair = CurrencyPair { base: USDT, quote: ETH };
		let dex_route = vec![
			DexRouteNode::Pablo(create_usdc_eth_pool()),
			DexRouteNode::Pablo(create_usdt_usdc_pool()),
		];
		// USDC/ETH
		// USDT/USDC
		// USDT/ETH
		assert_ok!(DexRouter::update_route(
			&ALICE,
			currency_pair,
			Some(dex_route.try_into().unwrap())
		));
		assert_ok!(Tokens::mint_into(ETH, &CHARLIE, 2_u128 * unit));
		let dy = DexRouter::buy(&CHARLIE, currency_pair, 3000_u128 * unit);
		assert_ok!(dy);
		let dy = dy.unwrap();
		let expected_value = 3000 * unit;
		let precision = 100;
		let epsilon = 1;
		assert_ok!(acceptable_computation_error(dy, expected_value, precision, epsilon));
	});
}
