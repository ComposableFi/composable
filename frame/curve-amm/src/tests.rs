use crate::mock::{StableSwap, *};
use composable_tests_helpers::{
	prop_assert_ok,
	test::helper::{acceptable_computation_error, default_acceptable_computation_error},
};
use composable_traits::{defi::CurrencyPair, dex::Amm};
use frame_support::{
	assert_noop, assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use proptest::prelude::*;
use sp_runtime::{Permill, TokenError};

fn create_pool(
	base_asset: AssetId,
	quote_asset: AssetId,
	base_amount: Balance,
	quote_amount: Balance,
	amplification_factor: u16,
	lp_fee: Permill,
	owner_fee: Permill,
) -> PoolId {
	let pool_id = StableSwap::do_create_pool(
		&ALICE,
		CurrencyPair::new(base_asset, quote_asset),
		amplification_factor,
		lp_fee,
		owner_fee,
	)
	.expect("pool creation failed");
	// Mint the tokens
	assert_ok!(Tokens::mint_into(base_asset, &ALICE, base_amount));
	assert_ok!(Tokens::mint_into(quote_asset, &ALICE, quote_amount));

	// Add the liquidity
	assert_ok!(StableSwap::add_liquidity(
		Origin::signed(ALICE),
		pool_id,
		base_amount,
		quote_amount,
		0,
		false
	));
	pool_id
}

#[test]
fn test_amp_zero_failure() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			StableSwap::do_create_pool(
				&ALICE,
				CurrencyPair::new(USDC, USDT),
				0_u16,
				Permill::zero(),
				Permill::zero(),
			),
			crate::Error::<Test>::AmpFactorMustBeGreaterThanZero
		);
	});
}

#[test]
fn test_dex_demo() {
	new_test_ext().execute_with(|| {
		let pool_id = StableSwap::do_create_pool(
			&ALICE,
			CurrencyPair::new(USDC, USDT),
			100_u16,
			Permill::zero(),
			Permill::zero(),
		)
		.expect("pool creation failed");

		let pool = StableSwap::pools(pool_id).expect("pool not found");

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
		assert_ok!(StableSwap::add_liquidity(
			Origin::signed(ALICE),
			pool_id,
			initial_usdc,
			initial_usdt,
			0,
			false
		));

		let precision = 100;
		let epsilon = 1;
		// 1 unit of usdc == 1 unit of usdt
		let ratio = <StableSwap as Amm>::get_exchange_value(pool_id, USDC, unit)
			.expect("get_exchange_value failed");
		assert_ok!(acceptable_computation_error(ratio, unit, precision, epsilon));

		let swap_usdc = 100_u128 * unit;
		assert_ok!(Tokens::mint_into(USDC, &BOB, swap_usdc));
		// mint 1 USDT, after selling 100 USDC we get 99 USDT so to buy 100 USDC we need 100 USDT
		assert_ok!(Tokens::mint_into(USDT, &BOB, unit));

		StableSwap::sell(Origin::signed(BOB), pool_id, USDC, swap_usdc, 0_u128, false)
			.expect("sell failed");

		StableSwap::buy(Origin::signed(BOB), pool_id, USDC, swap_usdc, 0_u128, false)
			.expect("buy failed");

		let bob_usdc = Tokens::balance(USDC, &BOB);

		assert_ok!(acceptable_computation_error(
			bob_usdc.into(),
			swap_usdc.into(),
			precision,
			epsilon
		));
		let lp = Tokens::balance(pool.lp_token, &ALICE);
		assert_ok!(StableSwap::remove_liquidity(Origin::signed(ALICE), pool_id, lp, 0, 0));

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
		let pool = StableSwap::pools(pool_id).expect("pool not found");
		let bob_usdc = 1000 * unit;
		let bob_usdt = 1000 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDC, &BOB, bob_usdc));
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));

		let lp = Tokens::balance(pool.lp_token, &BOB);
		assert_eq!(lp, 0_u128);
		// Add the liquidity
		assert_ok!(StableSwap::add_liquidity(
			Origin::signed(BOB),
			pool_id,
			bob_usdc,
			bob_usdt,
			0,
			false
		));
		let lp = Tokens::balance(pool.lp_token, &BOB);
		// must have received some lp tokens
		assert!(lp == bob_usdt + bob_usdc);
		assert_ok!(StableSwap::remove_liquidity(Origin::signed(BOB), pool_id, lp, 0, 0));
		let lp = Tokens::balance(pool.lp_token, &BOB);
		// all lp tokens must have been burnt
		assert_eq!(lp, 0_u128);
	});
}

//
// - add liquidity which creates imbalance in pool
#[test]
fn add_lp_imbalanced() {
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
			Permill::from_float(0.05), // 5% lp fee.
			Permill::from_float(0.10), // 10% of lp fee goes to owner
		);
		let pool = StableSwap::pools(pool_id).expect("pool not found");
		let bob_usdc = 1000 * unit;
		let bob_usdt = 1000 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDC, &BOB, bob_usdc));
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));

		let lp = Tokens::balance(pool.lp_token, &BOB);
		assert_eq!(lp, 0_u128);
		// Add the liquidity in balanced way
		assert_ok!(StableSwap::add_liquidity(
			Origin::signed(BOB),
			pool_id,
			bob_usdc,
			bob_usdt,
			0,
			false
		));
		let lp = Tokens::balance(pool.lp_token, &BOB);
		// must have received some lp tokens
		assert!(lp == bob_usdt + bob_usdc);
		// there must not be any fee charged. simple way is to check owner (ALICE) has not got any
		// tokens as owner fee.
		let alice_usdc = Tokens::balance(USDC, &ALICE);
		let alice_usdt = Tokens::balance(USDT, &ALICE);
		assert!(alice_usdt == 0);
		assert!(alice_usdc == 0);

		let bob_usdc = 100000 * unit;
		let bob_usdt = 5000 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDC, &BOB, bob_usdc));
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));
		// Add the liquidity in imbalanced way
		assert_ok!(StableSwap::add_liquidity(
			Origin::signed(BOB),
			pool_id,
			bob_usdc,
			bob_usdt,
			0,
			false
		));
		// there must fee charged. simple way is to check owner (ALICE) has got
		// tokens as owner fee.
		let alice_usdc = Tokens::balance(USDC, &ALICE);
		let alice_usdt = Tokens::balance(USDT, &ALICE);
		assert!(alice_usdt != 0);
		assert!(alice_usdc != 0);
	});
}

// test add liquidity with min_mint_amount
#[test]
fn add_lp_with_min_mint_amount_success() {
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
		let pool = StableSwap::pools(pool_id).expect("pool not found");
		let bob_usdc = 1000 * unit;
		let bob_usdt = 1000 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDC, &BOB, bob_usdc));
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));

		let lp = Tokens::balance(pool.lp_token, &BOB);
		assert_eq!(lp, 0_u128);
		let expected_min_value = bob_usdt + bob_usdc;
		// Add the liquidity in balanced way
		assert_ok!(StableSwap::add_liquidity(
			Origin::signed(BOB),
			pool_id,
			bob_usdc,
			bob_usdt,
			expected_min_value,
			false
		));
		let lp = Tokens::balance(pool.lp_token, &BOB);
		// must have received some lp tokens
		assert!(lp == bob_usdc + bob_usdt);

		let bob_usdc = 1000 * unit;
		let bob_usdt = 900 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDC, &BOB, bob_usdc));
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));
		// Add the liquidity in imbalanced way, but have expected_min_value higher
		assert_noop!(
			StableSwap::add_liquidity(
				Origin::signed(BOB),
				pool_id,
				bob_usdc,
				bob_usdt,
				expected_min_value,
				false
			),
			crate::Error::<Test>::CannotRespectMinimumRequested
		);
	});
}

// test add liquidity with min_mint_amount
#[test]
fn add_lp_with_min_mint_amount_fail() {
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

		let bob_usdc = 1000 * unit;
		let bob_usdt = 900 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDC, &BOB, bob_usdc));
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));
		let expected_min_value = bob_usdt + bob_usdc + 1_u128;
		// Add the liquidity in imbalanced way, but have expected_min_value higher
		assert_noop!(
			StableSwap::add_liquidity(
				Origin::signed(BOB),
				pool_id,
				bob_usdc,
				bob_usdt,
				expected_min_value,
				false
			),
			crate::Error::<Test>::CannotRespectMinimumRequested
		);
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
		let pool = StableSwap::pools(pool_id).expect("pool not found");
		let bob_usdc = 1000 * unit;
		let bob_usdt = 1000 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDC, &BOB, bob_usdc));
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));

		// Add the liquidity
		assert_ok!(StableSwap::add_liquidity(
			Origin::signed(BOB),
			pool_id,
			bob_usdc,
			bob_usdt,
			0,
			false
		));
		let lp = Tokens::balance(pool.lp_token, &BOB);
		assert_noop!(
			StableSwap::remove_liquidity(Origin::signed(BOB), pool_id, lp + 1, 0, 0),
			TokenError::NoFunds
		);
		let min_expected_usdt = 1001 * unit;
		let min_expected_usdc = 1001 * unit;
		assert_noop!(
			StableSwap::remove_liquidity(
				Origin::signed(BOB),
				pool_id,
				lp,
				min_expected_usdc,
				min_expected_usdt
			),
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
		let initial_usdt = 1_000_000_u128 * unit;
		let initial_usdc = 1_000_000_u128 * unit;
		let pool_id = create_pool(
			USDC,
			USDT,
			initial_usdc,
			initial_usdt,
			100_u16,
			Permill::zero(),
			Permill::zero(),
		);
		let bob_usdc = 1000 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDC, &BOB, bob_usdc));

		let exchange_usdc = 1001 * unit;
		assert_noop!(
			StableSwap::swap(
				Origin::signed(BOB),
				pool_id,
				CurrencyPair::new(USDT, USDC),
				exchange_usdc,
				0,
				false
			),
			orml_tokens::Error::<Test>::BalanceTooLow
		);
		let exchange_value = 1000 * unit;
		let expected_value = 1001 * unit;
		assert_noop!(
			StableSwap::swap(
				Origin::signed(BOB),
				pool_id,
				CurrencyPair::new(USDT, USDC),
				exchange_value,
				expected_value,
				false
			),
			crate::Error::<Test>::CannotRespectMinimumRequested
		);
	});
}

//
// - test lp fees
#[test]
fn lp_fee() {
	new_test_ext().execute_with(|| {
		let precision = 100;
		let epsilon = 1;
		let unit = 1_000_000_000_000_u128;
		let initial_usdt = 1_000_000_000_000_u128 * unit;
		let initial_usdc = 1_000_000_000_000_u128 * unit;
		let lp_fee = Permill::from_float(0.05);
		let pool_id =
			create_pool(USDC, USDT, initial_usdc, initial_usdt, 100_u16, lp_fee, Permill::zero());
		let bob_usdt = 1000 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));

		assert_ok!(StableSwap::sell(Origin::signed(BOB), pool_id, USDT, bob_usdt, 0_u128, false));
		let usdc_balance = Tokens::balance(USDC, &BOB);
		// received usdc should bob_usdt - lp_fee
		assert_ok!(acceptable_computation_error(
			usdc_balance,
			bob_usdt - lp_fee.mul_ceil(bob_usdt),
			precision,
			epsilon
		));
	});
}

//
// - test protocol fees
#[test]
fn owner_fee() {
	new_test_ext().execute_with(|| {
		let precision = 100;
		let epsilon = 1;
		let unit = 1_000_000_000_000_u128;
		let initial_usdt = 1_000_000_000_000_u128 * unit;
		let initial_usdc = 1_000_000_000_000_u128 * unit;
		let lp_fee = Permill::from_float(0.05);
		let owner_fee = Permill::from_float(0.01); // 10% of lp fees goes to pool owner
		let pool_id =
			create_pool(USDC, USDT, initial_usdc, initial_usdt, 100_u16, lp_fee, owner_fee);
		let bob_usdt = 1000 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));
		assert_ok!(StableSwap::sell(Origin::signed(BOB), pool_id, USDT, bob_usdt, 0_u128, false));
		let usdc_balance = Tokens::balance(USDC, &BOB);
		// received usdc should bob_usdt - lp_fee
		assert_ok!(acceptable_computation_error(
			usdc_balance,
			bob_usdt - lp_fee.mul_floor(bob_usdt),
			precision,
			epsilon
		));
		// from lp_fee 1 % (as per owner_fee) goes to pool owner (ALICE)
		let alice_usdc_bal = Tokens::balance(USDC, &ALICE);
		assert_ok!(acceptable_computation_error(
			alice_usdc_bal,
			owner_fee.mul_floor(lp_fee.mul_floor(bob_usdt)),
			precision,
			epsilon
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
		let bob_usdt = 1_000_000_000_00_u128 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));

		assert_ok!(StableSwap::sell(Origin::signed(BOB), pool_id, USDT, bob_usdt, 0_u128, false));
		let usdc_balance = Tokens::balance(USDC, &BOB);
		assert!((bob_usdt - usdc_balance) > 5_u128);
	});
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(10000))]
	#[test]
	fn add_remove_liquidity_proptest(
		usdc_balance in 1..u32::MAX,
		usdt_balance in 1..u32::MAX,
	) {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let usdt_balance = usdt_balance as u128 * unit;
		let usdc_balance = usdc_balance as u128 * unit;
		let initial_usdt = u64::MAX as u128 * unit;
		let initial_usdc = u64::MAX as u128 * unit;
		let pool_id = create_pool(
			USDC,
			USDT,
			initial_usdc,
			initial_usdt,
			100_u16,
			Permill::zero(),
			Permill::zero(),
		);
		let pool = StableSwap::pools(pool_id).expect("pool not found");
		prop_assert_ok!(Tokens::mint_into(USDT, &BOB, usdt_balance));
		prop_assert_ok!(Tokens::mint_into(USDC, &BOB, usdc_balance));
		prop_assert_ok!(StableSwap::add_liquidity(
			Origin::signed(BOB),
			pool_id,
			usdc_balance,
			usdt_balance,
			0,
			false
		));
		let lp = Tokens::balance(pool.lp_token, &BOB);
		let expected_lp = usdt_balance + usdc_balance;
		prop_assert_ok!(default_acceptable_computation_error(lp, expected_lp));
		prop_assert_ok!(StableSwap::remove_liquidity(
			Origin::signed(BOB),
			pool_id,
			lp,
			0,
			0,
				));
		let bob_usdc = Tokens::balance(USDC, &BOB);
		let bob_usdt = Tokens::balance(USDT, &BOB);
		prop_assert_ok!(default_acceptable_computation_error(usdc_balance + usdt_balance, bob_usdc + bob_usdt));
		Ok(())
	})?;
	}

	#[test]
	fn buy_sell_proptest(
		value in 1..u32::MAX,
	) {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let initial_usdt = u64::MAX as u128 * unit;
		let initial_usdc = u64::MAX as u128 * unit;
		let value = value as u128 * unit;
		let pool_id = create_pool(
			USDC,
			USDT,
			initial_usdc,
			initial_usdt,
			100_u16,
			Permill::zero(),
			Permill::zero(),
		);
		prop_assert_ok!(Tokens::mint_into(USDT, &BOB, value));
		prop_assert_ok!(StableSwap::sell(Origin::signed(BOB), pool_id, USDT, value, 0_u128, false));
		// mint 1 extra USDC so that original amount of USDT can be buy back even with small slippage
		prop_assert_ok!(Tokens::mint_into(USDC, &BOB, unit));
		prop_assert_ok!(StableSwap::buy(Origin::signed(BOB), pool_id, USDT, value, 0_u128, false));
		let bob_usdt = Tokens::balance(USDT, &BOB);
		prop_assert_ok!(default_acceptable_computation_error(bob_usdt, value));
		Ok(())
	})?;
	}

	#[test]
	fn swap_proptest(
		value in 1..u32::MAX,
	) {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let initial_usdt = u64::MAX as u128 * unit;
		let initial_usdc = u64::MAX as u128 * unit;
		let value = value as u128 * unit;
		let pool_id = create_pool(
			USDC,
			USDT,
			initial_usdc,
			initial_usdt,
			100_u16,
			Permill::from_float(0.025),
			Permill::zero(),
		);
		let pool = StableSwap::pools(pool_id).expect("pool not found");
		prop_assert_ok!(Tokens::mint_into(USDT, &BOB, value));
		prop_assert_ok!(StableSwap::swap(Origin::signed(BOB), pool_id, CurrencyPair::new(USDC, USDT), value, 0, false));
		let bob_usdc = Tokens::balance(USDC, &BOB);
		let expected_usdc =  value - pool.fee.mul_floor(value);
		prop_assert_ok!(default_acceptable_computation_error(bob_usdc, expected_usdc));
		Ok(())
	})?;
	}
}

#[cfg(feature = "visualization")]
#[test]
fn get_base_graph() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let initial_usdt = 100000_u128 * unit;
		let initial_usdc = 100000_u128 * unit;
		let pool_id = create_pool(
			USDC,
			SDT,
			initial_usdc,
			initial_usdt,
			100_u16,
			Permill::zero(),
			Permill::zero(),
		);

		let start_quote = 0;
		let end_quote = 120000;
		let points = (start_quote..end_quote)
			.map(|quote| {
				(
					quote,
					<StableSwap as Amm>::get_exchange_value(pool_id, USDC, quote * unit)
						.expect("get_exchange_value not found") as f64 /
						unit as f64,
				)
			})
			.collect::<Vec<_>>();
		let max_amount = points.iter().copied().fold(f64::NAN, |x, (_, y)| f64::max(x, y));

		use plotters::prelude::*;
		let area = BitMapBackend::new("./curve_base.png", (1024, 768)).into_drawing_area();
		area.fill(&WHITE).unwrap();

		let mut chart = ChartBuilder::on(&area)
			.caption("Curve price, pool has 100000 USDC 100000 USDT", ("Arial", 25).into_font())
			.margin(100u32)
			.x_label_area_size(30u32)
			.y_label_area_size(30u32)
			.build_cartesian_2d(start_quote..end_quote, 0f64..max_amount)
			.unwrap();

		chart
			.configure_mesh()
			.y_desc("base amount")
			.x_desc("quote amount")
			.draw()
			.unwrap();
		chart.draw_series(LineSeries::new(points, &RED)).unwrap();
		chart
			.configure_series_labels()
			.background_style(&WHITE.mix(0.8))
			.border_style(&BLACK)
			.draw()
			.unwrap();
	});
}

#[cfg(feature = "visualization")]
#[test]
fn slippage_graph() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let initial_usdt = 1000000_u128 * unit;
		let initial_usdc = 1000000_u128 * unit;
		let pool_id = create_pool(
			USDC,
			USDT,
			initial_usdc,
			initial_usdt,
			100_u16,
			Permill::zero(),
			Permill::zero(),
		);

		let start_quote = 0;
		let end_quote = 120000;
		let points = (start_quote..end_quote)
			.map(|quote| {
				let quote = quote * unit;
				let base = <StableSwap as Amm>::get_exchange_value(pool_id, USDC, quote)
					.expect("get_exchange_value failed");
				let slippage = if base <= quote { quote - base } else { base };
				(quote / unit, slippage as f64 / unit as f64)
			})
			.collect::<Vec<_>>();
		let max_amount = points.iter().copied().fold(f64::NAN, |x, (_, y)| f64::max(x, y));

		use plotters::prelude::*;
		let area = BitMapBackend::new("./curve_slippage.png", (1024, 768)).into_drawing_area();
		area.fill(&WHITE).unwrap();

		let mut chart = ChartBuilder::on(&area)
			.caption("Curve price, pool has 100000 USDC 100000 USDT", ("Arial", 25).into_font())
			.margin(100u32)
			.x_label_area_size(30u32)
			.y_label_area_size(30u32)
			.build_cartesian_2d(start_quote..end_quote, 0f64..max_amount)
			.unwrap();

		chart.configure_mesh().y_desc("slippage").x_desc("quote amount").draw().unwrap();
		chart.draw_series(LineSeries::new(points, &RED)).unwrap();
		chart
			.configure_series_labels()
			.background_style(&WHITE.mix(0.8))
			.border_style(&BLACK)
			.draw()
			.unwrap();
	});
}

#[cfg(feature = "visualization")]
#[test]
fn curve_graph() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let initial_usdt = 5_u128 * unit;
		let initial_usdc = initial_usdt;
		let pool_id = create_pool(
			USDC,
			USDT,
			initial_usdc,
			initial_usdt,
			5_u16,
			Permill::zero(),
			Permill::zero(),
		);
		let window = 15u128;
		let max_base = (initial_usdt + window * unit) as f64 / unit as f64;
		let max_quote = max_base;
		let pool_account = StableSwap::account_id(&pool_id);
		let range: Vec<u128> = (0..window).collect();

		let quote_balance = Tokens::balance(USDT, &pool_account);
		let base_balance = Tokens::balance(USDC, &pool_account);
		let points1 = range
			.iter()
			.map(|_| {
				let amount = unit;
				Tokens::mint_into(USDC, &BOB, amount);
				let _base = <StableSwap as Amm>::sell(&BOB, pool_id, USDC, amount, 0_u128, true)
					.expect("sell failed");
				let pool_sell_asset_balance =
					Tokens::balance(USDC, &pool_account) as f64 / unit as f64;
				let pool_buy_asset_balance =
					Tokens::balance(USDT, &pool_account) as f64 / unit as f64;
				(pool_buy_asset_balance, pool_sell_asset_balance)
			})
			.collect::<Vec<_>>();
		let pool_id = create_pool(
			USDC,
			USDT,
			initial_usdc,
			initial_usdt,
			5_u16,
			Permill::zero(),
			Permill::zero(),
		);
		let pool_account = StableSwap::account_id(&pool_id);
		let points2 = range
			.iter()
			.map(|_| {
				let amount = unit;
				Tokens::mint_into(USDT, &BOB, amount);
				let _base = <StableSwap as Amm>::sell(&BOB, pool_id, USDT, amount, 0_u128, true)
					.expect("sell failed");
				let pool_sell_asset_balance =
					Tokens::balance(USDC, &pool_account) as f64 / unit as f64;
				let pool_buy_asset_balance =
					Tokens::balance(USDT, &pool_account) as f64 / unit as f64;
				(pool_buy_asset_balance, pool_sell_asset_balance)
			})
			.collect::<Vec<_>>();
		let points: Vec<_> = points1.into_iter().rev().chain(points2.into_iter()).collect();
		use plotters::prelude::*;
		let area = BitMapBackend::new("./curve_graph.png", (1024, 768)).into_drawing_area();
		area.fill(&WHITE).unwrap();

		let mut chart = ChartBuilder::on(&area)
			.caption("Curve price, pool has 1000 USDC 1000 USDT", ("Arial", 25).into_font())
			.margin(100u32)
			.x_label_area_size(30u32)
			.y_label_area_size(30u32)
			.build_cartesian_2d(0f64..max_base as f64, 0f64..max_quote as f64)
			.unwrap();

		chart
			.configure_mesh()
			.y_desc("quote amount")
			.x_desc("base amount")
			.draw()
			.unwrap();
		chart.draw_series(LineSeries::new(points, &RED)).unwrap();
		chart
			.configure_series_labels()
			.background_style(&WHITE.mix(0.8))
			.border_style(&BLACK)
			.draw()
			.unwrap();
	});
}
