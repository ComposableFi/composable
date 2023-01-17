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
			Pablo::add_liquidity(
				RuntimeOrigin::signed(ALICE),
				pool_id,
				assets_with_amounts.clone(),
				0,
				false,
			),
			crate::Event::LiquidityAdded {
				who: ALICE,
				pool_id,
				asset_amounts: assets_with_amounts,
				minted_lp: 199_999_999_814_806,
			},
		);

		// Mint the tokens
		assert_ok!(Tokens::mint_into(first_asset, &BOB, next_first_asset_amount));
		assert_ok!(Tokens::mint_into(second_asset, &BOB, next_second_asset_amount));

		assert_eq!(Tokens::balance(lp_token, &BOB), 0_u128);

		process_and_progress_blocks::<Pablo, Test>(1);

		// Add the liquidity
		Test::assert_extrinsic_event(
			Pablo::add_liquidity(
				RuntimeOrigin::signed(BOB),
				pool_id,
				assets_with_next_amounts.clone(),
				0,
				false,
			),
			crate::Event::LiquidityAdded {
				who: BOB,
				pool_id,
				asset_amounts: assets_with_next_amounts,
				minted_lp: 19999999981480,
			},
		);

		assert!(Tokens::balance(lp_token, &BOB) > 0);

		assert_ok!(Pablo::remove_liquidity(
			RuntimeOrigin::signed(BOB),
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
	use frame_support::assert_noop;

	use super::*;

	/// Bob buys USDT with BTC while paying fees in BTC
	#[test]
	fn should_deduct_fees_from_user_when_non_zero_fees() {
		new_test_ext().execute_with(|| {
			process_and_progress_blocks::<Pablo, Test>(1);

			// Create pool
			let fee = Permill::from_rational::<u32>(3, 1000);
			// 50/50 BTC/USDT Pool with a 0.3% fee
			let init_config = PoolInitConfiguration::DualAssetConstantProduct {
				owner: ALICE,
				assets_weights: dual_asset_pool_weights(BTC, Permill::from_percent(50), USDT),
				fee,
			};
			let pool_id = create_pool_from_config(init_config);

			// Mint tokens for adding liquidity
			let initial_btc = 512_000_000_000;
			let initial_usdt = 512_000_000_000;
			assert_ok!(Tokens::mint_into(BTC, &ALICE, initial_btc));
			assert_ok!(Tokens::mint_into(USDT, &ALICE, initial_usdt));

			// Add liquidity
			assert_ok!(Pablo::add_liquidity(
				RuntimeOrigin::signed(ALICE),
				pool_id,
				BTreeMap::from([(BTC, initial_btc), (USDT, initial_usdt)]),
				0,
				false
			));

			// Mint tokens for buy
			let bob_btc = 256_000;
			assert_ok!(Tokens::mint_into(BTC, &BOB, bob_btc));
			assert_eq!(Tokens::balance(BTC, &BOB), bob_btc);

			// Do buy
			let usdt_to_buy = AssetAmount::new(USDT, 128_000);
			assert_ok!(Pablo::do_buy(&BOB, pool_id, BTC, usdt_to_buy, false));

			let expected_btc_amount = 128_000;
			let expected_fee_amount = fee.mul_ceil(expected_btc_amount);

			// Fees were deducted from user account
			assert!(default_acceptable_computation_error(
				Tokens::balance(BTC, &BOB),
				bob_btc - expected_btc_amount - expected_fee_amount
			)
			.is_ok());
			assert_eq!(Tokens::balance(USDT, &BOB), usdt_to_buy.amount);

			// Fees are in pool account
			assert!(default_acceptable_computation_error(
				Tokens::balance(BTC, &Pablo::account_id(&pool_id)),
				initial_btc + expected_btc_amount + expected_fee_amount
			)
			.is_ok());
		});
	}

	#[test]
	fn cannot_buy_asset_with_itself() {
		new_test_ext().execute_with(|| {
			process_and_progress_blocks::<Pablo, Test>(1);

			// 50/50 BTC/USDT Pool with a 0.3% fee
			let pool_id =
				create_pool_from_config(PoolInitConfiguration::DualAssetConstantProduct {
					owner: ALICE,
					assets_weights: dual_asset_pool_weights(BTC, Permill::from_percent(50), USDT),
					fee: Permill::from_rational::<u32>(3, 1000),
				});

			assert_noop!(
				Pablo::buy(RuntimeOrigin::signed(BOB), pool_id, BTC, AssetAmount::new(BTC, 0), false),
				crate::Error::<Test>::CannotBuyAssetWithItself,
			);
		});
	}
}

mod do_swap {
	use composable_tests_helpers::test::helper::default_acceptable_computation_error;
	use composable_traits::dex::{Amm, AssetAmount};
	use frame_support::assert_noop;

	use super::*;

	/// Bob will swap BTC for USDT with fees included in his swap value
	#[test]
	fn should_deduct_fees_from_user_when_non_zero_fees() {
		new_test_ext().execute_with(|| {
			process_and_progress_blocks::<Pablo, Test>(1);

			// Create pool
			let fee = Permill::from_rational::<u32>(3, 1000);
			// 50/50 BTC/USDT Pool with a 0.3% fee
			let init_config = PoolInitConfiguration::DualAssetConstantProduct {
				owner: ALICE,
				assets_weights: dual_asset_pool_weights(BTC, Permill::from_percent(50), USDT),
				fee,
			};
			let pool_id = create_pool_from_config(init_config);

			// Mint tokens for adding liquidity
			let initial_btc = 512_000_000_000;
			let initial_usdt = 512_000_000_000;
			assert_ok!(Tokens::mint_into(BTC, &ALICE, initial_btc));
			assert_ok!(Tokens::mint_into(USDT, &ALICE, initial_usdt));

			// Add liquidity
			assert_ok!(Pablo::add_liquidity(
				RuntimeOrigin::signed(ALICE),
				pool_id,
				BTreeMap::from([(BTC, initial_btc), (USDT, initial_usdt)]),
				0,
				false
			));

			// Mint tokens for swap
			let bob_btc = 256_000;
			assert_ok!(Tokens::mint_into(BTC, &BOB, bob_btc));
			assert_eq!(Tokens::balance(BTC, &BOB), bob_btc);

			// Do swap
			let btc_to_swap = AssetAmount::new(BTC, 128_000);
			assert_ok!(Pablo::do_swap(
				&BOB,
				pool_id,
				btc_to_swap,
				AssetAmount::new(USDT, 0),
				false
			));

			let expected_fee_amount = fee.mul_ceil(btc_to_swap.amount);
			let expected_usdt_amount = 128_000;

			assert_eq!(Tokens::balance(BTC, &BOB), 128_000);
			// Fees deducted from swap result
			assert!(default_acceptable_computation_error(
				Tokens::balance(USDT, &BOB),
				expected_usdt_amount - expected_fee_amount
			)
			.is_ok());
			assert_eq!(
				Tokens::balance(BTC, &Pablo::account_id(&pool_id)),
				initial_btc + btc_to_swap.amount
			)
		});
	}

	#[test]
	fn cannot_swap_same_asset() {
		new_test_ext().execute_with(|| {
			process_and_progress_blocks::<Pablo, Test>(1);

			// 50/50 BTC/USDT Pool with a 0.3% fee
			let pool_id =
				create_pool_from_config(PoolInitConfiguration::DualAssetConstantProduct {
					owner: ALICE,
					assets_weights: dual_asset_pool_weights(BTC, Permill::from_percent(50), USDT),
					fee: Permill::from_rational::<u32>(3, 1000),
				});

			assert_noop!(
				Pablo::swap(
					RuntimeOrigin::signed(BOB),
					pool_id,
					AssetAmount::new(BTC, 128_000),
					AssetAmount::new(BTC, 0),
					false
				),
				crate::Error::<Test>::CannotSwapSameAsset
			);
		});
	}
}

mod remove_liquidity {
	use composable_traits::dex::{Amm, AssetAmount};

	use super::*;

	/// Zero value `AssetAmount` for BTC
	const ZERO_BTC: AssetAmount<AssetId, Balance> = AssetAmount { asset_id: BTC, amount: 0 };

	/// Zero value `AssetAmount` for USDT
	const ZERO_USDT: AssetAmount<AssetId, Balance> = AssetAmount { asset_id: USDT, amount: 0 };

	/// Create pool with 50/50 BTC/USDT Pool with a 0.3% fee
	/// Mint tokens for adding liquidity in accounts for Alice, Charlie, and 256 other LPs
	/// Add liquidity from Alice, Charlie, and 256 other LPs
	/// Mint tokens for swap into Bob and Dave's accounts
	/// Swap/Buy with Bob and Dave's account
	/// Remove liquidity of both Alice and Charlie
	/// Return total amounts of BTC and USDT in Charlies accounts
	fn remove_liquidity_simulation(with_swaps: bool, with_buys: bool) -> (Balance, Balance) {
		process_and_progress_blocks::<Pablo, Test>(1);

		// Create pool
		let fee = Permill::from_rational::<u32>(3, 1000);
		// 50/50 BTC/USDT Pool with a 0.3% fee
		let init_config = PoolInitConfiguration::DualAssetConstantProduct {
			owner: ALICE,
			assets_weights: dual_asset_pool_weights(BTC, Permill::from_percent(50), USDT),
			fee,
		};
		let pool_id = create_pool_from_config(init_config);
		let lp_token = lp_token_of_pool(pool_id);

		// Mint tokens for adding liquidity
		let initial_btc = 512_000_000_000;
		let initial_usdt = 512_000_000_000;
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
			RuntimeOrigin::signed(ALICE),
			pool_id,
			BTreeMap::from([(BTC, initial_btc), (USDT, initial_usdt)]),
			0,
			false
		));
		// Charlie
		assert_ok!(Pablo::add_liquidity(
			RuntimeOrigin::signed(CHARLIE),
			pool_id,
			BTreeMap::from([(BTC, initial_btc), (USDT, initial_usdt)]),
			0,
			false
		));
		// Other LPs
		other_lps.iter().for_each(|account_id| {
			assert_ok!(Pablo::add_liquidity(
				RuntimeOrigin::signed(*account_id),
				pool_id,
				BTreeMap::from([(BTC, initial_btc), (USDT, initial_usdt)]),
				0,
				false
			));
		});

		let alice_lpt_balance = Tokens::balance(lp_token, &ALICE);
		let charlie_lpt_balance = Tokens::balance(lp_token, &CHARLIE);

		// Mint tokens for swap
		let bob_btc = 256_000_000;
		let dave_usdt = 256_000_000;
		assert_ok!(Tokens::mint_into(BTC, &BOB, bob_btc));
		assert_eq!(Tokens::balance(BTC, &BOB), bob_btc);
		assert_ok!(Tokens::mint_into(USDT, &DAVE, dave_usdt));
		assert_eq!(Tokens::balance(USDT, &DAVE), dave_usdt);

		let btc_to_move = AssetAmount::new(BTC, 128_000_000);
		let usdt_to_move = AssetAmount::new(USDT, 128_000_000);

		if with_swaps {
			assert_ok!(Pablo::do_swap(&BOB, pool_id, btc_to_move, ZERO_USDT, false));
			assert_ok!(Pablo::do_swap(&DAVE, pool_id, usdt_to_move, ZERO_BTC, false));
		}

		if with_buys {
			assert_ok!(Pablo::do_buy(&BOB, pool_id, BTC, usdt_to_move, false));
			assert_ok!(Pablo::do_buy(&DAVE, pool_id, USDT, btc_to_move, false));
		}

		let min_receive = || BTreeMap::from([(BTC, 0), (USDT, 0)]);

		// Remove liquidity
		// Charlie
		let pool_btc_pre_charlie_withdraw = Tokens::balance(BTC, &Pablo::account_id(&pool_id));
		let pool_usdt_pre_charlie_withdraw = Tokens::balance(USDT, &Pablo::account_id(&pool_id));
		let total_lp_pre_charlie_withdraw = Tokens::total_issuance(lp_token);
		assert_ok!(Pablo::remove_liquidity(
			RuntimeOrigin::signed(CHARLIE),
			pool_id,
			charlie_lpt_balance,
			min_receive()
		));
		// Alice
		let pool_btc_pre_alice_withdraw = Tokens::balance(BTC, &Pablo::account_id(&pool_id));
		let pool_usdt_pre_alice_withdraw = Tokens::balance(USDT, &Pablo::account_id(&pool_id));
		let total_lp_pre_alice_withdraw = Tokens::total_issuance(lp_token);
		assert_ok!(Pablo::remove_liquidity(
			RuntimeOrigin::signed(ALICE),
			pool_id,
			alice_lpt_balance,
			min_receive()
		));

		// NOTE: Alice had more LPT as they were the first LP. As it stands, the first LP will be
		// awarded more LPT than the following LPs when the deposit is the same.
		let expected_alice_btc = composable_maths::dex::constant_product::compute_redeemed_for_lp(
			total_lp_pre_alice_withdraw,
			alice_lpt_balance,
			pool_btc_pre_alice_withdraw,
			Permill::one(),
		)
		.expect("input will does not overflow");
		let expected_alice_usdt = composable_maths::dex::constant_product::compute_redeemed_for_lp(
			total_lp_pre_alice_withdraw,
			alice_lpt_balance,
			pool_usdt_pre_alice_withdraw,
			Permill::one(),
		)
		.expect("input will does not overflow");
		let expected_charlie_btc =
			composable_maths::dex::constant_product::compute_redeemed_for_lp(
				total_lp_pre_charlie_withdraw,
				charlie_lpt_balance,
				pool_btc_pre_charlie_withdraw,
				Permill::one(),
			)
			.expect("input will does not overflow");
		let expected_charlie_usdt =
			composable_maths::dex::constant_product::compute_redeemed_for_lp(
				total_lp_pre_charlie_withdraw,
				charlie_lpt_balance,
				pool_usdt_pre_charlie_withdraw,
				Permill::one(),
			)
			.expect("input will does not overflow");

		assert_eq!(Tokens::balance(BTC, &ALICE), expected_alice_btc);
		assert_eq!(Tokens::balance(USDT, &ALICE), expected_alice_usdt);
		assert_eq!(Tokens::balance(BTC, &CHARLIE), expected_charlie_btc);
		assert_eq!(Tokens::balance(USDT, &CHARLIE), expected_charlie_usdt);

		(Tokens::balance(BTC, &CHARLIE), Tokens::balance(USDT, &CHARLIE))
	}

	#[test]
	fn should_distribute_fees_to_lps_after_swaps() {
		let with_swaps = new_test_ext().execute_with(|| remove_liquidity_simulation(true, false));
		let without_swaps =
			new_test_ext().execute_with(|| remove_liquidity_simulation(false, false));

		// Ensure that fees are distributed to LPs by simulating remove liquidity with and without
		// swaps
		assert!(with_swaps.0 > without_swaps.0);
		assert!(with_swaps.1 > without_swaps.1);
	}

	#[test]
	fn should_distribute_fees_to_lps_after_buys() {
		let with_buys = new_test_ext().execute_with(|| remove_liquidity_simulation(false, true));
		let without_buys =
			new_test_ext().execute_with(|| remove_liquidity_simulation(false, false));

		// Ensure that fees are distributed to LPs by simulating remove liquidity with and without
		// buys
		assert!(with_buys.0 > without_buys.0);
		assert!(with_buys.1 > without_buys.1);
	}
}

mod integration {
	use super::*;

	#[test]
	fn split_lp_tokens_then_remove() {
		new_test_ext().execute_with(|| {
			process_and_progress_blocks::<Pablo, Test>(1);

			// Create pool
			let fee = Permill::from_rational::<u32>(3, 1000);
			// 50/50 BTC/USDT Pool with a 0.3% fee
			let init_config = PoolInitConfiguration::DualAssetConstantProduct {
				owner: ALICE,
				assets_weights: dual_asset_pool_weights(BTC, Permill::from_percent(50), USDT),
				fee,
			};
			let pool_id = create_pool_from_config(init_config);
			let lp_token = lp_token_of_pool(pool_id);

			// Mint tokens for adding liquidity
			let initial_btc = 512_000_000_000;
			let initial_usdt = 512_000_000_000;
			// Alice
			assert_ok!(Tokens::mint_into(BTC, &ALICE, initial_btc));
			assert_ok!(Tokens::mint_into(USDT, &ALICE, initial_usdt));

			// Add liquidity
			// Alice
			assert_ok!(Pablo::add_liquidity(
				RuntimeOrigin::signed(ALICE),
				pool_id,
				BTreeMap::from([(BTC, initial_btc), (USDT, initial_usdt)]),
				0,
				false
			));

			let alice_lpt_balance = Tokens::balance(lp_token, &ALICE);
			Tokens::transfer(RuntimeOrigin::signed(ALICE), CHARLIE, lp_token, alice_lpt_balance / 2)
				.expect("Alice has tokens");
			let alice_lpt_balance = Tokens::balance(lp_token, &ALICE);
			let charlie_lpt_balance = Tokens::balance(lp_token, &CHARLIE);

			let min_receive = || BTreeMap::from([(BTC, 0), (USDT, 0)]);

			// Remove liquidity
			// Charlie
			let pool_btc_pre_charlie_withdraw = Tokens::balance(BTC, &Pablo::account_id(&pool_id));
			let pool_usdt_pre_charlie_withdraw =
				Tokens::balance(USDT, &Pablo::account_id(&pool_id));
			let total_lp_pre_charlie_withdraw = Tokens::total_issuance(lp_token);
			let expected_charlie_btc =
				composable_maths::dex::constant_product::compute_redeemed_for_lp(
					total_lp_pre_charlie_withdraw,
					charlie_lpt_balance,
					pool_btc_pre_charlie_withdraw,
					Permill::one(),
				)
				.expect("input will does not overflow");
			let expected_charlie_usdt =
				composable_maths::dex::constant_product::compute_redeemed_for_lp(
					total_lp_pre_charlie_withdraw,
					charlie_lpt_balance,
					pool_usdt_pre_charlie_withdraw,
					Permill::one(),
				)
				.expect("input will does not overflow");
			Test::assert_extrinsic_event(
				Pablo::remove_liquidity(
					RuntimeOrigin::signed(CHARLIE),
					pool_id,
					charlie_lpt_balance,
					min_receive(),
				),
				crate::Event::LiquidityRemoved {
					who: CHARLIE,
					pool_id,
					asset_amounts: BTreeMap::from([
						(BTC, expected_charlie_btc),
						(USDT, expected_charlie_usdt),
					]),
				},
			);
			// Alice
			let pool_btc_pre_alice_withdraw = Tokens::balance(BTC, &Pablo::account_id(&pool_id));
			let pool_usdt_pre_alice_withdraw = Tokens::balance(USDT, &Pablo::account_id(&pool_id));
			let total_lp_pre_alice_withdraw = Tokens::total_issuance(lp_token);
			let expected_alice_btc =
				composable_maths::dex::constant_product::compute_redeemed_for_lp(
					total_lp_pre_alice_withdraw,
					alice_lpt_balance,
					pool_btc_pre_alice_withdraw,
					Permill::one(),
				)
				.expect("input will does not overflow");
			let expected_alice_usdt =
				composable_maths::dex::constant_product::compute_redeemed_for_lp(
					total_lp_pre_alice_withdraw,
					alice_lpt_balance,
					pool_usdt_pre_alice_withdraw,
					Permill::one(),
				)
				.expect("input will does not overflow");
			Test::assert_extrinsic_event(
				Pablo::remove_liquidity(
					RuntimeOrigin::signed(ALICE),
					pool_id,
					alice_lpt_balance,
					min_receive(),
				),
				crate::Event::LiquidityRemoved {
					who: ALICE,
					pool_id,
					asset_amounts: BTreeMap::from([
						(BTC, expected_alice_btc),
						(USDT, expected_alice_usdt),
					]),
				},
			);

			assert_eq!(Tokens::balance(BTC, &ALICE), expected_alice_btc);
			assert_eq!(Tokens::balance(USDT, &ALICE), expected_alice_usdt);
			assert_eq!(Tokens::balance(BTC, &CHARLIE), expected_charlie_btc);
			assert_eq!(Tokens::balance(USDT, &CHARLIE), expected_charlie_usdt);
		})
	}
}
