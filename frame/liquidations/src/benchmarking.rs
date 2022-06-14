use super::*;
use crate::Pallet as Liquidations;
use codec::Decode;
use composable_traits::{
	defi::{CurrencyPair, DeFiComposableConfig, Ratio, Sell},
	liquidation::Liquidation,
};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::traits::{fungibles::Mutate, Currency, Get, Hooks};
use frame_system::{pallet_prelude::BlockNumberFor, RawOrigin};
use sp_runtime::FixedPointNumber;
use sp_std::prelude::*;

pub type AssetIdOf<T> = <T as DeFiComposableConfig>::MayBeAssetId;
fn assets<T>() -> CurrencyPair<AssetIdOf<T>>
where
	T: Config,
{
	let a = 1_u128.to_be_bytes();
	let b = 2_u128.to_be_bytes();
	CurrencyPair::new(
		AssetIdOf::<T>::decode(&mut &a[..]).unwrap(),
		AssetIdOf::<T>::decode(&mut &b[..]).unwrap(),
	)
}

// meaningless sell of 1 to 1
pub fn sell_identity<T: Config>(
) -> Sell<<T as DeFiComposableConfig>::MayBeAssetId, <T as DeFiComposableConfig>::Balance> {
	let one: <T as DeFiComposableConfig>::Balance = 1_u64.into();
	let pair = assets::<T>();
	Sell::new(pair.base, pair.quote, one, Ratio::saturating_from_integer(one))
}

benchmarks! {
	add_liquidation_strategy {
		let sell = sell_identity::<T>();
		let account_id : T::AccountId = whitelisted_caller();
		let caller = RawOrigin::Signed(account_id.clone());
		let amount = 100;
		}: {
		<Liquidations::<T> as Liquidation>::liquidate(
				&account_id,
				sell,
				vec![])
			}
}

impl_benchmark_test_suite!(
	Liquidations,
	crate::mock::runtime::new_test_externalities(),
	crate::mock::runtime::Runtime,
);
