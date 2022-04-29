use super::*;
use crate::Pallet as LBP;
use composable_support::validation::Validated;
use composable_traits::{defi::CurrencyPair, dex::Amm};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{assert_ok, traits::fungibles::Mutate};
use frame_system::RawOrigin;
use sp_arithmetic::Permill;

benchmarks! {
  where_clause { where T::BlockNumber: From<u32>, T::Balance: From<u128>, T::AssetId: From<u128> }

  create {
	  let unit = 1_000_000_000_000u128;
	  let project_token: T::AssetId = 0.into();
	  let usdt: T::AssetId = 1.into();
	  let pair = CurrencyPair::new(project_token, usdt);
	  let owner: T::AccountId = whitelisted_caller();
	  let fee = Permill::from_perthousand(1);
	  let pool = Validated::new(Pool {
		  owner: owner.clone(),
		  pair,
		  sale: Sale {
			  start: T::BlockNumber::from(100u32),
			  end: T::BlockNumber::from(21600u32 + 100u32),
			  initial_weight: Permill::from_percent(92),
			  final_weight: Permill::from_percent(50),
		  },
		  fee
	  }).expect("impossible; qed;");
  }: _(RawOrigin::Root, pool)

	buy {
		let unit = 1_000_000_000_000u128;
		let project_token: T::AssetId = 0.into();
		let usdt: T::AssetId = 1.into();
		let pair = CurrencyPair::new(project_token, usdt);
		let owner: T::AccountId = whitelisted_caller();
		let fee = Permill::from_perthousand(1);
		let pool = Validated::new(Pool {
			owner: owner.clone(),
			pair,
			sale: Sale {
				start: T::BlockNumber::from(100u32),
				end: T::BlockNumber::from(21600u32 + 100u32),
				initial_weight: Permill::from_percent(92),
				final_weight: Permill::from_percent(50),
			},
			fee
		}).expect("impossible; qed;");
		let pool_id = LBP::<T>::do_create_pool(
			pool
		) .expect("impossible; qed;");
		let nb_of_project_tokens = 200_000_000;
		let nb_of_usdt = 5_000_000;
		let initial_project_tokens: T::Balance = (nb_of_project_tokens * unit).into();
		let initial_usdt: T::Balance = (nb_of_usdt * unit).into();
		// Mint the tokens
		assert_ok!(T::Assets::mint_into(project_token, &owner, initial_project_tokens));
		assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
		assert_ok!(<LBP<T> as Amm>::add_liquidity(
				&owner,
				pool_id,
				initial_project_tokens,
				initial_usdt,
				0.into(),
				false
		));
		let user = account("user", 0, 0);
		assert_ok!(T::Assets::mint_into(usdt, &user, unit.into()));
		frame_system::Pallet::<T>::set_block_number(1000.into());
	}: _(RawOrigin::Signed(user.clone()), pool_id, project_token, unit.into(), 0_u128.into(), false)

	  sell {
		  let unit = 1_000_000_000_000u128;
		  let project_token: T::AssetId = 0.into();
		  let usdt: T::AssetId = 1.into();
		  let pair = CurrencyPair::new(project_token, usdt);
		  let owner: T::AccountId = whitelisted_caller();
		  let fee = Permill::from_perthousand(1);
		  let pool = Validated::new(Pool {
			  owner: owner.clone(),
			  pair,
			  sale: Sale {
				  start: T::BlockNumber::from(100u32),
				  end: T::BlockNumber::from(21600u32 + 100u32),
				  initial_weight: Permill::from_percent(92),
				  final_weight: Permill::from_percent(50),
			  },
			  fee
		  }).expect("impossible; qed;");
		  let pool_id = LBP::<T>::do_create_pool(
			  pool
		  ) .expect("impossible; qed;");
		  let nb_of_project_tokens = 200_000_000;
		  let nb_of_usdt = 5_000_000;
		  let initial_project_tokens: T::Balance = (nb_of_project_tokens * unit).into();
		  let initial_usdt: T::Balance = (nb_of_usdt * unit).into();
		  // Mint the tokens
		  assert_ok!(T::Assets::mint_into(project_token, &owner, initial_project_tokens));
		  assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
		  assert_ok!(<LBP<T> as Amm>::add_liquidity(
				  &owner,
				  pool_id,
				  initial_project_tokens,
				  initial_usdt,
				  0.into(),
				  false
		  ));
		  let user = account("user", 0, 0);
		  assert_ok!(T::Assets::mint_into(project_token, &user, unit.into()));
		  frame_system::Pallet::<T>::set_block_number(1000.into());
	  }: _(RawOrigin::Signed(user), pool_id, project_token, unit.into(), 0_u128.into(), false)

	swap {
		let unit = 1_000_000_000_000u128;
		let project_token: T::AssetId = 0.into();
		let usdt: T::AssetId = 1.into();
		let pair = CurrencyPair::new(project_token, usdt);
		let owner: T::AccountId = whitelisted_caller();
		let fee = Permill::from_perthousand(1);
		let pool = Validated::new(Pool {
			owner: owner.clone(),
			pair,
			sale: Sale {
				start: T::BlockNumber::from(100u32),
				end: T::BlockNumber::from(21600u32 + 100u32),
				initial_weight: Permill::from_percent(92),
				final_weight: Permill::from_percent(50),
			},
			fee
		}).expect("impossible; qed;");
		let pool_id = LBP::<T>::do_create_pool(
			pool
		) .expect("impossible; qed;");
		let nb_of_project_tokens = 200_000_000;
		let nb_of_usdt = 5_000_000;
		let initial_project_tokens: T::Balance = (nb_of_project_tokens * unit).into();
		let initial_usdt: T::Balance = (nb_of_usdt * unit).into();
		// Mint the tokens
		assert_ok!(T::Assets::mint_into(project_token, &owner, initial_project_tokens));
		assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
		assert_ok!(<LBP<T> as Amm>::add_liquidity(
				&owner,
				pool_id,
				initial_project_tokens,
				initial_usdt,
				0.into(),
				false
		));
		let user = account("user", 0, 0);
		assert_ok!(T::Assets::mint_into(usdt, &user, unit.into()));
		frame_system::Pallet::<T>::set_block_number(1000.into());
	  }: _(RawOrigin::Signed(user), pool_id, pair, unit.into(), 0.into(), false)

	add_liquidity {
		let unit = 1_000_000_000_000u128;
		let project_token: T::AssetId = 0.into();
		let usdt: T::AssetId = 1.into();
		let pair = CurrencyPair::new(project_token, usdt);
		let owner: T::AccountId = whitelisted_caller();
		let fee = Permill::from_perthousand(1);
		let pool = Validated::new(Pool {
			owner: owner.clone(),
			pair,
			sale: Sale {
				start: T::BlockNumber::from(100u32),
				end: T::BlockNumber::from(21600u32 + 100u32),
				initial_weight: Permill::from_percent(92),
				final_weight: Permill::from_percent(50),
			},
			fee
		}).expect("impossible; qed;");
		let pool_id = LBP::<T>::do_create_pool(
			pool
		) .expect("impossible; qed;");
		let nb_of_project_tokens = 200_000_000;
		let nb_of_usdt = 5_000_000;
		let initial_project_tokens: T::Balance = (nb_of_project_tokens * unit).into();
		let initial_usdt: T::Balance = (nb_of_usdt * unit).into();
		// Mint the tokens
		assert_ok!(T::Assets::mint_into(project_token, &owner, initial_project_tokens));
		assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
	}: _(RawOrigin::Signed(owner), pool_id, initial_project_tokens, initial_usdt, false)

	remove_liquidity {
		let unit = 1_000_000_000_000u128;
		let project_token: T::AssetId = 0.into();
		let usdt: T::AssetId = 1.into();
		let pair = CurrencyPair::new(project_token, usdt);
		let owner: T::AccountId = whitelisted_caller();
		let fee = Permill::from_perthousand(1);
		let start = T::BlockNumber::from(100u32);
		let end = T::BlockNumber::from(21600u32 + 100u32);
		let pool = Validated::new(Pool {
			owner: owner.clone(),
			pair,
			sale: Sale {
				start,
				end,
				initial_weight: Permill::from_percent(92),
				final_weight: Permill::from_percent(50),
			},
			fee
		}).expect("impossible; qed;");
		let pool_id = LBP::<T>::do_create_pool(
			pool
		) .expect("impossible; qed;");
		let nb_of_project_tokens = 200_000_000;
		let nb_of_usdt = 5_000_000;
		let initial_project_tokens: T::Balance = (nb_of_project_tokens * unit).into();
		let initial_usdt: T::Balance = (nb_of_usdt * unit).into();
		// Mint the tokens
		assert_ok!(T::Assets::mint_into(project_token, &owner, initial_project_tokens));
		assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
		assert_ok!(<LBP<T> as Amm>::add_liquidity(
				&owner,
				pool_id,
				initial_project_tokens,
				initial_usdt,
				0.into(),
				false
		));
		let user = account("user", 0, 0);
		assert_ok!(T::Assets::mint_into(usdt, &user, unit.into()));
		frame_system::Pallet::<T>::set_block_number(end);
	}: _(RawOrigin::Signed(owner), pool_id)
}

impl_benchmark_test_suite!(LBP, crate::mock::new_test_ext(), crate::mock::Test);
