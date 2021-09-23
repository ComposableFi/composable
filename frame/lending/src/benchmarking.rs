use super::*;

use crate::Pallet as Lending;
use composable_traits::{
	lending::MarketConfigInput, rate_model::NormalizedCollateralFactor, vault::Vault,
};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::traits::fungibles::Mutate;
use frame_system::{EventRecord, RawOrigin};
use sp_runtime::{FixedPointNumber, Perquintill};
use sp_std::prelude::*;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::Event = generic_event.into();
	// compare to the last event record
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

const BTC: u128 = 1000;
const USDT: u128 = 2000;

fn set_prices<T: Config>() {
	pallet_oracle::Prices::<T>::insert(
		<T as pallet_oracle::Config>::AssetId::from(BTC),
		pallet_oracle::Price {
			price: <T as pallet_oracle::Config>::PriceValue::from(48_000u64),
			block: 0u32.into(),
		},
	);
	pallet_oracle::Prices::<T>::insert(
		<T as pallet_oracle::Config>::AssetId::from(USDT),
		pallet_oracle::Price {
			price: <T as pallet_oracle::Config>::PriceValue::from(1u64),
			block: 0u32.into(),
		},
	);
}

fn create_market<T: Config>(manager: T::AccountId) -> (crate::MarketIndex, <T as Config>::VaultId) {
	let market_config = MarketConfigInput {
		manager,
		reserved: Perquintill::from_percent(10),
		collateral_factor: NormalizedCollateralFactor::saturating_from_rational(200, 100),
	};
	Lending::<T>::create(
		<T as Config>::AssetId::from(BTC),
		<T as Config>::AssetId::from(USDT),
		market_config,
	)
	.unwrap()
}

benchmarks! {
	create_new_market {
		let caller: T::AccountId = whitelisted_caller();
		let borrow_asset_id = <T as Config>::AssetId::from(BTC);
		let collateral_asset_id = <T as Config>::AssetId::from(USDT);
		let reserved_factor = Perquintill::from_percent(10);
		let collateral_factor = NormalizedCollateralFactor::saturating_from_rational(200, 100);
		let market_id = MarketIndex::new(1);
		let vault_id = 1u64.into();
		set_prices::<T>();
	}: _(RawOrigin::Signed(caller.clone()), borrow_asset_id, collateral_asset_id, reserved_factor, collateral_factor)
	verify {
		assert_last_event::<T>(Event::NewMarketCreated {
			market_id,
			vault_id,
			manager: caller,
			borrow_asset_id,
			collateral_asset_id,
			reserved_factor,
			collateral_factor,
		}.into())
	}

	deposit_collateral {
		let caller: T::AccountId = whitelisted_caller();
		let market: MarketIndex = MarketIndex::new(1u32);
		let amount: T::Balance = 1_000_000u64.into();
		set_prices::<T>();
		let _ = create_market::<T>(caller.clone());
		<T as pallet::Config>::Currency::mint_into(USDT.into(), &caller, amount).unwrap();
	}: _(RawOrigin::Signed(caller.clone()), market, amount)
	verify {
		assert_last_event::<T>(Event::CollateralDeposited {
			sender: caller,
			market_id: market,
			amount,
		}.into())
	}

	withdraw_collateral {
		let caller: T::AccountId = whitelisted_caller();
		let market: MarketIndex = MarketIndex::new(1u32);
		let amount: T::Balance = 1_000_000u64.into();
		set_prices::<T>();
		let (market, _vault_id) = create_market::<T>(caller.clone());
		<T as pallet::Config>::Currency::mint_into(USDT.into(), &caller, amount).unwrap();
		Lending::<T>::deposit_collateral_internal(&market, &caller, amount).unwrap();
	}: _(RawOrigin::Signed(caller.clone()), market, amount)
	verify {
		assert_last_event::<T>(Event::CollateralWithdrawed {
			sender: caller,
			market_id: market,
			amount
		}.into())
	}

	borrow {
		let caller: T::AccountId = whitelisted_caller();
		let amount_to_borrow = 1_000_000u64.into();
		set_prices::<T>();
		let (market_id, vault_id) = create_market::<T>(caller.clone());
		<T as pallet::Config>::Currency::mint_into(USDT.into(), &caller, amount_to_borrow * 6u64.into()).unwrap();
		<T as pallet::Config>::Currency::mint_into(BTC.into(), &caller, amount_to_borrow * 6u64.into()).unwrap();
		Lending::<T>::deposit_collateral_internal(&market_id, &caller, amount_to_borrow * 2u64.into()).unwrap();
		<T as pallet::Config>::Vault::deposit(&vault_id, &caller, amount_to_borrow * 2u64.into()).unwrap();
	}: _(RawOrigin::Signed(caller.clone()), market_id, amount_to_borrow)
	verify {
		assert_last_event::<T>(Event::Borrowed {
			sender: caller,
			market_id,
			amount: amount_to_borrow
		}.into())
	}

	repay_borrow {
		let caller: T::AccountId = whitelisted_caller();
		set_prices::<T>();
		let (market_id, vault_id) = create_market::<T>(caller.clone());
		let repay_amount = 1_000_000u64.into();
		<T as pallet::Config>::Currency::mint_into(USDT.into(), &caller, repay_amount * 6u64.into()).unwrap();
		<T as pallet::Config>::Currency::mint_into(BTC.into(), &caller, repay_amount * 6u64.into()).unwrap();
		Lending::<T>::deposit_collateral_internal(&market_id, &caller, repay_amount * 2u64.into()).unwrap();
		<T as pallet::Config>::Vault::deposit(&vault_id, &caller, repay_amount * 2u64.into()).unwrap();
		Lending::<T>::borrow_internal(&market_id, &caller, repay_amount).unwrap();
		crate::LastBlockTimestamp::<T>::put(6);
	}: _(RawOrigin::Signed(caller.clone()), market_id, caller.clone(), repay_amount)
	verify {
		assert_last_event::<T>(
			Event::RepaidBorrow {
				sender: caller.clone(),
				market_id,
				beneficiary: caller,
				amount: repay_amount,
			}.into()
		)
	}
}

impl_benchmark_test_suite!(Lending, crate::mock::new_test_ext(), crate::mock::Test,);
