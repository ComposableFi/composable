use core::ops::Mul;

use composable_support::validation::TryIntoValidated;
use composable_tests_helpers::test::{
	block::process_and_progress_blocks,
	currency::{PICA, USDT, XPICA},
	helper::RuntimeTrait,
};
use composable_traits::{
	staking::{
		lock::LockConfig, Reward, RewardConfig, RewardPoolConfiguration, RewardRate,
		RewardRatePeriod, RewardUpdate,
	},
	time::{ONE_HOUR, ONE_MINUTE},
};
use frame_support::{
	bounded_btree_map,
	traits::{fungibles::Inspect, TryCollect},
	BoundedBTreeMap,
};
use sp_arithmetic::fixed_point::FixedU64;
use sp_runtime::Perbill;

use crate::{
	runtime::{
		MaxRewardConfigsPerPool, RuntimeOrigin, StakingRewards, System, Test, Tokens, ALICE, BOB,
		CHARLIE,
	},
	test::{
		default_lock_config, mint_assets, new_test_ext,
		prelude::{block_seconds, init_logger, MINIMUM_STAKING_AMOUNT, STAKING_FNFT_COLLECTION_ID},
		test_reward_accumulation_hook::{check_rewards, CheckRewards, PoolRewards},
	},
	test_helpers::{
		add_to_rewards_pot_and_assert, create_rewards_pool_and_assert, stake_and_assert,
	},
};

#[test]
fn test_update_reward_pool() {
	new_test_ext().execute_with(|| {
		const INITIAL_AMOUNT: u128 = PICA::units(100);

		const INITIAL_REWARD_RATE_AMOUNT: u128 = 10;
		const UPDATED_REWARD_RATE_AMOUNT: u128 = 5;

		init_logger();

		process_and_progress_blocks::<StakingRewards, Test>(1);

		create_rewards_pool_and_assert::<Test>(RewardPoolConfiguration::RewardRateBasedIncentive {
			owner: ALICE,
			asset_id: PICA::ID,
			start_block: 2,
			reward_configs: [(
				USDT::ID,
				RewardConfig { reward_rate: RewardRate::per_second(INITIAL_REWARD_RATE_AMOUNT) },
			)]
			.into_iter()
			.try_collect()
			.expect("Rewards pool has a valid config for creation; QED"),
			lock: default_lock_config(),
			share_asset_id: XPICA::ID,
			financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID,
			minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
		});

		mint_assets([ALICE], [USDT::ID], INITIAL_AMOUNT);
		add_to_rewards_pot_and_assert::<Test>(ALICE, PICA::ID, USDT::ID, INITIAL_AMOUNT, false);

		process_and_progress_blocks::<StakingRewards, Test>(2);

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

		Test::assert_extrinsic_event(
			StakingRewards::update_rewards_pool(
				RuntimeOrigin::root(),
				PICA::ID,
				reward_updates.clone(),
			),
			crate::Event::RewardPoolUpdated {
				pool_id: PICA::ID,
				reward_updates: reward_updates.into(),
			},
		);

		process_and_progress_blocks::<StakingRewards, Test>(1);

		let pool = StakingRewards::pools(PICA::ID).unwrap();
		assert!(matches!(
			pool.rewards.get(&USDT::ID).unwrap(),
			Reward {
				reward_rate: RewardRate {
					period: RewardRatePeriod::PerSecond,
					amount: UPDATED_REWARD_RATE_AMOUNT
				},
				..
			}
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
	});
}

#[test]
fn update_accumulates_properly() {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(10);

		let reward_rate = RewardRate::per_second(USDT::units(1) / 1_000);

		let pool_config = RewardPoolConfiguration::RewardRateBasedIncentive {
			owner: ALICE,
			asset_id: PICA::ID,
			start_block: 50,
			reward_configs: bounded_btree_map! {
				USDT::ID => RewardConfig {
					reward_rate: reward_rate.clone(),
				},
			},
			lock: LockConfig {
				duration_multipliers: bounded_btree_map! {
					// 1%
					ONE_HOUR => FixedU64::from_rational(101, 100)
						.try_into_validated()
						.expect(">= 1"),
					// 0.1%
					ONE_MINUTE => FixedU64::from_rational(1_001, 1_000)
						.try_into_validated()
						.expect(">= 1"),
				}
				.into(),
				unlock_penalty: Perbill::from_percent(5),
			},
			share_asset_id: XPICA::ID,
			financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID,
			minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
		};

		Test::assert_extrinsic_event(
			StakingRewards::create_reward_pool(RuntimeOrigin::root(), pool_config.clone()),
			crate::Event::<Test>::RewardPoolCreated {
				pool_id: PICA::ID,
				owner: ALICE,
				pool_config,
			},
		);

		process_and_progress_blocks::<StakingRewards, Test>(10);

		mint_assets([BOB], [USDT::ID], USDT::units(100_000));
		add_to_rewards_pot_and_assert::<Test>(BOB, PICA::ID, USDT::ID, USDT::units(100_000), false);

		process_and_progress_blocks::<StakingRewards, Test>(30);

		assert_eq!(System::block_number(), 50);

		mint_assets([CHARLIE], [PICA::ID], PICA::units(101));
		let stake_id = stake_and_assert::<Test>(CHARLIE, PICA::ID, PICA::units(100), ONE_HOUR);

		process_and_progress_blocks::<StakingRewards, Test>(1);

		Test::assert_extrinsic_event(
			StakingRewards::claim(
				RuntimeOrigin::signed(CHARLIE),
				STAKING_FNFT_COLLECTION_ID,
				stake_id,
			),
			crate::Event::Claimed {
				owner: CHARLIE,
				fnft_collection_id: STAKING_FNFT_COLLECTION_ID,
				fnft_instance_id: stake_id,
				claimed_amounts: [(USDT::ID, USDT::units(1) / 1_000 * 6)].into_iter().collect(),
			},
		);

		let claimed = Tokens::balance(USDT::ID, &CHARLIE);

		dbg!(claimed);

		let expected = dbg!(reward_rate).amount.mul(block_seconds(1));

		assert_eq!(expected, claimed);
	});
}
