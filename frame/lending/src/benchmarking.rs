//! Benchmarks and  sanity tests for lending. Only test that action do not error, not that produce
//! positive side effects

use super::{setup::*, *};
use crate::{self as pallet_lending, Pallet as Lending};
use composable_support::validation::Validated;
use composable_traits::{
	defi::{CurrencyPair, DeFiComposableConfig, MoreThanOneFixedU128},
	lending::{math::InterestRateModel, CreateInput, Lending as LendingTrait, UpdateInput},
	oracle::Price,
	vault::StrategicVault,
};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::traits::fungibles::Mutate;
use frame_system::EventRecord;
use sp_runtime::{FixedPointNumber, Percent, Perquintill};
use sp_std::prelude::*;

#[allow(dead_code)]
pub fn assert_last_event<T: pallet_lending::Config>(generic_event: <T as Config>::Event) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::Event = generic_event.into();
	// compare to the last event record
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

pub fn set_price<
	T: pallet_lending::Config + pallet_oracle::Config + composable_traits::defi::DeFiComposableConfig,
>(
	asset_id: T::MayBeAssetId,
	price: u64,
) {
	let asset_id = asset_id.encode();
	let asset_id = <T as pallet_oracle::Config>::AssetId::decode(&mut &asset_id[..]).unwrap();
	pallet_oracle::Prices::<T>::insert(
		asset_id,
		Price { price: <T as pallet_oracle::Config>::PriceValue::from(price), block: 0_u32.into() },
	);
}

fn set_prices<T: Config + pallet_oracle::Config>() {
	let pair = assets::<T>();
	set_price::<T>(pair.base, 48_000_000_000_u64);
	set_price::<T>(pair.quote, 1_000_000_000_u64);
}

#[allow(dead_code)]
fn create_new_market<T: pallet_lending::Config + pallet_oracle::Config + DeFiComposableConfig>(
	manager: T::AccountId,
	borrow_asset: <T as composable_traits::defi::DeFiComposableConfig>::MayBeAssetId,
	collateral_asset: <T as composable_traits::defi::DeFiComposableConfig>::MayBeAssetId,
) -> (crate::MarketIndex, <T as Config>::VaultId) {
	let market_config = create_market_config::<T>(borrow_asset, collateral_asset);
	Lending::<T>::create(manager, market_config).unwrap()
}

fn create_market_config<T: DeFiComposableConfig + pallet_lending::Config>(
	collateral_asset: <T as DeFiComposableConfig>::MayBeAssetId,
	borrow_asset: <T as DeFiComposableConfig>::MayBeAssetId,
) -> CreateInput<<T as Config>::LiquidationStrategyId, <T as DeFiComposableConfig>::MayBeAssetId> {
	CreateInput {
		updatable: UpdateInput {
			collateral_factor: MoreThanOneFixedU128::saturating_from_rational(200_u128, 100_u128),
			under_collateralized_warn_percent: Percent::from_percent(10),
			liquidators: vec![],
			interest_rate_model: InterestRateModel::default(),
		},
		reserved_factor: Perquintill::from_percent(10),
		currency_pair: CurrencyPair::new(collateral_asset, borrow_asset),
	}
}

benchmarks! {
	where_clause {
		where
			T:
					pallet_oracle::Config
					+ pallet_lending::Config
					+ DeFiComposableConfig
					+ pallet_balances::Config
					+ frame_system::Config
					+ pallet_timestamp::Config
					+ pallet_vault::Config,
			<T as pallet_balances::Config>::Balance : From<u64>,
			<T as frame_system::Config>::BlockNumber : From<u32>,
			<T as pallet_timestamp::Config>::Moment : From<u64>,
			<T as pallet_vault::Config>::Balance : From<u64>,
	}
	create_market {
		let caller= whitelisted_caller::<T::AccountId>();
		let origin =  whitelisted_origin::<T>();
		let amount: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_000_u64.into();
		let bank: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 10_000_000_000_000_u64.into();
		let pair = assets::<T>();
		set_prices::<T>();
		let input = create_market_config::<T>(pair.base, pair.quote);
		<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.base, &caller, bank).unwrap();
		<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.quote, &caller, bank).unwrap();
	} : {
		Lending::<T>::create_market(origin.into(), Validated::new(input).unwrap()).unwrap()
	}
	deposit_collateral {
		let caller= whitelisted_caller::<T::AccountId>();
		let origin =  whitelisted_origin::<T>();
		let amount: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_000_u64.into();
		let bank: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 10_000_000_000_000_u64.into();
		let pair = assets::<T>();
		let input = create_market_config::<T>(pair.base, pair.quote);
		set_prices::<T>();
		<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.base, &caller, bank).unwrap();
		<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.quote, &caller, bank).unwrap();
		Lending::<T>::create_market(origin.clone().into(), Validated::new(input).unwrap()).unwrap();
		let market_id = MarketIndex::new(1);
		}:  {
			Lending::<T>::deposit_collateral(origin.into(), market_id, amount).unwrap();
		}
		withdraw_collateral {
			let caller= whitelisted_caller::<T::AccountId>();
			let origin =  whitelisted_origin::<T>();
			let amount: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_000_u64.into();
			let part: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_u64.into();
			let bank: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 10_000_000_000_000_u64.into();
			let pair = assets::<T>();
			let input = create_market_config::<T>(pair.base, pair.quote);
			set_prices::<T>();
			<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.base, &caller, bank).unwrap();
			<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.quote, &caller, bank).unwrap();
			Lending::<T>::create_market(origin.clone().into(), Validated::new(input).unwrap()).unwrap();
			let market_id = MarketIndex::new(1);
			Lending::<T>::deposit_collateral(origin.clone().into(), market_id, amount).unwrap();
		}:  {
				Lending::<T>::withdraw_collateral(origin.into(), market_id, part).unwrap();
			}
		borrow {
			let caller= whitelisted_caller::<T::AccountId>();
			let origin =  whitelisted_origin::<T>();
			let amount: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_000_u64.into();
			let part: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_u64.into();
			let bank: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 10_000_000_000_000_u64.into();
			let pair = assets::<T>();
			let input = create_market_config::<T>(pair.base, pair.quote);
			set_prices::<T>();
			//pallet_balances::::<T>
			<pallet_balances::Pallet::<T> as frame_support::traits::fungible::Mutate<T::AccountId>>::mint_into(&caller, 10_000_000_000_000_u64.into()).unwrap();
			<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.base, &caller, bank).unwrap();
			<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.quote, &caller, bank).unwrap();
			Lending::<T>::create_market(origin.clone().into(), Validated::new(input).unwrap()).unwrap();
			let market_id = MarketIndex::new(1);
			Lending::<T>::deposit_collateral(origin.clone().into(), market_id, amount).unwrap();
		}:  {
				Lending::<T>::borrow(origin.into(), market_id, part).unwrap();
			}
		repay_borrow {
			let caller= whitelisted_caller::<T::AccountId>();
			let origin =  whitelisted_origin::<T>();
			let amount: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_000_000_000_u64.into();
			let borrow_amount: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_000_000_u64.into();
			let part: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_u64.into();
			let bank: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 10_000_000_000_000_u64.into();
			let pair = assets::<T>();
			let input = create_market_config::<T>(pair.base, pair.quote);
			set_prices::<T>();
			<pallet_balances::Pallet::<T> as frame_support::traits::fungible::Mutate<T::AccountId>>::mint_into(&caller, 10_000_000_000_000_u64.into()).unwrap();
			<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.base, &caller, bank).unwrap();
			<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.quote, &caller, bank).unwrap();
			Lending::<T>::create_market(origin.clone().into(), Validated::new(input).unwrap()).unwrap();
			let market_id = MarketIndex::new(1);
			Lending::<T>::deposit_collateral(origin.clone().into(), market_id, amount).unwrap();
			Lending::<T>::borrow(origin.clone().into(), market_id, borrow_amount).unwrap();
			produce_block::<T>(42_u32.into(),4200_u64.into());
			produce_block::<T>(43_u32.into(),4300_u64.into());

		}:  {
				Lending::<T>::repay_borrow_partial(origin.clone().into(), market_id, caller, part).unwrap();
			}
		now {
		}: {
			Lending::<T>::now()
		}

		accrue_interest {
			let caller= whitelisted_caller::<T::AccountId>();
			let origin =  whitelisted_origin::<T>();
			let amount: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_000_000_000_u64.into();
			let borrow_amount: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_000_000_u64.into();
			let part: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_u64.into();
			let bank: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 10_000_000_000_000_u64.into();
			let pair = assets::<T>();
			let input = create_market_config::<T>(pair.base, pair.quote);
			set_prices::<T>();
			<pallet_balances::Pallet::<T> as frame_support::traits::fungible::Mutate<T::AccountId>>::mint_into(&caller, 10_000_000_000_000_u64.into()).unwrap();
			<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.base, &caller, bank).unwrap();
			<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.quote, &caller, bank).unwrap();
			Lending::<T>::create_market(origin.clone().into(), Validated::new(input).unwrap()).unwrap();
			let market_id = MarketIndex::new(1);
			Lending::<T>::deposit_collateral(origin.clone().into(), market_id, amount).unwrap();
			Lending::<T>::borrow(origin.into(), market_id, borrow_amount).unwrap();
			produce_block::<T>(42_u32.into(),4200_u64.into());
			produce_block::<T>(43_u32.into(),4300_u64.into());
		}:  {
			// TODO: fix it, make timestamp depend on x increased OR make the value passed be DELTA
			Lending::<T>::accrue_interest(&market_id, 4400_u64).unwrap();
			}
		account_id {
			let caller= whitelisted_caller::<T::AccountId>();
			let origin =  whitelisted_origin::<T>();
			let amount: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_000_000_000_u64.into();
			let part: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_u64.into();
			let bank: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 10_000_000_000_000_u64.into();
			let pair = assets::<T>();
			let input = create_market_config::<T>(pair.base, pair.quote);
			set_prices::<T>();
			<pallet_balances::Pallet::<T> as frame_support::traits::fungible::Mutate<T::AccountId>>::mint_into(&caller, 10_000_000_000_000_u64.into()).unwrap();
			<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.base, &caller, bank).unwrap();
			<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.quote, &caller, bank).unwrap();
			Lending::<T>::create_market(origin.into(), Validated::new(input).unwrap()).unwrap();
			let market_id = MarketIndex::new(1);
		}:  {
			// TODO: fix it, make timestamp depend on x increased OR make the value passed be DELTA
			Lending::<T>::account_id(&market_id)
			}
			available_funds {
				let caller= whitelisted_caller::<T::AccountId>();
				let origin =  whitelisted_origin::<T>();
				let amount: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_000_000_000_u64.into();
				let part: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_u64.into();
				let bank: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 10_000_000_000_000_u64.into();
				let pair = assets::<T>();
				let input = create_market_config::<T>(pair.base, pair.quote);
				set_prices::<T>();
				<pallet_balances::Pallet::<T> as frame_support::traits::fungible::Mutate<T::AccountId>>::mint_into(&caller, 10_000_000_000_000_u64.into()).unwrap();
				<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.base, &caller, bank).unwrap();
				<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.quote, &caller, bank).unwrap();
				Lending::<T>::create_market(origin.into(), Validated::new(input).unwrap()).unwrap();
				let market_id = MarketIndex::new(1);
				let market_config = Markets::<T>::try_get(market_id).unwrap();
			}:  {
					// TODO: make changes to vault state so something happens
					Lending::<T>::available_funds(&market_config, &caller).unwrap()
				}
			handle_withdrawable {
				let caller= whitelisted_caller::<T::AccountId>();
				let origin =  whitelisted_origin::<T>();
				let amount: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_000_000_000_u64.into();
				let part: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_u64.into();
				let bank: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 10_000_000_000_000_u64.into();
				let pair = assets::<T>();
				let input = create_market_config::<T>(pair.base, pair.quote);
				set_prices::<T>();
				<pallet_balances::Pallet::<T> as frame_support::traits::fungible::Mutate<T::AccountId>>::mint_into(&caller, 10_000_000_000_000_u64.into()).unwrap();
				<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.base, &caller, bank).unwrap();
				<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.quote, &caller, bank).unwrap();
				Lending::<T>::create_market(origin.clone().into(), Validated::new(input).unwrap()).unwrap();
				let market_id = MarketIndex::new(1);
				Lending::<T>::deposit_collateral(origin.into(), market_id, amount).unwrap();
				let market_config = Markets::<T>::try_get(market_id).unwrap();
				let account = &Lending::<T>::account_id(&market_id);
				<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.base, account, bank).unwrap();
				<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.quote, account, bank).unwrap();
				<T as pallet_lending::Config>::Vault::deposit(&market_config.borrow, account, bank).unwrap();
			}:  {
					Lending::<T>::handle_withdrawable(&market_config, &caller, part ).unwrap()
				}
			handle_depositable {
			let caller= whitelisted_caller::<T::AccountId>();
			let origin =  whitelisted_origin::<T>();
			let amount: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_000_000_000_u64.into();
			let part: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_u64.into();
			let bank: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 10_000_000_000_000_u64.into();
			let pair = assets::<T>();
			let input = create_market_config::<T>(pair.base, pair.quote);
			set_prices::<T>();
			<pallet_balances::Pallet::<T> as frame_support::traits::fungible::Mutate<T::AccountId>>::mint_into(&caller, 10_000_000_000_000_u64.into()).unwrap();
			<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.base, &caller, bank).unwrap();
			<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.quote, &caller, bank).unwrap();
			Lending::<T>::create_market(origin.clone().into(), Validated::new(input).unwrap()).unwrap();
			let market_id = MarketIndex::new(1);
			Lending::<T>::deposit_collateral(origin.into(), market_id, amount).unwrap();
			let market_config = Markets::<T>::try_get(market_id).unwrap();
			let account = &Lending::<T>::account_id(&market_id);
			<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.base, account, bank).unwrap();
			<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.quote, account, bank).unwrap();
		}:  {
				// TODO: make it variable with x
				Lending::<T>::handle_depositable(&market_config, &caller, part ).unwrap()
			}
			handle_must_liquidate {
			let caller= whitelisted_caller::<T::AccountId>();
			let origin =  whitelisted_origin::<T>();
			let amount: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_000_000_000_u64.into();
			let part: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 1_000_u64.into();
			let bank: <T as composable_traits::defi::DeFiComposableConfig>::Balance  = 10_000_000_000_000_u64.into();
			let pair = assets::<T>();
			let input = create_market_config::<T>(pair.base, pair.quote);
			set_prices::<T>();
			<pallet_balances::Pallet::<T> as frame_support::traits::fungible::Mutate<T::AccountId>>::mint_into(&caller, 10_000_000_000_000_u64.into()).unwrap();
			<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.base, &caller, bank).unwrap();
			<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.quote, &caller, bank).unwrap();
			Lending::<T>::create_market(origin.clone().into(), Validated::new(input).unwrap()).unwrap();
			let market_id = MarketIndex::new(1);
			Lending::<T>::deposit_collateral(origin.into(), market_id, amount).unwrap();
			let market_config = Markets::<T>::try_get(market_id).unwrap();
			let account = &Lending::<T>::account_id(&market_id);
			<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.base, account, bank).unwrap();
			<T as pallet_lending::Config>::MultiCurrency::mint_into(pair.quote, account, bank).unwrap();
		}:  {
				// TODO: make it variable with x
				Lending::<T>::handle_must_liquidate(&market_config, &caller ).unwrap()
			}
}

impl_benchmark_test_suite!(Lending, crate::mocks::new_test_ext(), crate::mocks::Runtime,);
