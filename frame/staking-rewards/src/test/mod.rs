use crate::{
	test::{prelude::H256, runtime::*},
	Config,
};
use composable_tests_helpers::test::currency::{PICA, USDT};
use composable_traits::{
	staking::{
		lock::LockConfig, RewardConfig, RewardPoolConfiguration,
		RewardPoolConfiguration::RewardRateBasedIncentive,
	},
	time::{DurationSeconds, ONE_HOUR, ONE_MINUTE},
};
use frame_support::{assert_err, assert_ok, traits::fungibles::Mutate, BoundedBTreeMap};
use frame_system::EventRecord;
use sp_arithmetic::Perbill;
use sp_core::sr25519::Public;
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
fn test_stake() {
	new_test_ext().execute_with(|| {
		assert_eq!(StakingRewards::stake_count(), 0);

		let pool_init_config = get_default_reward_pool();
		assert_ok!(StakingRewards::create_reward_pool(Origin::root(), pool_init_config));

		let (staker, pool_id, amount, duration_preset) = (ALICE, StakingRewards::pool_count(), 100_500u32.into(), ONE_HOUR);
		let asset_id = StakingRewards::pools(StakingRewards::pool_count()).unwrap().asset_id;
		<<Test as crate::Config>::Assets as Mutate<<Test as frame_system::Config>::AccountId>>::mint_into(asset_id, &staker, amount * 2);

		assert_ok!(StakingRewards::stake(Origin::signed(staker), pool_id, amount, duration_preset));
		assert_eq!(StakingRewards::stake_count(), 1);
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
