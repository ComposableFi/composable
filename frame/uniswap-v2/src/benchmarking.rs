use super::*;
use crate::Pallet as Uni;
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
	  let btc: T::AssetId = 100.into();
	  let usdt: T::AssetId = 101.into();
	  let owner = whitelisted_caller();
	  let pair = CurrencyPair::new(btc, usdt);
	  let fee = Permill::from_percent(1);
	  let owner_fee = Permill::from_percent(1);
  }: _(RawOrigin::Signed(owner), pair, fee, owner_fee)

  add_liquidity {
	  let btc: T::AssetId = 100.into();
	  let usdt: T::AssetId = 101.into();
	  let owner = whitelisted_caller();
	  let pool_id = Uni::<T>::do_create_pool(
		  &owner,
		  CurrencyPair::new(btc, usdt),
		  Permill::from_percent(1),
		  Permill::from_percent(1),
	  ) .expect("impossible; qed;");
	  let unit = 1_000_000_000_000;
	  let btc_price = 45_000;
	  let nb_of_btc = 100;
	  // 100 btc/4.5M usdt
	  let initial_btc: T::Balance = (nb_of_btc * unit).into();
	  let initial_usdt: T::Balance = (nb_of_btc * btc_price * unit).into();
	  // Mint the tokens
	  assert_ok!(T::Assets::mint_into(btc, &owner, initial_btc));
	  assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
  }: _(RawOrigin::Signed(owner), pool_id, initial_btc, initial_usdt, 0.into(), false)

  remove_liquidity {
	  let btc: T::AssetId = 100.into();
	  let usdt: T::AssetId = 101.into();
	  let owner = whitelisted_caller();
	  let pool_id = Uni::<T>::do_create_pool(
		  &owner,
		  CurrencyPair::new(btc, usdt),
		  Permill::from_percent(1),
		  Permill::from_percent(1),
	  ) .expect("impossible; qed;");
	  let unit = 1_000_000_000_000;
	  let btc_price = 45_000;
	  let nb_of_btc = 100;
	  // 100 btc/4.5M usdt
	  let initial_btc: T::Balance = (nb_of_btc * unit).into();
	  let initial_usdt: T::Balance = (nb_of_btc * btc_price * unit).into();
	  // Mint the tokens
	  assert_ok!(T::Assets::mint_into(btc, &owner, initial_btc));
	  assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
	  // Add the liquidity
	  assert_ok!(<Uni<T> as Amm>::add_liquidity(
			  &owner,
			  pool_id,
			  initial_btc,
			  initial_usdt,
			  0.into(),
			  false
	  ));
	  let pool_info = Uni::<T>::get_pool(pool_id).expect("impossible; qed;");
	  let lp_amount = T::Assets::balance(pool_info.lp_token, &owner);
  }: _(RawOrigin::Signed(owner), pool_id, lp_amount, (0_u128).into(), (0_u128).into())

  buy {
	  let btc: T::AssetId = 100.into();
	  let usdt: T::AssetId = 101.into();
	  let owner = whitelisted_caller();
	  let pool_id = Uni::<T>::do_create_pool(
		  &owner,
		  CurrencyPair::new(btc, usdt),
		  Permill::from_percent(1),
		  Permill::from_percent(1),
	  ) .expect("impossible; qed;");
	  let unit = 1_000_000_000_000;
	  let btc_price = 45_000;
	  let nb_of_btc = 100;
	  // 100 btc/4.5M usdt
	  let initial_btc: T::Balance = (nb_of_btc * unit).into();
	  let initial_usdt: T::Balance = (nb_of_btc * btc_price * unit).into();
	  // Mint the tokens
	  assert_ok!(T::Assets::mint_into(btc, &owner, initial_btc));
	  assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
	  // Add the liquidity
	  assert_ok!(<Uni<T> as Amm>::add_liquidity(
			  &owner,
			  pool_id,
			  initial_btc,
			  initial_usdt,
			  0.into(),
			  false
	  ));
	  let user = account("user", 0, 0);
	  let price = <Uni<T> as Amm>::get_exchange_value(pool_id, btc, unit.into())?;
	  assert_ok!(T::Assets::mint_into(usdt, &user, price));
	  // buy 1 btc
  }: _(RawOrigin::Signed(user), pool_id, btc, unit.into(), false)

 sell {
	  let btc: T::AssetId = 100.into();
	  let usdt: T::AssetId = 101.into();
	  let owner = whitelisted_caller();
	  let pool_id = Uni::<T>::do_create_pool(
		  &owner,
		  CurrencyPair::new(btc, usdt),
		  Permill::from_percent(1),
		  Permill::from_percent(1),
	  ) .expect("impossible; qed;");
	  let unit = 1_000_000_000_000;
	  let btc_price = 45_000;
	  let nb_of_btc = 100;
	  // 100 btc/4.5M usdt
	  let initial_btc: T::Balance = (nb_of_btc * unit).into();
	  let initial_usdt: T::Balance = (nb_of_btc * btc_price * unit).into();
	  // Mint the tokens
	  assert_ok!(T::Assets::mint_into(btc, &owner, initial_btc));
	  assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
	  // Add the liquidity
	  assert_ok!(<Uni<T> as Amm>::add_liquidity(
			  &owner,
			  pool_id,
			  initial_btc,
			  initial_usdt,
			  0.into(),
			  false
	  ));
	  let user = account("user", 0, 0);
	  assert_ok!(T::Assets::mint_into(btc, &user, unit.into()));
 }: _(RawOrigin::Signed(user), pool_id, btc, unit.into(), false)

 swap {
	  let btc: T::AssetId = 100.into();
	  let usdt: T::AssetId = 101.into();
	  let owner = whitelisted_caller();
	  let pair = CurrencyPair::new(btc, usdt);
	  let pool_id = Uni::<T>::do_create_pool(
		  &owner,
		  pair,
		  Permill::from_percent(1),
		  Permill::from_percent(1),
	  ) .expect("impossible; qed;");
	  let unit = 1_000_000_000_000;
	  let btc_price = 45_000;
	  let nb_of_btc = 100;
	  // 100 btc/4.5M usdt
	  let initial_btc: T::Balance = (nb_of_btc * unit).into();
	  let initial_usdt: T::Balance = (nb_of_btc * btc_price * unit).into();
	  // Mint the tokens
	  assert_ok!(T::Assets::mint_into(btc, &owner, initial_btc));
	  assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
	  // Add the liquidity
	  assert_ok!(<Uni<T> as Amm>::add_liquidity(
			  &owner,
			  pool_id,
			  initial_btc,
			  initial_usdt,
			  0.into(),
			  false
	  ));
	  let user = account("user", 0, 0);
	  assert_ok!(T::Assets::mint_into(btc, &user, unit.into()));
 }: _(RawOrigin::Signed(user), pool_id, pair.swap(), unit.into(), 0.into(), false)
}

impl_benchmark_test_suite!(Uni, crate::mock::new_test_ext(), crate::mock::Test);
