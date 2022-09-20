//! Benchmarks
use crate::*;

use composable_support::validation::TryIntoValidated;
use composable_traits::{
	staking::{
		lock::LockConfig, RateBasedConfig, RewardConfig, RewardPoolConfig, RewardRate, RewardUpdate,
	},
	time::{ONE_HOUR, ONE_MINUTE},
};
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::traits::{fungibles::Mutate, Get, TryCollect, UnixTime};
use frame_system::{EventRecord, RawOrigin};
use sp_arithmetic::{traits::SaturatedConversion, Perbill, Permill};
use sp_std::collections::btree_map::BTreeMap;

// PICA as configured in the Test runtime (./frame/staking-rewards/src/test/runtime.rs)
pub const BASE_ASSET_ID: u128 = 42;
pub const X_ASSET_ID: u128 = 142;
pub const STAKING_FNFT_COLLECTION_ID: u128 = 1042;
pub const FNFT_INSTANCE_ID_BASE: u64 = 0;

fn get_reward_pool<T: Config>(
	owner: T::AccountId,
	reward_count: u32,
) -> RewardPoolConfigurationOf<T> {
	let pool_init_config = RewardPoolConfig {
		owner,
		asset_id: BASE_ASSET_ID.into(),
		end_block: 5_u128.saturated_into(),
		reward_configs: (0..reward_count)
			.map(|asset_id| {
				(
					((asset_id as u128) + BASE_ASSET_ID).into(),
					RewardConfig::RateBased(RateBasedConfig {
						max_rewards: 100_u128.into(),
						reward_rate: RewardRate::per_second(1_u128),
					}),
				)
			})
			.try_collect()
			.unwrap(),
		lock: lock_config::<T>(),
		share_asset_id: X_ASSET_ID.into(),
		fnft_asset_id: STAKING_FNFT_COLLECTION_ID.into(),
	};
	pool_init_config
}

fn lock_config<T: Config>() -> LockConfig<T::MaxStakingDurationPresets> {
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
	}

	create_reward_pool {
		let r in 1 .. T::MaxRewardConfigsPerPool::get();
		let owner: T::AccountId = account("owner", 0, 0);
		let pool_id = BASE_ASSET_ID.into();
		let end_block = 5_u128.saturated_into();
	}: _(RawOrigin::Root, get_reward_pool::<T>(owner.clone(), r))
	verify {
		assert_last_event::<T>(Event::RewardPoolCreated { pool_id, owner, end_block }.into());
	}

	stake {
		let r in 1 .. T::MaxRewardConfigsPerPool::get();
		let asset_id = BASE_ASSET_ID.into();
		let amount = 100_500_u128.into();
		let duration_preset = ONE_HOUR;
		let keep_alive = true;
		let staker = whitelisted_caller();
		let pool_owner: T::AccountId = account("owner", 0, 0);
		<Pallet<T>>::create_reward_pool(RawOrigin::Root.into(), get_reward_pool::<T>(pool_owner, r))?;
		<T::Assets as Mutate<T::AccountId>>::mint_into(asset_id, &staker, amount * 2.into())?;
	}: _(RawOrigin::Signed(staker.clone()), asset_id, amount, duration_preset)
	verify {
		assert_last_event::<T>(Event::Staked { pool_id: asset_id, owner: staker, amount, duration_preset, fnft_collection_id: STAKING_FNFT_COLLECTION_ID.into(), fnft_instance_id: FNFT_INSTANCE_ID_BASE.into(), keep_alive }.into());
	}

	extend {
		let r in 1 .. T::MaxRewardConfigsPerPool::get();
		let asset_id = BASE_ASSET_ID.into();
		let amount = 100_500_u128.into();
		let duration_preset = ONE_HOUR;
		let keep_alive = true;
		let staker = whitelisted_caller();
		let pool_owner: T::AccountId = account("owner", 0, 0);
		<Pallet<T>>::create_reward_pool(RawOrigin::Root.into(), get_reward_pool::<T>(pool_owner, r))?;
		<T::Assets as Mutate<T::AccountId>>::mint_into(asset_id, &staker, amount * 3.into()).expect("an asset minting expected");
		<Pallet<T>>::stake(RawOrigin::Signed(staker.clone()).into(), asset_id, amount, duration_preset)?;
	}: _(RawOrigin::Signed(staker.clone()), STAKING_FNFT_COLLECTION_ID.into(), FNFT_INSTANCE_ID_BASE.into(), amount)
	verify {
		assert_last_event::<T>(Event::StakeAmountExtended { fnft_collection_id: STAKING_FNFT_COLLECTION_ID.into(), fnft_instance_id: FNFT_INSTANCE_ID_BASE.into(), amount }.into());
	}

	unstake {
		let r in 1 .. T::MaxRewardConfigsPerPool::get();
		let asset_id = BASE_ASSET_ID.into();
		let amount = 100_500_u128.into();
		let duration_preset = ONE_HOUR;
		let keep_alive = true;
		let staker = whitelisted_caller();
		let pool_owner: T::AccountId = account("owner", 0, 0);
		<Pallet<T>>::create_reward_pool(RawOrigin::Root.into(), get_reward_pool::<T>(pool_owner, r))?;
		<T::Assets as Mutate<T::AccountId>>::mint_into(asset_id, &staker, amount * 2.into())?;
		<Pallet<T>>::stake(RawOrigin::Signed(staker.clone()).into(), asset_id, amount, duration_preset)?;
	}: _(RawOrigin::Signed(staker.clone()), STAKING_FNFT_COLLECTION_ID.into(), FNFT_INSTANCE_ID_BASE.into())
	verify {
		assert_last_event::<T>(Event::Unstaked { owner: staker, fnft_collection_id: STAKING_FNFT_COLLECTION_ID.into(), fnft_instance_id: FNFT_INSTANCE_ID_BASE.into() }.into());
	}

	split {
		let r in 1 .. T::MaxRewardConfigsPerPool::get();

		frame_system::Pallet::<T>::set_block_number(1.into());

		let user: T::AccountId = account("user", 0, 0);

		Pallet::<T>::create_reward_pool(
			RawOrigin::Root.into(),
			get_reward_pool::<T>(user.clone(), r)
		).unwrap();

		<T::Assets as Mutate<T::AccountId>>::mint_into(
			BASE_ASSET_ID.into(),
			&user,
			// PICA::units(1_000).into()
			100_000_000.into(),
		).unwrap();

		Pallet::<T>::stake(
			RawOrigin::Signed(user.clone()).into(),
			BASE_ASSET_ID.into(),
			// PICA::units(1_000).into(),
			100_000_000.into(),
			ONE_HOUR,
		).unwrap();

		let ratio =  Permill::from_rational(1_u32,7_u32).try_into_validated().unwrap();

	}: _(RawOrigin::Signed(user), STAKING_FNFT_COLLECTION_ID.into(), FNFT_INSTANCE_ID_BASE.into(), ratio)

	reward_accumulation_hook_reward_update_calculation {
		let now = T::UnixTime::now().as_secs();
		let user: T::AccountId = account("user", 0, 0);
		let seconds_per_block = 12;
		let pool_asset_id = 100.into();
		let reward_asset_id = 1_u128.into();

		let reward_config = RewardConfig::RateBased(RateBasedConfig {
			max_rewards: 1_000_000.into(),
			reward_rate: RewardRate::per_second(10_000),
		});

		let pool_id = <Pallet<T> as ManageStaking>::create_staking_pool(RewardPoolConfig {
			owner: user,
			asset_id: pool_asset_id,
			end_block: 5_u128.saturated_into(),
			reward_configs: [(reward_asset_id, reward_config)]
				.into_iter()
				.try_collect()
				.unwrap(),
			lock: lock_config::<T>(),
			share_asset_id: 1000.into(),
			fnft_asset_id: 2000.into(),
		}).unwrap();

		let now = now + seconds_per_block;

		let reward = RewardPools::<T>::get(&pool_id).unwrap().rewards.get(&reward_asset_id).unwrap().clone();
		let mut reward = match reward {
			Reward::ProtocolDistribution() => panic!("reward should be rate based"),
			Reward::RateBased(rate_based_reward) => rate_based_reward,
		};
	}: {
		Pallet::<T>::reward_accumulation_hook_rate_based_reward_update_calculation(
			pool_id,
			reward_asset_id,
			&mut reward,
			now
		);
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

	claim {
		let r in 1 .. T::MaxRewardConfigsPerPool::get();
		let asset_id = BASE_ASSET_ID.into();
		let amount = 100_500_u128.into();
		let duration_preset = ONE_HOUR;
		let keep_alive = true;
		let staker = whitelisted_caller();
		let pool_owner: T::AccountId = account("owner", 0, 0);
		<Pallet<T>>::create_reward_pool(RawOrigin::Root.into(), get_reward_pool::<T>(pool_owner, r))?;
		<T::Assets as Mutate<T::AccountId>>::mint_into(asset_id, &staker, amount * 2.into())?;
		<Pallet<T>>::stake(RawOrigin::Signed(staker.clone()).into(), asset_id, amount, duration_preset)?;
	}: _(RawOrigin::Signed(staker.clone()), STAKING_FNFT_COLLECTION_ID.into(), FNFT_INSTANCE_ID_BASE.into())
	verify {
		assert_last_event::<T>(Event::Claimed { owner: staker, fnft_collection_id: STAKING_FNFT_COLLECTION_ID.into(), fnft_instance_id: FNFT_INSTANCE_ID_BASE.into() }.into());
	}

	add_to_rewards_pot {
		frame_system::Pallet::<T>::set_block_number(1.into());

		let asset_id = BASE_ASSET_ID.into();
		let amount = 100_500_u128.into();

		let user: T::AccountId = account("user", 0, 0);
		let pool_id = <Pallet<T> as ManageStaking>::create_staking_pool(get_reward_pool::<T>(user.clone(), 1)).unwrap();
		<T::Assets as Mutate<T::AccountId>>::mint_into(asset_id, &user, amount * 2.into())?;

	}: _(RawOrigin::Signed(user), pool_id,  asset_id, amount, true)

	impl_benchmark_test_suite!(Pallet, crate::test::new_test_ext(), crate::test::Test);
}
