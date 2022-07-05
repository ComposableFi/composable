use crate::{
	test::{prelude::H256, runtime::*},
	Config, StakeCount, Stakes
};
use composable_tests_helpers::test::currency::{PICA, USDT};
use composable_traits::{
	staking::{
		lock::{Lock, LockConfig}, RewardConfig, RewardPoolConfiguration,
		RewardPoolConfiguration::RewardRateBasedIncentive, Rewards,
        Staking, Stake, 
	},
	time::{DurationSeconds, ONE_HOUR, ONE_MINUTE},
};
use composable_support::abstractions::utils::increment::Increment;
use frame_support::{assert_err, assert_ok, BoundedBTreeMap};
use frame_system::EventRecord;
use sp_arithmetic::{Perbill, Permill};
use sp_core::sr25519::Public;
use sp_runtime::PerThing;
use sp_std::collections::btree_map::BTreeMap;
use composable_tests_helpers::test::currency::CurrencyId;

mod prelude;
mod runtime;

#[test]
fn test_create_reward_pool() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let mut pool_init_config = get_default_reward_pool();
		assert_ok!(StakingRewards::create_reward_pool(Origin::root(), pool_init_config));

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
fn test_split_postion() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let pool_init_config = get_default_reward_pool();
		assert_ok!(StakingRewards::create_reward_pool(Origin::root(), pool_init_config));
        let new_position = StakeCount::<Test>::increment();
        assert_ok!(new_position);
        let stake = Stake::<RewardPoolId, Balance, Rewards<CurrencyId, Balance, MaxRewardConfigsPerPool>> {
            reward_pool_id: 1,
            stake: 1000_000_000_000_000_u128,
            share: 1000_000_000_000_000_u128,
            reductions: Rewards::<_,_,_>::new(),
            lock: Lock {
                started_at: 10000_u64,
                duration: 10000000_u64,
                unlock_penalty: Perbill::from_percent(2)
            } 
        };
        Stakes::<Test>::insert(1, stake.clone());
        let ratio =  Permill::from_rational(1_u32,7_u32);
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
        assert_eq!(stake2.stake, left_from_one_ratio.mul_floor(stake.stake));
        assert_eq!(stake2.share, left_from_one_ratio.mul_floor(stake.share));
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
