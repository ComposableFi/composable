use frame_support::assert_ok;
use sp_runtime::{FixedPointNumber, FixedU128, Permill};

use crate::mock::*;
use composable_traits::{
	defi::CurrencyPair,
	dex::{CurveAmm as CurveAmmTrait, DexRouteNode, DexRouter as DexRouterTrait},
};
use frame_support::traits::fungibles::Mutate;

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
	assert_ok!(CurveAmm::add_liquidity(&BOB, pool_id, amounts.clone(), 0_u128));
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
	assert_ok!(ConstantProductAmm::add_liquidity(&ALICE, pool_id, amounts.clone(), 0u128));
	// Add liquidity from BOB's account to pool
	assert_ok!(ConstantProductAmm::add_liquidity(&BOB, pool_id, amounts.clone(), 0u128));
	pool_id
}

#[test]
fn simple_test() {
	new_test_ext().execute_with(|| {
		let assets = vec![MockCurrencyId::USDC, MockCurrencyId::USDT];
		let amp_coeff = FixedU128::saturating_from_rational(1000_i128, 1_i128);
		let amounts = vec![100000, 100000];
		let fee = Permill::zero();
		let admin_fee = Permill::zero();

		let usdc_usdt_pool = create_curve_amm_pool(assets, amounts, amp_coeff, fee, admin_fee);
		let assets = vec![MockCurrencyId::ETH, MockCurrencyId::USDC];
		let amounts = vec![1000, 3000000];
		let eth_usdc_pool = create_constant_product_amm_pool(assets, amounts, fee, admin_fee);
		let currency_pair = CurrencyPair { base: MockCurrencyId::ETH, quote: MockCurrencyId::USDT };
		let dex_route =
			vec![DexRouteNode::Uniswap(eth_usdc_pool), DexRouteNode::Curve(usdc_usdt_pool)];
		assert_ok!(DexRouter::update_route(&ALICE, currency_pair, Some(dex_route)));
		assert_ok!(Tokens::mint_into(MockCurrencyId::ETH, &CHARLIE, 10u128));
		let dy = DexRouter::exchange(&CHARLIE, currency_pair, 1u128);
		assert_ok!(dy);
		let dy = dy.unwrap();
		assert!(3000 >= dy);
		assert!(2995 < dy);
	});
}
