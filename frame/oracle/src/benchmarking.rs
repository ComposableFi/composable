use super::*;

#[allow(unused)]
use crate::Pallet as Oracle;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{
	assert_ok,
	traits::{Currency, EnsureOrigin, Get},
};
use frame_system::{EventRecord, RawOrigin};
use sp_runtime::{DispatchResult, Percent};
use sp_std::prelude::*;

// pub type BalanceOf<T> =
//     <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::Event = generic_event.into();
	// compare to the last event record
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

macro_rules! whitelist {
	($acc:ident) => {
		frame_benchmarking::benchmarking::add_to_whitelist(
			frame_system::Account::<T>::hashed_key_for(&$acc).into(),
		);
	};
}

benchmarks! {
	add_asset_and_info {
		let caller = T::AddOracle::successful_origin();
		let asset_id = 1;
		let threshold = Percent::from_percent(80);
		let min_answers = 3;
		let max_answers = 5;
	}: {
		assert_ok!(
			<Oracle<T>>::add_asset_and_info(caller, asset_id, threshold, min_answers, max_answers)
		);
	}
	verify {
		assert_last_event::<T>(Event::AssetInfoChange(asset_id, threshold, min_answers, max_answers).into());
	}

	request_price {
		let caller: T::AccountId = whitelisted_caller();
		let asset_id = 1;
		AssetsInfo::<T>::insert(asset_id, AssetInfo {
			threshold: Percent::from_percent(80),
			min_answers: 3,
			max_answers: 5,
		});
		AssetsCount::<T>::mutate(|a| *a += 1);
		T::Currency::make_free_balance_be(&caller, T::RequestCost::get() + T::Currency::minimum_balance());
	}: _(RawOrigin::Signed(caller.clone()), asset_id)
	verify {
		assert_last_event::<T>(Event::PriceRequested(caller, asset_id).into())
	}

	set_signer {
		let caller: T::AccountId = whitelisted_caller();
		let signer: T::AccountId = account("candidate", 0, SEED);
		whitelist!(signer);
		let stake = T::MinStake::get();
		T::Currency::make_free_balance_be(&caller, stake + T::Currency::minimum_balance());
	}: _(RawOrigin::Signed(caller.clone()), signer.clone())
	verify {
		assert_last_event::<T>(Event::StakeAdded(signer, stake.clone(), stake).into());
	}

	add_stake {
		let caller: T::AccountId = whitelisted_caller();
		let stake = T::MinStake::get();
		T::Currency::make_free_balance_be(&caller, stake * 2u32.into());
		let signer: T::AccountId = account("candidate", 0, SEED);
		ControllerToSigner::<T>::insert(&caller, signer.clone());
	}: _(RawOrigin::Signed(caller.clone()), stake)
	verify {
		assert_last_event::<T>(Event::StakeAdded(signer, stake, stake).into())
	}

	remove_stake {
		let signer: T::AccountId = account("candidate", 0, SEED);
		let stake = T::MinStake::get();
		ControllerToSigner::<T>::insert(&signer, signer.clone());
		OracleStake::<T>::insert(&signer, stake);
		let unlock_block = frame_system::Pallet::<T>::block_number() + T::StakeLock::get() + 1u32.into();
	}: _(RawOrigin::Signed(signer.clone()))
	verify {
		assert_last_event::<T>(Event::StakeRemoved(signer, stake, unlock_block).into())
	}

	reclaim_stake {
		let signer: T::AccountId = account("candidate", 0, SEED);
		let stake = T::MinStake::get();
		ControllerToSigner::<T>::insert(&signer, signer.clone());
		OracleStake::<T>::insert(&signer, stake);
		let unlock_block = frame_system::Pallet::<T>::block_number();
		DeclaredWithdraws::<T>::insert(&signer, Withdraw { stake, unlock_block });
	}: _(RawOrigin::Signed(signer.clone()))
	verify {
		assert_last_event::<T>(Event::StakeReclaimed(signer, stake).into())
	}

	submit_price {
		let p in 1 .. T::MaxAnswerBound::get();
		let p = p - 1; // We will submit a new price now, then the number of prices will equal T::MaxAnswerBound::get().
		let price_submitters = (0..p).map(|c| account("candidate", c, SEED)).collect::<Vec<T::AccountId>>();
		let caller: T::AccountId = whitelisted_caller();
		let price = 100_000;
		let asset_id = 1;
		let stake = T::MinStake::get();
		OracleStake::<T>::insert(&caller, stake);
		RequestedId::<T>::mutate(asset_id, |request_id| *request_id += 1);
		Requested::<T>::insert(asset_id, true);
		AssetsInfo::<T>::insert(asset_id, AssetInfo {
			threshold: Percent::from_percent(80),
			min_answers: 1,
			max_answers: T::MaxAnswerBound::get(),
		});
		PrePrices::<T>::mutate(asset_id, |current_prices| -> DispatchResult {
			for (i, price_submitter) in price_submitters.iter().enumerate() {
				let set_price = PrePrice {
					price: price + i as u64,
					block: frame_system::Pallet::<T>::block_number(),
					who: price_submitter.clone(),
				};
				current_prices.push(set_price);
			}
			Ok(())
		})?;
	}: _(RawOrigin::Signed(caller.clone()), price, asset_id)
	verify {
		assert_last_event::<T>(Event::PriceSubmitted(caller, asset_id, price).into())
	}

}

impl_benchmark_test_suite!(Oracle, crate::mock::new_test_ext(), crate::mock::Test,);
