use super::*;
use crate::{Pallet as Pablo, PoolConfiguration::DualAssetConstantProduct};
use composable_traits::{
	defi::CurrencyPair,
	dex::{Amm, AssetAmount},
};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{
	assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use frame_system::RawOrigin;
use sp_arithmetic::{PerThing, Permill};
use sp_runtime::BoundedBTreeMap;
use sp_std::collections::btree_map::BTreeMap;

fn amm_init_config<T: Config>(
	owner: T::AccountId,
	pair: CurrencyPair<T::AssetId>,
	base_weight: Permill,
	fee: Permill,
) -> PoolInitConfigurationOf<T> {
	let mut assets_weights = BoundedBTreeMap::new();
	assets_weights.try_insert(pair.base, base_weight).expect("Should work");
	assets_weights
		.try_insert(pair.quote, base_weight.left_from_one())
		.expect("Should work");
	PoolInitConfiguration::DualAssetConstantProduct { owner, fee, assets_weights }
}

fn create_amm_pool<T: Config>(
	owner: T::AccountId,
	pair: CurrencyPair<T::AssetId>,
	lp_token_id: T::AssetId,
) -> T::PoolId {
	let swap_pool_init =
		amm_init_config::<T>(owner, pair, Permill::from_percent(50), Permill::from_percent(1));
	Pablo::<T>::do_create_pool(swap_pool_init, Some(lp_token_id)).expect("impossible; qed;")
}

fn get_lp_token<T: Config>(pool_id: T::PoolId) -> T::AssetId {
	let pool_info = Pablo::<T>::get_pool(pool_id).expect("impossible; qed;");
	match pool_info {
		DualAssetConstantProduct(pool) => pool.lp_token,
	}
}

benchmarks! {
  where_clause { where T::BlockNumber: From<u32>, T::Balance: From<u128>, T::AssetId: From<u128> }
	create {
		let usdc = 100.into();
		let usdt = 101.into();
		let owner: T::AccountId = whitelisted_caller();
		let pair = CurrencyPair::new(usdc, usdt);
		let fee = Permill::from_percent(1);
		let protocol_fee = Permill::from_percent(1);
		let stable_swap_pool_init = amm_init_config::<T>(owner, pair, Permill::from_percent(50_u32), fee);
	  } : _(RawOrigin::Root, stable_swap_pool_init)

	add_liquidity {
		let usdc = 100.into();
		let usdt = 101.into();
		let lp_token_id = 1000.into();
		let owner: T::AccountId = whitelisted_caller();
		let pool_id = create_amm_pool::<T>(owner.clone(), CurrencyPair::new(usdc, usdt), lp_token_id);
		let unit = 1_000_000_000_000;
		// 100_000_000 USDC , 100_000_000 USDT
		let initial_usdc: T::Balance = (100_000_000_u128 * unit).into();
		let initial_usdt: T::Balance = (100_000_000_u128 * unit).into();
		// Mint the tokens
		assert_ok!(T::Assets::mint_into(usdc, &owner, initial_usdc));
		assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
	  }: _(RawOrigin::Signed(owner), pool_id, BTreeMap::from([(usdc, initial_usdc), (usdt, initial_usdt)]), 0.into(), false)

	remove_liquidity {
		let usdc = 100.into();
		let usdt = 101.into();
		let lp_token_id = 1000.into();
		let owner: T::AccountId = whitelisted_caller();
		let pool_id = create_amm_pool::<T>(owner.clone(), CurrencyPair::new(usdc, usdt), lp_token_id);
		let unit = 1_000_000_000_000;
		// 100_000_000 USDC , 100_000_000 USDT
		let initial_usdc: T::Balance = (100_000_000_u128 * unit).into();
		let initial_usdt: T::Balance = (100_000_000_u128 * unit).into();
		// Mint the tokens
		assert_ok!(T::Assets::mint_into(usdc, &owner, initial_usdc));
		assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
		// Add the liquidity
		assert_ok!(<Pablo<T> as Amm>::add_liquidity(
			&owner,
			pool_id,
			BTreeMap::from([(usdc, initial_usdc), (usdt, initial_usdt)]),
			0.into(),
			false
		));
		let lp_amount = T::Assets::balance(get_lp_token::<T>(pool_id), &owner);
	  }: _(RawOrigin::Signed(owner), pool_id, lp_amount, BTreeMap::from([(usdc, 0.into()), (usdt, 0.into())]))

	buy {
		let usdc = 100.into();
		let usdt = 101.into();
		let lp_token_id = 1000.into();
		let owner: T::AccountId = whitelisted_caller();
		let pool_id = create_amm_pool::<T>(owner.clone(), CurrencyPair::new(usdc, usdt), lp_token_id);
		let unit = 1_000_000_000_000;
		// 100_000_000 USDC , 100_000_000 USDT
		let initial_usdc: T::Balance = (100_000_000_u128 * unit).into();
		let initial_usdt: T::Balance = (100_000_000_u128 * unit).into();
		// Mint the tokens
		assert_ok!(T::Assets::mint_into(usdc, &owner, initial_usdc));
		assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
		// Add the liquidity
		assert_ok!(<Pablo<T> as Amm>::add_liquidity(
			&owner,
			pool_id,
			BTreeMap::from([(usdc, initial_usdc), (usdt, initial_usdt)]),
			0.into(),
			false
		));
		let user = account("user", 0, 0);
		assert_ok!(T::Assets::mint_into(usdt, &user, (1020_u128 * unit).into()));
		// buy 1000 USDC
	 }: _(RawOrigin::Signed(user), pool_id, usdt, AssetAmount::new(usdc, (1000_u128 * unit).into()), false)

	 swap {
		let usdc = 100.into();
		let usdt = 101.into();
		let lp_token_id = 1000.into();
		let owner: T::AccountId = whitelisted_caller();
		let pool_id = create_amm_pool::<T>(owner.clone(), CurrencyPair::new(usdc, usdt), lp_token_id);

		let unit = 1_000_000_000_000;
		// 100_000_000 USDC , 100_000_000 USDT
		let initial_usdc: T::Balance = (100_000_000_u128 * unit).into();
		let initial_usdt: T::Balance = (100_000_000_u128 * unit).into();
		// Mint the tokens
		assert_ok!(T::Assets::mint_into(usdc, &owner, initial_usdc));
		assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
		// Add the liquidity
		assert_ok!(<Pablo<T> as Amm>::add_liquidity(
			&owner,
			pool_id,
			BTreeMap::from([(usdc, initial_usdc), (usdt, initial_usdt)]),
			0.into(),
			false
		));
		let user = account("user", 0, 0);
		assert_ok!(T::Assets::mint_into(usdt, &user, (1000_u128 * unit).into()));
		// swap 1000 USDC
	 }: _(RawOrigin::Signed(user), pool_id, AssetAmount::new(usdt, (1000_u128 * unit).into()), AssetAmount::new(usdc, 0.into()), false)

	do_create_pool {
		let usdc = 100.into();
		let usdt = 101.into();
		let lp_token_id = 1000.into();
		let owner: T::AccountId = whitelisted_caller();
		let pair = CurrencyPair::new(usdc, usdt);
		let fee = Permill::from_percent(1);
		let protocol_fee = Permill::from_percent(1);
		let stable_swap_pool_init = amm_init_config::<T>(owner, pair, Permill::from_percent(50_u32), fee);
	}: {
			Pallet::<T>::do_create_pool(
				stable_swap_pool_init,
				Some(lp_token_id),
			).expect("Pool has valid config");
	}
}

impl_benchmark_test_suite!(Pablo, crate::mock::new_test_ext(), crate::mock::Test);
