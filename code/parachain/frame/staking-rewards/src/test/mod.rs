#![allow(clippy::disallowed_methods)] // disabled for now to make running clippy on the tests easier

// re-export this for the benchmarks tests
pub(crate) use crate::runtime::{new_test_ext, Test};

use crate::{
	claim_of_stake,
	runtime::*,
	test::prelude::MINIMUM_STAKING_AMOUNT,
	test_helpers::{
		add_to_rewards_pot_and_assert, create_rewards_pool_and_assert, split_and_assert,
		stake_and_assert, unstake_and_assert,
	},
	FinancialNftInstanceIdOf, Pallet, RewardPoolConfigurationOf, RewardPools, Stakes,
};

use composable_support::validation::TryIntoValidated;
use composable_tests_helpers::test::{
	block::{next_block, process_and_progress_blocks, process_and_progress_blocks_with},
	currency::{BTC, PICA, USDT, XPICA},
	helper::RuntimeTrait,
};

use crate::test::prelude::block_seconds;
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
	assert_err, assert_noop, assert_ok, bounded_btree_map,
	traits::{
		fungibles::{Inspect, InspectHold, Mutate},
		tokens::nonfungibles::InspectEnumerable,
		TryCollect,
	},
	BoundedBTreeMap,
};
use orml_traits::MultiCurrency;
use proptest::prelude::*;
use sp_arithmetic::{fixed_point::FixedU64, Perbill, Permill};
use sp_core::sr25519::Public;
use sp_runtime::{traits::One, PerThing};
use sp_std::collections::{btree_map::BTreeMap, btree_set::BTreeSet};

use self::prelude::init_logger;

pub(crate) mod prelude;

mod test_reward_accumulation_hook;
mod test_update_reward_pools;

#[test]
fn test_create_reward_pool() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		create_default_reward_pool();

		let fnft_collection_id =
			RewardPools::<Test>::get(PICA::ID).expect("Pool exists").financial_nft_asset_id;

		assert_eq!(
			FinancialNft::collections().collect::<BTreeSet<_>>(),
			BTreeSet::from([fnft_collection_id])
		);
		assert_eq!(
			<StakingRewards as FinancialNftProtocol>::collection_asset_ids(),
			vec![fnft_collection_id]
		);
	});
}

#[test]
fn duration_presets_minimum_is_1() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(StakingRewards::create_reward_pool(
			RuntimeOrigin::root(),
			RewardRateBasedIncentive {
				owner: ALICE,
				asset_id: PICA::ID,
				start_block: 2,
				reward_configs: default_reward_config(),
				lock: LockConfig {
					duration_multipliers: bounded_btree_map! {
						// 0.1%
						ONE_MINUTE => FixedU64::from_rational(110, 100).try_into_validated().expect(">= 1"),
					}
					.into(),
					unlock_penalty: Perbill::from_percent(5),
				},
				minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
			},
		));
	});
}

#[test]
fn zero_length_duration_preset_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(StakingRewards::create_reward_pool(
			RuntimeOrigin::root(),
			RewardRateBasedIncentive {
				owner: ALICE,
				asset_id: PICA::ID,
				start_block: 2,
				reward_configs: default_reward_config(),
				lock: LockConfig {
					duration_multipliers: bounded_btree_map! {
						0 => FixedU64::one().try_into_validated().expect(">= 1"),
					}
					.into(),
					unlock_penalty: Perbill::from_percent(5),
				},
				minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
			},
		));
	});
}

#[test]
fn create_staking_reward_pool_should_fail_when_pool_asset_id_is_zero() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		assert_err!(
			StakingRewards::create_reward_pool(
				RuntimeOrigin::root(),
				RewardRateBasedIncentive {
					owner: ALICE,
					asset_id: 0,
					// end block can't be before the current block
					start_block: 2,
					reward_configs: default_reward_config(),
					lock: default_lock_config(),
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
				RuntimeOrigin::root(),
				RewardRateBasedIncentive {
					owner: ALICE,
					asset_id: PICA::ID,
					start_block: 2,
					reward_configs: default_reward_config(),
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
						unlock_penalty: Perbill::from_percent(99),
					},
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
				RuntimeOrigin::root(),
				RewardRateBasedIncentive {
					owner: ALICE,
					asset_id: PICA::ID,
					start_block: 2,
					reward_configs: default_reward_config(),
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
						unlock_penalty: Perbill::from_percent(60),
					},
					minimum_staking_amount: 10,
				}
			),
			crate::Error::<Test>::SlashedMinimumStakingAmountTooLow,
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
		assert_ok!(StakingRewards::create_reward_pool(
			RuntimeOrigin::root(),
			get_default_reward_pool()
		));

		assert_noop!(
			StakingRewards::stake(RuntimeOrigin::signed(staker), pool_id, amount, duration),
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
			StakingRewards::stake(RuntimeOrigin::signed(ALICE), PICA::ID, AMOUNT, ONE_HOUR),
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
		assert_ok!(StakingRewards::create_reward_pool(
			RuntimeOrigin::root(),
			get_default_reward_pool()
		));

		assert_noop!(
			StakingRewards::stake(RuntimeOrigin::signed(staker), pool_id, amount, duration),
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

		create_rewards_pool_and_assert::<Test>(RewardRateBasedIncentive {
			owner: ALICE,
			asset_id: PICA::ID,
			start_block: 2,
			reward_configs: default_reward_config(),
			lock: default_lock_config(),
			minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
		});

		process_and_progress_blocks::<StakingRewards, Test>(1);

		let amount = 15_000;
		let fnft_collection_id =
			RewardPools::<Test>::get(PICA::ID).expect("Pool exists").financial_nft_asset_id;
		let original_fnft_instance_id =
			stake_and_assert::<Test>(ALICE, PICA::ID, amount, ONE_MINUTE);

		// Original stake is less than the minimum and new stake is greater.
		let ratio = Permill::from_rational(1_u32, 3_u32);

		assert_eq!(ratio.mul_floor(amount), 4_999);
		assert_eq!(ratio.left_from_one().mul_ceil(amount), 10_001);

		assert_noop!(
			StakingRewards::split(
				RuntimeOrigin::signed(staker),
				fnft_collection_id,
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
				RuntimeOrigin::signed(staker),
				fnft_collection_id,
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
				RuntimeOrigin::signed(staker),
				fnft_collection_id,
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

		create_rewards_pool_and_assert::<Test>(RewardRateBasedIncentive {
			owner: ALICE,
			asset_id: PICA::ID,
			start_block: 2,
			reward_configs: default_reward_config(),
			lock: default_lock_config(),
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

		let fnft_collection_id =
			RewardPools::<Test>::get(PICA::ID).expect("Pool exists").financial_nft_asset_id;
		let original_fnft_instance_id =
			stake_and_assert::<Test>(ALICE, PICA::ID, amount, ONE_MINUTE);

		// split_and_assert checks for loss of assets
		split_and_assert::<Test>(
			ALICE,
			fnft_collection_id,
			original_fnft_instance_id,
			ratio.try_into_validated().unwrap(),
		);
	})
}

#[test]
#[ignore = "Fix `FinancialNftProtocol::value_of` Implementation [PICA-175]"]
fn stake_in_case_of_zero_inflation_should_work() {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);

		assert_ok!(StakingRewards::create_reward_pool(
			RuntimeOrigin::root(),
			get_default_reward_pool()
		));
		process_and_progress_blocks::<StakingRewards, Test>(1);
		let staker: Public = ALICE;
		let amount: u128 = 100_500_u128;
		let duration_preset: u64 = ONE_HOUR;

		let staked_asset_id = PICA::ID;
		mint_assets([staker], [staked_asset_id], amount * 2);

		let fnft_collection_id = RewardPools::<Test>::get(staked_asset_id)
			.expect("Pool exists")
			.financial_nft_asset_id;
		let fnft_instance_id = Test::assert_extrinsic_event_with(
			StakingRewards::stake(RuntimeOrigin::signed(staker), PICA::ID, amount, duration_preset),
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

		let fnft_asset_account =
			FinancialNft::asset_account(&fnft_collection_id, &fnft_instance_id);

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
			Stakes::<Test>::get(fnft_collection_id, fnft_instance_id),
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

		Test::assert_last_event_with(|event| {
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

		process_and_progress_blocks::<StakingRewards, Test>(1);

		create_rewards_pool_and_assert::<Test>(get_default_reward_pool());

		process_and_progress_blocks::<StakingRewards, Test>(1);

		let staked_asset_id = PICA::ID;
		mint_assets([ALICE], [staked_asset_id], AMOUNT * 2);

		let fnft_collection_id = RewardPools::<Test>::get(staked_asset_id)
			.expect("Pool exists")
			.financial_nft_asset_id;
		let fnft_instance_id = stake_and_assert::<Test>(ALICE, PICA::ID, AMOUNT, DURATION_PRESET);
		let fnft_asset_account =
			FinancialNft::asset_account(&fnft_collection_id, &fnft_instance_id);

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
			StakingRewards::stakes(fnft_collection_id, fnft_instance_id),
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
		currency::{Currency, USDT},
		helper::RuntimeTrait,
	};
	use composable_traits::{
		staking::{
			lock::{Lock, LockConfig},
			RewardConfig, RewardPoolConfiguration, RewardRate, Stake,
		},
		time::{ONE_HOUR, ONE_MINUTE},
	};
	use frame_support::{bounded_btree_map, traits::UnixTime};
	use sp_arithmetic::fixed_point::FixedU64;
	use sp_runtime::Perbill;

	use crate::{
		runtime::{RuntimeOrigin, StakingRewards, ALICE, BOB},
		test::{btree_map, mint_assets, prelude::MINIMUM_STAKING_AMOUNT, Test},
		test_helpers::{
			add_to_rewards_pot_and_assert, create_rewards_pool_and_assert, stake_and_assert,
		},
		Pallet, RewardPools, Stakes,
	};

	use super::new_test_ext;

	#[allow(non_camel_case_types)]
	type STAKED_ASSET = Currency<1337, 12>;

	#[test]
	fn with_additional_stake() {
		new_test_ext().execute_with(|| {
			process_and_progress_blocks::<StakingRewards, Test>(1);

			let current_block_number = frame_system::Pallet::<Test>::block_number();

			create_rewards_pool_and_assert::<Test>(
				RewardPoolConfiguration::RewardRateBasedIncentive {
					owner: ALICE,
					asset_id: STAKED_ASSET::ID,
					start_block: current_block_number + 1,
					reward_configs: btree_map([(
						USDT::ID,
						RewardConfig { reward_rate: RewardRate::per_second(USDT::units(1)) },
					)]),
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
					minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
				},
			);

			mint_assets([ALICE], [USDT::ID], USDT::units(100_000));
			add_to_rewards_pot_and_assert::<Test>(
				ALICE,
				STAKED_ASSET::ID,
				USDT::ID,
				USDT::units(100_000),
				// should NOT emit the `resumed` event since the pot was funded before the pool
				// started
				false,
			);

			// progress to the start block
			process_and_progress_blocks::<StakingRewards, Test>(1);

			Test::assert_event(crate::Event::<Test>::RewardPoolStarted {
				pool_id: STAKED_ASSET::ID,
			});

			let staked_amount = STAKED_ASSET::units(5);
			let extended_amount = STAKED_ASSET::units(6);
			let existential_deposit = 1_000_u128;

			mint_assets(
				[BOB],
				[STAKED_ASSET::ID],
				staked_amount + extended_amount + existential_deposit,
			);

			let fnft_collection_id = RewardPools::<Test>::get(STAKED_ASSET::ID)
				.expect("Pool exists")
				.financial_nft_asset_id;
			let fnft_instance_id =
				stake_and_assert::<Test>(BOB, STAKED_ASSET::ID, staked_amount, ONE_MINUTE);

			process_and_progress_blocks::<StakingRewards, Test>(10);

			Test::assert_extrinsic_event(
				StakingRewards::extend(
					RuntimeOrigin::signed(BOB),
					fnft_collection_id,
					fnft_instance_id,
					extended_amount,
				),
				crate::Event::<Test>::StakeAmountExtended {
					fnft_collection_id,
					fnft_instance_id,
					amount: extended_amount,
				},
			);

			let stake_after_extend = Stakes::<Test>::get(fnft_collection_id, fnft_instance_id)
				.expect("stake should exist");

			let rewards_pool =
				StakingRewards::pools(STAKED_ASSET::ID).expect("rewards pool should exist");

			assert_eq!(
				stake_after_extend,
				Stake {
					reward_pool_id: STAKED_ASSET::ID,
					stake: staked_amount + extended_amount,
					share: Pallet::<Test>::boosted_amount(
						rewards_pool
							.lock
							.duration_multipliers
							.multiplier(ONE_MINUTE)
							.copied()
							.unwrap(),
						staked_amount + extended_amount
					)
					.expect("boosted amount calculation should not fail"),
					// 5 units already staked, 6 more units added, 10 blocks worth of rewards
					// (as during one of the blocks the reward accumulation was paused)
					// already accumulated at 1 unit per second, this is the resulting inflation:
					// (60*10^12) * ((6*10^12) * 1.01) / ((5*10^12) * 1.01)
					reductions: btree_map([(USDT::ID, USDT::units(72))]),
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

			create_rewards_pool_and_assert::<Test>(
				RewardPoolConfiguration::RewardRateBasedIncentive {
					owner: ALICE,
					asset_id: STAKED_ASSET::ID,
					start_block: current_block_number + 1,
					reward_configs: btree_map([(
						USDT::ID,
						RewardConfig { reward_rate: RewardRate::per_second(USDT::units(1)) },
					)]),
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
					minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
				},
			);

			process_and_progress_blocks::<StakingRewards, Test>(1);

			mint_assets([ALICE], [USDT::ID], USDT::units(100_000));
			add_to_rewards_pot_and_assert::<Test>(
				ALICE,
				STAKED_ASSET::ID,
				USDT::ID,
				USDT::units(100_000),
				// should NOT emit the `resume` event due to progressing to the start block (block
				// 2) above
				false,
			);

			let staked_amount = STAKED_ASSET::units(5);
			let extended_amount = STAKED_ASSET::units(0);
			let existential_deposit = 1_000_u128;

			mint_assets(
				[BOB],
				[STAKED_ASSET::ID],
				staked_amount + extended_amount + existential_deposit,
			);

			let fnft_collection_id = RewardPools::<Test>::get(STAKED_ASSET::ID)
				.expect("Pool exists")
				.financial_nft_asset_id;
			let fnft_instance_id =
				stake_and_assert::<Test>(BOB, STAKED_ASSET::ID, staked_amount, ONE_MINUTE);

			process_and_progress_blocks::<StakingRewards, Test>(10);

			Test::assert_extrinsic_event(
				StakingRewards::extend(
					RuntimeOrigin::signed(BOB),
					fnft_collection_id,
					fnft_instance_id,
					extended_amount,
				),
				crate::Event::<Test>::StakeAmountExtended {
					fnft_collection_id,
					fnft_instance_id,
					amount: extended_amount,
				},
			);

			let stake_after_extend = Stakes::<Test>::get(fnft_collection_id, fnft_instance_id)
				.expect("stake should exist");

			let rewards_pool =
				StakingRewards::pools(STAKED_ASSET::ID).expect("rewards pool should exist");

			assert_eq!(
				stake_after_extend,
				Stake {
					reward_pool_id: STAKED_ASSET::ID,
					stake: staked_amount + extended_amount,
					share: Pallet::<Test>::boosted_amount(
						rewards_pool
							.lock
							.duration_multipliers
							.multiplier(ONE_MINUTE)
							.copied()
							.unwrap(),
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
			StakingRewards::unstake(RuntimeOrigin::signed(staker), 1, 0),
			crate::Error::<Test>::FnftNotFound
		);
	});
}

#[test]
fn not_owner_of_stake_can_not_unstake() {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);
		assert_ok!(StakingRewards::create_reward_pool(
			RuntimeOrigin::root(),
			get_default_reward_pool()
		));
		let owner = ALICE;
		let not_owner = BOB;
		let pool_id = PICA::ID;
		let amount = 100_500_u32.into();
		let duration_preset = ONE_HOUR;
		let fnft_collection_id =
			RewardPools::<Test>::get(pool_id).expect("Pool exists").financial_nft_asset_id;
		assert_ne!(owner, not_owner);

		let staked_asset_id = PICA::ID;
		mint_assets([owner, not_owner], [staked_asset_id], amount * 2);

		process_and_progress_blocks::<StakingRewards, Test>(1);
		assert_ok!(StakingRewards::stake(
			RuntimeOrigin::signed(owner),
			pool_id,
			amount,
			duration_preset
		));

		assert_noop!(
			StakingRewards::unstake(RuntimeOrigin::signed(not_owner), fnft_collection_id, 0),
			crate::Error::<Test>::OnlyStakeOwnerCanInteractWithStake,
		);
	});
}

#[test]
fn unstake_in_case_of_zero_claims_and_early_unlock_should_work() {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);

		create_rewards_pool_and_assert::<Test>(get_default_reward_pool());

		process_and_progress_blocks::<StakingRewards, Test>(1);

		// far more than is necessary
		mint_assets([CHARLIE], [USDT::ID], USDT::units(100_000_000));
		add_to_rewards_pot_and_assert::<Test>(
			CHARLIE,
			PICA::ID,
			USDT::ID,
			USDT::units(100_000_000),
			// should NOT emit the `resume` event due to progressing to the start block (block 2)
			// above
			false,
		);

		process_and_progress_blocks::<StakingRewards, Test>(1);

		mint_assets([BOB], [PICA::ID], PICA::units(200));
		let fnft_collection_id =
			RewardPools::<Test>::get(PICA::ID).expect("Pool exists").financial_nft_asset_id;
		let fnft_instance_id = stake_and_assert::<Test>(BOB, PICA::ID, 100_500, ONE_HOUR);

		// TODO(benluelo): Proper test helper for claim
		assert_ok!(StakingRewards::claim(
			RuntimeOrigin::signed(BOB),
			fnft_collection_id,
			fnft_instance_id
		));

		let rewards_pool = RewardPools::<Test>::get(PICA::ID).unwrap();

		for (reward_asset_id, reward) in rewards_pool.rewards {
			assert_eq!(
				claim_of_stake::<Test>(
					&Stakes::<Test>::get(fnft_collection_id, fnft_instance_id).unwrap(),
					&rewards_pool.share_asset_id,
					&reward,
					&reward_asset_id
				),
				Ok(0)
			);
		}

		unstake_and_assert::<Test>(BOB, fnft_collection_id, fnft_instance_id, true);
	});
}

#[test]
fn unstake_in_case_of_not_zero_claims_and_early_unlock_should_work() {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);

		create_rewards_pool_and_assert::<Test>(get_default_reward_pool());

		process_and_progress_blocks::<StakingRewards, Test>(1);

		// far more than is necessary
		mint_assets([CHARLIE], [USDT::ID], USDT::units(100_000_000));
		add_to_rewards_pot_and_assert::<Test>(
			CHARLIE,
			PICA::ID,
			USDT::ID,
			USDT::units(100_000_000),
			// should NOT emit the `resume` event due to progressing to the start block (block 2)
			// above
			false,
		);

		process_and_progress_blocks::<StakingRewards, Test>(1);

		mint_assets([BOB], [PICA::ID], PICA::units(200));
		let fnft_collection_id =
			RewardPools::<Test>::get(PICA::ID).expect("Pool exists").financial_nft_asset_id;
		let fnft_instance_id = stake_and_assert::<Test>(BOB, PICA::ID, 100_500, ONE_HOUR);

		process_and_progress_blocks::<StakingRewards, Test>(1);

		unstake_and_assert::<Test>(BOB, fnft_collection_id, fnft_instance_id, true);
	});
}

#[test]
fn unstake_in_case_of_not_zero_claims_and_not_early_unlock_should_work() {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);

		create_rewards_pool_and_assert::<Test>(get_default_reward_pool());

		process_and_progress_blocks::<StakingRewards, Test>(1);

		// far more than is necessary
		mint_assets([CHARLIE], [USDT::ID], USDT::units(100_000_000));
		add_to_rewards_pot_and_assert::<Test>(
			CHARLIE,
			PICA::ID,
			USDT::ID,
			USDT::units(100_000_000),
			// should NOT emit the `resume` event due to progressing to the start block (block 2)
			// above
			false,
		);

		process_and_progress_blocks::<StakingRewards, Test>(1);

		mint_assets([BOB], [PICA::ID], PICA::units(200));
		let fnft_collection_id =
			RewardPools::<Test>::get(PICA::ID).expect("Pool exists").financial_nft_asset_id;
		let fnft_instance_id = stake_and_assert::<Test>(BOB, PICA::ID, 100_500, ONE_HOUR);

		// 700 blocks * 6 seconds per block > 1 hour
		process_and_progress_blocks::<StakingRewards, Test>(700);

		unstake_and_assert::<Test>(BOB, fnft_collection_id, fnft_instance_id, false);
	});
}

#[test]
fn test_transfer_reward() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let pool_init_config = get_default_reward_pool();
		assert_ok!(StakingRewards::create_reward_pool(RuntimeOrigin::root(), pool_init_config));
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
			reward_configs: default_reward_config(),
			lock: default_lock_config(),
			minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
		};

		Test::assert_extrinsic_event(
			StakingRewards::create_reward_pool(RuntimeOrigin::root(), pool_init_config),
			crate::Event::<Test>::RewardPoolCreated { pool_id: PICA::ID, owner: ALICE },
		);
		process_and_progress_blocks::<StakingRewards, Test>(1);

		mint_assets([ALICE], [PICA::ID], PICA::units(2000));

		let fnft_collection_id =
			RewardPools::<Test>::get(PICA::ID).expect("Pool exits").financial_nft_asset_id;
		let existing_fnft_instance_id =
			stake_and_assert::<Test>(ALICE, PICA::ID, PICA::units(1_000), ONE_HOUR);
		let share_asset_id = RewardPools::<Test>::get(PICA::ID).expect("Pool exits").share_asset_id;

		let existing_stake_before_split =
			Stakes::<Test>::get(fnft_collection_id, existing_fnft_instance_id)
				.expect("stake should exist");

		let ratio = Permill::from_rational(1_u32, 7_u32);
		let left_from_one_ratio = ratio.left_from_one();

		let new_fnft_instance_id = split_and_assert::<Test>(
			ALICE,
			fnft_collection_id,
			0,
			ratio.try_into_validated().expect("valid split ratio"),
		);

		let existing_stake = Stakes::<Test>::get(fnft_collection_id, existing_fnft_instance_id)
			.expect("stake should exist");
		let new_stake = Stakes::<Test>::get(fnft_collection_id, new_fnft_instance_id)
			.expect("stake should exist");

		Test::assert_last_event(RuntimeEvent::StakingRewards(crate::Event::SplitPosition {
			positions: vec![
				(fnft_collection_id, 0, existing_stake.stake),
				(fnft_collection_id, 1, new_stake.stake),
			],
		}));

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

		assert_eq!(
			balance(PICA::ID, &FinancialNft::asset_account(&fnft_collection_id, &0)),
			existing_stake.stake
		);
		assert_eq!(
			balance(share_asset_id, &FinancialNft::asset_account(&fnft_collection_id, &0)),
			existing_stake.share
		);

		assert_eq!(
			balance(PICA::ID, &FinancialNft::asset_account(&fnft_collection_id, &1)),
			new_stake.stake
		);
		assert_eq!(
			balance(share_asset_id, &FinancialNft::asset_account(&fnft_collection_id, &1)),
			new_stake.share
		);

		assert_eq!(new_stake.reductions.get(&USDT::ID), Some(&0));
	});
}

#[test]
fn split_positions_accrue_same_as_original_position() {
	fn create_pool_and_stake() -> FinancialNftInstanceIdOf<Test> {
		create_rewards_pool_and_assert::<Test>(RewardRateBasedIncentive {
			owner: ALICE,
			asset_id: PICA::ID,
			start_block: 2,
			reward_configs: [(
				USDT::ID,
				RewardConfig { reward_rate: RewardRate::per_second(USDT::units(1)) },
			)]
			.into_iter()
			.try_collect()
			.unwrap(),
			lock: default_lock_config(),
			minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
		});

		process_and_progress_blocks::<StakingRewards, Test>(1);

		mint_assets([BOB], [USDT::ID], USDT::units(1_000));
		add_to_rewards_pot_and_assert::<Test>(
			BOB,
			PICA::ID,
			USDT::ID,
			USDT::units(1_000),
			// should NOT emit the `resume` event due to progressing to the start block (block 2)
			// above
			false,
		);

		mint_assets([ALICE], [PICA::ID], PICA::units(2000));
		stake_and_assert::<Test>(ALICE, PICA::ID, PICA::units(1_000), ONE_HOUR)
	}

	fn process_blocks() {
		process_and_progress_blocks_with::<StakingRewards, Test>(BLOCKS_TO_ACCRUE_FOR, || {
			<Test as RuntimeTrait<crate::Event<Test>>>::assert_no_event(
				crate::Event::RewardPoolPaused { pool_id: PICA::ID, asset_id: USDT::ID },
			);
		});
	}

	const BLOCKS_TO_ACCRUE_FOR: usize = 150;

	let accrued_without_split = new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let existing_fnft_instance_id = create_pool_and_stake();
		let fnft_collection_id =
			RewardPools::<Test>::get(PICA::ID).expect("Pool exists").financial_nft_asset_id;

		process_blocks();

		crate::Pallet::<Test>::claim(
			RuntimeOrigin::signed(ALICE),
			fnft_collection_id,
			existing_fnft_instance_id,
		)
		.unwrap();

		Tokens::balance(USDT::ID, &ALICE)
	});

	let accrued_with_split = new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let existing_fnft_instance_id = create_pool_and_stake();
		let fnft_collection_id =
			RewardPools::<Test>::get(PICA::ID).expect("Pool exists").financial_nft_asset_id;

		let ratio = Permill::from_rational(1_u32, 7_u32);

		let new_fnft_instance_id = split_and_assert::<Test>(
			ALICE,
			fnft_collection_id,
			existing_fnft_instance_id,
			ratio.try_into_validated().expect("valid split ratio"),
		);

		process_blocks();

		crate::Pallet::<Test>::claim(
			RuntimeOrigin::signed(ALICE),
			fnft_collection_id,
			existing_fnft_instance_id,
		)
		.unwrap();
		crate::Pallet::<Test>::claim(
			RuntimeOrigin::signed(ALICE),
			fnft_collection_id,
			new_fnft_instance_id,
		)
		.unwrap();

		Tokens::balance(USDT::ID, &ALICE)
	});

	assert_eq!(accrued_with_split, accrued_without_split)
}

#[test]
fn claim_with_insufficient_pot_funds() {
	init_logger();

	new_test_ext().execute_with(|| {
		next_block::<Pallet<Test>, Test>();

		create_rewards_pool_and_assert::<Test>(RewardRateBasedIncentive {
			owner: ALICE,
			asset_id: PICA::ID,
			start_block: 10,
			reward_configs: bounded_btree_map! {
				// 0.01 USDT/second
				USDT::ID => RewardConfig {
					reward_rate: RewardRate::per_second(USDT::units(1)/ 100)
				}
			},
			lock: LockConfig {
				duration_multipliers: bounded_btree_map! {
					ONE_HOUR => FixedU64::from_rational(101, 100).try_into_validated().expect(">= 1"), /* 1% */
					ONE_MINUTE => FixedU64::from_rational(1_001, 1_000).try_into_validated().expect(">= 1"), /* 0.1% */
				}
				.into(),
				unlock_penalty: Perbill::from_percent(5),
			},
			minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
		});

		// add 2 USDT to the pot (200 periods, 83.3̅ blocks (84 full blocks)
		mint_assets([CHARLIE], [USDT::ID], USDT::units(2));
		add_to_rewards_pot_and_assert::<Test>(
			CHARLIE,
			PICA::ID,
			USDT::ID,
			USDT::units(2),
			// should NOT emit the `resume` event since the pot is being funded before the pool
			// starts
			false,
		);

		// run until the pool starts
		process_and_progress_blocks::<Pallet<Test>, Test>(9);

		Test::assert_event(crate::Event::<Test>::RewardPoolStarted { pool_id: PICA::ID });

		// bob gets 10 blocks of rewards to himself, then dave stakes
		// they both then get 10 blocks of rewards split between them

		mint_assets([BOB], [PICA::ID], PICA::units(100));
		let bob_stake_id = stake_and_assert::<Test>(BOB, PICA::ID, PICA::units(10), ONE_MINUTE);

		process_and_progress_blocks::<Pallet<Test>, Test>(10);

		mint_assets([DAVE], [PICA::ID], PICA::units(100));
		let fnft_collection_id =
			RewardPools::<Test>::get(PICA::ID).expect("Pool exists").financial_nft_asset_id;
		let dave_stake_id = stake_and_assert::<Test>(DAVE, PICA::ID, PICA::units(10), ONE_MINUTE);

		process_and_progress_blocks::<Pallet<Test>, Test>(10);

		// bob claims their whole amount, which will be some portion of the unlocked rewards
		Test::assert_extrinsic_event(
			Pallet::<Test>::claim(RuntimeOrigin::signed(BOB), fnft_collection_id, bob_stake_id),
			crate::Event::<Test>::Claimed {
				owner: BOB,
				fnft_collection_id,
				fnft_instance_id: bob_stake_id,
				// after first 10 blocks Bob gets 10*6*10_000 USDT of rewards.
				// after Dave stakes, there are 600_000 more rewards accumulated after next 10
				// blocks These rewards are split equally because they have the same shares amount
				claimed_amounts: [(USDT::ID, 900_000)].into_iter().collect(),
			},
		);

		Test::assert_extrinsic_event(
			Pallet::<Test>::claim(RuntimeOrigin::signed(DAVE), fnft_collection_id, dave_stake_id),
			crate::Event::<Test>::Claimed {
				owner: DAVE,
				fnft_collection_id,
				fnft_instance_id: dave_stake_id,
				claimed_amounts: [(USDT::ID, 300_000)].into_iter().collect(),
			},
		);

		let pool_account = Pallet::<Test>::pool_account_id(&PICA::ID);

		assert_eq!(Tokens::free_balance(USDT::ID, &pool_account), 0)
	})
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
		|_pool_id,
		 _unlock_penalty,
		 _stake_duration,
		 _staked_asset_id,
		 fnft_collection_id,
		 fnft_instance_id| {
			assert_noop!(
				StakingRewards::extend(
					RuntimeOrigin::signed(non_owner),
					fnft_collection_id,
					fnft_instance_id,
					1_000
				),
				crate::Error::<Test>::OnlyStakeOwnerCanInteractWithStake
			);
		},
	)
}

#[test]
fn unstake_should_not_allow_non_owner() {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);

		create_rewards_pool_and_assert::<Test>(RewardRateBasedIncentive {
			owner: ALICE,
			asset_id: PICA::ID,
			start_block: 2,
			reward_configs: default_reward_config(),
			lock: default_lock_config(),
			minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
		});

		process_and_progress_blocks::<StakingRewards, Test>(1);

		// far more than is necessary
		mint_assets([CHARLIE], [USDT::ID], USDT::units(100_000_000));
		add_to_rewards_pot_and_assert::<Test>(
			CHARLIE,
			PICA::ID,
			USDT::ID,
			USDT::units(100_000_000),
			// should NOT emit the `resume` event due to progressing to the start block (block 2)
			// above
			false,
		);

		process_and_progress_blocks::<StakingRewards, Test>(1);

		mint_assets([BOB], [PICA::ID], PICA::units(200));
		let fnft_collection_id =
			RewardPools::<Test>::get(PICA::ID).expect("Pool exists").financial_nft_asset_id;
		let fnft_instance_id = stake_and_assert::<Test>(BOB, PICA::ID, 100_500, ONE_HOUR);

		assert_noop!(
			StakingRewards::unstake(
				RuntimeOrigin::signed(DAVE),
				fnft_collection_id,
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
		|_pool_id,
		 _unlock_penalty,
		 _stake_duration,
		 _staked_asset_id,
		 fnft_collection_id,
		 fnft_intance_id| {
			assert_noop!(
				StakingRewards::unstake(
					RuntimeOrigin::signed(non_owner),
					fnft_collection_id,
					fnft_intance_id
				),
				crate::Error::<Test>::OnlyStakeOwnerCanInteractWithStake
			);
		},
	)
}

#[test]
fn unstake_should_work() {
	new_test_ext().execute_with(|| {
		next_block::<crate::Pallet<Test>, Test>();

		create_rewards_pool_and_assert::<Test>(RewardRateBasedIncentive {
			owner: ALICE,
			asset_id: PICA::ID,
			start_block: 2,
			reward_configs: [(
				USDT::ID,
				RewardConfig { reward_rate: RewardRate::per_second(USDT::units(1)) },
			)]
			.into_iter()
			.try_collect()
			.unwrap(),
			lock: default_lock_config(),
			minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
		});

		// far more than is necessary
		mint_assets([CHARLIE], [USDT::ID], USDT::units(100_000_000));
		add_to_rewards_pot_and_assert::<Test>(
			CHARLIE,
			PICA::ID,
			USDT::ID,
			USDT::units(100_000_000),
			false,
		);

		next_block::<crate::Pallet<Test>, Test>();

		mint_assets([BOB], [PICA::ID], PICA::units(200));
		let fnft_collection_id =
			RewardPools::<Test>::get(PICA::ID).expect("Pool exists").financial_nft_asset_id;
		let fnft_instance_id = stake_and_assert::<Test>(BOB, PICA::ID, PICA::units(100), ONE_HOUR);

		// 100 blocks * 6 seconds per block < 1 hour
		process_and_progress_blocks::<crate::Pallet<Test>, Test>(100);

		unstake_and_assert::<Test>(BOB, fnft_collection_id, fnft_instance_id, true);
	})
}
mod claim {
	use crate::test::prelude::init_logger;

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
			|_pool_id,
			 _unlock_penalty,
			 _stake_duration,
			 _staked_asset_id,
			 fnft_collection_id,
			 fnft_instance_id| {
				assert_noop!(
					StakingRewards::claim(
						RuntimeOrigin::signed(non_owner),
						fnft_collection_id,
						fnft_instance_id
					),
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
			|pool_id,
			 _unlock_penalty,
			 _stake_duration,
			 staked_asset_id,
			 fnft_collection_id,
			 fnft_instance_id| {
				let rewards_pool = StakingRewards::pools(pool_id).expect("rewards_pool expected");

				// Ensure that the value of the staked asset has **not** changed
				assert_eq!(balance(staked_asset_id, &staker), amount);
				process_and_progress_blocks::<StakingRewards, Test>(1);
				assert_ok!(StakingRewards::claim(
					RuntimeOrigin::signed(staker),
					fnft_collection_id,
					fnft_instance_id
				));
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
			|pool_id,
			 _unlock_penalty,
			 _stake_duration,
			 staked_asset_id,
			 fnft_collection_id,
			 fnft_instance_id| {
				let rewards_pool = StakingRewards::pools(pool_id).expect("rewards_pool expected");

				// First claim
				assert_ok!(StakingRewards::claim(
					RuntimeOrigin::signed(staker),
					fnft_collection_id,
					fnft_instance_id
				));
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
				assert_ok!(StakingRewards::claim(
					RuntimeOrigin::signed(staker),
					fnft_collection_id,
					fnft_instance_id
				));
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
			init_logger();

			process_and_progress_blocks::<StakingRewards, Test>(1);
			assert_ok!(StakingRewards::create_reward_pool(
				RuntimeOrigin::root(),
				get_default_reward_pool()
			));

			let staked_asset_id = PICA::ID;
			let fnft_collection_id = RewardPools::<Test>::get(staked_asset_id)
				.expect("Pool exists")
				.financial_nft_asset_id;

			process_and_progress_blocks::<StakingRewards, Test>(1);

			Test::assert_event(crate::Event::<Test>::RewardPoolStarted { pool_id: PICA::ID });

			// far more than is necessary
			mint_assets([CHARLIE], [USDT::ID], USDT::units(100_000_000));
			add_to_rewards_pot_and_assert::<Test>(
				CHARLIE,
				PICA::ID,
				USDT::ID,
				USDT::units(100_000_000),
				false,
			);

			process_and_progress_blocks::<StakingRewards, Test>(1);

			mint_assets([ALICE], [PICA::ID], PICA::units(100_000_000));

			let fnft_instance_id =
				stake_and_assert::<Test>(ALICE, staked_asset_id, AMOUNT, DURATION);

			assert_eq!(balance(staked_asset_id, &ALICE), PICA::units(100_000_000) - AMOUNT);
			// first staker should have 0 reductions
			assert_eq!(
				Stakes::<Test>::get(fnft_collection_id, fnft_instance_id)
					.expect("expected stake. QED")
					.reductions
					.get(&USDT::ID),
				Some(&0)
			);

			Test::assert_extrinsic_event(
				StakingRewards::claim(
					RuntimeOrigin::signed(ALICE),
					fnft_collection_id,
					fnft_instance_id,
				),
				crate::Event::Claimed {
					owner: ALICE,
					fnft_collection_id,
					fnft_instance_id,
					// 60 because 6000 miliseconds per block, default reward rate is 10 per 1 second
					claimed_amounts: [(USDT::ID, 60)].into_iter().collect(),
				},
			);

			let stake = Stakes::<Test>::get(fnft_collection_id, fnft_instance_id)
				.expect("expected stake. QED");

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
			|pool_id,
			 _unlock_penalty,
			 _stake_duration,
			 _staked_asset_id,
			 fnft_collection_id,
			 fnft_instance_id| {
				assert_ok!(StakingRewards::claim(
					RuntimeOrigin::signed(staker),
					fnft_collection_id,
					fnft_instance_id
				));

				Test::assert_last_event(RuntimeEvent::StakingRewards(crate::Event::Claimed {
					owner: staker,
					fnft_collection_id,
					fnft_instance_id: 0,
					claimed_amounts: [(USDT::ID, 0)].into_iter().collect(),
				}));

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
				RuntimeOrigin::root(),
				RewardRateBasedIncentive {
					owner: ALICE,
					asset_id: PICA::ID,
					start_block: 2,
					reward_configs: default_reward_config(),
					lock: LockConfig {
						duration_multipliers: BoundedBTreeMap::new().into(),
						unlock_penalty: Perbill::from_percent(5),
					},
					minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
				},
			),
			crate::Error::<Test>::NoDurationPresetsProvided
		);
	});
}

mod stake_proptests {
	use super::*;
	use crate::Error;
	use composable_tests_helpers::{prop_assert_noop, prop_assert_ok};

	proptest! {
		#![proptest_config(ProptestConfig::with_cases(10000))]

		#[test]
		fn stake_should_work(
			amount in MINIMUM_STAKING_AMOUNT..(u128::MAX / 10),
		) {
			new_test_ext().execute_with(|| {
				let staker = ALICE;
				let existential_deposit = 1_000_u128;
				let owner = RuntimeOrigin::signed(staker);
				let pool_id = PICA::ID;
				let duration_preset = ONE_HOUR;

				process_and_progress_blocks::<StakingRewards, Test>(1);

				assert_ok!(StakingRewards::create_reward_pool(RuntimeOrigin::root(), get_default_reward_pool()));

				process_and_progress_blocks::<StakingRewards, Test>(1);

				let mint_amount = amount + existential_deposit;
				mint_assets([staker], [PICA::ID], mint_amount);

				prop_assert_ok!(StakingRewards::stake(
					owner,
					pool_id,
					amount,
					duration_preset,
				));

				Ok(())
			})?;
		}

		#[test]
		fn stake_should_not_work_with_low_amounts(
			amount in 0_u128..(MINIMUM_STAKING_AMOUNT - 1)
		) {
			new_test_ext().execute_with(|| {
				process_and_progress_blocks::<StakingRewards, Test>(1);

				assert_ok!(StakingRewards::create_reward_pool(RuntimeOrigin::root(), get_default_reward_pool()));

				let staker = ALICE;
				let existential_deposit = 1_000_u128;

				process_and_progress_blocks::<StakingRewards, Test>(1);

				let owner = RuntimeOrigin::signed(staker);
				let pool_id = PICA::ID;
				let duration_preset = ONE_HOUR;

				let mint_amount = amount * 2 + existential_deposit;
				mint_assets([staker], [PICA::ID], mint_amount);

				prop_assert_noop!(
					StakingRewards::stake(
						owner,
						pool_id,
						amount,
						duration_preset,
					),
					Error::<Test>::StakedAmountTooLow
				);

				Ok(())
			})?;
		}
	}
}

mod split_proptests {
	use super::*;
	use crate::Error;
	use composable_tests_helpers::{prop_assert_noop, prop_assert_ok};

	proptest! {
		#![proptest_config(ProptestConfig::with_cases(10000))]

		#[test]
		fn split_should_work(
			parts in MINIMUM_STAKING_AMOUNT..MINIMUM_STAKING_AMOUNT*99
		) {
			new_test_ext().execute_with(|| {
				let ratio = Permill::from_rational(parts, MINIMUM_STAKING_AMOUNT*100);
				System::set_block_number(1);

				let staker = ALICE;
				let owner = RuntimeOrigin::signed(staker);
				let pool_id = PICA::ID;
				let duration_preset = ONE_HOUR;
				let staking_amount = 100 * MINIMUM_STAKING_AMOUNT;

				assert_ok!(StakingRewards::create_reward_pool(RuntimeOrigin::root(), get_default_reward_pool()));

				process_and_progress_blocks::<StakingRewards, Test>(1);

				mint_assets([staker], [PICA::ID], PICA::units(200));

				let fnft_collection_id = RewardPools::<Test>::get(pool_id)
					.expect("Pool exists")
					.financial_nft_asset_id;
				let original_fnft_instance_id =
					stake_and_assert::<Test>(staker, pool_id, staking_amount, duration_preset);

				prop_assert_ok!(StakingRewards::split(
					owner,
					fnft_collection_id,
					original_fnft_instance_id,
					ratio.try_into_validated().unwrap(),
				));

				Ok(())
			})?;
		}

		#[test]
		fn split_should_not_work_when_low_ratio_results_in_too_low_amount(
			parts in 1_u128..MINIMUM_STAKING_AMOUNT-1
		) {
			new_test_ext().execute_with(|| {
				let ratio = Permill::from_rational(parts, MINIMUM_STAKING_AMOUNT*100);
				System::set_block_number(1);

				let staker = ALICE;
				let owner = RuntimeOrigin::signed(staker);
				let pool_id = PICA::ID;
				let duration_preset = ONE_HOUR;
				let staking_amount = 100 * MINIMUM_STAKING_AMOUNT;

				assert_ok!(StakingRewards::create_reward_pool(RuntimeOrigin::root(), get_default_reward_pool()));

				process_and_progress_blocks::<StakingRewards, Test>(1);

				mint_assets([staker], [PICA::ID], PICA::units(200));

				let fnft_collection_id = RewardPools::<Test>::get(pool_id)
					.expect("Pool exists")
					.financial_nft_asset_id;
				let original_fnft_instance_id =
					stake_and_assert::<Test>(staker, pool_id, staking_amount, duration_preset);

				prop_assert_noop!(
					StakingRewards::split(
						owner,
						fnft_collection_id,
						original_fnft_instance_id,
						ratio.try_into_validated().unwrap(),
					),
					Error::<Test>::StakedAmountTooLowAfterSplit
				);

				Ok(())
			})?;
		}

		#[test]
		fn split_should_not_work_when_high_ratio_results_in_too_low_amount(
			parts in MINIMUM_STAKING_AMOUNT*99+1..MINIMUM_STAKING_AMOUNT*100
		) {
			new_test_ext().execute_with(|| {
				let ratio = Permill::from_rational(parts, MINIMUM_STAKING_AMOUNT*100);
				System::set_block_number(1);

				let staker = ALICE;
				let owner = RuntimeOrigin::signed(staker);
				let pool_id = PICA::ID;
				let duration_preset = ONE_HOUR;
				let staking_amount = 100 * MINIMUM_STAKING_AMOUNT;

				assert_ok!(StakingRewards::create_reward_pool(RuntimeOrigin::root(), get_default_reward_pool()));

				process_and_progress_blocks::<StakingRewards, Test>(1);

				mint_assets([staker], [PICA::ID], PICA::units(200));

				let fnft_collection_id = RewardPools::<Test>::get(pool_id)
					.expect("Pool exists")
					.financial_nft_asset_id;
				let original_fnft_instance_id =
					stake_and_assert::<Test>(staker, pool_id, staking_amount, duration_preset);

				prop_assert_noop!(
					StakingRewards::split(
						owner,
						fnft_collection_id,
						original_fnft_instance_id,
						ratio.try_into_validated().unwrap(),
					),
					Error::<Test>::StakedAmountTooLowAfterSplit
				);

				Ok(())
			})?;
		}
	}
}

mod extend_proptests {
	use composable_tests_helpers::prop_assert_ok;

	use super::*;

	proptest! {
		#![proptest_config(ProptestConfig::with_cases(10000))]

		#[test]
		fn extend_should_work(
			amount in 0_u128..PICA::units(1_000_000),
		) {
			new_test_ext().execute_with(|| {
				System::set_block_number(1);

				let staker = ALICE;
				let owner = RuntimeOrigin::signed(staker);
				let pool_id = PICA::ID;
				let duration_preset = ONE_HOUR;
				let staking_amount = MINIMUM_STAKING_AMOUNT;

				assert_ok!(StakingRewards::create_reward_pool(RuntimeOrigin::root(), get_default_reward_pool()));

				process_and_progress_blocks::<StakingRewards, Test>(1);

				mint_assets([staker], [PICA::ID], PICA::units(1_000_000));

				let fnft_collection_id = RewardPools::<Test>::get(pool_id)
					.expect("Pool exists")
					.financial_nft_asset_id;
				let original_fnft_instance_id =
					stake_and_assert::<Test>(staker, pool_id, staking_amount, duration_preset);

				prop_assert_ok!(StakingRewards::extend(
					owner,
					fnft_collection_id,
					original_fnft_instance_id,
					amount,
				));

				Ok(())
			})?;
		}
	}
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
	execute: impl FnOnce(u128, Perbill, u64, u128, u128, u64) -> R,
) -> R {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);
		assert_ok!(StakingRewards::create_reward_pool(
			RuntimeOrigin::root(),
			get_default_reward_pool()
		));

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
		let fnft_collection_id = RewardPools::<Test>::get(staked_asset_id)
			.expect("Pool exists")
			.financial_nft_asset_id;
		let fnft_instance_id = stake_and_assert::<Test>(staker, PICA::ID, amount, duration);

		// assert_ok!(StakingRewards::stake(RuntimeOrigin::signed(staker), pool_id, amount,
		// duration));
		assert_eq!(balance(staked_asset_id, &staker), amount);

		let stake = StakingRewards::stakes(fnft_collection_id, 0).expect("stake expected. QED");
		let unlock_penalty = stake.lock.unlock_penalty;
		let stake_duration = stake.lock.duration;

		if should_claim {
			// update_reductions(&mut stake.reductions, claim);
			assert_ok!(StakingRewards::claim(
				RuntimeOrigin::signed(staker),
				fnft_collection_id,
				fnft_instance_id
			));
		}

		execute(
			staked_asset_id,
			unlock_penalty,
			stake_duration,
			staked_asset_id,
			fnft_collection_id,
			fnft_instance_id,
		)
	})
}

fn create_default_reward_pool() {
	Test::assert_extrinsic_event(
		StakingRewards::create_reward_pool(
			RuntimeOrigin::root(),
			RewardRateBasedIncentive {
				owner: ALICE,
				asset_id: PICA::ID,
				start_block: 2,
				reward_configs: default_reward_config(),
				lock: default_lock_config(),
				minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
			},
		),
		crate::Event::<Test>::RewardPoolCreated { pool_id: PICA::ID, owner: ALICE },
	);
}

/// Creates a PICA staking reward pool. Calls [`default_reward_pool`] and [`default_lock_config`].
fn get_default_reward_pool() -> RewardPoolConfigurationOf<Test> {
	RewardRateBasedIncentive {
		owner: ALICE,
		asset_id: PICA::ID,
		start_block: 2,
		reward_configs: default_reward_config(),
		lock: default_lock_config(),
		minimum_staking_amount: MINIMUM_STAKING_AMOUNT,
	}
}

fn default_lock_config() -> LockConfig<MaxStakingDurationPresets> {
	LockConfig {
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
	}
}

fn default_reward_config() -> BoundedBTreeMap<u128, RewardConfig<u128>, MaxRewardConfigsPerPool> {
	bounded_btree_map! {
		USDT::ID => RewardConfig { reward_rate: RewardRate::per_second(10_u128) }
	}
}

fn mint_assets(
	accounts: impl IntoIterator<Item = Public>,
	asset_ids: impl IntoIterator<Item = u128>,
	amount: u128,
) {
	let asset_ids = Vec::from_iter(asset_ids);
	for account in accounts {
		for asset_id in &asset_ids {
			<<Test as crate::Config>::AssetsTransactor as Mutate<
				<Test as frame_system::Config>::AccountId,
			>>::mint_into(*asset_id, &account, amount)
			.expect("an asset minting expected");
		}
	}
}

fn balance(asset_id: u128, account: &Public) -> u128 {
	<<Test as crate::Config>::AssetsTransactor as Inspect<
		<Test as frame_system::Config>::AccountId,
	>>::balance(asset_id, account)
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

#[test]
fn zero_penalty_early_unlock() {
	new_test_ext().execute_with(|| {
		next_block::<StakingRewards, Test>();

		create_rewards_pool_and_assert::<Test>(RewardRateBasedIncentive {
			owner: ALICE,
			asset_id: PICA::ID,
			start_block: 3,
			reward_configs: bounded_btree_map! {
				BTC::ID => RewardConfig { reward_rate: RewardRate::per_second(0_u128) }
			},
			lock: LockConfig {
				duration_multipliers: bounded_btree_map! {
					// 0 => FixedU64::one().try_into_validated().expect("1 >= 1")
					ONE_HOUR => FixedU64::one().try_into_validated().expect("1 >= 1")
				}
				.into(),
				unlock_penalty: Perbill::zero(),
			},
			minimum_staking_amount: 100_000,
		});

		mint_assets([ALICE], [BTC::ID], BTC::units(100));
		add_to_rewards_pot_and_assert::<Test>(ALICE, PICA::ID, BTC::ID, BTC::units(100), false);

		process_and_progress_blocks::<StakingRewards, Test>(2);

		mint_assets([BOB], [PICA::ID], PICA::units(10));
		let fnft_collection_id =
			RewardPools::<Test>::get(PICA::ID).expect("Pool exists").financial_nft_asset_id;
		let stake_id = stake_and_assert::<Test>(BOB, PICA::ID, PICA::units(1), ONE_HOUR);

		next_block::<StakingRewards, Test>();

		unstake_and_assert::<Test>(BOB, fnft_collection_id, stake_id, true);
	})
}

#[test]
fn pbl_295() {
	new_test_ext().execute_with(|| {
		init_logger();

		next_block::<StakingRewards, Test>();

		// === Utility functions
		fn pot_balance_available() -> Balance {
			let pot_account = crate::Pallet::<Test>::pool_account_id(&PICA::ID);

			let balance_on_hold =
				<Test as crate::Config>::AssetsTransactor::balance_on_hold(USDT::ID, &pot_account);
			let balance =
				<Test as crate::Config>::AssetsTransactor::balance(USDT::ID, &pot_account);
			let available = balance - balance_on_hold;
			println!(
				"Pot balance: {} (available: {}, on hold: {})",
				balance, available, balance_on_hold
			);
			available
		}

		fn claimable_amount(fnft_id: u64) -> Balance {
			let pool = RewardPools::<Test>::get(PICA::ID).unwrap();
			let fnft_collection_id =
				RewardPools::<Test>::get(PICA::ID).expect("Pool exists").financial_nft_asset_id;

			let amount = claim_of_stake::<Test>(
				&Stakes::<Test>::get(fnft_collection_id, fnft_id).unwrap(),
				&pool.share_asset_id,
				&pool.rewards[&USDT::ID],
				&USDT::ID,
			)
			.unwrap();
			println!("Claimable amount: {}", amount);
			amount
		}
		let reward_rate = USDT::units(1) / 1_000;
		let rewards_for_blocks = |blocks: u64| -> Balance { reward_rate * block_seconds(blocks) };

		// === 1. Create rewards pool
		create_rewards_pool_and_assert::<Test>(RewardRateBasedIncentive {
			owner: ALICE,
			asset_id: PICA::ID,
			start_block: 5,
			reward_configs: bounded_btree_map! {
				USDT::ID => RewardConfig { reward_rate: RewardRate::per_second(reward_rate) }
			},
			lock: LockConfig {
				duration_multipliers: bounded_btree_map! {
					0 => FixedU64::from_inner(1_000_000_000).try_into_validated().expect(">= 1"),
					12 => FixedU64::from_inner(1_250_000_000).try_into_validated().expect(">= 1"),
					600 => FixedU64::from_inner(1_500_000_000).try_into_validated().expect(">= 1"),
					1200 => FixedU64::from_inner(2_000_000_000).try_into_validated().expect(">= 1"),
				}
				.into(),
				unlock_penalty: Perbill::from_percent(10),
			},
			minimum_staking_amount: 10_000,
		});
		let fnft_collection_id =
			RewardPools::<Test>::get(PICA::ID).expect("Pool exists").financial_nft_asset_id;

		// === 2. Add funds (USD) to rewards pot
		mint_assets([BOB], [USDT::ID], USDT::units(100_000_000_001));
		add_to_rewards_pot_and_assert::<Test>(BOB, PICA::ID, USDT::ID, USDT::units(1), false);
		assert_eq!(pot_balance_available(), USDT::units(0));
		process_and_progress_blocks::<StakingRewards, Test>(4);
		Test::assert_event(crate::Event::<Test>::RewardPoolStarted { pool_id: PICA::ID });
		process_and_progress_blocks::<StakingRewards, Test>(10);
		assert_eq!(pot_balance_available(), rewards_for_blocks(10));

		// === 3. Stake by Dave 1000 PICA
		mint_assets([DAVE], [PICA::ID], PICA::units(1_001));
		let dave_id = stake_and_assert::<Test>(DAVE, PICA::ID, PICA::units(1) / 100_000, 0);
		process_and_progress_blocks::<StakingRewards, Test>(2);

		// === 4. Stake by Charlie 1000 PICA
		mint_assets([CHARLIE], [PICA::ID], PICA::units(1_001));
		let charlie_id = stake_and_assert::<Test>(CHARLIE, PICA::ID, PICA::units(1) / 100_000, 0);

		// === 5. Claim by Dave
		assert_eq!(pot_balance_available(), rewards_for_blocks(12));
		assert_eq!(claimable_amount(dave_id), rewards_for_blocks(12));
		assert_eq!(claimable_amount(charlie_id), 0);
		StakingRewards::claim(RuntimeOrigin::signed(DAVE), fnft_collection_id, dave_id).unwrap();
		assert_eq!(pot_balance_available(), 0);
		assert_eq!(claimable_amount(dave_id), 0);
		assert_eq!(claimable_amount(charlie_id), 0);

		process_and_progress_blocks::<StakingRewards, Test>(2);

		// === 6. Claim by Charlie (can claim half the rewards in the pool as per shares)
		assert_eq!(pot_balance_available(), rewards_for_blocks(2));
		assert_eq!(claimable_amount(dave_id), rewards_for_blocks(2) / 2);
		assert_eq!(claimable_amount(charlie_id), rewards_for_blocks(2) / 2);
		StakingRewards::claim(RuntimeOrigin::signed(CHARLIE), fnft_collection_id, charlie_id)
			.unwrap();
		assert_eq!(pot_balance_available(), rewards_for_blocks(2) / 2);
		assert_eq!(claimable_amount(dave_id), rewards_for_blocks(2) / 2);
		assert_eq!(claimable_amount(charlie_id), 0);

		process_and_progress_blocks::<StakingRewards, Test>(2);

		// === 7. Split by Dave 50/50
		assert_eq!(pot_balance_available(), rewards_for_blocks(2) + rewards_for_blocks(2) / 2);
		assert_eq!(claimable_amount(dave_id), rewards_for_blocks(2));
		assert_eq!(claimable_amount(charlie_id), rewards_for_blocks(2) / 2);
		let dave_new = split_and_assert::<Test>(
			DAVE,
			fnft_collection_id,
			dave_id,
			Permill::from_percent(50).try_into_validated().unwrap(),
		);
		assert_eq!(pot_balance_available(), rewards_for_blocks(2) + rewards_for_blocks(2) / 2);
		assert_eq!(claimable_amount(dave_id), rewards_for_blocks(2) / 2);
		assert_eq!(claimable_amount(dave_new), rewards_for_blocks(2) / 2);
		assert_eq!(claimable_amount(charlie_id), rewards_for_blocks(2) / 2);

		process_and_progress_blocks::<StakingRewards, Test>(2);

		// === 8. Unstake by Dave of first stake
		assert_eq!(
			pot_balance_available(),
			rewards_for_blocks(2) + rewards_for_blocks(2) / 2 + rewards_for_blocks(2)
		);
		assert_eq!(
			claimable_amount(dave_id),
			rewards_for_blocks(2) / 2 + rewards_for_blocks(2) / 4
		);
		assert_eq!(
			claimable_amount(dave_new),
			rewards_for_blocks(2) / 2 + rewards_for_blocks(2) / 4
		);
		assert_eq!(
			claimable_amount(charlie_id),
			rewards_for_blocks(2) / 2 + rewards_for_blocks(2) / 2
		);
		unstake_and_assert::<Test>(DAVE, fnft_collection_id, dave_id, false);
		assert_eq!(
			pot_balance_available(),
			rewards_for_blocks(2) + rewards_for_blocks(2) - rewards_for_blocks(2) / 4
		);
		assert_eq!(
			claimable_amount(dave_new),
			rewards_for_blocks(2) / 2 + rewards_for_blocks(2) / 4
		);
		assert_eq!(
			claimable_amount(charlie_id),
			rewards_for_blocks(2) / 2 + rewards_for_blocks(2) / 2
		);

		// Dave can still claim (bugfixed)
		assert_ok!(StakingRewards::claim(
			RuntimeOrigin::signed(DAVE),
			fnft_collection_id,
			dave_new
		));
		assert_eq!(claimable_amount(dave_new), 0);
		assert_eq!(
			claimable_amount(charlie_id),
			rewards_for_blocks(2) / 2 + rewards_for_blocks(2) / 2
		);

		// Charlie can still claim (bugfixed)
		assert_ok!(StakingRewards::claim(
			RuntimeOrigin::signed(CHARLIE),
			fnft_collection_id,
			charlie_id
		));
		assert_eq!(claimable_amount(dave_new), 0);
		assert_eq!(claimable_amount(charlie_id), 0);
	})
}

#[test]
fn zero_penalty_no_multiplier_doesnt_slash() {
	new_test_ext().execute_with(|| {
		next_block::<StakingRewards, Test>();

		create_rewards_pool_and_assert::<Test>(RewardRateBasedIncentive {
			owner: ALICE,
			asset_id: PICA::ID,
			start_block: 3,
			reward_configs: bounded_btree_map! {
				BTC::ID => RewardConfig { reward_rate: RewardRate::per_second(0_u128) }
			},
			lock: LockConfig {
				duration_multipliers: bounded_btree_map! {
					0 => FixedU64::one().try_into_validated().expect("1 >= 1")
				}
				.into(),
				unlock_penalty: Perbill::zero(),
			},
			minimum_staking_amount: 100_000,
		});

		mint_assets([ALICE], [BTC::ID], BTC::units(100));
		add_to_rewards_pot_and_assert::<Test>(ALICE, PICA::ID, BTC::ID, BTC::units(100), false);

		process_and_progress_blocks::<StakingRewards, Test>(2);

		mint_assets([BOB], [PICA::ID], PICA::units(10));
		let fnft_collection_id =
			RewardPools::<Test>::get(PICA::ID).expect("Pool exists").financial_nft_asset_id;
		let stake_id = stake_and_assert::<Test>(BOB, PICA::ID, PICA::units(1), 0);

		next_block::<StakingRewards, Test>();

		unstake_and_assert::<Test>(
			BOB,
			fnft_collection_id,
			stake_id,
			false, // shouldn't be an early unlock since the lock period is 0
		);
	})
}
