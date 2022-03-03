#![cfg(feature = "runtime-benchmarks")]

#[cfg(test)]
use crate::Pallet as Vesting;
use crate::{AssetIdOf, BalanceOf, BlockNumberOf, Call, Config, Pallet, VestingScheduleOf};
use sp_runtime::{
	traits::StaticLookup,
};
use composable_traits::vesting::VestingSchedule;
use codec::Decode;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller, account, vec};
use frame_support::{
	dispatch::UnfilteredDispatchable,
	traits::{fungibles::Mutate as Mutate, Get},
};
use frame_system::RawOrigin;

fn asset<T>() -> AssetIdOf<T>
	where
		T: Config,
{
	let a = 0u128.to_be_bytes();
	AssetIdOf::<T>::decode(&mut &a[..]).unwrap()
}

fn fund_account<T>(caller: &T::AccountId, asset_id: AssetIdOf<T>, amount: BalanceOf<T>)
	where
		T: Config,
		BalanceOf<T>: From<u128>,
		<T as Config>::Currency: Mutate<T::AccountId, Balance = BalanceOf<T>, AssetId = AssetIdOf<T>>,
{
	T::Currency::mint_into(asset_id, &caller, amount).unwrap()
}

fn create_account<T>(name: &'static str, index: u32) -> T::AccountId
	where
		T: Config,
		BalanceOf<T>: From<u128>,
{
	let caller: T::AccountId = account(name, index, 0);
	caller
}

fn vesting_schedule<T>(start: BlockNumberOf<T>, period: BlockNumberOf<T>, period_count: u32,
					   per_period: BalanceOf<T>) -> VestingScheduleOf<T>
	where
		T: Config,
		BalanceOf<T>: From<u128>,
{
	VestingSchedule {
		start,
		period,
		period_count,
		per_period,
	}
}

benchmarks! {
  where_clause {
	  where
		T::Lookup: StaticLookup,
		BalanceOf<T>: From<u128>,
		BlockNumberOf<T>: From<u32>,
		<T as Config>::Currency: Mutate<T::AccountId, Balance = BalanceOf<T>, AssetId = AssetIdOf<T>>,
  }
	claim {
		// let s = T::MaxVestingSchedules::get();
		// for i in 0 .. (p - 1) {
		// 	VestingSchedules<T>.insert();
		// }
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), asset::<T>())

	vested_transfer {
		let asset_id = asset::<T>();
		let caller: T::AccountId = whitelisted_caller();
		fund_account::<T>(&caller, asset_id.clone(), 1_000_000_000_000.into());
		let dest = T::Lookup::unlookup(create_account::<T>("dest", 1));
		let start_block_number = 1;
		let period = 1;
		let per_period = T::MinVestedTransfer::get();
		let schedule = vesting_schedule::<T>(
			start_block_number.into(),
			period.into(),
			10,
			per_period.into()
		);
	}: _(RawOrigin::Signed(caller), dest, asset_id, schedule)

	update_vesting_schedules {
		let caller: T::AccountId = whitelisted_caller();
		let dest = T::Lookup::unlookup(create_account::<T>("dest", 1));
		let start_block_number = 1;
		let period = 1;
		let per_period = T::MinVestedTransfer::get();
		let schedule = vesting_schedule::<T>(
			start_block_number.into(),
			period.into(),
			1,
			per_period.into()
		);
	}: _(RawOrigin::Signed(caller), dest, asset::<T>(), vec![schedule])

	claim_for {
		let caller: T::AccountId = whitelisted_caller();
		let dest = T::Lookup::unlookup(create_account::<T>("dest", 1));
	}: _(RawOrigin::Signed(caller), dest, asset::<T>())
}

impl_benchmark_test_suite!(Vesting, crate::mock::ExtBuilder::build(), crate::mock::Runtime);
