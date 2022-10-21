use crate::{mock::*, Error};
use composable_tests_helpers::test::{currency::USDT, helper::acceptable_computation_error};
use composable_traits::{
	defi::CurrencyPair,
	dex::{Amm as AmmTrait, AssetAmount, DexRouter as DexRouterTrait},
};
use frame_support::{
	assert_noop, assert_ok,
	error::BadOrigin,
	traits::fungibles::{Inspect, Mutate},
};
use pallet_pablo::{Error as PabloError, PoolInitConfiguration};
use sp_arithmetic::PerThing;
use sp_runtime::{traits::ConstU32, BoundedBTreeMap, Permill};
use std::collections::BTreeMap;

pub fn dual_asset_pool_weights(
	first_asset: AssetId,
	first_asset_weight: Permill,
	second_asset: AssetId,
) -> BoundedBTreeMap<AssetId, Permill, ConstU32<2>> {
	let mut asset_weights = BoundedBTreeMap::new();
	asset_weights.try_insert(first_asset, first_asset_weight).expect("Should work");
	asset_weights
		.try_insert(second_asset, first_asset_weight.left_from_one())
		.expect("Should work");
	asset_weights
}

// Create Pablo pool with given amounts added as liquidity to the pool.
fn create_constant_product_amm_pool(
	assets: CurrencyPair<AssetId>,
	amounts: Vec<Balance>,
	fee: Permill,
) -> PoolId {
	let base = assets.base;
	let quote = assets.quote;
	assert_ok!(Tokens::mint_into(base, &ALICE, amounts[0]));
	assert_ok!(Tokens::mint_into(quote, &ALICE, amounts[1]));
	assert_ok!(Tokens::mint_into(base, &BOB, amounts[0]));
	assert_ok!(Tokens::mint_into(quote, &BOB, amounts[1]));

	let init_config = PoolInitConfiguration::DualAssetConstantProduct {
		owner: ALICE,
		assets_weights: dual_asset_pool_weights(base, Permill::from_percent(50), quote),
		fee,
	};
	// Create Pablo pool
	let p = Pablo::do_create_pool(init_config);
	assert_ok!(&p);
	let pool_id = p.unwrap();
	// Add liquidity from ALICE's account to pool
	assert_ok!(<Pablo as AmmTrait>::add_liquidity(
		&ALICE,
		pool_id,
		BTreeMap::from([(base, amounts[0]), (quote, amounts[1])]),
		0_u128,
		true
	));
	// Add liquidity from BOB's account to pool
	assert_ok!(<Pablo as AmmTrait>::add_liquidity(
		&BOB,
		pool_id,
		BTreeMap::from([(base, amounts[0]), (quote, amounts[1])]),
		0_u128,
		true
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
	let assets = CurrencyPair::new(USDT, USDC);
	let amounts = vec![initial_usdt, initial_usdc];
	create_constant_product_amm_pool(assets, amounts, fee)
}

fn create_usdc_usdt_pool() -> PoolId {
	let unit = 1_000_000_000_000_u128;
	// usdc usdt have same price which is 1 USD
	let initial_usdc = 1_000_000_000 * unit;
	let initial_usdt = 1_000_000_000 * unit;
	let amp_coeff = 100;
	let fee = Permill::zero();
	let assets = CurrencyPair::new(USDC, USDT);
	let amounts = vec![initial_usdc, initial_usdt];
	create_constant_product_amm_pool(assets, amounts, fee)
}

fn create_usdt_dai_pool() -> PoolId {
	let unit = 1_000_000_000_000_u128;
	// usdc usdt have same price which is 1 USD
	let initial_dai = 1_000_000_000 * unit;
	let initial_usdt = 1_000_000_000 * unit;
	let amp_coeff = 100;
	let fee = Permill::zero();
	let assets = CurrencyPair::new(USDT, DAI);
	let amounts = vec![initial_usdt, initial_dai];
	create_constant_product_amm_pool(assets, amounts, fee)
}

fn create_usdc_eth_pool() -> PoolId {
	let unit = 1_000_000_000_000_u128;
	let eth_price = 3000_u128;
	let eth_balance = 1_000_000_000 * unit;
	let usdc_balance = eth_price * eth_balance;
	let fee = Permill::zero();
	let assets = CurrencyPair::new(USDC, ETH);
	let amounts = vec![usdc_balance, eth_balance];
	create_constant_product_amm_pool(assets, amounts, fee)
}

fn create_dai_eth_pool() -> PoolId {
	let unit = 1_000_000_000_000_u128;
	let eth_price = 3000_u128;
	let eth_balance = 1_000_000_000 * unit;
	let dai_balance = eth_price * eth_balance;
	let fee = Permill::zero();
	let assets = CurrencyPair::new(DAI, ETH);
	let amounts = vec![dai_balance, eth_balance];
	create_constant_product_amm_pool(assets, amounts, fee)
}

#[test]
fn get_route_tests() {
	new_test_ext().execute_with(|| {
		let currency_pair = CurrencyPair { base: USDT, quote: ETH };
		assert_eq!(DexRouter::get_route(currency_pair), None);

		let dex_route = vec![create_usdc_eth_pool(), create_usdt_usdc_pool()];
		assert_ok!(DexRouter::update_route(
			Origin::root(),
			currency_pair,
			Some(dex_route.clone().try_into().unwrap())
		));
		assert_eq!(DexRouter::get_route(currency_pair), Some((dex_route.clone(), false)));
		assert_eq!(DexRouter::get_route(currency_pair.swap()), Some((dex_route, true)));
	});
}

#[test]
fn update_route_origin_tests() {
	new_test_ext().execute_with(|| {
		let currency_pair = CurrencyPair { base: USDT, quote: ETH };
		assert_eq!(DexRouter::get_route(currency_pair), None);

		let dex_route = vec![create_usdc_eth_pool(), create_usdt_usdc_pool()];
		// only UpdateRouteOrigin can update the route which is set to EnsureRoot in mock.
		assert_noop!(
			DexRouter::update_route(
				Origin::signed(ALICE),
				currency_pair,
				Some(dex_route.clone().try_into().unwrap())
			),
			BadOrigin
		);
	});
}

#[test]
fn halborn_hal11_route_with_cycle() {
	new_test_ext().execute_with(|| {
		let currency_pair = CurrencyPair { base: USDT, quote: USDC };
		assert_eq!(DexRouter::get_route(currency_pair), None);

		let dex_route =
			vec![create_usdt_usdc_pool(), create_usdc_usdt_pool(), create_usdt_usdc_pool()];
		assert_noop!(
			DexRouter::update_route(
				Origin::root(),
				currency_pair,
				Some(dex_route.clone().try_into().unwrap())
			),
			Error::<Test>::LoopSuspectedInRouteUpdate
		);
		// An other variant, where same pool is used twice
		let currency_pair = CurrencyPair { base: USDT, quote: USDC };
		assert_eq!(DexRouter::get_route(currency_pair), None);
		let usdt_usdc_pool = create_usdt_usdc_pool();
		let usdc_usdt_pool = create_usdc_usdt_pool();
		let dex_route = vec![usdt_usdc_pool, usdc_usdt_pool, usdt_usdc_pool];
		assert_noop!(
			DexRouter::update_route(
				Origin::root(),
				currency_pair,
				Some(dex_route.clone().try_into().unwrap())
			),
			Error::<Test>::LoopSuspectedInRouteUpdate,
		);

		let dex_route = vec![usdt_usdc_pool, usdc_usdt_pool];
		assert_noop!(
			DexRouter::update_route(
				Origin::root(),
				CurrencyPair::new(USDC, USDC),
				Some(dex_route.clone().try_into().unwrap())
			),
			Error::<Test>::LoopSuspectedInRouteUpdate,
		);
	});
}

#[test]
fn update_route_tests() {
	new_test_ext().execute_with(|| {
		let currency_pair = CurrencyPair { base: USDT, quote: ETH };
		assert_eq!(DexRouter::get_route(currency_pair), None);

		// insert
		let dex_route = vec![create_usdc_eth_pool(), create_usdt_usdc_pool()];
		assert_ok!(DexRouter::update_route(
			Origin::root(),
			currency_pair,
			Some(dex_route.clone().try_into().unwrap())
		));
		assert_eq!(DexRouter::get_route(currency_pair), Some((dex_route, false)));

		// update
		let dex_route = vec![create_dai_eth_pool(), create_usdt_dai_pool()];
		assert_ok!(DexRouter::update_route(
			Origin::root(),
			currency_pair,
			Some(dex_route.clone().try_into().unwrap())
		));
		assert_eq!(DexRouter::get_route(currency_pair), Some((dex_route, false)));

		// delete
		assert_ok!(DexRouter::update_route(Origin::root(), currency_pair, None));
		assert_eq!(DexRouter::get_route(currency_pair), None);

		// invalid route, case #1
		let dex_route = vec![
			create_usdc_eth_pool(),
			42, // fake route
			create_usdt_usdc_pool(),
		];
		assert_noop!(
			DexRouter::update_route(
				Origin::root(),
				currency_pair,
				Some(dex_route.try_into().unwrap())
			),
			PabloError::<Test>::PoolNotFound,
		);

		// invalid route, case #2
		let dex_route = vec![create_usdt_usdc_pool(), create_usdc_eth_pool()];
		assert_noop!(
			DexRouter::update_route(
				Origin::root(),
				currency_pair,
				Some(dex_route.try_into().unwrap())
			),
			Error::<Test>::UnexpectedNodeFoundWhileValidation,
		);

		// invalid route, case #3
		let dex_route = vec![create_usdc_eth_pool()];
		assert_noop!(
			DexRouter::update_route(
				Origin::root(),
				currency_pair,
				Some(dex_route.try_into().unwrap())
			),
			Error::<Test>::UnexpectedNodeFoundWhileValidation,
		);

		// route with a single pool.
		let dex_route = vec![create_usdt_usdc_pool()];
		assert_ok!(DexRouter::update_route(
			Origin::root(),
			CurrencyPair::new(USDT, USDC),
			Some(dex_route.clone().try_into().unwrap())
		));
	});
}

#[test]
fn exchange_tests() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let currency_pair = CurrencyPair { base: USDT, quote: ETH };
		let dex_route = vec![create_usdc_eth_pool(), create_usdt_usdc_pool()];
		assert_ok!(DexRouter::update_route(
			Origin::root(),
			currency_pair,
			Some(dex_route.try_into().unwrap())
		));
		assert_ok!(Tokens::mint_into(ETH, &CHARLIE, 10_u128 * unit));
		// exchange ETH for USDT
		let dy = <DexRouter as AmmTrait>::do_swap(
			&CHARLIE,
			currency_pair,
			AssetAmount::new(currency_pair.quote, unit),
			AssetAmount::new(currency_pair.base, 2998_000_000_000_00_u128),
			false,
		);
		assert_ok!(dy);
		let dy = dy.unwrap();
		let expected_value = 3000 * unit;
		let precision = 100;
		let epsilon = 1;
		assert_ok!(acceptable_computation_error(
			dy.value.amount,
			expected_value,
			precision,
			epsilon
		));
		assert_ok!(Tokens::mint_into(USDT, &CHARLIE, 6000_u128 * unit));
		// exchange USDT for ETH
		let dy = <DexRouter as AmmTrait>::do_swap(
			&CHARLIE,
			currency_pair,
			AssetAmount::new(currency_pair.base, 3000_u128 * unit),
			AssetAmount::new(currency_pair.quote, 980000000000_u128),
			false,
		);
		assert_ok!(dy);
		let dy = dy.unwrap();
		let expected_value = unit;
		assert_ok!(acceptable_computation_error(
			dy.value.amount,
			expected_value,
			precision,
			epsilon
		));
		// exchange USDT for ETH but expect high value
		assert_noop!(
			<DexRouter as AmmTrait>::do_swap(
				&CHARLIE,
				currency_pair.swap(),
				AssetAmount::new(currency_pair.quote, 3000_u128 * unit),
				AssetAmount::new(currency_pair.base, 1100000007962_u128),
				false,
			),
			Error::<Test>::CanNotRespectMinAmountRequested
		);
	});
}

#[test]
fn buy_test() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let currency_pair = CurrencyPair { base: USDT, quote: ETH };
		let dex_route = vec![create_usdc_eth_pool(), create_usdt_usdc_pool()];
		// USDC/ETH
		// USDT/USDC
		// USDT/ETH
		assert_ok!(DexRouter::update_route(
			Origin::root(),
			currency_pair,
			Some(dex_route.try_into().unwrap())
		));
		assert_ok!(Tokens::mint_into(ETH, &CHARLIE, 2_u128 * unit));
		// buy 3000 USDT
		let dy = <DexRouter as AmmTrait>::do_buy(
			&CHARLIE,
			currency_pair,
			currency_pair.quote,
			AssetAmount::new(currency_pair.base, 3000_u128 * unit),
			false,
		);
		assert_ok!(dy);
		let dy = dy.unwrap();
		let expected_value = 3000 * unit;
		let precision = 100;
		let epsilon = 1;
		assert_ok!(acceptable_computation_error(
			dy.value.amount,
			expected_value,
			precision,
			epsilon
		));
		assert_ok!(Tokens::mint_into(USDT, &CHARLIE, 6100_u128 * unit));
		// buy 1 ETH
		let dy = <DexRouter as AmmTrait>::do_buy(
			&CHARLIE,
			currency_pair,
			currency_pair.base, /* will be ignored */
			AssetAmount::new(currency_pair.quote, 1_u128 * unit),
			false,
		);
		assert_ok!(dy);
		let dy = dy.unwrap();
		let expected_value = 1_u128 * unit;
		let precision = 100;
		let epsilon = 1;
		assert_ok!(acceptable_computation_error(
			dy.value.amount,
			expected_value,
			precision,
			epsilon
		));
		// buy 1 ETH but expect 1.0005
		assert_noop!(
			<DexRouter as AmmTrait>::do_buy(
				&CHARLIE,
				currency_pair,
				currency_pair.quote,
				AssetAmount::new(currency_pair.base, 1_u128 * unit),
				false,
			),
			Error::<Test>::CanNotRespectMinAmountRequested
		);
	});
}

#[test]
fn unsupported_operation_test() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let currency_pair = CurrencyPair { base: USDT, quote: ETH };
		let dex_route = vec![create_usdc_eth_pool(), create_usdt_usdc_pool()];
		// USDC/ETH
		// USDT/USDC
		// USDT/ETH
		assert_ok!(DexRouter::update_route(
			Origin::root(),
			currency_pair,
			Some(dex_route.try_into().unwrap())
		));
		// EVE adds liquidity to pool via dex-router
		let eth_amount = 3_u128 * unit;
		let usdt_amount = eth_amount * 3000_u128;
		assert_ok!(Tokens::mint_into(ETH, &EVE, eth_amount));
		assert_ok!(Tokens::mint_into(USDT, &EVE, usdt_amount));
		// base, quote amount should match currency_pair's base quote asset
		assert_noop!(
			<DexRouter as AmmTrait>::add_liquidity(
				&EVE,
				currency_pair.swap(),
				BTreeMap::from([(ETH, eth_amount), (USDT, usdt_amount)]),
				0_u128,
				false
			),
			Error::<Test>::UnsupportedOperation
		);
		assert_noop!(
			<DexRouter as AmmTrait>::remove_liquidity(
				&EVE,
				currency_pair,
				unit,
				BTreeMap::from([(ETH, 0), (USDT, 0)]),
			),
			Error::<Test>::UnsupportedOperation
		);
	});
}

#[test]
fn single_pool_route_test() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let currency_pair = CurrencyPair { base: USDC, quote: ETH };
		let dex_route = vec![create_usdc_eth_pool()];
		// USDC/ETH
		assert_ok!(DexRouter::update_route(
			Origin::root(),
			currency_pair,
			Some(dex_route.clone().try_into().unwrap())
		));
		assert_ok!(Tokens::mint_into(ETH, &CHARLIE, 3_u128 * unit));
		// buy 3000 USDC
		let dy = <DexRouter as AmmTrait>::do_buy(
			&CHARLIE,
			currency_pair,
			ETH, /* will be ignored */
			AssetAmount::new(USDC, 3000_u128 * unit),
			false,
		);
		assert_ok!(dy);
		let dy = dy.unwrap();
		let expected_value = 3000 * unit;
		let precision = 100;
		let epsilon = 1;
		assert_ok!(acceptable_computation_error(
			dy.value.amount,
			expected_value,
			precision,
			epsilon
		));
		// exchange ETH for USDT
		let dy = <DexRouter as AmmTrait>::do_swap(
			&CHARLIE,
			currency_pair,
			AssetAmount::new(currency_pair.quote, unit),
			AssetAmount::new(currency_pair.base, 2998_000_000_000_00_u128),
			false,
		);
		assert_ok!(dy);
		let dy = dy.unwrap();
		let expected_value = 3000 * unit;
		let precision = 100;
		let epsilon = 1;
		assert_ok!(acceptable_computation_error(
			dy.value.amount,
			expected_value,
			precision,
			epsilon
		));

		let pool_id = dex_route[0];
		let lp_token = Pablo::lp_token(pool_id);
		assert_ok!(lp_token);
		let lp_token = lp_token.unwrap();
		// EVE adds liquidity to pool via dex-router
		let eth_amount = 3_u128 * unit;
		let usdc_amount = eth_amount * 3000_u128;
		assert_ok!(Tokens::mint_into(ETH, &EVE, eth_amount));
		assert_ok!(Tokens::mint_into(USDC, &EVE, usdc_amount));
		// base, quote amount should match currency_pair's base quote asset
		assert_ok!(DexRouter::add_liquidity(
			Origin::signed(EVE),
			BTreeMap::from([(ETH, eth_amount), (USDT, usdc_amount)]),
			0_u128,
			false
		));
		let lp_amount = Tokens::balance(lp_token, &EVE);
		// min_base_amount, min_quote_amount should match currency_pair's base quote asset
		assert_ok!(DexRouter::remove_liquidity(
			Origin::signed(EVE),
			lp_amount,
			BTreeMap::from([(ETH, 0), (USDT, 0)]),
		));
		let bob_eth_amount = Tokens::balance(ETH, &EVE);
		let bob_usdc_amount = Tokens::balance(USDC, &EVE);
		assert_ok!(acceptable_computation_error(eth_amount, bob_eth_amount, precision, epsilon));
		assert_ok!(acceptable_computation_error(usdc_amount, bob_usdc_amount, precision, epsilon));
	});
}
