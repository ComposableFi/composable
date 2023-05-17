use super::*;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::{assert_ok, traits::Hooks};
use frame_system::RawOrigin;
use sp_std::vec;

type CurrencyId = u128;

const PICA: CurrencyId = 1;
const KSM: CurrencyId = 2;
const DOT: CurrencyId = 0;
const REWARD: CurrencyId = 1000;
const CURRENCY_1: CurrencyId = 1001;
const CURRENCY_2: CurrencyId = 1002;
const CURRENCY_3: CurrencyId = 1003;

// Pallets
use crate::Pallet as Farming;
use frame_system::Pallet as System;

fn default_reward_schedule<T: Config>(reward_currency_id: CurrencyId) -> RewardScheduleOf<T> {
	let reward_schedule = RewardSchedule { period_count: 100u32, per_period: 1000u32.into() };
	let total_amount = reward_schedule.total().unwrap();

	assert_ok!(T::MultiCurrency::deposit(
		reward_currency_id.into(),
		&T::TreasuryAccountId::get(),
		total_amount,
	));

	reward_schedule
}

fn create_reward_schedule<T: Config>(pool_currency_id: CurrencyId, reward_currency_id: CurrencyId) {
	let reward_schedule = default_reward_schedule::<T>(reward_currency_id);

	assert_ok!(Farming::<T>::update_reward_schedule(
		RawOrigin::Root.into(),
		pool_currency_id.into(),
		reward_currency_id.into(),
		reward_schedule.period_count,
		reward_schedule.total().unwrap(),
	));
}

fn create_default_reward_schedule<T: Config>() -> (CurrencyId, CurrencyId) {
	let pool_currency_id = REWARD;
	let reward_currency_id = PICA;
	create_reward_schedule::<T>(pool_currency_id, reward_currency_id);
	(pool_currency_id, reward_currency_id)
}

fn deposit_lp_tokens<T: Config>(
	pool_currency_id: CurrencyId,
	account_id: &T::AccountId,
	amount: BalanceOf<T>,
) {
	assert_ok!(T::MultiCurrency::deposit(pool_currency_id.into(), account_id, amount));
	assert_ok!(Farming::<T>::deposit(
		RawOrigin::Signed(account_id.clone()).into(),
		pool_currency_id.into(),
		amount
	));
}

pub fn get_benchmarking_currency_ids() -> Vec<(CurrencyId, CurrencyId)> {
	vec![(DOT, PICA), (KSM, CURRENCY_1), (DOT, CURRENCY_2), (KSM, CURRENCY_3)]
}

benchmarks! {
	on_initialize {
		let c in 0 .. get_benchmarking_currency_ids().len() as u32;
		let currency_ids = get_benchmarking_currency_ids();
		let block_number = T::RewardPeriod::get();

		for i in 0 .. c {
			let (pool_currency_id, reward_currency_id) = currency_ids[i as usize];
			create_reward_schedule::<T>(pool_currency_id, reward_currency_id);
		}

		Farming::<T>::on_initialize(1u32.into());
		System::<T>::set_block_number(block_number);
	}: {
		Farming::<T>::on_initialize(System::<T>::block_number());
	}

	update_reward_schedule {
		let pool_currency_id = REWARD;
		let reward_currency_id = PICA;
		let reward_schedule = default_reward_schedule::<T>(reward_currency_id);

	}: _(RawOrigin::Root, pool_currency_id.into(), reward_currency_id.into(), reward_schedule.period_count, reward_schedule.total().unwrap())

	remove_reward_schedule {
		let (pool_currency_id, reward_currency_id) = create_default_reward_schedule::<T>();

	}: _(RawOrigin::Root, pool_currency_id.into(), reward_currency_id.into())

	deposit {
		let origin: T::AccountId = account("Origin", 0, 0);
		let (pool_currency_id, _) = create_default_reward_schedule::<T>();
		assert_ok!(T::MultiCurrency::deposit(
			pool_currency_id.into(),
			&origin,
			100u32.into(),
		));

	}: _(RawOrigin::Signed(origin), pool_currency_id.into(), 100u32.into())

	withdraw {
		let origin: T::AccountId = account("Origin", 0, 0);
		let (pool_currency_id, _) = create_default_reward_schedule::<T>();
		let amount = 100u32.into();
		deposit_lp_tokens::<T>(pool_currency_id.into(), &origin, amount);

	}: _(RawOrigin::Signed(origin), pool_currency_id.into(), amount)

	claim {
		let origin: T::AccountId = account("Origin", 0, 0);
		let (pool_currency_id, reward_currency_id) = create_default_reward_schedule::<T>();
		let amount = 100u32.into();
		deposit_lp_tokens::<T>(pool_currency_id.into(), &origin, amount);
		assert_ok!(T::RewardPools::distribute_reward(&pool_currency_id.into(), reward_currency_id.into(), amount));

	}: _(RawOrigin::Signed(origin), pool_currency_id.into(), reward_currency_id.into())
}

impl_benchmark_test_suite!(Farming, crate::mock::ExtBuilder::build(), crate::mock::Test);
