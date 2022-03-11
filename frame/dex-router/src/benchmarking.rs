use super::*;
use crate::{self as pallet_dex_router, Pallet as DexRouter};
use composable_traits::{
	defi::CurrencyPair,
	dex::{Amm, DexRouteNode},
};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{assert_ok, traits::fungibles::Mutate};
use frame_system::RawOrigin;
use sp_arithmetic::Permill;
use sp_std::{vec, vec::Vec};

benchmarks! {
  where_clause { where
	  T: pallet_dex_router::Config
		 + pallet_curve_amm::Config
		 + pallet_uniswap_v2::Config,
	  <T as pallet_curve_amm::Config>::Balance: From<u128>, <T as pallet_curve_amm::Config>::AssetId: From<u128>,
	  <T as pallet_uniswap_v2::Config>::Balance: From<u128>, <T as pallet_uniswap_v2::Config>::AssetId: From<u128>,
	  <T as pallet_dex_router::Config>::Balance: From<u128>, <T as pallet_dex_router::Config>::AssetId: From<u128>, <T as pallet_dex_router::Config>::PoolId: From<u128>,
  }
  // benchmark inserting new route
  update_route {
	let owner = whitelisted_caller();
	let amp_coeff = 5_u16;
	let fee = Permill::zero();
	let admin_fee = Permill::zero();
	let usdt: <T as pallet_curve_amm::Config>::AssetId = 100_u128.into();
	let usdc: <T as pallet_curve_amm::Config>::AssetId = 101_u128.into();
	let assets = CurrencyPair::new(usdt, usdc);
	let p = pallet_curve_amm::Pallet::<T>::do_create_pool(
			&owner,
			assets,
			amp_coeff,
			fee,
			admin_fee
		);
	assert_ok!(&p);
	let eth: <T as pallet_uniswap_v2::Config>::AssetId = 102_u128.into();
	let usdc: <T as pallet_uniswap_v2::Config>::AssetId = 101_u128.into();
	let assets = CurrencyPair::new(usdc, eth);
	let p = pallet_uniswap_v2::Pallet::<T>::do_create_pool(
		&owner,
		assets,
		fee,
		admin_fee
		);
	assert_ok!(&p);
	let dex_route = vec![
			DexRouteNode::Uniswap(0_u128.into()),
			DexRouteNode::Curve(0_u128.into()),
	];
	let eth: <T as pallet_dex_router::Config>::AssetId = 102_u128.into();
	let usdt: <T as pallet_dex_router::Config>::AssetId = 100_u128.into();
	let currency_pair = CurrencyPair::new(eth, usdt);
  } : _(RawOrigin::Signed(owner), currency_pair, Some(dex_route.clone().try_into().unwrap()))

  exchange {
	let unit = 1_000_000_000_000_u128;
	let owner = whitelisted_caller();
	let amp_coeff = 100_u16;
	let fee = Permill::zero();
	let admin_fee = Permill::zero();
	let usdt: <T as pallet_curve_amm::Config>::AssetId = 100_u128.into();
	let usdc: <T as pallet_curve_amm::Config>::AssetId = 101_u128.into();
	let assets = CurrencyPair::new(usdt, usdc);
	let p = pallet_curve_amm::Pallet::<T>::do_create_pool(
			&owner,
			assets,
			amp_coeff,
			fee,
			admin_fee
		);
	let stable_swap_pool_id = p.expect("curve_amm pool creation failed");
	let usdt_amount = 1_000_000_000_u128 * unit;
	let usdc_amount = 1_000_000_000_u128 * unit;
	<T as pallet_curve_amm::Config>::Assets::mint_into(usdt, &owner, usdt_amount.into()).expect("Mint USDT failed");
	<T as pallet_curve_amm::Config>::Assets::mint_into(usdc, &owner, usdc_amount.into()).expect("Mint USDC failed");
	<pallet_curve_amm::Pallet::<T> as Amm>::add_liquidity(
			&owner,
			stable_swap_pool_id,
			usdt_amount.into(),
			usdc_amount.into(),
			0_u128.into(),
			false,
	).expect("curve_amm add_liquidity failed");
	let eth: <T as pallet_uniswap_v2::Config>::AssetId = 102_u128.into();
	let usdc: <T as pallet_uniswap_v2::Config>::AssetId = 101_u128.into();
	let assets = CurrencyPair::new(usdc, eth);
	let p = pallet_uniswap_v2::Pallet::<T>::do_create_pool(
		&owner,
		assets,
		fee,
		admin_fee
		);
	assert_ok!(&p);
	let uniswap_pool_id = p.expect("uniswap_v2 pool creation failed");
	let eth_amount = 1_000_000_000_u128 * unit;
	let usdc_amount = eth_amount * 3000_u128;
	<T as pallet_uniswap_v2::Config>::Assets::mint_into(eth, &owner, eth_amount.into()).expect("Mint ETH failed");
	<T as pallet_uniswap_v2::Config>::Assets::mint_into(usdc, &owner, usdc_amount.into()).expect("Mint USDC failed");
	<pallet_uniswap_v2::Pallet::<T> as Amm> ::add_liquidity(
			&owner,
			uniswap_pool_id,
			usdc_amount.into(),
			eth_amount.into(),
			0_u128.into(),
			false,
	).expect("uniswap_v2 add_liquidity failed");
	// add more ETH used for exchange.
	<T as pallet_uniswap_v2::Config>::Assets::mint_into(eth, &owner, eth_amount.into()).expect("Mint ETH failed");
	let eth: <T as pallet_dex_router::Config>::AssetId = 102_u128.into();
	let usdt: <T as pallet_dex_router::Config>::AssetId = 100_u128.into();
	let currency_pair = CurrencyPair::new(usdt, eth);
	let dex_route : Vec<DexRouteNode<<T as pallet_dex_router::Config>::PoolId>> = vec![
			DexRouteNode::Uniswap(0_u128.into()),
			DexRouteNode::Curve(0_u128.into()),
	];
	let origin = RawOrigin::Signed(owner.clone());
	pallet_dex_router::Pallet::<T>::update_route(origin.clone().into(), currency_pair, Some(dex_route.clone().try_into().unwrap())).expect("insert route failed");
	// exchange 1 ETH via route
  } : _((RawOrigin::Signed(owner)), currency_pair, (1_u128 * unit).into(), 0_u128.into())

  buy {
	let unit = 1_000_000_000_000_u128;
	let owner = whitelisted_caller();
	let amp_coeff = 100_u16;
	let fee = Permill::zero();
	let admin_fee = Permill::zero();
	let usdt: <T as pallet_curve_amm::Config>::AssetId = 100_u128.into();
	let usdc: <T as pallet_curve_amm::Config>::AssetId = 101_u128.into();
	let assets = CurrencyPair::new(usdt, usdc);
	let p = pallet_curve_amm::Pallet::<T>::do_create_pool(
			&owner,
			assets,
			amp_coeff,
			fee,
			admin_fee
		);
	let stable_swap_pool_id = p.expect("curve_amm pool creation failed");
	let usdt_amount = 1_000_000_000_u128 * unit;
	let usdc_amount = 1_000_000_000_u128 * unit;
	<T as pallet_curve_amm::Config>::Assets::mint_into(usdt, &owner, usdt_amount.into()).expect("Mint USDT failed");
	<T as pallet_curve_amm::Config>::Assets::mint_into(usdc, &owner, usdc_amount.into()).expect("Mint USDC failed");
	<pallet_curve_amm::Pallet::<T> as Amm>::add_liquidity(
			&owner,
			stable_swap_pool_id,
			usdt_amount.into(),
			usdc_amount.into(),
			0_u128.into(),
			false,
	).expect("curve_amm add_liquidity failed");
	let eth: <T as pallet_uniswap_v2::Config>::AssetId = 102_u128.into();
	let usdc: <T as pallet_uniswap_v2::Config>::AssetId = 101_u128.into();
	let assets = CurrencyPair::new(usdc, eth);
	let p = pallet_uniswap_v2::Pallet::<T>::do_create_pool(
		&owner,
		assets,
		fee,
		admin_fee
		);
	assert_ok!(&p);
	let uniswap_pool_id = p.expect("uniswap_v2 pool creation failed");
	let eth_amount = 1_000_000_000_u128 * unit;
	let usdc_amount = eth_amount * 3000_u128;
	<T as pallet_uniswap_v2::Config>::Assets::mint_into(eth, &owner, eth_amount.into()).expect("Mint ETH failed");
	<T as pallet_uniswap_v2::Config>::Assets::mint_into(usdc, &owner, usdc_amount.into()).expect("Mint USDC failed");
	<pallet_uniswap_v2::Pallet::<T> as Amm> ::add_liquidity(
			&owner,
			uniswap_pool_id,
			usdc_amount.into(),
			eth_amount.into(),
			0_u128.into(),
			false,
	).expect("uniswap_v2 add_liquidity failed");
	// add more ETH used for exchange.
	<T as pallet_uniswap_v2::Config>::Assets::mint_into(eth, &owner, eth_amount.into()).expect("Mint ETH failed");
	let eth: <T as pallet_dex_router::Config>::AssetId = 102_u128.into();
	let usdt: <T as pallet_dex_router::Config>::AssetId = 100_u128.into();
	let currency_pair = CurrencyPair::new(eth, usdt);
	let dex_route : Vec<DexRouteNode<<T as pallet_dex_router::Config>::PoolId>> = vec![
			DexRouteNode::Uniswap(0_u128.into()),
			DexRouteNode::Curve(0_u128.into()),
	];
	let origin = RawOrigin::Signed(owner.clone());
	pallet_dex_router::Pallet::<T>::update_route(origin.clone().into(), currency_pair, Some(dex_route.clone().try_into().unwrap())).expect("insert route failed");
	// buy 3000 USDT via route
  } : _((RawOrigin::Signed(owner)), currency_pair, (3000_u128 * unit).into(), 0_u128.into())
}

impl_benchmark_test_suite!(DexRouter, crate::mock::new_test_ext(), crate::mock::Test);
