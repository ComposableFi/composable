use super::*;
use crate::{self as pallet_dex_router, Pallet as DexRouter};
use composable_traits::{
	defi::CurrencyPair,
	dex::Amm,
};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{assert_ok, traits::fungibles::Mutate};
use frame_system::RawOrigin;
use sp_arithmetic::Permill;
use sp_std::{vec, vec::Vec};

benchmarks! {
    where_clause {
        where
        T: pallet_dex_router::Config + pallet_pablo::Config,
        <T as pallet_dex_router::Config>::Balance: From<u128>,
        <T as pallet_dex_router::Config>::AssetId: From<u128>,
        <T as pallet_dex_router::Config>::PoolId: From<u128>,
        <T as pallet_pablo::Config>::Balance: From<u128>,
        <T as pallet_pablo::Config>::AssetId: From<u128>,
    }

    // benchmarks inserting new route
    update_route {
        let owner : <T as frame_system::Config>::AccountId= whitelisted_caller();
        let pica : <T as pallet_pablo::Config>::AssetId = 100_u128.into();
        let ksm : <T as pallet_pablo::Config>::AssetId = 101_u128.into();
        let eth : <T as pallet_pablo::Config>::AssetId = 102_u128.into();
        let usdc : <T as pallet_pablo::Config>::AssetId = 103_u128.into();
        let usdt : <T as pallet_pablo::Config>::AssetId = 104_u128.into();
        let pica_ksm_config= pallet_pablo::PoolInitConfiguration::ConstantProduct {
            owner : owner.clone(),
            pair: CurrencyPair::new(pica, ksm),
            fee: Permill::zero(),
            owner_fee: Permill::zero(),
        };
        let pica_ksm = pallet_pablo::Pallet::<T>::do_create_pool(pica_ksm_config).unwrap();
        sp_std::if_std!{
            println!(" pica_ksm {:?}", pica_ksm);
        }
        let ksm_eth_config = pallet_pablo::PoolInitConfiguration::ConstantProduct {
            owner : owner.clone(),
            pair: CurrencyPair::new(ksm, eth),
            fee: Permill::zero(),
            owner_fee: Permill::zero(),
        }; 
        let ksm_eth = pallet_pablo::Pallet::<T>::do_create_pool(ksm_eth_config).unwrap();
        sp_std::if_std!{
            println!(" ksm_eth {:?}", ksm_eth);
        }

        let eth_usdc_config = pallet_pablo::PoolInitConfiguration::ConstantProduct {
            owner : owner.clone(),
            pair: CurrencyPair::new(eth, usdc),
            fee: Permill::zero(),
            owner_fee: Permill::zero(),
        }; 
        let eth_usdc = pallet_pablo::Pallet::<T>::do_create_pool(eth_usdc_config).unwrap();

        sp_std::if_std!{
            println!(" eth_usdc {:?}", eth_usdc);
        }
        let usdc_usdt_config = pallet_pablo::PoolInitConfiguration::StableSwap {
            owner : owner.clone(),
            pair : CurrencyPair::new(usdc, usdt),
            amplification_coefficient: 5_u16,
            fee: Permill::zero(),
            owner_fee: Permill::zero(),
        };
        let usdc_usdt = pallet_pablo::Pallet::<T>::do_create_pool(usdc_usdt_config).unwrap();
        sp_std::if_std!{
            println!(" usdc_usdt {:?}", usdc_usdt);
        }
        let dex_route = vec![
            // pica_ksm.into(),
            // ksm_eth.into(),
            // eth_usdc.into(),
            // usdc_usdt.into(),
            3_u128.into(),
            2_u128.into(),
            1_u128.into(),
            0_u128.into(),
            // 0_u128.into(),
            // 1_u128.into(),
            // 2_u128.into(),
            // 3_u128.into(),
        ];
        let pica : <T as pallet_dex_router::Config>::AssetId = 100_u128.into();
        let usdt : <T as pallet_dex_router::Config>::AssetId = 104_u128.into();
        let currency_pair = CurrencyPair::new(pica, usdt);
    } : _(RawOrigin::Signed(owner), currency_pair, Some(dex_route.clone().try_into().unwrap()))
}
impl_benchmark_test_suite!(DexRouter, crate::mock::new_test_ext(), crate::mock::Test);
