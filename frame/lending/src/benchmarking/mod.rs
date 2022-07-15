//! Benchmarks and sanity tests for lending. Only test that action do not error, not that produce
//! positive side effects
#![warn(unused_imports)]
use self::currency::NORMALIZED;

use super::*;
use crate::{self as pallet_lending, Pallet as Lending};
use composable_traits::{
	defi::{CurrencyPair, DeFiComposableConfig},
	lending::{CreateInput, Lending as LendingTrait, RepayStrategy},
	vault::StrategicVault,
};
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::{
	traits::{fungible, fungibles::Mutate, Get},
	BoundedVec,
};
use frame_system::RawOrigin;
use setup::*;
use sp_std::prelude::*;
type BalanceOf<T> = <T as DeFiComposableConfig>::Balance;

mod setup;

/// Create a market with the given origin and input.
///
/// NOTE: ***ONLY CALL THIS ONCE PER BENCHMARK!!!*** The [`MarketIndex`] returned is always `1`.
///
/// TODO: Get market index from events, to allow for calling twice in one benchmark.
fn create_market_from_raw_origin<T: Config>(
	origin: RawOrigin<<T as frame_system::Config>::AccountId>,
	input: CreateInput<
		<T as Config>::LiquidationStrategyId,
		<T as DeFiComposableConfig>::MayBeAssetId,
		<T as frame_system::Config>::BlockNumber,
	>,
) -> MarketIndex {
	Lending::<T>::create_market(origin.clone().into(), input, false).unwrap();

	// FIXME: This ain't ideal
	MarketIndex::new(1)
}

fn lending_benchmarking_setup<T: Config + pallet_oracle::Config>() -> LendingBenchmarkingSetup<T> {
	let caller: <T as frame_system::Config>::AccountId = whitelisted_caller::<T::AccountId>();
	let origin: RawOrigin<<T as frame_system::Config>::AccountId> = whitelisted_origin::<T>();
	// 100k units of normalized asset in the bank to work with
	let bank: BalanceOf<T> = NORMALIZED::units(100_000).into();
	let max_price_age = 10_000_u32.try_into().unwrap_or_default();

	let pair = setup_currency_pair::<T>(&caller, bank);
	let input = create_market_config::<T>(pair.base, pair.quote, max_price_age);

	LendingBenchmarkingSetup { caller, origin, bank, pair, input }
}

pub struct LendingBenchmarkingSetup<T: Config> {
	caller: <T as frame_system::Config>::AccountId,
	origin: RawOrigin<<T as frame_system::Config>::AccountId>,
	bank: BalanceOf<T>,
	pair: CurrencyPair<<T as DeFiComposableConfig>::MayBeAssetId>,
	input: CreateInput<
		<T as Config>::LiquidationStrategyId,
		<T as DeFiComposableConfig>::MayBeAssetId,
		<T as frame_system::Config>::BlockNumber,
	>,
}

benchmarks! {
	where_clause {
		where
			T: pallet_oracle::Config
				+ pallet_lending::Config
				+ DeFiComposableConfig
				+ pallet_balances::Config
				+ frame_system::Config
				+ pallet_timestamp::Config
				+ pallet_vault::Config,
			<T as pallet_balances::Config>::Balance: From<u64>,
			<T as frame_system::Config>::BlockNumber: From<u32>,
			<T as pallet_timestamp::Config>::Moment: From<u64>,
			<T as pallet_vault::Config>::Balance: From<u64>,
	}

	create_market {
		let LendingBenchmarkingSetup {
			caller,
			origin,
			bank,
			pair,
			input,
		} = lending_benchmarking_setup::<T>();
	}: _(origin, input, false)

	deposit_collateral {
		let LendingBenchmarkingSetup {
			caller,
			origin,
			bank,
			pair,
			input,
		} = lending_benchmarking_setup::<T>();

		let amount: BalanceOf<T> = 1_000_000_u64.into();

		let market_id = create_market_from_raw_origin::<T>(origin.clone(), input);
	}: _(origin, market_id,amount, false)

	withdraw_collateral {
		let LendingBenchmarkingSetup {
			caller,
			origin,
			bank,
			pair,
			input,
		} = lending_benchmarking_setup::<T>();

		let amount: BalanceOf<T> = 1_000_000_u64.into();
		let part: BalanceOf<T> = 1_000_u64.into();

		let market_id = create_market_from_raw_origin::<T>(origin.clone(), input);

		Lending::<T>::deposit_collateral(origin.clone().into(), market_id, amount, false).unwrap();
	}: _(origin, market_id, part)

	borrow {
		let LendingBenchmarkingSetup {
			caller,
			origin,
			bank,
			pair,
			input,
		} = lending_benchmarking_setup::<T>();

		let amount: BalanceOf<T> = 1_000_000_u64.into();
		let part: BalanceOf<T> = 1_000_u64.into();

		<pallet_balances::Pallet::<T> as fungible::Mutate<T::AccountId>>::mint_into(&caller, 10_000_000_000_000_u64.into()).unwrap();

		let market_id = create_market_from_raw_origin::<T>(origin.clone(), input);

		<pallet_balances::Pallet::<T> as fungible::Mutate<T::AccountId>>::mint_into(&Lending::<T>::account_id(&market_id), 10_000_000_000_000_u64.into()).unwrap();

		Lending::<T>::deposit_collateral(origin.clone().into(), market_id, amount, false).unwrap();
	}: _(origin, market_id, part)

	repay_borrow {
		let LendingBenchmarkingSetup {
			caller,
			origin,
			bank,
			pair,
			input,
		} = lending_benchmarking_setup::<T>();

		let amount: BalanceOf<T> = 1_000_000_000_u64.into();
		let part: BalanceOf<T> = 1_000_u64.into();

		let borrow_amount: BalanceOf<T> = 1_000_000_000_u64.into();

		<pallet_balances::Pallet::<T> as fungible::Mutate<T::AccountId>>::mint_into(&caller, 10_000_000_000_000_u64.into()).unwrap();

		let market_id = create_market_from_raw_origin::<T>(origin.clone(), input);

		<pallet_balances::Pallet::<T> as fungible::Mutate<T::AccountId>>::mint_into(&Lending::<T>::account_id(&market_id), 10_000_000_000_000_u64.into()).unwrap();

		Lending::<T>::deposit_collateral(origin.clone().into(), market_id, amount, false).unwrap();
		Lending::<T>::borrow(origin.clone().into(), market_id, part).unwrap();

		produce_block::<T>(42_u32.into(),4200_u64.into());
		produce_block::<T>(43_u32.into(),4300_u64.into());
	}: _(origin, market_id, caller, RepayStrategy::TotalDebt, false)

	liquidate {
		let b in 1..T::MaxLiquidationBatchSize::get();
		let LendingBenchmarkingSetup {
			caller,
			origin,
			bank,
			pair,
			input,
		} = lending_benchmarking_setup::<T>();

		let amount: BalanceOf<T> = 1_000_000_000_u64.into();

		<pallet_balances::Pallet::<T> as fungible::Mutate<T::AccountId>>::mint_into(&caller, 10_000_000_000_000_u64.into()).unwrap();

		let market_id = create_market_from_raw_origin::<T>(origin.clone(), input);

		<pallet_balances::Pallet::<T> as fungible::Mutate<T::AccountId>>::mint_into(&Lending::<T>::account_id(&market_id), 10_000_000_000_000_u64.into()).unwrap();

		let mut borrowers = vec![];
		for i in 0..b {
			let borrower = whitelisted_caller();
			<pallet_balances::Pallet::<T> as fungible::Mutate<T::AccountId>>::mint_into(&borrower, 10_000_000_000_000_u64.into()).unwrap();
			Lending::<T>::deposit_collateral(RawOrigin::Signed(borrower.clone()).into(), market_id, amount, false).unwrap();
			borrowers.push(borrower);
		}
	}: _(origin, market_id, BoundedVec::<_,T::MaxLiquidationBatchSize>::try_from(borrowers).unwrap())

	// HOOKS

	now {}: {
		Lending::<T>::now()
	}

	accrue_interest {
		let x in 1..1000;

		let LendingBenchmarkingSetup {
			caller,
			origin,
			bank,
			pair,
			input,
		} = lending_benchmarking_setup::<T>();

		let amount: BalanceOf<T> = 1_000_000_000_000_u64.into();
		let borrow_amount: BalanceOf<T> = 1_000_000_000_u64.into();
		let part: BalanceOf<T> = 1_000_u64.into();

		<pallet_balances::Pallet::<T> as fungible::Mutate<T::AccountId>>::mint_into(&caller, 10_000_000_000_000_u64.into()).unwrap();

		let market_id = create_market_from_raw_origin::<T>(origin.clone(), input);

		<pallet_balances::Pallet::<T> as fungible::Mutate<T::AccountId>>::mint_into(&Lending::<T>::account_id(&market_id), 10_000_000_000_000_u64.into()).unwrap();

		Lending::<T>::deposit_collateral(origin.clone().into(), market_id, amount, false).unwrap();
		Lending::<T>::borrow(origin.into(), market_id, part).unwrap();

		for block in 0..x {
			produce_block::<T>(block.into(),(block * 100).into());
		}
	}:  {
		Lending::<T>::accrue_interest(&market_id, ((x + 1) * 100).into()).unwrap();
	}

	account_id {
		let LendingBenchmarkingSetup {
			caller,
			origin,
			bank,
			pair,
			input,
		} = lending_benchmarking_setup::<T>();

		let amount: BalanceOf<T> = 1_000_000_000_000_u64.into();
		let part: BalanceOf<T> = 1_000_u64.into();

		<pallet_balances::Pallet::<T> as fungible::Mutate<T::AccountId>>::mint_into(&caller, 10_000_000_000_000_u64.into()).unwrap();

		let market_id = create_market_from_raw_origin::<T>(origin.clone(), input);
	}: {
		// TODO: fix it, make timestamp depend on x increased OR make the value passed be DELTA
		// ^ ???
		Lending::<T>::account_id(&market_id)
	}

	available_funds {
		let LendingBenchmarkingSetup {
			caller,
			origin,
			bank,
			pair,
			input,
		} = lending_benchmarking_setup::<T>();

		let amount: BalanceOf<T> = 1_000_000_000_000_u64.into();
		let part: BalanceOf<T> = 1_000_u64.into();

		<pallet_balances::Pallet::<T> as fungible::Mutate<T::AccountId>>::mint_into(&caller, 10_000_000_000_000_u64.into()).unwrap();

		let market_id = create_market_from_raw_origin::<T>(origin.clone(), input);

		let market_config = Markets::<T>::try_get(market_id).unwrap();
	}:  {
		// TODO: make changes to vault state so something happens
		Lending::<T>::available_funds(&market_config, &caller).unwrap()
	}

	handle_withdrawable {
		let LendingBenchmarkingSetup {
			caller,
			origin,
			bank,
			pair,
			input,
		} = lending_benchmarking_setup::<T>();

		let amount: BalanceOf<T> = 1_000_000_000_000_u64.into();
		let part: BalanceOf<T> = 1_000_u64.into();

		<pallet_balances::Pallet::<T> as fungible::Mutate<T::AccountId>>::mint_into(&caller, 10_000_000_000_000_u64.into()).unwrap();

		let market_id = create_market_from_raw_origin::<T>(origin.clone(), input);

		Lending::<T>::deposit_collateral(origin.into(), market_id, amount, false).unwrap();

		let market_config = Markets::<T>::try_get(market_id).unwrap();
		let account = Lending::<T>::account_id(&market_id);

		<T as Config>::MultiCurrency::mint_into(pair.base, &account, bank).unwrap();
		<T as Config>::MultiCurrency::mint_into(pair.quote, &account, bank).unwrap();
		<T as Config>::Vault::deposit(&market_config.borrow_asset_vault, &account, bank).unwrap();
	}: {
		Lending::<T>::handle_withdrawable(&market_config, &caller, part ).unwrap()
	}

	handle_depositable {
		let LendingBenchmarkingSetup {
			caller,
			origin,
			bank,
			pair,
			input,
		} = lending_benchmarking_setup::<T>();

		let amount: BalanceOf<T> = 1_000_000_000_000_u64.into();
		let part: BalanceOf<T> = 1_000_u64.into();

		<pallet_balances::Pallet::<T> as fungible::Mutate<T::AccountId>>::mint_into(&caller, 10_000_000_000_000_u64.into()).unwrap();

		let market_id = create_market_from_raw_origin::<T>(origin.clone(), input);

		Lending::<T>::deposit_collateral(origin.into(), market_id, amount, false).unwrap();
		let market_config = Markets::<T>::try_get(market_id).unwrap();
		let account = &Lending::<T>::account_id(&market_id);

		<T as Config>::MultiCurrency::mint_into(pair.base, account, bank).unwrap();
		<T as Config>::MultiCurrency::mint_into(pair.quote, account, bank).unwrap();
	}:  {
		// TODO: make it variable with x
		Lending::<T>::handle_depositable(&market_config, &caller, part).unwrap()
	}

	handle_must_liquidate {
		let LendingBenchmarkingSetup {
			caller,
			origin,
			bank,
			pair,
			input,
		} = lending_benchmarking_setup::<T>();

		let amount: BalanceOf<T> = 1_000_000_000_000_u64.into();
		let part: BalanceOf<T> = 1_000_u64.into();

		<pallet_balances::Pallet::<T> as fungible::Mutate<T::AccountId>>::mint_into(&caller, 10_000_000_000_000_u64.into()).unwrap();

		let market_id = create_market_from_raw_origin::<T>(origin.clone(), input);

		Lending::<T>::deposit_collateral(origin.into(), market_id, amount, false).unwrap();

		let market_config = Markets::<T>::try_get(market_id).unwrap();
		let account = &Lending::<T>::account_id(&market_id);

		<T as Config>::MultiCurrency::mint_into(pair.base, account, bank).unwrap();
		<T as Config>::MultiCurrency::mint_into(pair.quote, account, bank).unwrap();
	}:  {
		// TODO: make it variable with x
		Lending::<T>::handle_must_liquidate(&market_config, &caller).unwrap()
	}

	impl_benchmark_test_suite!(Lending, crate::mocks::general::new_test_ext(), crate::mocks::general::Runtime);
}
