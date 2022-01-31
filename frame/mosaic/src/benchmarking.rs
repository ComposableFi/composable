use super::*;

use crate::Pallet as Mosaic;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::{EventRecord, RawOrigin};
use crate::mock::Origin;
use crate::decay::*;


benchmarks! {
    where_clause {
        where T::BlockNumber: From<u32>, T::NetworkId: From<u32>, BalanceOf<T>: From<u32>, AssetIdOf<T>: From<u32>
    }

    set_relayer {
        let relayer = whitelisted_caller();
    }: _ (RawOrigin::Root, relayer)

    rotate_relayer {
        let relayer: T::AccountId = whitelisted_caller();
        Mosaic::<T>::set_relayer(RawOrigin::Root.into(), relayer.clone());

        let new_relayer = account("new_relayer", 0, 0);
    }: _ (RawOrigin::Signed(relayer), new_relayer, 42.into())


    set_network {
        let relayer: T::AccountId = whitelisted_caller();
        Mosaic::<T>::set_relayer(RawOrigin::Root.into(), relayer.clone());

        let network_id: T::NetworkId = 1.into();
        let network_info = NetworkInfo {
            enabled: true,
            max_transfer_size: 100.into(),
        };

    }: _ (RawOrigin::Signed(relayer), network_id, network_info)

    set_budget {
        let asset_id: AssetIdOf<T> = 1.into();
        let amount: BalanceOf<T> = 100.into();
        let decayer: BudgetPenaltyDecayer<BalanceOf<T>, T::BlockNumber> = BudgetPenaltyDecayer::linear(5);
    }: _ (RawOrigin::Root, asset_id, amount, decayer)
}

impl_benchmark_test_suite!(Mosaic, crate::mock::new_test_ext(), crate::mock::Test,);
