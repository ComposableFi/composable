use super::*;
use crate::Pallet as Liquidations;
use codec::Decode;
use composable_traits::{
	defi::{CurrencyPair, DeFiComposableConfig, Ratio, Sell},
	time::{LinearDecrease, TimeReleaseFunction},
};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::traits::{fungibles::Mutate, Currency, Get};
use frame_system::RawOrigin;
use sp_runtime::{traits::Saturating, FixedPointNumber};
use sp_std::prelude::*;
pub type AssetIdOf<T> = <T as DeFiComposableConfig>::MayBeAssetId;
fn assets<T>() -> CurrencyPair<AssetIdOf<T>>
where
	T: Config,
{
	let a = 1_u128.to_be_bytes();
	let b = 129_u128.to_be_bytes();
	CurrencyPair::new(
		AssetIdOf::<T>::decode(&mut &a[..]).unwrap(),
		AssetIdOf::<T>::decode(&mut &b[..]).unwrap(),
	)
}

benchmarks! {
	where_clause {
		where
			T: pallet_assets::Config + DeFiComposableConfig + orml_tokens::Config + pallet_balances::Config + pallet_dutch_auction::Config,
			<T as orml_tokens::Config>::CurrencyId: From<<T as DeFiComposableConfig>::MayBeAssetId>,
			<T as pallet_dutch_auction::Config>::NativeCurrency: Currency<T::AccountId>,
				}

	add_liquidation_strategy {
		// Only root allowed to add new strategies.
		let origin = RawOrigin::Root;
		let config = LiquidationStrategyConfiguration::DutchAuction(
				TimeReleaseFunction::LinearDecrease(LinearDecrease { total: 10 * 60 }),
			);
		}: _(origin, config)

   sell {
		let x in 1..<T as Config>::MaxLiquidationStrategiesAmount::get() - 1;
		let pair = assets::<T>();
		let one: <T as DeFiComposableConfig>::Balance = 1_u32.into();
		let order = Sell::new(pair.base, pair.quote, one, Ratio::saturating_from_integer(one));
		let caller: T::AccountId = whitelisted_caller();
		let origin = RawOrigin::Signed(caller.clone());
		let root_origin = RawOrigin::<T::AccountId>::Root;
		let config = LiquidationStrategyConfiguration::DutchAuction(
				TimeReleaseFunction::LinearDecrease(LinearDecrease { total: 10 * 60 }),
		);
		Liquidations::<T>::add_liquidation_strategy(root_origin.clone().into(), config.clone()).unwrap();
		let native_token_amount = <<T as pallet_dutch_auction::Config>::NativeCurrency as Currency<T::AccountId>>::minimum_balance().saturating_mul(1_000_000_000_u32.into());
		<<T as pallet_dutch_auction::Config>::NativeCurrency as Currency<T::AccountId>>::make_free_balance_be(&caller, native_token_amount);
		orml_tokens::Pallet::<T>::mint_into(order.pair.base.into(), &caller, 1_000_000_u32.into()).unwrap();
		let begin = 1000;
		let end = begin + x;
		let mut configurations:Vec<T::LiquidationStrategyId> = (begin..end).map(|x| x.into()).collect();
		configurations.push(1.into());
		   }: _(origin, order, configurations)
}

impl_benchmark_test_suite!(
	Liquidations,
	crate::mock::runtime::new_test_externalities(),
	crate::mock::runtime::Runtime,
);
