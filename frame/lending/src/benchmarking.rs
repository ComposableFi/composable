use super::*;

#[allow(unused)]
use crate::Pallet as Lending;
use composable_traits::{lending::MarketConfigInput, rate_model::NormalizedCollateralFactor};
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
	deposit_collateral {
		let caller: T::AccountId = whitelisted_caller();
		let market: MarketIndex = MarketIndex::new(1u32);
		let amount: T::Balance = 1_000_000u64.into();
		set_prices::<T>();
		let _ = create_market::<T>(caller.clone());
		<T as pallet::Config>::Currency::mint_into(USDT.into(), &caller, amount).unwrap();
	}: _(RawOrigin::Signed(caller.clone()), market, amount)
	verify {
		assert_last_event::<T>(Event::CollateralDeposited(caller, market, amount).into())
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
		assert_last_event::<T>(Event::CollateralWithdrawed(caller, market, amount).into())
	}
}

impl_benchmark_test_suite!(Lending, crate::mock::new_test_ext(), crate::mock::Test,);
