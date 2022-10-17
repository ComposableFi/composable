#![allow(clippy::disallowed_methods)] // disabled for now to make running clippy on the tests easier

pub(crate) use crate::test::runtime::{new_test_ext, Test}; // for benchmarks
use crate::{
	claim_of_stake,
	test::{
		prelude::{stake_and_assert, unstake_and_assert, H256, MINIMUM_STAKING_AMOUNT},
		runtime::*,
	},
	Config, RewardPoolConfigurationOf, RewardPools, Stakes,
};
use composable_support::validation::TryIntoValidated;
use composable_tests_helpers::test::{
	block::{next_block, process_and_progress_blocks},
	currency::{BTC, PICA, USDT, XPICA},
	helper::{self, assert_extrinsic_event, assert_extrinsic_event_with, assert_last_event_with},
};
use composable_traits::{
	fnft::{FinancialNft as FinancialNftT, FinancialNftProtocol},
	staking::{
		lock::{Lock, LockConfig},
		ProtocolStaking, RewardConfig,
		RewardPoolConfiguration::RewardRateBasedIncentive,
		RewardRate, Stake,
	},
	time::{DurationSeconds, ONE_HOUR, ONE_MINUTE},
};
use frame_support::{
	assert_err, assert_noop, assert_ok,
	traits::{
		fungibles::{Inspect, Mutate},
		tokens::nonfungibles::InspectEnumerable,
		TryCollect,
	},
	BoundedBTreeMap,
};
use frame_system::EventRecord;
use sp_arithmetic::{fixed_point::FixedU64, Perbill, Permill};
use sp_core::sr25519::Public;
use sp_runtime::PerThing;
use sp_std::collections::{btree_map::BTreeMap, btree_set::BTreeSet};

use self::prelude::{
	add_to_rewards_pot_and_assert, create_rewards_pool_and_assert, split_and_assert,
	STAKING_FNFT_COLLECTION_ID,
};

mod prelude;
mod runtime;

mod test_reward_accumulation_hook;
mod test_update_reward_pools;

#[test]
fn test_create_reward_pool() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		create_default_reward_pool();

		assert_eq!(
			FinancialNft::collections().collect::<BTreeSet<_>>(),
			BTreeSet::from([PICA::ID])
		);

		assert_eq!(
			<StakingRewards as FinancialNftProtocol>::collection_asset_ids(),
			vec![PICA::ID]
		);
	});
}

#[test]
fn duration_presets_minimum_is_1() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(StakingRewards::create_reward_pool(
			Origin::root(),
			RewardRateBasedIncentive {
				owner: ALICE,
				asset_id: PICA::ID,
				start_block: 2,
				end_block: 5,
				reward_configs: default_reward_config(),
				lock: LockConfig {
					duration_presets: [
						(
							ONE_MINUTE,
							FixedU64::from_rational(110, 100).try_into_validated().expect(">= 1")
						), // 0.1%
					]
					.into_iter()
					.try_collect()
					.unwrap(),
					unlock_penalty: Perbill::from_percent(5),
				},
				share_asset_id: XPICA::ID,
				financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID,
				minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
			},
		));
	});
}

#[test]
fn test_create_reward_pool_invalid_end_block() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		assert_err!(
			StakingRewards::create_reward_pool(
				Origin::root(),
				RewardRateBasedIncentive {
					owner: ALICE,
					asset_id: PICA::ID,
					// end block can't be before the current block
					start_block: 2,
					end_block: 0,
					reward_configs: default_reward_config(),
					lock: default_lock_config(),
					share_asset_id: XPICA::ID,
					financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID,
					minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
				}
			),
			crate::Error::<Test>::EndBlockMustBeAfterStartBlock
		);
	});
}

#[test]
fn create_staking_reward_pool_should_fail_when_pool_asset_id_is_zero() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		assert_err!(
			StakingRewards::create_reward_pool(
				Origin::root(),
				RewardRateBasedIncentive {
					owner: ALICE,
					asset_id: 0,
					// end block can't be before the current block
					start_block: 2,
					end_block: 0,
					reward_configs: default_reward_config(),
					lock: default_lock_config(),
					share_asset_id: XPICA::ID,
					financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID,
					minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
				}
			),
			crate::Error::<Test>::InvalidAssetId,
		);
	});
}

#[test]
fn create_staking_reward_pool_should_fail_when_slashed_amount_is_less_than_existential_deposit() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		assert_err!(
			StakingRewards::create_reward_pool(
				Origin::root(),
				RewardRateBasedIncentive {
					owner: ALICE,
					asset_id: PICA::ID,
					start_block: 2,
					end_block: 5,
					reward_configs: default_reward_config(),
					lock: LockConfig {
						duration_presets: [
							(
								ONE_HOUR,
								FixedU64::from_rational(101, 100)
									.try_into_validated()
									.expect(">= 1")
							), // 1%
							(
								ONE_MINUTE,
								FixedU64::from_rational(1_001, 1_000)
									.try_into_validated()
									.expect(">= 1")
							), // 0.1%
						]
						.into_iter()
						.try_collect()
						.unwrap(),
						unlock_penalty: Perbill::from_percent(99),
					},
					share_asset_id: XPICA::ID,
					financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID,
					minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
				}
			),
			crate::Error::<Test>::SlashedAmountTooLow,
		);
	});
}

#[test]
fn create_staking_reward_pool_should_fail_when_slashed_minimum_amount_is_less_than_existential_deposit(
) {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		assert_err!(
			StakingRewards::create_reward_pool(
				Origin::root(),
				RewardRateBasedIncentive {
					owner: ALICE,
					asset_id: PICA::ID,
					start_block: 2,
					end_block: 5,
					reward_configs: default_reward_config(),
					lock: LockConfig {
						duration_presets: [
							(
								ONE_HOUR,
								FixedU64::from_rational(101, 100)
									.try_into_validated()
									.expect("valid reward multiplier")
							), // 1%
							(
								ONE_MINUTE,
								FixedU64::from_rational(11, 10)
									.try_into_validated()
									.expect("valid reward multiplier")
							), // 0.1%
						]
						.into_iter()
						.try_collect()
						.unwrap(),
						unlock_penalty: Perbill::from_percent(60),
					},
					share_asset_id: XPICA::ID,
					financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID,
					minimum_staking_amount: 10,
				}
			),
			crate::Error::<Test>::SlashedMinimumStakingAmountTooLow,
		);
	});
}

#[test]
fn create_staking_reward_pool_should_fail_when_share_asset_id_is_zero() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		assert_err!(
			StakingRewards::create_reward_pool(
				Origin::root(),
				RewardRateBasedIncentive {
					owner: ALICE,
					asset_id: PICA::ID,
					// end block can't be before the current block
					start_block: 2,
					end_block: 0,
					reward_configs: default_reward_config(),
					lock: default_lock_config(),
					share_asset_id: 0,
					financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID,
					minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
				}
			),
			crate::Error::<Test>::InvalidAssetId
		);
	});
}

#[test]
fn create_staking_reward_pool_should_fail_when_fnft_collection_asset_id_is_zero() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		assert_err!(
			StakingRewards::create_reward_pool(
				Origin::root(),
				RewardRateBasedIncentive {
					owner: ALICE,
					asset_id: PICA::ID,
					// end block can't be before the current block
					start_block: 2,
					end_block: 0,
					reward_configs: default_reward_config(),
					lock: default_lock_config(),
					share_asset_id: XPICA::ID,
					financial_nft_asset_id: 0,
					minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
				}
			),
			crate::Error::<Test>::InvalidAssetId
		);
	});
}

#[test]
fn stake_should_fail_before_start_of_rewards_pool() {
	new_test_ext().execute_with(|| {
		let staker = ALICE;
		let amount = 100_500;
		let duration = ONE_HOUR;
		let pool_id = PICA::ID;

		process_and_progress_blocks::<StakingRewards, Test>(1);
		assert_ok!(StakingRewards::create_reward_pool(Origin::root(), get_default_reward_pool()));

		assert_noop!(
			StakingRewards::stake(Origin::signed(staker), pool_id, amount, duration),
			crate::Error::<Test>::RewardsPoolHasNotStarted
		);
	});
}

#[test]
fn stake_in_case_of_low_balance_should_not_work() {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);
		const AMOUNT: u128 = 100_500_u128;

		create_default_reward_pool();

		let asset_id = PICA::ID;
		assert_eq!(balance(asset_id, &ALICE), 0);

		process_and_progress_blocks::<StakingRewards, Test>(1);
		assert_noop!(
			StakingRewards::stake(Origin::signed(ALICE), PICA::ID, AMOUNT, ONE_HOUR),
			crate::Error::<Test>::NotEnoughAssets
		);

		assert!(Stakes::<Test>::iter_prefix_values(PICA::ID).next().is_none());
	});
}

#[test]
fn stake_should_fail_if_amount_is_less_than_minimum() {
	new_test_ext().execute_with(|| {
		let staker = ALICE;
		let amount = 9_000_u128;
		let duration = ONE_HOUR;
		let pool_id = PICA::ID;

		process_and_progress_blocks::<StakingRewards, Test>(1);
		assert_ok!(StakingRewards::create_reward_pool(Origin::root(), get_default_reward_pool()));

		assert_noop!(
			StakingRewards::stake(Origin::signed(staker), pool_id, amount, duration),
			crate::Error::<Test>::StakedAmountTooLow
		);
	});
}

#[test]
fn split_should_fail_if_any_amount_is_less_than_minimum() {
	new_test_ext().execute_with(|| {
		let staker = ALICE;

		process_and_progress_blocks::<StakingRewards, Test>(1);

		mint_assets([ALICE], [PICA::ID], PICA::units(10_000));

		create_rewards_pool_and_assert::<Test, runtime::Event>(RewardRateBasedIncentive {
			owner: ALICE,
			asset_id: PICA::ID,
			start_block: 2,
			end_block: 5,
			reward_configs: default_reward_config(),
			lock: default_lock_config(),
			share_asset_id: XPICA::ID,
			financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID,
			minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
		});

		process_and_progress_blocks::<StakingRewards, Test>(1);

		let amount = 15_000;
		let original_fnft_instance_id =
			stake_and_assert::<Test, runtime::Event>(ALICE, PICA::ID, amount, ONE_MINUTE);

		// Original stake is less than the minimum and new stake is greater.
		let ratio = Permill::from_rational(1_u32, 3_u32);

		assert_eq!(ratio.mul_floor(amount), 4_999);
		assert_eq!(ratio.left_from_one().mul_ceil(amount), 10_001);

		assert_noop!(
			StakingRewards::split(
				Origin::signed(staker),
				STAKING_FNFT_COLLECTION_ID,
				original_fnft_instance_id,
				ratio.try_into_validated().unwrap(),
			),
			crate::Error::<Test>::StakedAmountTooLowAfterSplit
		);

		// New stake is less than the minimum and original stake is greater.
		let ratio = Permill::from_rational(2_u32, 3_u32);

		assert_eq!(ratio.mul_floor(amount), 9_999);
		assert_eq!(ratio.left_from_one().mul_ceil(amount), 5_001);

		assert_noop!(
			StakingRewards::split(
				Origin::signed(staker),
				STAKING_FNFT_COLLECTION_ID,
				original_fnft_instance_id,
				ratio.try_into_validated().unwrap(),
			),
			crate::Error::<Test>::StakedAmountTooLowAfterSplit
		);

		// Both stakes are less than the minimum.
		let ratio = Permill::from_rational(1_u32, 2_u32);

		assert_eq!(ratio.mul_floor(amount), 7_500);
		assert_eq!(ratio.left_from_one().mul_ceil(amount), 7_500);

		assert_noop!(
			StakingRewards::split(
				Origin::signed(staker),
				STAKING_FNFT_COLLECTION_ID,
				original_fnft_instance_id,
				ratio.try_into_validated().unwrap(),
			),
			crate::Error::<Test>::StakedAmountTooLowAfterSplit
		);
	});
}

#[test]
fn split_doesnt_cause_loss_in_assets() {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);

		mint_assets([ALICE], [PICA::ID], PICA::units(10_000));

		create_rewards_pool_and_assert::<Test, runtime::Event>(RewardRateBasedIncentive {
			owner: ALICE,
			asset_id: PICA::ID,
			start_block: 2,
			end_block: 5,
			reward_configs: default_reward_config(),
			lock: default_lock_config(),
			share_asset_id: XPICA::ID,
			financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID,
			minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
		});

		process_and_progress_blocks::<StakingRewards, Test>(1);

		let amount = 100_000;
		let ratio = Permill::from_parts(555_555);

		// 0.555_555 * 100_000 is not an integer, there will be rounding when calculating the
		// amounts for the split positions
		// the following rounding scheme is used to prevent loss of assets when splitting:
		assert_eq!(ratio.mul_floor(amount), 55_555);
		assert_eq!(ratio.left_from_one().mul_ceil(amount), 44_445);

		let original_fnft_instance_id =
			stake_and_assert::<Test, runtime::Event>(ALICE, PICA::ID, amount, ONE_MINUTE);

		// split_and_assert checks for loss of assets
		split_and_assert::<Test, runtime::Event>(
			ALICE,
			STAKING_FNFT_COLLECTION_ID,
			original_fnft_instance_id,
			ratio.try_into_validated().unwrap(),
		);
	})
}

#[test]
fn stake_in_case_of_zero_inflation_should_work() {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);

		assert_ok!(StakingRewards::create_reward_pool(Origin::root(), get_default_reward_pool()));
		process_and_progress_blocks::<StakingRewards, Test>(1);
		let staker: Public = ALICE;
		let amount: u128 = 100_500_u128;
		let duration_preset: u64 = ONE_HOUR;
		let fnft_asset_account = FinancialNft::asset_account(&1, &0);

		let staked_asset_id = PICA::ID;
		mint_assets([staker], [staked_asset_id], amount * 2);

		let fnft_instance_id = assert_extrinsic_event_with::<Test, Event, _, _, _, _>(
			StakingRewards::stake(Origin::signed(staker), PICA::ID, amount, duration_preset),
			|event| match event {
				crate::Event::<Test>::Staked {
					pool_id: PICA::ID,
					owner: _staker,
					amount: _,
					duration_preset: _,
					fnft_collection_id: _,
					fnft_instance_id,
					reward_multiplier: _,
					keep_alive: _,
				} => Some(fnft_instance_id),
				_ => None,
			},
		);
		assert_eq!(Stakes::<Test>::iter_prefix_values(PICA::ID).count(), 1);

		let rewards_pool = StakingRewards::pools(PICA::ID).expect("rewards_pool expected");
		let reward_multiplier = StakingRewards::reward_multiplier(&rewards_pool, duration_preset)
			.expect("reward_multiplier expected");
		let inflation = 0;
		let reductions = rewards_pool
			.rewards
			.keys()
			.map(|asset_id| (*asset_id, inflation))
			.try_collect()
			.expect("reductions expected");

		assert_eq!(
			Stakes::<Test>::get(PICA::ID, fnft_instance_id),
			Some(Stake {
				reward_pool_id: PICA::ID,
				stake: amount,
				share: StakingRewards::boosted_amount(reward_multiplier, amount)
					.expect("boosted amount should not overflow"),
				reductions,
				lock: Lock {
					started_at: 12,
					duration: duration_preset,
					unlock_penalty: rewards_pool.lock.unlock_penalty,
				},
			})
		);
		assert_eq!(
			<StakingRewards as FinancialNftProtocol>::value_of(&PICA::ID, &fnft_instance_id)
				.expect("must return a value"),
			vec![(
				XPICA::ID,
				StakingRewards::boosted_amount(reward_multiplier, amount)
					.expect("boosted amount should not overflow"),
			)]
		);
		assert_eq!(balance(staked_asset_id, &staker), amount);
		assert_eq!(balance(staked_asset_id, &fnft_asset_account), amount);
		assert_eq!(
			balance(XPICA::ID, &fnft_asset_account),
			StakingRewards::boosted_amount(reward_multiplier, amount)
				.expect("boosted amount should not overflow"),
		);

		assert_last_event_with::<Test, Event, crate::Event<Test>, _>(|event| {
			matches!(
				event,
				crate::Event::Staked {
					pool_id: PICA::ID,
					owner,
					amount: _,
					duration_preset: _,
					fnft_collection_id: PICA::ID,
					fnft_instance_id: _,
					reward_multiplier: _,
					keep_alive: _,
				} if owner == staker
			)
			.then_some(())
		});
	});
}

// this is almost the exact same as the above function
// spot the difference!
// maybe do a proptest with different inflation rates?
#[test]
fn stake_in_case_of_not_zero_inflation_should_work() {
	new_test_ext().execute_with(|| {
		const AMOUNT: u128 = 100_500_u128;
		const DURATION_PRESET: u64 = ONE_HOUR;

		let fnft_asset_account = FinancialNft::asset_account(&1, &0);

		process_and_progress_blocks::<StakingRewards, Test>(1);

		create_rewards_pool_and_assert::<Test, runtime::Event>(get_default_reward_pool());

		process_and_progress_blocks::<StakingRewards, Test>(1);

		let staked_asset_id = PICA::ID;
		mint_assets([ALICE], [staked_asset_id], AMOUNT * 2);

		stake_and_assert::<Test, runtime::Event>(ALICE, PICA::ID, AMOUNT, DURATION_PRESET);

		let rewards_pool = StakingRewards::pools(PICA::ID).expect("rewards_pool expected");
		let reward_multiplier = StakingRewards::reward_multiplier(&rewards_pool, DURATION_PRESET)
			.expect("reward_multiplier expected");
		let _ = StakingRewards::boosted_amount(reward_multiplier, AMOUNT)
			.expect("boosted amount should not overflow");

		let reductions = rewards_pool
			.rewards
			.iter()
			.map(|(asset_id, _reward)| (*asset_id, 0))
			.try_collect()
			.expect("reductions expected");

		assert_eq!(
			StakingRewards::stakes(1, 0),
			Some(Stake {
				reward_pool_id: PICA::ID,
				stake: AMOUNT,
				share: StakingRewards::boosted_amount(reward_multiplier, AMOUNT)
					.expect("boosted amount should not overflow"),
				reductions,
				lock: Lock {
					started_at: 12,
					duration: DURATION_PRESET,
					unlock_penalty: rewards_pool.lock.unlock_penalty,
				},
			})
		);

		assert_eq!(balance(PICA::ID, &ALICE), AMOUNT);
		assert_eq!(balance(PICA::ID, &fnft_asset_account), AMOUNT);
	});
}

mod extend {
	use composable_support::validation::TryIntoValidated;
	use composable_tests_helpers::test::{
		block::process_and_progress_blocks,
		currency::{Currency, USDT, XPICA},
		helper::assert_extrinsic_event,
	};
	use composable_traits::{
		staking::{
			lock::{Lock, LockConfig},
			RewardConfig, RewardPoolConfiguration, RewardRate, Stake,
		},
		time::{ONE_HOUR, ONE_MINUTE},
	};
	use frame_support::traits::UnixTime;
	use sp_arithmetic::fixed_point::FixedU64;
	use sp_runtime::Perbill;

	use crate::{
		test::{
			btree_map, mint_assets,
			prelude::{
				add_to_rewards_pot_and_assert, create_rewards_pool_and_assert, stake_and_assert,
				MINIMUM_STAKING_AMOUNT, STAKING_FNFT_COLLECTION_ID,
			},
			runtime::{self, Origin, StakingRewards, ALICE, BOB},
			Test,
		},
		Pallet, Stakes,
	};

	use super::new_test_ext;

	#[allow(non_camel_case_types)]
	type STAKED_ASSET = Currency<1337, 12>;

	#[test]
	fn with_additional_stake() {
		new_test_ext().execute_with(|| {
			process_and_progress_blocks::<StakingRewards, Test>(1);

			let current_block_number = frame_system::Pallet::<Test>::block_number();

			create_rewards_pool_and_assert::<Test, runtime::Event>(
				RewardPoolConfiguration::RewardRateBasedIncentive {
					owner: ALICE,
					asset_id: STAKED_ASSET::ID,
					start_block: current_block_number + 1,
					end_block: current_block_number + 100_001,
					reward_configs: btree_map([(
						USDT::ID,
						RewardConfig {
							max_rewards: USDT::units(100_000),
							reward_rate: RewardRate::per_second(USDT::units(1)),
						},
					)]),
					lock: LockConfig {
						duration_presets: btree_map([
							(
								ONE_MINUTE,
								FixedU64::from_rational(1_001, 1_000)
									.try_into_validated()
									.expect(">= 1"),
							), /* 1% */
							(
								ONE_HOUR,
								FixedU64::from_rational(101, 100)
									.try_into_validated()
									.expect(">= 1"),
							), /* 0.1% */
						]),
						unlock_penalty: Perbill::from_percent(5),
					},
					share_asset_id: XPICA::ID,
					financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID,
					minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
				},
			);

			process_and_progress_blocks::<StakingRewards, Test>(1);

			mint_assets([ALICE], [USDT::ID], USDT::units(100_000));
			add_to_rewards_pot_and_assert(ALICE, STAKED_ASSET::ID, USDT::ID, USDT::units(100_000));

			let staked_amount = STAKED_ASSET::units(5);
			let extended_amount = STAKED_ASSET::units(6);
			let existential_deposit = 1_000_u128;

			mint_assets(
				[BOB],
				[STAKED_ASSET::ID],
				staked_amount + extended_amount + existential_deposit,
			);

			let fnft_instance_id = stake_and_assert::<Test, runtime::Event>(
				BOB,
				STAKED_ASSET::ID,
				staked_amount,
				ONE_MINUTE,
			);

			process_and_progress_blocks::<StakingRewards, Test>(10);

			assert_extrinsic_event::<Test, runtime::Event, _, _, _>(
				StakingRewards::extend(
					Origin::signed(BOB),
					STAKING_FNFT_COLLECTION_ID,
					fnft_instance_id,
					extended_amount,
				),
				crate::Event::<Test>::StakeAmountExtended {
					fnft_collection_id: STAKING_FNFT_COLLECTION_ID,
					fnft_instance_id,
					amount: extended_amount,
				},
			);

			let stake_after_extend =
				Stakes::<Test>::get(STAKING_FNFT_COLLECTION_ID, fnft_instance_id)
					.expect("stake should exist");

			let rewards_pool =
				StakingRewards::pools(STAKED_ASSET::ID).expect("rewards pool should exist");

			assert_eq!(
				stake_after_extend,
				Stake {
					reward_pool_id: STAKED_ASSET::ID,
					stake: staked_amount + extended_amount,
					share: Pallet::<Test>::boosted_amount(
						rewards_pool.lock.duration_presets[&ONE_MINUTE],
						staked_amount + extended_amount
					)
					.expect("boosted amount calculation should not fail"),
					// 5 units already staked, 6 more units added, 11 blocks worth of rewards
					// already accumulated at 1 unit per second,, this is the resulting inflation:
					// (66*10^12) * ((6*10^12) * 1.01) / ((5*10^12) * 1.01)
					reductions: btree_map([(USDT::ID, 79_200_000_000_000)]),
					lock: Lock {
						started_at: <<Test as crate::Config>::UnixTime as UnixTime>::now()
							.as_secs(),
						duration: ONE_MINUTE,
						unlock_penalty: rewards_pool.lock.unlock_penalty
					}
				}
			);
		});
	}

	#[test]
	fn with_no_additional_stake() {
		new_test_ext().execute_with(|| {
			process_and_progress_blocks::<StakingRewards, Test>(1);

			let current_block_number = frame_system::Pallet::<Test>::block_number();

			create_rewards_pool_and_assert::<Test, runtime::Event>(
				RewardPoolConfiguration::RewardRateBasedIncentive {
					owner: ALICE,
					asset_id: STAKED_ASSET::ID,
					start_block: current_block_number + 1,
					end_block: current_block_number + 100_001,
					reward_configs: btree_map([(
						USDT::ID,
						RewardConfig {
							max_rewards: USDT::units(100_000),
							reward_rate: RewardRate::per_second(USDT::units(1)),
						},
					)]),
					lock: LockConfig {
						duration_presets: btree_map([
							(
								ONE_MINUTE,
								FixedU64::from_rational(1_001, 1_000)
									.try_into_validated()
									.expect(">= 1"),
							), /* 1% */
							(
								ONE_HOUR,
								FixedU64::from_rational(101, 100)
									.try_into_validated()
									.expect(">= 1"),
							), /* 0.1% */
						]),
						unlock_penalty: Perbill::from_percent(5),
					},
					share_asset_id: XPICA::ID,
					financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID,
					minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
				},
			);

			process_and_progress_blocks::<StakingRewards, Test>(1);

			mint_assets([ALICE], [USDT::ID], USDT::units(100_000));
			add_to_rewards_pot_and_assert(ALICE, STAKED_ASSET::ID, USDT::ID, USDT::units(100_000));

			let staked_amount = STAKED_ASSET::units(5);
			let extended_amount = STAKED_ASSET::units(0);
			let existential_deposit = 1_000_u128;

			mint_assets(
				[BOB],
				[STAKED_ASSET::ID],
				staked_amount + extended_amount + existential_deposit,
			);

			let fnft_instance_id = stake_and_assert::<Test, runtime::Event>(
				BOB,
				STAKED_ASSET::ID,
				staked_amount,
				ONE_MINUTE,
			);

			process_and_progress_blocks::<StakingRewards, Test>(10);

			assert_extrinsic_event::<Test, runtime::Event, _, _, _>(
				StakingRewards::extend(
					Origin::signed(BOB),
					STAKING_FNFT_COLLECTION_ID,
					fnft_instance_id,
					extended_amount,
				),
				crate::Event::<Test>::StakeAmountExtended {
					fnft_collection_id: STAKING_FNFT_COLLECTION_ID,
					fnft_instance_id,
					amount: extended_amount,
				},
			);

			let stake_after_extend =
				Stakes::<Test>::get(STAKING_FNFT_COLLECTION_ID, fnft_instance_id)
					.expect("stake should exist");

			let rewards_pool =
				StakingRewards::pools(STAKED_ASSET::ID).expect("rewards pool should exist");

			assert_eq!(
				stake_after_extend,
				Stake {
					reward_pool_id: STAKED_ASSET::ID,
					stake: staked_amount + extended_amount,
					share: Pallet::<Test>::boosted_amount(
						rewards_pool.lock.duration_presets[&ONE_MINUTE],
						staked_amount + extended_amount
					)
					.expect("boosted amount calculation should not fail"),
					// reductions should be zero as this is the only stake in the pool and no
					// additional stake was added to it
					reductions: btree_map([(USDT::ID, 0)]),
					lock: Lock {
						started_at: <<Test as crate::Config>::UnixTime as UnixTime>::now()
							.as_secs(),
						duration: ONE_MINUTE,
						unlock_penalty: rewards_pool.lock.unlock_penalty
					}
				}
			);
		});
	}
}

#[test]
fn unstake_non_existent_stake_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let staker = ALICE;
		assert_noop!(
			StakingRewards::unstake(Origin::signed(staker), 1, 0),
			crate::Error::<Test>::FnftNotFound
		);
	});
}

#[test]
fn not_owner_of_stake_can_not_unstake() {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);
		assert_ok!(StakingRewards::create_reward_pool(Origin::root(), get_default_reward_pool()));
		let owner = ALICE;
		let not_owner = BOB;
		let pool_id = PICA::ID;
		let amount = 100_500_u32.into();
		let duration_preset = ONE_HOUR;
		assert_ne!(owner, not_owner);

		let staked_asset_id = PICA::ID;
		mint_assets([owner, not_owner], [staked_asset_id], amount * 2);

		process_and_progress_blocks::<StakingRewards, Test>(1);
		assert_ok!(StakingRewards::stake(Origin::signed(owner), pool_id, amount, duration_preset));

		assert_noop!(
			StakingRewards::unstake(Origin::signed(not_owner), 1, 0),
			crate::Error::<Test>::OnlyStakeOwnerCanInteractWithStake,
		);
	});
}

#[test]
fn unstake_in_case_of_zero_claims_and_early_unlock_should_work() {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);

		create_rewards_pool_and_assert::<Test, runtime::Event>(get_default_reward_pool());

		process_and_progress_blocks::<StakingRewards, Test>(1);

		// far more than is necessary
		mint_assets([CHARLIE], [USDT::ID], USDT::units(100_000_000));
		add_to_rewards_pot_and_assert(CHARLIE, PICA::ID, USDT::ID, USDT::units(100_000_000));

		process_and_progress_blocks::<StakingRewards, Test>(1);

		mint_assets([BOB], [PICA::ID], PICA::units(200));
		let fnft_instance_id =
			stake_and_assert::<Test, runtime::Event>(BOB, PICA::ID, 100_500, ONE_HOUR);

		// TODO(benluelo): Proper test helper for claim
		assert_ok!(StakingRewards::claim(
			Origin::signed(BOB),
			STAKING_FNFT_COLLECTION_ID,
			fnft_instance_id
		));

		let rewards_pool = RewardPools::<Test>::get(PICA::ID).unwrap();

		for (reward_asset_id, reward) in rewards_pool.rewards {
			assert_eq!(
				claim_of_stake::<Test>(
					&Stakes::<Test>::get(STAKING_FNFT_COLLECTION_ID, fnft_instance_id).unwrap(),
					&rewards_pool.share_asset_id,
					&reward,
					&reward_asset_id
				),
				Ok(0)
			);
		}

		unstake_and_assert::<Test, runtime::Event>(
			BOB,
			STAKING_FNFT_COLLECTION_ID,
			fnft_instance_id,
			true,
		);
	});
}

#[test]
fn unstake_in_case_of_not_zero_claims_and_early_unlock_should_work() {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);

		create_rewards_pool_and_assert::<Test, runtime::Event>(get_default_reward_pool());

		process_and_progress_blocks::<StakingRewards, Test>(1);

		// far more than is necessary
		mint_assets([CHARLIE], [USDT::ID], USDT::units(100_000_000));
		add_to_rewards_pot_and_assert(CHARLIE, PICA::ID, USDT::ID, USDT::units(100_000_000));

		process_and_progress_blocks::<StakingRewards, Test>(1);

		mint_assets([BOB], [PICA::ID], PICA::units(200));
		let fnft_instance_id =
			stake_and_assert::<Test, runtime::Event>(BOB, PICA::ID, 100_500, ONE_HOUR);

		process_and_progress_blocks::<StakingRewards, Test>(1);

		unstake_and_assert::<Test, runtime::Event>(
			BOB,
			STAKING_FNFT_COLLECTION_ID,
			fnft_instance_id,
			true,
		);
	});
}

#[test]
fn unstake_in_case_of_not_zero_claims_and_not_early_unlock_should_work() {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);

		create_rewards_pool_and_assert::<Test, runtime::Event>(get_default_reward_pool());

		process_and_progress_blocks::<StakingRewards, Test>(1);

		// far more than is necessary
		mint_assets([CHARLIE], [USDT::ID], USDT::units(100_000_000));
		add_to_rewards_pot_and_assert(CHARLIE, PICA::ID, USDT::ID, USDT::units(100_000_000));

		process_and_progress_blocks::<StakingRewards, Test>(1);

		mint_assets([BOB], [PICA::ID], PICA::units(200));
		let fnft_instance_id =
			stake_and_assert::<Test, runtime::Event>(BOB, PICA::ID, 100_500, ONE_HOUR);

		// 700 blocks * 6 seconds per block > 1 hour
		process_and_progress_blocks::<StakingRewards, Test>(700);

		unstake_and_assert::<Test, runtime::Event>(
			BOB,
			STAKING_FNFT_COLLECTION_ID,
			fnft_instance_id,
			false,
		);
	});
}

#[test]
fn test_transfer_reward() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let pool_init_config = get_default_reward_pool();
		assert_ok!(StakingRewards::create_reward_pool(Origin::root(), pool_init_config));
		assert_ok!(<Tokens as Mutate<<StakingRewards as ProtocolStaking>::AccountId>>::mint_into(
			USDT::ID,
			&ALICE,
			20_000_u128
		));
		assert_ok!(<Tokens as Mutate<<StakingRewards as ProtocolStaking>::AccountId>>::mint_into(
			BTC::ID,
			&ALICE,
			20_000_u128
		));
		assert_ok!(<Tokens as Mutate<<StakingRewards as ProtocolStaking>::AccountId>>::mint_into(
			BTC::ID,
			&BOB,
			20_000_u128
		));
		assert_ok!(<StakingRewards as ProtocolStaking>::transfer_reward(
			&ALICE,
			&1,
			USDT::ID,
			10_u128,
			false
		));
		// only pool owner can add new reward
		// TODO (vim): Consider enabling this later
		// assert_noop!(
		// 	<StakingRewards as ProtocolStaking>::transfer_reward(&BOB, &1, BTC::ID, 10_000_u128),
		// 	crate::Error::<Test>::OnlyPoolOwnerCanAddNewReward
		// );

		assert_ok!(<StakingRewards as ProtocolStaking>::transfer_reward(
			&ALICE,
			&1,
			BTC::ID,
			10_000_u128,
			false
		));
	});
}

#[test]
fn test_split_position() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let pool_init_config = RewardRateBasedIncentive {
			owner: ALICE,
			asset_id: PICA::ID,
			start_block: 2,
			end_block: 5,
			reward_configs: default_reward_config(),
			lock: default_lock_config(),
			share_asset_id: XPICA::ID,
			financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID,
			minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
		};

		assert_extrinsic_event::<Test, _, _, _, _>(
			StakingRewards::create_reward_pool(Origin::root(), pool_init_config),
			crate::Event::<Test>::RewardPoolCreated {
				pool_id: PICA::ID,
				owner: ALICE,
				end_block: 5,
			},
		);
		process_and_progress_blocks::<StakingRewards, Test>(1);

		mint_assets([ALICE], [PICA::ID], PICA::units(2000));

		let existing_fnft_instance_id = stake_and_assert::<Test, runtime::Event>(
			ALICE,
			PICA::ID,
			PICA::units(1_000),
			ONE_HOUR,
			// crate::Event::Staked {
			// 	pool_id: PICA::ID,
			// 	owner: BOB,
			// 	amount: PICA::units(1_000),
			// 	duration_preset: ONE_HOUR,
			// 	fnft_collection_id: 1,
			// 	fnft_instance_id: 0,
			// 	reward_multiplier: FixedU64::from_rational(101, 100),
			// 	keep_alive: true,
			// },
		);

		let existing_stake_before_split =
			Stakes::<Test>::get(1, existing_fnft_instance_id).expect("stake should exist");

		let ratio = Permill::from_rational(1_u32, 7_u32);
		let left_from_one_ratio = ratio.left_from_one();

		let new_fnft_instance_id = split_and_assert::<Test, runtime::Event>(
			ALICE,
			1,
			0,
			ratio.try_into_validated().expect("valid split ratio"),
		);

		let existing_stake =
			Stakes::<Test>::get(1, existing_fnft_instance_id).expect("stake should exist");
		let new_stake = Stakes::<Test>::get(1, new_fnft_instance_id).expect("stake should exist");

		// validate stake and share as per ratio
		assert_eq!(existing_stake.stake, ratio.mul_floor(existing_stake_before_split.stake));
		assert_eq!(existing_stake.share, ratio.mul_floor(existing_stake_before_split.share));

		assert_eq!(existing_stake.reductions.get(&USDT::ID), Some(&0));

		assert_eq!(
			new_stake.stake,
			left_from_one_ratio.mul_floor(existing_stake_before_split.stake)
		);
		assert_eq!(
			new_stake.share,
			left_from_one_ratio.mul_floor(existing_stake_before_split.share)
		);

		assert_eq!(balance(PICA::ID, &FinancialNft::asset_account(&1, &0)), existing_stake.stake);
		assert_eq!(balance(XPICA::ID, &FinancialNft::asset_account(&1, &0)), existing_stake.share);

		assert_eq!(balance(PICA::ID, &FinancialNft::asset_account(&1, &1)), new_stake.stake);
		assert_eq!(balance(XPICA::ID, &FinancialNft::asset_account(&1, &1)), new_stake.share);

		assert_eq!(new_stake.reductions.get(&USDT::ID), Some(&0));

		helper::assert_last_event::<Test>(Event::StakingRewards(crate::Event::SplitPosition {
			positions: vec![(PICA::ID, 0, existing_stake.stake), (PICA::ID, 1, new_stake.stake)],
		}));
	});
}

// TODO(connor): Move unit tests for functions to one (or more) modules per function

#[test]
fn extend_should_not_allow_non_owner() {
	let staker = ALICE;
	let non_owner = BOB;
	let amount = 100_500;
	let duration_preset = ONE_HOUR;
	let total_rewards = 100;

	with_stake(
		staker,
		amount,
		duration_preset,
		total_rewards,
		false,
		|_pool_id, _unlock_penalty, _stake_duration, _staked_asset_id| {
			assert_noop!(
				StakingRewards::extend(Origin::signed(non_owner), 1, 0, 1_000),
				crate::Error::<Test>::OnlyStakeOwnerCanInteractWithStake
			);
		},
	)
}

#[test]
fn unstake_should_not_allow_non_owner() {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);

		create_rewards_pool_and_assert::<Test, runtime::Event>(get_default_reward_pool());

		process_and_progress_blocks::<StakingRewards, Test>(1);

		// far more than is necessary
		mint_assets([CHARLIE], [USDT::ID], USDT::units(100_000_000));
		add_to_rewards_pot_and_assert(CHARLIE, PICA::ID, USDT::ID, USDT::units(100_000_000));

		process_and_progress_blocks::<StakingRewards, Test>(1);

		mint_assets([BOB], [PICA::ID], PICA::units(200));
		let fnft_instance_id =
			stake_and_assert::<Test, runtime::Event>(BOB, PICA::ID, 100_500, ONE_HOUR);

		assert_noop!(
			StakingRewards::unstake(
				Origin::signed(DAVE),
				STAKING_FNFT_COLLECTION_ID,
				fnft_instance_id
			),
			crate::Error::<Test>::OnlyStakeOwnerCanInteractWithStake
		);
	})
}

#[test]
fn split_should_not_allow_non_owner() {
	let staker = ALICE;
	let non_owner = BOB;
	let amount = 100_500;
	let duration_preset = ONE_HOUR;
	let total_rewards = 100;

	with_stake(
		staker,
		amount,
		duration_preset,
		total_rewards,
		false,
		|_pool_id, _unlock_penalty, _stake_duration, _staked_asset_id| {
			assert_noop!(
				StakingRewards::unstake(Origin::signed(non_owner), 1, 0),
				crate::Error::<Test>::OnlyStakeOwnerCanInteractWithStake
			);
		},
	)
}

#[test]
fn unstake_should_work() {
	new_test_ext().execute_with(|| {
		next_block::<crate::Pallet<Test>, Test>();

		create_rewards_pool_and_assert::<Test, runtime::Event>(RewardRateBasedIncentive {
			owner: ALICE,
			asset_id: PICA::ID,
			start_block: 2,
			end_block: 100_000,
			reward_configs: [(
				USDT::ID,
				RewardConfig {
					max_rewards: USDT::units(1_000_000),
					reward_rate: RewardRate::per_second(USDT::units(1)),
				},
			)]
			.into_iter()
			.try_collect()
			.unwrap(),
			lock: default_lock_config(),
			share_asset_id: XPICA::ID,
			financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID,
			minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
		});

		// far more than is necessary
		mint_assets([CHARLIE], [USDT::ID], USDT::units(100_000_000));
		add_to_rewards_pot_and_assert(CHARLIE, PICA::ID, USDT::ID, USDT::units(100_000_000));

		next_block::<crate::Pallet<Test>, Test>();

		mint_assets([BOB], [PICA::ID], PICA::units(200));
		let fnft_instance_id =
			stake_and_assert::<Test, runtime::Event>(BOB, PICA::ID, PICA::units(100), ONE_HOUR);

		// 100 blocks * 6 seconds per block < 1 hour
		process_and_progress_blocks::<crate::Pallet<Test>, Test>(100);

		unstake_and_assert::<Test, runtime::Event>(
			BOB,
			STAKING_FNFT_COLLECTION_ID,
			fnft_instance_id,
			true,
		);
	})
}
mod claim {
	use super::*;

	#[test]
	fn should_not_allow_non_owner() {
		let staker = ALICE;
		let non_owner = BOB;
		let amount = 100_500;
		let duration_preset = ONE_HOUR;
		let total_rewards = 100;

		with_stake(
			staker,
			amount,
			duration_preset,
			total_rewards,
			false,
			|_pool_id, _unlock_penalty, _stake_duration, _staked_asset_id| {
				assert_noop!(
					StakingRewards::claim(Origin::signed(non_owner), 1, 0),
					crate::Error::<Test>::OnlyStakeOwnerCanInteractWithStake
				);
			},
		)
	}

	#[test]
	fn should_reward_correct_amount() {
		let staker = ALICE;
		let amount = 100_500;
		let duration_preset = ONE_HOUR;
		let total_rewards = 100;
		let claim = 100;

		with_stake(
			staker,
			amount,
			duration_preset,
			total_rewards,
			true,
			|pool_id, _unlock_penalty, _stake_duration, staked_asset_id| {
				let rewards_pool = StakingRewards::pools(pool_id).expect("rewards_pool expected");

				// Ensure that the value of the staked asset has **not** changed
				assert_eq!(balance(staked_asset_id, &staker), amount);
				process_and_progress_blocks::<StakingRewards, Test>(1);
				assert_ok!(StakingRewards::claim(Origin::signed(staker), 1, 0));
				assert_eq!(balance(staked_asset_id, &staker), amount);

				// Ensure that the value of the reward asset has changed
				for (rewarded_asset_id, _) in rewards_pool.rewards.iter() {
					assert_eq!(balance(*rewarded_asset_id, &staker), amount * 2 + claim);
					assert_eq!(
						balance(*rewarded_asset_id, &StakingRewards::pool_account_id(&pool_id)),
						amount * 2 - claim
					);
				}
			},
		);
	}

	#[test]
	fn should_not_allow_for_double_claim() {
		let staker = ALICE;
		let amount = 100_500;
		let duration_preset = ONE_HOUR;
		let total_rewards = 100;
		let claim = 100;

		with_stake(
			staker,
			amount,
			duration_preset,
			total_rewards,
			true,
			|pool_id, _unlock_penalty, _stake_duration, staked_asset_id| {
				let rewards_pool = StakingRewards::pools(pool_id).expect("rewards_pool expected");

				// First claim
				assert_ok!(StakingRewards::claim(Origin::signed(staker), 1, 0));
				// Ensure no change in staked asset
				assert_eq!(balance(staked_asset_id, &staker), amount);
				// Ensure change in reward asset
				for (rewarded_asset_id, _) in rewards_pool.rewards.iter() {
					assert_eq!(balance(*rewarded_asset_id, &staker), amount * 2 + claim);
					assert_eq!(
						balance(*rewarded_asset_id, &StakingRewards::pool_account_id(&pool_id)),
						amount * 2 - claim
					);
				}

				// Second claim, should not change balance
				assert_ok!(StakingRewards::claim(Origin::signed(staker), 1, 0));
				// Ensure no change in staked asset
				assert_eq!(balance(staked_asset_id, &staker), amount);
				// Ensure no change in reward asset
				for (rewarded_asset_id, _) in rewards_pool.rewards.iter() {
					assert_eq!(balance(*rewarded_asset_id, &staker), amount * 2 + claim);
					assert_eq!(
						balance(*rewarded_asset_id, &StakingRewards::pool_account_id(&pool_id)),
						amount * 2 - claim
					);
				}
			},
		);
	}

	#[test]
	fn should_change_stake_reductions_in_storage() {
		const AMOUNT: u128 = 100_500;
		const DURATION: u64 = ONE_HOUR;

		new_test_ext().execute_with(|| {
			process_and_progress_blocks::<StakingRewards, Test>(1);
			assert_ok!(StakingRewards::create_reward_pool(
				Origin::root(),
				get_default_reward_pool()
			));

			let staked_asset_id = PICA::ID;
			let rewards_pool =
				StakingRewards::pools(staked_asset_id).expect("rewards_pool expected. QED");

			// far more than is necessary
			mint_assets([CHARLIE], [USDT::ID], USDT::units(100_000_000));
			add_to_rewards_pot_and_assert(CHARLIE, PICA::ID, USDT::ID, USDT::units(100_000_000));

			process_and_progress_blocks::<StakingRewards, Test>(1);

			mint_assets([ALICE], [PICA::ID], PICA::units(100_000_000));
			let _ = stake_and_assert::<Test, runtime::Event>(ALICE, PICA::ID, AMOUNT, DURATION);

			assert_eq!(balance(staked_asset_id, &ALICE), PICA::units(100_000_000) - AMOUNT);

			// first staker should have 0 reductions
			assert_eq!(
				Stakes::<Test>::get(1, 0)
					.expect("expected stake. QED")
					.reductions
					.get(&USDT::ID),
				Some(&0)
			);

			assert_extrinsic_event::<Test, runtime::Event, _, _, _>(
				StakingRewards::claim(Origin::signed(ALICE), 1, 0),
				crate::Event::Claimed { owner: ALICE, fnft_collection_id: 1, fnft_instance_id: 0 },
			);

			let stake = Stakes::<Test>::get(1, 0).expect("expected stake. QED");

			// should be 1 block's worth of claims
			assert_eq!(stake.reductions.get(&USDT::ID), Some(&60));

			process_and_progress_blocks::<StakingRewards, Test>(1);

			// reductions don't change per block
			assert_eq!(stake.reductions.get(&USDT::ID), Some(&60));
		});
	}

	#[test]
	fn should_change_reward_pool_claimed_in_storage() {
		let staker = ALICE;
		let amount = 100_500;
		let duration_preset = ONE_HOUR;
		let total_rewards = 100;
		let claim = 100;

		with_stake(
			staker,
			amount,
			duration_preset,
			total_rewards,
			true,
			|pool_id, _unlock_penalty, _stake_duration, _staked_asset_id| {
				assert_ok!(StakingRewards::claim(Origin::signed(staker), 1, 0));

				assert_last_event::<Test, _>(|e| {
					matches!(&e.event,
            		Event::StakingRewards(crate::Event::Claimed{ owner, fnft_collection_id, fnft_instance_id })
            		if owner == &staker && fnft_collection_id == &1 && fnft_instance_id == &0)
				});

				let rewards_pool = StakingRewards::pools(pool_id).expect("rewards_pool expected");

				assert_eq!(
					rewards_pool
						.rewards
						.get(&USDT::ID)
						.expect("expected value. QED")
						.claimed_rewards,
					claim
				);
			},
		);
	}
}

#[test]
fn duration_presets_are_required() {
	new_test_ext().execute_with(|| {
		assert_err!(
			StakingRewards::create_reward_pool(
				Origin::root(),
				RewardRateBasedIncentive {
					owner: ALICE,
					asset_id: PICA::ID,
					start_block: 2,
					end_block: 5,
					reward_configs: default_reward_config(),
					lock: LockConfig {
						duration_presets: BoundedBTreeMap::new(),
						unlock_penalty: Perbill::from_percent(5),
					},
					share_asset_id: XPICA::ID,
					financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID,
					minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
				},
			),
			crate::Error::<Test>::NoDurationPresetsProvided
		);
	});
}

/// Runs code inside of `new_test_ext().execute_with` closure while creating a stake with the given
/// values.
///
/// `execute` closure will provide:
/// - `pool_id`
/// - `unlock_penalty`
/// - `stake_duration`
/// - `staked_asset_id`
fn with_stake<R>(
	staker: Public,
	amount: u128,
	duration: DurationSeconds,
	total_rewards: u128,
	should_claim: bool,
	execute: impl FnOnce(u128, Perbill, u64, u128) -> R,
) -> R {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);
		assert_ok!(StakingRewards::create_reward_pool(Origin::root(), get_default_reward_pool()));

		let staked_asset_id = PICA::ID;
		let rewards_pool =
			StakingRewards::pools(staked_asset_id).expect("rewards_pool expected. QED");

		mint_assets(
			[staker, StakingRewards::pool_account_id(&staked_asset_id)],
			rewards_pool.rewards.keys().copied().chain([staked_asset_id]),
			amount.saturating_mul(2),
		);

		update_total_rewards_and_total_shares_in_rewards_pool(staked_asset_id, total_rewards);

		process_and_progress_blocks::<StakingRewards, Test>(1);
		let fnft_instance_id =
			stake_and_assert::<Test, runtime::Event>(staker, PICA::ID, amount, duration);
		// assert_ok!(StakingRewards::stake(Origin::signed(staker), pool_id, amount, duration));
		assert_eq!(balance(staked_asset_id, &staker), amount);

		let stake = StakingRewards::stakes(1, 0).expect("stake expected. QED");
		let unlock_penalty = stake.lock.unlock_penalty;
		let stake_duration = stake.lock.duration;

		if should_claim {
			// update_reductions(&mut stake.reductions, claim);
			assert_ok!(StakingRewards::claim(
				Origin::signed(staker),
				STAKING_FNFT_COLLECTION_ID,
				fnft_instance_id
			));
		}

		execute(staked_asset_id, unlock_penalty, stake_duration, staked_asset_id)
	})
}

fn create_default_reward_pool() {
	assert_extrinsic_event::<Test, _, _, _, _>(
		StakingRewards::create_reward_pool(
			Origin::root(),
			RewardRateBasedIncentive {
				owner: ALICE,
				asset_id: PICA::ID,
				start_block: 2,
				end_block: 5,
				reward_configs: default_reward_config(),
				lock: default_lock_config(),
				share_asset_id: XPICA::ID,
				financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID,
				minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
			},
		),
		crate::Event::<Test>::RewardPoolCreated { pool_id: PICA::ID, owner: ALICE, end_block: 5 },
	);
}

/// Creates a PICA staking reward pool. Calls [`default_reward_pool`] and [`default_lock_config`].
fn get_default_reward_pool() -> RewardPoolConfigurationOf<Test> {
	RewardRateBasedIncentive {
		owner: ALICE,
		asset_id: PICA::ID,
		start_block: 2,
		end_block: 5,
		reward_configs: default_reward_config(),
		lock: default_lock_config(),
		share_asset_id: XPICA::ID,
		financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID,
		minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
	}
}

fn default_lock_config() -> LockConfig<MaxStakingDurationPresets> {
	LockConfig {
		duration_presets: [
			(ONE_HOUR, FixedU64::from_rational(101, 100).try_into_validated().expect(">= 1")), /* 1% */
			(ONE_MINUTE, FixedU64::from_rational(1_001, 1_000).try_into_validated().expect(">= 1")), /* 0.1% */
		]
		.into_iter()
		.try_collect()
		.unwrap(),
		unlock_penalty: Perbill::from_percent(5),
	}
}

fn default_reward_config() -> BoundedBTreeMap<u128, RewardConfig<u128>, MaxRewardConfigsPerPool> {
	[(
		USDT::ID,
		RewardConfig { max_rewards: 100_u128, reward_rate: RewardRate::per_second(10_u128) },
	)]
	.into_iter()
	.try_collect()
	.unwrap()
}

pub fn assert_last_event<T, F>(matcher: F)
where
	T: Config,
	F: FnOnce(&EventRecord<Event, H256>) -> bool,
{
	assert!(matcher(System::events().last().expect("events expected")));
}

fn mint_assets(
	accounts: impl IntoIterator<Item = Public>,
	asset_ids: impl IntoIterator<Item = u128>,
	amount: u128,
) {
	let asset_ids = Vec::from_iter(asset_ids);
	for account in accounts {
		for asset_id in &asset_ids {
			<<Test as crate::Config>::Assets as Mutate<
				<Test as frame_system::Config>::AccountId,
			>>::mint_into(*asset_id, &account, amount)
			.expect("an asset minting expected");
		}
	}
}

fn balance(asset_id: u128, account: &Public) -> u128 {
	<<Test as crate::Config>::Assets as Inspect<<Test as frame_system::Config>::AccountId>>::balance(
		asset_id, account,
	)
}

fn update_total_rewards_and_total_shares_in_rewards_pool(pool_id: u128, total_rewards: u128) {
	let mut rewards_pool = StakingRewards::pools(pool_id).expect("rewards_pool expected");
	let mut inner_rewards = rewards_pool.rewards.into_inner();
	for (_asset_id, reward) in inner_rewards.iter_mut() {
		reward.total_rewards += total_rewards;
	}
	rewards_pool.rewards = inner_rewards.try_into().expect("rewards expected");
	RewardPools::<Test>::insert(pool_id, rewards_pool);
}

fn btree_map<K: Ord, V, Max: sp_runtime::traits::Get<u32>>(
	iter: impl IntoIterator<Item = (K, V)>,
) -> BoundedBTreeMap<K, V, Max> {
	iter.into_iter().collect::<BTreeMap<K, V>>().try_into().unwrap()
}
