use super::*;
use crate::{Pallet as Pablo, PoolConfiguration::DualAssetConstantProduct};
use composable_traits::{
	defi::CurrencyPair,
	dex::{Amm, AssetAmount},
};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{
	assert_ok, bounded_btree_map,
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

fn generate_benchmark_pools<T: Config>(n: u128) -> Vec<PoolCreationInput<T>> {
	(0..n)
		.map(|i| {
			PoolCreationInput::new_two_token_pool(
				(i + 1).into(),
				Permill::from_percent(50),
				(i + 2).into(),
				(i + 100).into(),
				Permill::from_rational::<u32>(3, 1000),
			)
		})
		.collect()
}

#[derive(Clone)]
struct PoolCreationInput<T: Config> {
	/// Initial Configuration for the Pool
	init_config: PoolInitConfiguration<T::AccountId, T::AssetId>,
	/// LP Token for pool to mint
	lp_token: T::AssetId,
}

impl<T: Config> PoolCreationInput<T> {
	fn new_two_token_pool(
		first_asset_id: T::AssetId,
		first_asset_weight: Permill,
		second_asset_id: T::AssetId,
		lp_asset: T::AssetId,
		fee: Permill,
	) -> Self {
		Self {
			init_config: create_two_token_pool_config::<T>(
				first_asset_id,
				second_asset_id,
				first_asset_weight,
				fee,
			),
			lp_token: lp_asset,
		}
	}
}

fn create_two_token_pool_config<T: Config>(
	first_asset_id: T::AssetId,
	second_asset_id: T::AssetId,
	first_asset_weight: Permill,
	fee: Permill,
) -> PoolInitConfiguration<T::AccountId, T::AssetId> {
	let owner = whitelisted_caller();

	#[allow(clippy::disallowed_methods)] // BTree size is within bounds
	let assets_weights = bounded_btree_map! {
		first_asset_id => first_asset_weight,
		second_asset_id => first_asset_weight.left_from_one(),
	};

	PoolInitConfiguration::<T::AccountId, T::AssetId>::DualAssetConstantProduct {
		owner,
		assets_weights,
		fee,
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
		let pool = generate_benchmark_pools::<T>(1)[0].to_owned();
	}: {
			Pallet::<T>::do_create_pool(
				pool.init_config.to_owned(),
				Some(pool.lp_token),
			).expect("Pool has valid config");
	}
}

impl_benchmark_test_suite!(Pablo, crate::mock::new_test_ext(), crate::mock::Test);
