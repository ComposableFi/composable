use super::*;
use crate::{
	Pallet as Pablo,
	PoolConfiguration::{ConstantProduct, LiquidityBootstrapping, StableSwap},
};
use composable_traits::{
	defi::CurrencyPair,
	dex::{Amm, FeeConfig, LiquidityBootstrappingPoolInfo, Sale},
};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{
	assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use frame_system::RawOrigin;
use sp_arithmetic::Permill;

fn stable_swap_init_config<T: Config>(
	owner: T::AccountId,
	pair: CurrencyPair<T::AssetId>,
	amplification_coefficient: u16,
	fee: Permill,
) -> PoolInitConfigurationOf<T> {
	PoolInitConfiguration::StableSwap { owner, pair, amplification_coefficient, fee }
}

fn create_stable_swap_pool<T: Config>(
	owner: T::AccountId,
	pair: CurrencyPair<T::AssetId>,
) -> T::PoolId {
	let stable_swap_pool_init =
		stable_swap_init_config::<T>(owner.clone(), pair, 1000_u16, Permill::from_percent(1));
	Pablo::<T>::do_create_pool(stable_swap_pool_init).expect("impossible; qed;")
}

fn get_lp_token<T: Config>(pool_id: T::PoolId) -> T::AssetId {
	let pool_info = Pablo::<T>::get_pool(pool_id).expect("impossible; qed;");
	match pool_info {
		StableSwap(pool) => pool.lp_token,
		ConstantProduct(pool) => pool.lp_token,
		LiquidityBootstrapping(_) => panic!("Not implemented"),
	}
}

benchmarks! {
  where_clause { where T::BlockNumber: From<u32>, T::Balance: From<u128>, T::AssetId: From<u128> }
	create {
		let usdc: T::AssetId = 100.into();
		let usdt: T::AssetId = 101.into();
		let owner: T::AccountId = whitelisted_caller();
		let pair = CurrencyPair::new(usdc, usdt);
		let amplification_factor = 100_u16;
		let fee = Permill::from_percent(1);
		let protocol_fee = Permill::from_percent(1);
		let stable_swap_pool_init = stable_swap_init_config::<T>(owner.clone(), pair, amplification_factor, fee);
	  } : _(RawOrigin::Root, stable_swap_pool_init)

	create_lbp {
		let unit = 1_000_000_000_000u128;
		let project_token: T::AssetId = 9999.into();
		let usdt: T::AssetId = 1.into();
		let pair = CurrencyPair::new(project_token, usdt);
		let owner: T::AccountId = whitelisted_caller();
		let fee = Permill::from_perthousand(1);
		let pool = LiquidityBootstrappingPoolInfo {
			owner: owner.clone(),
			pair,
			sale: Sale {
				start: T::BlockNumber::from(100u32),
				end: T::BlockNumber::from(21600u32 + 100u32),
				initial_weight: Permill::from_percent(92),
				final_weight: Permill::from_percent(50),
			},
			fee_config: FeeConfig {
					fee_rate: fee,
					owner_fee_rate: Permill::zero(),
					protocol_fee_rate: Permill::zero()
			}
		};
	  }: create(RawOrigin::Root, PoolInitConfiguration::LiquidityBootstrapping(pool))

	add_liquidity {
		let usdc: T::AssetId = 100.into();
		let usdt: T::AssetId = 101.into();
		let owner: T::AccountId = whitelisted_caller();
		let pool_id = create_stable_swap_pool::<T>(owner.clone(), CurrencyPair::new(usdc, usdt));
		let unit = 1_000_000_000_000;
		// 100_000_000 USDC , 100_000_000 USDT
		let initial_usdc: T::Balance = (100_000_000_u128 * unit).into();
		let initial_usdt: T::Balance = (100_000_000_u128 * unit).into();
		// Mint the tokens
		assert_ok!(T::Assets::mint_into(usdc, &owner, initial_usdc));
		assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
	  }: _(RawOrigin::Signed(owner), pool_id, initial_usdc, initial_usdt, 0.into(), false)

	add_liquidity_lbp {
		let unit = 1_000_000_000_000u128;
		let project_token: T::AssetId = 9999.into();
		let usdt: T::AssetId = 1.into();
		let pair = CurrencyPair::new(project_token, usdt);
		let owner: T::AccountId = whitelisted_caller();
		let fee = Permill::from_perthousand(1);
		let pool = LiquidityBootstrappingPoolInfo {
			owner: owner.clone(),
			pair,
			sale: Sale {
				start: T::BlockNumber::from(100u32),
				end: T::BlockNumber::from(21600u32 + 100u32),
				initial_weight: Permill::from_percent(92),
				final_weight: Permill::from_percent(50),
			},
			fee_config: FeeConfig {
					fee_rate: fee,
					owner_fee_rate: Permill::zero(),
					protocol_fee_rate: Permill::zero()
			}
		};
		let pool_id = Pablo::<T>::do_create_pool(
			PoolInitConfiguration::LiquidityBootstrapping(pool)
		).expect("impossible; qed;");
		let nb_of_project_tokens = 200_000_000;
		let nb_of_usdt = 5_000_000;
		let initial_project_tokens: T::Balance = (nb_of_project_tokens * unit).into();
		let initial_usdt: T::Balance = (nb_of_usdt * unit).into();
		// Mint the tokens
		assert_ok!(T::Assets::mint_into(project_token, &owner, initial_project_tokens));
		assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
	}: add_liquidity(RawOrigin::Signed(owner), pool_id, initial_project_tokens, initial_usdt, 0.into(), false)

	remove_liquidity {
		let usdc: T::AssetId = 100.into();
		let usdt: T::AssetId = 101.into();
		let owner: T::AccountId = whitelisted_caller();
		let pool_id = create_stable_swap_pool::<T>(owner.clone(), CurrencyPair::new(usdc, usdt));
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

	remove_liquidity_lbp {
		let unit = 1_000_000_000_000u128;
		let project_token: T::AssetId = 9999.into();
		let usdt: T::AssetId = 1.into();
		let pair = CurrencyPair::new(project_token, usdt);
		let owner: T::AccountId = whitelisted_caller();
		let fee = Permill::from_perthousand(1);
		let start = T::BlockNumber::from(100u32);
		let end = T::BlockNumber::from(21600u32 + 100u32);
		let pool = LiquidityBootstrappingPoolInfo {
			owner: owner.clone(),
			pair,
			sale: Sale {
				start: start,
				end: end,
				initial_weight: Permill::from_percent(92),
				final_weight: Permill::from_percent(50),
			},
			fee_config: FeeConfig {
					fee_rate: fee,
					owner_fee_rate: Permill::zero(),
					protocol_fee_rate: Permill::zero()
			}
		};
		let pool_id = Pablo::<T>::do_create_pool(
			PoolInitConfiguration::LiquidityBootstrapping(pool)
		).expect("impossible; qed;");
		let nb_of_project_tokens = 200_000_000;
		let nb_of_usdt = 5_000_000;
		let initial_project_tokens: T::Balance = (nb_of_project_tokens * unit).into();
		let initial_usdt: T::Balance = (nb_of_usdt * unit).into();
		// Mint the tokens
		assert_ok!(T::Assets::mint_into(project_token, &owner, initial_project_tokens));
		assert_ok!(T::Assets::mint_into(usdt, &owner, initial_usdt));
		assert_ok!(<Pablo<T> as Amm>::add_liquidity(
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
	}: remove_liquidity(RawOrigin::Signed(owner), pool_id, (0_u128).into(), (0_u128).into(), (0_u128).into())

	buy {
		let usdc: T::AssetId = 100.into();
		let usdt: T::AssetId = 101.into();
		let owner: T::AccountId = whitelisted_caller();
		let pool_id = create_stable_swap_pool::<T>(owner.clone(), CurrencyPair::new(usdc, usdt));
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
		let pool_id = create_stable_swap_pool::<T>(owner.clone(), CurrencyPair::new(usdc, usdt));
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
		let pool_id = create_stable_swap_pool::<T>(owner.clone(), CurrencyPair::new(usdc, usdt));

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
