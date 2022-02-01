use super::*;

use crate::{decay::*, Pallet as Mosaic};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{
	assert_ok,
	traits::{fungibles::Mutate, Get},
};
use frame_system::RawOrigin;

const MAX_TRANSFER_SIZE: u128 = 100_000_000_000_000_000;
const BUDGET_AMOUNT: u128 = 100_000_000_000_000_000_000;
const TRANSFER_AMOUNT: u128 = 100_000_000_000_000;

benchmarks! {
	where_clause {
		where T::BlockNumber: From<u32>, T::NetworkId: From<u32>, BalanceOf<T>: From<u128>, AssetIdOf<T>: From<u128>,
		  T::BudgetPenaltyDecayer: From<BudgetPenaltyDecayer<BalanceOf<T>, T::BlockNumber>>
	}

	set_relayer {
		let relayer = whitelisted_caller();
	}: _(RawOrigin::Root, relayer)

	rotate_relayer {
		let relayer: T::AccountId = whitelisted_caller();
	  assert_ok!(Mosaic::<T>::set_relayer(RawOrigin::Root.into(), relayer.clone()));

		let new_relayer = account("new_relayer", 0, 0);
	}: _(RawOrigin::Signed(relayer), new_relayer, 42.into())

	set_network {
		let relayer: T::AccountId = whitelisted_caller();
	  assert_ok!(Mosaic::<T>::set_relayer(RawOrigin::Root.into(), relayer.clone()));

		let network_id: T::NetworkId = 1.into();
		let network_info = NetworkInfo {
			enabled: true,
			max_transfer_size: MAX_TRANSFER_SIZE.into(),
		};

	}: _(RawOrigin::Signed(relayer), network_id, network_info)

	set_budget {
		let asset_id: AssetIdOf<T> = 1.into();
		let amount: BalanceOf<T> = BUDGET_AMOUNT.into();
		let decayer: BudgetPenaltyDecayer<BalanceOf<T>, T::BlockNumber> =
	  BudgetPenaltyDecayer::linear(5.into());
	}: _(RawOrigin::Root, asset_id, amount, decayer.into())

  transfer_to {
		let relayer: T::AccountId = whitelisted_caller();
	  assert_ok!(Mosaic::<T>::set_relayer(RawOrigin::Root.into(), relayer.clone()));

		let network_id: T::NetworkId = 1.into();
		let network_info = NetworkInfo {
			enabled: true,
			max_transfer_size: MAX_TRANSFER_SIZE.into(),
		};
	  assert_ok!(Mosaic::<T>::set_network(RawOrigin::Signed(relayer).into(), network_id.clone(), network_info));

		let asset_id: AssetIdOf<T> = 1.into();
		let budget_amount: BalanceOf<T> =  BUDGET_AMOUNT.into();
		let decayer: BudgetPenaltyDecayer<BalanceOf<T>, T::BlockNumber> =
		  BudgetPenaltyDecayer::linear(5.into());
	  assert_ok!(Mosaic::<T>::set_budget(RawOrigin::Root.into(), asset_id, budget_amount, decayer.into()));

		let alice = account("alice", 0, 0);
	  let address = [0u8; 20];
		let transfer_amount: BalanceOf<T> = TRANSFER_AMOUNT.into();

		assert_ok!(T::Assets::mint_into(asset_id, &alice, transfer_amount));
  }: _(RawOrigin::Signed(alice), network_id, asset_id, address, transfer_amount, true)

  accept_transfer {
		let relayer: T::AccountId = whitelisted_caller();
	  assert_ok!(Mosaic::<T>::set_relayer(RawOrigin::Root.into(), relayer.clone()));

		let network_id: T::NetworkId = 1.into();
		let network_info = NetworkInfo {
			enabled: true,
			max_transfer_size: MAX_TRANSFER_SIZE.into(),
		};
	  assert_ok!(Mosaic::<T>::set_network(RawOrigin::Signed(relayer.clone()).into(), network_id.clone(), network_info));

		let asset_id: AssetIdOf<T> = 1.into();
		let budget_amount: BalanceOf<T> =  BUDGET_AMOUNT.into();
		let decayer: BudgetPenaltyDecayer<BalanceOf<T>, T::BlockNumber> =
		  BudgetPenaltyDecayer::linear(5.into());
	  assert_ok!(Mosaic::<T>::set_budget(RawOrigin::Root.into(), asset_id, budget_amount, decayer.into()));

		let alice = account("alice", 0, 0);
	  let address = [0u8; 20];
		let transfer_amount: BalanceOf<T> = TRANSFER_AMOUNT.into();

		assert_ok!(T::Assets::mint_into(asset_id, &alice, transfer_amount));
	  assert_ok!(Mosaic::<T>::transfer_to(RawOrigin::Signed(alice.clone()).into(), network_id, asset_id, address, transfer_amount, true));
  }: _(RawOrigin::Signed(relayer), alice, asset_id, transfer_amount)

  claim_stale_to {
		let relayer: T::AccountId = whitelisted_caller();
	  assert_ok!(Mosaic::<T>::set_relayer(RawOrigin::Root.into(), relayer.clone()));

		let network_id: T::NetworkId = 1.into();
		let network_info = NetworkInfo {
			enabled: true,
			max_transfer_size: MAX_TRANSFER_SIZE.into(),
		};
	  assert_ok!(Mosaic::<T>::set_network(RawOrigin::Signed(relayer.clone()).into(), network_id.clone(), network_info));

		let asset_id: AssetIdOf<T> = 1.into();
		let budget_amount: BalanceOf<T> =  BUDGET_AMOUNT.into();
		let decayer: BudgetPenaltyDecayer<BalanceOf<T>, T::BlockNumber> =
		  BudgetPenaltyDecayer::linear(5.into());
	  assert_ok!(Mosaic::<T>::set_budget(RawOrigin::Root.into(), asset_id, budget_amount, decayer.into()));

		let alice = account("alice", 0, 0);
	  let address = [0u8; 20];
		let transfer_amount: BalanceOf<T> = TRANSFER_AMOUNT.into();

		assert_ok!(T::Assets::mint_into(asset_id, &alice, transfer_amount));
	  assert_ok!(Mosaic::<T>::transfer_to(RawOrigin::Signed(alice.clone()).into(), network_id, asset_id, address, transfer_amount, true));
		frame_system::Pallet::<T>::set_block_number(T::MinimumTimeLockPeriod::get() + 1.into());
  }: _(RawOrigin::Signed(alice.clone()), asset_id, alice.clone())

  timelocked_mint {
		let relayer: T::AccountId = whitelisted_caller();
	  assert_ok!(Mosaic::<T>::set_relayer(RawOrigin::Root.into(), relayer.clone()));

		let network_id: T::NetworkId = 1.into();
		let network_info = NetworkInfo {
			enabled: true,
			max_transfer_size: MAX_TRANSFER_SIZE.into(),
		};
	  assert_ok!(Mosaic::<T>::set_network(RawOrigin::Signed(relayer.clone()).into(), network_id.clone(), network_info));

		let asset_id: AssetIdOf<T> = 1.into();
		let budget_amount: BalanceOf<T> = BUDGET_AMOUNT.into();
		let decayer: BudgetPenaltyDecayer<BalanceOf<T>, T::BlockNumber> =
		  BudgetPenaltyDecayer::linear(5.into());
	  assert_ok!(Mosaic::<T>::set_budget(RawOrigin::Root.into(), asset_id, budget_amount, decayer.into()));

		let alice = account("alice", 0, 0);
	  let address = [0u8; 20];
		let transfer_amount: BalanceOf<T> = TRANSFER_AMOUNT.into();

		assert_ok!(T::Assets::mint_into(asset_id, &alice, transfer_amount));

	  assert_ok!(Mosaic::<T>::transfer_to(RawOrigin::Signed(alice.clone()).into(), network_id.clone(), asset_id, address, transfer_amount, true));
	  assert_ok!(Mosaic::<T>::accept_transfer(RawOrigin::Signed(relayer.clone()).into(), alice.clone(), asset_id, transfer_amount));
	  let current_block = frame_system::Pallet::<T>::block_number();
	  let tx_id = generate_id::<T>(&alice, &network_id, &asset_id, &address, &transfer_amount, &current_block);
  }: _(RawOrigin::Signed(relayer), asset_id, alice.clone(), transfer_amount, T::MinimumTimeLockPeriod::get(), tx_id)

  set_timelock_duration {
  }: _(RawOrigin::Root, 100.into())

  rescind_timelocked_mint {
		let relayer: T::AccountId = whitelisted_caller();
	  assert_ok!(Mosaic::<T>::set_relayer(RawOrigin::Root.into(), relayer.clone()));

		let network_id: T::NetworkId = 1.into();
		let network_info = NetworkInfo {
			enabled: true,
			max_transfer_size: MAX_TRANSFER_SIZE.into(),
		};
	  assert_ok!(Mosaic::<T>::set_network(RawOrigin::Signed(relayer.clone()).into(), network_id.clone(), network_info));

		let asset_id: AssetIdOf<T> = 1.into();
		let budget_amount: BalanceOf<T> =  BUDGET_AMOUNT.into();
		let decayer: BudgetPenaltyDecayer<BalanceOf<T>, T::BlockNumber> =
		  BudgetPenaltyDecayer::linear(5.into());
	  assert_ok!(Mosaic::<T>::set_budget(RawOrigin::Root.into(), asset_id, budget_amount, decayer.into()));

		let alice = account("alice", 0, 0);
	  let address = [0u8; 20];
		let transfer_amount: BalanceOf<T> = TRANSFER_AMOUNT.into();

		assert_ok!(T::Assets::mint_into(asset_id, &alice, transfer_amount));

	  assert_ok!(Mosaic::<T>::transfer_to(RawOrigin::Signed(alice.clone()).into(), network_id.clone(), asset_id, address, transfer_amount, true));
	  assert_ok!(Mosaic::<T>::accept_transfer(RawOrigin::Signed(relayer.clone()).into(), alice.clone(), asset_id, transfer_amount));
	  let current_block = frame_system::Pallet::<T>::block_number();
	  let tx_id = generate_id::<T>(&alice, &network_id, &asset_id, &address, &transfer_amount, &current_block);

	  assert_ok!(Mosaic::<T>::timelocked_mint(RawOrigin::Signed(relayer.clone()).into(), asset_id, alice.clone(), transfer_amount, T::MinimumTimeLockPeriod::get(), tx_id));
  }: _(RawOrigin::Signed(relayer), asset_id, alice.clone(), transfer_amount)


	claim_to {
		let relayer: T::AccountId = whitelisted_caller();
	  assert_ok!(Mosaic::<T>::set_relayer(RawOrigin::Root.into(), relayer.clone()));

		let network_id: T::NetworkId = 1.into();
		let network_info = NetworkInfo {
			enabled: true,
			max_transfer_size: MAX_TRANSFER_SIZE.into(),
		};
	  assert_ok!(Mosaic::<T>::set_network(RawOrigin::Signed(relayer.clone()).into(), network_id.clone(), network_info));

		let asset_id: AssetIdOf<T> = 1.into();
		let budget_amount: BalanceOf<T> = BUDGET_AMOUNT.into();
		let decayer: BudgetPenaltyDecayer<BalanceOf<T>, T::BlockNumber> =
		  BudgetPenaltyDecayer::linear(5.into());
	  assert_ok!(Mosaic::<T>::set_budget(RawOrigin::Root.into(), asset_id, budget_amount, decayer.into()));

		let alice = account("alice", 0, 0);
		let address = [0u8; 20];
		let transfer_amount: BalanceOf<T> = TRANSFER_AMOUNT.into();

		assert_ok!(T::Assets::mint_into(asset_id, &alice, transfer_amount));

	  assert_ok!(Mosaic::<T>::transfer_to(RawOrigin::Signed(alice.clone()).into(), network_id.clone(), asset_id, address, transfer_amount, true));
	  assert_ok!(Mosaic::<T>::accept_transfer(RawOrigin::Signed(relayer.clone()).into(), alice.clone(), asset_id, transfer_amount));
	  let current_block = frame_system::Pallet::<T>::block_number();
	  let tx_id = generate_id::<T>(&alice, &network_id, &asset_id, &address, &transfer_amount, &current_block);

	  assert_ok!(Mosaic::<T>::timelocked_mint(RawOrigin::Signed(relayer.clone()).into(), asset_id, alice.clone(), transfer_amount, T::MinimumTimeLockPeriod::get(), tx_id));
		frame_system::Pallet::<T>::set_block_number(T::MinimumTimeLockPeriod::get() + 1.into());
	}: _(RawOrigin::Signed(alice.clone()), asset_id, alice.clone())
}

impl_benchmark_test_suite!(Mosaic, crate::mock::new_test_ext(), crate::mock::Test,);
