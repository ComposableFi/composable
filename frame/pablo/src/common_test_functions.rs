use crate::{
	mock::{Pablo, *},
	PoolConfiguration::{ConstantProduct, StableSwap},
	PoolInitConfiguration,
};
use frame_support::{
	assert_ok,
	traits::fungibles::{Inspect, Mutate},
};

/// `expected_lp_check` takes base_amount, quote_amount and lp_tokens in order and returns
/// true if lp_tokens are expected for given base_amount, quote_amount.
pub fn common_add_remove_lp(
	init_config: PoolInitConfiguration<AssetId>,
	init_base_amount: Balance,
	init_quote_amount: Balance,
	base_amount: Balance,
	quote_amount: Balance,
	expected_lp_check: impl Fn(Balance, Balance, Balance) -> bool,
) {
	let pool_id = Pablo::do_create_pool(&ALICE, init_config.clone()).expect("pool creation failed");
	let pair = match init_config {
		PoolInitConfiguration::StableSwap { pair, .. } => pair,
		PoolInitConfiguration::ConstantProduct { pair, .. } => pair,
	};
	// Mint the tokens
	assert_ok!(Tokens::mint_into(pair.base, &ALICE, init_base_amount));
	assert_ok!(Tokens::mint_into(pair.quote, &ALICE, init_quote_amount));

	// Add the liquidity
	assert_ok!(Pablo::add_liquidity(
		Origin::signed(ALICE),
		pool_id,
		init_base_amount,
		init_quote_amount,
		0,
		false
	));

	let pool = Pablo::pools(pool_id).expect("pool not found");
	let lp_token = match pool {
		StableSwap(pool) => pool.lp_token,
		ConstantProduct(pool) => pool.lp_token,
	};
	// Mint the tokens
	assert_ok!(Tokens::mint_into(pair.base, &BOB, base_amount));
	assert_ok!(Tokens::mint_into(pair.quote, &BOB, quote_amount));

	let lp = Tokens::balance(lp_token, &BOB);
	assert_eq!(lp, 0_u128);
	// Add the liquidity
	assert_ok!(Pablo::add_liquidity(
		Origin::signed(BOB),
		pool_id,
		base_amount,
		quote_amount,
		0,
		false
	));
	let lp = Tokens::balance(lp_token, &BOB);
	assert!(expected_lp_check(base_amount, quote_amount, lp));
	assert_ok!(Pablo::remove_liquidity(Origin::signed(BOB), pool_id, lp, 0, 0));
	let lp = Tokens::balance(lp_token, &BOB);
	// all lp tokens must have been burnt
	assert_eq!(lp, 0_u128);
}
