#![allow(clippy::disallowed_methods, clippy::panic)]

use super::*;

#[allow(unused_imports)]
use crate::Pallet as CurrencyFactory;
use crate::{self as currency_factory};
use codec::Decode;
use composable_traits::{
	assets::BasicAssetMetadata,
	currency::{CurrencyFactory as DeFiCurrencyFactory, RangeId},
};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;

// pub fn whitelisted_origin<T: frame_system::Config>() -> RawOrigin<T::AccountId> {
// 	let caller: T::AccountId = whitelisted_caller();
// 	RawOrigin::Signed(caller)
// }

benchmarks! {
	where_clause {
		where
			T: currency_factory::Config,
			<T as currency_factory::Config>::AssetId : Decode,
	}
	add_range {
	}: _(RawOrigin::Root, 100000000000000)
	set_metadata {
		currency_factory::Pallet::<T>::add_range(RawOrigin::Root.into(), 0).unwrap();
		let asset_id = <currency_factory::Pallet::<T> as DeFiCurrencyFactory>::create(RangeId::from(0)).unwrap();
		let metadata = BasicAssetMetadata::try_from(b"SMB", b"Symbol Name").unwrap();
	}: {
		currency_factory::Pallet::<T>::set_metadata(RawOrigin::Root.into(), asset_id,  metadata).unwrap();
	}
}

impl_benchmark_test_suite!(CurrencyFactory, crate::mocks::new_test_ext(), crate::mocks::Test,);
