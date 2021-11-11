use super::*;

use crate::tests::{ASSET_ID, FROM_ACCOUNT, INIT_AMOUNT, TO_ACCOUNT, TRANSFER_AMOUNT};
use crate::Pallet as Assets;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::prelude::*;

benchmarks! {
	transfer {
		let caller: T::AccountId = whitelisted_caller();
		// T::Currency::mint_into(T::AssetId::from(A), &caller, amount)
	}: _(RawOrigin::Signed(caller), ASSET_ID, TO_ACCOUNT, TRANSFER_AMOUNT, true)

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

impl_benchmark_test_suite!(Assets, crate::mock::new_test_ext(), crate::mock::Test,);
