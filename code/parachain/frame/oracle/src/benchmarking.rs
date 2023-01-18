use super::*;

use crate::validation::ValidBlockInterval;
#[allow(unused)]
use crate::Pallet as Oracle;
use composable_support::validation::Validated;
use composable_traits::oracle::Price;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{
	assert_ok,
	traits::{Currency, Get},
	BoundedVec,
};
use frame_system::{EventRecord, RawOrigin};
use sp_runtime::{DispatchResult, Percent};

use sp_std::{prelude::*, vec};

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::RuntimeEvent = generic_event.into();
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
		let asset_id = 1;
		let threshold = Validated::new(Percent::from_percent(80)).unwrap();
		let min_answers = Validated::new(3).unwrap();
		let max_answers = Validated::new(5).unwrap();
		let block_interval = Validated::<T::BlockNumber, ValidBlockInterval<T::StalePrice>>::new(T::StalePrice::get() + 1u32.into()).unwrap();
		let reward: BalanceOf<T> = T::Currency::minimum_balance();
		let slash: BalanceOf<T> = T::Currency::minimum_balance();
		let emit_price_changes: bool = false;
	}: {
		assert_ok!(
			<Oracle<T>>::add_asset_and_info(RawOrigin::Root.into(), asset_id.into(), threshold, min_answers, max_answers, block_interval, reward, slash, emit_price_changes)
		);
	}
	verify {
		assert_last_event::<T>(Event::AssetInfoChange(asset_id.into(), *threshold, *min_answers, *max_answers, *block_interval, reward, slash).into());
	}

	adjust_rewards {
		let annual_cost_per_oracle: T::Balance = 100_000_u128.into();
		let num_ideal_oracles: u8 = 10;
	}: _(RawOrigin::Root, annual_cost_per_oracle, num_ideal_oracles)

	set_signer {
		let caller: T::AccountId = whitelisted_caller();
		let signer: T::AccountId = account("candidate", 0, SEED);
		whitelist!(signer);
		let stake = T::MinStake::get();
		T::Currency::make_free_balance_be(&caller, stake + T::Currency::minimum_balance());
	}: _(RawOrigin::Signed(caller.clone()), signer.clone())
	verify {
		assert_last_event::<T>(Event::SignerSet(signer, caller).into());
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
		let asset_id: T::AssetId = 1.into();
		let stake = T::MinStake::get();
		OracleStake::<T>::insert(&caller, stake);
		AssetsInfo::<T>::insert(asset_id, AssetInfo {
			threshold: Percent::from_percent(80),
			min_answers: 1,
			max_answers: T::MaxAnswerBound::get(),
			block_interval: 0u32.into(),
			reward_weight: T::Currency::minimum_balance(),
			slash: T::Currency::minimum_balance(),
			emit_price_changes: false,
		});
		frame_system::Pallet::<T>::set_block_number(6u32.into());
		PrePrices::<T>::mutate(asset_id, |current_prices| -> DispatchResult {
			for (i, price_submitter) in price_submitters.iter().enumerate() {
				let set_price = PrePrice {
					price: (price + i as u128).into(),
					block: frame_system::Pallet::<T>::block_number(),
					who: price_submitter.clone(),
				};
				current_prices.try_push(set_price).unwrap();
			}
			Ok(())
		})?;
	}: _(RawOrigin::Signed(caller.clone()), price.into(), asset_id)
	verify {
		assert_last_event::<T>(Event::PriceSubmitted(caller, asset_id, price.into()).into())
	}

	update_pre_prices {
		let p in 1 .. T::MaxAnswerBound::get();
		let who: T::AccountId = whitelisted_caller();
		let asset_id: T::AssetId = 1.into();
		let block = T::StalePrice::get();
		let asset_info = AssetInfo {
			threshold: Percent::from_percent(80),
			min_answers: 1,
			max_answers: p,
			block_interval: T::StalePrice::get(),
			reward_weight: T::Currency::minimum_balance(),
			slash: T::Currency::minimum_balance(),
			emit_price_changes: false,
		};
		let pre_prices = (0..p).map(|i| {
			PrePrice {
				price: (100u128 + i as u128).into(),
				block: 0u32.into(),
				who: who.clone()
			}
		})
		.collect::<Vec<_>>();
		PrePrices::<T>::insert(asset_id, BoundedVec::try_from(pre_prices).unwrap());
	}: {
		Oracle::<T>::update_pre_prices(asset_id, &asset_info, block)
	}

	update_price {
		let p in 1 .. T::MaxAnswerBound::get();
		let who: T::AccountId = whitelisted_caller();
		let asset_id: T::AssetId =  T::AssetId::from(1u128);
		let block = T::StalePrice::get();
		let asset_info = AssetInfo {
			threshold: Percent::from_percent(80),
			min_answers: 1,
			max_answers: p,
			block_interval: T::StalePrice::get(),
			reward_weight: T::Currency::minimum_balance(),
			slash: T::Currency::minimum_balance(),
			emit_price_changes: false,
		};
		let pre_prices = (0..p).map(|_| {
			PrePrice {
				price: (100u128 + p as u128).into(),
				block: 0u32.into(),
				who: who.clone()
			}
		})
		.collect::<Vec<_>>();
		// the worst scenario is when we need to remove a price first so gonna need to fill the price history
		let price = Price { price: 100u32.into(), block };
		let historic_prices = BoundedVec::try_from(vec![price; T::MaxHistory::get() as usize]).unwrap();
		PriceHistory::<T>::insert(asset_id, historic_prices);
	}: {
		Oracle::<T>::update_price(asset_id, asset_info.into(), block, pre_prices)
	}
}

impl_benchmark_test_suite!(Oracle, crate::mock::new_test_ext(), crate::mock::Test,);
