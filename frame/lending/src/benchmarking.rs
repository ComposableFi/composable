use super::*;

use crate::Pallet as Lending;
use composable_traits::{
	lending::{
		math::{InterestRateModel, MoreThanOneFixedU128},
		CreateInput, Lending as LendingTrait,
	},
	vault::Vault,
};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::traits::fungibles::Mutate;
use frame_system::{EventRecord, RawOrigin};
use sp_runtime::{FixedPointNumber, Percent, Perquintill};
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

fn set_price<T: Config>(asset_id: u128, price: u64) {
	pallet_oracle::Prices::<T>::insert(
		<T as pallet_oracle::Config>::AssetId::from(asset_id),
		pallet_oracle::Price {
			price: <T as pallet_oracle::Config>::PriceValue::from(price),
			block: 0u32.into(),
		},
	);
}

fn set_prices<T: Config>() {
	set_price::<T>(BTC, 48_000u64);
	set_price::<T>(USDT, 1u64);
}

fn create_market<T: Config>(
	manager: T::AccountId,
	borrow_asset: u128,
	collateral_asset: u128,
) -> (crate::MarketIndex, <T as Config>::VaultId) {
	let market_config = CreateInput {
		liquidator: None,
		manager,
		reserved: Perquintill::from_percent(10),
		collateral_factor: MoreThanOneFixedU128::saturating_from_rational(200, 100),
		under_collaterized_warn_percent: Percent::from_percent(10),
	};
	Lending::<T>::create(
		<T as Config>::AssetId::from(borrow_asset),
		<T as Config>::AssetId::from(collateral_asset),
		market_config,
		&InterestRateModel::default(),
	)
	.unwrap()
}

benchmarks! {
	create_new_market {
		let caller: T::AccountId = whitelisted_caller();
		let borrow_asset_id = <T as Config>::AssetId::from(BTC);
		let collateral_asset_id = <T as Config>::AssetId::from(USDT);
		let reserved_factor = Perquintill::from_percent(10);
		let collateral_factor = MoreThanOneFixedU128::saturating_from_rational(200, 100);
		let under_collaterized_warn_percent = Percent::from_percent(10);
		let market_id = MarketIndex::new(1);
		let vault_id = 1u64.into();
		set_prices::<T>();
	}: _(
		RawOrigin::Signed(caller.clone()),
		borrow_asset_id,
		collateral_asset_id,
		reserved_factor,
		collateral_factor,
		under_collaterized_warn_percent,
		InterestRateModel::default(),
	  None
	)
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
		let _ = create_market::<T>(caller.clone(), BTC, USDT);
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
		let market: MarketIndex = MarketIndex::new(1_u32);
		let amount: T::Balance = 1_000_000_u64.into();
		set_prices::<T>();
		let (market, _vault_id) = create_market::<T>(caller.clone(), BTC, USDT);
		<T as pallet::Config>::Currency::mint_into(USDT.into(), &caller, amount).unwrap();
		Lending::<T>::deposit_collateral_internal(&market, &caller, amount).unwrap();
	}: _(RawOrigin::Signed(caller.clone()), market, amount)
	verify {
		assert_last_event::<T>(Event::CollateralWithdrawn {
			sender: caller,
			market_id: market,
			amount
		}.into())
	}

	borrow {
		let caller: T::AccountId = whitelisted_caller();
		let balance: T::Balance = 1_000_000u64.into();
		set_prices::<T>();
		let (market_id, vault_id) = create_market::<T>(caller.clone(), BTC, USDT);
		<T as pallet::Config>::Currency::mint_into(USDT.into(), &caller, balance * 6u64.into()).unwrap();
		<T as pallet::Config>::Currency::mint_into(BTC.into(), &caller, balance * 6u64.into()).unwrap();
		Lending::<T>::deposit_collateral_internal(&market_id, &caller, balance * 2u64.into()).unwrap();
		<T as pallet::Config>::Vault::deposit(&vault_id, &caller, balance * 2u64.into()).unwrap();
		let amount_to_borrow: T::Balance = 0u32.into();
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
		let (market_id, vault_id) = create_market::<T>(caller.clone(), BTC, USDT);
		let balance: T::Balance = 1_000_000u64.into();
		<T as pallet::Config>::Currency::mint_into(USDT.into(), &caller, balance * 6u64.into()).unwrap();
		<T as pallet::Config>::Currency::mint_into(BTC.into(), &caller, balance * 6u64.into()).unwrap();
		Lending::<T>::deposit_collateral_internal(&market_id, &caller, balance * 2u64.into()).unwrap();
		<T as pallet::Config>::Vault::deposit(&vault_id, &caller, balance * 2u64.into()).unwrap();
		let repay_amount: T::Balance = 0u32.into();
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

	now {
	}: {
		Lending::<T>::now()
	}

	accrue_interest {
		let caller: T::AccountId = whitelisted_caller();
		let (borrow_asset_id, collateral_asset_id) = (1u32, 2u32);
		set_price::<T>(borrow_asset_id.into(), u64::from(borrow_asset_id) * 10);
		set_price::<T>(collateral_asset_id.into(), u64::from(collateral_asset_id) * 10);
		let (market_id, _) = create_market::<T>(caller, borrow_asset_id.into(), collateral_asset_id.into());
	}: {
		Lending::<T>::accrue_interest(&market_id, 6).unwrap()
	}

	account_id {
		let caller: T::AccountId = whitelisted_caller();
		let (borrow_asset_id, collateral_asset_id) = (1u32, 2u32);
		set_price::<T>(borrow_asset_id.into(), u64::from(borrow_asset_id) * 10);
		set_price::<T>(collateral_asset_id.into(), u64::from(collateral_asset_id) * 10);
		let (market_id, _) = create_market::<T>(caller, borrow_asset_id.into(), collateral_asset_id.into());
	}: {
		Lending::<T>::account_id(&market_id)
	}

	available_funds {
		let caller: T::AccountId = whitelisted_caller();
		let (borrow_asset_id, collateral_asset_id) = (1u32, 2u32);
		set_price::<T>(borrow_asset_id.into(), u64::from(borrow_asset_id) * 10);
		set_price::<T>(collateral_asset_id.into(), u64::from(collateral_asset_id) * 10);
		let (market_id, _) = create_market::<T>(caller.clone(), borrow_asset_id.into(), collateral_asset_id.into());
		let market_config = Markets::<T>::try_get(market_id).unwrap();
	}: {
		Lending::<T>::available_funds(&market_config, &caller).unwrap()
	}

	handle_withdrawable {
		let caller: T::AccountId = whitelisted_caller();
		let (borrow_asset_id, collateral_asset_id) = (1_u32, 2u32);
		set_price::<T>(borrow_asset_id.into(), u64::from(borrow_asset_id) * 10);
		set_price::<T>(collateral_asset_id.into(), u64::from(collateral_asset_id) * 10);
		let (market_id, vault_id) = create_market::<T>(caller.clone(), borrow_asset_id.into(), collateral_asset_id.into());
		let market_config = Markets::<T>::try_get(market_id).unwrap();
		let balance = 0_u32.into();
	}: {
		Lending::<T>::handle_withdrawable(&market_config, &caller, balance).unwrap()
	}

	handle_depositable {
		let caller: T::AccountId = whitelisted_caller();
		let (borrow_asset_id, collateral_asset_id) = (1u32, 2u32);
		set_price::<T>(borrow_asset_id.into(), u64::from(borrow_asset_id) * 10);
		set_price::<T>(collateral_asset_id.into(), u64::from(collateral_asset_id) * 10);
		let (market_id, _) = create_market::<T>(caller.clone(), borrow_asset_id.into(), collateral_asset_id.into());
		let market_config = Markets::<T>::try_get(market_id).unwrap();
		let balance = 1_000_000u32.into();
	}: {
		Lending::<T>::handle_depositable(&market_config, &caller, balance).unwrap()
	}

	handle_must_liquidate {
		let caller: T::AccountId = whitelisted_caller();
		let (borrow_asset_id, collateral_asset_id) = (1u32, 2u32);
		set_price::<T>(borrow_asset_id.into(), u64::from(borrow_asset_id) * 10);
		set_price::<T>(collateral_asset_id.into(), u64::from(collateral_asset_id) * 10);
		let (market_id, _) = create_market::<T>(caller.clone(), borrow_asset_id.into(), collateral_asset_id.into());
		let market_config = Markets::<T>::try_get(market_id).unwrap();
	}: {
		Lending::<T>::handle_must_liquidate(&market_config, &caller).unwrap()
	}
}

impl_benchmark_test_suite!(Lending, crate::mock::new_test_ext(), crate::mock::Test,);
