use composable_tests_helpers::test::{
	block::process_and_progress_blocks,
	currency::{PICA, USDT, XPICA},
	helper::assert_extrinsic_event,
};
use composable_traits::staking::{
	RateBasedConfig, RateBasedReward, RewardConfigType, RewardPoolConfig, RewardRate,
	RewardRatePeriod, RewardType, RewardUpdate,
};
use frame_support::{traits::TryCollect, BoundedBTreeMap};

use crate::test::{
	default_lock_config, mint_assets, new_test_ext,
	prelude::{
		add_to_rewards_pot_and_assert, block_seconds, create_rewards_pool_and_assert,
		ONE_YEAR_OF_BLOCKS,
	},
	runtime::{MaxRewardConfigsPerPool, Origin, StakingRewards, ALICE},
	test_reward_accumulation_hook::{check_rewards, CheckRewards, PoolRewards},
	Test,
};

use super::prelude::STAKING_FNFT_COLLECTION_ID;

#[test]
fn test_update_reward_pool() {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);

		const INITIAL_AMOUNT: u128 = PICA::units(100);

		const INITIAL_REWARD_RATE_AMOUNT: u128 = 10;
		const UPDATED_REWARD_RATE_AMOUNT: u128 = 5;

		let pool_id = create_rewards_pool_and_assert(RewardPoolConfig {
			owner: ALICE,
			asset_id: PICA::ID,
			end_block: ONE_YEAR_OF_BLOCKS,
			reward_configs: [(
				USDT::ID,
				RewardConfigType::RateBased(RateBasedConfig {
					max_rewards: 1_000_u128,
					reward_rate: RewardRate::per_second(INITIAL_REWARD_RATE_AMOUNT),
				}),
			)]
			.into_iter()
			.try_collect()
			.unwrap(),
			lock: default_lock_config(),
			share_asset_id: XPICA::ID,
			financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID,
		});

		mint_assets([ALICE], [USDT::ID], INITIAL_AMOUNT);
		add_to_rewards_pot_and_assert(ALICE, pool_id, USDT::ID, INITIAL_AMOUNT);

		process_and_progress_blocks::<StakingRewards, Test>(1);

		check_rewards(&[CheckRewards {
			owner: ALICE,
			pool_asset_id: PICA::ID,
			pool_rewards: &[PoolRewards {
				reward_asset_id: USDT::ID,
				expected_total_rewards: (INITIAL_REWARD_RATE_AMOUNT * block_seconds(1)),
				expected_locked_balance: INITIAL_AMOUNT -
					(INITIAL_REWARD_RATE_AMOUNT * block_seconds(1)),
				expected_unlocked_balance: (INITIAL_REWARD_RATE_AMOUNT * block_seconds(1)),
			}],
		}]);

		let reward_updates: BoundedBTreeMap<_, _, MaxRewardConfigsPerPool> = [(
			USDT::ID,
			RewardUpdate { reward_rate: RewardRate::per_second(UPDATED_REWARD_RATE_AMOUNT) },
		)]
		.into_iter()
		.try_collect()
		.unwrap();

		assert_extrinsic_event::<Test, _, _, _>(
			StakingRewards::update_rewards_pool(Origin::root(), pool_id, reward_updates),
			crate::Event::RewardPoolUpdated { pool_id },
		);

		process_and_progress_blocks::<StakingRewards, Test>(1);

		let pool = StakingRewards::pools(pool_id).unwrap();
		assert!(matches!(
			pool.rewards.get(&USDT::ID).unwrap(),
			RewardType::RateBased(RateBasedReward {
				max_rewards: 1_000,
				reward_rate: RewardRate {
					period: RewardRatePeriod::PerSecond,
					amount: UPDATED_REWARD_RATE_AMOUNT
				},
				..
			})
		));

		check_rewards(&[CheckRewards {
			owner: ALICE,
			pool_asset_id: PICA::ID,
			pool_rewards: &[PoolRewards {
				reward_asset_id: USDT::ID,
				expected_total_rewards: (INITIAL_REWARD_RATE_AMOUNT * block_seconds(1)) +
					(UPDATED_REWARD_RATE_AMOUNT * block_seconds(1)),
				expected_locked_balance: INITIAL_AMOUNT -
					((INITIAL_REWARD_RATE_AMOUNT * block_seconds(1)) +
						(UPDATED_REWARD_RATE_AMOUNT * block_seconds(1))),
				expected_unlocked_balance: (INITIAL_REWARD_RATE_AMOUNT * block_seconds(1)) +
					(UPDATED_REWARD_RATE_AMOUNT * block_seconds(1)),
			}],
		}]);

		process_and_progress_blocks::<StakingRewards, Test>(10);

		check_rewards(&[CheckRewards {
			owner: ALICE,
			pool_asset_id: PICA::ID,
			pool_rewards: &[PoolRewards {
				reward_asset_id: USDT::ID,
				expected_total_rewards: (INITIAL_REWARD_RATE_AMOUNT * block_seconds(1)) +
					(UPDATED_REWARD_RATE_AMOUNT * block_seconds(11)),
				expected_locked_balance: INITIAL_AMOUNT -
					((INITIAL_REWARD_RATE_AMOUNT * block_seconds(1)) +
						(UPDATED_REWARD_RATE_AMOUNT * block_seconds(11))),
				expected_unlocked_balance: (INITIAL_REWARD_RATE_AMOUNT * block_seconds(1)) +
					(UPDATED_REWARD_RATE_AMOUNT * block_seconds(11)),
			}],
		}]);
	})
}
