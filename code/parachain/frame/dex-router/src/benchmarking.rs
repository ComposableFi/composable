use super::*;
use crate::{self as pallet_dex_router, Pallet as DexRouter};
use composable_traits::{
	defi::CurrencyPair,
	dex::{Amm, AssetAmount},
};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::traits::fungibles::Mutate;
use frame_system::RawOrigin;
use pallet_pablo::PoolInitConfiguration;
use sp_arithmetic::{PerThing, Permill};
use sp_runtime::{traits::ConstU32, BoundedBTreeMap};
use sp_std::{collections::btree_map::BTreeMap, vec, vec::Vec};

pub fn dual_asset_pool_weights<T>(
	first_asset: <T as pallet_pablo::Config>::AssetId,
	first_asset_weight: Permill,
	second_asset: <T as pallet_pablo::Config>::AssetId,
) -> BoundedBTreeMap<<T as pallet_pablo::Config>::AssetId, Permill, ConstU32<2>>
where
	T: pallet_pablo::Config,
{
	let mut asset_weights = BoundedBTreeMap::new();
	asset_weights.try_insert(first_asset, first_asset_weight).expect("Should work");
	asset_weights
		.try_insert(second_asset, first_asset_weight.left_from_one())
		.expect("Should work");
	asset_weights
}

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
	let usdc = 100_u128.into();
	let usdt = 104_u128.into();
	let lp_token_id = 1000.into();
	let owner: <T as frame_system::Config>::AccountId = whitelisted_caller();
	let usdc_usdt_config = PoolInitConfiguration::DualAssetConstantProduct {
		owner: owner.clone(),
		fee: Permill::zero(),
		assets_weights: dual_asset_pool_weights::<T>(usdc, Permill::from_percent(50), usdt),
	};
	let usdc_usdt =
		pallet_pablo::Pallet::<T>::do_create_pool(usdc_usdt_config, Some(lp_token_id)).unwrap();
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
	let pica = 100_u128.into();
	let ksm = 101_u128.into();
	let eth = 102_u128.into();
	let usdc = 103_u128.into();
	let usdt = 104_u128.into();
	let pica_ksm_lpt_id = 1000.into();
	let ksm_eth_lpt_id = 1001.into();
	let eth_usdc_lpt_id = 1002.into();
	let usdc_usdt_lpt_id = 1003.into();
	let pica_ksm_config = pallet_pablo::PoolInitConfiguration::DualAssetConstantProduct {
		owner: owner.clone(),
		fee: Permill::zero(),
		assets_weights: dual_asset_pool_weights::<T>(pica, Permill::from_percent(50), ksm),
	};
	let pica_ksm =
		pallet_pablo::Pallet::<T>::do_create_pool(pica_ksm_config, Some(pica_ksm_lpt_id)).unwrap();
	// 100 pica == 1 ksm
	let pica_amount = 100 * 1000 * unit;
	let ksm_amount = 1000 * unit;
	<T as pallet_pablo::Config>::Assets::mint_into(pica, &owner, pica_amount.into())
		.expect("mint pica failed");
	<T as pallet_pablo::Config>::Assets::mint_into(ksm, &owner, ksm_amount.into())
		.expect("mint ksm failed");
	<pallet_pablo::Pallet<T> as Amm>::add_liquidity(
		&owner,
		pica_ksm,
		BTreeMap::from([(pica, pica_amount.into()), (ksm, ksm_amount.into())]),
		0_u128.into(),
		false,
	)
	.expect("add_liquidity to pica_ksm_pool failed");
	sp_std::if_std! {
		println!(" pica_ksm {:?}", pica_ksm);
	}
	let ksm_eth_config = pallet_pablo::PoolInitConfiguration::DualAssetConstantProduct {
		owner: owner.clone(),
		fee: Permill::zero(),
		assets_weights: dual_asset_pool_weights::<T>(ksm, Permill::from_percent(50), eth),
	};
	let ksm_eth =
		pallet_pablo::Pallet::<T>::do_create_pool(ksm_eth_config, Some(ksm_eth_lpt_id)).unwrap();
	// 10 ksm == 1 eth
	let ksm_amount = 10 * 1000 * unit;
	let eth_amount = 1000 * unit;
	<T as pallet_pablo::Config>::Assets::mint_into(ksm, &owner, ksm_amount.into())
		.expect("mint ksm failed");
	<T as pallet_pablo::Config>::Assets::mint_into(eth, &owner, eth_amount.into())
		.expect("mint eth failed");
	<pallet_pablo::Pallet<T> as Amm>::add_liquidity(
		&owner,
		ksm_eth,
		BTreeMap::from([(ksm, ksm_amount.into()), (eth, eth_amount.into())]),
		0_u128.into(),
		false,
	)
	.expect("add_liquidity to ksm_eth_pool failed");
	sp_std::if_std! {
		println!(" ksm_eth {:?}", ksm_eth);
	}

	let eth_usdc_config = pallet_pablo::PoolInitConfiguration::DualAssetConstantProduct {
		owner: owner.clone(),
		fee: Permill::zero(),
		assets_weights: dual_asset_pool_weights::<T>(eth, Permill::from_percent(50), usdc),
	};
	let eth_usdc =
		pallet_pablo::Pallet::<T>::do_create_pool(eth_usdc_config, Some(eth_usdc_lpt_id)).unwrap();
	// 1 eth = 200 usdc
	let eth_amount = 1000 * unit;
	let usdc_amount = 200 * 1000 * unit;
	<T as pallet_pablo::Config>::Assets::mint_into(eth, &owner, eth_amount.into())
		.expect("mint eth failed");
	<T as pallet_pablo::Config>::Assets::mint_into(usdc, &owner, usdc_amount.into())
		.expect("mint usdc failed");
	<pallet_pablo::Pallet<T> as Amm>::add_liquidity(
		&owner,
		eth_usdc,
		BTreeMap::from([(usdc, usdc_amount.into()), (eth, eth_amount.into())]),
		0_u128.into(),
		false,
	)
	.expect("add_liquidity to eth_usdc_pool failed");

	sp_std::if_std! {
		println!(" eth_usdc {:?}", eth_usdc);
	}
	let usdc_usdt_config = pallet_pablo::PoolInitConfiguration::DualAssetConstantProduct {
		owner: owner.clone(),
		fee: Permill::zero(),
		assets_weights: dual_asset_pool_weights::<T>(usdc, Permill::from_percent(50), usdt),
	};
	let usdc_usdt =
		pallet_pablo::Pallet::<T>::do_create_pool(usdc_usdt_config, Some(usdc_usdt_lpt_id))
			.unwrap();
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
		BTreeMap::from([(usdc, usdc_amount.into()), (usdt, usdt_amount.into())]),
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
	impl_benchmark_test_suite!(DexRouter, crate::mock::new_test_ext(), crate::mock::Test);

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
	} : _(RawOrigin::Root, currency_pair, Some(dex_route.try_into().unwrap()))

	swap {
		let unit = 1_000_000_000_000_u128;
		let pica_amount =  2000_u128 * unit;
		let (currency_pair, dex_route) = create_pools_route::<T>();
		let owner : <T as frame_system::Config>::AccountId= whitelisted_caller();
		let origin = RawOrigin::Signed(owner.clone());
		let pica : <T as pallet_pablo::Config>::AssetId = 100_u128.into();
		let pica_ : <T as pallet::Config>::AssetId = 100_u128.into();
		let usdt : <T as pallet::Config>::AssetId = 104_u128.into();
		<T as pallet_pablo::Config>::Assets::mint_into(pica, &owner, pica_amount.into()).expect("Mint pica failed");
		pallet_dex_router::Pallet::<T>::update_route(RawOrigin::Root.into(), currency_pair, Some(dex_route.try_into().unwrap())).expect("update route failed");
		// exchange 1000 PICA via route
	} : _(origin, AssetAmount::new(pica_, (1000_u128 * unit).into()), AssetAmount::new(usdt, 0_u128.into()))

	buy {
		let unit = 1_000_000_000_000_u128;
		let usdc_amount =  20_u128 * 100 * unit; // 1 pica = 20 usdc based on liquidity added while pool creation
		let (currency_pair, dex_route) = create_pools_route::<T>();
		let owner : <T as frame_system::Config>::AccountId= whitelisted_caller();
		let origin = RawOrigin::Signed(owner.clone());
		let usdc : <T as pallet_pablo::Config>::AssetId = 104_u128.into();
		<T as pallet_pablo::Config>::Assets::mint_into(usdc, &owner, usdc_amount.into()).expect("Mint usdc failed");
		pallet_dex_router::Pallet::<T>::update_route(RawOrigin::Root.into(), currency_pair, Some(dex_route.try_into().unwrap())).expect("update route failed");
		let pica_ : <T as pallet::Config>::AssetId = 100_u128.into();
		let usdt : <T as pallet::Config>::AssetId = 104_u128.into();
		// buy 100 PICA via route
	} : _(origin, usdt, AssetAmount::new(pica_, (100_u128 * unit).into()))

	add_liquidity {
		let unit = 1_000_000_000_000_u128;
		let owner : <T as frame_system::Config>::AccountId= whitelisted_caller();
		let (currency_pair, dex_route) = create_single_node_pool::<T>();
		// 1 usdc == 1 usdt
		let usdc_amount = 1000 * unit;
		let usdt_amount = 1000 * unit;
		let origin = RawOrigin::Signed(owner);
		pallet_dex_router::Pallet::<T>::update_route(RawOrigin::Root.into(), currency_pair, Some(dex_route.try_into().unwrap())).expect("update route failed");
	} : _(origin, BTreeMap::from([(currency_pair.base, usdc_amount.into()), (currency_pair.quote, usdt_amount.into())]), 0_u128.into(), false)

	remove_liquidity {
		let unit = 1_000_000_000_000_u128;
		let owner : <T as frame_system::Config>::AccountId= whitelisted_caller();
		let (currency_pair, dex_route) = create_single_node_pool::<T>();
		// 1 usdc == 1 usdt
		let usdc_amount = 1000 * unit;
		let usdt_amount = 1000 * unit;
		let origin = RawOrigin::Signed(owner);
		pallet_dex_router::Pallet::<T>::update_route(RawOrigin::Root.into(), currency_pair, Some(dex_route.try_into().unwrap())).expect("update route failed");
		pallet_dex_router::Pallet::<T>::add_liquidity(origin.clone().into(), BTreeMap::from([(currency_pair.base, usdc_amount.into()), (currency_pair.quote, usdt_amount.into())]), 0_u128.into(), false).expect("add_liquidity failed");
		// remove 1 lp_token
	} : _(origin, 1_u128.into(), BTreeMap::from([(currency_pair.base, 0.into()), (currency_pair.quote, 0.into())]))
}
