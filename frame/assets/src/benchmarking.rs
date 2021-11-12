use super::*;

use crate::Pallet as Assets;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::traits::{
	fungible::{Inspect as NativeInspect, Mutate as NativeMutate, Transfer as NativeTransfer},
	fungibles::{Inspect, Mutate, Transfer},
};
use frame_system::RawOrigin;
use sp_std::prelude::*;
use sp_runtime::traits::StaticLookup;
use frame_system::Config as SystemConfig;
const FROM_ACCOUNT: u64 = 1;
const TO_ACCOUNT: u64 = 2;
const ASSET_ID: u128 = 1;
const INIT_AMOUNT: u64 = 1000;
const TRANSFER_AMOUNT: u32 = 500;

benchmarks! {
	where_clause {
		 where <T as Config>::Currency: NativeTransfer<T::AccountId, Balance = T::Balance>
				   + NativeInspect<T::AccountId, Balance = T::Balance>
				   + NativeMutate<T::AccountId, Balance = T::Balance>,
			   <T as Config>::MultiCurrency: Inspect<T::AccountId, Balance = T::Balance, AssetId = T::AssetId>
				   + Transfer<T::AccountId, Balance = T::Balance, AssetId = T::AssetId>
				   + Mutate<T::AccountId, Balance = T::Balance, AssetId = T::AssetId>,
				<T as Config>::AssetId: From<u64>,   
				<T as SystemConfig>::AccountId: From<u64>,   

	}

	transfer {
		let caller: T::AccountId = whitelisted_caller();
		let dest = 2_u64.into();
		let dest = T::Lookup::unlookup(dest);
		// T::Currency::mint_into(T::AssetId::from(A), &caller, amount)
	}: _(RawOrigin::Signed(caller), 1_u64.into(), dest, 1_u32.into(), true)

	// transfer_native {
	// 	let caller: T::AccountId = whitelisted_caller();
	// 	// T::Currency::mint_into(T::AssetId::from(A), &caller, amount)
	// }: _(RawOrigin::Signed(caller), TO_ACCOUNT, TRANSFER_AMOUNT, true)

	// force_transfer {
	// }: _(RawOrigin::Root, ASSET_ID, FROM_ACCOUNT, TO_ACCOUNT, TRANSFER_AMOUNT, true)

	// force_transfer_native {
	// }: _(RawOrigin::Root, FROM_ACCOUNT, TO_ACCOUNT, TRANSFER_AMOUNT, true)

	// transfer_all {
	// 	let caller: T::AccountId = whitelisted_caller();
	// 	// T::Currency::mint_into(T::AssetId::from(A), &caller, amount)
	// }: _(RawOrigin::Signed(caller), ASSET_ID, TO_ACCOUNT, true)

	// transfer_all_native {
	// 	let caller: T::AccountId = whitelisted_caller();
	// 	// T::Currency::mint_into(T::AssetId::from(A), &caller, amount)
	// }: _(RawOrigin::Signed(caller), TO_ACCOUNT, true)

	// mint_initialize {
	// }: _(RawOrigin::Root, TRANSFER_AMOUNT, TO_ACCOUNT, true)

	// mint_initialize_with_governance {
	// }: _(RawOrigin::Root, TRANSFER_AMOUNT, TO_ACCOUNT, TO_ACCOUNT, true)

	// mint_into {
	// }: _(RawOrigin::Root, ASSET_ID, TO_ACCOUNT, TRANSFER_AMOUNT, true)

	// burn_from {
	// }: _(RawOrigin::Root, ASSET_ID, TO_ACCOUNT, TRANSFER_AMOUNT, true)

}

impl_benchmark_test_suite!(Assets, crate::mocks::new_test_ext(), crate::mocks::Test,);
