use super::*;
use crate::Pallet as StableSwap;
use composable_traits::{defi::CurrencyPair, dex::Amm};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{
	assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use frame_system::RawOrigin;
use sp_arithmetic::Permill;

benchmarks! {
  where_clause { where T::Balance: From<u128>, T::AssetId: From<u128> }

  create {
	  let usdc: T::AssetId = 100.into();
	  let usdt: T::AssetId = 101.into();
	  let owner = whitelisted_caller();
	  let pair = CurrencyPair::new(usdc, usdt);
	  let amplification_factor = 100_u16;
	  let fee = Permill::from_percent(1);
	  let protocol_fee = Permill::from_percent(1);
  } : _(RawOrigin::Signed(owner), pair, amplification_factor, fee, protocol_fee)

  add_liquidity {
	  let usdc: T::AssetId = 100.into();
	  let usdt: T::AssetId = 101.into();
	  let owner = whitelisted_caller();
	  let pool_id = StableSwap::<T>::do_create_pool(
		  &owner,
		  CurrencyPair::new(usdc, usdt),
		  1000_u16,
		  Permill::from_percent(1),
		  Permill::from_percent(1),
	  ) .expect("impossible; qed;");
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
	  let owner = whitelisted_caller();
	  let pool_id = StableSwap::<T>::do_create_pool(
		  &owner,
		  CurrencyPair::new(usdc, usdt),
		  1000_u16,
		  Permill::from_percent(1),
		  Permill::from_percent(1),
	  ) .expect("impossible; qed;");
	  let unit = 1_000_000_000_000;
	  // 100_000_000 USDC , 100_000_000 USDT
	  let initial_usdc: T::Balance = (100_000_000_u128 * unit).into();
	  let initial_usdt: T::Balance = (100_000_000_u128 * unit).into();
	  // Mint the tokens
	  assert_ok!(T::Assets::mint_into(usdc, &owner, initial_usdc));
	  assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
	  // Add the liquidity
	  assert_ok!(<StableSwap<T> as Amm>::add_liquidity(
			  &owner,
			  pool_id,
			  initial_usdc,
			  initial_usdt,
			  0.into(),
			  false
	  ));
	  let pool_info = StableSwap::<T>::get_pool(pool_id).expect("impossible; qed;");
	  let lp_amount = T::Assets::balance(pool_info.lp_token, &owner);
  }: _(RawOrigin::Signed(owner), pool_id, lp_amount, (0_u128).into(), (0_u128).into())

 buy {
	  let usdc: T::AssetId = 100.into();
	  let usdt: T::AssetId = 101.into();
	  let owner = whitelisted_caller();
	  let pool_id = StableSwap::<T>::do_create_pool(
		  &owner,
		  CurrencyPair::new(usdc, usdt),
		  1000_u16,
		  Permill::from_percent(1),
		  Permill::from_percent(1),
	  ) .expect("impossible; qed;");
	  let unit = 1_000_000_000_000;
	  // 100_000_000 USDC , 100_000_000 USDT
	  let initial_usdc: T::Balance = (100_000_000_u128 * unit).into();
	  let initial_usdt: T::Balance = (100_000_000_u128 * unit).into();
	  // Mint the tokens
	  assert_ok!(T::Assets::mint_into(usdc, &owner, initial_usdc));
	  assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
	  // Add the liquidity
	  assert_ok!(<StableSwap<T> as Amm>::add_liquidity(
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
 }: _(RawOrigin::Signed(user), pool_id, usdc, (1000_u128 * unit).into(), false)

 sell {
	  let usdc: T::AssetId = 100.into();
	  let usdt: T::AssetId = 101.into();
	  let owner = whitelisted_caller();
	  let pool_id = StableSwap::<T>::do_create_pool(
		  &owner,
		  CurrencyPair::new(usdc, usdt),
		  100_u16,
		  Permill::from_percent(1),
		  Permill::from_percent(1),
	  ) .expect("impossible; qed;");
	  let unit = 1_000_000_000_000;
	  // 100_000_000 USDC , 100_000_000 USDT
	  let initial_usdc: T::Balance = (100_000_000_u128 * unit).into();
	  let initial_usdt: T::Balance = (100_000_000_u128 * unit).into();
	  // Mint the tokens
	  assert_ok!(T::Assets::mint_into(usdc, &owner, initial_usdc));
	  assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
	  // Add the liquidity
	  assert_ok!(<StableSwap<T> as Amm>::add_liquidity(
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
 }: _(RawOrigin::Signed(user), pool_id, usdc, (1000_u128 * unit).into(), false)

 swap {
	  let usdc: T::AssetId = 100.into();
	  let usdt: T::AssetId = 101.into();
	  let owner = whitelisted_caller();
	  let pair = CurrencyPair::new(usdc, usdt);
	  let pool_id = StableSwap::<T>::do_create_pool(
		  &owner,
		  pair,
		  100_u16,
		  Permill::from_percent(1),
		  Permill::from_percent(1),
	  ) .expect("impossible; qed;");
	  let unit = 1_000_000_000_000;
	  // 100_000_000 USDC , 100_000_000 USDT
	  let initial_usdc: T::Balance = (100_000_000_u128 * unit).into();
	  let initial_usdt: T::Balance = (100_000_000_u128 * unit).into();
	  // Mint the tokens
	  assert_ok!(T::Assets::mint_into(usdc, &owner, initial_usdc));
	  assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
	  // Add the liquidity
	  assert_ok!(<StableSwap<T> as Amm>::add_liquidity(
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
 }: _(RawOrigin::Signed(user), pool_id, pair, (1000_u128 * unit).into(), 0.into(), false)
}

impl_benchmark_test_suite!(StableSwap, crate::mock::new_test_ext(), crate::mock::Test);
