use crate::mock::*;
use composable_support::math::safe::safe_multiply_by_rational;
use composable_tests_helpers::{
	prop_assert_ok,
	test::helper::{acceptable_computation_error, default_acceptable_computation_error},
};
use composable_traits::{defi::CurrencyPair, dex::Amm};
use frame_support::{
	assert_err, assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use proptest::prelude::*;
use sp_runtime::{traits::IntegerSquareRoot, Permill, TokenError};

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
	.expect("pool creation failed");
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
		.expect("pool creation failed");

		let pool = Uni::pools(pool_id).expect("pool not found");

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
		let ratio = <Uni as Amm>::get_exchange_value(pool_id, BTC, unit)
			.expect("get_exchange_value failed");
		assert!(ratio > (initial_usdt / initial_btc) * unit);

		let initial_pool_invariant = current_pool_product();

		assert_eq!(initial_user_invariant, initial_pool_invariant);

		// swap a btc
		let swap_btc = unit;
		assert_ok!(Tokens::mint_into(BTC, &BOB, swap_btc));

		<Uni as Amm>::sell(&BOB, pool_id, BTC, swap_btc, false).expect("sell failed");

		let new_pool_invariant = current_pool_product();
		assert_ok!(default_acceptable_computation_error(
			initial_pool_invariant,
			new_pool_invariant
		));

		<Uni as Amm>::buy(&BOB, pool_id, BTC, swap_btc, false).expect("buy failed");

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
		let pool = Uni::pools(pool_id).expect("pool not found");
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
		let pool = Uni::pools(pool_id).expect("pool not found");
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
		let expected_btc_value = <Uni as Amm>::get_exchange_value(pool_id, USDT, quote_usdt)
			.expect("get_exchange_value failed");
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));

		assert_ok!(<Uni as Amm>::sell(&BOB, pool_id, USDT, bob_usdt, false));
		let btc_balance = Tokens::balance(BTC, &BOB);
		assert_ok!(default_acceptable_computation_error(expected_btc_value, btc_balance));
	});
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(10000))]
	#[test]
	fn buy_sell_proptest(
		btc_value in 1..u32::MAX,
	) {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let initial_btc = 1_000_000_000_000_u128 * unit;
		let btc_price = 45_000_u128;
		let initial_usdt = initial_btc * btc_price;
		let btc_value = btc_value as u128 * unit;
		let usdt_value = btc_value * btc_price;
		let pool_id = create_pool(
			BTC,
			USDT,
			initial_btc,
			initial_usdt,
			Permill::zero(),
			Permill::zero(),
		);
		prop_assert_ok!(Tokens::mint_into(USDT, &BOB, usdt_value));
		prop_assert_ok!(Uni::sell(Origin::signed(BOB), pool_id, USDT, usdt_value, false));
		let bob_btc = Tokens::balance(BTC, &BOB);
		// mint extra BTC equal to slippage so that original amount of USDT can be buy back
		prop_assert_ok!(Tokens::mint_into(BTC, &BOB, btc_value - bob_btc));
		prop_assert_ok!(Uni::buy(Origin::signed(BOB), pool_id, USDT, usdt_value, false));
		let bob_usdt = Tokens::balance(USDT, &BOB);
		let slippage = usdt_value -  bob_usdt;
		let slippage_percentage = slippage as f64 * 100.0_f64 / usdt_value as f64;
		prop_assert!(slippage_percentage < 1.0_f64);
		Ok(())
	})?;
	}

	#[test]
	fn add_remove_liquidity_proptest(
		btc_value in 1..u32::MAX,
	) {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let initial_btc = 1_000_000_000_000_u128 * unit;
		let btc_price = 45_000_u128;
		let initial_usdt = initial_btc * btc_price;
		let btc_value = btc_value as u128 * unit;
		let usdt_value = btc_value * btc_price;
		let pool_id = create_pool(
			BTC,
			USDT,
			initial_btc,
			initial_usdt,
			Permill::zero(),
			Permill::zero(),
		);
		let pool = Uni::pools(pool_id).expect("pool not found");
		prop_assert_ok!(Tokens::mint_into(USDT, &BOB, usdt_value));
		prop_assert_ok!(Tokens::mint_into(BTC, &BOB, btc_value));
		prop_assert_ok!(Uni::add_liquidity(Origin::signed(BOB), pool_id, btc_value, usdt_value, 0, false));
		let term1 = initial_usdt.integer_sqrt_checked().expect("integer_sqrt failed");
		let term2 = initial_btc.integer_sqrt_checked().expect("integer_sqrt failed");
		let expected_lp_tokens = safe_multiply_by_rational(term1, btc_value, term2).expect("multiply_by_rational failed");
		let lp_token = Tokens::balance(pool.lp_token, &BOB);
		prop_assert_ok!(default_acceptable_computation_error(expected_lp_tokens, lp_token));
		prop_assert_ok!(Uni::remove_liquidity(Origin::signed(BOB), pool_id, lp_token, 0, 0));
		let btc_value_redeemed = Tokens::balance(BTC, &BOB);
		let usdt_value_redeemed = Tokens::balance(USDT, &BOB);
		prop_assert_ok!(default_acceptable_computation_error(btc_value_redeemed, btc_value));
		prop_assert_ok!(default_acceptable_computation_error(usdt_value_redeemed, usdt_value));
		Ok(())
	})?;
	}

	#[test]
	fn swap_proptest(
		usdt_value in 1..u32::MAX,
	) {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let initial_btc = 1_000_000_000_000_u128 * unit;
		let btc_price = 45_000_u128;
		let initial_usdt = initial_btc * btc_price;
		let usdt_value = usdt_value as u128 * unit;
		let pool_id = create_pool(
			BTC,
			USDT,
			initial_btc,
			initial_usdt,
			Permill::from_float(0.025),
			Permill::zero(),
		);
		let pool = Uni::pools(pool_id).expect("pool not found");
		prop_assert_ok!(Tokens::mint_into(USDT, &BOB, usdt_value));
		prop_assert_ok!(Uni::swap(Origin::signed(BOB), pool_id, CurrencyPair::new(BTC, USDT), usdt_value, 0, false));
		let usdt_value_after_fee = usdt_value - pool.fee.mul_floor(usdt_value);
		let ratio = initial_btc as f64 / initial_usdt as f64;
		let expected_btc_value = ratio * usdt_value_after_fee as f64;
		let expected_btc_value = expected_btc_value as u128;
		let bob_btc = Tokens::balance(BTC, &BOB);
		prop_assert_ok!(default_acceptable_computation_error(bob_btc, expected_btc_value));
		Ok(())
	})?;
}
}
