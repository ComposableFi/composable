use crate::RewardAccumulationHookError;
use composable_tests_helpers::test::{
	block::process_and_progress_blocks,
	helper::{assert_last_event, assert_no_event},
};
use frame_support::traits::{fungibles::InspectHold, TryCollect, UnixTime};

use crate::test::prelude::*;

use super::*;

type A = Currency<97, 12>;
type B = Currency<98, 12>;
type C = Currency<99, 12>;
type D = Currency<100, 12>;
type E = Currency<101, 12>;
type F = Currency<102, 12>;
type XA = Currency<1097, 12>;
type XC = Currency<1099, 12>;
type XF = Currency<1099, 12>;

#[test]
fn test_reward_update_calculation() {
	new_test_ext().execute_with(|| {
		const MAX_REWARDS: u128 = PICA::units(51);

		// block 0 is weird, start at 1 instead
		process_and_progress_blocks::<crate::Pallet<Test>, Test>(1);

		let now = <<Test as crate::Config>::UnixTime as UnixTime>::now().as_secs();

		let reward_config = RewardConfig {
			max_rewards: MAX_REWARDS,
			reward_rate: RewardRate::per_second(PICA::units(2)),
		};

		// just mint a whole bunch of pica
		mint_assets([ALICE], [PICA::ID], PICA::units(10_000));

		create_rewards_pool_and_assert::<Test, runtime::Event>(RewardRateBasedIncentive {
			owner: ALICE,
			asset_id: PICA::ID,
			start_block: 2,
			end_block: ONE_YEAR_OF_BLOCKS * 10 + 1,
			reward_configs: [(PICA::ID, reward_config)].into_iter().try_collect().unwrap(),
			lock: default_lock_config(),
			share_asset_id: XPICA::ID,
			financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID,
			minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
		});

		add_to_rewards_pot_and_assert(ALICE, PICA::ID, PICA::ID, PICA::units(10_000));

		// the expected total_rewards amount for each block surpassed
		let expected = [
			(1, PICA::units(12)),
			(2, PICA::units(24)),
			(3, PICA::units(36)),
			(4, PICA::units(48)),
		];

		let mut reward = RewardPools::<Test>::get(&PICA::ID)
			.unwrap()
			.rewards
			.get(&PICA::ID)
			.unwrap()
			.clone();

		for (block_number, expected_total_rewards) in expected {
			// to clear events
			process_and_progress_blocks::<crate::Pallet<Test>, Test>(1);

			StakingRewards::reward_accumulation_hook_reward_update_calculation(
				PICA::ID,
				PICA::ID,
				&mut reward,
				now.safe_add(&block_seconds(block_number).try_into().unwrap()).unwrap(),
			);

			println!("blocks surpassed: {}", block_number);
			for error in [
				RewardAccumulationHookError::BackToTheFuture,
				RewardAccumulationHookError::RewardsPotEmpty,
			] {
				assert_no_event::<Test>(Event::StakingRewards(
					crate::Event::<Test>::RewardAccumulationHookError {
						pool_id: PICA::ID,
						asset_id: PICA::ID,
						error,
					},
				));
			}

			assert_no_event::<Test>(Event::StakingRewards(
				crate::Event::<Test>::MaxRewardsAccumulated {
					pool_id: PICA::ID,
					asset_id: PICA::ID,
				},
			));

			assert_eq!(
				reward.total_rewards, expected_total_rewards,
				"blocks surpassed: {block_number}"
			);
		}

		let current_block_number = (expected.len() + 1) as u64;

		StakingRewards::reward_accumulation_hook_reward_update_calculation(
			PICA::ID,
			PICA::ID,
			&mut reward,
			now.safe_add(&block_seconds(current_block_number).try_into().unwrap()).unwrap(),
		);

		// should be capped at max rewards
		// note that the max rewards is 51 and the reward amount per period is 2 - when the max is
		// reached, as much as possible up to the max amount is rewarded (even if it's a smaller
		// increment than amount)
		assert_eq!(reward.total_rewards, MAX_REWARDS);

		// should report an error since the max was hit
		assert_last_event::<Test>(Event::StakingRewards(
			crate::Event::<Test>::MaxRewardsAccumulated { pool_id: PICA::ID, asset_id: PICA::ID },
		));
	})
}

#[test]
fn test_accumulate_rewards_pool_empty_refill() {
	new_test_ext().execute_with(|| {
		const STARTING_BLOCK: u64 = 10;
		type A = Currency<97, 12>;
		type XA = Currency<1097, 12>;
		type B = Currency<98, 12>;
		type C = Currency<99, 12>;
		type XC = Currency<1099, 12>;
		type D = Currency<100, 12>;
		type E = Currency<101, 12>;
		type F = Currency<102, 12>;
		type XF = Currency<1102, 12>;

		let mut current_block = System::block_number();

		progress_to_block(STARTING_BLOCK, &mut current_block);

		// 0.000_02 A per second, capped at 0.1 A
		// cap will be hit after 5_000 seconds (834 blocks)
		const A_A_REWARD_RATE: u128 = A::units(2) / 100_000;
		const A_A_MAX_REWARDS: u128 = A::units(1) / 10;

		// 0.000_02 B per second, capped at 0.05 B
		// cap will be hit after 2_500 seconds (417 blocks)
		const A_B_REWARD_RATE: u128 = B::units(2) / 100_000;
		const A_B_MAX_REWARDS: u128 = B::units(5) / 100;

		mint_assets([ALICE], [A::ID], A::units(10_000));
		mint_assets([ALICE], [B::ID], B::units(10_000));

		create_rewards_pool_and_assert::<Test, runtime::Event>(
			RewardPoolConfiguration::RewardRateBasedIncentive {
				owner: ALICE,
				asset_id: A::ID,
				start_block: current_block + 1,
				end_block: current_block + ONE_YEAR_OF_BLOCKS + 1,
				reward_configs: [
					(
						A::ID,
						RewardConfig {
							max_rewards: A_A_MAX_REWARDS,
							reward_rate: RewardRate::per_second(A_A_REWARD_RATE),
						},
					),
					(
						B::ID,
						RewardConfig {
							max_rewards: A_B_MAX_REWARDS,
							reward_rate: RewardRate::per_second(A_B_REWARD_RATE),
						},
					),
				]
				.into_iter()
				.try_collect()
				.unwrap(),
				lock: default_lock_config(),
				share_asset_id: XA::ID,
				financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID,
				minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
			},
		);

		progress_to_block(current_block + 1, &mut current_block);

		check_events([
			crate::Event::<Test>::RewardAccumulationHookError {
				pool_id: A::ID,
				asset_id: A::ID,
				error: RewardAccumulationHookError::RewardsPotEmpty,
			},
			crate::Event::<Test>::RewardAccumulationHookError {
				pool_id: A::ID,
				asset_id: B::ID,
				error: RewardAccumulationHookError::RewardsPotEmpty,
			},
		]);

		progress_to_block(current_block + 1, &mut current_block);

		// RewardsPotEmpty event should only be emitted once, not every block
		check_events([]);

		add_to_rewards_pot_and_assert(ALICE, A::ID, A::ID, A_A_REWARD_RATE * block_seconds(5));

		check_events([crate::Event::<Test>::RewardsPotIncreased {
			pool_id: A::ID,
			asset_id: A::ID,
			amount: A_A_REWARD_RATE * block_seconds(5),
		}]);

		progress_to_block(STARTING_BLOCK + 5, &mut current_block);

		check_rewards(&[CheckRewards {
			owner: ALICE,
			pool_asset_id: A::ID,
			pool_rewards: &[
				PoolRewards {
					reward_asset_id: A::ID,
					expected_total_rewards: A_A_REWARD_RATE * block_seconds(5),
					expected_locked_balance: 0,
					expected_unlocked_balance: A_A_REWARD_RATE * block_seconds(5),
				},
				PoolRewards {
					reward_asset_id: B::ID,
					expected_total_rewards: 0,
					expected_locked_balance: 0,
					expected_unlocked_balance: 0,
				},
			],
		}]);

		check_events([]);

		assert_eq!(current_block, 15);

		progress_to_block(current_block + 1, &mut current_block);

		check_events([crate::Event::<Test>::RewardAccumulationHookError {
			pool_id: A::ID,
			asset_id: A::ID,
			error: RewardAccumulationHookError::RewardsPotEmpty,
		}]);
	});
}

#[test]
// takes about 3 minutes to run in debug, 20 seconds in release
// does not do any claiming
fn test_accumulate_rewards_hook() {
	new_test_ext().execute_with(|| {
		const STARTING_BLOCK: u64 = 10;

		let mut current_block = System::block_number();

		progress_to_block(STARTING_BLOCK, &mut current_block);

		// 0.000_002 A per second, capped at 0.1 A
		// cap will be hit after 50_000 seconds (8334 blocks)
		const A_A_REWARD_RATE: u128 = A::units(2) / 1_000_000;
		const A_A_MAX_REWARDS: u128 = A::units(1) / 10;
		const A_A_INITIAL_AMOUNT: u128 = A::units(1_000_000);

		// 0.000_002 B per second, capped at 0.05 B
		// cap will be hit after 25_000 seconds (4167 blocks)
		const A_B_REWARD_RATE: u128 = B::units(2) / 1_000_000;
		const A_B_MAX_REWARDS: u128 = B::units(5) / 100;
		const A_B_INITIAL_AMOUNT: u128 = A::units(1_000_000);

		// 2 D per second, capped at 100k D
		// cap will be hit after 5_000 seconds (834 blocks)
		const C_D_REWARD_RATE: u128 = D::units(2);
		const C_D_MAX_REWARDS: u128 = D::units(10_000);
		const C_D_INITIAL_AMOUNT: u128 = A::units(1_000_000);

		// 0.005 E per second, capped at 10 E
		// cap will be hit after 2_000 seconds (334 blocks)
		const C_E_REWARD_RATE: u128 = E::units(5) / 1_000;
		const C_E_MAX_REWARDS: u128 = E::units(10);
		const C_E_INITIAL_AMOUNT: u128 = A::units(1_000_000);

		const ALICES_POOL_ID: u128 = A::ID;

		create_rewards_pool_and_assert::<Test, runtime::Event>(
			RewardPoolConfiguration::RewardRateBasedIncentive {
				owner: ALICE,
				asset_id: ALICES_POOL_ID,
				start_block: current_block + 1,
				end_block: current_block + ONE_YEAR_OF_BLOCKS + 1,
				reward_configs: [
					(
						A::ID,
						RewardConfig {
							max_rewards: A_A_MAX_REWARDS,
							reward_rate: RewardRate::per_second(A_A_REWARD_RATE),
						},
					),
					(
						B::ID,
						RewardConfig {
							max_rewards: A_B_MAX_REWARDS,
							reward_rate: RewardRate::per_second(A_B_REWARD_RATE),
						},
					),
				]
				.into_iter()
				.try_collect()
				.unwrap(),
				lock: default_lock_config(),
				share_asset_id: XA::ID,
				financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID,
				minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
			},
		);

		mint_assets([ALICE], [A::ID], A_A_INITIAL_AMOUNT);
		add_to_rewards_pot_and_assert(ALICE, ALICES_POOL_ID, A::ID, A_A_INITIAL_AMOUNT);
		mint_assets([ALICE], [B::ID], A_B_INITIAL_AMOUNT);
		add_to_rewards_pot_and_assert(ALICE, ALICES_POOL_ID, B::ID, A_B_INITIAL_AMOUNT);

		const BOBS_POOL_ID: u128 = C::ID;

		create_rewards_pool_and_assert::<Test, runtime::Event>(
			RewardPoolConfiguration::RewardRateBasedIncentive {
				owner: BOB,
				asset_id: BOBS_POOL_ID,
				start_block: current_block + 1,
				end_block: current_block + ONE_YEAR_OF_BLOCKS + 1,
				reward_configs: [
					(
						D::ID,
						RewardConfig {
							max_rewards: C_D_MAX_REWARDS,
							reward_rate: RewardRate::per_second(C_D_REWARD_RATE),
						},
					),
					(
						E::ID,
						RewardConfig {
							max_rewards: C_E_MAX_REWARDS,
							reward_rate: RewardRate::per_second(C_E_REWARD_RATE),
						},
					),
				]
				.into_iter()
				.try_collect()
				.unwrap(),
				lock: default_lock_config(),
				share_asset_id: XC::ID,
				financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID + 1,
				minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
			},
		);

		mint_assets([ALICE], [D::ID], C_D_INITIAL_AMOUNT);
		add_to_rewards_pot_and_assert(ALICE, BOBS_POOL_ID, D::ID, C_D_INITIAL_AMOUNT);
		mint_assets([ALICE], [E::ID], C_E_INITIAL_AMOUNT);
		add_to_rewards_pot_and_assert(ALICE, BOBS_POOL_ID, E::ID, C_E_INITIAL_AMOUNT);

		{
			progress_to_block(STARTING_BLOCK + 2, &mut current_block);

			check_rewards(&[
				CheckRewards {
					owner: ALICE,
					pool_asset_id: A::ID,
					pool_rewards: &[
						PoolRewards {
							reward_asset_id: A::ID,
							expected_total_rewards: A_A_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
							expected_locked_balance: A_A_INITIAL_AMOUNT -
								(A_A_REWARD_RATE * block_seconds(current_block - STARTING_BLOCK)),
							expected_unlocked_balance: A_A_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
						},
						PoolRewards {
							reward_asset_id: B::ID,
							expected_total_rewards: A_B_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
							expected_locked_balance: A_B_INITIAL_AMOUNT -
								(A_B_REWARD_RATE * block_seconds(current_block - STARTING_BLOCK)),
							expected_unlocked_balance: A_B_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
						},
					],
				},
				CheckRewards {
					owner: BOB,
					pool_asset_id: C::ID,
					pool_rewards: &[
						PoolRewards {
							reward_asset_id: D::ID,
							expected_total_rewards: C_D_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
							expected_locked_balance: C_D_INITIAL_AMOUNT -
								(C_D_REWARD_RATE * block_seconds(current_block - STARTING_BLOCK)),
							expected_unlocked_balance: C_D_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
						},
						PoolRewards {
							reward_asset_id: E::ID,
							expected_total_rewards: C_E_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
							expected_locked_balance: C_E_INITIAL_AMOUNT -
								(C_E_REWARD_RATE * block_seconds(current_block - STARTING_BLOCK)),
							expected_unlocked_balance: C_E_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
						},
					],
				},
			]);

			check_events([]);
		}

		{
			progress_to_block(STARTING_BLOCK + 10, &mut current_block);

			check_rewards(&[
				CheckRewards {
					owner: ALICE,
					pool_asset_id: A::ID,
					pool_rewards: &[
						PoolRewards {
							reward_asset_id: A::ID,
							expected_total_rewards: A_A_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
							expected_locked_balance: A_A_INITIAL_AMOUNT -
								(A_A_REWARD_RATE * block_seconds(current_block - STARTING_BLOCK)),
							expected_unlocked_balance: A_A_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
						},
						PoolRewards {
							reward_asset_id: B::ID,
							expected_total_rewards: A_B_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
							expected_locked_balance: A_B_INITIAL_AMOUNT -
								(A_B_REWARD_RATE * block_seconds(current_block - STARTING_BLOCK)),
							expected_unlocked_balance: A_B_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
						},
					],
				},
				CheckRewards {
					owner: BOB,
					pool_asset_id: C::ID,
					pool_rewards: &[
						PoolRewards {
							reward_asset_id: D::ID,
							expected_total_rewards: C_D_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
							expected_locked_balance: C_D_INITIAL_AMOUNT -
								(C_D_REWARD_RATE * block_seconds(current_block - STARTING_BLOCK)),
							expected_unlocked_balance: C_D_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
						},
						PoolRewards {
							reward_asset_id: E::ID,
							expected_total_rewards: C_E_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
							expected_locked_balance: C_E_INITIAL_AMOUNT -
								(C_E_REWARD_RATE * block_seconds(current_block - STARTING_BLOCK)),
							expected_unlocked_balance: C_E_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
						},
					],
				},
			]);

			check_events([]);
		}

		{
			progress_to_block(STARTING_BLOCK + 334, &mut current_block);

			check_rewards(&[
				CheckRewards {
					owner: ALICE,
					pool_asset_id: A::ID,
					pool_rewards: &[
						PoolRewards {
							reward_asset_id: A::ID,
							expected_total_rewards: A_A_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
							expected_locked_balance: A_A_INITIAL_AMOUNT -
								(A_A_REWARD_RATE * block_seconds(current_block - STARTING_BLOCK)),
							expected_unlocked_balance: A_A_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
						},
						PoolRewards {
							reward_asset_id: B::ID,
							expected_total_rewards: A_B_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
							expected_locked_balance: A_B_INITIAL_AMOUNT -
								(A_B_REWARD_RATE * block_seconds(current_block - STARTING_BLOCK)),
							expected_unlocked_balance: A_B_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
						},
					],
				},
				CheckRewards {
					owner: BOB,
					pool_asset_id: C::ID,
					pool_rewards: &[
						PoolRewards {
							reward_asset_id: D::ID,
							expected_total_rewards: C_D_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
							expected_locked_balance: C_D_INITIAL_AMOUNT -
								(C_D_REWARD_RATE * block_seconds(current_block - STARTING_BLOCK)),
							expected_unlocked_balance: C_D_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
						},
						PoolRewards {
							reward_asset_id: E::ID,
							expected_total_rewards: C_E_MAX_REWARDS,
							expected_locked_balance: C_E_INITIAL_AMOUNT - C_E_MAX_REWARDS,
							expected_unlocked_balance: C_E_MAX_REWARDS,
						},
					],
				},
			]);

			check_events([crate::Event::<Test>::MaxRewardsAccumulated {
				pool_id: C::ID,
				asset_id: E::ID,
			}]);
		}

		{
			progress_to_block(STARTING_BLOCK + 834, &mut current_block);

			check_rewards(&[
				CheckRewards {
					owner: ALICE,
					pool_asset_id: A::ID,
					pool_rewards: &[
						PoolRewards {
							reward_asset_id: A::ID,
							expected_total_rewards: A_A_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
							expected_locked_balance: A_A_INITIAL_AMOUNT -
								(A_A_REWARD_RATE * block_seconds(current_block - STARTING_BLOCK)),
							expected_unlocked_balance: A_A_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
						},
						PoolRewards {
							reward_asset_id: B::ID,
							expected_total_rewards: A_B_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
							expected_locked_balance: A_B_INITIAL_AMOUNT -
								(A_B_REWARD_RATE * block_seconds(current_block - STARTING_BLOCK)),
							expected_unlocked_balance: A_B_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
						},
					],
				},
				CheckRewards {
					owner: BOB,
					pool_asset_id: C::ID,
					pool_rewards: &[
						PoolRewards {
							reward_asset_id: D::ID,
							expected_total_rewards: C_D_MAX_REWARDS,
							expected_locked_balance: C_D_INITIAL_AMOUNT - C_D_MAX_REWARDS,
							expected_unlocked_balance: C_D_MAX_REWARDS,
						},
						PoolRewards {
							reward_asset_id: E::ID,
							expected_total_rewards: C_E_MAX_REWARDS,
							expected_locked_balance: C_E_INITIAL_AMOUNT - C_E_MAX_REWARDS,
							expected_unlocked_balance: C_E_MAX_REWARDS,
						},
					],
				},
			]);

			check_events([crate::Event::<Test>::MaxRewardsAccumulated {
				pool_id: C::ID,
				asset_id: D::ID,
			}]);
		}

		// add a new, zero-reward pool
		// nothing needs to be added to the rewards pot as there are no rewards
		create_rewards_pool_and_assert::<Test, runtime::Event>(
			RewardPoolConfiguration::RewardRateBasedIncentive {
				owner: CHARLIE,
				asset_id: F::ID,
				start_block: current_block + 1,
				end_block: current_block + ONE_YEAR_OF_BLOCKS + 1,
				reward_configs: [(
					F::ID,
					RewardConfig {
						max_rewards: F::units(0xDEADC0DE),
						reward_rate: RewardRate::per_second(0_u128),
					},
				)]
				.into_iter()
				.try_collect()
				.unwrap(),
				lock: default_lock_config(),
				share_asset_id: XF::ID,
				financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID + 2,
				minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
			},
		);

		{
			progress_to_block(STARTING_BLOCK + 4167, &mut current_block);

			check_rewards(&[
				CheckRewards {
					owner: ALICE,
					pool_asset_id: A::ID,
					pool_rewards: &[
						PoolRewards {
							reward_asset_id: A::ID,
							expected_total_rewards: A_A_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
							expected_locked_balance: A_A_INITIAL_AMOUNT -
								(A_A_REWARD_RATE * block_seconds(current_block - STARTING_BLOCK)),
							expected_unlocked_balance: A_A_REWARD_RATE *
								block_seconds(current_block - STARTING_BLOCK),
						},
						PoolRewards {
							reward_asset_id: B::ID,
							expected_total_rewards: A_B_MAX_REWARDS,
							expected_locked_balance: A_B_INITIAL_AMOUNT - A_B_MAX_REWARDS,
							expected_unlocked_balance: A_B_MAX_REWARDS,
						},
					],
				},
				CheckRewards {
					owner: BOB,
					pool_asset_id: C::ID,
					pool_rewards: &[
						PoolRewards {
							reward_asset_id: D::ID,
							expected_total_rewards: C_D_MAX_REWARDS,
							expected_locked_balance: C_D_INITIAL_AMOUNT - C_D_MAX_REWARDS,
							expected_unlocked_balance: C_D_MAX_REWARDS,
						},
						PoolRewards {
							reward_asset_id: E::ID,
							expected_total_rewards: C_E_MAX_REWARDS,
							expected_locked_balance: C_E_INITIAL_AMOUNT - C_E_MAX_REWARDS,
							expected_unlocked_balance: C_E_MAX_REWARDS,
						},
					],
				},
				CheckRewards {
					owner: CHARLIE,
					pool_asset_id: F::ID,
					pool_rewards: &[PoolRewards {
						reward_asset_id: F::ID,
						expected_total_rewards: 0,
						expected_locked_balance: 0,
						expected_unlocked_balance: 0,
					}],
				},
			]);

			check_events([crate::Event::<Test>::MaxRewardsAccumulated {
				pool_id: A::ID,
				asset_id: B::ID,
			}]);
		}

		{
			progress_to_block(STARTING_BLOCK + 8334, &mut current_block);

			check_rewards(&[
				CheckRewards {
					owner: ALICE,
					pool_asset_id: A::ID,
					pool_rewards: &[
						PoolRewards {
							reward_asset_id: A::ID,
							expected_total_rewards: A_A_MAX_REWARDS,
							expected_locked_balance: A_A_INITIAL_AMOUNT - A_A_MAX_REWARDS,
							expected_unlocked_balance: A_A_MAX_REWARDS,
						},
						PoolRewards {
							reward_asset_id: B::ID,
							expected_total_rewards: A_B_MAX_REWARDS,
							expected_locked_balance: A_B_INITIAL_AMOUNT - A_B_MAX_REWARDS,
							expected_unlocked_balance: A_B_MAX_REWARDS,
						},
					],
				},
				CheckRewards {
					owner: BOB,
					pool_asset_id: C::ID,
					pool_rewards: &[
						PoolRewards {
							reward_asset_id: D::ID,
							expected_total_rewards: C_D_MAX_REWARDS,
							expected_locked_balance: C_D_INITIAL_AMOUNT - C_D_MAX_REWARDS,
							expected_unlocked_balance: C_D_MAX_REWARDS,
						},
						PoolRewards {
							reward_asset_id: E::ID,
							expected_total_rewards: C_E_MAX_REWARDS,
							expected_locked_balance: C_E_INITIAL_AMOUNT - C_E_MAX_REWARDS,
							expected_unlocked_balance: C_E_MAX_REWARDS,
						},
					],
				},
				CheckRewards {
					owner: CHARLIE,
					pool_asset_id: F::ID,
					pool_rewards: &[PoolRewards {
						reward_asset_id: F::ID,
						expected_total_rewards: 0,
						expected_locked_balance: 0,
						expected_unlocked_balance: 0,
					}],
				},
			]);

			check_events([crate::Event::<Test>::MaxRewardsAccumulated {
				pool_id: A::ID,
				asset_id: A::ID,
			}]);
		}
	});
}

fn check_events(expected_events: impl IntoIterator<Item = crate::Event<Test>>) {
	// dbg!(System::events());
	let mut expected_events = expected_events.into_iter().collect::<Vec<_>>();
	for record in System::events() {
		match record.event {
			Event::StakingRewards(staking_event) => {
				let idx = expected_events
					.iter()
					.position(|e| e.eq(&&staking_event))
					.expect(&format!("unexpected event: {staking_event:#?}"));

				expected_events.remove(idx);
			},
			_ => {},
		}
	}

	assert!(
		expected_events.is_empty(),
		"not all expected events were emitted, missing {expected_events:#?}",
	);
}

fn progress_to_block(block: u64, counter: &mut u64) {
	assert!(
		block > *counter,
		"cannot progress backwards: currently at {counter}, requested {block}"
	);

	let new_blocks = block - *counter;
	process_and_progress_blocks::<StakingRewards, Test>(new_blocks.try_into().unwrap());

	*counter = System::block_number();
	assert_eq!(
		*counter, block,
		r#"
sanity check; counter and block should be the same at this point.
found:
counter: {counter},
block:   {block}"#
	);

	println!("current block: {counter}");
}

pub(crate) fn check_rewards(expected: &[CheckRewards<'_>]) {
	let mut all_rewards = RewardPools::<Test>::iter().collect::<BTreeMap<_, _>>();

	for CheckRewards { owner, pool_asset_id, pool_rewards } in expected.into_iter() {
		let mut pool = all_rewards
			.remove(&pool_asset_id)
			.expect(&format!("pool {pool_asset_id} not present in RewardPools"));

		assert_eq!(pool.owner, *owner, "error at pool {pool_asset_id}");

		let pool_account = StakingRewards::pool_account_id(pool_asset_id);

		for PoolRewards {
			reward_asset_id,
			expected_total_rewards,
			expected_locked_balance,
			expected_unlocked_balance,
		} in *pool_rewards
		{
			let actual_locked_balance = <<Test as crate::Config>::Assets as InspectHold<
				<Test as frame_system::Config>::AccountId,
			>>::balance_on_hold(*reward_asset_id, &pool_account);

			let actual_unlocked_balance =
				balance(*reward_asset_id, &pool_account) - actual_locked_balance;

			let reward = pool.rewards.remove(&reward_asset_id).expect(&format!(
				"reward asset {reward_asset_id} not present in pool {pool_asset_id}"
			));

			assert!(
				&reward.total_rewards == expected_total_rewards,
				r#"
error at pool {pool_asset_id}, asset {reward_asset_id}: unexpected total_rewards:
	expected: {expected_total_rewards}
	found:    {found_total_rewards}"#,
				found_total_rewards = reward.total_rewards
			);

			assert!(
				&actual_locked_balance == expected_locked_balance,
				r#"
error at pool {pool_asset_id}, asset {reward_asset_id}: unexpected locked balance:
	expected: {expected_locked_balance}
	found:    {actual_locked_balance}"#
			);

			assert!(
				&actual_unlocked_balance == expected_unlocked_balance,
				r#"
error at pool {pool_asset_id}, asset {reward_asset_id}: unexpected unlocked balance:
	expected: {expected_unlocked_balance}
	found:    {actual_unlocked_balance}"#
			);
		}

		assert!(
			pool.rewards.is_empty(),
			"not all pool rewards were tested for pool {pool_asset_id}, missing {:#?}",
			pool.rewards
		);
	}

	assert!(all_rewards.is_empty(), "not all pools were tested, missing {all_rewards:#?}");
}

pub(crate) struct CheckRewards<'a> {
	pub(crate) owner: Public,
	pub(crate) pool_asset_id: u128,
	pub(crate) pool_rewards: &'a [PoolRewards],
}

pub(crate) struct PoolRewards {
	pub(crate) reward_asset_id: u128,
	pub(crate) expected_total_rewards: u128,
	pub(crate) expected_locked_balance: u128,
	pub(crate) expected_unlocked_balance: u128,
}
