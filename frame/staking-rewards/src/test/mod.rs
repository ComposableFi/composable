use sp_std::collections::btree_map::BTreeMap;
use frame_support::{assert_err, assert_noop, assert_ok, BoundedBTreeMap};
use frame_system::EventRecord;
use sp_arithmetic::Perbill;
use composable_tests_helpers::test::currency::{CurrencyId, PICA, USDT};
use composable_traits::staking::lock::LockConfig;
use composable_traits::staking::{RewardConfig, RewardPoolConfiguration};
use composable_traits::staking::RewardPoolConfiguration::RewardRateBasedIncentive;
use composable_traits::time::{DurationSeconds, ONE_HOUR, ONE_MINUTE};
use crate::Config;
use crate::test::prelude::{AccountId, H256};
use crate::test::runtime::*;

mod prelude;
#[cfg(test)]
mod runtime;
#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;

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
        assert_err!(StakingRewards::create_reward_pool(
            Origin::root(), get_reward_pool_config_invalida_end_block()
        ), crate::Error::<Test>::InvalidEndBlock);

    });
}

fn get_default_reward_pool() -> RewardPoolConfiguration<
    AccountId, CurrencyId, Balance, BlockNumber, BoundedBTreeMap<u64, Perbill, MaxStakingDurationPresets>> {
    let pool_init_config = RewardRateBasedIncentive {
        owner: ALICE,
        asset_id: PICA::ID,
        end_block: 5,
        initial_reward_config: default_reward_config(),
        lock: default_lock_config()
    };
    pool_init_config
}

fn get_reward_pool_config_invalida_end_block() -> RewardPoolConfiguration<
    AccountId, CurrencyId, Balance, BlockNumber, BoundedBTreeMap<u64, Perbill, MaxStakingDurationPresets>> {
    let pool_init_config = RewardRateBasedIncentive {
        owner: ALICE,
        asset_id: PICA::ID,
        end_block: 0,
        initial_reward_config: default_reward_config(),
        lock: default_lock_config()
    };
    pool_init_config
}

fn default_lock_config() -> LockConfig<BoundedBTreeMap<DurationSeconds, Perbill, MaxStakingDurationPresets>> {
    let mut duration_presets = BTreeMap::new();
    duration_presets.insert(ONE_HOUR, Perbill::from_percent(1));
    duration_presets.insert(ONE_MINUTE, Perbill::from_rational(1_u32, 10_u32));
    LockConfig {
        duration_presets: BoundedBTreeMap::try_from(duration_presets).unwrap(),
        unlock_penalty: Perbill::from_percent(5),
    }
}

fn default_reward_config() -> RewardConfig<u128, u128> {
    RewardConfig {
        asset_id: USDT::ID,
        max_rewards: 100_u128,
        reward_rate: Perbill::from_percent(10),
    }
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
