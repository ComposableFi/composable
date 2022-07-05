use super::*;
use crate::Pallet as DutchAuction;
use codec::Decode;
use composable_traits::defi::{CurrencyPair, DeFiComposableConfig, Ratio, Sell, Take};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::traits::{fungibles::Mutate, Currency, Get, Hooks};
use frame_system::{pallet_prelude::BlockNumberFor, RawOrigin};
use sp_runtime::{
	traits::{AccountIdConversion, Saturating},
	FixedPointNumber,
};
use sp_std::prelude::*;

// meaningless sell of 1 to 1
pub fn sell_identity<T: Config>(
) -> Sell<<T as DeFiComposableConfig>::MayBeAssetId, <T as DeFiComposableConfig>::Balance> {
	let one: <T as DeFiComposableConfig>::Balance = 1_u64.into();
	let pair = assets::<T>();
	Sell::new(pair.base, pair.quote, one, Ratio::saturating_from_integer(one))
}

// meaningless take of 1 to 1
pub fn take_identity<T: Config>() -> Take<<T as DeFiComposableConfig>::Balance> {
	let one: <T as DeFiComposableConfig>::Balance = 1_u64.into();
	Take::new(one, Ratio::saturating_from_integer(one))
}

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

fn mint_native_tokens<T>(account_id: &T::AccountId)
where
	T: Config,
	<T as Config>::MultiCurrency:
		Mutate<T::AccountId, Balance = T::Balance, AssetId = T::MayBeAssetId>,
	<T as Config>::NativeCurrency: frame_support::traits::tokens::currency::Currency<T::AccountId>,
{
	let treasury = &T::PalletId::get().into_account_truncating();
	let native_token_amount = <T as pallet::Config>::NativeCurrency::minimum_balance()
		.saturating_mul(1_000_000_000u32.into());
	<T as pallet::Config>::NativeCurrency::make_free_balance_be(&treasury, native_token_amount);
	<T as pallet::Config>::NativeCurrency::make_free_balance_be(account_id, native_token_amount);
}

benchmarks! {
	where_clause {
		where
		<T as Config>::MultiCurrency:
				Mutate<T::AccountId, Balance = T::Balance, AssetId = T::MayBeAssetId>,
				<T as Config>::NativeCurrency : frame_support::traits::tokens::currency::Currency<T::AccountId>,

	}
	ask {
		let sell = sell_identity::<T>();
		let account_id : T::AccountId = whitelisted_caller();
		let caller = RawOrigin::Signed(account_id.clone());
		let amount: T::Balance = 1_000_000_000_000_u64.into();
		mint_native_tokens::<T>(&account_id);
		<T as pallet::Config>::MultiCurrency::mint_into(sell.pair.base, &account_id, amount).unwrap();
		}: _(
			caller,
			sell,
			<_>::default()
		)
	take {
		let sell = sell_identity::<T>();
		let account_id : T::AccountId = whitelisted_caller();
		let caller = RawOrigin::Signed(account_id.clone());
		let amount: T::Balance = 1_000_000_000_000_u64.into();
		mint_native_tokens::<T>(&account_id);
		<T as pallet::Config>::MultiCurrency::mint_into(sell.pair.base, &account_id, amount).unwrap();
		<T as pallet::Config>::MultiCurrency::mint_into(sell.pair.quote, &account_id, amount).unwrap();
		DutchAuction::<T>::ask(caller.clone().into(), sell, <_>::default()).unwrap();
		let order_id = OrdersIndex::<T>::get();
		let take_order = take_identity::<T>();
		DutchAuction::<T>::take(caller.clone().into(), order_id, take_order.clone()).unwrap();
		}: _(
			caller,
			order_id,
			take_order
		)
	liquidate {
		let sell = sell_identity::<T>();
		let account_id : T::AccountId = whitelisted_caller();
		let caller = RawOrigin::Signed(account_id.clone());
		let amount: T::Balance = 1_000_000_000_000_u64.into();
		mint_native_tokens::<T>(&account_id);
		<T as pallet::Config>::MultiCurrency::mint_into(sell.pair.base, &account_id, amount).unwrap();
		DutchAuction::<T>::ask(caller.clone().into(), sell, <_>::default()).unwrap();
		let order_id = OrdersIndex::<T>::get();
		}: _(
			caller,
			order_id
		)
	known_overhead_for_on_finalize {
		let sell = sell_identity::<T>();
		let account_id : T::AccountId = whitelisted_caller();
		let caller = RawOrigin::Signed(account_id.clone());
		let amount: T::Balance = 1_000_000_000_000_u64.into();
		mint_native_tokens::<T>(&account_id);
		<T as pallet::Config>::MultiCurrency::mint_into(sell.pair.base, &account_id, amount).unwrap();
		<T as pallet::Config>::MultiCurrency::mint_into(sell.pair.quote, &account_id, amount).unwrap();
		DutchAuction::<T>::ask(caller.clone().into(), sell, <_>::default()).unwrap();
		let order_id = OrdersIndex::<T>::get();
		let take_order = take_identity::<T>();
		DutchAuction::<T>::take(caller.into(), order_id, take_order).unwrap();
	} : {
		<DutchAuction::<T> as Hooks<BlockNumberFor<T>>>::on_finalize(T::BlockNumber::default())
	}
}

impl_benchmark_test_suite!(
	DutchAuction,
	crate::mock::runtime::new_test_externalities(),
	crate::mock::runtime::Runtime,
);
