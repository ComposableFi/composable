#[cfg(test)]
use crate::{
	common_test_functions::*,
	mock,
	mock::{Pablo, *},
	pallet,
	stable_swap::StableSwap as SS,
	Error,
	PoolConfiguration::StableSwap,
	PoolInitConfiguration,
};
use composable_tests_helpers::{
	prop_assert_ok,
	test::helper::{acceptable_computation_error, default_acceptable_computation_error},
};
use composable_traits::{
	defi::CurrencyPair,
	dex::{Amm, FeeConfig},
};
use frame_support::{
	assert_err, assert_noop, assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use proptest::prelude::*;
use sp_runtime::{DispatchError, Permill};

fn create_stable_swap_pool(
	base_asset: AssetId,
	quote_asset: AssetId,
	base_amount: Balance,
	quote_amount: Balance,
	amplification_factor: u16,
	lp_fee: Permill,
	owner_fee: Permill,
) -> PoolId {
	System::set_block_number(1);
	let actual_pool_id = SS::<Test>::do_create_pool(
		&ALICE,
		CurrencyPair::new(base_asset, quote_asset),
		amplification_factor,
		FeeConfig {
			fee_rate: lp_fee,
			owner_fee_rate: owner_fee,
			protocol_fee_rate: Permill::zero(),
		},
	)
	.expect("pool creation failed");

	// Mint the tokens
	assert_ok!(Tokens::mint_into(base_asset, &ALICE, base_amount));
	assert_ok!(Tokens::mint_into(quote_asset, &ALICE, quote_amount));

	// Add the liquidity
	assert_ok!(Pablo::add_liquidity(
		Origin::signed(ALICE),
		actual_pool_id,
		base_amount,
		quote_amount,
		0,
		false
	));
	assert_last_event::<Test, _>(|e| {
		matches!(e.event,
            mock::Event::Pablo(crate::Event::LiquidityAdded { who, pool_id, .. })
            if who == ALICE && pool_id == actual_pool_id)
	});
	actual_pool_id
}

#[test]
fn test_amp_zero_pool_creation_failure() {
	new_test_ext().execute_with(|| {
		let pool_init_config = PoolInitConfiguration::StableSwap {
			owner: ALICE,
			pair: CurrencyPair::new(USDC, USDT),
			amplification_coefficient: 0_u16,
			fee: Permill::zero(),
		};
		assert_noop!(
			Pablo::do_create_pool(pool_init_config),
			crate::Error::<Test>::AmpFactorMustBeGreaterThanZero
		);
	});
}

#[test]
fn test_dex_demo() {
	new_test_ext().execute_with(|| {
		let pool_init_config = PoolInitConfiguration::StableSwap {
			owner: ALICE,
			pair: CurrencyPair::new(USDC, USDT),
			amplification_coefficient: 100_u16,
			fee: Permill::zero(),
		};
		let pool_id = Pablo::do_create_pool(pool_init_config).expect("pool creation failed");
		let pool = Pablo::pools(pool_id).expect("pool not found");
		let pool = match pool {
			StableSwap(pool) => pool,
			_ => panic!("expected stable_swap pool"),
		};

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
		assert_ok!(Pablo::add_liquidity(
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
		let ratio = <Pablo as Amm>::get_exchange_value(pool_id, USDC, unit)
			.expect("get_exchange_value failed");
		assert_ok!(acceptable_computation_error(ratio, unit, precision, epsilon));

		let swap_usdc = 100_u128 * unit;
		assert_ok!(Tokens::mint_into(USDC, &BOB, swap_usdc));
		// mint 1 USDT, after selling 100 USDC we get 99 USDT so to buy 100 USDC we need 100 USDT
		assert_ok!(Tokens::mint_into(USDT, &BOB, unit));

		Pablo::sell(Origin::signed(BOB), pool_id, USDC, swap_usdc, 0_u128, false)
			.expect("sell failed");

		Pablo::buy(Origin::signed(BOB), pool_id, USDC, swap_usdc, 0_u128, false)
			.expect("buy failed");

		let bob_usdc = Tokens::balance(USDC, &BOB);

		assert_ok!(acceptable_computation_error(
			bob_usdc.into(),
			swap_usdc.into(),
			precision,
			epsilon
		));
		let lp = Tokens::balance(pool.lp_token, &ALICE);
		assert_ok!(Pablo::remove_liquidity(Origin::signed(ALICE), pool_id, lp, 0, 0));

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
		let pool_init_config = PoolInitConfiguration::StableSwap {
			owner: ALICE,
			pair: CurrencyPair::new(USDC, USDT),
			amplification_coefficient: 10_u16,
			fee: Permill::zero(),
		};
		let unit = 1_000_000_000_000_u128;
		let initial_usdt = 1_000_000_000_000_u128 * unit;
		let initial_usdc = 1_000_000_000_000_u128 * unit;
		let usdc_amount = 1000 * unit;
		let usdt_amount = 1000 * unit;
		let expected_lp_check = |base_amount: Balance,
		                         quote_amount: Balance,
		                         lp: Balance|
		 -> bool { base_amount + quote_amount == lp };
		common_add_remove_lp(
			pool_init_config,
			initial_usdc,
			initial_usdt,
			usdc_amount,
			usdt_amount,
			expected_lp_check,
		);
	});
}

#[test]
fn test_enable_twap() {
	new_test_ext().execute_with(|| {
		Tokens::mint_into(USDC, &ALICE, 1_000_000_000_000_000).unwrap();
		Tokens::mint_into(BTC, &ALICE, 1_000_000_000_000_000).unwrap();

		Pablo::create(
			Origin::signed(ALICE),
			PoolInitConfiguration::StableSwap {
				amplification_coefficient: 10,
				fee: Permill::from_rational(10_000_u128, 1_000_000),
				owner: ALICE,
				pair: CurrencyPair::new(USDC, BTC),
			},
		)
		.unwrap();

		assert_err!(Pablo::enable_twap(Origin::root(), 0), Error::<Test>::NotEnoughLiquidity);

		Pablo::add_liquidity(Origin::signed(ALICE), 0, 1_000_000_000, 1_000_000_000, 100, true)
			.unwrap();

		Pablo::enable_twap(Origin::root(), 0).unwrap();
	})
}

//
// - add liquidity which creates imbalance in pool
#[test]
fn add_lp_imbalanced() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let initial_usdt = 1_000_000_000_000_u128 * unit;
		let initial_usdc = 1_000_000_000_000_u128 * unit;
		let pool_id = create_stable_swap_pool(
			USDC,
			USDT,
			initial_usdc,
			initial_usdt,
			100_u16,
			Permill::from_float(0.05), // 5% lp fee.
			Permill::from_float(0.10), // 10% of lp fee goes to owner
		);
		let pool = Pablo::pools(pool_id).expect("pool not found");
		let pool = match pool {
			StableSwap(pool) => pool,
			_ => panic!("expected stable_swap pool"),
		};
		let bob_usdc = 1000 * unit;
		let bob_usdt = 1000 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDC, &BOB, bob_usdc));
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));

		let lp = Tokens::balance(pool.lp_token, &BOB);
		assert_eq!(lp, 0_u128);
		// Add the liquidity in balanced way
		assert_ok!(Pablo::add_liquidity(
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
		assert_ok!(Pablo::add_liquidity(
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
		assert_eq!(alice_usdt, 118749999985968);
		assert_eq!(alice_usdc, 118750000014031);
	});
}

// test add liquidity with min_mint_amount
#[test]
fn add_lp_with_min_mint_amount() {
	new_test_ext().execute_with(|| {
		let pool_init_config = PoolInitConfiguration::StableSwap {
			owner: ALICE,
			pair: CurrencyPair::new(USDC, USDT),
			amplification_coefficient: 10_u16,
			fee: Permill::zero(),
		};
		let unit = 1_000_000_000_000_u128;
		let initial_usdt = 1_000_000_000_000_u128 * unit;
		let initial_usdc = 1_000_000_000_000_u128 * unit;
		let expected_lp = |base_amount: Balance,
		                   quote_amount: Balance,
		                   _lp_total_issuance: Balance,
		                   _pool_base_amount: Balance,
		                   _pool_quote_amount: Balance|
		 -> Balance { base_amount + quote_amount };
		let usdc_amount = 1000 * unit;
		let usdt_amount = 1000 * unit;
		common_add_lp_with_min_mint_amount(
			pool_init_config,
			initial_usdc,
			initial_usdt,
			usdc_amount,
			usdt_amount,
			expected_lp,
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
		let pool_init_config = PoolInitConfiguration::StableSwap {
			owner: ALICE,
			pair: CurrencyPair::new(USDC, USDT),
			amplification_coefficient: 10_u16,
			fee: Permill::zero(),
		};
		let bob_usdc = 1000 * unit;
		let bob_usdt = 1000 * unit;
		common_remove_lp_failure(pool_init_config, initial_usdc, initial_usdt, bob_usdc, bob_usdt);
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
		let exchange_base_amount = 1000 * unit;
		let pool_init_config = PoolInitConfiguration::StableSwap {
			owner: ALICE,
			pair: CurrencyPair::new(USDC, USDT),
			amplification_coefficient: 10_u16,
			fee: Permill::zero(),
		};
		common_exchange_failure(pool_init_config, initial_usdc, initial_usdt, exchange_base_amount);
	});
}

//
// - test lp_fees and owner_fee
#[test]
fn fees() {
	new_test_ext().execute_with(|| {
		let precision = 100;
		let epsilon = 1;
		let unit = 1_000_000_000_000_u128;
		let initial_usdt = 1_000_000_000_000_u128 * unit;
		let initial_usdc = 1_000_000_000_000_u128 * unit;
		let lp_fee = Permill::from_float(0.05);
		let owner_fee = Permill::from_float(0.01); // 10% of lp fees goes to pool owner
		let created_pool_id = create_stable_swap_pool(
			USDC,
			USDT,
			initial_usdc,
			initial_usdt,
			100_u16,
			lp_fee,
			owner_fee,
		);
		let bob_usdt = 1000 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));
		assert_ok!(Pablo::sell(
			Origin::signed(BOB),
			created_pool_id,
			USDT,
			bob_usdt,
			0_u128,
			false
		));
		let price = pallet::prices_for::<Test>(created_pool_id, USDC, USDT, 1 * unit).unwrap();
		assert_eq!(price.spot_price, 999999999991);

		assert_has_event::<Test, _>(
			|e| matches!(
				e.event,
				mock::Event::Pablo(crate::Event::Swapped { pool_id, fee, .. }) if pool_id == created_pool_id && fee.asset_id == USDC),
		);
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
		let pool_id = create_stable_swap_pool(
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

		assert_ok!(Pablo::sell(Origin::signed(BOB), pool_id, USDT, bob_usdt, 0_u128, false));
		let usdc_balance = Tokens::balance(USDC, &BOB);
		assert!((bob_usdt - usdc_balance) > 5_u128);
	});
}

#[test]
fn avoid_exchange_without_liquidity() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let lp_fee = Permill::from_float(0.05);
		let pool_init_config = PoolInitConfiguration::StableSwap {
			owner: ALICE,
			pair: CurrencyPair::new(USDC, USDT),
			amplification_coefficient: 40_000,
			fee: lp_fee,
		};
		System::set_block_number(1);
		let created_pool_id =
			Pablo::do_create_pool(pool_init_config).expect("pool creation failed");
		let bob_usdt = 1000 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));
		assert_noop!(
			Pablo::sell(Origin::signed(BOB), created_pool_id, USDT, bob_usdt, 0_u128, false),
			DispatchError::from(Error::<Test>::NotEnoughLiquidity)
		);
		assert_noop!(
			pallet::prices_for::<Test>(created_pool_id, USDC, USDT, 1 * unit),
			DispatchError::from(Error::<Test>::NotEnoughLiquidity)
		);
	});
}

#[test]
fn cannot_swap_between_wrong_pairs() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let lp_fee = Permill::from_float(0.05);
		let pool_init_config = PoolInitConfiguration::StableSwap {
			owner: ALICE,
			pair: CurrencyPair::new(BTC, USDT),
			amplification_coefficient: 40_000,
			fee: lp_fee,
		};
		System::set_block_number(1);
		let pool_id = Pablo::do_create_pool(pool_init_config).expect("pool creation failed");
		let base_amount = 100_000_u128 * unit;
		let quote_amount = 100_000_u128 * unit;
		assert_ok!(Tokens::mint_into(BTC, &ALICE, base_amount));
		assert_ok!(Tokens::mint_into(USDT, &ALICE, quote_amount));

		assert_ok!(Tokens::mint_into(BTC, &BOB, base_amount));
		assert_ok!(Tokens::mint_into(USDC, &BOB, quote_amount));
		assert_ok!(<Pablo as Amm>::add_liquidity(
			&ALICE,
			pool_id,
			base_amount,
			quote_amount,
			0,
			false
		));
		let usdc_amount = 2000_u128 * unit;
		let bad_pair = CurrencyPair::new(BTC, USDC);
		assert_noop!(
			Pablo::swap(Origin::signed(BOB), pool_id, bad_pair, usdc_amount, 0_u128, false),
			Error::<Test>::PairMismatch
		);
		assert_noop!(
			Pablo::swap(Origin::signed(BOB), pool_id, bad_pair.swap(), usdc_amount, 0_u128, false),
			Error::<Test>::PairMismatch
		);
	});
}

#[test]
fn cannot_get_exchange_value_for_wrong_asset() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let lp_fee = Permill::from_float(0.05);
		let pool_init_config = PoolInitConfiguration::StableSwap {
			owner: ALICE,
			pair: CurrencyPair::new(BTC, USDT),
			amplification_coefficient: 40_000,
			fee: lp_fee,
		};
		System::set_block_number(1);
		let pool_id = Pablo::do_create_pool(pool_init_config).expect("pool creation failed");
		let base_amount = 100_000_u128 * unit;
		let quote_amount = 100_000_u128 * unit;
		assert_ok!(Tokens::mint_into(BTC, &ALICE, base_amount));
		assert_ok!(Tokens::mint_into(USDT, &ALICE, quote_amount));

		assert_ok!(<Pablo as Amm>::add_liquidity(
			&ALICE,
			pool_id,
			base_amount,
			quote_amount,
			0,
			false
		));
		let usdc_amount = 2000_u128 * unit;
		assert_noop!(
			<Pablo as Amm>::get_exchange_value(pool_id, USDC, usdc_amount,),
			Error::<Test>::InvalidAsset
		);
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
		let pool_id = create_stable_swap_pool(
			USDC,
			USDT,
			initial_usdc,
			initial_usdt,
			100_u16,
			Permill::zero(),
			Permill::zero(),
		);
		let pool = Pablo::pools(pool_id).expect("pool not found");
		let pool = match pool {
				StableSwap(pool) => pool,
				_ => panic!("expected stable_swap pool"),
		};
		prop_assert_ok!(Tokens::mint_into(USDT, &BOB, usdt_balance));
		prop_assert_ok!(Tokens::mint_into(USDC, &BOB, usdc_balance));
		prop_assert_ok!(Pablo::add_liquidity(
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
		prop_assert_ok!(Pablo::remove_liquidity(
			Origin::signed(BOB),
			pool_id,
			lp,
			0,
			0,
				));
		let bob_usdc = Tokens::balance(USDC, &BOB);
		let bob_usdt = Tokens::balance(USDT, &BOB);
		prop_assert_ok!(default_acceptable_computation_error(usdc_balance + usdt_balance, bob_usdc +
 bob_usdt)); 		Ok(())
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
		let pool_id = create_stable_swap_pool(
			USDC,
			USDT,
			initial_usdc,
			initial_usdt,
			100_u16,
			Permill::zero(),
			Permill::zero(),
		);
		prop_assert_ok!(Tokens::mint_into(USDT, &BOB, value));
		prop_assert_ok!(Pablo::sell(Origin::signed(BOB), pool_id, USDT, value, 0_u128, false));
		// mint 1 extra USDC so that original amount of USDT can be buy back even with small slippage
		prop_assert_ok!(Tokens::mint_into(USDC, &BOB, unit));
		prop_assert_ok!(Pablo::buy(Origin::signed(BOB), pool_id, USDT, value, 0_u128, false));
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
		let pool_id = create_stable_swap_pool(
			USDC,
			USDT,
			initial_usdc,
			initial_usdt,
			100_u16,
			Permill::from_float(0.025),
			Permill::zero(),
		);
		let pool = Pablo::pools(pool_id).expect("pool not found");
		let pool = match pool {
				StableSwap(pool) => pool,
				_ => panic!("expected stable_swap pool"),
		};
		prop_assert_ok!(Tokens::mint_into(USDT, &BOB, value));
		prop_assert_ok!(Pablo::swap(Origin::signed(BOB), pool_id, CurrencyPair::new(USDC, USDT),
 value, 0, false)); 		let bob_usdc = Tokens::balance(USDC, &BOB);
		let expected_usdc =  value - pool.fee_config.fee_rate.mul_floor(value);
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
		let pool_id = create_stable_swap_pool(
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
					<Pablo as Amm>::get_exchange_value(pool_id, USDC, quote * unit)
						.expect("get_exchange_value not found") as f64 /
						unit as f64,
				)
			})
			.collect::<Vec<_>>();
		let max_amount = points.iter().copied().fold(f64::NAN, |x, (_, y)| f64::max(x, y));

		use plotters::prelude::*;
		let area = BitMapBackend::new("./plots/stable_swap/curve_base.png", (1024, 768))
			.into_drawing_area();
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
		let pool_id = create_stable_swap_pool(
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
				let base = <Pablo as Amm>::get_exchange_value(pool_id, USDC, quote)
					.expect("get_exchange_value failed");
				let slippage = if base <= quote { quote - base } else { base };
				(quote / unit, slippage as f64 / unit as f64)
			})
			.collect::<Vec<_>>();
		let max_amount = points.iter().copied().fold(f64::NAN, |x, (_, y)| f64::max(x, y));

		use plotters::prelude::*;
		let area = BitMapBackend::new("./plots/stable_swap/curve_slippage.png", (1024, 768))
			.into_drawing_area();
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
		let pool_id = create_stable_swap_pool(
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
		let pool_account = Pablo::account_id(&pool_id);
		let range: Vec<u128> = (0..window).collect();

		let points1 = range
			.iter()
			.map(|_| {
				let amount = unit;
				let _ = Tokens::mint_into(USDC, &BOB, amount).expect("mint failed");
				let _base = <Pablo as Amm>::sell(&BOB, pool_id, USDC, amount, 0_u128, true)
					.expect("sell failed");
				let pool_sell_asset_balance =
					Tokens::balance(USDC, &pool_account) as f64 / unit as f64;
				let pool_buy_asset_balance =
					Tokens::balance(USDT, &pool_account) as f64 / unit as f64;
				(pool_buy_asset_balance, pool_sell_asset_balance)
			})
			.collect::<Vec<_>>();
		let pool_id = create_stable_swap_pool(
			USDC,
			USDT,
			initial_usdc,
			initial_usdt,
			5_u16,
			Permill::zero(),
			Permill::zero(),
		);
		let pool_account = Pablo::account_id(&pool_id);
		let points2 = range
			.iter()
			.map(|_| {
				let amount = unit;
				let _ = Tokens::mint_into(USDT, &BOB, amount).expect("mint failed");
				let _base = <Pablo as Amm>::sell(&BOB, pool_id, USDT, amount, 0_u128, true)
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
		let area = BitMapBackend::new("./plots/stable_swap/curve_graph.png", (1024, 768))
			.into_drawing_area();
		area.fill(&WHITE).unwrap();

		let mut chart = ChartBuilder::on(&area)
			.caption("Curve price, pool has 1000 USDC 1000 USDT", ("Arial", 25).into_font())
			.margin(100u32)
			.x_label_area_size(30u32)
			.y_label_area_size(30u32)
			.build_cartesian_2d(0f64..max_base, 0f64..max_quote)
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
