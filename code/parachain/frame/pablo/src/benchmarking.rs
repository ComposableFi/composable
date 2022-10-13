use super::*;
use crate::{Pallet as Pablo, PoolConfiguration::ConstantProduct};
use composable_traits::{defi::CurrencyPair, dex::Amm};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{
	assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use frame_system::RawOrigin;
use sp_arithmetic::Permill;

fn amm_init_config<T: Config>(
	owner: T::AccountId,
	pair: CurrencyPair<T::AssetId>,
	base_weight: Permill,
	fee: Permill,
) -> PoolInitConfigurationOf<T> {
	PoolInitConfiguration::ConstantProduct { owner, pair, fee, base_weight }
}

fn create_amm_pool<T: Config>(owner: T::AccountId, pair: CurrencyPair<T::AssetId>) -> T::PoolId {
	let swap_pool_init = amm_init_config::<T>(
		owner.clone(),
		pair,
		Permill::from_percent(50),
		Permill::from_percent(1),
	);
	Pablo::<T>::do_create_pool(swap_pool_init).expect("impossible; qed;")
}

fn get_lp_token<T: Config>(pool_id: T::PoolId) -> T::AssetId {
	let pool_info = Pablo::<T>::get_pool(pool_id).expect("impossible; qed;");
	match pool_info {
		ConstantProduct(pool) => pool.lp_token,
	}
}

benchmarks! {
  where_clause { where T::BlockNumber: From<u32>, T::Balance: From<u128>, T::AssetId: From<u128> }
	create {
		let usdc: T::AssetId = 100.into();
		let usdt: T::AssetId = 101.into();
		let owner: T::AccountId = whitelisted_caller();
		let pair = CurrencyPair::new(usdc, usdt);
		let fee = Permill::from_percent(1);
		let protocol_fee = Permill::from_percent(1);
		let stable_swap_pool_init = amm_init_config::<T>(owner.clone(), pair, Permill::from_percent(50_u32), fee);
	  } : _(RawOrigin::Root, stable_swap_pool_init)

	add_liquidity {
		let usdc: T::AssetId = 100.into();
		let usdt: T::AssetId = 101.into();
		let owner: T::AccountId = whitelisted_caller();
		let pool_id = create_amm_pool::<T>(owner.clone(), CurrencyPair::new(usdc, usdt));
		let unit = 1_000_000_000_000;
		// 100_000_000 USDC , 100_000_000 USDT
		let initial_usdc: T::Balance = (100_000_000_u128 * unit).into();
		let initial_usdt: T::Balance = (100_000_000_u128 * unit).into();
		// Mint the tokens
		assert_ok!(T::Assets::mint_into(usdc, &owner, initial_usdc));
		assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
	  }: _(RawOrigin::Signed(owner), pool_id, initial_usdc, initial_usdt, 0.into(), false)

	remove_liquidity {
		let usdc: T::AssetId = 100.into();
		let usdt: T::AssetId = 101.into();
		let owner: T::AccountId = whitelisted_caller();
		let pool_id = create_amm_pool::<T>(owner.clone(), CurrencyPair::new(usdc, usdt));
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
			  initial_usdc,
			  initial_usdt,
			  0.into(),
			  false
		));
		let lp_amount = T::Assets::balance(get_lp_token::<T>(pool_id), &owner);
	  }: _(RawOrigin::Signed(owner), pool_id, lp_amount, (0_u128).into(), (0_u128).into())

	buy {
		let usdc: T::AssetId = 100.into();
		let usdt: T::AssetId = 101.into();
		let owner: T::AccountId = whitelisted_caller();
		let pool_id = create_amm_pool::<T>(owner.clone(), CurrencyPair::new(usdc, usdt));
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
			  initial_usdc,
			  initial_usdt,
			  0.into(),
			  false
		));
		let user = account("user", 0, 0);
		assert_ok!(T::Assets::mint_into(usdt, &user, (1000_u128 * unit).into()));
		// buy 1000 USDC
	 }: _(RawOrigin::Signed(user), pool_id, usdc, (1000_u128 * unit).into(), 0_u128.into(), false)

	sell {
		let usdc: T::AssetId = 100.into();
		let usdt: T::AssetId = 101.into();
		let owner: T::AccountId = whitelisted_caller();
		let pool_id = create_amm_pool::<T>(owner.clone(), CurrencyPair::new(usdc, usdt));
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
			initial_usdc,
			initial_usdt,
			0.into(),
			false
		));
		let user = account("user", 0, 0);
		assert_ok!(T::Assets::mint_into(usdc, &user, (1000_u128 * unit).into()));
		// sell 1000 USDC
	 }: _(RawOrigin::Signed(user), pool_id, usdc, (1000_u128 * unit).into(), 0_u128.into(), false)

	 swap {
		let usdc: T::AssetId = 100.into();
		let usdt: T::AssetId = 101.into();
		let owner: T::AccountId = whitelisted_caller();
		let pool_id = create_amm_pool::<T>(owner.clone(), CurrencyPair::new(usdc, usdt));

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
			  initial_usdc,
			  initial_usdt,
			  0.into(),
			  false
		));
		let user = account("user", 0, 0);
		assert_ok!(T::Assets::mint_into(usdt, &user, (1000_u128 * unit).into()));
		// swap 1000 USDC
	 }: _(RawOrigin::Signed(user), pool_id, CurrencyPair::new(usdc, usdt), (1000_u128 * unit).into(), 0.into(), false)
}

impl_benchmark_test_suite!(Pablo, crate::mock::new_test_ext(), crate::mock::Test);
