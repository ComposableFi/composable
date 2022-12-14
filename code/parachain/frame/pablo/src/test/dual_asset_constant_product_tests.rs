use crate::mock::Test;

use crate::{
	dual_asset_constant_product::DualAssetConstantProduct as DACP,
	mock,
	mock::{Pablo, *},
	pallet,
	test::common_test_functions::*,
	Error,
	PoolConfiguration::DualAssetConstantProduct,
	PoolConfigurationType, PoolInitConfiguration,
};
use composable_support::math::safe::safe_multiply_by_rational;
use composable_tests_helpers::{
	prop_assert_ok,
	test::{
		block::next_block,
		currency,
		helper::{
			acceptable_computation_error, default_acceptable_computation_error, RuntimeTrait,
		},
	},
};
use composable_traits::dex::{Amm, AssetAmount, BasicPoolInfo, FeeConfig};
use frame_support::{
	assert_noop, assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use proptest::prelude::*;
use sp_runtime::{
	traits::{ConstU32, IntegerSquareRoot},
	BoundedBTreeMap, DispatchError, Perbill, Permill, TokenError,
};
use sp_std::collections::btree_map::BTreeMap;

fn create_pool(
	base_asset: AssetId,
	quote_asset: AssetId,
	base_amount: Balance,
	quote_amount: Balance,
	lp_token_id: AssetId,
	lp_fee: Permill,
	protocol_fee: Permill,
) -> PoolId {
	// TODO(benluelo): why is this set here
	System::set_block_number(1);
	let asset_weights = dual_asset_pool_weights(base_asset, Permill::from_percent(50), quote_asset);
	let actual_pool_id = DACP::<Test>::do_create_pool(
		&ALICE,
		FeeConfig {
			fee_rate: lp_fee,
			owner_fee_rate: protocol_fee,
			protocol_fee_rate: Permill::zero(),
		},
		asset_weights,
		Some(lp_token_id),
	)
	.expect("pool creation failed");

	// Mint the tokens
	assert_ok!(Tokens::mint_into(base_asset, &ALICE, base_amount));
	assert_ok!(Tokens::mint_into(quote_asset, &ALICE, quote_amount));

	// Add the liquidity
	assert_ok!(<Pablo as Amm>::add_liquidity(
		&ALICE,
		actual_pool_id,
		BTreeMap::from([(base_asset, base_amount), (quote_asset, quote_amount)]),
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

fn get_pool(pool_id: PoolId) -> BasicPoolInfo<AccountId, AssetId, ConstU32<2>> {
	match Pablo::pools(pool_id).expect("pool not found") {
		DualAssetConstantProduct(pool) => pool,
	}
}

#[test]
#[ignore = "broken and unclear what it is testing"]
fn test() {
	new_test_ext().execute_with(|| {
		let pool_init_config = valid_pool_init_config(
			&ALICE,
			BTC,
			Permill::from_percent(50_u32),
			USDT,
			Permill::zero(),
		);
		let pool_id = Pablo::do_create_pool(pool_init_config, Some(LP_TOKEN_ID))
			.expect("pool creation failed");

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
			BTreeMap::from([(BTC, initial_btc), (USDT, initial_usdt)]),
			0,
			false
		));

		// TODO: Re-evaluate this assertion. The calculation of expected value is incorrect.
		// 1 unit of btc = 45k + some unit of usdt
		let ratio = <Pablo as Amm>::spot_price(pool_id, AssetAmount::new(BTC, unit), USDT, true)
			.expect("get_exchange_value failed");
		// 4_500_000_000_000_000_000 / 101 ~= 44_554_455_445_544_554
		assert!(ratio.value.amount == 44_554_455_445_544_554);

		let initial_pool_invariant = current_pool_product();

		assert_eq!(initial_user_invariant, initial_pool_invariant);

		// swap a btc
		let swap_btc = unit;
		assert_ok!(Tokens::mint_into(BTC, &BOB, swap_btc));
		// in_given_out uses greedy rounding so bob will need some extra USDT
		assert_ok!(Tokens::mint_into(USDT, &BOB, swap_btc));

		<Pablo as Amm>::do_swap(
			&BOB,
			pool_id,
			AssetAmount::new(BTC, swap_btc),
			AssetAmount::new(USDT, 0),
			false,
		)
		.expect("sell failed");

		let new_pool_invariant = current_pool_product();
		assert_ok!(default_acceptable_computation_error(
			initial_pool_invariant,
			new_pool_invariant
		));

		<Pablo as Amm>::do_buy(&BOB, pool_id, USDT, AssetAmount::new(BTC, swap_btc), false)
			.expect("buy failed");

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
		assert_ok!(<Pablo as Amm>::remove_liquidity(
			&ALICE,
			pool_id,
			lp,
			BTreeMap::from([(USDT, 0), (BTC, 0)])
		));

		// Alice should get back a different amount of tokens.
		let alice_btc = Tokens::balance(BTC, &ALICE);
		let alice_usdt = Tokens::balance(USDT, &ALICE);
		assert_ok!(default_acceptable_computation_error(alice_btc, initial_btc));
		assert_ok!(default_acceptable_computation_error(alice_usdt, initial_usdt));
	});
}

pub fn valid_pool_init_config(
	owner: &AccountId,
	first_asset: AssetId,
	first_asset_weight: Permill,
	second_asset: AssetId,
	fee: Permill,
) -> PoolInitConfiguration<AccountId, AssetId> {
	PoolInitConfiguration::DualAssetConstantProduct {
		owner: *owner,
		assets_weights: dual_asset_pool_weights(first_asset, first_asset_weight, second_asset),
		fee,
	}
}

#[test]
fn test_redeemable_assets() {
	new_test_ext().execute_with(|| {
		let pool_init_config = valid_pool_init_config(
			&ALICE,
			BTC,
			Permill::from_percent(50_u32),
			USDT,
			Permill::zero(),
		);
		let pool_id = Pablo::do_create_pool(pool_init_config, Some(LP_TOKEN_ID))
			.expect("pool creation failed");

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
			BTreeMap::from([(BTC, initial_btc), (USDT, initial_usdt)]),
			0,
			false
		));

		let lp = Tokens::balance(pool.lp_token, &ALICE);
		// if we want to redeem all lp token, it must give same values as used for add_liquidity
		let redeemable_assets = <Pablo as Amm>::redeemable_assets_for_lp_tokens(pool_id, lp)
			.expect("redeemable_assets failed");
		let base_amount = *redeemable_assets.assets.get(&BTC).expect("Invalid asset");
		let quote_amount = *redeemable_assets.assets.get(&USDT).expect("Invalid asset");

		assert_ok!(default_acceptable_computation_error(base_amount, initial_btc));
		assert_ok!(default_acceptable_computation_error(quote_amount, initial_usdt));
	});
}

pub fn create_pool_from_config(init_config: PoolInitConfiguration<u128, u128>) -> u128 {
	Test::assert_extrinsic_event_with(Pablo::create(Origin::root(), init_config), |event| {
		match event {
			crate::Event::PoolCreated { pool_id, .. } => Some(pool_id),
			_ => None,
		}
	})
}

// TODO (vim): Enable when weight validation is done
#[ignore]
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
			LP_TOKEN_ID,
			Permill::from_percent(1),
			Permill::zero(),
		);

		let base_amount = 30_u128 * unit;
		let quote_amount = 1_00_u128 * unit;
		assert_ok!(Tokens::mint_into(USDC, &ALICE, base_amount));
		assert_ok!(Tokens::mint_into(USDT, &ALICE, quote_amount));

		// Add the liquidity, user tries to provide more quote_amount compare to
		// pool's ratio
		assert_noop!(
			<Pablo as Amm>::add_liquidity(
				&ALICE,
				pool,
				BTreeMap::from([(USDC, base_amount), (USDT, quote_amount)]),
				0,
				false
			),
			Error::<Test>::InvalidAmount
		);
	});
}

// test add liquidity with min_mint_amount
#[test]
fn add_lp_with_min_mint_amount() {
	new_test_ext().execute_with(|| {
		next_block::<Pablo, Test>();

		let first_asset = BTC;
		let second_asset = USDT;

		let pool_init_config = PoolInitConfiguration::DualAssetConstantProduct {
			owner: ALICE,
			assets_weights: dual_asset_pool_weights(
				first_asset,
				Permill::from_percent(50_u32),
				second_asset,
			),
			fee: Permill::zero(),
		};

		let init_first_asset_amount = currency::BTC::units(100);
		let init_second_asset_amount = currency::USDT::units(100);
		let first_asset_amount = currency::BTC::units(10);
		let second_asset_amount = currency::USDT::units(10);

		let assets_with_init_amounts = BTreeMap::from([
			(first_asset, init_first_asset_amount),
			(second_asset, init_second_asset_amount),
		]);

		let assets_with_amounts = BTreeMap::from([
			(first_asset, first_asset_amount),
			(second_asset, second_asset_amount),
		]);

		let pool_id = create_pool_from_config(pool_init_config);
		let lp_token = lp_token_of_pool(pool_id);

		// Mint the tokens
		assert_ok!(Tokens::mint_into(first_asset, &ALICE, init_first_asset_amount));
		assert_ok!(Tokens::mint_into(second_asset, &ALICE, init_second_asset_amount));

		// Add the liquidity, min amount = 0
		Test::assert_extrinsic_event(
			Pablo::add_liquidity(
				Origin::signed(ALICE),
				pool_id,
				assets_with_init_amounts.clone(),
				0,
				false,
			),
			crate::Event::<Test>::LiquidityAdded {
				who: ALICE,
				pool_id: 0,
				asset_amounts: assets_with_init_amounts.into(),
				minted_lp: 199_999_999_814_806,
				pool_type: PoolConfigurationType::DualAssetConstantProduct,
			},
		);

		// Mint the tokens
		assert_ok!(Tokens::mint_into(first_asset, &BOB, first_asset_amount));
		assert_ok!(Tokens::mint_into(second_asset, &BOB, second_asset_amount));

		let _alice_lp = Tokens::balance(lp_token, &ALICE);
		let bob_lp = Tokens::balance(lp_token, &BOB);

		assert_eq!(bob_lp, 0_u128, "BOB should not have any LP tokens");

		// no idea what this was calculating, but the following add_liquidity was successful when it
		// should not have been when using this value
		// let min_mint_amount = dbg!(alice_lp) * dbg!(first_asset_amount) /
		// dbg!(init_first_asset_amount); dbg!(min_mint_amount);

		// Add the liquidity, but expect more lp tokens, hence errors
		assert_noop!(
			Pablo::add_liquidity(
				Origin::signed(BOB),
				pool_id,
				assets_with_amounts.clone(),
				// Arbitrarily large number, 200 * 10^12
				200_000_000_000_000,
				false
			),
			crate::Error::<Test>::CannotRespectMinimumRequested
		);

		// Add liquidity with min_mint_amount
		assert_ok!(Pablo::add_liquidity(
			Origin::signed(BOB),
			pool_id,
			assets_with_amounts,
			1,
			false
		));
	});
}

pub fn lp_token_of_pool(pool_id: u128) -> u128 {
	let pool = Pablo::pools(pool_id).expect("pool not found");

	match pool {
		DualAssetConstantProduct(pool) => pool.lp_token,
	}
}

//
// - test error if trying to remove > lp than we have
#[test]
fn remove_lp_failure() {
	new_test_ext().execute_with(|| {
		next_block::<Pablo, Test>();

		let pool_init_config = valid_pool_init_config(
			&ALICE,
			currency::BTC::ID,
			Permill::from_percent(50_u32),
			currency::USDT::ID,
			Permill::zero(),
		);

		let first_asset = currency::BTC::ID;
		let second_asset = currency::USDT::ID;

		let init_first_asset_amount = currency::BTC::units(100);
		let init_second_asset_amount = currency::USDT::units(100);
		let first_asset_amount = currency::BTC::units(10);
		let second_asset_amount = currency::USDT::units(10);

		let pool_id = create_pool_from_config(pool_init_config);

		// Mint the tokens
		assert_ok!(Tokens::mint_into(first_asset, &ALICE, init_first_asset_amount));
		assert_ok!(Tokens::mint_into(second_asset, &ALICE, init_second_asset_amount));

		let lp_token = lp_token_of_pool(pool_id);

		// Add the liquidity
		assert_ok!(Pablo::add_liquidity(
			Origin::signed(ALICE),
			pool_id,
			[(first_asset, init_first_asset_amount), (second_asset, init_second_asset_amount)]
				.into_iter()
				.collect(),
			0, // minimum lp of zero is fine
			false
		));

		// Mint the tokens
		assert_ok!(Tokens::mint_into(first_asset, &BOB, first_asset_amount));
		assert_ok!(Tokens::mint_into(second_asset, &BOB, second_asset_amount));

		let bob_lp_before_adding_liquidity = Tokens::balance(lp_token, &BOB);
		assert_eq!(bob_lp_before_adding_liquidity, 0_u128);

		// Add the liquidity
		assert_ok!(Pablo::add_liquidity(
			Origin::signed(BOB),
			pool_id,
			[(first_asset, first_asset_amount), (second_asset, second_asset_amount)]
				.into_iter()
				.collect(),
			0,
			false
		));

		let bob_lp_after_adding_liquidity = Tokens::balance(lp_token, &BOB);

		// error as trying to redeem more tokens than lp
		assert_noop!(
			Pablo::remove_liquidity(
				Origin::signed(BOB),
				pool_id,
				bob_lp_after_adding_liquidity + 1,
				[(first_asset, 1), (second_asset, 1)].into_iter().collect()
			),
			TokenError::NoFunds
		);

		// error as expected values are more than actual redeemed values.
		assert_noop!(
			Pablo::remove_liquidity(
				Origin::signed(BOB),
				pool_id,
				bob_lp_after_adding_liquidity,
				[
					(first_asset, first_asset_amount + 1_000_000_000_000_000),
					(second_asset, second_asset_amount + 1_000_000_000_000_000)
				]
				.into_iter()
				.collect()
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
		let initial_btc = 1_00_u128 * unit;
		let btc_price = 45_000_u128;
		let initial_usdt = initial_btc * btc_price;
		let pool_init_config = valid_pool_init_config(
			&ALICE,
			BTC,
			Permill::from_percent(50_u32),
			USDT,
			Permill::zero(),
		);
		let exchange_base_amount = 100 * unit;
		common_exchange_failure(
			pool_init_config,
			AssetAmount::new(USDT, initial_usdt),
			AssetAmount::new(BTC, initial_btc),
			AssetAmount::new(BTC, exchange_base_amount),
			LP_TOKEN_ID,
		)
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
		let pool_id = create_pool(
			BTC,
			USDT,
			initial_btc,
			initial_usdt,
			LP_TOKEN_ID,
			Permill::zero(),
			Permill::zero(),
		);
		let bob_btc = 99_u128 * unit;
		// Mint the tokens
		assert_ok!(Tokens::mint_into(BTC, &BOB, bob_btc));

		assert_ok!(<Pablo as Amm>::do_swap(
			&BOB,
			pool_id,
			AssetAmount::new(BTC, bob_btc),
			AssetAmount::new(USDT, 0_u128),
			false
		));
		let usdt_balance = Tokens::balance(USDT, &BOB);
		let idea_usdt_balance = bob_btc * btc_price;
		assert!((idea_usdt_balance - usdt_balance) > 5_u128);
	});
}

//
// - test lp_fee and owner_fee
// TODO (vim): Enable after fee refactor
#[ignore]
#[test]
fn fees() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let initial_btc = 1_00_u128 * unit;
		let btc_price = 45_000_u128;
		let initial_usdt = initial_btc * btc_price;
		let lp_fee = Permill::from_float(0.05);
		let owner_fee = Permill::from_float(0.01);
		let pool_id = create_pool(BTC, USDT, initial_btc, initial_usdt, LP_TOKEN_ID, lp_fee, owner_fee);
		let bob_usdt = 45_000_u128 * unit;
		let quote_usdt = bob_usdt - lp_fee.mul_floor(bob_usdt);
		let expected_btc_value = <Pablo as Amm>::spot_price(pool_id, AssetAmount::new(USDT, quote_usdt), BTC, true)
			.expect("get_exchange_value failed");
		// Mint the tokens
		assert_ok!(Tokens::mint_into(USDT, &BOB, bob_usdt));

		assert_ok!(<Pablo as Amm>::do_buy(&BOB, pool_id, USDT, AssetAmount::new(BTC, expected_btc_value.value.amount), false));
		let price = pallet::prices_for::<Test>(
			pool_id,
			BTC,
			USDT,
			unit,
		).expect("success");
		assert_eq!(price.spot_price, 46_326_729_585_161_862);
		let btc_balance = Tokens::balance(BTC, &BOB);
        sp_std::if_std! {
            println!("expected_btc_value {:?}, btc_balance {:?}", expected_btc_value.value.amount, btc_balance);
        }
		assert_ok!(default_acceptable_computation_error(expected_btc_value.value.amount, btc_balance));
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

// NOTE(connor): Ignored until Pablo depends on pallet-staking
#[ignore]
#[test]
fn staking_pool_test() {
	new_test_ext().execute_with(|| {
	System::set_block_number(1);
		let unit = 1_000_000_000_000_u128;
		let initial_btc = 1_00_u128 * unit;
		let btc_price = 45_000_u128;
		let initial_usdt = initial_btc * btc_price;
		let pool_init_config = valid_pool_init_config(&ALICE, BTC, Permill::from_percent(50_u32), USDT, Permill::from_float(0.05));

		let pool_id = Pablo::do_create_pool(pool_init_config, Some(LP_TOKEN_ID)).expect("pool creation failed");
		// Mint the tokens
		assert_ok!(Tokens::mint_into(BTC, &ALICE, initial_btc));
		assert_ok!(Tokens::mint_into(USDT, &ALICE, initial_usdt));
		// Add the liquidity
		assert_ok!(<Pablo as Amm>::add_liquidity(
			&ALICE,
			pool_id,
			BTreeMap::from([(BTC, initial_btc), (USDT, initial_usdt)]),
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

		assert_ok!(<Pablo as Amm>::do_swap(
			&BOB,
			pool_id,
			AssetAmount::new(USDT, bob_usdt),
			AssetAmount::new(BTC, 0_u128),
			false
		));
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
		let pool_init_config =
			valid_pool_init_config(&ALICE, BTC, Permill::from_percent(50_u32), USDT, lp_fee);
		System::set_block_number(1);
		let pool_id = Pablo::do_create_pool(pool_init_config, Some(LP_TOKEN_ID))
			.expect("pool creation failed");
		let bob_usdt = 45_000_u128 * unit;
		let quote_usdt = bob_usdt - lp_fee.mul_floor(bob_usdt);
		assert_noop!(
			<Pablo as Amm>::do_swap(
				&ALICE,
				pool_id,
				AssetAmount::new(USDT, quote_usdt),
				AssetAmount::new(BTC, 0_u128),
				false
			),
			DispatchError::from(Error::<Test>::NotEnoughLiquidity)
		);
	});
}

#[test]
fn cannot_swap_between_wrong_pairs() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let lp_fee = Permill::from_float(0.05);
		let pool_init_config =
			valid_pool_init_config(&ALICE, BTC, Permill::from_percent(50_u32), USDT, lp_fee);
		System::set_block_number(1);
		let pool_id = Pablo::do_create_pool(pool_init_config, Some(LP_TOKEN_ID))
			.expect("pool creation failed");
		let base_amount = 100_000_u128 * unit;
		let quote_amount = 100_000_u128 * unit;
		assert_ok!(Tokens::mint_into(BTC, &ALICE, base_amount));
		assert_ok!(Tokens::mint_into(USDT, &ALICE, quote_amount));

		assert_ok!(Tokens::mint_into(BTC, &BOB, base_amount));
		assert_ok!(Tokens::mint_into(USDC, &BOB, quote_amount));
		assert_ok!(<Pablo as Amm>::add_liquidity(
			&ALICE,
			pool_id,
			BTreeMap::from([(BTC, base_amount), (USDT, quote_amount)]),
			0,
			false
		));
		let usdc_amount = 2000_u128 * unit;
		assert_noop!(
			Pablo::swap(
				Origin::signed(BOB),
				pool_id,
				AssetAmount::new(USDC, usdc_amount),
				AssetAmount::new(BTC, 0_u128),
				false
			),
			Error::<Test>::AssetNotFound
		);
		assert_noop!(
			Pablo::swap(
				Origin::signed(BOB),
				pool_id,
				AssetAmount::new(BTC, usdc_amount),
				AssetAmount::new(USDC, 0_u128),
				false
			),
			Error::<Test>::AssetNotFound
		);
	});
}

#[test]
fn cannot_get_exchange_value_for_wrong_asset() {
	new_test_ext().execute_with(|| {
		let unit = 1_000_000_000_000_u128;
		let lp_fee = Permill::from_float(0.05);
		let pool_init_config =
			valid_pool_init_config(&ALICE, BTC, Permill::from_percent(50_u32), USDT, lp_fee);
		System::set_block_number(1);
		let pool_id = Pablo::do_create_pool(pool_init_config, Some(LP_TOKEN_ID))
			.expect("pool creation failed");
		let base_amount = 100_000_u128 * unit;
		let quote_amount = 100_000_u128 * unit;
		assert_ok!(Tokens::mint_into(BTC, &ALICE, base_amount));
		assert_ok!(Tokens::mint_into(USDT, &ALICE, quote_amount));

		assert_ok!(<Pablo as Amm>::add_liquidity(
			&ALICE,
			pool_id,
			BTreeMap::from([(BTC, base_amount), (USDT, quote_amount)]),
			0,
			false
		));
		let usdc_amount = 2000_u128 * unit;
		assert_noop!(
			<Pablo as Amm>::spot_price(pool_id, AssetAmount::new(USDC, usdc_amount), BTC, true),
			Error::<Test>::AssetNotFound
		);
	});
}

#[test]
fn weights_zero() {
	new_test_ext().execute_with(|| {
		let pool_init_config =
			valid_pool_init_config(&ALICE, BTC, Permill::zero(), USDT, Permill::zero());
		assert_noop!(
			Pablo::do_create_pool(pool_init_config, Some(LP_TOKEN_ID)),
			Error::<Test>::WeightsMustBeNonZero
		);
	});
}

#[test]
fn weights_sum_to_more_than_one() {
	new_test_ext().execute_with(|| {
		let mut asset_weights = BoundedBTreeMap::new();
		asset_weights.try_insert(BTC, Permill::from_percent(50)).expect("Should work");
		asset_weights.try_insert(USDT, Permill::from_percent(51)).expect("Should work");
		let pool_init_config = PoolInitConfiguration::DualAssetConstantProduct {
			owner: ALICE,
			assets_weights: asset_weights,
			fee: Permill::zero(),
		};

		assert_noop!(
			Pablo::do_create_pool(pool_init_config, Some(LP_TOKEN_ID)),
			Error::<Test>::WeightsMustSumToOne
		);
	});
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(10000))]
	#[test]
	#[ignore = "very broken, unsure what it's testing"]
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
				LP_TOKEN_ID,
				Permill::zero(),
				Permill::zero(),
			);
			prop_assert_ok!(Tokens::mint_into(USDT, &BOB, usdt_value));
			prop_assert_ok!(
				Pablo::swap(
					Origin::signed(BOB),
					pool_id,
					AssetAmount::new(USDT, usdt_value),
					AssetAmount::new(BTC, 0_u128),
					false
				)
			);
			let bob_btc = Tokens::balance(BTC, &BOB);

			// mint extra BTC equal to slippage so that original amount of USDT can be buy back
			prop_assert_ok!(
				Tokens::mint_into(BTC, &BOB, btc_value - bob_btc)
			);
			prop_assert_ok!(
				Pablo::buy(
					Origin::signed(BOB),
					pool_id,
					BTC,
					AssetAmount::new(USDT, usdt_value),
					false
				)
			);
			let bob_usdt = Tokens::balance(USDT, &BOB);

			let slippage = usdt_value -  bob_usdt;
			let slippage_percentage = slippage as f64 * 100.0_f64 / usdt_value as f64;
			prop_assert!(slippage_percentage < 1.0_f64);
			Ok(())
	  })?;
	}

	#[test]
	#[ignore = "very broken, unsure what it's testing"]
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
			LP_TOKEN_ID,
			Permill::zero(),
			Permill::zero(),
		  );
		  let pool = get_pool(pool_id);
		  prop_assert_ok!(Tokens::mint_into(USDT, &BOB, usdt_value));
		  prop_assert_ok!(Tokens::mint_into(BTC, &BOB, btc_value));
		  prop_assert_ok!(Pablo::add_liquidity(Origin::signed(BOB), pool_id,
				BTreeMap::from([(BTC, btc_value), (USDT, usdt_value)]), 0, false));
		  let term1 = initial_usdt.integer_sqrt_checked().expect("integer_sqrt failed");
		  let term2 = initial_btc.integer_sqrt_checked().expect("integer_sqrt failed");
		  let expected_lp_tokens = safe_multiply_by_rational(term1, btc_value, term2).expect("multiply_by_rational failed");
		  let lp_token = Tokens::balance(pool.lp_token, &BOB);
		  prop_assert_ok!(default_acceptable_computation_error(expected_lp_tokens, lp_token));
		  prop_assert_ok!(Pablo::remove_liquidity(Origin::signed(BOB), pool_id, lp_token,
				BTreeMap::from([(USDT, 0), (BTC, 0)])
			));
		  let btc_value_redeemed = Tokens::balance(BTC, &BOB);
		  let usdt_value_redeemed = Tokens::balance(USDT, &BOB);
		  prop_assert_ok!(default_acceptable_computation_error(btc_value_redeemed, btc_value));
		  prop_assert_ok!(default_acceptable_computation_error(usdt_value_redeemed, usdt_value));
		  Ok(())
	  })?;
	}

	#[test]
	#[ignore = "very broken, unsure what it's testing"]
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
			LP_TOKEN_ID,
			Permill::from_float(0.025),
			Permill::zero(),
		  );
		  let pool = get_pool(pool_id);
		  prop_assert_ok!(Tokens::mint_into(USDT, &BOB, usdt_value));
		  prop_assert_ok!(Pablo::swap(Origin::signed(BOB), pool_id, AssetAmount::new(USDT, usdt_value), AssetAmount::new(BTC, 0), false));
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
		base_weight_in_percent in 1..100_u32,
	) {
	  new_test_ext().execute_with(|| {
		let pool_init_config = valid_pool_init_config(&ALICE, BTC, Permill::from_percent(base_weight_in_percent), USDT, Permill::zero());
		  prop_assert_ok!(Pablo::do_create_pool(pool_init_config, Some(LP_TOKEN_ID)));
		  Ok(())
	  })?;
  }
}

mod twap {
	use super::*;
	use crate::types::TimeWeightedAveragePrice;
	use composable_tests_helpers::test::block::process_and_progress_blocks;
	use composable_traits::defi::Rate;
	use sp_runtime::traits::One;

	#[test]
	fn twap_asset_prices_change_after_twap_interval() {
		new_test_ext().execute_with(|| {
			let unit = 1_000_000_000_000_u128;
			let initial_btc = 100_u128 * unit;
			let initial_usdt = 100_u128 * unit;
			let pool_id = create_pool(
				BTC,
				USDT,
				initial_btc,
				initial_usdt,
				LP_TOKEN_ID,
				Permill::zero(),
				Permill::zero(),
			);

			System::set_block_number(0);
			assert_eq!(Pablo::twap(pool_id), None);
			assert_ok!(Pablo::enable_twap(Origin::root(), pool_id));
			process_and_progress_blocks::<Pablo, Test>(1);

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
			process_and_progress_blocks::<Pablo, Test>(TWAP_INTERVAL_BLOCKS.try_into().unwrap());
			// execute a swap to invoke update_twap() however it will only update price_cumulative
			// and not twap as elapsed time is < TWAPInterval
			let usdt_value = unit;
			assert_ok!(Tokens::mint_into(USDT, &BOB, usdt_value));
			assert_ok!(Pablo::swap(
				Origin::signed(BOB),
				pool_id,
				AssetAmount::new(USDT, usdt_value),
				AssetAmount::new(BTC, 0),
				false
			));

			let price_cumulative =
				Pablo::price_cumulative(pool_id).expect("price_cumulative not found");
			assert_eq!(
				price_cumulative.timestamp,
				(TWAP_INTERVAL_BLOCKS + 1) * MILLISECS_PER_BLOCK
			);
			// was previously 131764
			assert_eq!(price_cumulative.base_price_cumulative, 66121);
			// was previously 132242
			assert_eq!(price_cumulative.quote_price_cumulative, 65882);
			// here in on_initialize() TWAP does not get updated as elapsed < TWAPInterval
			// and as TWAP does not get updated, price_cumulative will also not be updated.
			process_and_progress_blocks::<Pablo, Test>(TWAP_INTERVAL_BLOCKS as usize / 2_usize);
			let price_cumulative_new =
				Pablo::price_cumulative(pool_id).expect("price_cumulative not found");
			assert_eq!(price_cumulative.timestamp, price_cumulative_new.timestamp);
			assert_eq!(
				price_cumulative.base_price_cumulative,
				price_cumulative_new.base_price_cumulative
			);
			assert_eq!(
				price_cumulative.quote_price_cumulative,
				price_cumulative_new.quote_price_cumulative
			);

			let elapsed = TWAP_INTERVAL_BLOCKS * MILLISECS_PER_BLOCK;
			let twap = Pablo::twap(pool_id).expect("twap not found");
			assert_eq!(twap.timestamp, elapsed);
			// was previously 120001
			assert_eq!(twap.base_price_cumulative, 60001);
			// was previously 120001
			assert_eq!(twap.quote_price_cumulative, 60001);
			assert_eq!(twap.base_twap, Rate::one());
			assert_eq!(twap.quote_twap, Rate::one());
		});
	}

	#[test]
	#[ignore = "large and broken"]
	fn twap_asset_prices_change_within_twap_interval() {
		new_test_ext().execute_with(|| {
			let unit = 1_000_000_000_000_u128;
			let initial_btc = 100_u128 * unit;
			let initial_usdt = 100_u128 * unit;
			let pool_identifier = create_pool(
				BTC,
				USDT,
				initial_btc,
				initial_usdt,
				LP_TOKEN_ID,
				Permill::zero(),
				Permill::zero(),
			);

			let mut min_base_price = Rate::from_float(99999999.0);
			let mut min_quote_price = Rate::from_float(99999999.0);
			let mut max_base_price = Rate::from_float(0.0);
			let mut max_quote_price = Rate::from_float(0.0);
			let mut update_min_max_price = || {
				let base_price =
					Pablo::do_get_exchange_rate(pool_identifier, crate::PriceRatio::NotSwapped)
						.expect("success");
				let quote_price =
					Pablo::do_get_exchange_rate(pool_identifier, crate::PriceRatio::Swapped)
						.expect("success");
				min_base_price = sp_std::cmp::min(base_price, min_base_price);
				min_quote_price = sp_std::cmp::min(quote_price, min_quote_price);
				max_base_price = sp_std::cmp::max(base_price, max_base_price);
				max_quote_price = sp_std::cmp::max(quote_price, max_quote_price);
			};
			System::set_block_number(0);
			assert_eq!(Pablo::twap(pool_identifier), None);
			assert_ok!(Pablo::enable_twap(Origin::root(), pool_identifier));
			process_and_progress_blocks::<Pablo, Test>(1);

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
				process_and_progress_blocks::<Pablo, Test>(block_number.try_into().unwrap());
				// execute a swap to invoke update_twap() on given block_number
				let usdt_value = unit;
				assert_ok!(Tokens::mint_into(USDT, &BOB, usdt_value));
				assert_ok!(Pablo::swap(
					Origin::signed(BOB),
					pool_identifier,
					AssetAmount::new(USDT, usdt_value),
					AssetAmount::new(BTC, 0),
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

			run_to_block_and_swap(TWAP_INTERVAL_BLOCKS + 1);
			assert_has_event::<Test, _>(|e| {
				matches!(e.event,
				mock::Event::Pablo(crate::Event::TwapUpdated { pool_id, ..})
				if pool_id == pool_identifier
				)
			});
			let price_cumulative =
				Pablo::price_cumulative(pool_identifier).expect("price_cumulative not found");
			assert_eq!(
				price_cumulative.timestamp,
				(TWAP_INTERVAL_BLOCKS + 1) * MILLISECS_PER_BLOCK
			);
			assert_eq!(price_cumulative.base_price_cumulative, 127799);
			assert_eq!(price_cumulative.quote_price_cumulative, 136359);
			update_min_max_price();
			let elapsed = (TWAP_INTERVAL_BLOCKS) * MILLISECS_PER_BLOCK;
			let twap = Pablo::twap(pool_identifier).expect("twap not found");
			assert_eq!(twap.timestamp, elapsed);
			assert!(twap.base_twap > min_base_price);
			assert!(twap.quote_twap > min_quote_price);
			assert!(twap.base_twap < max_base_price);
			assert!(twap.quote_twap < max_quote_price);
		});
	}
}
