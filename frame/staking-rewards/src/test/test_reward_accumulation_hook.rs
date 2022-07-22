use std::time::{SystemTime, UNIX_EPOCH};

use composable_tests_helpers::test::helper::{assert_last_event, assert_no_event};

use crate::{test::prelude::*, Pallet};

use super::*;

#[test]
fn test_reward_update_calculation() {
	new_test_ext().execute_with(|| {
		const SECONDS_PER_BLOCK: u64 = 12;
		const MAX_REWARD_UNITS: u128 = 39;
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
			reward_rate: RewardRate { period: 10, amount: PICA::units(2) },
			last_updated_timestamp: now,
		};

		// the expected total_rewards amount (in units) for each block
		let expected = [2, 4, 6, 8, 12, 14, 16, 18, 20, 24, 26, 28, 30, 32, 36, 38];

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
		// note that the max rewards is 39 and the reward amount per period is 2 - when the max is
		// reached, as much as possible up to the max amount rewarded (even if it's a smaller
		// increment than amount)
		assert_eq!(reward.total_rewards, PICA::units(MAX_REWARD_UNITS));

		// should report an error since the max was hit
		assert_last_event::<Test>(Event::StakingRewards(
			crate::Event::<Test>::RewardAccumulationError { pool_id: POOL_ID, asset_id: PICA::ID },
		));
	})
}
