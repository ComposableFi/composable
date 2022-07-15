pub(crate) use crate::test::runtime::{new_test_ext, Test}; // for benchmarks
use crate::{
	test::{prelude::H256, runtime::*},
	Config, RewardPools, StakeCount, Stakes,
};
use composable_support::abstractions::utils::increment::Increment;
use composable_tests_helpers::test::currency::{CurrencyId, BTC, PICA, USDT};
use composable_traits::{
	staking::{
		lock::{Lock, LockConfig},
		ProtocolStaking, Reductions, RewardConfig, RewardPoolConfiguration,
		RewardPoolConfiguration::RewardRateBasedIncentive,
		Rewards, Stake, Staking,
	},
	time::{DurationSeconds, ONE_HOUR, ONE_MINUTE},
};
use frame_support::{
	assert_err, assert_noop, assert_ok,
	traits::fungibles::{Inspect, Mutate},
	BoundedBTreeMap,
};
use frame_system::EventRecord;
use sp_arithmetic::{Perbill, Permill};
use sp_core::sr25519::Public;
use sp_runtime::PerThing;
use sp_std::collections::btree_map::BTreeMap;

mod prelude;
mod runtime;

#[test]
fn test_create_reward_pool() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_eq!(StakingRewards::pool_count(), 0);
		let pool_init_config = get_default_reward_pool();
		assert_ok!(StakingRewards::create_reward_pool(Origin::root(), pool_init_config));
		assert_eq!(StakingRewards::pool_count(), 1);

		assert_last_event::<Test, _>(|e| {
			matches!(e.event,
            Event::StakingRewards(crate::Event::RewardPoolCreated { owner, pool_id, .. })
            if owner == ALICE && pool_id == 1)
		});

		// invalid end block
		assert_err!(
			StakingRewards::create_reward_pool(
				Origin::root(),
				get_reward_pool_config_invalid_end_block()
			),
			crate::Error::<Test>::EndBlockMustBeInTheFuture
		);
	});
}

#[test]
fn stake_in_case_of_low_balance_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_eq!(StakingRewards::stake_count(), 0);

		let pool_init_config = get_default_reward_pool();
		assert_ok!(StakingRewards::create_reward_pool(Origin::root(), pool_init_config));
		let (staker, pool_id, amount, duration_preset) =
			(ALICE, StakingRewards::pool_count(), 100_500u32.into(), ONE_HOUR);

		let asset_id = StakingRewards::pools(pool_id).expect("asset_id expected").asset_id;
		assert_eq!(
			<<Test as crate::Config>::Assets as Inspect<
				<Test as frame_system::Config>::AccountId,
			>>::balance(asset_id, &staker),
			0
		);

		assert_noop!(
			StakingRewards::stake(Origin::signed(staker), pool_id, amount, duration_preset),
			crate::Error::<Test>::NotEnoughAssets
		);

		assert_eq!(StakingRewards::stake_count(), 0);
	});
}

#[test]
fn stake_in_case_of_zero_inflation_should_work() {
	new_test_ext().execute_with(|| {
		assert_eq!(StakingRewards::stake_count(), 0);

		let pool_init_config = get_default_reward_pool();
		assert_ok!(StakingRewards::create_reward_pool(Origin::root(), pool_init_config));
		let (staker, pool_id, amount, duration_preset) = (ALICE, StakingRewards::pool_count(), 100_500u32.into(), ONE_HOUR);

		let asset_id = StakingRewards::pools(StakingRewards::pool_count()).expect("asset_id expected").asset_id;
		<<Test as crate::Config>::Assets as Mutate<<Test as frame_system::Config>::AccountId>>::mint_into(asset_id, &staker, amount * 2).expect("an asset minting expected");

		assert_ok!(StakingRewards::stake(Origin::signed(staker), pool_id, amount, duration_preset));
		assert_eq!(StakingRewards::stake_count(), 1);
		let rewards_pool = StakingRewards::pools(pool_id).expect("rewards_pool expected");
		let reward_multiplier = StakingRewards::reward_multiplier(&rewards_pool, duration_preset).expect("reward_multiplier expected");
		let inflation = 0;
		let reductions = Reductions::try_from(rewards_pool.rewards.into_inner().iter().map(|(asset_id, _reward)| (*asset_id, inflation)).collect::<BTreeMap<_, _>>()).expect("reductions expected");
		assert_eq!(
			StakingRewards::stakes(StakingRewards::stake_count()),
			Some(Stake {
				reward_pool_id: pool_id,
				stake: amount,
				share: StakingRewards::boosted_amount(reward_multiplier, amount),
				reductions,
				lock: Lock {
					started_at: <Test as crate::Config>::UnixTime::now(),
					duration: duration_preset,
					unlock_penalty: rewards_pool.lock.unlock_penalty,
				},
			})
		);
		assert_eq!(<<Test as crate::Config>::Assets as Inspect<<Test as frame_system::Config>::AccountId>>::balance(asset_id, &staker), amount);
		assert_eq!(<<Test as crate::Config>::Assets as Inspect<<Test as frame_system::Config>::AccountId>>::balance(asset_id, &StakingRewards::pool_account_id(&pool_id)), amount);
	});
}

#[test]
fn stake_in_case_of_not_zero_inflation_should_work() {
	new_test_ext().execute_with(|| {
		assert_eq!(StakingRewards::stake_count(), 0);

		let pool_init_config = get_default_reward_pool();
		assert_ok!(StakingRewards::create_reward_pool(Origin::root(), pool_init_config));
		let (staker, pool_id, amount, duration_preset, total_rewards, total_shares) = (ALICE, StakingRewards::pool_count(), 100_500u32.into(), ONE_HOUR, 100, 200);

		let asset_id = StakingRewards::pools(StakingRewards::pool_count()).expect("asset_id expected").asset_id;
		<<Test as crate::Config>::Assets as Mutate<<Test as frame_system::Config>::AccountId>>::mint_into(asset_id, &staker, amount * 2).expect("an asset minting expected");

		let mut rewards_pool = StakingRewards::pools(pool_id).expect("rewards_pool expected");
		let mut inner_rewards = rewards_pool.rewards.into_inner();
		for (_asset_id, reward) in inner_rewards.iter_mut() {
			reward.total_rewards += total_rewards;
		}
		rewards_pool.rewards = inner_rewards.try_into().expect("rewards expected");
		rewards_pool.total_shares = total_shares;
		RewardPools::<Test>::insert(pool_id, rewards_pool.clone());

		assert_ok!(StakingRewards::stake(Origin::signed(staker), pool_id, amount, duration_preset));
		assert_eq!(StakingRewards::stake_count(), 1);
		let rewards_pool = StakingRewards::pools(pool_id).expect("rewards_pool expected");
		let reward_multiplier = StakingRewards::reward_multiplier(&rewards_pool, duration_preset).expect("reward_multiplier expected");
		let inflation = StakingRewards::boosted_amount(reward_multiplier, amount) * total_rewards / total_shares;
		assert_eq!(inflation, 502);
		let reductions = Reductions::try_from(rewards_pool.rewards.into_inner().iter().map(|(asset_id, _reward)| (*asset_id, inflation)).collect::<BTreeMap<_, _>>()).expect("reductions expected");
		assert_eq!(
			StakingRewards::stakes(StakingRewards::stake_count()),
			Some(Stake {
				reward_pool_id: pool_id,
				stake: amount,
				share: StakingRewards::boosted_amount(reward_multiplier, amount),
				reductions,
				lock: Lock {
					started_at: <Test as crate::Config>::UnixTime::now(),
					duration: duration_preset,
					unlock_penalty: rewards_pool.lock.unlock_penalty,
				},
			})
		);
		assert_eq!(<<Test as crate::Config>::Assets as Inspect<<Test as frame_system::Config>::AccountId>>::balance(asset_id, &staker), amount);
		assert_eq!(<<Test as crate::Config>::Assets as Inspect<<Test as frame_system::Config>::AccountId>>::balance(asset_id, &StakingRewards::pool_account_id(&pool_id)), amount);
	});
}

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
		assert_noop!(
			<StakingRewards as ProtocolStaking>::transfer_reward(&BOB, &1, BTC::ID, 10_000_u128),
			crate::Error::<Test>::OnlyPoolOwnerCanAddNewReward
		);

		assert_ok!(<StakingRewards as ProtocolStaking>::transfer_reward(
			&ALICE,
			&1,
			BTC::ID,
			10_000_u128
		));
	});
}

#[test]
fn test_split_postion() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let pool_init_config = get_default_reward_pool();
		assert_ok!(StakingRewards::create_reward_pool(Origin::root(), pool_init_config));
		let new_position = StakeCount::<Test>::increment();
		assert_ok!(new_position);
		let reduction = 10_000_000_000_000_u128;
		let stake = Stake::<
			RewardPoolId,
			Balance,
			Reductions<CurrencyId, Balance, MaxRewardConfigsPerPool>,
		> {
			reward_pool_id: 1,
			stake: 1000_000_000_000_000_u128,
			share: 1000_000_000_000_000_u128,
			reductions: Reductions::<_, _, _>::try_from(BTreeMap::from([(USDT::ID, reduction)]))
				.expect("BoundedBTreeMap creation failed"),
			lock: Lock {
				started_at: 10000_u64,
				duration: 10000000_u64,
				unlock_penalty: Perbill::from_percent(2),
			},
		};
		Stakes::<Test>::insert(1, stake.clone());
		let ratio = Permill::from_rational(1_u32, 7_u32);
		let left_from_one_ratio = ratio.left_from_one();
		let split = <StakingRewards as Staking>::split(&ALICE, &1_u128, ratio);
		assert_ok!(split);
		let stake1 = Stakes::<Test>::get(1);
		let stake2 = Stakes::<Test>::get(2);
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
	});
}

fn get_default_reward_pool() -> RewardPoolConfiguration<
	Public,
	u128,
	BlockNumber,
	BoundedBTreeMap<u128, RewardConfig<u128, u128>, MaxRewardConfigsPerPool>,
	BoundedBTreeMap<DurationSeconds, Perbill, MaxStakingDurationPresets>,
> {
	let pool_init_config = RewardRateBasedIncentive {
		owner: ALICE,
		asset_id: PICA::ID,
		end_block: 5,
		reward_configs: default_reward_config(),
		lock: default_lock_config(),
	};
	pool_init_config
}

fn get_reward_pool_config_invalid_end_block() -> RewardPoolConfiguration<
	Public,
	u128,
	BlockNumber,
	BoundedBTreeMap<u128, RewardConfig<u128, u128>, MaxRewardConfigsPerPool>,
	BoundedBTreeMap<DurationSeconds, Perbill, MaxStakingDurationPresets>,
> {
	let pool_init_config = RewardRateBasedIncentive {
		owner: ALICE,
		asset_id: PICA::ID,
		end_block: 0,
		reward_configs: default_reward_config(),
		lock: default_lock_config(),
	};
	pool_init_config
}

fn default_lock_config(
) -> LockConfig<BoundedBTreeMap<DurationSeconds, Perbill, MaxStakingDurationPresets>> {
	let mut duration_presets = BTreeMap::new();
	duration_presets.insert(ONE_HOUR, Perbill::from_percent(1));
	duration_presets.insert(ONE_MINUTE, Perbill::from_rational(1_u32, 10_u32));
	LockConfig {
		duration_presets: BoundedBTreeMap::try_from(duration_presets).unwrap(),
		unlock_penalty: Perbill::from_percent(5),
	}
}

fn default_reward_config(
) -> BoundedBTreeMap<u128, RewardConfig<u128, u128>, MaxRewardConfigsPerPool> {
	let config = RewardConfig {
		asset_id: USDT::ID,
		max_rewards: 100_u128,
		reward_rate: Perbill::from_percent(10),
	};
	let mut rewards = BTreeMap::new();
	rewards.insert(USDT::ID, config);
	BoundedBTreeMap::try_from(rewards).unwrap()
}

pub fn assert_has_event<T, F>(matcher: F)
where
	T: Config,
	F: Fn(&EventRecord<Event, H256>) -> bool,
{
	assert!(System::events().iter().any(matcher));
}

pub fn assert_last_event<T, F>(matcher: F)
where
	T: Config,
	F: FnOnce(&EventRecord<Event, H256>) -> bool,
{
	assert!(matcher(System::events().last().expect("events expected")));
}
