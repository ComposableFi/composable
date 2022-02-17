use crate::mock::*;
use composable_tests_helpers::test::helper::{
	acceptable_computation_error, default_acceptable_computation_error,
};
use composable_traits::{defi::CurrencyPair, dex::CurveAmm};
use frame_support::{
	assert_err, assert_ok,
	traits::fungibles::{Inspect, Mutate},
};

use crate::mock::StableSwap;

use sp_runtime::{Permill, TokenError};

// TODO
/*
- test lp mint/burn
- test high slippage scenario
*/

fn create_pool(
	base_asset: AssetId,
	quote_asset: AssetId,
	base_amount: Balance,
	quote_amount: Balance,
	amplification_factor: u16,
	lp_fee: Permill,
	protocol_fee: Permill,
) -> PoolId {
	let pool_id = StableSwap::do_create_pool(
		&ALICE,
		CurrencyPair::new(base_asset, quote_asset),
		amplification_factor,
		lp_fee,
		protocol_fee,
	)
	.expect("impossible; qed;");
	// Mint the tokens
	assert_ok!(Tokens::mint_into(base_asset, &ALICE, base_amount));
	assert_ok!(Tokens::mint_into(quote_asset, &ALICE, quote_amount));

	// Add the liquidity
	assert_ok!(<StableSwap as CurveAmm>::add_liquidity(
		&ALICE,
		pool_id,
		base_amount,
		quote_amount,
		0,
		false
	));
	pool_id
}

#[test]
fn test() {
	new_test_ext().execute_with(|| {
		let pool_id = StableSwap::do_create_pool(
			&ALICE,
			CurrencyPair::new(USDC, USDT),
			10000_u16,
			Permill::zero(),
			Permill::zero(),
		)
		.expect("impossible; qed;");

		let pool = StableSwap::pools(pool_id).expect("impossible; qed;");

		let usdc_price = 1;

		let nb_of_usdc = 1_000_000;
		let usdt_price = 1;

		let nb_of_usdt = 1_000_000;

		// 1M USDC/1M USDT
		let initial_usdc = nb_of_usdc * usdc_price;
		let initial_usdt = nb_of_usdt * usdt_price;

		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDC, &ALICE, initial_usdc));
		assert_ok!(Tokens::mint_into(USDT, &ALICE, initial_usdt));

		// Add the liquidity
		assert_ok!(<StableSwap as CurveAmm>::add_liquidity(
			&ALICE,
			pool_id,
			initial_usdc,
			initial_usdt,
			0,
			false
		));

		// 1 unit of usdc == 1 unit of usdt
		let ratio = <StableSwap as CurveAmm>::get_exchange_value(pool_id, USDC, 1_u128)
			.expect("impossible; qed;");
		assert_eq!(ratio, 1_u128);

		let swap_usdc = 100_u128;
		assert_ok!(Tokens::mint_into(USDC, &BOB, swap_usdc));

		<StableSwap as CurveAmm>::sell(&BOB, pool_id, USDC, swap_usdc, false)
			.expect("impossible; qed;");

		<StableSwap as CurveAmm>::buy(&BOB, pool_id, USDC, swap_usdc, false)
			.expect("impossible; qed;");

		let precision = 100;
		let bob_usdc = Tokens::balance(USDC, &BOB);

		assert_ok!(acceptable_computation_error(bob_usdc.into(), swap_usdc.into(), precision));

		let lp = Tokens::balance(pool.lp_token, &ALICE);
		assert_ok!(<StableSwap as CurveAmm>::remove_liquidity(&ALICE, pool_id, lp, 0, 0));

		// Alice should get back a different amount of tokens.
		let alice_usdc = Tokens::balance(USDC, &ALICE);
		let alice_usdt = Tokens::balance(USDT, &ALICE);
		assert_ok!(default_acceptable_computation_error(alice_usdc.into(), initial_usdc.into()));
		assert_ok!(default_acceptable_computation_error(alice_usdt.into(), initial_usdt.into()));
	});
}

// 
// - test error if trying to remove > lp than we have
#[test]
fn remove_lp_failure() {
	new_test_ext().execute_with(|| {
		let pool_id = create_pool(
			USDC,
			USDT,
			1_000_000_00,
			1_000_000_00,
			1000_u16,
			Permill::zero(),
			Permill::zero(),
		);
		let initial_usdt = 1000;
		let initial_usdc = 1000;
		let pool = StableSwap::pools(pool_id).expect("impossible; qed;");
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDC, &BOB, initial_usdc));
		assert_ok!(Tokens::mint_into(USDT, &BOB, initial_usdt));

		// Add the liquidity
		assert_ok!(<StableSwap as CurveAmm>::add_liquidity(
			&BOB,
			pool_id,
			initial_usdc,
			initial_usdt,
			0,
			false
		));
		let lp = Tokens::balance(pool.lp_token, &BOB);
		assert_err!(
			<StableSwap as CurveAmm>::remove_liquidity(&BOB, pool_id, lp + 1, 0, 0),
			TokenError::NoFunds
		);
	});
}

// 
// - test lp fees
#[test]
fn lp_fee() {
	new_test_ext().execute_with(|| {
		let lp_fee = Permill::from_float(0.05);
		let pool_id =
			create_pool(USDC, USDT, 1_000_000_00, 1_000_000_00, 1000_u16, lp_fee, Permill::zero());
		let initial_usdt = 1000;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDT, &BOB, initial_usdt));

		assert_ok!(<StableSwap as CurveAmm>::sell(&BOB, pool_id, USDT, initial_usdt, false));
		let usdc_balance = Tokens::balance(USDC, &BOB);
		// received usdc should initial_usdt - lp_fee
		assert_eq!(usdc_balance, initial_usdt - lp_fee.mul_ceil(initial_usdt));
	});
}

// 
// - test protocol fees
#[test]
fn protocol_fee() {
	new_test_ext().execute_with(|| {
		let lp_fee = Permill::from_float(0.05);
		let protocol_fee = Permill::from_float(0.1);
		let pool_id =
			create_pool(USDC, USDT, 1_000_000_00, 1_000_000_00, 1000_u16, lp_fee, protocol_fee);
		let initial_usdt = 1000;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDT, &BOB, initial_usdt));

		assert_ok!(<StableSwap as CurveAmm>::sell(&BOB, pool_id, USDT, initial_usdt, false));
		let usdc_balance = Tokens::balance(USDC, &BOB);
		// received usdc should initial_usdt - lp_fee
		assert_eq!(usdc_balance, initial_usdt - lp_fee.mul_ceil(initial_usdt));
		// from lp_fee 1 % (as per protocol_fee) goes to pool owner (ALICE)
		let alice_usdc_bal = Tokens::balance(USDC, &ALICE);
		assert_eq!(alice_usdc_bal, 5_u128);
	});
}
