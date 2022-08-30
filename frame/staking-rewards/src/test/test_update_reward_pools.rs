use composable_tests_helpers::test::{
	block::process_and_progress_blocks,
	currency::{PICA, USDT},
	helper::assert_extrinsic_event,
};
use composable_traits::staking::{
	Reward, RewardConfig, RewardPoolConfiguration, RewardRate, RewardRatePeriod, RewardUpdate,
};
use frame_support::{traits::TryCollect, BoundedBTreeMap};

use crate::test::{
	default_lock_config, new_test_ext,
	prelude::{block_seconds, ONE_YEAR_OF_BLOCKS},
	runtime::{MaxRewardConfigsPerPool, Origin, StakingRewards, System, ALICE},
	test_reward_accumulation_hook::check_rewards,
	Test,
};

use super::runtime;

#[test]
fn test_update_reward_pool() {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);
		// Review [Andy] fix an initial reward rate
		let reward_rate = 10_u128;

		StakingRewards::create_reward_pool(
			Origin::root(),
			RewardPoolConfiguration::RewardRateBasedIncentive {
				owner: ALICE,
				asset_id: PICA::ID,
				end_block: ONE_YEAR_OF_BLOCKS,
				reward_configs: [(
					USDT::ID,
					RewardConfig {
						asset_id: USDT::ID,
						max_rewards: 1_000_u128,
						reward_rate: RewardRate::per_second(reward_rate),
					},
				)]
				.into_iter()
				.try_collect()
				.unwrap(),
				lock: default_lock_config(),
			},
		)
		.unwrap();

		let pool_id = match System::events().first().unwrap().event {
			runtime::Event::StakingRewards(crate::Event::RewardPoolCreated { pool_id, .. }) =>
				pool_id,
			_ => panic!("pool creation event not found"),
		};

		process_and_progress_blocks::<StakingRewards, Test>(1);

		check_rewards(&[(ALICE, PICA::ID, &[(USDT::ID, reward_rate * block_seconds(1))])]);

		// Review [Andy] a new lower reward rate
		let new_reward_rate = 5_u128;
		let reward_updates: BoundedBTreeMap<_, _, MaxRewardConfigsPerPool> =
			[(USDT::ID, RewardUpdate { reward_rate: RewardRate::per_second(new_reward_rate) })]
				.into_iter()
				.try_collect()
				.unwrap();

		assert_extrinsic_event::<Test, _, _, _>(
			StakingRewards::update_rewards_pool(Origin::root(), pool_id, reward_updates),
			crate::Event::RewardPoolUpdated { pool_id },
		);

		process_and_progress_blocks::<StakingRewards, Test>(1);

		let pool = StakingRewards::pools(pool_id).unwrap();
		// Review [Andy] the max rewards
		let max_rewards = 1_000;
		assert!(matches!(
			pool.rewards.get(&USDT::ID).unwrap(),
			Reward {
				max_rewards,
				reward_rate: RewardRate { period: RewardRatePeriod::PerSecond, amount: new_reward_rate },
				..
			}
		));

		check_rewards(&[(
			ALICE,
			PICA::ID,
			&[(USDT::ID, (reward_rate * block_seconds(1)) + (new_reward_rate * block_seconds(1)))],
		)]);

		process_and_progress_blocks::<StakingRewards, Test>(10);

		check_rewards(&[(
			ALICE,
			PICA::ID,
			&[(USDT::ID, (reward_rate * block_seconds(1)) + (new_reward_rate * block_seconds(11)))],
		)]);

		// Review [Andy]
		// Rate of reward that should reach its ceiling and stay there
		let mega_reward_rate = 100_u128;
		let reward_updates: BoundedBTreeMap<_, _, MaxRewardConfigsPerPool> =
			[(USDT::ID, RewardUpdate { reward_rate: RewardRate::per_second(mega_reward_rate) })]
				.into_iter()
				.try_collect()
				.unwrap();

		assert_extrinsic_event::<Test, _, _, _>(
			StakingRewards::update_rewards_pool(Origin::root(), pool_id, reward_updates),
			crate::Event::RewardPoolUpdated { pool_id },
		);

		process_and_progress_blocks::<StakingRewards, Test>(10);

		check_rewards(&[(
			ALICE,
			PICA::ID,
			&[(USDT::ID, max_rewards)],
		)]);
	})
}
