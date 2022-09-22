pub(crate) use crate::test::runtime::{new_test_ext, Test}; // for benchmarks
use crate::{
	test::{prelude::H256, runtime::*},
	Config, RewardPoolConfigurationOf, RewardPools, StakeOf, Stakes,
};
use composable_tests_helpers::test::{
	block::process_and_progress_blocks,
	currency::{BTC, PICA, USDT, XPICA},
	helper::{assert_extrinsic_event, assert_extrinsic_event_with, assert_last_event_with},
};
use composable_traits::{
	fnft::FinancialNft as FinancialNftT,
	staking::{
		lock::{Lock, LockConfig},
		ProtocolStaking, RewardConfig,
		RewardPoolConfiguration::RewardRateBasedIncentive,
		RewardRate, Stake, Staking,
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
use sp_arithmetic::{Perbill, Permill};
use sp_core::sr25519::Public;
use sp_runtime::PerThing;
use sp_std::collections::{btree_map::BTreeMap, btree_set::BTreeSet};

use self::prelude::STAKING_FNFT_COLLECTION_ID;

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
				}
			),
			crate::Error::<Test>::EndBlockMustBeAfterStartBlock
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

		let asset_id = StakingRewards::pools(PICA::ID).expect("asset_id expected").asset_id;
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
fn stake_in_case_of_zero_inflation_should_work() {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);

		assert_ok!(StakingRewards::create_reward_pool(Origin::root(), get_default_reward_pool()));
		let staker: Public = ALICE;
		let amount: u128 = 100_500_u128;
		let duration_preset: u64 = ONE_HOUR;
		let fnft_asset_account = FinancialNft::asset_account(&1, &0);

		let staked_asset_id = StakingRewards::pools(PICA::ID).expect("asset_id expected").asset_id;
		mint_assets([staker], [staked_asset_id], amount * 2);

		process_and_progress_blocks::<StakingRewards, Test>(1);
		let fnft_instance_id = assert_extrinsic_event_with::<Test, _, _, _>(
			StakingRewards::stake(Origin::signed(staker), PICA::ID, amount, duration_preset),
			|event| match event {
				Event::StakingRewards(crate::Event::<Test>::Staked {
					pool_id: PICA::ID,
					owner: _staker,
					amount: _,
					duration_preset: _,
					fnft_collection_id: _,
					fnft_instance_id,
					keep_alive: _,
				}) => Some(fnft_instance_id),
				_ => None,
			},
		);
		assert_eq!(Stakes::<Test>::iter_prefix_values(PICA::ID).collect::<Vec<_>>().len(), 1);

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
				share: StakingRewards::boosted_amount(reward_multiplier, amount),
				reductions,
				lock: Lock {
					started_at: 12,
					duration: duration_preset,
					unlock_penalty: rewards_pool.lock.unlock_penalty,
				},
				fnft_instance_id,
			})
		);

		assert_eq!(balance(staked_asset_id, &staker), amount);
		assert_eq!(balance(staked_asset_id, &fnft_asset_account), amount);

		assert_last_event_with::<Test, _>(|event| {
			matches!(
				event,
				Event::StakingRewards(crate::Event::Staked {
					pool_id: PICA::ID,
					owner,
					amount: _,
					duration_preset: _,
					fnft_collection_id: PICA::ID,
					fnft_instance_id: _,
					keep_alive: _,
				}) if owner == staker
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
		const TOTAL_REWARDS: u128 = 100;
		const TOTAL_SHARES: u128 = 200;
		let fnft_asset_account = FinancialNft::asset_account(&1, &0);

		process_and_progress_blocks::<StakingRewards, Test>(1);
		assert_ok!(StakingRewards::create_reward_pool(Origin::root(), get_default_reward_pool()));

		let staked_asset_id = StakingRewards::pools(PICA::ID).expect("asset_id expected").asset_id;
		mint_assets([ALICE], [staked_asset_id], AMOUNT * 2);
		update_total_rewards_and_total_shares_in_rewards_pool(
			PICA::ID,
			TOTAL_REWARDS,
			TOTAL_SHARES,
		);

		process_and_progress_blocks::<StakingRewards, Test>(1);
		assert_ok!(StakingRewards::stake(Origin::signed(ALICE), PICA::ID, AMOUNT, DURATION_PRESET));
		let rewards_pool = StakingRewards::pools(PICA::ID).expect("rewards_pool expected");
		let reward_multiplier = StakingRewards::reward_multiplier(&rewards_pool, DURATION_PRESET)
			.expect("reward_multiplier expected");
		let inflation = StakingRewards::boosted_amount(reward_multiplier, AMOUNT) * TOTAL_REWARDS /
			TOTAL_SHARES;
		assert_eq!(inflation, 502);
		let reductions = rewards_pool
			.rewards
			.into_inner()
			.iter()
			.map(|(asset_id, _reward)| (*asset_id, inflation))
			.try_collect()
			.expect("reductions expected");

		assert_eq!(
			StakingRewards::stakes(1, 0),
			Some(Stake {
				reward_pool_id: PICA::ID,
				stake: AMOUNT,
				share: StakingRewards::boosted_amount(reward_multiplier, AMOUNT),
				reductions,
				lock: Lock {
					started_at: 12,
					duration: DURATION_PRESET,
					unlock_penalty: rewards_pool.lock.unlock_penalty,
				},
				fnft_instance_id: 0,
			})
		);

		assert_eq!(balance(staked_asset_id, &ALICE), AMOUNT);
		assert_eq!(balance(staked_asset_id, &fnft_asset_account), AMOUNT);

		assert!(FinancialNft::instance(1, 0).is_some());

		assert_last_event_with::<Test, _>(|event| match event {
			Event::StakingRewards(crate::Event::Staked {
				pool_id,
				owner,
				amount,
				duration_preset: _,
				fnft_collection_id: _,
				fnft_instance_id: _,
				keep_alive: _,
			}) => {
				assert_eq!(owner, ALICE);
				assert_eq!(pool_id, PICA::ID);
				assert_eq!(amount, 100_500_u32.into());

				Some(())
			},
			_ => None,
		});
	});
}

#[test]
fn test_extend_stake_amount() {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);

		assert_ok!(StakingRewards::create_reward_pool(Origin::root(), get_default_reward_pool()));
		let staker = ALICE;
		let pool_id = PICA::ID;
		let amount = 100_500_u32.into();
		let extend_amount = 100_500_u32.into();
		let existential_deposit = 1_000_u128;
		let duration_preset = ONE_HOUR;
		let total_rewards = 100;
		let total_shares = 200;

		let staked_asset_id = StakingRewards::pools(PICA::ID).expect("asset_id expected").asset_id;
		mint_assets([staker], [staked_asset_id], amount * 2 + existential_deposit);
		update_total_rewards_and_total_shares_in_rewards_pool(pool_id, total_rewards, total_shares);

		process_and_progress_blocks::<StakingRewards, Test>(1);
		assert_ok!(StakingRewards::stake(Origin::signed(staker), pool_id, amount, duration_preset));

		let rewards_pool = StakingRewards::pools(pool_id).expect("rewards_pool expected");
		let reward_multiplier = StakingRewards::reward_multiplier(&rewards_pool, duration_preset)
			.expect("reward_multiplier expected");
		let boosted_amount = StakingRewards::boosted_amount(reward_multiplier, amount);
		let inflation = boosted_amount * total_rewards / total_shares;

		assert_ok!(StakingRewards::extend(Origin::signed(staker), 1, 0, extend_amount));

		let rewards_pool = StakingRewards::pools(pool_id).expect("rewards_pool expected");

		let total_rewards =	rewards_pool
			.rewards
			.iter()
			.fold(0, |total_rewards, (_asset_id, reward)| {
				total_rewards + reward.total_rewards
			});

		let inflation_extended = extend_amount * total_rewards / rewards_pool.total_shares;
		let inflation = inflation + inflation_extended;
		assert_eq!(inflation, 50710);

		let reductions = rewards_pool
			.rewards
			.iter()
			.map(|(asset_id, _reward)| (*asset_id, inflation))
			.try_collect()
			.expect("reductions expected");

		let stake = StakingRewards::stakes(1, 0);
		assert_eq!(
			stake,
			Some(Stake {
				fnft_instance_id: 0,
				reward_pool_id: pool_id,
				stake: amount + extend_amount,
				share: boosted_amount + extend_amount,
				reductions,
				lock: Lock {
					started_at: 12,
					duration: duration_preset,
					unlock_penalty: rewards_pool.lock.unlock_penalty,
				},
			})
		);
		assert_eq!(balance(staked_asset_id, &staker), existential_deposit);
		assert_eq!(
			balance(staked_asset_id, &FinancialNft::asset_account(&1, &0)),
			amount + extend_amount
		);
		assert_last_event::<Test, _>(|e| {
			matches!(e.event,
            Event::StakingRewards(crate::Event::StakeAmountExtended { fnft_collection_id, fnft_instance_id, amount})
            if fnft_collection_id == 1_u128 && fnft_instance_id == 0_u64 && amount == extend_amount)
		});
	});
}

#[test]
fn unstake_non_existent_stake_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let staker = ALICE;
		assert_noop!(
			StakingRewards::unstake(Origin::signed(staker), 1, 0),
			crate::Error::<Test>::StakeNotFound
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

		let staked_asset_id = StakingRewards::pools(PICA::ID).expect("asset_id expected").asset_id;
		mint_assets([owner, not_owner], [staked_asset_id], amount * 2);

		process_and_progress_blocks::<StakingRewards, Test>(1);
		assert_ok!(StakingRewards::stake(Origin::signed(owner), pool_id, amount, duration_preset));

		assert_noop!(
			StakingRewards::unstake(Origin::signed(not_owner), 1, 0),
			crate::Error::<Test>::OnlyStakeOwnerCanUnstake
		);
	});
}

#[test]
fn unstake_in_case_of_zero_claims_and_early_unlock_should_work() {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);

		assert_ok!(StakingRewards::create_reward_pool(Origin::root(), get_default_reward_pool()));
		let staker = ALICE;
		let pool_id = PICA::ID;
		let amount = 100_500_u32.into();
		let duration_preset = ONE_HOUR;
		let fnft_asset_account = FinancialNft::asset_account(&1, &0);

		let staked_asset_id = StakingRewards::pools(PICA::ID).expect("asset_id expected").asset_id;
		mint_assets([staker], [staked_asset_id], amount * 2);

		process_and_progress_blocks::<StakingRewards, Test>(1);
		assert_ok!(StakingRewards::stake(Origin::signed(staker), pool_id, amount, duration_preset));
		let unlock_penalty =
			StakingRewards::stakes(1, 0).expect("stake expected").lock.unlock_penalty;
		assert_eq!(balance(staked_asset_id, &staker), amount);

		assert_ok!(StakingRewards::unstake(Origin::signed(staker), 1, 0));
		assert_eq!(StakingRewards::stakes(1, 0), None);
		assert_last_event::<Test, _>(|e| {
			matches!(
				e.event,
				Event::StakingRewards(crate::Event::Unstaked { owner, fnft_collection_id, fnft_instance_id })
					if owner == staker && fnft_collection_id == 1 && fnft_instance_id == 0
			)
		});

		let penalty = unlock_penalty.mul_ceil(amount);
		assert_eq!(balance(staked_asset_id, &staker), amount + (amount - penalty));
		// NOTE(connor): Used to keep penalty in the pool account, fNFT design has assets being
		// burned instead
		assert_eq!(balance(staked_asset_id, &fnft_asset_account), 0);
		// assert_eq!(balance(staked_asset_id, &fnft_asset_account), penalty);

		// Assert fNFT is removed from storage
		assert!(FinancialNft::instance(1, 0).is_none());
	});
}

#[test]
fn unstake_in_case_of_not_zero_claims_and_early_unlock_should_work() {
	let staker = ALICE;
	let amount = 100_500;
	let duration = ONE_HOUR;
	let total_rewards = 100;
	let total_shares = 200;
	let claim = 50;

	with_stake(
		staker,
		amount,
		duration,
		total_rewards,
		total_shares,
		Some(claim),
		|pool_id, unlock_penalty, _, staked_asset_id| {
			let fnft_asset_account = FinancialNft::asset_account(&1, &0);

			assert_ok!(StakingRewards::unstake(Origin::signed(staker), 1, 0));
			assert_eq!(StakingRewards::stakes(1, 0), None);
			assert_last_event::<Test, _>(|e| {
				matches!(
					e.event,
					Event::StakingRewards(crate::Event::Unstaked { owner, fnft_collection_id, fnft_instance_id })
						if owner == staker && fnft_collection_id == 1 && fnft_instance_id == 0
				)
			});

			let penalty = unlock_penalty.mul_ceil(amount);
			let claim_with_penalty = (Perbill::one() - unlock_penalty).mul_ceil(claim);
			let rewards_pool = StakingRewards::pools(pool_id).expect("rewards_pool expected");
			assert_eq!(balance(staked_asset_id, &staker), amount * 2 - penalty);
			// NOTE(connor): Used to keep penalty in the pool account, fNFT design has assets being
			// burned instead
			assert_eq!(balance(staked_asset_id, &fnft_asset_account), 0);
			// assert_eq!(balance(staked_asset_id, &fnft_asset_account), amount * 2 + penalty);
			for (rewarded_asset_id, _) in rewards_pool.rewards.iter() {
				assert_eq!(balance(*rewarded_asset_id, &staker), amount * 2 + claim_with_penalty);
				assert_eq!(
					balance(*rewarded_asset_id, &StakingRewards::pool_account_id(&pool_id)),
					amount * 2 - claim_with_penalty
				);
			}
		},
	);
}

#[test]
fn unstake_in_case_of_not_zero_claims_and_not_early_unlock_should_work() {
	let staker = ALICE;
	let amount = 100_500_u32.into();
	let duration_preset = ONE_HOUR;
	let total_rewards = 100;
	let total_shares = 200;
	let claim = 50;

	with_stake(
		staker,
		amount,
		duration_preset,
		total_rewards,
		total_shares,
		Some(claim),
		|pool_id, _, stake_duration, staked_asset_id| {
			let second_in_milliseconds = 1000;
			Timestamp::set_timestamp(
				Timestamp::now() + stake_duration * second_in_milliseconds + second_in_milliseconds,
			);
			assert_ok!(StakingRewards::unstake(Origin::signed(staker), 1, 0));
			assert_eq!(StakingRewards::stakes(1, 0), None);
			assert_last_event::<Test, _>(|e| {
				matches!(
					e.event,
					Event::StakingRewards(crate::Event::Unstaked { owner, fnft_collection_id, fnft_instance_id })
						if owner == staker && fnft_collection_id == 1 && fnft_instance_id == 0
				)
			});

			let rewards_pool = StakingRewards::pools(pool_id).expect("rewards_pool expected");
			assert_eq!(balance(staked_asset_id, &staker), amount * 2);
			assert_eq!(
				balance(staked_asset_id, &StakingRewards::pool_account_id(&pool_id)),
				amount * 2
			);
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
			10_u128
		));
		// can't transfer more than max_rewards set in the rewards config
		assert_noop!(
			<StakingRewards as ProtocolStaking>::transfer_reward(&ALICE, &1, USDT::ID, 10_000_u128),
			crate::Error::<Test>::MaxRewardLimitReached
		);
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
			10_000_u128
		));
	});
}

// NOTE(connor): Ignoring because the test fails to correctly configure the reward pool
#[ignore]
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
		};

		assert_extrinsic_event::<Test, _, _, _>(
			StakingRewards::create_reward_pool(Origin::root(), pool_init_config),
			crate::Event::<Test>::RewardPoolCreated {
				pool_id: PICA::ID,
				owner: ALICE,
				end_block: 5,
			},
		);

		let reduction = PICA::units(10);
		let stake = StakeOf::<Test> {
			reward_pool_id: PICA::ID,
			stake: PICA::units(1_000),
			share: PICA::units(1_000),
			reductions: [(USDT::ID, reduction)]
				.into_iter()
				.try_collect()
				.expect("BoundedBTreeMap creation failed"),
			lock: Lock {
				started_at: 10000_u64,
				duration: 10000000_u64,
				unlock_penalty: Perbill::from_percent(2),
			},
			fnft_instance_id: 1,
		};

		assert_extrinsic_event::<Test, _, _, _>(
			StakingRewards::stake(Origin::signed(BOB), PICA::ID, PICA::units(1_000), 10_000_000),
			crate::Event::Staked {
				pool_id: PICA::ID,
				owner: BOB,
				amount: PICA::units(1_000),
				duration_preset: 10_000_000,
				fnft_collection_id: 1,
				fnft_instance_id: 1,
				keep_alive: false,
			},
		);
		Stakes::<Test>::insert(1, 0, stake.clone());
		let ratio = Permill::from_rational(1_u32, 7_u32);
		let left_from_one_ratio = ratio.left_from_one();
		let split = <StakingRewards as Staking>::split(&ALICE, &(1, 0), ratio);
		assert_ok!(split);
		let stake1 = Stakes::<Test>::get(1, 0);
		let stake2 = Stakes::<Test>::get(1, 1);
		assert!(stake1.is_some());
		assert!(stake2.is_some());
		let stake1 = stake1.unwrap();
		let stake2 = stake2.unwrap();
		// validate stake and share as per ratio
		assert_eq!(stake1.stake, ratio.mul_floor(stake.stake));
		assert_eq!(stake1.share, ratio.mul_floor(stake.share));
		assert_eq!(stake1.reductions.get(&USDT::ID), Some(&ratio.mul_floor(reduction)));
		assert_eq!(stake2.stake, left_from_one_ratio.mul_floor(stake.stake));
		assert_eq!(stake2.share, left_from_one_ratio.mul_floor(stake.share));
		assert_eq!(
			stake2.reductions.get(&USDT::ID),
			Some(&left_from_one_ratio.mul_floor(reduction))
		);
		assert_last_event::<Test, _>(|e| {
			matches!(&e.event,
			Event::StakingRewards(crate::Event::SplitPosition { positions })
			if positions == &vec![(PICA::ID, 0, stake1.stake), (PICA::ID, 1, stake2.stake)])
		});
	});
}

mod claim {
	use super::*;

	#[test]
	fn should_reward_correct_amount() {
		let staker = ALICE;
		let amount = 100_500;
		let duration_preset = ONE_HOUR;
		let total_rewards = 100;
		let total_shares = 200;
		let claim = 50;

		with_stake(
			staker,
			amount,
			duration_preset,
			total_rewards,
			total_shares,
			Some(claim),
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
		let total_shares = 200;
		let claim = 50;

		with_stake(
			staker,
			amount,
			duration_preset,
			total_rewards,
			total_shares,
			Some(claim),
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
		let staker = ALICE;
		let amount = 100_500;
		let duration_preset = ONE_HOUR;
		let total_rewards = 100;
		let total_shares = 200;
		let claim = 50;

		with_stake(
			staker,
			amount,
			duration_preset,
			total_rewards,
			total_shares,
			Some(claim),
			|_pool_id, _unlock_penalty, _stake_duration, _staked_asset_id| {
				assert_ok!(StakingRewards::claim(Origin::signed(staker), 1, 0));

				assert_last_event::<Test, _>(|e| {
					matches!(&e.event,
            		Event::StakingRewards(crate::Event::Claimed{ owner, fnft_collection_id, fnft_instance_id })
            		if owner == &staker && fnft_collection_id == &1 && fnft_instance_id == &0)
				});

				let stake = Stakes::<Test>::get(1, 0).expect("expected stake. QED");

				assert_eq!(stake.reductions.get(&USDT::ID), Some(&502_u128));
			},
		);
	}

	#[test]
	fn should_change_reward_pool_claimed_in_storage() {
		let staker = ALICE;
		let amount = 100_500;
		let duration_preset = ONE_HOUR;
		let total_rewards = 100;
		let total_shares = 200;
		let claim = 50;

		with_stake(
			staker,
			amount,
			duration_preset,
			total_rewards,
			total_shares,
			Some(claim),
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
					50
				);
			},
		);
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
	total_shares: u128,
	claim: Option<u128>,
	execute: impl FnOnce(u128, Perbill, u64, u128) -> R,
) -> R {
	new_test_ext().execute_with(|| {
		process_and_progress_blocks::<StakingRewards, Test>(1);
		assert_ok!(StakingRewards::create_reward_pool(Origin::root(), get_default_reward_pool()));

		let pool_id = PICA::ID;
		let rewards_pool = StakingRewards::pools(pool_id).expect("rewards_pool expected. QED");
		let staked_asset_id = rewards_pool.asset_id;

		mint_assets(
			[staker, StakingRewards::pool_account_id(&pool_id)],
			rewards_pool.rewards.keys().copied().chain([staked_asset_id]),
			amount.saturating_mul(2),
		);

		update_total_rewards_and_total_shares_in_rewards_pool(pool_id, total_rewards, total_shares);

		process_and_progress_blocks::<StakingRewards, Test>(1);
		assert_ok!(StakingRewards::stake(Origin::signed(staker), pool_id, amount, duration));
		assert_eq!(balance(staked_asset_id, &staker), amount);

		let mut stake = StakingRewards::stakes(1, 0).expect("stake expected. QED");
		let unlock_penalty = stake.lock.unlock_penalty;
		let stake_duration = stake.lock.duration;

		match claim {
			Some(claim) => {
				update_reductions(&mut stake.reductions, claim);
				Stakes::<Test>::insert(1, 0, stake);
			},
			None => (),
		}

		execute(pool_id, unlock_penalty, stake_duration, staked_asset_id)
	})
}

fn create_default_reward_pool() {
	assert_extrinsic_event::<Test, _, _, _>(
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
	}
}

fn default_lock_config() -> LockConfig<MaxStakingDurationPresets> {
	LockConfig {
		duration_presets: [
			(ONE_HOUR, Perbill::from_percent(1)),                // 1%
			(ONE_MINUTE, Perbill::from_rational(1_u32, 10_u32)), // 0.1%
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

fn mint_assets<'a>(
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

fn update_total_rewards_and_total_shares_in_rewards_pool(
	pool_id: u128,
	total_rewards: u128,
	total_shares: u128,
) {
	let mut rewards_pool = StakingRewards::pools(pool_id).expect("rewards_pool expected");
	let mut inner_rewards = rewards_pool.rewards.into_inner();
	for (_asset_id, reward) in inner_rewards.iter_mut() {
		reward.total_rewards += total_rewards;
	}
	rewards_pool.rewards = inner_rewards.try_into().expect("rewards expected");
	rewards_pool.total_shares = total_shares;
	RewardPools::<Test>::insert(pool_id, rewards_pool.clone());
}

fn update_reductions(
	reductions: &mut BoundedBTreeMap<u128, u128, MaxRewardConfigsPerPool>,
	claim: u128,
) {
	for (_asset_id, inflation) in reductions {
		*inflation -= claim;
	}
}
