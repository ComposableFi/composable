#![cfg(feature = "runtime-benchmarks")]

#[cfg(test)]
use crate::Pallet as Vesting;
use crate::{
	AssetIdOf, BalanceOf, BlockNumberOf, Call, Config, Pallet, VestedTransfer, VestingScheduleOf,
};
use codec::Decode;
use composable_traits::vesting::{VestingSchedule, VestingWindow::BlockNumberBased};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller};
use frame_support::traits::{fungibles::Mutate, Get};
use frame_system::RawOrigin;
use sp_runtime::traits::{StaticLookup, TrailingZeroInput};

const FUNDING: u64 = 1_000_000_000_000_000;
const PERIOD_COUNT: u32 = 10;
const PERIOD: u32 = 1;
const START_BLOCK_NUMBER: u32 = 1;

fn asset<T>() -> AssetIdOf<T>
where
	T: Config,
{
	let a = 1u128.to_be_bytes();
	AssetIdOf::<T>::decode(&mut &a[..]).unwrap()
}

fn fund_account<T>(caller: &T::AccountId, asset_id: AssetIdOf<T>, amount: BalanceOf<T>)
where
	T: Config,
	BalanceOf<T>: From<u64>,
	<T as Config>::Currency: Mutate<T::AccountId, Balance = BalanceOf<T>, AssetId = AssetIdOf<T>>,
{
	T::Currency::mint_into(asset_id, &caller, amount).unwrap()
}

fn create_account<T>(name: &'static str, index: u32) -> T::AccountId
where
	T: Config,
{
	let caller: T::AccountId = account(name, index, 0);
	caller
}

fn vesting_schedule<T>(
	start: BlockNumberOf<T>,
	period: BlockNumberOf<T>,
	period_count: u32,
	per_period: BalanceOf<T>,
	vesting_schedule_id: u128,
) -> VestingScheduleOf<T>
where
	T: Config,
	BalanceOf<T>: From<u64>,
{
	VestingSchedule {
		vesting_schedule_id,
		window: BlockNumberBased { start, period },
		period_count,
		per_period,
		already_claimed: BalanceOf::<T>::zero(),
	}
}

fn zero_account<T>() -> T::AccountId
where
	T: Config,
{
	T::AccountId::decode(&mut TrailingZeroInput::zeroes()).unwrap()
}

benchmarks! {
  where_clause {
	  where
		T::Lookup: StaticLookup,
		BalanceOf<T>: From<u64>,
		BlockNumberOf<T>: From<u32>,
		<T as Config>::Currency: Mutate<T::AccountId, Balance = BalanceOf<T>, AssetId = AssetIdOf<T>>,
  }
	claim {
		let s in 0 .. T::MaxVestingSchedules::get();
		let asset_id = asset::<T>();
		let caller: T::AccountId = whitelisted_caller();
		let per_period = T::MinVestedTransfer::get();
		let schedule = vesting_schedule::<T>(
			START_BLOCK_NUMBER.into(),
			PERIOD.into(),
			PERIOD_COUNT,
			per_period.into(),
			VestingScheduleCount::<T>::get().increment()?
		);
		for i in 0 .. s {
			let source = create_account::<T>("source", i);
			fund_account::<T>(&source, asset_id.clone(), FUNDING.into());
			<Pallet<T> as VestedTransfer>::vested_transfer(asset_id.clone(), &source, &caller, schedule.clone()).unwrap();
		}
	}: _(RawOrigin::Signed(caller), asset_id)

	vested_transfer {
		let asset_id = asset::<T>();
		let from: T::AccountId = create_account::<T>("from", 0xCAFEBABE);
		fund_account::<T>(&from, asset_id.clone(), FUNDING.into());
		let dest = T::Lookup::unlookup(create_account::<T>("dest", 1));
		let per_period = T::MinVestedTransfer::get();
		let schedule = vesting_schedule::<T>(
			START_BLOCK_NUMBER.into(),
			PERIOD.into(),
			PERIOD_COUNT,
			per_period.into()
		);
	}: _(RawOrigin::Root, T::Lookup::unlookup(from), dest, asset_id, schedule)

	update_vesting_schedules {
		let s in 0 .. T::MaxVestingSchedules::get();
		let mut schedules = vec![];
		let asset_id = asset::<T>();
		let caller: T::AccountId = whitelisted_caller();
		let dest = create_account::<T>("dest", 1);
		let dest_look_up = T::Lookup::unlookup(dest.clone());
		let per_period = T::MinVestedTransfer::get();
		for i in 0..s {
			fund_account::<T>(&dest, asset_id.clone(), FUNDING.into());
			schedules.push(vesting_schedule::<T>(
				START_BLOCK_NUMBER.into(),
				PERIOD.into(),
				PERIOD_COUNT,
				per_period.into()
			));
		}
	}: _(RawOrigin::Root, dest_look_up, asset_id, schedules)

	claim_for {
		let s in 0 .. T::MaxVestingSchedules::get();
		let asset_id = asset::<T>();
		let caller: T::AccountId = whitelisted_caller();
		let per_period = T::MinVestedTransfer::get();
		let schedule = vesting_schedule::<T>(
			START_BLOCK_NUMBER.into(),
			PERIOD.into(),
			PERIOD_COUNT,
			per_period.into()
		);
		let dest = create_account::<T>("dest", 1);
		let dest_look_up = T::Lookup::unlookup(dest.clone());
		for i in 0 .. s {
			fund_account::<T>(&caller, asset_id.clone(), FUNDING.into());
			<Pallet<T> as VestedTransfer>::vested_transfer(asset_id.clone(), &caller, &dest, schedule.clone()).unwrap();
		}
	}: _(RawOrigin::Signed(caller), dest_look_up, asset_id)
}

impl_benchmark_test_suite!(Vesting, crate::mock::ExtBuilder::build(), crate::mock::Runtime);
