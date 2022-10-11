use super::*;
use crate::{self as pallet_dex_router, Pallet as DexRouter};
use composable_traits::{defi::CurrencyPair, dex::Amm};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::traits::fungibles::Mutate;
use frame_system::RawOrigin;
use sp_arithmetic::Permill;
use sp_std::{vec, vec::Vec};

fn create_single_node_pool<T>() -> (
	CurrencyPair<<T as pallet_dex_router::Config>::AssetId>,
	Vec<<T as pallet_dex_router::Config>::PoolId>,
)
where
	T: pallet_dex_router::Config + pallet_pablo::Config,
	<T as pallet_dex_router::Config>::Balance: From<u128>,
	<T as pallet_dex_router::Config>::AssetId: From<u128>,
	<T as pallet_dex_router::Config>::PoolId: From<u128>,
	<T as pallet_pablo::Config>::Balance: From<u128>,
	<T as pallet_pablo::Config>::AssetId: From<u128>,
{
	let unit = 1_000_000_000_000_u128;
	let usdc: <T as pallet_pablo::Config>::AssetId = 100_u128.into();
	let usdt: <T as pallet_pablo::Config>::AssetId = 104_u128.into();
	let owner: <T as frame_system::Config>::AccountId = whitelisted_caller();
	let usdc_usdt_config = pallet_pablo::PoolInitConfiguration::ConstantProduct {
		owner: owner.clone(),
		pair: CurrencyPair::new(usdc, usdt),
		fee: Permill::zero(),
		base_weight: Permill::from_percent(50_u32),
	};
	let usdc_usdt = pallet_pablo::Pallet::<T>::do_create_pool(usdc_usdt_config).unwrap();
	sp_std::if_std! {
		println!(" usdc_usdt {:?}", usdc_usdt);
	}
	// 1 usdc == 1 usdt
	let usdc_amount = 1000 * unit;
	let usdt_amount = 1000 * unit;
	<T as pallet_pablo::Config>::Assets::mint_into(usdc, &owner, usdc_amount.into())
		.expect("mint usdc failed");
	<T as pallet_pablo::Config>::Assets::mint_into(usdt, &owner, usdt_amount.into())
		.expect("mint usdt failed");
	let dex_route = vec![0_u128.into()];
	let usdc: <T as pallet_dex_router::Config>::AssetId = 100_u128.into();
	let usdt: <T as pallet_dex_router::Config>::AssetId = 104_u128.into();
	let currency_pair = CurrencyPair::new(usdc, usdt);
	(currency_pair, dex_route)
}

fn create_pools_route<T>() -> (
	CurrencyPair<<T as pallet_dex_router::Config>::AssetId>,
	Vec<<T as pallet_dex_router::Config>::PoolId>,
)
where
	T: pallet_dex_router::Config + pallet_pablo::Config,
	<T as pallet_dex_router::Config>::Balance: From<u128>,
	<T as pallet_dex_router::Config>::AssetId: From<u128>,
	<T as pallet_dex_router::Config>::PoolId: From<u128>,
	<T as pallet_pablo::Config>::Balance: From<u128>,
	<T as pallet_pablo::Config>::AssetId: From<u128>,
{
	let unit = 1_000_000_000_000_u128;
	let owner: <T as frame_system::Config>::AccountId = whitelisted_caller();
	let pica: <T as pallet_pablo::Config>::AssetId = 100_u128.into();
	let ksm: <T as pallet_pablo::Config>::AssetId = 101_u128.into();
	let eth: <T as pallet_pablo::Config>::AssetId = 102_u128.into();
	let usdc: <T as pallet_pablo::Config>::AssetId = 103_u128.into();
	let usdt: <T as pallet_pablo::Config>::AssetId = 104_u128.into();
	let pica_ksm_config = pallet_pablo::PoolInitConfiguration::ConstantProduct {
		owner: owner.clone(),
		pair: CurrencyPair::new(pica, ksm),
		fee: Permill::zero(),
		base_weight: Permill::from_percent(50),
	};
	let pica_ksm = pallet_pablo::Pallet::<T>::do_create_pool(pica_ksm_config).unwrap();
	// 100 pica == 1 ksm
	let pica_amount = 100 * 1000 * unit;
	let ksm_amount = 1 * 1000 * unit;
	<T as pallet_pablo::Config>::Assets::mint_into(pica, &owner, pica_amount.into())
		.expect("mint pica failed");
	<T as pallet_pablo::Config>::Assets::mint_into(ksm, &owner, ksm_amount.into())
		.expect("mint ksm failed");
	<pallet_pablo::Pallet<T> as Amm>::add_liquidity(
		&owner,
		pica_ksm,
		pica_amount.into(),
		ksm_amount.into(),
		0_u128.into(),
		false,
	)
	.expect("add_liquidity to pica_ksm_pool failed");
	sp_std::if_std! {
		println!(" pica_ksm {:?}", pica_ksm);
	}
	let ksm_eth_config = pallet_pablo::PoolInitConfiguration::ConstantProduct {
		owner: owner.clone(),
		pair: CurrencyPair::new(ksm, eth),
		fee: Permill::zero(),
		base_weight: Permill::from_percent(50),
	};
	let ksm_eth = pallet_pablo::Pallet::<T>::do_create_pool(ksm_eth_config).unwrap();
	// 10 ksm == 1 eth
	let ksm_amount = 10 * 1000 * unit;
	let eth_amount = 1 * 1000 * unit;
	<T as pallet_pablo::Config>::Assets::mint_into(ksm, &owner, ksm_amount.into())
		.expect("mint ksm failed");
	<T as pallet_pablo::Config>::Assets::mint_into(eth, &owner, eth_amount.into())
		.expect("mint eth failed");
	<pallet_pablo::Pallet<T> as Amm>::add_liquidity(
		&owner,
		ksm_eth,
		ksm_amount.into(),
		eth_amount.into(),
		0_u128.into(),
		false,
	)
	.expect("add_liquidity to ksm_eth_pool failed");
	sp_std::if_std! {
		println!(" ksm_eth {:?}", ksm_eth);
	}

	let eth_usdc_config = pallet_pablo::PoolInitConfiguration::ConstantProduct {
		owner: owner.clone(),
		pair: CurrencyPair::new(eth, usdc),
		fee: Permill::zero(),
		base_weight: Permill::from_percent(50),
	};
	let eth_usdc = pallet_pablo::Pallet::<T>::do_create_pool(eth_usdc_config).unwrap();
	// 1 eth = 200 usdc
	let eth_amount = 1 * 1000 * unit;
	let usdc_amount = 200 * 1000 * unit;
	<T as pallet_pablo::Config>::Assets::mint_into(eth, &owner, eth_amount.into())
		.expect("mint eth failed");
	<T as pallet_pablo::Config>::Assets::mint_into(usdc, &owner, usdc_amount.into())
		.expect("mint usdc failed");
	<pallet_pablo::Pallet<T> as Amm>::add_liquidity(
		&owner,
		eth_usdc,
		eth_amount.into(),
		usdc_amount.into(),
		0_u128.into(),
		false,
	)
	.expect("add_liquidity to eth_usdc_pool failed");

	sp_std::if_std! {
		println!(" eth_usdc {:?}", eth_usdc);
	}
	let usdc_usdt_config = pallet_pablo::PoolInitConfiguration::ConstantProduct {
		owner: owner.clone(),
		pair: CurrencyPair::new(usdc, usdt),
		fee: Permill::zero(),
		base_weight: Permill::from_percent(50_u32),
	};
	let usdc_usdt = pallet_pablo::Pallet::<T>::do_create_pool(usdc_usdt_config).unwrap();
	sp_std::if_std! {
		println!(" usdc_usdt {:?}", usdc_usdt);
	}
	// 1 usdc == 1 usdt
	let usdc_amount = 1000 * unit;
	let usdt_amount = 1000 * unit;
	<T as pallet_pablo::Config>::Assets::mint_into(usdc, &owner, usdc_amount.into())
		.expect("mint usdc failed");
	<T as pallet_pablo::Config>::Assets::mint_into(usdt, &owner, usdt_amount.into())
		.expect("mint usdt failed");
	<pallet_pablo::Pallet<T> as Amm>::add_liquidity(
		&owner,
		usdc_usdt,
		usdc_amount.into(),
		usdt_amount.into(),
		0_u128.into(),
		false,
	)
	.expect("add_liquidity to usdc_usdt_pool failed");
	let dex_route = vec![3_u128.into(), 2_u128.into(), 1_u128.into(), 0_u128.into()];
	let pica: <T as pallet_dex_router::Config>::AssetId = 100_u128.into();
	let usdt: <T as pallet_dex_router::Config>::AssetId = 104_u128.into();
	let currency_pair = CurrencyPair::new(pica, usdt);
	(currency_pair, dex_route)
}

benchmarks! {
	where_clause {
		where
		T: pallet_dex_router::Config + pallet_pablo::Config,
		<T as pallet_dex_router::Config>::Balance: From<u128>,
		<T as pallet_dex_router::Config>::AssetId: From<u128>,
		<T as pallet_dex_router::Config>::PoolId: From<u128>,
		<T as pallet_pablo::Config>::Balance: From<u128>,
		<T as pallet_pablo::Config>::AssetId: From<u128>,
		<T as pallet_pablo::Config>::PoolId: From<u128>,
	}

	// benchmarks inserting new route
	update_route {
		let (currency_pair, dex_route) = create_pools_route::<T>();
		// let owner : <T as frame_system::Config>::AccountId= whitelisted_caller();
	} : _(RawOrigin::Root, currency_pair, Some(dex_route.clone().try_into().unwrap()))

	exchange {
		let unit = 1_000_000_000_000_u128;
		let pica_amount =  2000_u128 * unit;
		let (currency_pair, dex_route) = create_pools_route::<T>();
		let owner : <T as frame_system::Config>::AccountId= whitelisted_caller();
		let origin = RawOrigin::Signed(owner.clone());
		let pica : <T as pallet_pablo::Config>::AssetId = 100_u128.into();
		<T as pallet_pablo::Config>::Assets::mint_into(pica, &owner, pica_amount.into()).expect("Mint pica failed");
		pallet_dex_router::Pallet::<T>::update_route(RawOrigin::Root.into(), currency_pair, Some(dex_route.clone().try_into().unwrap())).expect("update route failed");
		// exchange 1000 PICA via route
	} : _(origin, currency_pair.swap(), (1000_u128 * unit).into(), 0_u128.into())

	buy {
		let unit = 1_000_000_000_000_u128;
		let usdc_amount =  20_u128 * 100 * unit; // 1 pica = 20 usdc based on liquidity added while pool creation
		let (currency_pair, dex_route) = create_pools_route::<T>();
		let owner : <T as frame_system::Config>::AccountId= whitelisted_caller();
		let origin = RawOrigin::Signed(owner.clone());
		let usdc : <T as pallet_pablo::Config>::AssetId = 104_u128.into();
		<T as pallet_pablo::Config>::Assets::mint_into(usdc, &owner, usdc_amount.into()).expect("Mint usdc failed");
		pallet_dex_router::Pallet::<T>::update_route(RawOrigin::Root.into(), currency_pair, Some(dex_route.clone().try_into().unwrap())).expect("update route failed");
		// buy 100 PICA via route
	} : _(origin, currency_pair, (100_u128 * unit).into(), 0_u128.into())

	// sell calls exchange so just added benchmark for completeness
	sell {
		let unit = 1_000_000_000_000_u128;
		let usdc_amount =  2000_u128 * unit;
		let (currency_pair, dex_route) = create_pools_route::<T>();
		let owner : <T as frame_system::Config>::AccountId= whitelisted_caller();
		let origin = RawOrigin::Signed(owner.clone());
		let usdc : <T as pallet_pablo::Config>::AssetId = 104_u128.into();
		<T as pallet_pablo::Config>::Assets::mint_into(usdc, &owner, usdc_amount.into()).expect("Mint usdc failed");
		pallet_dex_router::Pallet::<T>::update_route(RawOrigin::Root.into(), currency_pair, Some(dex_route.clone().try_into().unwrap())).expect("update route failed");
		// sell 1000 usdc via route
	} : _(origin, currency_pair, (1000_u128 * unit).into(), 0_u128.into())

	add_liquidity {
		let unit = 1_000_000_000_000_u128;
		let owner : <T as frame_system::Config>::AccountId= whitelisted_caller();
		let (currency_pair, dex_route) = create_single_node_pool::<T>();
		// 1 usdc == 1 usdt
		let usdc_amount = 1000 * unit;
		let usdt_amount = 1000 * unit;
		let origin = RawOrigin::Signed(owner.clone());
		pallet_dex_router::Pallet::<T>::update_route(RawOrigin::Root.into(), currency_pair, Some(dex_route.clone().try_into().unwrap())).expect("update route failed");
	} : _(origin, currency_pair, usdc_amount.into(), usdt_amount.into(), 0_u128.into(), false)

	remove_liquidity {
		let unit = 1_000_000_000_000_u128;
		let owner : <T as frame_system::Config>::AccountId= whitelisted_caller();
		let (currency_pair, dex_route) = create_single_node_pool::<T>();
		// 1 usdc == 1 usdt
		let usdc_amount = 1000 * unit;
		let usdt_amount = 1000 * unit;
		let origin = RawOrigin::Signed(owner.clone());
		pallet_dex_router::Pallet::<T>::update_route(RawOrigin::Root.into(), currency_pair, Some(dex_route.clone().try_into().unwrap())).expect("update route failed");
		pallet_dex_router::Pallet::<T>::add_liquidity(origin.clone().into(), currency_pair, usdc_amount.into(), usdt_amount.into(), 0_u128.into(), false).expect("add_liquidity failed");
		// remove 1 lp_token
	} : _(origin, currency_pair, 1_u128.into(), 0_u128.into(), 0_u128.into())
}
impl_benchmark_test_suite!(DexRouter, crate::mock::new_test_ext(), crate::mock::Test);
