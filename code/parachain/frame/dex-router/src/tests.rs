use crate::{mock::*, Error};
use composable_tests_helpers::test::helper::acceptable_computation_error;
use composable_traits::{
	defi::CurrencyPair,
	dex::{Amm as AmmTrait, DexRouter as DexRouterTrait},
};
use frame_support::{
	assert_noop, assert_ok,
	error::BadOrigin,
	traits::fungibles::{Inspect, Mutate},
};
use pallet_pablo::{Error as PabloError, PoolInitConfiguration};
use sp_runtime::Permill;

// Create Amm pool with given amounts added as liquidity to the pool.
fn create_curve_amm_pool(
	assets: CurrencyPair<AssetId>,
	amounts: Vec<Balance>,
	amp_coeff: u16,
	fee: Permill,
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
		base_weight: Permill::from_percent(50_u32),
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
		base_weight: Permill::from_percent(50),
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
	let assets = CurrencyPair::new(USDT, USDC);
	let amounts = vec![initial_usdt, initial_usdc];
	create_curve_amm_pool(assets, amounts, amp_coeff, fee)
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
	create_curve_amm_pool(assets, amounts, amp_coeff, fee)
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
	create_curve_amm_pool(assets, amounts, amp_coeff, fee)
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
		let dy = <DexRouter as AmmTrait>::exchange(
			&CHARLIE,
			currency_pair,
			currency_pair,
			1_u128 * unit,
			2998_000_000_000_00_u128,
			false,
		);
		assert_ok!(dy);
		let dy = dy.unwrap();
		let expected_value = 3000 * unit;
		let precision = 100;
		let epsilon = 1;
		assert_ok!(acceptable_computation_error(dy, expected_value, precision, epsilon));
		assert_ok!(Tokens::mint_into(USDT, &CHARLIE, 6000_u128 * unit));
		// exchange USDT for ETH
		let dy = <DexRouter as AmmTrait>::exchange(
			&CHARLIE,
			currency_pair.swap(),
			currency_pair.swap(),
			3000_u128 * unit,
			980000000000_u128,
			false,
		);
		assert_ok!(dy);
		let dy = dy.unwrap();
		let expected_value = unit;
		assert_ok!(acceptable_computation_error(dy, expected_value, precision, epsilon));
		// exchange USDT for ETH but expect high value
		assert_noop!(
			<DexRouter as AmmTrait>::exchange(
				&CHARLIE,
				currency_pair.swap(),
				currency_pair.swap(),
				3000_u128 * unit,
				1100000007962_u128,
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
		let dy = <DexRouter as AmmTrait>::buy(
			&CHARLIE,
			currency_pair,
			currency_pair.base, /* will be ignored */
			3000_u128 * unit,
			0_u128,
			false,
		);
		assert_ok!(dy);
		let dy = dy.unwrap();
		let expected_value = 3000 * unit;
		let precision = 100;
		let epsilon = 1;
		assert_ok!(acceptable_computation_error(dy, expected_value, precision, epsilon));
		assert_ok!(Tokens::mint_into(USDT, &CHARLIE, 6100_u128 * unit));
		// buy 1 ETH
		let dy = <DexRouter as AmmTrait>::buy(
			&CHARLIE,
			currency_pair.swap(),
			currency_pair.base, /* will be ignored */
			1_u128 * unit,
			980000000000_u128,
			false,
		);
		assert_ok!(dy);
		let dy = dy.unwrap();
		let expected_value = 1_u128 * unit;
		let precision = 100;
		let epsilon = 1;
		assert_ok!(acceptable_computation_error(dy, expected_value, precision, epsilon));
		// buy 1 ETH but expect 1.0005
		assert_noop!(
			<DexRouter as AmmTrait>::buy(
				&CHARLIE,
				currency_pair.swap(),
				currency_pair.base, /* will be ignored */
				1_u128 * unit,
				1_000_500_000_000_u128,
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
				eth_amount,
				usdt_amount,
				0_u128,
				false
			),
			Error::<Test>::UnsupportedOperation
		);
		assert_noop!(
			<DexRouter as AmmTrait>::remove_liquidity(&EVE, currency_pair, unit, 0_u128, 0_u128,),
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
		let dy = <DexRouter as AmmTrait>::buy(
			&CHARLIE,
			currency_pair,
			currency_pair.base, /* will be ignored */
			3000_u128 * unit,
			0_u128,
			false,
		);
		assert_ok!(dy);
		let dy = dy.unwrap();
		let expected_value = 3000 * unit;
		let precision = 100;
		let epsilon = 1;
		assert_ok!(acceptable_computation_error(dy, expected_value, precision, epsilon));
		// exchange ETH for USDT
		let dy = <DexRouter as AmmTrait>::exchange(
			&CHARLIE,
			currency_pair,
			currency_pair,
			1_u128 * unit,
			2998_000_000_000_00_u128,
			false,
		);
		assert_ok!(dy);
		let dy = dy.unwrap();
		let expected_value = 3000 * unit;
		let precision = 100;
		let epsilon = 1;
		assert_ok!(acceptable_computation_error(dy, expected_value, precision, epsilon));

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
			currency_pair.swap(),
			eth_amount,
			usdc_amount,
			0_u128,
			false
		));
		let lp_amount = Tokens::balance(lp_token, &EVE);
		// min_base_amount, min_quote_amount should match currency_pair's base quote asset
		assert_ok!(DexRouter::remove_liquidity(
			Origin::signed(EVE),
			currency_pair,
			lp_amount,
			0_u128,
			0_u128
		));
		let bob_eth_amount = Tokens::balance(ETH, &EVE);
		let bob_usdc_amount = Tokens::balance(USDC, &EVE);
		assert_ok!(acceptable_computation_error(eth_amount, bob_eth_amount, precision, epsilon));
		assert_ok!(acceptable_computation_error(usdc_amount, bob_usdc_amount, precision, epsilon));
	});
}
