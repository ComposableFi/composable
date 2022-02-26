use crate::mock::{StableSwap, *};
use composable_tests_helpers::test::helper::{
	acceptable_computation_error, default_acceptable_computation_error,
};
use composable_traits::{defi::CurrencyPair, dex::CurveAmm};
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
		// mint 1 USDT, after selling 100 USDC we get 99 USDT so to buy 100 USDC we need 100 USDT
		assert_ok!(Tokens::mint_into(USDT, &BOB, unit));

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
		let pool = StableSwap::pools(pool_id).expect("impossible; qed;");
		let bob_usdc = 1000 * unit;
		let bob_usdt = 1000 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDC, &BOB, bob_usdc));
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));

		let lp = Tokens::balance(pool.lp_token, &BOB);
		assert_eq!(lp, 0_u128);
		// Add the liquidity in balanced way
		assert_ok!(<StableSwap as CurveAmm>::add_liquidity(
			&BOB, pool_id, bob_usdc, bob_usdt, 0, false
		));
		let lp = Tokens::balance(pool.lp_token, &BOB);
		// must have received some lp tokens
		assert!(lp > 0_u128);
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
		assert_ok!(<StableSwap as CurveAmm>::add_liquidity(
			&BOB, pool_id, bob_usdc, bob_usdt, 0, false
		));
		// there must fee charged. simple way is to check owner (ALICE) has got
		// tokens as owner fee.
		let alice_usdc = Tokens::balance(USDC, &ALICE);
		let alice_usdt = Tokens::balance(USDT, &ALICE);
		assert!(alice_usdt != 0);
		assert!(alice_usdc != 0);
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
		let bob_usdt = 1_000_000_000_00_u128 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));

		assert_ok!(<StableSwap as CurveAmm>::sell(&BOB, pool_id, USDT, bob_usdt, false));
		let usdc_balance = Tokens::balance(USDC, &BOB);
		assert!((bob_usdt - usdc_balance) > 5_u128);
	});
}

// do exchange when pool does not have enough balance
#[test]
fn low_balance_pool() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let initial_usdt = 1_000_u128 * unit;
		let initial_usdc = 1_000_u128 * unit;
		let pool_id = create_pool(
			USDC,
			USDT,
			initial_usdc,
			initial_usdt,
			100_u16,
			Permill::zero(),
			Permill::zero(),
		);
		let bob_usdt = 1_000_u128 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));
		// pool's USDC will be very low after this sell
		assert_ok!(<StableSwap as CurveAmm>::sell(&BOB, pool_id, USDT, bob_usdt, false));
		assert_err!(
			<StableSwap as CurveAmm>::sell(&BOB, pool_id, USDT, bob_usdt, false),
			orml_tokens::Error::<Test>::BalanceTooLow
		);
	});
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
				(
					quote,
					<StableSwap as CurveAmm>::get_exchange_value(pool_id, USDC, quote * unit)
						.expect("impossible; qed;") as f64 /
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
				let base = <StableSwap as CurveAmm>::get_exchange_value(pool_id, USDC, quote)
					.expect("impossible; qed;");
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
				let _base = <StableSwap as CurveAmm>::sell(&BOB, pool_id, USDC, amount, true)
					.expect("impossible; qed;");
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
				let _base = <StableSwap as CurveAmm>::sell(&BOB, pool_id, USDT, amount, true)
					.expect("impossible; qed;");
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
