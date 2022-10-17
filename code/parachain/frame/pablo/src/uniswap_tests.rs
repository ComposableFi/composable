use crate::uniswap::Uniswap;
#[cfg(test)]
use crate::{
	common_test_functions::*,
	mock,
	mock::{Pablo, *},
	pallet, Error,
	PoolConfiguration::ConstantProduct,
	PoolInitConfiguration,
};
use composable_support::math::safe::safe_multiply_by_rational;
use composable_tests_helpers::{
	prop_assert_noop, prop_assert_ok,
	test::helper::{acceptable_computation_error, default_acceptable_computation_error},
};
use composable_traits::{
	defi::CurrencyPair,
	dex::{Amm, ConstantProductPoolInfo, FeeConfig},
};
use frame_support::{
	assert_noop, assert_ok,
	traits::{
		fungibles::{Inspect, Mutate},
		Hooks,
	},
};
use proptest::prelude::*;
use sp_runtime::{traits::IntegerSquareRoot, DispatchError, Perbill, Permill};
use sp_std::collections::btree_map::BTreeMap;

fn create_pool(
	base_asset: AssetId,
	quote_asset: AssetId,
	base_amount: Balance,
	quote_amount: Balance,
	lp_fee: Permill,
	protocol_fee: Permill,
) -> PoolId {
	System::set_block_number(1);
	let actual_pool_id = Uniswap::<Test>::do_create_pool(
		&ALICE,
		CurrencyPair::new(base_asset, quote_asset),
		FeeConfig {
			fee_rate: lp_fee,
			owner_fee_rate: protocol_fee,
			protocol_fee_rate: Permill::zero(),
		},
		Permill::from_percent(50),
	)
	.expect("pool creation failed");

	// Mint the tokens
	assert_ok!(Tokens::mint_into(base_asset, &ALICE, base_amount));
	assert_ok!(Tokens::mint_into(quote_asset, &ALICE, quote_amount));

	// Add the liquidity
	assert_ok!(<Pablo as Amm>::add_liquidity(
		&ALICE,
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

fn get_pool(pool_id: PoolId) -> ConstantProductPoolInfo<AccountId, AssetId> {
	match Pablo::pools(pool_id).expect("pool not found") {
		ConstantProduct(pool) => pool,
	}
}

#[test]
fn test() {
	new_test_ext().execute_with(|| {
		let pool_init_config = PoolInitConfiguration::ConstantProduct {
			owner: ALICE,
			pair: CurrencyPair::new(BTC, USDT),
			fee: Permill::zero(),
			base_weight: Permill::from_percent(50),
		};
		let pool_id = Pablo::do_create_pool(pool_init_config).expect("pool creation failed");

		let pool = get_pool(pool_id);

		let current_product = |a| {
			let balance_btc = Tokens::balance(BTC, &a);
			let balance_usdt = Tokens::balance(USDT, &a);
			balance_btc * balance_usdt
		};
		let current_pool_product = || current_product(Pablo::account_id(&pool_id));

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
		assert_ok!(<Pablo as Amm>::add_liquidity(
			&ALICE,
			pool_id,
			initial_btc,
			initial_usdt,
			0,
			false
		));

		// 1 unit of btc = 45k + some unit of usdt
		let ratio = <Pablo as Amm>::get_exchange_value(pool_id, BTC, unit)
			.expect("get_exchange_value failed");
		assert!(ratio > (initial_usdt / initial_btc) * unit);

		let initial_pool_invariant = current_pool_product();

		assert_eq!(initial_user_invariant, initial_pool_invariant);

		// swap a btc
		let swap_btc = unit;
		assert_ok!(Tokens::mint_into(BTC, &BOB, swap_btc));

		<Pablo as Amm>::sell(&BOB, pool_id, BTC, swap_btc, 0_u128, false).expect("sell failed");

		let new_pool_invariant = current_pool_product();
		assert_ok!(default_acceptable_computation_error(
			initial_pool_invariant,
			new_pool_invariant
		));

		<Pablo as Amm>::buy(&BOB, pool_id, BTC, swap_btc, 0_u128, false).expect("buy failed");

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
		assert_ok!(<Pablo as Amm>::remove_liquidity(&ALICE, pool_id, lp, 0, 0));

		// Alice should get back a different amount of tokens.
		let alice_btc = Tokens::balance(BTC, &ALICE);
		let alice_usdt = Tokens::balance(USDT, &ALICE);
		assert_ok!(default_acceptable_computation_error(alice_btc, initial_btc));
		assert_ok!(default_acceptable_computation_error(alice_usdt, initial_usdt));
	});
}

#[test]
fn test_redeemable_assets() {
	new_test_ext().execute_with(|| {
		let pool_init_config = PoolInitConfiguration::ConstantProduct {
			owner: ALICE,
			pair: CurrencyPair::new(BTC, USDT),
			fee: Permill::zero(),
			base_weight: Permill::from_percent(50),
		};
		let pool_id = Pablo::do_create_pool(pool_init_config).expect("pool creation failed");

		let pool = get_pool(pool_id);

		let unit = 1_000_000_000_000;

		let btc_price = 45_000;

		let nb_of_btc = 100;

		// 100 BTC/4.5M USDT
		let initial_btc = nb_of_btc * unit;
		let initial_usdt = nb_of_btc * btc_price * unit;

		// Mint the tokens
		assert_ok!(Tokens::mint_into(BTC, &ALICE, initial_btc));
		assert_ok!(Tokens::mint_into(USDT, &ALICE, initial_usdt));

		// Add the liquidity
		assert_ok!(<Pablo as Amm>::add_liquidity(
			&ALICE,
			pool_id,
			initial_btc,
			initial_usdt,
			0,
			false
		));

		let lp = Tokens::balance(pool.lp_token, &ALICE);
		// if we want to redeem all lp token, it must give same values as used for add_liquidity
		let redeemable_assets = <Pablo as Amm>::redeemable_assets_for_lp_tokens(
			pool_id,
			lp,
			BTreeMap::from([(BTC, initial_btc), (USDT, initial_usdt)]),
		)
		.expect("redeemable_assets failed");
		let base_amount = *redeemable_assets.assets.get(&BTC).expect("Invalid asset");
		let quote_amount = *redeemable_assets.assets.get(&USDT).expect("Invalid asset");

		assert_ok!(default_acceptable_computation_error(base_amount, initial_btc));
		assert_ok!(default_acceptable_computation_error(quote_amount, initial_usdt));
	});
}

//- test lp mint/burn
#[test]
fn add_remove_lp() {
	new_test_ext().execute_with(|| {
		let pool_init_config = PoolInitConfiguration::ConstantProduct {
			owner: ALICE,
			pair: CurrencyPair::new(BTC, USDT),
			fee: Permill::zero(),
			base_weight: Permill::from_percent(50),
		};
		let unit = 1_000_000_000_000_u128;
		let initial_btc = 1_00_u128 * unit;
		let btc_price = 45_000_u128;
		let initial_usdt = initial_btc * btc_price;
		let btc_amount = 10 * unit;
		let usdt_amount = btc_amount * btc_price;
		let expected_lp_check =
			|_base_amount: Balance, _quote_amount: Balance, lp: Balance| -> bool { lp > 0_u128 };
		common_add_remove_lp(
			pool_init_config,
			initial_btc,
			initial_usdt,
			btc_amount,
			usdt_amount,
			expected_lp_check,
		);
	});
}

#[test]
fn test_add_liquidity_with_disproportionate_amount() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let initial_usdc = 2500_u128 * unit;
		let initial_usdt = 2500_u128 * unit;
		let pool = create_pool(
			USDC,
			USDT,
			initial_usdc,
			initial_usdt,
			Permill::from_percent(1),
			Permill::zero(),
		);

		let base_amount = 30_u128 * unit;
		let quote_amount = 1_00_u128 * unit;
		assert_ok!(Tokens::mint_into(USDC, &ALICE, base_amount));
		assert_ok!(Tokens::mint_into(USDT, &ALICE, quote_amount));

		// Add the liquidity, user tries to provide more quote_amount compare to
        // pool's ratio
		assert_ok!(<Pablo as Amm>::add_liquidity(
			&ALICE,
			pool,
			base_amount,
			quote_amount,
			0,
			false
		));
	assert_last_event::<Test, _>(|e| {
		matches!(e.event,
            mock::Event::Pablo(crate::Event::LiquidityAdded { who, pool_id, base_amount, quote_amount, .. })
            if who == ALICE
            && pool_id == pool
            && base_amount == 30_u128 * unit
            && quote_amount == 30_u128 * unit)
	});
	});
}

// test add liquidity with min_mint_amount
#[test]
fn add_lp_with_min_mint_amount() {
	new_test_ext().execute_with(|| {
		let pool_init_config = PoolInitConfiguration::ConstantProduct {
			owner: ALICE,
			pair: CurrencyPair::new(BTC, USDT),
			fee: Permill::zero(),
			base_weight: Permill::from_percent(50),
		};
		let unit = 1_000_000_000_000_u128;
		let initial_btc = 1_00_u128 * unit;
		let btc_price = 45_000_u128;
		let initial_usdt = initial_btc * btc_price;
		let btc_amount = 10 * unit;
		let usdt_amount = btc_amount * btc_price;
		let expected_lp = |base_amount: Balance,
		                   _quote_amount: Balance,
		                   lp_total_issuance: Balance,
		                   pool_base_amount: Balance,
		                   _pool_quote_amount: Balance|
		 -> Balance { lp_total_issuance * base_amount / pool_base_amount };
		common_add_lp_with_min_mint_amount(
			pool_init_config,
			initial_btc,
			initial_usdt,
			btc_amount,
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
		let pool_init_config = PoolInitConfiguration::ConstantProduct {
			owner: ALICE,
			pair: CurrencyPair::new(BTC, USDT),
			fee: Permill::zero(),
			base_weight: Permill::from_percent(50),
		};
		let unit = 1_000_000_000_000_u128;
		let initial_btc = 1_00_u128 * unit;
		let btc_price = 45_000_u128;
		let initial_usdt = initial_btc * btc_price;
		let bob_btc = 10 * unit;
		let bob_usdt = bob_btc * btc_price;
		common_remove_lp_failure(pool_init_config, initial_btc, initial_usdt, bob_btc, bob_usdt);
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
		let pool_init_config = PoolInitConfiguration::ConstantProduct {
			owner: ALICE,
			pair: CurrencyPair::new(BTC, USDT),
			fee: Permill::zero(),
			base_weight: Permill::from_percent(50),
		};
		let exchange_base_amount = 100 * unit;
		common_exchange_failure(pool_init_config, initial_usdt, initial_btc, exchange_base_amount)
	});
}

//
// - test high slippage scenario
// trying to exchange a large value, will result in high_slippage scenario
// there should be substantial difference between expected exchange value and received amount.
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

		assert_ok!(<Pablo as Amm>::sell(&BOB, pool_id, BTC, bob_btc, 0_u128, false));
		let usdt_balance = Tokens::balance(USDT, &BOB);
		let idea_usdt_balance = bob_btc * btc_price;
		assert!((idea_usdt_balance - usdt_balance) > 5_u128);
	});
}

//
// - test lp_fee and owner_fee
#[test]
fn fees() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let initial_btc = 1_00_u128 * unit;
		let btc_price = 45_000_u128;
		let initial_usdt = initial_btc * btc_price;
		let lp_fee = Permill::from_float(0.05);
		let owner_fee = Permill::from_float(0.01);
		let pool_id = create_pool(BTC, USDT, initial_btc, initial_usdt, lp_fee, owner_fee);
		let bob_usdt = 45_000_u128 * unit;
		let quote_usdt = bob_usdt - lp_fee.mul_floor(bob_usdt);
		let expected_btc_value = <Pablo as Amm>::get_exchange_value(pool_id, USDT, quote_usdt)
			.expect("get_exchange_value failed");
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));

		assert_ok!(<Pablo as Amm>::sell(&BOB, pool_id, USDT, bob_usdt, 0_u128, false));
		let price = pallet::prices_for::<Test>(
			pool_id,
			BTC,
			USDT,
			1 * unit,
		).unwrap();
		assert_eq!(price.spot_price, 46_326_729_585_161_862);
		let btc_balance = Tokens::balance(BTC, &BOB);
        sp_std::if_std! {
            println!("expected_btc_value {:?}, btc_balance {:?}", expected_btc_value, btc_balance);
        }
		assert_ok!(default_acceptable_computation_error(expected_btc_value, btc_balance));
        // lp_fee is taken from quote 
		// from lp_fee 1 % (as per owner_fee) goes to pool owner (ALICE)
        let alice_usdt_bal = Tokens::balance(USDT, &ALICE);
        let expected_alice_usdt_bal = owner_fee.mul_floor(lp_fee.mul_floor(bob_usdt));
        sp_std::if_std! {
            println!("alice_usdt_bal {:?}, expected_alice_usdt_bal {:?}", alice_usdt_bal, expected_alice_usdt_bal);
        }
		assert_ok!(default_acceptable_computation_error(expected_alice_usdt_bal, alice_usdt_bal));

	});
}
#[test]
fn staking_pool_test() {
	new_test_ext().execute_with(|| {
	System::set_block_number(1);
		let unit = 1_000_000_000_000_u128;
		let initial_btc = 1_00_u128 * unit;
		let btc_price = 45_000_u128;
		let initial_usdt = initial_btc * btc_price;
		let pool_init_config = PoolInitConfiguration::ConstantProduct {
			owner: ALICE,
			pair: CurrencyPair::new(BTC, USDT),
			fee: Permill::from_float(0.05),
			base_weight: Permill::from_percent(50),
		};

		let pool_id = Pablo::do_create_pool(pool_init_config).expect("pool creation failed");
		// Mint the tokens
		assert_ok!(Tokens::mint_into(BTC, &ALICE, initial_btc));
		assert_ok!(Tokens::mint_into(USDT, &ALICE, initial_usdt));
		// Add the liquidity
		assert_ok!(<Pablo as Amm>::add_liquidity(
			&ALICE,
			pool_id,
			initial_btc,
			initial_usdt,
			0,
			false
		));
        // make sure a Staking pool is created.
	assert_has_event::<Test, _>(|e| {
		matches!(e.event,
            mock::Event::StakingRewards(pallet_staking_rewards::Event::RewardPoolCreated { owner, .. })
            if owner == Pablo::account_id(&pool_id) )
	});

		let bob_usdt = 45_000_u128 * unit;
        let trading_fee = Perbill::from_float(0.05).mul_floor(bob_usdt);
        let protocol_fee = Perbill::from_float(0.2).mul_floor(trading_fee);
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));

		assert_ok!(<Pablo as Amm>::sell(&BOB, pool_id, USDT, bob_usdt, 0_u128, false));
        // lp_fee is taken from quote 
		// from lp_fee 20 % (default) (as per owner_fee) goes to staking pool
	assert_has_event::<Test, _>(|e| {
        println!("{:?}", e.event);
		matches!(e.event,
            mock::Event::StakingRewards(pallet_staking_rewards::Event::RewardTransferred { from, reward_currency, reward_increment, ..})
            if from == BOB && reward_currency == USDT && reward_increment == protocol_fee)
	});

	});
}

#[test]
fn avoid_exchange_without_liquidity() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let lp_fee = Permill::from_float(0.05);
		let pool_init_config = PoolInitConfiguration::ConstantProduct {
			owner: ALICE,
			pair: CurrencyPair::new(BTC, USDT),
			fee: lp_fee,
			base_weight: Permill::from_percent(50),
		};
		System::set_block_number(1);
		let pool_id = Pablo::do_create_pool(pool_init_config).expect("pool creation failed");
		let bob_usdt = 45_000_u128 * unit;
		let quote_usdt = bob_usdt - lp_fee.mul_floor(bob_usdt);
		assert_noop!(
			<Pablo as Amm>::get_exchange_value(pool_id, USDT, quote_usdt),
			DispatchError::from(Error::<Test>::NotEnoughLiquidity)
		);
	});
}

#[test]
fn cannot_swap_between_wrong_pairs() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let lp_fee = Permill::from_float(0.05);
		let pool_init_config = PoolInitConfiguration::ConstantProduct {
			owner: ALICE,
			pair: CurrencyPair::new(BTC, USDT),
			fee: lp_fee,
			base_weight: Permill::from_percent(50),
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
		let pool_init_config = PoolInitConfiguration::ConstantProduct {
			owner: ALICE,
			pair: CurrencyPair::new(BTC, USDT),
			fee: lp_fee,
			base_weight: Permill::from_percent(50),
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

#[test]
fn weights_zero() {
	new_test_ext().execute_with(|| {
		let pool_init_config = PoolInitConfiguration::ConstantProduct {
			owner: ALICE,
			pair: CurrencyPair::new(BTC, USDT),
			fee: Permill::zero(),
			base_weight: Permill::zero(),
		};
		assert_noop!(Pablo::do_create_pool(pool_init_config), Error::<Test>::WeightsMustBeNonZero);
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
		  prop_assert_ok!(Pablo::sell(Origin::signed(BOB), pool_id, USDT, usdt_value, 0_u128, false));
		  let bob_btc = Tokens::balance(BTC, &BOB);
		  // mint extra BTC equal to slippage so that original amount of USDT can be buy back
		  prop_assert_ok!(Tokens::mint_into(BTC, &BOB, btc_value - bob_btc));
		  prop_assert_ok!(Pablo::buy(Origin::signed(BOB), pool_id, USDT, usdt_value, 0_u128, false));
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
		  let pool = get_pool(pool_id);
		  prop_assert_ok!(Tokens::mint_into(USDT, &BOB, usdt_value));
		  prop_assert_ok!(Tokens::mint_into(BTC, &BOB, btc_value));
		  prop_assert_ok!(Pablo::add_liquidity(Origin::signed(BOB), pool_id, btc_value, usdt_value, 0, false));
		  let term1 = initial_usdt.integer_sqrt_checked().expect("integer_sqrt failed");
		  let term2 = initial_btc.integer_sqrt_checked().expect("integer_sqrt failed");
		  let expected_lp_tokens = safe_multiply_by_rational(term1, btc_value, term2).expect("multiply_by_rational failed");
		  let lp_token = Tokens::balance(pool.lp_token, &BOB);
		  prop_assert_ok!(default_acceptable_computation_error(expected_lp_tokens, lp_token));
		  prop_assert_ok!(Pablo::remove_liquidity(Origin::signed(BOB), pool_id, lp_token, 0, 0));
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
		  let pool = get_pool(pool_id);
		  prop_assert_ok!(Tokens::mint_into(USDT, &BOB, usdt_value));
		  prop_assert_ok!(Pablo::swap(Origin::signed(BOB), pool_id, CurrencyPair::new(BTC, USDT), usdt_value, 0, false));
		  let usdt_value_after_fee = usdt_value - pool.fee_config.fee_rate.mul_floor(usdt_value);
		  let ratio = initial_btc as f64 / initial_usdt as f64;
		  let expected_btc_value = ratio * usdt_value_after_fee as f64;
		  let expected_btc_value = expected_btc_value as u128;
		  let bob_btc = Tokens::balance(BTC, &BOB);
		  prop_assert_ok!(default_acceptable_computation_error(bob_btc, expected_btc_value));
		  Ok(())
	  })?;
  }

	#[test]
	fn weights_sum_to_one(
		base_weight_in_percent in 1..100u32,
	) {
	  new_test_ext().execute_with(|| {
		  let pool_init_config = PoolInitConfiguration::ConstantProduct {
			  owner: ALICE,
			  pair: CurrencyPair::new(BTC, USDT),
			fee: Permill::zero(),
			  base_weight: Permill::from_percent(base_weight_in_percent),
		  };
		  let pool_id = Pablo::do_create_pool(pool_init_config).expect("pool is valid");

		  let pool = get_pool(pool_id);

		  prop_assert_eq!(Permill::one(), pool.base_weight + pool.quote_weight);

		  Ok(())
	  })?;
  }

	#[test]
	fn weights_sum_to_more_than_one(
		base_weight_in_percent in 100..u32::MAX,
	) {
	  new_test_ext().execute_with(|| {
		  let pool_init_config = PoolInitConfiguration::ConstantProduct {
			  owner: ALICE,
			  pair: CurrencyPair::new(BTC, USDT),
			fee: Permill::zero(),
			  base_weight: Permill::from_percent(base_weight_in_percent),
		  };

		  prop_assert_noop!(Pablo::do_create_pool(pool_init_config), Error::<Test>::WeightsMustSumToOne);

		  Ok(())
	  })?;
  }
}

mod twap {
	use super::*;
	use crate::types::TimeWeightedAveragePrice;
	use composable_traits::defi::Rate;
	use sp_runtime::traits::One;

	fn run_to_block(n: BlockNumber) {
		Pablo::on_finalize(System::block_number());
		for b in (System::block_number() + 1)..=n {
			next_block(b);
			if b != n {
				Pablo::on_finalize(System::block_number());
			}
		}
	}

	fn next_block(n: u64) {
		Timestamp::set_timestamp(MILLISECS_PER_BLOCK * n);
		System::set_block_number(n);
		Pablo::on_initialize(n);
	}

	#[test]
	fn twap_asset_prices_change_after_twap_interval() {
		new_test_ext().execute_with(|| {
			let unit = 1_000_000_000_000_u128;
			let initial_btc = 100_u128 * unit;
			let initial_usdt = 100_u128 * unit;
			let pool_id =
				create_pool(BTC, USDT, initial_btc, initial_usdt, Permill::zero(), Permill::zero());

			System::set_block_number(0);
			assert_eq!(Pablo::twap(pool_id), None);
			assert_ok!(Pablo::enable_twap(Origin::root(), pool_id));
			run_to_block(1);

			assert_eq!(
				Pablo::twap(pool_id),
				Some(TimeWeightedAveragePrice {
					base_price_cumulative: 1,
					quote_price_cumulative: 1,
					timestamp: 0,
					base_twap: Rate::one(),
					quote_twap: Rate::one()
				})
			);

			// TWAP get updated from on_initialize()
			run_to_block(TWAP_INTERVAL + 1);
			// execute a swap to invoke update_twap() however it will only update price_cumulative
			// and not twap as elapsed time is < TWAPInterval
			let usdt_value = 1_u128 * unit;
			assert_ok!(Tokens::mint_into(USDT, &BOB, usdt_value));
			assert_ok!(Pablo::swap(
				Origin::signed(BOB),
				pool_id,
				CurrencyPair::new(BTC, USDT),
				usdt_value,
				0,
				false
			));

			let price_cumulative =
				Pablo::price_cumulative(pool_id).expect("price_cumulative not found");
			assert_eq!(price_cumulative.timestamp, (TWAP_INTERVAL + 1) * MILLISECS_PER_BLOCK);
			assert_eq!(price_cumulative.base_price_cumulative, 131764);
			assert_eq!(price_cumulative.quote_price_cumulative, 132242);
			// here in on_initialize() TWAP does not get updated as elapsed < TWAPInterval
			// and as TWAP does not get updated, price_cumulative will also not be updated.
			run_to_block(TWAP_INTERVAL + 2);
			let price_cumulative_ =
				Pablo::price_cumulative(pool_id).expect("price_cumulative not found");
			assert_eq!(price_cumulative.timestamp, price_cumulative_.timestamp);
			assert_eq!(
				price_cumulative.base_price_cumulative,
				price_cumulative_.base_price_cumulative
			);
			assert_eq!(
				price_cumulative.quote_price_cumulative,
				price_cumulative_.quote_price_cumulative
			);

			let elapsed = TWAP_INTERVAL * MILLISECS_PER_BLOCK;
			let twap = Pablo::twap(pool_id).expect("twap not found");
			assert_eq!(twap.timestamp, elapsed);
			assert_eq!(twap.base_price_cumulative, 120001);
			assert_eq!(twap.quote_price_cumulative, 120001);
			assert_eq!(twap.base_twap, Rate::one());
			assert_eq!(twap.quote_twap, Rate::one());
		});
	}

	#[test]
	fn twap_asset_prices_change_within_twap_interval() {
		new_test_ext().execute_with(|| {
			let unit = 1_000_000_000_000_u128;
			let initial_btc = 100_u128 * unit;
			let initial_usdt = 100_u128 * unit;
			let pool_identifier =
				create_pool(BTC, USDT, initial_btc, initial_usdt, Permill::zero(), Permill::zero());

			let mut min_base_price = Rate::from_float(99999999.0);
			let mut min_quote_price = Rate::from_float(99999999.0);
			let mut max_base_price = Rate::from_float(0.0);
			let mut max_quote_price = Rate::from_float(0.0);
			let mut update_min_max_price = || {
				let base_price =
					Pablo::do_get_exchange_rate(pool_identifier, crate::PriceRatio::NotSwapped);
				assert_ok!(base_price);
				let base_price = base_price.unwrap();
				let quote_price =
					Pablo::do_get_exchange_rate(pool_identifier, crate::PriceRatio::Swapped);
				assert_ok!(quote_price);
				let quote_price = quote_price.unwrap();
				min_base_price = sp_std::cmp::min(base_price, min_base_price);
				min_quote_price = sp_std::cmp::min(quote_price, min_quote_price);
				max_base_price = sp_std::cmp::max(base_price, max_base_price);
				max_quote_price = sp_std::cmp::max(quote_price, max_quote_price);
			};
			System::set_block_number(0);
			assert_eq!(Pablo::twap(pool_identifier), None);
			assert_ok!(Pablo::enable_twap(Origin::root(), pool_identifier));
			run_to_block(1);

			assert_eq!(
				Pablo::twap(pool_identifier),
				Some(TimeWeightedAveragePrice {
					base_price_cumulative: 1,
					quote_price_cumulative: 1,
					timestamp: 0,
					base_twap: Rate::one(),
					quote_twap: Rate::one()
				})
			);
			let run_to_block_and_swap = |block_number: BlockNumber| {
				run_to_block(block_number);
				// execute a swap to invoke update_twap() on given block_number
				let usdt_value = 1_u128 * unit;
				assert_ok!(Tokens::mint_into(USDT, &BOB, usdt_value));
				assert_ok!(Pablo::swap(
					Origin::signed(BOB),
					pool_identifier,
					CurrencyPair::new(BTC, USDT),
					usdt_value,
					0,
					false
				));
			};
			run_to_block_and_swap(5);
			let price_cumulative =
				Pablo::price_cumulative(pool_identifier).expect("price_cumulative not found");
			assert_eq!(price_cumulative.timestamp, 5 * MILLISECS_PER_BLOCK);
			assert_eq!(price_cumulative.base_price_cumulative, 58818);
			assert_eq!(price_cumulative.quote_price_cumulative, 61206);
			update_min_max_price();

			run_to_block_and_swap(8);
			let price_cumulative =
				Pablo::price_cumulative(pool_identifier).expect("price_cumulative not found");
			assert_eq!(price_cumulative.timestamp, 8 * MILLISECS_PER_BLOCK);
			assert_eq!(price_cumulative.base_price_cumulative, 93420);
			assert_eq!(price_cumulative.quote_price_cumulative, 98660);
			update_min_max_price();

			run_to_block_and_swap(TWAP_INTERVAL + 1);
			assert_has_event::<Test, _>(|e| {
				matches!(e.event,
				mock::Event::Pablo(crate::Event::TwapUpdated { pool_id, ..})
				if pool_id == pool_identifier
				)
			});
			let price_cumulative =
				Pablo::price_cumulative(pool_identifier).expect("price_cumulative not found");
			assert_eq!(price_cumulative.timestamp, (TWAP_INTERVAL + 1) * MILLISECS_PER_BLOCK);
			assert_eq!(price_cumulative.base_price_cumulative, 127799);
			assert_eq!(price_cumulative.quote_price_cumulative, 136359);
			update_min_max_price();
			let elapsed = (TWAP_INTERVAL) * MILLISECS_PER_BLOCK;
			let twap = Pablo::twap(pool_identifier).expect("twap not found");
			assert_eq!(twap.timestamp, elapsed);
			assert!(twap.base_twap > min_base_price);
			assert!(twap.quote_twap > min_quote_price);
			assert!(twap.base_twap < max_base_price);
			assert!(twap.quote_twap < max_quote_price);
		});
	}
}
