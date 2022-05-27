#![cfg(feature = "runtime-benchmarks")]

#[cfg(test)]
use crate::Pallet as Airdrop;
use crate::mocks::{self, AccountId, RelayChainAccountId, ALICE, VESTING_STEP};
use codec::Decode;
use composable_support::validation::Validated;
use composable_traits::bonded_finance::{BondDuration, BondOffer, BondOfferReward};
use crate::models::{Proof, RemoteAccount};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{
	dispatch::UnfilteredDispatchable,
	traits::{fungible::Mutate as _, fungibles::Mutate as _},
};
use frame_system::{Config, Pallet, RawOrigin};
use sp_core::{ed25519, keccak_256, Pair};
use sp_runtime::{AccountId32, traits::One};
use sp_std::prelude::*;

benchmarks! {
  where_clause {
	  where
		T: frame_system::Config<BlockNumber = BlockNumber>,
	  T: Config<Balance = Balance, RelayChainAccountId = RelayChainAccountId, AccountId = AccountId>,
  }

	create_airdrop {
        let x in 100..1000;
	}: _(RawOrigin::Signed(ALICE), None, VESTING_STEP)

	add_recipient {
		let x in 100..1000;
		let accounts = mocks::generate_accounts(x as _).into_iter().map(|(_, a)| (a.as_remote_public(), 1_000_000_000_000, false)).collect();
		let airdrop_id = T::AirdropId::one();
		<Airdrop<T> as AirdropManagement>::create_airdrop(RawOrigin::Signed(ALICE), None, VESTING_STEP);
	}: _(RawOrigin::Signed(ALICE), airdrop_id, accounts)

	remove_recipient {
		let x in 100..1000;
		let accounts = mocks::generate_accounts(x as _).into_iter().map(|(_, a)| (a.as_remote_public(), 1_000_000_000_000, false)).collect();
		let airdrop_id = T::AirdropId::one();
		<Airdrop<T> as AirdropManagement>::create_airdrop(RawOrigin::Signed(ALICE), None, VESTING_STEP);
		<Airdrop<T> as AirdropManagement>::add_recipient(RawOrigin::Signed(ALICE), airdrop_id, accounts_creation);
	}: _(RawOrigin::Signed(ALICE), airdrop_id, accounts[0 as usize].0)

	enable_airdrop {
		let x in 100..1000;
		let accounts = mocks::generate_accounts(x as _).into_iter().map(|(_, a)| (a.as_remote_public(), 1_000_000_000_000, false)).collect();
		let airdrop_id = T::AirdropId::one();
		<Airdrop<T> as AirdropManagement>::create_airdrop(RawOrigin::Signed(ALICE), None, VESTING_STEP);
		<Airdrop<T> as AirdropManagement>::add_recipient(RawOrigin::Signed(ALICE), airdrop_id, accounts);
	}: _(RawOrigin::Signed(ALICE), airdrop_id)

	disable_airdrop {
		let x in 100..1000;
		let accounts = mocks::generate_accounts(x as _).into_iter().map(|(_, a)| (a.as_remote_public(), 1_000_000_000_000, false)).collect();
		let airdrop_id = T::AirdropId::one();
		<Airdrop<T> as AirdropManagement>::create_airdrop(RawOrigin::Signed(ALICE), None, VESTING_STEP);
		<Airdrop<T> as AirdropManagement>::add_recipient(RawOrigin::Signed(ALICE), airdrop_id, accounts_creation);
	}: _(RawOrigin::Signed(ALICE), airdrop_id)

	claim {
		let x in 100..1000;
		let accounts = mocks::generate_accounts(x as _);
		let remote_accounts = local_accounts.clone().into_iter().map(|(_, a)| (a.as_remote_public(), 1_000_000_000_000, false)).collect();
		let airdrop_id = T::AirdropId::one();
		<Airdrop<T> as AirdropManagement>::create_airdrop(RawOrigin::Signed(ALICE), None, VESTING_STEP);
		<Airdrop<T> as AirdropManagement>::add_recipient(RawOrigin::Signed(ALICE), airdrop_id, accounts_creation);
		frame_system::Pallet::<T>::set_block_number(VESTING_STEP);
	}: _(RawOrigin::None, airdrop_id, accounts[0 as usize].0.clone(), accounts[0 as usize].1.clone().proof(accounts[0 as usize].0.clone()))

    impl_benchmark_test_suite!(Airdrop, crate::mocks::ExtBuilder::default().build(), crate::mocks::MockRuntime)
}

