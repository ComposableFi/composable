use super::*;
use crate::Pallet as StableSwap;
use composable_traits::{defi::CurrencyPair, dex::CurveAmm};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{assert_ok, traits::fungibles::Mutate};
use frame_system::RawOrigin;
use sp_arithmetic::Permill;

benchmarks! {
  where_clause { where T::Balance: From<u128>, T::AssetsId: From<u128> }

  create {
		let usdc: T::AssetsId = 0.into();
		let usdt: T::AssetsId = 1.into();
		let owner = whitelisted_caller();
		let pair = CurrencyPair::new(usdc, usdt);
		let amplification_factor = 1000_u16;
		let fee = Permill::from_float(0.002);
		let protocl_fee = Permill::from_float(0.01);
  } : _(RawOrigin::Signed(owner), pair, amplification_factor, fee, protocl_fee)

  buy {
	  let usdc: T::AssetId = 0.into();
	  let usdt: T::AssetId = 1.into();
	  let owner = whitelisted_caller();
		let pool_id = Uni::<T>::do_create_pool(
			&owner,
			CurrencyPair::new(btc, usdt),
			1000_u16,
			Permill::from_float(0.002),
			Permill::from_float(0.01),
		) .expect("impossible; qed;");
		// 100_000_000 USDC , 100_000_000 USDT
		let initial_usdc: T::Balance = (100_000_000_u128).into();
		let initial_usdt: T::Balance = (100_000_000_u128).into();
		// Mint the tokens
		assert_ok!(T::Assets::mint_into(usdc, &owner, initial_usdc));
		assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
		// Add the liquidity
		assert_ok!(<StableSwap<T> as CurveAmm>::add_liquidity(
			&owner,
			pool_id,
			initial_btc,
			initial_usdt,
			0.into(),
			false
		));
	  let user = account("user", 0, 0);
		assert_ok!(T::Assets::mint_into(usdt, &user, (1000_u128).into()));
	// buy 1000 USDC
  }: _(RawOrigin::Signed(user), pool_id, usdc, 1000_u128.into(), false)

  sell {
	  let usdc: T::AssetId = 0.into();
	  let usdt: T::AssetId = 1.into();
	  let owner = whitelisted_caller();
		let pool_id = Uni::<T>::do_create_pool(
			&owner,
			CurrencyPair::new(btc, usdt),
			1000_u16,
			Permill::from_float(0.002),
			Permill::from_float(0.01),
		) .expect("impossible; qed;");
		// 100_000_000 USDC , 100_000_000 USDT
		let initial_usdc: T::Balance = (100_000_000_u128).into();
		let initial_usdt: T::Balance = (100_000_000_u128).into();
		// Mint the tokens
		assert_ok!(T::Assets::mint_into(usdc, &owner, initial_usdc));
		assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
		// Add the liquidity
		assert_ok!(<StableSwap<T> as CurveAmm>::add_liquidity(
			&owner,
			pool_id,
			initial_btc,
			initial_usdt,
			0.into(),
			false
		));
	  let user = account("user", 0, 0);
		assert_ok!(T::Assets::mint_into(usdc, &user, (1000_u128).into()));
	// sell 1000 USDC
  }: _(RawOrigin::Signed(user), pool_id, usdc, 1000_u128.into(), false)

  swap {
	  let usdc: T::AssetId = 0.into();
	  let usdt: T::AssetId = 1.into();
	  let owner = whitelisted_caller();
		let pool_id = Uni::<T>::do_create_pool(
			&owner,
			CurrencyPair::new(btc, usdt),
			1000_u16,
			Permill::from_float(0.002),
			Permill::from_float(0.01),
		) .expect("impossible; qed;");
		// 100_000_000 USDC , 100_000_000 USDT
		let initial_usdc: T::Balance = (100_000_000_u128).into();
		let initial_usdt: T::Balance = (100_000_000_u128).into();
		// Mint the tokens
		assert_ok!(T::Assets::mint_into(usdc, &owner, initial_usdc));
		assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
		// Add the liquidity
		assert_ok!(<StableSwap<T> as CurveAmm>::add_liquidity(
			&owner,
			pool_id,
			initial_btc,
			initial_usdt,
			0.into(),
			false
		));
	  let user = account("user", 0, 0);
		assert_ok!(T::Assets::mint_into(usdc, &user, (1000_u128).into()));
	// swap 1000 USDC
  }: _(RawOrigin::Signed(user), pool_id, pair, usdc, 1000_u128.into(), 0.into(), false)
}

impl_benchmark_test_suite!(StableSwap, crate::mock::new_test_ext(), crate::mock::Test);
