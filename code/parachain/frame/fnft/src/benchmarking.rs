use super::*;
use crate::Pallet as Fnft;
use codec::Encode;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::traits::tokens::nonfungibles::{Create, Mutate};
use frame_system::RawOrigin;

benchmarks! {
  where_clause { where T::BlockNumber: From<u32>, T::FinancialNftCollectionId: From<u128>, T::FinancialNftInstanceId: From<u64> }
	transfer {
		let user1 = account("user1", 0, 0);
		let user2 = account("user2", 0, 0);
		let collection_id = 1_u128.into();
		Fnft::<T>::create_collection(&collection_id, &user1, &user1).unwrap();
		let created_nft_id = 1_u64.into();
		Fnft::<T>::mint_into(&collection_id, &created_nft_id, &user1)?;

		Fnft::<T>::set_attribute(
			&collection_id,
			&created_nft_id,
			&1_u32.encode(),
			&1_u32.encode()
		)?;
	}: {
		Fnft::<T>::transfer(
			RawOrigin::Signed(user1).into(), collection_id, created_nft_id, user2
		).expect("Transfer failed")
	}
}

impl_benchmark_test_suite!(Fnft, crate::test::mock::new_test_ext(), crate::test::mock::MockRuntime);
