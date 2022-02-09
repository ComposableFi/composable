use crate::mock::*;
use composable_tests_helpers::test::helper::{
	acceptable_computation_error, default_acceptable_computation_error,
};
use composable_traits::{defi::CurrencyPair, dex::CurveAmm as ConstantProductAmmTrait};
use frame_support::{
	assert_err_with_weight, assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
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
		let pool_id = Uni::create_pool(
			&ALICE,
			CurrencyPair::new(BTC, USDT),
			Permill::zero(),
			Permill::zero(),
		)
		.expect("impossible; qed;");

		let pool = Uni::pools(pool_id).expect("impossible; qed;");

		let current_product = |a| {
			let balance_btc = Tokens::balance(BTC, &a);
			let balance_usdt = Tokens::balance(USDT, &a);
			balance_btc * balance_usdt
		};
		let current_pool_product = || current_product(Uni::account_id(&pool_id));

		let unit = 1_000_000_000_000;

    let nb_of_btc = 100;

		// 100 BTC/45M USDT
		let initial_btc = nb_of_btc * unit;
		let initial_usdt = nb_of_btc * 45_000 * unit;

		// Mint the tokens
		assert_ok!(Tokens::mint_into(BTC, &ALICE, initial_btc));
		assert_ok!(Tokens::mint_into(USDT, &ALICE, initial_usdt));

		let initial_user_invariant = current_product(ALICE);

		// Add the liquidity
		assert_ok!(Uni::add_liquidity(&ALICE, pool_id, initial_btc, initial_usdt, 0, false));

    // 1 unit of btc = 45k unit of usdt
		let ratio = Uni::get_exchange_value(pool_id, BTC, unit).expect("impossible; qed;");
    assert_eq!(ratio, (initial_usdt / initial_btc) * unit);

		let initial_pool_invariant = current_pool_product();

		assert_eq!(initial_user_invariant, initial_pool_invariant);

		// swap half a btc
		let swap_btc = unit;
		assert_ok!(Tokens::mint_into(BTC, &BOB, swap_btc));

		Uni::sell(&BOB, pool_id, swap_btc, false).expect("impossible; qed;");

		let new_pool_invariant = current_pool_product();
		assert_ok!(default_acceptable_computation_error(
			initial_pool_invariant,
			new_pool_invariant
		));

		Uni::buy(&BOB, pool_id, swap_btc, false).expect("impossible; qed;");

		let precision = 100;
		let bob_btc = Tokens::balance(BTC, &BOB);
		assert_ok!(acceptable_computation_error(bob_btc, swap_btc, precision));

		let new_pool_invariant = current_pool_product();
		assert_ok!(default_acceptable_computation_error(
			initial_pool_invariant,
			new_pool_invariant
		));

		let lp = Tokens::balance(pool.lp_token, &ALICE);
		assert_ok!(Uni::remove_liquidity(&ALICE, pool_id, lp, 0, 0));

		// Alice should get back a different amount of tokens.
		let alice_btc = Tokens::balance(BTC, &ALICE);
		let alice_usdt = Tokens::balance(USDT, &ALICE);
		assert_ok!(default_acceptable_computation_error(alice_btc, initial_btc));
		assert_ok!(default_acceptable_computation_error(alice_usdt, initial_usdt));
	});
}
