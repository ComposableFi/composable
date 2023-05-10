use composable_tests_helpers::test::{
	block::{next_block, process_and_progress_blocks},
	currency::{BTC, USDT},
	helper::RuntimeTrait,
};
use composable_traits::dex::AssetAmount;
use frame_support::{
	assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use proptest::prelude::*;
use sp_runtime::Permill;
use sp_std::collections::btree_map::BTreeMap;

//- test lp mint/burn
use crate::{
	mock::*,
	test::{
		common_test_functions::{dual_asset_pool_weights, dual_asset_pool_weights_vec},
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
			assets_weights: dual_asset_pool_weights_vec(
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

/// Zero value `AssetAmount` for BTC
const ZERO_BTC: AssetAmount<AssetId, Balance> = AssetAmount { asset_id: BTC, amount: 0 };

/// Zero value `AssetAmount` for USDT
const ZERO_USDT: AssetAmount<AssetId, Balance> = AssetAmount { asset_id: USDT, amount: 0 };

const FIRST_ASSET_WEIGHTS: [u32; 6] = [80, 20, 75, 25, 60, 40];

prop_compose! {
	fn first_asset_weight()
	(x in 0..FIRST_ASSET_WEIGHTS.len()) -> Permill {
		Permill::from_percent(FIRST_ASSET_WEIGHTS[x])
	}
}

mod do_buy {
	use composable_maths::dex::constant_product::compute_in_given_out;
	use composable_tests_helpers::test::helper::default_acceptable_computation_error;
	use composable_traits::dex::{Amm, AssetAmount};
	use frame_support::assert_noop;
	use sp_runtime::PerThing;

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
				assets_weights: dual_asset_pool_weights_vec(BTC, Permill::from_percent(50), USDT),
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
					assets_weights: dual_asset_pool_weights_vec(
						BTC,
						Permill::from_percent(50),
						USDT,
					),
					fee: Permill::from_rational::<u32>(3, 1000),
				});

			assert_noop!(
				Pablo::buy(
					RuntimeOrigin::signed(BOB),
					pool_id,
					BTC,
					AssetAmount::new(BTC, 0),
					false
				),
				crate::Error::<Test>::CannotBuyAssetWithItself,
			);
		});
	}

	proptest! {
		#![proptest_config(ProptestConfig::with_cases(FIRST_ASSET_WEIGHTS.len() as u32))]

		/// Ensure the outcome of the `do_buy` function respects token pool weights other than 50/50
		#[test]
		fn should_respect_non_50_50_weight(first_asset_weight in first_asset_weight()) {
			new_test_ext().execute_with(|| {
				process_and_progress_blocks::<Pablo, Test>(1);

				// BTC/USDT Pool with a 0% fee
				let pool_id =
					create_pool_from_config(PoolInitConfiguration::DualAssetConstantProduct {
						owner: ALICE,
						assets_weights: dual_asset_pool_weights_vec(BTC, first_asset_weight, USDT),
						fee: Permill::zero(),
					});

				// Mint tokens for adding liquidity
				let initial_btc = BTC::units(512_000);
				let initial_usdt = USDT::units(512_000);
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
				let bob_btc = BTC::units(256_000);
				assert_ok!(Tokens::mint_into(BTC, &BOB, bob_btc));
				assert_eq!(Tokens::balance(BTC, &BOB), bob_btc);

				// Do buy
				let usdt_to_buy = AssetAmount::new(USDT, USDT::units(128));
				assert_ok!(Pablo::do_buy(&BOB, pool_id, BTC, usdt_to_buy, false));

				let expected_btc_post_buy = bob_btc -
					compute_in_given_out(
						first_asset_weight,
						first_asset_weight.left_from_one(),
						initial_btc,
						initial_usdt,
						usdt_to_buy.amount,
						Permill::zero(),
					)
					.expect("no overflow")
					.value;

				assert_eq!(Tokens::balance(BTC, &BOB), expected_btc_post_buy);
			})
		}
	}
}

mod do_swap {
	use composable_maths::dex::constant_product::compute_out_given_in;
	use composable_tests_helpers::test::helper::default_acceptable_computation_error;
	use composable_traits::dex::{Amm, AssetAmount};
	use frame_support::assert_noop;
	use sp_runtime::PerThing;

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
				assets_weights: dual_asset_pool_weights_vec(BTC, Permill::from_percent(50), USDT),
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
					assets_weights: dual_asset_pool_weights_vec(
						BTC,
						Permill::from_percent(50),
						USDT,
					),
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

	proptest! {
		#![proptest_config(ProptestConfig::with_cases(FIRST_ASSET_WEIGHTS.len() as u32))]

		/// Ensure the outcome of the `do_swap` function respects token pool weights other than 50/50
		#[test]
		fn should_respect_non_50_50_weight(first_asset_weight in first_asset_weight()) {
			new_test_ext().execute_with(|| {
				process_and_progress_blocks::<Pablo, Test>(1);

				// BTC/USDT Pool with a 0% fee
				let pool_id =
					create_pool_from_config(PoolInitConfiguration::DualAssetConstantProduct {
						owner: ALICE,
						assets_weights: dual_asset_pool_weights_vec(BTC, first_asset_weight, USDT),
						fee: Permill::zero(),
					});

				// Mint tokens for adding liquidity
				let initial_btc = BTC::units(512_000);
				let initial_usdt = USDT::units(512_000);
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
				let usdt_to_swap = AssetAmount::new(USDT, USDT::units(256_000));
				assert_ok!(Tokens::mint_into(USDT, &BOB, usdt_to_swap.amount));
				assert_eq!(Tokens::balance(USDT, &BOB), usdt_to_swap.amount);

				// Do swap
				assert_ok!(Pablo::do_swap(&BOB, pool_id, usdt_to_swap, ZERO_BTC, false));

				let expected_btc_post_buy =	compute_out_given_in(
						first_asset_weight.left_from_one(),
						first_asset_weight,
						initial_usdt,
						initial_btc,
						usdt_to_swap.amount,
						Permill::zero(),
					)
					.expect("no overflow")
					.value;

				assert_eq!(Tokens::balance(BTC, &BOB), expected_btc_post_buy);
			})
		}
	}
}

mod add_liquidity {
	use composable_maths::dex::constant_product::compute_deposit_lp;

	use super::*;

	proptest! {
		#![proptest_config(ProptestConfig::with_cases(FIRST_ASSET_WEIGHTS.len() as u32))]

		/// Ensure the outcome of the `do_swap` function respects token pool weights other than 50/50
		#[test]
		fn should_respect_non_50_50_weight_on_second_deposit(first_asset_weight in first_asset_weight()) {
			new_test_ext().execute_with(|| {
				process_and_progress_blocks::<Pablo, Test>(1);

				// BTC/USDT Pool with a 0% fee
				let pool_id =
					create_pool_from_config(PoolInitConfiguration::DualAssetConstantProduct {
						owner: ALICE,
						assets_weights: dual_asset_pool_weights_vec(BTC, first_asset_weight, USDT),
						fee: Permill::zero(),
					});
				let lp_token = lp_token_of_pool(pool_id);
				let pool_account = Pablo::account_id(&pool_id);

				// Mint tokens for adding initial liquidity
				let initial_btc = BTC::units(512_000);
				let initial_usdt = USDT::units(512_000);
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

				// Mint tokens for second deposit
				let second_btc_deposit_max = BTC::units(512_000);
				let second_usdt_deposit_max = USDT::units(512_000);
				assert_ok!(Tokens::mint_into(BTC, &BOB, second_btc_deposit_max));
				assert_ok!(Tokens::mint_into(USDT, &BOB, second_usdt_deposit_max));

				let expected_lp = compute_deposit_lp(
					Tokens::total_issuance(lp_token),
					second_btc_deposit_max,
					Tokens::balance(BTC, &pool_account),
					Permill::one(),
					Permill::zero()
				).expect("no overflow").value;

				// Add liquidity for second deposit
				assert_ok!(Pablo::add_liquidity(
					RuntimeOrigin::signed(BOB),
					pool_id,
					BTreeMap::from([(BTC, second_btc_deposit_max), (USDT, second_usdt_deposit_max)]),
					0,
					false
				));

				assert_eq!(Tokens::balance(lp_token, &BOB), expected_lp);
			})
		}
	}
}

mod remove_liquidity {
	use composable_maths::dex::constant_product::compute_redeemed_for_lp;
	use composable_traits::dex::{Amm, AssetAmount};

	use super::*;

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
			assets_weights: dual_asset_pool_weights_vec(BTC, Permill::from_percent(50), USDT),
			fee,
		};
		let pool_id = create_pool_from_config(init_config);
		let lp_token = lp_token_of_pool(pool_id);
		let pool_account = Pablo::account_id(&pool_id);

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

		// Remove liquidity

		// Charlie
		let expected_charlie_btc = compute_redeemed_for_lp(
			Tokens::total_issuance(lp_token),
			charlie_lpt_balance,
			Tokens::balance(BTC, &pool_account),
			Permill::one(),
		)
		.expect("input will not overflow");
		let expected_charlie_usdt = compute_redeemed_for_lp(
			Tokens::total_issuance(lp_token),
			charlie_lpt_balance,
			Tokens::balance(USDT, &pool_account),
			Permill::one(),
		)
		.expect("input will not overflow");

		assert_ok!(Pablo::remove_liquidity(
			RuntimeOrigin::signed(CHARLIE),
			pool_id,
			charlie_lpt_balance,
			BTreeMap::new()
		));

		// Alice
		// NOTE: Alice had more LPT as they were the first LP. As it stands, the first LP will be
		// awarded more LPT than the following LPs when the deposit is the same.
		let expected_alice_btc = compute_redeemed_for_lp(
			Tokens::total_issuance(lp_token),
			alice_lpt_balance,
			Tokens::balance(BTC, &Pablo::account_id(&pool_id)),
			Permill::one(),
		)
		.expect("input will not overflow");
		let expected_alice_usdt = compute_redeemed_for_lp(
			Tokens::total_issuance(lp_token),
			alice_lpt_balance,
			Tokens::balance(USDT, &Pablo::account_id(&pool_id)),
			Permill::one(),
		)
		.expect("input will not overflow");

		assert_ok!(Pablo::remove_liquidity(
			RuntimeOrigin::signed(ALICE),
			pool_id,
			alice_lpt_balance,
			BTreeMap::new(),
		));

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

	proptest! {
		#![proptest_config(ProptestConfig::with_cases(FIRST_ASSET_WEIGHTS.len() as u32))]

		/// Ensure the outcome of the `remove_liquidity` function respects token pool weights other than 50/50
		#[test]
		fn should_respect_non_50_50_weight(first_asset_weight in first_asset_weight()) {
			new_test_ext().execute_with(|| {
				process_and_progress_blocks::<Pablo, Test>(1);

				// BTC/USDT Pool with a 0% fee
				let pool_id =
					create_pool_from_config(PoolInitConfiguration::DualAssetConstantProduct {
						owner: ALICE,
						assets_weights: dual_asset_pool_weights_vec(BTC, first_asset_weight, USDT),
						fee: Permill::zero(),
					});
				let lp_token = lp_token_of_pool(pool_id);
				let pool_account = Pablo::account_id(&pool_id);

				// Mint tokens for adding liquidity
				let initial_btc = BTC::units(512_000);
				let initial_usdt = USDT::units(512_000);
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
				let alice_lpt_balance = Tokens::balance(lp_token, &ALICE);

				// Do swap
				let usdt_to_swap = AssetAmount::new(USDT, USDT::units(256_000));
				assert_ok!(Tokens::mint_into(USDT, &BOB, usdt_to_swap.amount));
				assert_ok!(Pablo::do_swap(&BOB, pool_id, usdt_to_swap, ZERO_BTC, false));

				// Remove liquidity
				let expected_alice_btc = compute_redeemed_for_lp(
					Tokens::total_issuance(lp_token),
					alice_lpt_balance,
					Tokens::balance(BTC, &pool_account),
					Permill::one(),
				)
				.expect("input will not overflow");
				let expected_alice_usdt = compute_redeemed_for_lp(
					Tokens::total_issuance(lp_token),
					alice_lpt_balance,
					Tokens::balance(USDT, &pool_account),
					Permill::one(),
				)
				.expect("input will not overflow");

				assert_ok!(Pablo::remove_liquidity(
					RuntimeOrigin::signed(ALICE),
					pool_id,
					alice_lpt_balance,
					BTreeMap::new(),
				));

				assert_eq!(Tokens::balance(BTC, &ALICE), expected_alice_btc);
				assert_eq!(Tokens::balance(USDT, &ALICE), expected_alice_usdt);
			})
		}
	}
}

mod integration {
	use composable_maths::dex::constant_product::compute_redeemed_for_lp;

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
				assets_weights: dual_asset_pool_weights_vec(BTC, Permill::from_percent(50), USDT),
				fee,
			};
			let pool_id = create_pool_from_config(init_config);
			let lp_token = lp_token_of_pool(pool_id);
			let pool_account = Pablo::account_id(&pool_id);

			// Mint tokens for adding liquidity
			let initial_btc = BTC::units(512_000);
			let initial_usdt = USDT::units(512_000);
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
			Tokens::transfer(
				RuntimeOrigin::signed(ALICE),
				CHARLIE,
				lp_token,
				alice_lpt_balance / 2,
			)
			.expect("Alice has tokens");
			let alice_lpt_balance = Tokens::balance(lp_token, &ALICE);
			let charlie_lpt_balance = Tokens::balance(lp_token, &CHARLIE);

			// Remove liquidity
			// Charlie
			let expected_charlie_btc = compute_redeemed_for_lp(
				Tokens::total_issuance(lp_token),
				charlie_lpt_balance,
				Tokens::balance(BTC, &pool_account),
				Permill::one(),
			)
			.expect("input will does not overflow");
			let expected_charlie_usdt = compute_redeemed_for_lp(
				Tokens::total_issuance(lp_token),
				charlie_lpt_balance,
				Tokens::balance(USDT, &pool_account),
				Permill::one(),
			)
			.expect("input will does not overflow");

			Test::assert_extrinsic_event(
				Pablo::remove_liquidity(
					RuntimeOrigin::signed(CHARLIE),
					pool_id,
					charlie_lpt_balance,
					BTreeMap::new(),
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
			let expected_alice_btc = compute_redeemed_for_lp(
				Tokens::total_issuance(lp_token),
				alice_lpt_balance,
				Tokens::balance(BTC, &pool_account),
				Permill::one(),
			)
			.expect("input will does not overflow");
			let expected_alice_usdt = compute_redeemed_for_lp(
				Tokens::total_issuance(lp_token),
				alice_lpt_balance,
				Tokens::balance(USDT, &pool_account),
				Permill::one(),
			)
			.expect("input will does not overflow");

			Test::assert_extrinsic_event(
				Pablo::remove_liquidity(
					RuntimeOrigin::signed(ALICE),
					pool_id,
					alice_lpt_balance,
					BTreeMap::new(),
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
