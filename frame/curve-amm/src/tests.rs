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
			100_u16,
			Permill::zero(),
			Permill::zero(),
		)
		.expect("impossible; qed;");

		let pool = StableSwap::pools(pool_id).expect("impossible; qed;");

		let unit = 1_000_000_000_000_u128;
		let usdc_price = 1 * unit;

		let nb_of_usdc = 1_000_000_000;
		let usdt_price = 1 * unit;

		let nb_of_usdt = 1_000_000_000;

		// 10^9 USDC/10^9 USDT
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

		let precision = 100;
		// 1 unit of usdc == 1 unit of usdt
		let ratio = <StableSwap as CurveAmm>::get_exchange_value(pool_id, USDC, unit)
			.expect("impossible; qed;");
		assert_ok!(acceptable_computation_error(ratio, unit, precision));

		let swap_usdc = 100_u128 * unit;
		assert_ok!(Tokens::mint_into(USDC, &BOB, swap_usdc));

		<StableSwap as CurveAmm>::sell(&BOB, pool_id, USDC, swap_usdc, false)
			.expect("impossible; qed;");

		<StableSwap as CurveAmm>::buy(&BOB, pool_id, USDC, swap_usdc, false)
			.expect("impossible; qed;");

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

//- test lp mint/burn
#[test]
fn add_remove_lp() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let initial_usdt = 1_000_000_000_000_u128 * unit;
		let initial_usdc = 1_000_000_000_000_u128 * unit;
		let pool_id = create_pool(
			USDC,
			USDT,
			initial_usdc,
			initial_usdt,
			100_u16,
			Permill::zero(),
			Permill::zero(),
		);
		let pool = StableSwap::pools(pool_id).expect("impossible; qed;");
		let bob_usdc = 1000 * unit;
		let bob_usdt = 1000 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDC, &BOB, bob_usdc));
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));

		let lp = Tokens::balance(pool.lp_token, &BOB);
		assert_eq!(lp, 0_u128);
		// Add the liquidity
		assert_ok!(<StableSwap as CurveAmm>::add_liquidity(
			&BOB, pool_id, bob_usdc, bob_usdt, 0, false
		));
		let lp = Tokens::balance(pool.lp_token, &BOB);
		// must have received some lp tokens
		assert!(lp > 0_u128);
		assert_ok!(<StableSwap as CurveAmm>::remove_liquidity(&BOB, pool_id, lp, 0, 0));
		let lp = Tokens::balance(pool.lp_token, &BOB);
		// all lp tokens must have been burnt
		assert_eq!(lp, 0_u128);
	});
}

// 
// - test error if trying to remove > lp than we have
#[test]
fn remove_lp_failure() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let initial_usdt = 1_000_000_000_000_u128 * unit;
		let initial_usdc = 1_000_000_000_000_u128 * unit;
		let pool_id = create_pool(
			USDC,
			USDT,
			initial_usdc,
			initial_usdt,
			100_u16,
			Permill::zero(),
			Permill::zero(),
		);
		let pool = StableSwap::pools(pool_id).expect("impossible; qed;");
		let bob_usdc = 1000 * unit;
		let bob_usdt = 1000 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDC, &BOB, bob_usdc));
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));

		// Add the liquidity
		assert_ok!(<StableSwap as CurveAmm>::add_liquidity(
			&BOB, pool_id, bob_usdc, bob_usdt, 0, false
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
		let precision = 100;
		let unit = 1_000_000_000_000_u128;
		let initial_usdt = 1_000_000_000_000_u128 * unit;
		let initial_usdc = 1_000_000_000_000_u128 * unit;
		let lp_fee = Permill::from_float(0.05);
		let pool_id =
			create_pool(USDC, USDT, initial_usdc, initial_usdt, 100_u16, lp_fee, Permill::zero());
		let bob_usdt = 1000 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));

		assert_ok!(<StableSwap as CurveAmm>::sell(&BOB, pool_id, USDT, bob_usdt, false));
		let usdc_balance = Tokens::balance(USDC, &BOB);
		// received usdc should bob_usdt - lp_fee
		assert_ok!(acceptable_computation_error(
			usdc_balance,
			bob_usdt - lp_fee.mul_ceil(bob_usdt),
			precision
		));
	});
}

// 
// - test protocol fees
#[test]
fn protocol_fee() {
	new_test_ext().execute_with(|| {
		let precision = 100;
		let unit = 1_000_000_000_000_u128;
		let initial_usdt = 1_000_000_000_000_u128 * unit;
		let initial_usdc = 1_000_000_000_000_u128 * unit;
		let lp_fee = Permill::from_float(0.05);
		let protocol_fee = Permill::from_float(0.01); // 10% of lp fees goes to pool owner
		let pool_id =
			create_pool(USDC, USDT, initial_usdc, initial_usdt, 100_u16, lp_fee, protocol_fee);
		let bob_usdt = 1000 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));
		assert_ok!(<StableSwap as CurveAmm>::sell(&BOB, pool_id, USDT, bob_usdt, false));
		let usdc_balance = Tokens::balance(USDC, &BOB);
		// received usdc should bob_usdt - lp_fee
		assert_ok!(acceptable_computation_error(
			usdc_balance,
			bob_usdt - lp_fee.mul_floor(bob_usdt),
			precision
		));
		// from lp_fee 1 % (as per protocol_fee) goes to pool owner (ALICE)
		let alice_usdc_bal = Tokens::balance(USDC, &ALICE);
		assert_ok!(acceptable_computation_error(
			alice_usdc_bal,
			protocol_fee.mul_floor(lp_fee.mul_floor(bob_usdt)),
			precision
		));
	});
}

// 
// - test high slippage scenario
// trying to exchange a large value, will result in high_slippage scenario
// there should be substential difference between expected exchange value and received amount.
#[test]
fn high_slippage() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let initial_usdt = 1_000_000_000_000_u128 * unit;
		let initial_usdc = 1_000_000_000_000_u128 * unit;
		let pool_id = create_pool(
			USDC,
			USDT,
			initial_usdc,
			initial_usdt,
			100_u16,
			Permill::zero(),
			Permill::zero(),
		);
		let pool = StableSwap::pools(pool_id).expect("impossible; qed;");
		let bob_usdt = 1_000_000_000_00_u128 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));

		assert_ok!(<StableSwap as CurveAmm>::sell(&BOB, pool_id, USDT, bob_usdt, false));
		let usdc_balance = Tokens::balance(USDC, &BOB);
		assert!((bob_usdt - usdc_balance) > 5_u128);
	});
}
