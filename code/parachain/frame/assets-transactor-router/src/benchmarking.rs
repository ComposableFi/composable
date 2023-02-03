use super::*;

use crate::Pallet as Assets;
use composable_traits::assets::AssetInfo;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::traits::{fungible::Mutate as NativeMutate, fungibles::Mutate};
use frame_system::{Config as SystemConfig, RawOrigin};
use sp_runtime::traits::StaticLookup;

const FROM_ACCOUNT: u128 = 1;
const TO_ACCOUNT: u128 = 2;
const ASSET_ID: u128 = 2;
const TRANSFER_AMOUNT: u32 = 500;

benchmarks! {
	where_clause {
		where
				<T as Config>::AssetId: From<u128>,
				<T as SystemConfig>::AccountId: From<u128>,
	}

	transfer {
		let caller: T::AccountId = whitelisted_caller();
		let asset_id: T::AssetId = ASSET_ID.into();
		let dest = T::Lookup::unlookup(TO_ACCOUNT.into());
		let amount: T::Balance = TRANSFER_AMOUNT.into();
		<Assets<T> as Mutate<T::AccountId>>::mint_into(asset_id, &caller, amount)
			.expect("always can mint in test");
	}: _(RawOrigin::Signed(caller), asset_id, dest, amount, true)

	transfer_native {
		let caller: T::AccountId = whitelisted_caller();
		let dest = T::Lookup::unlookup(TO_ACCOUNT.into());
		let amount: T::Balance = TRANSFER_AMOUNT.into();
		<Assets<T> as NativeMutate<T::AccountId>>::mint_into(&caller, amount)
			.expect("always can mint in test");
	}: _(RawOrigin::Signed(caller), dest, amount, false)

	force_transfer {
		let caller: T::AccountId = FROM_ACCOUNT.into();
		let asset_id: T::AssetId = ASSET_ID.into();
		let from = T::Lookup::unlookup(FROM_ACCOUNT.into());
		let dest = T::Lookup::unlookup(TO_ACCOUNT.into());
		let amount: T::Balance = TRANSFER_AMOUNT.into();
		<Assets<T> as Mutate<T::AccountId>>::mint_into(asset_id, &caller, amount)
			.expect("always can mint in test");
	}: _(RawOrigin::Root, asset_id, from, dest, amount, false)

	force_transfer_native {
		let caller: T::AccountId = FROM_ACCOUNT.into();
		let from = T::Lookup::unlookup(FROM_ACCOUNT.into());
		let dest = T::Lookup::unlookup(TO_ACCOUNT.into());
		let amount: T::Balance = TRANSFER_AMOUNT.into();
		<Assets<T> as NativeMutate<T::AccountId>>::mint_into(&caller, amount)
			.expect("always can mint in test");
	}: _(RawOrigin::Root, from, dest, amount, false)

	transfer_all {
		let caller: T::AccountId = whitelisted_caller();
		let asset_id: T::AssetId = ASSET_ID.into();
		let dest = T::Lookup::unlookup(TO_ACCOUNT.into());
		let amount: T::Balance = TRANSFER_AMOUNT.into();
		<Assets<T> as Mutate<T::AccountId>>::mint_into(asset_id, &caller, amount)
			.expect("always can mint in test");
	}: _(RawOrigin::Signed(caller), asset_id, dest, false)

	transfer_all_native {
		let caller: T::AccountId = whitelisted_caller();
		let dest = T::Lookup::unlookup(TO_ACCOUNT.into());
		let amount: T::Balance = TRANSFER_AMOUNT.into();
		<Assets<T> as NativeMutate<T::AccountId>>::mint_into(&caller, amount)
			.expect("always can mint in test");
	}: _(RawOrigin::Signed(caller), dest, false)

	mint_initialize {
		let dest = T::Lookup::unlookup(TO_ACCOUNT.into());
		let amount: T::Balance = TRANSFER_AMOUNT.into();
		let asset_id = T::AssetId::from(100);
		let asset_info = AssetInfo {
			name: None,
			symbol: None,
			decimals: 12,
			ratio: None,
			existential_deposit: T::Balance::from(0_u32),
		};
	}: _(RawOrigin::Root, asset_id, asset_info, amount, dest)

	mint_initialize_with_governance {
		let governance = T::Lookup::unlookup(TO_ACCOUNT.into());
		let dest = T::Lookup::unlookup(TO_ACCOUNT.into());
		let amount: T::Balance = TRANSFER_AMOUNT.into();
		let asset_id = T::AssetId::from(100);
		let asset_info = AssetInfo {
			name: None,
			symbol: None,
			decimals: 12,
			ratio: None,
			existential_deposit: T::Balance::from(0_u32),
		};
	}: _(RawOrigin::Root, asset_id, asset_info, amount, governance, dest)

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
		<Assets<T> as Mutate<T::AccountId>>::mint_into(asset_id, &caller, amount)
			.expect("always can mint in test");
	}: _(RawOrigin::Root, asset_id, dest, amount)

}

impl_benchmark_test_suite!(Assets, crate::mocks::new_test_ext(), crate::mocks::Test,);
