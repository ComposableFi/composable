//! Benchmarks
use crate::{validation::ValidSplitRatio, *};

use composable_support::{abstractions::utils::increment::Increment, validation::Validated};
use composable_traits::{
	staking::{
		lock::{Lock, LockConfig},
		Reductions, RewardConfig, RewardPoolConfiguration,
		RewardPoolConfiguration::RewardRateBasedIncentive,
		RewardRate, RewardUpdate, Stake,
	},
	time::{DurationSeconds, ONE_HOUR, ONE_MINUTE},
};
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::{
	traits::{fungibles::Mutate, Get, TryCollect, UnixTime},
	BoundedBTreeMap,
};
use frame_system::{EventRecord, RawOrigin};
use sp_arithmetic::{traits::SaturatedConversion, Perbill, Permill};
use sp_std::collections::btree_map::BTreeMap;

pub const BASE_ASSET_ID: u128 = 101;

fn get_reward_pool<T: Config>(
	owner: T::AccountId,
	reward_count: u32,
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
		reward_configs: reward_config::<T>(reward_count),
		lock: lock_config::<T>(),
	};
	pool_init_config
}

fn lock_config<T: Config>(
) -> LockConfig<BoundedBTreeMap<DurationSeconds, Perbill, T::MaxStakingDurationPresets>> {
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

fn reward_config<T: Config>(
	reward_count: u32,
) -> BoundedBTreeMap<T::AssetId, RewardConfig<T::AssetId, T::Balance>, T::MaxRewardConfigsPerPool> {
	(0..reward_count)
		.map(|asset_id| {
			let asset_id = (asset_id as u128) + BASE_ASSET_ID;
			(
				asset_id.into(),
				RewardConfig {
					asset_id: asset_id.into(),
					max_rewards: 100_u128.into(),
					reward_rate: RewardRate::per_second(1_u128),
				},
			)
		})
		.try_collect()
		.unwrap()
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::Event = generic_event.into();
	// compare to the last event record
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

benchmarks! {
	where_clause {
		where
			T::BlockNumber: From<u32>,
			T::Balance: From<u128>,
			T::AssetId: From<u128>,
			T::RewardPoolId: From<u16>,
			T::PositionId: From<u128>,
	}

	create_reward_pool {
		let r in 1 .. T::MaxRewardConfigsPerPool::get();
		let owner: T::AccountId = account("owner", 0, 0);
		let pool_id = 1_u16.into();
		let end_block = 5_u128.saturated_into();
	}: _(RawOrigin::Root, get_reward_pool::<T>(owner.clone(), r))
	verify {
		assert_last_event::<T>(Event::RewardPoolCreated { pool_id, owner, end_block }.into());
	}

	stake {
		let r in 1 .. T::MaxRewardConfigsPerPool::get();
		let asset_id = 100.into();
		let pool_id = 1_u16.into();
		let amount = 100_500_u128.into();
		let duration_preset = ONE_HOUR;
		let position_id = 1_u128.into();
		let keep_alive = true;
		let staker = whitelisted_caller();
		let pool_owner: T::AccountId = account("owner", 0, 0);
		<Pallet<T>>::create_reward_pool(RawOrigin::Root.into(), get_reward_pool::<T>(pool_owner, r))?;
		<T::Assets as Mutate<T::AccountId>>::mint_into(asset_id, &staker, amount * 2.into())?;
	}: _(RawOrigin::Signed(staker.clone()), pool_id, amount, duration_preset)
	verify {
		assert_last_event::<T>(Event::Staked { pool_id, owner: staker, amount, duration_preset, position_id, keep_alive }.into());
	}

	extend {
		let r in 1 .. T::MaxRewardConfigsPerPool::get();
		let asset_id = 100.into();
		let pool_id = 1_u16.into();
		let amount = 100_500_u128.into();
		let duration_preset = ONE_HOUR;
		let position_id = 1_u128.into();
		let keep_alive = true;
		let staker = whitelisted_caller();
		let pool_owner: T::AccountId = account("owner", 0, 0);
		<Pallet<T>>::create_reward_pool(RawOrigin::Root.into(), get_reward_pool::<T>(pool_owner, r))?;
		<T::Assets as Mutate<T::AccountId>>::mint_into(asset_id, &staker, amount * 2.into()).expect("an asset minting expected");
		<Pallet<T>>::stake(RawOrigin::Signed(staker.clone()).into(), pool_id, amount, duration_preset)?;
	}: _(RawOrigin::Signed(staker.clone()), 1_u128.into(), amount)
	verify {
		assert_last_event::<T>(Event::StakeAmountExtended { position_id, amount}.into());
	}

	unstake {
		let r in 1 .. T::MaxRewardConfigsPerPool::get();
		let asset_id = 100.into();
		let pool_id = 1_u16.into();
		let amount = 100_500_u128.into();
		let duration_preset = ONE_HOUR;
		let position_id = 1_u128.into();
		let keep_alive = true;
		let staker = whitelisted_caller();
		let pool_owner: T::AccountId = account("owner", 0, 0);
		<Pallet<T>>::create_reward_pool(RawOrigin::Root.into(), get_reward_pool::<T>(pool_owner, r))?;
		<T::Assets as Mutate<T::AccountId>>::mint_into(asset_id, &staker, amount * 2.into())?;
		<Pallet<T>>::stake(RawOrigin::Signed(staker.clone()).into(), pool_id, amount, duration_preset)?;
	}: _(RawOrigin::Signed(staker.clone()), position_id)
	verify {
		assert_last_event::<T>(Event::Unstaked { owner: staker, position_id }.into());
	}

	split {
		let r in 1 .. T::MaxRewardConfigsPerPool::get();
		frame_system::Pallet::<T>::set_block_number(1.into());
		let user: T::AccountId = account("user", 0, 0);
		let _res = Pallet::<T>::create_reward_pool(RawOrigin::Root.into(), get_reward_pool::<T>(user.clone(), r));
		let _res = StakeCount::<T>::increment();
		let new_stake = Stake::<T::AccountId, T::RewardPoolId, T::Balance, Reductions<T::AssetId, T::Balance, T::MaxRewardConfigsPerPool>> {
			owner: user.clone(),
			reward_pool_id: 1_u16.into(),
			stake: 1_000_000_000_000_000_u128.into(),
			share: 1_000_000_000_000_000_u128.into(),
			reductions: Reductions::<_,_,_>::new(),
			lock: Lock {
				started_at: 10000_u64,
				duration: 10000000_u64,
				unlock_penalty: Perbill::from_percent(2)
			}
		};
		let position_id : T::PositionId = 1_u128.into();
		Stakes::<T>::insert(position_id, new_stake);
		let ratio =  Permill::from_rational(1_u32,7_u32);
		let validated_ratio = Validated::<Permill, ValidSplitRatio>::new(ratio).unwrap();

	}: _(RawOrigin::Signed(user), position_id, validated_ratio)

	reward_accumulation_hook_reward_update_calculation {
		let now = T::UnixTime::now().as_secs();
		let user: T::AccountId = account("user", 0, 0);
		let seconds_per_block = 12;
		let pool_asset_id = 100.into();
		let reward_asset_id = 1_u128.into();

		let reward_config = RewardConfig {
			asset_id: 1_u128.into(),
			max_rewards: 1_000_000.into(),
			reward_rate: RewardRate::per_second(10_000),
		};

		let pool_id = <Pallet<T> as ManageStaking>::create_staking_pool(RewardRateBasedIncentive {
			owner: user,
			asset_id: pool_asset_id,
			end_block: 5_u128.saturated_into(),
			reward_configs: [(reward_asset_id, reward_config)]
				.into_iter()
				.try_collect()
				.unwrap(),
			lock: lock_config::<T>(),
		}).unwrap();

		let now = now + seconds_per_block;

		let mut reward = RewardPools::<T>::get(&pool_id).unwrap().rewards.get(&reward_asset_id).unwrap().clone();
	}: {
		let reward = Pallet::<T>::reward_accumulation_hook_reward_update_calculation(pool_id, &mut reward, now);
	}

	unix_time_now {}: {
		T::UnixTime::now()
	}

	update_rewards_pool {
		let r in 1 .. T::MaxRewardConfigsPerPool::get();
		frame_system::Pallet::<T>::set_block_number(1.into());
		let user: T::AccountId = account("user", 0, 0);
		let pool_id = <Pallet<T> as ManageStaking>::create_staking_pool(get_reward_pool::<T>(user.clone(), r)).unwrap();

		let updates = (0..r).map(|r| (
			((r as u128) + BASE_ASSET_ID).into(),
			RewardUpdate {
				reward_rate: RewardRate::per_second(5)
			}
		))
		.into_iter()
		.collect::<BTreeMap<_, _>>()
		.try_into()
		.unwrap();
	}: _(RawOrigin::Root, pool_id, updates)

	add_to_rewards_pot {
		frame_system::Pallet::<T>::set_block_number(1.into());

		let asset_id = BASE_ASSET_ID.into();
		let amount = 100_u128.into();

		let user: T::AccountId = account("user", 0, 0);
		let pool_id = <Pallet<T> as ManageStaking>::create_staking_pool(get_reward_pool::<T>(user.clone(), 1)).unwrap();
		<T::Assets as Mutate<T::AccountId>>::mint_into(asset_id, &user, amount * 2.into())?;

	}: _(RawOrigin::Signed(user), pool_id,  asset_id, amount, true)

	impl_benchmark_test_suite!(Pallet, crate::test::new_test_ext(), crate::test::Test);
}
