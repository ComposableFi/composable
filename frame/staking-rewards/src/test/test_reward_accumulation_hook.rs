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

				let reward = Pallet::<Test>::reward_accumulation_hook_reward_update_calculation(
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

		let reward = Pallet::<Test>::reward_accumulation_hook_reward_update_calculation(
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
// takes about 3 minutes to run
fn test_acumulate_rewards_hook() {
	new_test_ext().execute_with(|| {
		type A = Currency<97, 12>;
		type B = Currency<98, 12>;
		type C = Currency<99, 12>;
		type D = Currency<100, 12>;
		type E = Currency<101, 12>;
		type F = Currency<102, 12>;

		const fn block_seconds(block_number: u64) -> u128 {
			((MILLISECS_PER_BLOCK / 1_000) * block_number) as u128
		}

		const ONE_YEAR_OF_BLOCKS: u64 = 60 * 60 * 24 * 365 / (block_seconds(1) as u64);

		let mut current_block = System::block_number();

		// 0.000_002 A per second, capped at 0.1 A
		// cap will be hit after 50_000 seconds (8334 blocks)
		let a_a_reward_rate = A::units(2) / 1_000_000;
		let a_a_max_rewards = A::units(1) / 10;

		// 0.000_002 B per second, capped at 0.05 B
		// cap will be hit after 25_000 seconds (4167 blocks)
		let a_b_reward_rate = B::units(2) / 1_000_000;
		let a_b_max_rewards = B::units(5) / 100;

		// 2 D per second, capped at 100k D
		// cap will be hit after 5_000 seconds (834 blocks)
		let c_d_reward_rate = D::units(2);
		let c_d_max_rewards = D::units(10_000);

		// 0.005 E per second, capped at 10 E
		// cap will be hit after 2_000 seconds (334 blocks)
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

		fn check_rewards(
			expected: &[(
				Public, // pool owner
				u128,   // pool asset_id
				// pool rewards
				&[(
					u128, // reward_asset_id
					u128, // expected_total_rewards
				)],
			)],
		) {
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

			assert!(all_rewards.is_empty(), "not all pools were tested, missing {all_rewards:#?}");
		}

		fn check_events(mut expected_events: Vec<crate::Event<Test>>) {
			dbg!(System::events());
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
				"not all expected events were emtted, missing {expected_events:#?}",
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
				r#"sanity check; counter and block should be the same at this point.
				found:
				    counter: {counter},
				    block:   {block}"#
			);
		}

		{
			progress_to_block(1, &mut current_block);

			check_rewards(&[
				(
					ALICE,
					A::ID,
					&[
						(A::ID, a_a_reward_rate * block_seconds(current_block)),
						(B::ID, a_b_reward_rate * block_seconds(current_block)),
					],
				),
				(
					BOB,
					C::ID,
					&[
						(D::ID, c_d_reward_rate * block_seconds(current_block)),
						(E::ID, c_e_reward_rate * block_seconds(current_block)),
					],
				),
			]);

			check_events(vec![]);
		}

		{
			progress_to_block(10, &mut current_block);

			check_rewards(&[
				(
					ALICE,
					A::ID,
					&[
						(A::ID, a_a_reward_rate * block_seconds(current_block)),
						(B::ID, a_b_reward_rate * block_seconds(current_block)),
					],
				),
				(
					BOB,
					C::ID,
					&[
						(D::ID, c_d_reward_rate * block_seconds(current_block)),
						(E::ID, c_e_reward_rate * block_seconds(current_block)),
					],
				),
			]);

			check_events(vec![]);
		}

		{
			progress_to_block(334, &mut current_block);

			check_rewards(&[
				(
					ALICE,
					A::ID,
					&[
						(A::ID, a_a_reward_rate * block_seconds(current_block)),
						(B::ID, a_b_reward_rate * block_seconds(current_block)),
					],
				),
				(
					BOB,
					C::ID,
					&[
						(D::ID, c_d_reward_rate * block_seconds(current_block)),
						(E::ID, c_e_max_rewards),
					],
				),
			]);

			check_events(vec![crate::Event::<Test>::MaxRewardsAccumulated {
				pool_id: 2,
				asset_id: E::ID,
			}]);
		}

		{
			progress_to_block(834, &mut current_block);

			check_rewards(&[
				(
					ALICE,
					A::ID,
					&[
						(A::ID, a_a_reward_rate * block_seconds(current_block)),
						(B::ID, a_b_reward_rate * block_seconds(current_block)),
					],
				),
				(BOB, C::ID, &[(D::ID, c_d_max_rewards), (E::ID, c_e_max_rewards)]),
			]);

			check_events(vec![crate::Event::<Test>::MaxRewardsAccumulated {
				pool_id: 2,
				asset_id: D::ID,
			}]);
		}

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

		{
			progress_to_block(4167, &mut current_block);

			check_rewards(&[
				(
					ALICE,
					A::ID,
					&[
						(A::ID, a_a_reward_rate * block_seconds(current_block)),
						(B::ID, a_b_max_rewards),
					],
				),
				(BOB, C::ID, &[(D::ID, c_d_max_rewards), (E::ID, c_e_max_rewards)]),
				(CHARLIE, F::ID, &[(F::ID, 0)]),
			]);

			check_events(vec![crate::Event::<Test>::MaxRewardsAccumulated {
				pool_id: 1,
				asset_id: B::ID,
			}]);
		}

		{
			progress_to_block(8334, &mut current_block);

			check_rewards(&[
				(ALICE, A::ID, &[(A::ID, a_a_max_rewards), (B::ID, a_b_max_rewards)]),
				(BOB, C::ID, &[(D::ID, c_d_max_rewards), (E::ID, c_e_max_rewards)]),
				(CHARLIE, F::ID, &[(F::ID, 0)]),
			]);

			check_events(vec![crate::Event::<Test>::MaxRewardsAccumulated {
				pool_id: 1,
				asset_id: A::ID,
			}]);
		}
	});
}
