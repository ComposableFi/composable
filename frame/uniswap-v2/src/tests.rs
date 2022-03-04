use crate::mock::*;
use composable_tests_helpers::test::helper::{
	acceptable_computation_error, default_acceptable_computation_error,
};
use composable_traits::{defi::CurrencyPair, dex::Amm};
use frame_support::{
	assert_err, assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use sp_runtime::{Permill, TokenError};

fn create_pool(
	base_asset: AssetId,
	quote_asset: AssetId,
	base_amount: Balance,
	quote_amount: Balance,
	lp_fee: Permill,
	protocol_fee: Permill,
) -> PoolId {
	let pool_id = Uni::do_create_pool(
		&ALICE,
		CurrencyPair::new(base_asset, quote_asset),
		lp_fee,
		protocol_fee,
	)
	.expect("impossible; qed;");
	// Mint the tokens
	assert_ok!(Tokens::mint_into(base_asset, &ALICE, base_amount));
	assert_ok!(Tokens::mint_into(quote_asset, &ALICE, quote_amount));

	// Add the liquidity
	assert_ok!(<Uni as Amm>::add_liquidity(&ALICE, pool_id, base_amount, quote_amount, 0, false));
	pool_id
}

#[test]
fn test() {
	new_test_ext().execute_with(|| {
		let pool_id = Uni::do_create_pool(
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

		let btc_price = 45_000;

		let nb_of_btc = 100;

		// 100 BTC/4.5M USDT
		let initial_btc = nb_of_btc * unit;
		let initial_usdt = nb_of_btc * btc_price * unit;

		// Mint the tokens
		assert_ok!(Tokens::mint_into(BTC, &ALICE, initial_btc));
		assert_ok!(Tokens::mint_into(USDT, &ALICE, initial_usdt));

		let initial_user_invariant = current_product(ALICE);

		// Add the liquidity
		assert_ok!(<Uni as Amm>::add_liquidity(
			&ALICE,
			pool_id,
			initial_btc,
			initial_usdt,
			0,
			false
		));

		// 1 unit of btc = 45k + some unit of usdt
		let ratio = <Uni as Amm>::get_exchange_value(pool_id, BTC, unit).expect("impossible; qed;");
		assert!(ratio > (initial_usdt / initial_btc) * unit);

		let initial_pool_invariant = current_pool_product();

		assert_eq!(initial_user_invariant, initial_pool_invariant);

		// swap a btc
		let swap_btc = unit;
		assert_ok!(Tokens::mint_into(BTC, &BOB, swap_btc));

		<Uni as Amm>::sell(&BOB, pool_id, BTC, swap_btc, false).expect("impossible; qed;");

		let new_pool_invariant = current_pool_product();
		assert_ok!(default_acceptable_computation_error(
			initial_pool_invariant,
			new_pool_invariant
		));

		<Uni as Amm>::buy(&BOB, pool_id, BTC, swap_btc, false).expect("impossible; qed;");

		let precision = 100;
		let epsilon = 5;
		let bob_btc = Tokens::balance(BTC, &BOB);
		assert_ok!(acceptable_computation_error(bob_btc, swap_btc, precision, epsilon));

		let new_pool_invariant = current_pool_product();
		assert_ok!(default_acceptable_computation_error(
			initial_pool_invariant,
			new_pool_invariant
		));

		let lp = Tokens::balance(pool.lp_token, &ALICE);
		assert_ok!(<Uni as Amm>::remove_liquidity(&ALICE, pool_id, lp, 0, 0));

		// Alice should get back a different amount of tokens.
		let alice_btc = Tokens::balance(BTC, &ALICE);
		let alice_usdt = Tokens::balance(USDT, &ALICE);
		assert_ok!(default_acceptable_computation_error(alice_btc, initial_btc));
		assert_ok!(default_acceptable_computation_error(alice_usdt, initial_usdt));
	});
}

//- test lp mint/burn
#[test]
fn add_remove_lp() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let initial_btc = 1_00_u128 * unit;
		let btc_price = 45_000_u128;
		let initial_usdt = initial_btc * btc_price;
		let pool_id =
			create_pool(BTC, USDT, initial_btc, initial_usdt, Permill::zero(), Permill::zero());
		let pool = Uni::pools(pool_id).expect("impossible; qed;");
		let bob_btc = 10 * unit;
		let bob_usdt = bob_btc * btc_price;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(BTC, &BOB, bob_btc));
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));

		let lp = Tokens::balance(pool.lp_token, &BOB);
		assert_eq!(lp, 0_u128);
		// Add the liquidity
		assert_ok!(<Uni as Amm>::add_liquidity(&BOB, pool_id, bob_btc, bob_usdt, 0, false));
		let lp = Tokens::balance(pool.lp_token, &BOB);
		// must have received some lp tokens
		assert!(lp > 0_u128);
		assert_ok!(<Uni as Amm>::remove_liquidity(&BOB, pool_id, lp, 0, 0));
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
		let initial_btc = 1_00_u128 * unit;
		let btc_price = 45_000_u128;
		let initial_usdt = initial_btc * btc_price;
		let pool_id =
			create_pool(BTC, USDT, initial_btc, initial_usdt, Permill::zero(), Permill::zero());
		let pool = Uni::pools(pool_id).expect("impossible; qed;");
		let bob_btc = 10 * unit;
		let bob_usdt = bob_btc * btc_price;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(BTC, &BOB, bob_btc));
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));

		// Add the liquidity
		assert_ok!(<Uni as Amm>::add_liquidity(&BOB, pool_id, bob_btc, bob_usdt, 0, false));
		let lp = Tokens::balance(pool.lp_token, &BOB);
		assert_err!(
			<Uni as Amm>::remove_liquidity(&BOB, pool_id, lp + 1, 0, 0),
			TokenError::NoFunds
		);
		let min_expected_btc = (bob_btc + 1) * unit;
		let min_expected_usdt = (bob_usdt + 1) * unit;
		assert_err!(
			<Uni as Amm>::remove_liquidity(&BOB, pool_id, lp, min_expected_btc, min_expected_usdt),
			crate::Error::<Test>::CannotRespectMinimumRequested
		);
	});
}

//
// - test exchange failure
#[test]
fn exchange_failure() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let initial_btc = 1_00_u128 * unit;
		let btc_price = 45_000_u128;
		let initial_usdt = initial_btc * btc_price;
		let pool_id =
			create_pool(BTC, USDT, initial_btc, initial_usdt, Permill::zero(), Permill::zero());
		let bob_btc = 10 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(BTC, &BOB, bob_btc));

		let exchange_btc = 100_u128 * unit;
		assert_err!(
			<Uni as Amm>::exchange(
				&BOB,
				pool_id,
				CurrencyPair::new(USDT, BTC),
				exchange_btc,
				0,
				false
			),
			orml_tokens::Error::<Test>::BalanceTooLow
		);
		let exchange_value = 10 * unit;
		let expected_value = exchange_value * btc_price + 1;
		assert_err!(
			<Uni as Amm>::exchange(
				&BOB,
				pool_id,
				CurrencyPair::new(USDT, BTC),
				exchange_value,
				expected_value,
				false
			),
			crate::Error::<Test>::CannotRespectMinimumRequested
		);
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
		let initial_btc = 1_00_u128 * unit;
		let btc_price = 45_000_u128;
		let initial_usdt = initial_btc * btc_price;
		let pool_id =
			create_pool(BTC, USDT, initial_btc, initial_usdt, Permill::zero(), Permill::zero());
		let bob_btc = 99_u128 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(BTC, &BOB, bob_btc));

		assert_ok!(<Uni as Amm>::sell(&BOB, pool_id, BTC, bob_btc, false));
		let usdt_balance = Tokens::balance(USDT, &BOB);
		let idea_usdt_balance = bob_btc * btc_price;
		assert!((idea_usdt_balance - usdt_balance) > 5_u128);
	});
}

//
// - test protocol_fee and owner_fee
#[test]
fn fees() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let initial_btc = 1_00_u128 * unit;
		let btc_price = 45_000_u128;
		let initial_usdt = initial_btc * btc_price;
		let fee = Permill::from_float(0.05);
		let protocol_fee = Permill::from_float(0.01);
		let total_fee = fee + protocol_fee;
		let pool_id = create_pool(BTC, USDT, initial_btc, initial_usdt, fee, protocol_fee);
		let bob_usdt = 45_000_u128 * unit;
		let quote_usdt = bob_usdt - total_fee.mul_floor(bob_usdt);
		let expected_btc_value =
			<Uni as Amm>::get_exchange_value(pool_id, USDT, quote_usdt).expect("impossible, qed.");
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));

		assert_ok!(<Uni as Amm>::sell(&BOB, pool_id, USDT, bob_usdt, false));
		let btc_balance = Tokens::balance(BTC, &BOB);
		assert_ok!(default_acceptable_computation_error(expected_btc_value, btc_balance));
	});
}
