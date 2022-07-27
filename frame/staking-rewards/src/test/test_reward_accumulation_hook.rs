use std::time::{SystemTime, UNIX_EPOCH};

use composable_tests_helpers::test::{
	block::{next_block, process_and_progress_blocks},
	helper::{assert_last_event, assert_no_event},
};

use frame_support::traits::TryCollect;

use crate::{test::prelude::*, Pallet};

use super::*;

#[test]
fn test_reward_update_calculation() {
	new_test_ext().execute_with(|| {
		const SECONDS_PER_BLOCK: u64 = 12;
		const MAX_REWARD_UNITS: u128 = 99;
		// this is arbitrary since there isn't actually a pool, just the reward update calculation
		// is being tested
		const POOL_ID: u16 = 1;

		let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

		let reward = Reward {
			asset_id: PICA::ID,
			total_rewards: 0,
			claimed_rewards: 0,
			total_dilution_adjustment: 0,
			max_rewards: PICA::units(MAX_REWARD_UNITS),
			reward_rate: RewardRate::per_second(PICA::units(2)),
			last_updated_timestamp: now,
		};

		// the expected total_rewards amount (in units) for each block
		let expected = [24, 48, 72, 96];

		let reward = expected.into_iter().zip(1..).fold(
			reward,
			|reward, (expected_total_rewards_units, current_block_number)| {
				System::set_block_number(current_block_number);

				let reward = Pallet::<Test>::reward_acumulation_hook_reward_update_calculation(
					POOL_ID,
					reward,
					now + (SECONDS_PER_BLOCK * current_block_number),
				);

				assert_eq!(reward.total_rewards, PICA::units(expected_total_rewards_units));
				assert_no_event::<Test>(Event::StakingRewards(
					crate::Event::<Test>::RewardAccumulationError {
						pool_id: POOL_ID,
						asset_id: PICA::ID,
					},
				));

				reward
			},
		);

		let current_block = (expected.len() + 1) as u64;

		let reward = Pallet::<Test>::reward_acumulation_hook_reward_update_calculation(
			POOL_ID,
			reward,
			now + (SECONDS_PER_BLOCK * current_block),
		);

		// should be capped at max rewards
		// note that the max rewards is 99 and the reward amount per period is 2 - when the max is
		// reached, as much as possible up to the max amount rewarded (even if it's a smaller
		// increment than amount)
		assert_eq!(reward.total_rewards, PICA::units(MAX_REWARD_UNITS));

		// should report an error since the max was hit
		assert_last_event::<Test>(Event::StakingRewards(
			crate::Event::<Test>::MaxRewardsAccumulated { pool_id: POOL_ID, asset_id: PICA::ID },
		));
	})
}

#[test]
// takes about 25 secs to run
fn test_acumulate_rewards_hook() {
	new_test_ext().execute_with(|| {
		type A = Currency<97, 12>;
		type B = Currency<98, 12>;
		type C = Currency<99, 12>;
		type D = Currency<100, 12>;
		type E = Currency<101, 12>;
		type F = Currency<102, 12>;

		const ONE_YEAR_OF_BLOCKS: u64 = 60 * 60 * 24 * 365 / (blocks(1) as u64);

		let mut current_block = System::block_number();

		// 2 micro-A per second, capped at 10 A
		// cap will be hit after 500_000 seconds
		let a_a_reward_rate = A::units(2) / 1_000_000;
		let a_a_max_rewards = A::units(1);

		// 2 micro-B per second, capped at 0.5 B
		// cap will be hit after 250_000 seconds
		let a_b_reward_rate = B::units(2) / 1_000_000;
		let a_b_max_rewards = B::units(5) / 10;

		// 2 D per second, capped at 100k D
		// cap will be hit after 50_000 seconds
		let c_d_reward_rate = D::units(2);
		let c_d_max_rewards = D::units(100_000);

		// 5 milli-E per second, capped at 10 E
		// cap will be hit after 2_000 seconds
		let c_e_reward_rate = E::units(5) / 1_000;
		let c_e_max_rewards = E::units(10);

		let cfgs = [
			RewardPoolConfiguration::RewardRateBasedIncentive {
				owner: ALICE,
				asset_id: A::ID,
				end_block: current_block + ONE_YEAR_OF_BLOCKS,
				reward_configs: [
					(
						A::ID,
						RewardConfig {
							asset_id: A::ID,
							max_rewards: a_a_max_rewards,
							reward_rate: RewardRate::per_second(a_a_reward_rate),
						},
					),
					(
						B::ID,
						RewardConfig {
							asset_id: B::ID,
							max_rewards: a_b_max_rewards,
							reward_rate: RewardRate::per_second(a_b_reward_rate),
						},
					),
				]
				.into_iter()
				.try_collect()
				.unwrap(),
				lock: default_lock_config(),
			},
			RewardPoolConfiguration::RewardRateBasedIncentive {
				owner: BOB,
				asset_id: C::ID,
				end_block: current_block + ONE_YEAR_OF_BLOCKS,
				reward_configs: [
					(
						D::ID,
						RewardConfig {
							asset_id: D::ID,
							max_rewards: c_d_max_rewards,
							reward_rate: RewardRate::per_second(c_d_reward_rate),
						},
					),
					(
						E::ID,
						RewardConfig {
							asset_id: E::ID,
							max_rewards: c_e_max_rewards,
							reward_rate: RewardRate::per_second(c_e_reward_rate),
						},
					),
				]
				.into_iter()
				.try_collect()
				.unwrap(),
				lock: default_lock_config(),
			},
		];

		for cfg in cfgs {
			StakingRewards::create_reward_pool(Origin::root(), cfg).unwrap();
		}

		fn check_rewards(expected: &[(Public, u128, &[(u128, u128)])]) {
			let mut all_rewards = RewardPools::<Test>::iter().collect::<BTreeMap<_, _>>();

			for ((owner, asset_id, rewards), pool_id) in expected.into_iter().zip(1..) {
				let mut pool = all_rewards
					.remove(&pool_id)
					.expect(&format!("pool {pool_id} not present in RewardPools"));

				assert_eq!(pool.owner, *owner, "error at pool {pool_id}");
				assert_eq!(pool.asset_id, *asset_id, "error at pool {pool_id}");

				for (reward_asset_id, expected_total_rewards) in *rewards {
					let reward = pool.rewards.remove(&reward_asset_id).expect(&format!(
						"reward asset {reward_asset_id} not present in pool {pool_id}"
					));

					assert_eq!(
						reward.asset_id, *reward_asset_id,
						"error at pool {pool_id}, asset {reward_asset_id}",
					);
					assert_eq!(
						reward.total_rewards, *expected_total_rewards,
						"error at pool {pool_id}, asset {reward_asset_id}",
					);
				}

				assert!(
					pool.rewards.is_empty(),
					"not all pool rewards were tested for pool {pool_id}, missing {:#?}",
					pool.rewards
				);
			}

			assert!(
				all_rewards.is_empty(),
				"not all pools were tested, missing {:#?}",
				all_rewards
			);
		}

		const fn blocks(amount: u64) -> u128 {
			((MILLISECS_PER_BLOCK / 1_000) * amount) as u128
		}

		next_block::<StakingRewards, Test>();

		current_block = System::block_number();
		assert_eq!(current_block, 1);

		check_rewards(&[
			(
				ALICE,
				A::ID,
				&[
					(A::ID, a_a_reward_rate * blocks(current_block)),
					(B::ID, a_b_reward_rate * blocks(current_block)),
				],
			),
			(
				BOB,
				C::ID,
				&[
					(D::ID, c_d_reward_rate * blocks(current_block)),
					(E::ID, c_e_reward_rate * blocks(current_block)),
				],
			),
		]);

		process_and_progress_blocks::<StakingRewards, Test>(9);

		current_block = System::block_number();
		assert_eq!(current_block, 10);

		check_rewards(&[
			(
				ALICE,
				A::ID,
				&[
					(A::ID, a_a_reward_rate * blocks(current_block)),
					(B::ID, a_b_reward_rate * blocks(current_block)),
				],
			),
			(
				BOB,
				C::ID,
				&[
					(D::ID, c_d_reward_rate * blocks(current_block)),
					(E::ID, c_e_reward_rate * blocks(current_block)),
				],
			),
		]);

		process_and_progress_blocks::<StakingRewards, Test>(490);

		current_block = System::block_number();
		assert_eq!(current_block, 500);
		// current block: 500

		check_rewards(&[
			(
				ALICE,
				A::ID,
				&[
					(A::ID, a_a_reward_rate * blocks(current_block)),
					(B::ID, a_b_reward_rate * blocks(current_block)),
				],
			),
			(
				BOB,
				C::ID,
				&[(D::ID, c_d_reward_rate * blocks(current_block)), (E::ID, c_e_max_rewards)],
			),
		]);

		process_and_progress_blocks::<StakingRewards, Test>(8000);

		current_block = System::block_number();
		assert_eq!(current_block, 8500);

		check_rewards(&[
			(
				ALICE,
				A::ID,
				&[
					(A::ID, a_a_reward_rate * blocks(current_block)),
					(B::ID, a_b_reward_rate * blocks(current_block)),
				],
			),
			(BOB, C::ID, &[(D::ID, c_d_max_rewards), (E::ID, c_e_max_rewards)]),
		]);

		// add a new, zero-reward pool
		StakingRewards::create_reward_pool(
			Origin::root(),
			RewardPoolConfiguration::RewardRateBasedIncentive {
				owner: CHARLIE,
				asset_id: F::ID,
				end_block: current_block + ONE_YEAR_OF_BLOCKS,
				reward_configs: [(
					F::ID,
					RewardConfig {
						asset_id: F::ID,
						max_rewards: F::units(0xDEADC0DE),
						reward_rate: RewardRate::per_second(0_u128),
					},
				)]
				.into_iter()
				.try_collect()
				.unwrap(),
				lock: default_lock_config(),
			},
		)
		.unwrap();

		process_and_progress_blocks::<StakingRewards, Test>(33500);

		current_block = System::block_number();
		assert_eq!(current_block, 42000);

		check_rewards(&[
			(
				ALICE,
				A::ID,
				&[(A::ID, a_a_reward_rate * blocks(current_block)), (B::ID, a_b_max_rewards)],
			),
			(BOB, C::ID, &[(D::ID, c_d_max_rewards), (E::ID, c_e_max_rewards)]),
			(CHARLIE, F::ID, &[(F::ID, 0)]),
		]);

		process_and_progress_blocks::<StakingRewards, Test>(41334);

		current_block = System::block_number();
		assert_eq!(current_block, 83334);

		check_rewards(&[
			(ALICE, A::ID, &[(A::ID, a_a_max_rewards), (B::ID, a_b_max_rewards)]),
			(BOB, C::ID, &[(D::ID, c_d_max_rewards), (E::ID, c_e_max_rewards)]),
			(CHARLIE, F::ID, &[(F::ID, 0)]),
		]);
	});
}
