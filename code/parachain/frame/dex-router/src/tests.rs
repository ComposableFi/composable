#![allow(clippy::unwrap_used, clippy::disallowed_methods)]

use crate::{mock::*, Error};
use composable_tests_helpers::test::{
	block::next_block,
	helper::{acceptable_computation_error, RuntimeTrait},
};
use composable_traits::{
	defi::CurrencyPair,
	dex::{Amm as AmmTrait, AssetAmount, DexRouter as DexRouterTrait},
};
use frame_support::{
	assert_noop, assert_ok, bounded_btree_map,
	error::BadOrigin,
	traits::fungibles::{Inspect, Mutate},
};
use pallet_pablo::{Error as PabloError, PoolInitConfiguration};
use sp_runtime::{PerThing, Permill};
use std::collections::BTreeMap;

#[derive(Debug)]
struct AssetAmountPair<AssetId, Balance> {
	base: AssetAmount<AssetId, Balance>,
	quote: AssetAmount<AssetId, Balance>,
}

// Create Pablo pool with given amounts added as liquidity to the pool.
fn create_constant_product_amm_pool(assets: AssetAmountPair<u128, u128>, fee: Permill) -> PoolId {
	dbg!(&assets);

	assert_ok!(Tokens::mint_into(assets.base.asset_id, &ALICE, assets.base.amount));
	assert_ok!(Tokens::mint_into(assets.quote.asset_id, &ALICE, assets.quote.amount));
	assert_ok!(Tokens::mint_into(assets.base.asset_id, &BOB, assets.base.amount));
	assert_ok!(Tokens::mint_into(assets.quote.asset_id, &BOB, assets.quote.amount));

	let base_to_quote_ratio =
		Permill::from_rational(assets.base.amount, assets.base.amount + assets.quote.amount);

	dbg!(base_to_quote_ratio);

	let init_config = PoolInitConfiguration::DualAssetConstantProduct {
		owner: ALICE,
		assets_weights: {
			bounded_btree_map! {
				assets.base.asset_id => base_to_quote_ratio,
				assets.quote.asset_id => base_to_quote_ratio.left_from_one(),
			}
		},
		fee,
	};

	dbg!();

	// Create Pablo pool
	let pool_id = Test::assert_extrinsic_event_with(
		Pablo::create(RuntimeOrigin::signed(ALICE), init_config),
		|event| match event {
			pallet_pablo::Event::<Test>::PoolCreated { pool_id, .. } => Some(pool_id),
			_ => None,
		},
	);

	dbg!(&pool_id);

	let assets = BTreeMap::from([
		(assets.base.asset_id, assets.base.amount),
		(assets.quote.asset_id, assets.quote.amount),
	]);

	<Pablo as AmmTrait>::add_liquidity(&ALICE, pool_id, assets.clone(), 0_u128, true).unwrap();
	<Pablo as AmmTrait>::add_liquidity(&BOB, pool_id, assets, 0_u128, true).unwrap();

	// Test::assert_extrinsic_event(
	// 	pallet_pablo::Event::<Test>::LiquidityAdded {
	// 		who: ALICE,
	// 		pool_id,
	// 		minted_lp: ???,
	// 	},
	// );

	pool_id
}

fn create_usdt_usdc_pool() -> PoolId {
	let unit = 1_000_000_000_000_u128;
	// usdc usdt have same price which is 1 USD
	let initial_usdc = 1_000 * unit;
	let initial_usdt = 1_000 * unit;

	let fee = Permill::zero();
	create_constant_product_amm_pool(
		AssetAmountPair {
			base: AssetAmount { asset_id: USDT, amount: initial_usdt },
			quote: AssetAmount { asset_id: USDC, amount: initial_usdc },
		},
		fee,
	)
}

fn create_usdc_usdt_pool() -> PoolId {
	let unit = 1_000_000_000_000_u128;
	// usdc usdt have same price which is 1 USD
	let initial_usdc = 1_000 * unit;
	let initial_usdt = 1_000 * unit;
	let fee = Permill::zero();

	create_constant_product_amm_pool(
		AssetAmountPair {
			base: AssetAmount { asset_id: USDC, amount: initial_usdc },
			quote: AssetAmount { asset_id: USDT, amount: initial_usdt },
		},
		fee,
	)
}

fn create_usdt_dai_pool() -> PoolId {
	let unit = 1_000_000_000_000_u128;
	// usdc usdt have same price which is 1 USD
	let initial_dai = 1_000 * unit;
	let initial_usdt = 1_000 * unit;
	// REVIEW(benluelo): Why is this here?
	// let amp_coeff = 100;
	let fee = Permill::zero();

	create_constant_product_amm_pool(
		AssetAmountPair {
			base: AssetAmount { asset_id: DAI, amount: initial_dai },
			quote: AssetAmount { asset_id: USDT, amount: initial_usdt },
		},
		fee,
	)
}

fn create_usdc_eth_pool() -> PoolId {
	let unit = 1_000_000_000_000_u128;
	let eth_price = 3_u128;
	let eth_balance = 1_000 * unit;
	let usdc_balance = eth_price * eth_balance;
	let fee = Permill::zero();

	create_constant_product_amm_pool(
		AssetAmountPair {
			base: AssetAmount { asset_id: USDC, amount: usdc_balance },
			quote: AssetAmount { asset_id: ETH, amount: eth_balance },
		},
		fee,
	)
}

fn create_dai_eth_pool() -> PoolId {
	let unit = 1_000_000_000_000_u128;
	let eth_price = 3_u128;
	let eth_balance = 1_000 * unit;
	let dai_balance = eth_price * eth_balance;
	let fee = Permill::zero();

	create_constant_product_amm_pool(
		AssetAmountPair {
			base: AssetAmount { asset_id: DAI, amount: dai_balance },
			quote: AssetAmount { asset_id: ETH, amount: eth_balance },
		},
		fee,
	)
}

#[test]
fn get_route_tests() {
	new_test_ext().execute_with(|| {
		next_block::<DexRouter, Test>();

		let currency_pair = CurrencyPair { base: USDT, quote: ETH };
		assert_eq!(DexRouter::get_route(currency_pair), None);

		let dex_route = vec![create_usdc_eth_pool(), create_usdt_usdc_pool()];
		assert_ok!(DexRouter::update_route(
			RuntimeOrigin::root(),
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
		next_block::<DexRouter, Test>();

		let currency_pair = CurrencyPair { base: USDT, quote: ETH };
		assert_eq!(DexRouter::get_route(currency_pair), None);

		let dex_route = vec![create_usdc_eth_pool(), create_usdt_usdc_pool()];
		// only UpdateRouteOrigin can update the route which is set to EnsureRoot in mock.
		assert_noop!(
			DexRouter::update_route(
				RuntimeOrigin::signed(ALICE),
				currency_pair,
				Some(dex_route.try_into().unwrap())
			),
			BadOrigin
		);
	});
}

#[test]
fn halborn_hal11_route_with_cycle() {
	new_test_ext().execute_with(|| {
		next_block::<DexRouter, Test>();

		let currency_pair = CurrencyPair { base: USDT, quote: USDC };
		assert_eq!(DexRouter::get_route(currency_pair), None);

		let dex_route =
			vec![create_usdt_usdc_pool(), create_usdc_usdt_pool(), create_usdt_usdc_pool()];
		assert_noop!(
			DexRouter::update_route(
				RuntimeOrigin::root(),
				currency_pair,
				Some(dex_route.try_into().unwrap())
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
				RuntimeOrigin::root(),
				currency_pair,
				Some(dex_route.try_into().unwrap())
			),
			Error::<Test>::LoopSuspectedInRouteUpdate,
		);

		let dex_route = vec![usdt_usdc_pool, usdc_usdt_pool];
		assert_noop!(
			DexRouter::update_route(
				RuntimeOrigin::root(),
				CurrencyPair::new(USDC, USDC),
				Some(dex_route.try_into().unwrap())
			),
			Error::<Test>::LoopSuspectedInRouteUpdate,
		);
	});
}

#[test]
fn update_route_tests() {
	new_test_ext().execute_with(|| {
		next_block::<DexRouter, Test>();

		let currency_pair = CurrencyPair { base: USDT, quote: ETH };
		assert_eq!(DexRouter::get_route(currency_pair), None);

		// insert
		let dex_route = vec![create_usdc_eth_pool(), create_usdt_usdc_pool()];
		assert_ok!(DexRouter::update_route(
			RuntimeOrigin::root(),
			currency_pair,
			Some(dex_route.clone().try_into().unwrap())
		));
		assert_eq!(DexRouter::get_route(currency_pair), Some((dex_route, false)));

		// update
		let dex_route = vec![create_dai_eth_pool(), create_usdt_dai_pool()];
		assert_ok!(DexRouter::update_route(
			RuntimeOrigin::root(),
			currency_pair,
			Some(dex_route.clone().try_into().unwrap())
		));
		assert_eq!(DexRouter::get_route(currency_pair), Some((dex_route, false)));

		// delete
		assert_ok!(DexRouter::update_route(RuntimeOrigin::root(), currency_pair, None));
		assert_eq!(DexRouter::get_route(currency_pair), None);

		// invalid route, case #1
		let dex_route = vec![
			create_usdc_eth_pool(),
			42, // fake route
			create_usdt_usdc_pool(),
		];
		assert_noop!(
			DexRouter::update_route(
				RuntimeOrigin::root(),
				currency_pair,
				Some(dex_route.try_into().unwrap())
			),
			PabloError::<Test>::PoolNotFound,
		);

		// invalid route, case #2
		let dex_route = vec![create_usdt_usdc_pool(), create_usdc_eth_pool()];
		assert_noop!(
			DexRouter::update_route(
				RuntimeOrigin::root(),
				currency_pair,
				Some(dex_route.try_into().unwrap())
			),
			Error::<Test>::UnexpectedNodeFoundWhileValidation,
		);

		// invalid route, case #3
		let dex_route = vec![create_usdc_eth_pool()];
		assert_noop!(
			DexRouter::update_route(
				RuntimeOrigin::root(),
				currency_pair,
				Some(dex_route.try_into().unwrap())
			),
			Error::<Test>::UnexpectedNodeFoundWhileValidation,
		);

		// route with a single pool.
		let dex_route = vec![create_usdt_usdc_pool()];
		assert_ok!(DexRouter::update_route(
			RuntimeOrigin::root(),
			CurrencyPair::new(USDT, USDC),
			Some(dex_route.try_into().unwrap())
		));
	});
}

#[test]
fn exchange_tests() {
	new_test_ext().execute_with(|| {
		next_block::<DexRouter, Test>();

		let unit = 1_000_000_000_000_u128;
		let currency_pair = CurrencyPair { base: USDT, quote: ETH };
		let dex_route = vec![create_usdc_eth_pool(), create_usdt_usdc_pool()];
		assert_ok!(DexRouter::update_route(
			RuntimeOrigin::root(),
			currency_pair,
			Some(dex_route.try_into().unwrap())
		));
		assert_ok!(Tokens::mint_into(ETH, &CHARLIE, 100_000_u128 * unit));
		// exchange ETH for USDT
		let dy = <DexRouter as AmmTrait>::do_swap(
			&CHARLIE,
			currency_pair,
			AssetAmount::new(currency_pair.quote, unit),
			AssetAmount::new(currency_pair.base, 999_167_374_395),
			false,
		)
		.unwrap();

		dbg!(dy);

		// let expected_value = 3000 * unit;
		let precision = 100;
		let epsilon = 1;
		// assert_ok!(acceptable_computation_error(
		// 	dy.value.amount,
		// 	expected_value,
		// 	precision,
		// 	epsilon
		// ));

		assert_ok!(Tokens::mint_into(USDT, &CHARLIE, 6000_u128 * unit));
		// exchange USDT for ETH
		let dy = <DexRouter as AmmTrait>::do_swap(
			&CHARLIE,
			currency_pair,
			AssetAmount::new(currency_pair.base, 3000_u128 * unit),
			AssetAmount::new(currency_pair.quote, 843_515_477_507_257),
			false,
		)
		.unwrap();

		dbg!(dy);

		// let expected_value = unit;
		// assert_ok!(acceptable_computation_error(
		// 	dy.value.amount,
		// 	expected_value,
		// 	precision,
		// 	epsilon
		// ));

		// exchange USDT for ETH but expect high value
		assert_noop!(
			<DexRouter as AmmTrait>::do_swap(
				&CHARLIE,
				currency_pair.swap(),
				AssetAmount::new(currency_pair.base, 3000_u128 * unit),
				AssetAmount::new(currency_pair.quote, 133_458_940_184_447 + 1),
				false,
			),
			Error::<Test>::CanNotRespectMinAmountRequested
		);
	});
}

#[test]
fn buy_test() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let unit = 1_000_000_000_000_u128;
		let currency_pair = CurrencyPair { base: USDT, quote: ETH };
		let dex_route = vec![create_usdc_eth_pool(), create_usdt_usdc_pool()];
		// USDC/ETH
		// USDT/USDC
		// USDT/ETH
		assert_ok!(DexRouter::update_route(
			RuntimeOrigin::root(),
			currency_pair,
			Some(dex_route.try_into().unwrap())
		));
		assert_ok!(Tokens::mint_into(ETH, &CHARLIE, 200_u128 * unit));

		// buy 3 USDT
		let dy = <DexRouter as AmmTrait>::do_buy(
			&CHARLIE,
			currency_pair,
			currency_pair.quote,
			AssetAmount::new(currency_pair.base, 3_u128 * unit),
			false,
		)
		.unwrap();

		let expected_value = 3 * unit;
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
			AssetAmount::new(currency_pair.quote, unit),
			false,
		);
		assert_ok!(dy);
		let dy = dy.unwrap();
		let expected_value = unit;
		let precision = 100;
		let epsilon = 1;
		assert_ok!(acceptable_computation_error(
			dy.value.amount,
			expected_value,
			precision,
			epsilon
		));
	});
}

#[test]
fn unsupported_operation_test() {
	new_test_ext().execute_with(|| {
		next_block::<DexRouter, Test>();

		let unit = 1_000_000_000_000_u128;
		let currency_pair = CurrencyPair { base: USDT, quote: ETH };
		let dex_route = vec![create_usdc_eth_pool(), create_usdt_usdc_pool()];
		// USDC/ETH
		// USDT/USDC
		// USDT/ETH
		assert_ok!(DexRouter::update_route(
			RuntimeOrigin::root(),
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
		next_block::<DexRouter, Test>();

		let unit = 1_000_000_000_000_u128;
		let currency_pair = CurrencyPair { base: USDC, quote: ETH };
		let dex_route = vec![create_usdc_eth_pool()];
		// USDC/ETH
		assert_ok!(DexRouter::update_route(
			RuntimeOrigin::root(),
			currency_pair,
			Some(dex_route.clone().try_into().unwrap())
		));
		assert_ok!(Tokens::mint_into(ETH, &CHARLIE, 3000_u128 * unit));
		// buy 3000 USDC
		let dy = <DexRouter as AmmTrait>::do_buy(
			&CHARLIE,
			currency_pair,
			ETH,
			AssetAmount::new(USDC, 3000_u128 * unit),
			false,
		)
		.unwrap();

		dbg!(dy);

		assert_eq!(dy.value.amount, 976343977246420);

		// let expected_value = 3000 * unit;
		// let precision = 100;
		// let epsilon = 1;
		// assert_ok!(acceptable_computation_error(
		// 	dy.value.amount,
		// 	expected_value,
		// 	precision,
		// 	epsilon
		// ));

		// exchange ETH for USDT
		let dy = <DexRouter as AmmTrait>::do_swap(
			&CHARLIE,
			currency_pair,
			AssetAmount::new(currency_pair.quote, unit),
			AssetAmount::new(currency_pair.base, 0),
			false,
		)
		.unwrap();

		dbg!(dy);

		assert_eq!(dy.value.amount, 491348479874);

		// let expected_value = 3000 * unit;
		let precision = 100;
		let epsilon = 1;
		// assert_ok!(acceptable_computation_error(
		// 	dy.value.amount,
		// 	expected_value,
		// 	precision,
		// 	epsilon
		// ));

		let pool_id = dex_route[0];
		let lp_token = Pablo::lp_token(pool_id);
		assert_ok!(lp_token);
		let lp_token = lp_token.unwrap();
		// EVE adds liquidity to pool via dex-router
		let eth_amount = 3_u128 * unit;
		let usdc_amount = eth_amount * 3_u128;
		assert_ok!(Tokens::mint_into(ETH, &EVE, eth_amount));
		assert_ok!(Tokens::mint_into(USDC, &EVE, usdc_amount));
		// base, quote amount should match currency_pair's base quote asset
		assert_ok!(DexRouter::add_liquidity(
			RuntimeOrigin::signed(EVE),
			BTreeMap::from([(ETH, eth_amount), (USDC, usdc_amount)]),
			0_u128,
			false
		));
		let lp_amount = Tokens::balance(lp_token, &EVE);
		// min_base_amount, min_quote_amount should match currency_pair's base quote asset
		assert_ok!(DexRouter::remove_liquidity(
			RuntimeOrigin::signed(EVE),
			lp_amount,
			BTreeMap::from([(ETH, 0), (USDC, 0)]),
		));
		let bob_eth_amount = Tokens::balance(ETH, &EVE);
		let bob_usdc_amount = Tokens::balance(USDC, &EVE);
		assert_eq!(2999999999999, bob_eth_amount);
		assert_eq!(8999999999999, bob_usdc_amount);
	});
}
