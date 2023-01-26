//! Benchmarks

use crate::*;

use composable_support::validation::TryIntoValidated;
use composable_tests_helpers::test::helper::RuntimeTrait;
// use composable_tests_helpers::test::helper::assert_extrinsic_event_with;
use composable_traits::{
	staking::{
		lock::{DurationMultipliers, LockConfig},
		RewardConfig,
		RewardPoolConfiguration::RewardRateBasedIncentive,
		RewardRate, RewardUpdate,
	},
	time::{ONE_HOUR, ONE_MINUTE},
};
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::{
	traits::{fungibles::Mutate, Get, OriginTrait, TryCollect, UnixTime},
	BoundedBTreeMap,
};
use frame_system::{pallet_prelude::OriginFor, EventRecord};
use sp_arithmetic::{fixed_point::FixedU64, traits::SaturatedConversion, Perbill, Permill};
use sp_runtime::traits::{BlockNumberProvider, One};
use sp_std::collections::btree_map::BTreeMap;

use crate::test_helpers::stake_and_assert;

// PICA as configured in the Test runtime (./frame/staking-rewards/src/test/runtime.rs)
pub const BASE_ASSET_ID: u128 = 42;
pub const X_ASSET_ID: u128 = 142;
pub const STAKING_FNFT_COLLECTION_ID: u128 = 1042;
pub const FNFT_INSTANCE_ID_BASE: u64 = 0;

fn get_reward_pool<T: Config>(
	owner: T::AccountId,
	reward_count: u32,
) -> RewardPoolConfigurationOf<T> {
	RewardRateBasedIncentive {
		owner,
		asset_id: BASE_ASSET_ID.into(),
		start_block: 2_u128.saturated_into(),
		reward_configs: reward_config::<T>(reward_count),
		lock: lock_config::<T>(),
		share_asset_id: X_ASSET_ID.into(),
		financial_nft_asset_id: STAKING_FNFT_COLLECTION_ID.into(),
		minimum_staking_amount: 10_000_u128.into(),
	}
}

fn lock_config<T: Config>() -> LockConfig<T::MaxStakingDurationPresets> {
	LockConfig {
		duration_multipliers: DurationMultipliers::Presets(
			[
				// 1%
				(ONE_HOUR, FixedU64::from_rational(101, 100).try_into_validated().expect(">= 1")),
				// 0.1%
				(
					ONE_MINUTE,
					FixedU64::from_rational(1_001, 1_000).try_into_validated().expect(">= 1"),
				),
			]
			.into_iter()
			.try_collect()
			.unwrap(),
		),
		unlock_penalty: Perbill::from_percent(5),
	}
}

fn reward_config<T: Config>(
	reward_count: u32,
) -> BoundedBTreeMap<T::AssetId, RewardConfig<T::Balance>, T::MaxRewardConfigsPerPool> {
	(0..reward_count)
		.map(|asset_id| {
			let asset_id = (asset_id as u128) + BASE_ASSET_ID;
			(asset_id.into(), RewardConfig { reward_rate: RewardRate::per_second(10_u128) })
		})
		.try_collect()
		.unwrap()
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::RuntimeEvent = generic_event.into();
	// compare to the last event record
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

benchmarks! {
	where_clause {
		where
			T::BlockNumber: From<u32> + One,
			T::Balance: From<u128>,
			T::AssetId: From<u128>,
			T: RuntimeTrait<crate::Event<T>> + Config,
	}

	create_reward_pool {
		let r in 1 .. T::MaxRewardConfigsPerPool::get();
		let owner: T::AccountId = account("owner", 0, 0);
		let pool_id = BASE_ASSET_ID.into();
	}: _(OriginFor::<T>::root(), get_reward_pool::<T>(owner.clone(), r))
	verify {
		assert_last_event::<T>(Event::RewardPoolCreated { pool_id, owner }.into());
	}

	stake {
		let r in 1 .. T::MaxRewardConfigsPerPool::get();
		let asset_id = BASE_ASSET_ID.into();
		let amount = 100_500_u128.into();
		let duration_preset = ONE_HOUR;
		let reward_multiplier = FixedU64::from_rational(101, 100);
		let keep_alive = true;
		let staker = whitelisted_caller();
		let pool_owner: T::AccountId = account("owner", 0, 0);

		frame_system::Pallet::<T>::set_block_number(1.into());
		<Pallet<T>>::create_reward_pool(OriginFor::<T>::root(), get_reward_pool::<T>(pool_owner, r))?;
		<T::Assets as Mutate<T::AccountId>>::mint_into(asset_id, &staker, amount * 2.into())?;

		frame_system::Pallet::<T>::set_block_number(2.into());
	}: _(OriginFor::<T>::signed(staker.clone()), asset_id, amount, duration_preset)
	verify {
		assert_last_event::<T>(
			Event::Staked {
				pool_id: asset_id,
				owner: staker,
				amount,
				duration_preset,
				fnft_collection_id: STAKING_FNFT_COLLECTION_ID.into(),
				fnft_instance_id: FNFT_INSTANCE_ID_BASE.into(),
				reward_multiplier,
				keep_alive
			}.into()
		);
	}

	extend {
		let r in 1 .. T::MaxRewardConfigsPerPool::get();
		let asset_id = BASE_ASSET_ID.into();
		let amount = 100_500_u128.into();
		let duration_preset = ONE_HOUR;
		let keep_alive = true;
		let staker = whitelisted_caller();
		let pool_owner: T::AccountId = account("owner", 0, 0);

		frame_system::Pallet::<T>::set_block_number(1.into());
		<Pallet<T>>::create_reward_pool(OriginFor::<T>::root(), get_reward_pool::<T>(pool_owner, r))?;
		<T::Assets as Mutate<T::AccountId>>::mint_into(asset_id, &staker, amount * 3.into()).expect("an asset minting expected");

		frame_system::Pallet::<T>::set_block_number(2.into());
		<Pallet<T>>::stake(OriginFor::<T>::signed(staker.clone()), asset_id, amount, duration_preset)?;
	}: _(OriginFor::<T>::signed(staker), STAKING_FNFT_COLLECTION_ID.into(), FNFT_INSTANCE_ID_BASE.into(), amount)
	verify {
		assert_last_event::<T>(
			Event::StakeAmountExtended {
				fnft_collection_id: STAKING_FNFT_COLLECTION_ID.into(),
				fnft_instance_id: FNFT_INSTANCE_ID_BASE.into(),
				amount
			}.into()
		);
	}

	unstake {
		let r in 1 .. T::MaxRewardConfigsPerPool::get();
		let asset_id = BASE_ASSET_ID.into();
		let amount = 100_500_u128.into();
		let duration_preset = ONE_HOUR;
		let keep_alive = true;
		let staker = whitelisted_caller();
		let pool_owner: T::AccountId = account("owner", 0, 0);

		frame_system::Pallet::<T>::set_block_number(1.into());
		<Pallet<T>>::create_reward_pool(OriginFor::<T>::root(), get_reward_pool::<T>(pool_owner, r))?;
		<T::Assets as Mutate<T::AccountId>>::mint_into(asset_id, &staker, amount * 2.into())?;

		frame_system::Pallet::<T>::set_block_number(2.into());
		<Pallet<T>>::stake(OriginFor::<T>::signed(staker.clone()), asset_id, amount, duration_preset)?;
	}: _(OriginFor::<T>::signed(staker.clone()), STAKING_FNFT_COLLECTION_ID.into(), FNFT_INSTANCE_ID_BASE.into())
	verify {
		assert_last_event::<T>(
			Event::Unstaked {
				owner: staker,
				fnft_collection_id: STAKING_FNFT_COLLECTION_ID.into(),
				fnft_instance_id: FNFT_INSTANCE_ID_BASE.into(),
				slash: Some(Perbill::from_percent(5).mul_ceil(amount))
			}.into(),
		);
	}

	split {
		let r in 1 .. T::MaxRewardConfigsPerPool::get();
		let user: T::AccountId = account("user", 0, 0);

		frame_system::Pallet::<T>::set_block_number(1.into());
		Pallet::<T>::create_reward_pool(
			OriginFor::<T>::root(),
			get_reward_pool::<T>(user.clone(), r)
		).expect("creating reward pool should succeed");

		frame_system::Pallet::<T>::set_block_number(frame_system::Pallet::<T>::current_block_number() + T::BlockNumber::one());

		<T::Assets as Mutate<T::AccountId>>::mint_into(
			BASE_ASSET_ID.into(),
			&user,
			100_000_000_000.into(),
		).expect("minting should succeed");

		let instance_id = stake_and_assert::<T>(
			user.clone(),
			BASE_ASSET_ID.into(),
			100_000_000.into(),
			ONE_HOUR,
		);

		let ratio = Permill::from_rational(1_u32, 7_u32)
			.try_into_validated()
			.unwrap();

	}: _(OriginFor::<T>::signed(user), STAKING_FNFT_COLLECTION_ID.into(), instance_id, ratio)

	reward_accumulation_hook_reward_update_calculation {
		let now = T::UnixTime::now().as_secs();
		let user: T::AccountId = account("user", 0, 0);
		let seconds_per_block = 12;
		let pool_asset_id = 100.into();
		let reward_asset_id = 1_u128.into();

		let reward_config = RewardConfig {
			reward_rate: RewardRate::per_second(10_000),
		};

		let pool_id = <Pallet<T> as ManageStaking>::create_staking_pool(RewardRateBasedIncentive {
			owner: user,
			asset_id: pool_asset_id,
			start_block: 2_u128.saturated_into(),
			reward_configs: [(reward_asset_id, reward_config)]
				.into_iter()
				.try_collect()
				.unwrap(),
			lock: lock_config::<T>(),
			share_asset_id: 1000.into(),
			financial_nft_asset_id: 2000.into(),
			minimum_staking_amount: 10_000.into(),
		}).unwrap();

		let now = now + seconds_per_block;

		let mut reward = RewardPools::<T>::get(&pool_id).unwrap().rewards.get(&reward_asset_id).unwrap().clone();
	}: {
		crate::reward_accumulation_hook_reward_update_calculation::<T>(pool_id, reward_asset_id,&mut reward, now);
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
	}: _(OriginFor::<T>::root(), pool_id, updates)

	claim {
		let r in 1 .. T::MaxRewardConfigsPerPool::get();
		let asset_id = BASE_ASSET_ID.into();
		let amount = 100_500_u128.into();
		let duration_preset = ONE_HOUR;
		let keep_alive = true;
		let staker = whitelisted_caller();
		let pool_owner: T::AccountId = account("owner", 0, 0);

		frame_system::Pallet::<T>::set_block_number(1.into());
		<Pallet<T>>::create_reward_pool(OriginFor::<T>::root(), get_reward_pool::<T>(pool_owner, r))?;
		<T::Assets as Mutate<T::AccountId>>::mint_into(asset_id, &staker, amount * 2.into())?;

		frame_system::Pallet::<T>::set_block_number(2.into());
		<Pallet<T>>::stake(OriginFor::<T>::signed(staker.clone()), asset_id, amount, duration_preset)?;
	}: _(OriginFor::<T>::signed(staker.clone()), STAKING_FNFT_COLLECTION_ID.into(), FNFT_INSTANCE_ID_BASE.into())
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

	}: _(OriginFor::<T>::signed(user), pool_id,  asset_id, amount, true)

	impl_benchmark_test_suite!(Pallet, crate::test::new_test_ext(), crate::runtime::Test);
}
