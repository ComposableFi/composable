use composable_tests_helpers::test::{
	block::{next_block, process_and_progress_blocks},
	currency::{BTC, USDT},
	helper::RuntimeTrait,
};
use frame_support::{
	assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use sp_runtime::Permill;
use sp_std::collections::btree_map::BTreeMap;

//- test lp mint/burn
use crate::{
	mock::*,
	test::{
		common_test_functions::dual_asset_pool_weights,
		dual_asset_constant_product_tests::{create_pool_from_config, lp_token_of_pool},
	},
	PoolInitConfiguration,
};

#[test]
fn add_remove_lp() {
	new_test_ext().execute_with(|| {
		next_block::<Pablo, Test>();

		let first_asset = BTC::ID;
		let second_asset = USDT::ID;

		let init_config = PoolInitConfiguration::DualAssetConstantProduct {
			owner: ALICE,
			assets_weights: dual_asset_pool_weights(
				first_asset,
				Permill::from_percent(50_u32),
				second_asset,
			),
			fee: Permill::zero(),
		};

		let first_asset_amount = BTC::units(100);
		let second_asset_amount = USDT::units(100);
		let next_first_asset_amount = BTC::units(10);
		let next_second_asset_amount = USDT::units(10);

		process_and_progress_blocks::<Pablo, Test>(1);

		let pool_id = create_pool_from_config(init_config);
		let lp_token = lp_token_of_pool(pool_id);

		let assets_with_amounts = BTreeMap::from([
			(first_asset, first_asset_amount),
			(second_asset, second_asset_amount),
		]);

		let assets_with_next_amounts = BTreeMap::from([
			(first_asset, next_first_asset_amount),
			(second_asset, next_second_asset_amount),
		]);

		// Mint the tokens
		assert_ok!(Tokens::mint_into(first_asset, &ALICE, first_asset_amount));
		assert_ok!(Tokens::mint_into(second_asset, &ALICE, second_asset_amount));

		process_and_progress_blocks::<Pablo, Test>(1);

		Test::assert_extrinsic_event(
			Pablo::add_liquidity(Origin::signed(ALICE), pool_id, assets_with_amounts, 0, false),
			crate::Event::LiquidityAdded { who: ALICE, pool_id, minted_lp: 199_999_999_814_806 },
		);

		// Mint the tokens
		assert_ok!(Tokens::mint_into(first_asset, &BOB, next_first_asset_amount));
		assert_ok!(Tokens::mint_into(second_asset, &BOB, next_second_asset_amount));

		assert_eq!(Tokens::balance(lp_token, &BOB), 0_u128);

		process_and_progress_blocks::<Pablo, Test>(1);

		// Add the liquidity
		Test::assert_extrinsic_event(
			Pablo::add_liquidity(Origin::signed(BOB), pool_id, assets_with_next_amounts, 0, false),
			crate::Event::LiquidityAdded { who: BOB, pool_id, minted_lp: 19999999981480 },
		);

		assert!(Tokens::balance(lp_token, &BOB) > 0);

		assert_ok!(Pablo::remove_liquidity(
			Origin::signed(BOB),
			pool_id,
			Tokens::balance(lp_token, &BOB),
			BTreeMap::from([(first_asset, 0_u128), (second_asset, 0_u128)]),
		));

		assert_eq!(Tokens::balance(lp_token, &BOB), 0_u128, "all lp tokens must have been burnt");
	});
}

mod do_buy {
	use composable_tests_helpers::test::helper::default_acceptable_computation_error;
	use composable_traits::dex::{Amm, AssetAmount};

	use super::*;

	#[test]
	fn should_deduct_fees_from_user_when_non_zero_fees() {
		let initial_btc = 512_000_000_000;
		let initial_usdt = 512_000_000_000;
		let fee = Permill::from_rational::<u32>(3, 1000);
		let bob_btc = 256_000;
		let usdt_to_buy = AssetAmount::new(USDT, 128_000);
		let fee_amount = fee.mul_ceil(usdt_to_buy.amount);
		// 50/50 BTC/USDT Pool with a 0.3% fee
		let init_config = PoolInitConfiguration::DualAssetConstantProduct {
			owner: ALICE,
			assets_weights: dual_asset_pool_weights(BTC, Permill::from_percent(50), USDT),
			fee,
		};

		new_test_ext().execute_with(|| {
			process_and_progress_blocks::<Pablo, Test>(1);

			// Create pool
			let pool_id = create_pool_from_config(init_config);

			// Mint tokens for adding liquidity
			assert_ok!(Tokens::mint_into(BTC, &ALICE, initial_btc));
			assert_ok!(Tokens::mint_into(USDT, &ALICE, initial_usdt));

			// Add liquidity
			assert_ok!(Pablo::add_liquidity(
				Origin::signed(ALICE),
				pool_id,
				BTreeMap::from([(BTC, initial_btc), (USDT, initial_usdt)]),
				0,
				false
			));

			// Mint tokens for buy
			assert_ok!(Tokens::mint_into(BTC, &BOB, bob_btc));
			assert_eq!(Tokens::balance(BTC, &BOB), bob_btc);

			assert_ok!(Pablo::do_buy(&BOB, pool_id, BTC, usdt_to_buy, false));

			// Fees deducted from buy amount in
			assert!(default_acceptable_computation_error(
				Tokens::balance(BTC, &BOB),
				bob_btc - usdt_to_buy.amount - fee_amount
			)
			.is_ok());
			assert_eq!(Tokens::balance(USDT, &BOB), usdt_to_buy.amount);
		});
	}
}

mod do_swap {
	use composable_tests_helpers::test::helper::default_acceptable_computation_error;
	use composable_traits::dex::{Amm, AssetAmount};

	use super::*;

	#[test]
	fn should_deduct_fees_from_user_when_non_zero_fees() {
		let initial_btc = 512_000_000_000;
		let initial_usdt = 512_000_000_000;
		let fee = Permill::from_rational::<u32>(3, 1000);
		let bob_btc = 256_000;
		let btc_to_swap = AssetAmount::new(BTC, 128_000);
		let fee_amount = fee.mul_ceil(btc_to_swap.amount);
		// 50/50 BTC/USDT Pool with a 0.3% fee
		let init_config = PoolInitConfiguration::DualAssetConstantProduct {
			owner: ALICE,
			assets_weights: dual_asset_pool_weights(BTC, Permill::from_percent(50), USDT),
			fee,
		};

		new_test_ext().execute_with(|| {
			process_and_progress_blocks::<Pablo, Test>(1);

			// Create pool
			let pool_id = create_pool_from_config(init_config);

			// Mint tokens for adding liquidity
			assert_ok!(Tokens::mint_into(BTC, &ALICE, initial_btc));
			assert_ok!(Tokens::mint_into(USDT, &ALICE, initial_usdt));

			// Add liquidity
			assert_ok!(Pablo::add_liquidity(
				Origin::signed(ALICE),
				pool_id,
				BTreeMap::from([(BTC, initial_btc), (USDT, initial_usdt)]),
				0,
				false
			));

			// Mint tokens for swap
			assert_ok!(Tokens::mint_into(BTC, &BOB, bob_btc));
			assert_eq!(Tokens::balance(BTC, &BOB), bob_btc);

			assert_ok!(Pablo::do_swap(
				&BOB,
				pool_id,
				btc_to_swap,
				AssetAmount::new(USDT, 0),
				false
			));

			assert_eq!(Tokens::balance(BTC, &BOB), 128_000);
			// Fees deducted from swap result
			assert!(default_acceptable_computation_error(
				Tokens::balance(USDT, &BOB),
				bob_btc - btc_to_swap.amount - fee_amount
			)
			.is_ok());
			assert_eq!(
				Tokens::balance(BTC, &Pablo::account_id(&pool_id)),
				initial_btc + btc_to_swap.amount
			)
		});
	}
}

mod remove_liquidity {
	use composable_traits::dex::{Amm, AssetAmount};

	use super::*;

	fn remove_liquidity_simulation(with_swaps: bool) -> (Balance, Balance) {
		let initial_btc = 512_000_000_000;
		let initial_usdt = 512_000_000_000;
		let fee = Permill::from_rational::<u32>(3, 1000);
		let bob_btc = 256_000_000;
		let dave_usdt = 256_000_000;
		let btc_to_swap = AssetAmount::new(BTC, 128_000_000);
		let usdt_to_swap = AssetAmount::new(USDT, 128_000_000);
		// 50/50 BTC/USDT Pool with a 0.3% fee
		let init_config = PoolInitConfiguration::DualAssetConstantProduct {
			owner: ALICE,
			assets_weights: dual_asset_pool_weights(BTC, Permill::from_percent(50), USDT),
			fee,
		};

		process_and_progress_blocks::<Pablo, Test>(1);

		// Create pool
		let pool_id = create_pool_from_config(init_config);
		let lp_token = lp_token_of_pool(pool_id);

		// Mint tokens for adding liquidity
		// Alice
		assert_ok!(Tokens::mint_into(BTC, &ALICE, initial_btc));
		assert_ok!(Tokens::mint_into(USDT, &ALICE, initial_usdt));
		// Charlie
		assert_ok!(Tokens::mint_into(BTC, &CHARLIE, initial_btc));
		assert_ok!(Tokens::mint_into(USDT, &CHARLIE, initial_usdt));
		// Other LPs
		let other_lps: Vec<AccountId> = (256..512)
			.map(|account_id| {
				assert_ok!(Tokens::mint_into(BTC, &account_id, initial_btc));
				assert_ok!(Tokens::mint_into(USDT, &account_id, initial_usdt));
				account_id
			})
			.collect();

		// Add liquidity
		// Alice
		assert_ok!(Pablo::add_liquidity(
			Origin::signed(ALICE),
			pool_id,
			BTreeMap::from([(BTC, initial_btc), (USDT, initial_usdt)]),
			0,
			false
		));
		// Charlie
		assert_ok!(Pablo::add_liquidity(
			Origin::signed(CHARLIE),
			pool_id,
			BTreeMap::from([(BTC, initial_btc), (USDT, initial_usdt)]),
			0,
			false
		));
		// Other LPs
		other_lps.iter().for_each(|account_id| {
			assert_ok!(Pablo::add_liquidity(
				Origin::signed(*account_id),
				pool_id,
				BTreeMap::from([(BTC, initial_btc), (USDT, initial_usdt)]),
				0,
				false
			));
		});

		let alice_lpt_balance = Tokens::balance(lp_token, &ALICE);
		let charlie_lpt_balance = Tokens::balance(lp_token, &CHARLIE);

		// Mint tokens for swap
		assert_ok!(Tokens::mint_into(BTC, &BOB, bob_btc));
		assert_eq!(Tokens::balance(BTC, &BOB), bob_btc);
		assert_ok!(Tokens::mint_into(USDT, &DAVE, dave_usdt));
		assert_eq!(Tokens::balance(USDT, &DAVE), dave_usdt);

		if with_swaps {
			assert_ok!(Pablo::do_swap(
				&BOB,
				pool_id,
				btc_to_swap,
				AssetAmount::new(USDT, 0),
				false
			));
			assert_ok!(Pablo::do_swap(
				&DAVE,
				pool_id,
				usdt_to_swap,
				AssetAmount::new(BTC, 0),
				false
			));
		}

		let min_receive = || BTreeMap::from([(BTC, 0), (USDT, 0)]);

		assert_ok!(Pablo::remove_liquidity(
			Origin::signed(CHARLIE),
			pool_id,
			charlie_lpt_balance,
			min_receive()
		));
		assert_ok!(Pablo::remove_liquidity(
			Origin::signed(ALICE),
			pool_id,
			alice_lpt_balance,
			min_receive()
		));

		assert!(Tokens::balance(BTC, &ALICE) > initial_btc);
		assert!(Tokens::balance(USDT, &ALICE) > initial_usdt);
		assert!(Tokens::balance(BTC, &CHARLIE) > initial_btc);
		assert!(Tokens::balance(USDT, &CHARLIE) > initial_usdt);

		(Tokens::balance(BTC, &CHARLIE), Tokens::balance(USDT, &CHARLIE))
	}

	#[test]
	fn should_distribute_fees_to_lps() {
		let with_swaps = new_test_ext().execute_with(|| remove_liquidity_simulation(true));
		let without_swaps = new_test_ext().execute_with(|| remove_liquidity_simulation(false));

		// Ensure that fees are distributed to LPs by simulating remove liquidity with and without
		// swaps
		assert!(with_swaps.0 > without_swaps.0);
		assert!(with_swaps.1 > without_swaps.1);
	}
}
