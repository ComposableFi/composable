use crate::mock::*;
use composable_tests_helpers::test::helper::{
	acceptable_computation_error, default_acceptable_computation_error,
};
use composable_traits::{defi::CurrencyPair, dex::CurveAmm};
use frame_support::{
	assert_ok,
	traits::fungibles::{Inspect, Mutate},
};

use crate::mock::StableSwap;

use sp_runtime::Permill;

// TODO
/*
- test lp mint/burn
- test error if trying to remove > lp than we have
- test high slippage scenario
- test lp fees
- test admin fees
*/

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
