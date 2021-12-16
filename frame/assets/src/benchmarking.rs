use super::*;

// FIXME(oleksii): why is this marked as unused?
#[allow(unused_imports)]
use crate::Pallet as Assets;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::traits::{
	fungible::{Inspect as NativeInspect, Mutate as NativeMutate, Transfer as NativeTransfer},
	fungibles::{Inspect, Mutate, Transfer},
};
use frame_system::{Config as SystemConfig, RawOrigin};
use sp_runtime::traits::StaticLookup;

const FROM_ACCOUNT: u64 = 1;
const TO_ACCOUNT: u64 = 2;
const ASSET_ID: u64 = 2;
const TRANSFER_AMOUNT: u32 = 500;

benchmarks! {
	where_clause {
		 where <T as Config>::NativeCurrency: NativeTransfer<T::AccountId, Balance = T::Balance>
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
		let asset_id: T::AssetId = ASSET_ID.into();
		let dest = T::Lookup::unlookup(TO_ACCOUNT.into());
		let amount: T::Balance = TRANSFER_AMOUNT.into();
		T::MultiCurrency::mint_into(asset_id, &caller, amount).unwrap();
	}: _(RawOrigin::Signed(caller), asset_id, dest, amount, true)

	transfer_native {
		let caller: T::AccountId = whitelisted_caller();
		let dest = T::Lookup::unlookup(TO_ACCOUNT.into());
		let amount: T::Balance = TRANSFER_AMOUNT.into();
		T::NativeCurrency::mint_into(&caller, amount).unwrap();
	}: _(RawOrigin::Signed(caller), dest, amount, false)

	force_transfer {
		let caller: T::AccountId = FROM_ACCOUNT.into();
		let asset_id: T::AssetId = ASSET_ID.into();
		let from = T::Lookup::unlookup(FROM_ACCOUNT.into());
		let dest = T::Lookup::unlookup(TO_ACCOUNT.into());
		let amount: T::Balance = TRANSFER_AMOUNT.into();
		T::MultiCurrency::mint_into(asset_id, &caller, amount).unwrap();
	}: _(RawOrigin::Root, asset_id, from, dest, amount, false)

	force_transfer_native {
		let caller: T::AccountId = FROM_ACCOUNT.into();
		let from = T::Lookup::unlookup(FROM_ACCOUNT.into());
		let dest = T::Lookup::unlookup(TO_ACCOUNT.into());
		let amount: T::Balance = TRANSFER_AMOUNT.into();
		T::NativeCurrency::mint_into(&caller, amount).unwrap();
	}: _(RawOrigin::Root, from, dest, amount, false)

	transfer_all {
		let caller: T::AccountId = whitelisted_caller();
		let asset_id: T::AssetId = ASSET_ID.into();
		let dest = T::Lookup::unlookup(TO_ACCOUNT.into());
		let amount: T::Balance = TRANSFER_AMOUNT.into();
		T::MultiCurrency::mint_into(asset_id, &caller, amount).unwrap();
	}: _(RawOrigin::Signed(caller), asset_id, dest, false)

	transfer_all_native {
		let caller: T::AccountId = whitelisted_caller();
		let dest = T::Lookup::unlookup(TO_ACCOUNT.into());
		let amount: T::Balance = TRANSFER_AMOUNT.into();
		T::NativeCurrency::mint_into(&caller, amount).unwrap();
	}: _(RawOrigin::Signed(caller), dest, false)

	mint_initialize {
		let dest = T::Lookup::unlookup(TO_ACCOUNT.into());
		let amount: T::Balance = TRANSFER_AMOUNT.into();
	}: _(RawOrigin::Root, amount, dest)

	mint_initialize_with_governance {
		let governance = T::Lookup::unlookup(TO_ACCOUNT.into());
		let dest = T::Lookup::unlookup(TO_ACCOUNT.into());
		let amount: T::Balance = TRANSFER_AMOUNT.into();
	}: _(RawOrigin::Root, amount, governance, dest)

	mint_into {
		let asset_id: T::AssetId = ASSET_ID.into();
		let dest = T::Lookup::unlookup(TO_ACCOUNT.into());
		let amount: T::Balance = TRANSFER_AMOUNT.into();
	}: _(RawOrigin::Root, asset_id, dest, amount)

	burn_from {
		let caller: T::AccountId = TO_ACCOUNT.into();
		let asset_id: T::AssetId = ASSET_ID.into();
		let dest = T::Lookup::unlookup(TO_ACCOUNT.into());
		let amount: T::Balance = TRANSFER_AMOUNT.into();
		T::MultiCurrency::mint_into(asset_id, &caller, amount).unwrap();
	}: _(RawOrigin::Root, asset_id, dest, amount)

}

impl_benchmark_test_suite!(Assets, crate::mocks::new_test_ext(), crate::mocks::Test,);
