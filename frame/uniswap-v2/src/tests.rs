use crate::mock::*;
use composable_tests_helpers::test::helper::default_acceptable_computation_error;
use composable_traits::{defi::CurrencyPair, dex::CurveAmm as ConstantProductAmmTrait};
use frame_support::{
	assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use proptest::prelude::*;
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
			CurrencyPair::new(TestAssetId::BTC, TestAssetId::USDT),
			Permill::from_perthousand(1),
			Permill::from_perthousand(1),
		)
		.expect("impossible; qed;");

		let pool = Uni::pools(pool_id).expect("impossible; qed;");

		let current_product = |a| {
			let balance_btc = Tokens::balance(TestAssetId::BTC, &a);
			let balance_usdt = Tokens::balance(TestAssetId::USDT, &a);
			balance_btc * balance_usdt
		};
		let current_pool_product = || current_product(Uni::account_id(&pool_id));

		// 1 BTC/45K USDT
		let initial_btc = 1_000_000_000_000;
		let initial_usdt = 45000_000_000_000_000;

		// Mint the tokens
		assert_ok!(Tokens::mint_into(TestAssetId::BTC, &ALICE, initial_btc));
		assert_ok!(Tokens::mint_into(TestAssetId::USDT, &ALICE, initial_usdt));

		let initial_user_invariant = current_product(ALICE);

		// Add the liquidity
		assert_ok!(Uni::add_liquidity(&ALICE, pool_id, vec![initial_btc, initial_usdt], 0, false,));

		let initial_pool_invariant = current_pool_product();

		assert_eq!(initial_user_invariant, initial_pool_invariant);

		// swap half a btc
		let swap_btc = initial_btc / 2;
		assert_ok!(Tokens::mint_into(TestAssetId::BTC, &BOB, swap_btc));

		let swapped_usdt = Uni::exchange(
			&BOB,
			pool_id,
			CurrencyPair::new(TestAssetId::USDT, TestAssetId::BTC),
			swap_btc,
			0,
			false,
		)
		.expect("impossible; qed;");
		let swapped_btc = Uni::exchange(
			&BOB,
			pool_id,
			CurrencyPair::new(TestAssetId::BTC, TestAssetId::USDT),
			swapped_usdt,
			0,
			false,
		)
		.expect("impossible; qed;");

		let new_pool_invariant = current_pool_product();
		assert_ok!(default_acceptable_computation_error(
			initial_pool_invariant,
			new_pool_invariant
		));

		let lp = Tokens::balance(pool.lp_token, &ALICE);
		assert_ok!(Uni::remove_liquidity(&ALICE, pool_id, lp, vec![0, 0]));

		// Alice should get back a different amount of tokens.
		let alice_btc = Tokens::balance(TestAssetId::BTC, &ALICE);
		let alice_usdt = Tokens::balance(TestAssetId::USDT, &ALICE);
		assert_ok!(default_acceptable_computation_error(alice_btc, initial_btc + swap_btc));
		assert_ok!(default_acceptable_computation_error(alice_usdt, initial_usdt - swapped_usdt));
	});
}
