//! Benchmarks
use crate::*;
use composable_traits::{
	staking::{
		lock::LockConfig, RewardConfig, RewardPoolConfiguration,
		RewardPoolConfiguration::RewardRateBasedIncentive,
	},
	time::{DurationSeconds, ONE_HOUR, ONE_MINUTE},
};
use frame_benchmarking::{account, benchmarks};
use frame_support::BoundedBTreeMap;
use frame_system::RawOrigin;
use sp_arithmetic::{traits::SaturatedConversion, Perbill};
use sp_std::collections::btree_map::BTreeMap;

fn get_reward_pool<T: Config>(
	owner: T::AccountId,
) -> RewardPoolConfiguration<
	T::AccountId,
	T::AssetId,
	T::BlockNumber,
	BoundedBTreeMap<T::AssetId, RewardConfig<T::AssetId, T::Balance>, T::MaxRewardConfigsPerPool>,
	BoundedBTreeMap<DurationSeconds, Perbill, T::MaxStakingDurationPresets>,
> {
	let pool_init_config = RewardRateBasedIncentive {
		owner,
		asset_id: 100.into(),
		end_block: 5_u128.saturated_into(),
		reward_configs: reward_config::<T>(),
		lock: lock_config::<T>(),
	};
	pool_init_config
}

fn lock_config<T: Config>(
) -> LockConfig<BoundedBTreeMap<DurationSeconds, Perbill, T::MaxStakingDurationPresets>> {
	let mut duration_presets = BTreeMap::new();
	duration_presets.insert(ONE_HOUR, Perbill::from_percent(1));
	duration_presets.insert(ONE_MINUTE, Perbill::from_rational(1_u32, 10_u32));
	LockConfig {
		duration_presets: BoundedBTreeMap::try_from(duration_presets).unwrap(),
		unlock_penalty: Perbill::from_percent(5),
	}
}

fn reward_config<T: Config>(
) -> BoundedBTreeMap<T::AssetId, RewardConfig<T::AssetId, T::Balance>, T::MaxRewardConfigsPerPool> {
	let config = RewardConfig {
		asset_id: 101.into(),
		max_rewards: 100_u128.into(),
		reward_rate: Perbill::from_percent(10),
	};
	let mut rewards = BTreeMap::new();
	rewards.insert(101.into(), config);
	BoundedBTreeMap::try_from(rewards).unwrap()
}

benchmarks! {
  where_clause {
		where T::BlockNumber: From<u32>, T::Balance: From<u128>, T::AssetId: From<u128>
	}
	create_reward_pool {
		let owner: T::AccountId = account("owner", 0, 0);
	} : _(RawOrigin::Root, get_reward_pool::<T>(owner))
}
