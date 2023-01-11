use super::*;
use crate::Pallet as Fnft;
use codec::Encode;
use composable_tests_helpers::test::helper::RuntimeTrait;
use frame_benchmarking::{account, benchmarks};
use frame_support::traits::{
	tokens::nonfungibles::{Create, Mutate},
	OriginTrait,
};
use frame_system::pallet_prelude::OriginFor;

benchmarks! {
	where_clause {
		where
			T::BlockNumber: From<u32>,
			T::FinancialNftCollectionId: From<u128>,
			T::FinancialNftInstanceId: From<u64>,
			T: RuntimeTrait<crate::Event<T>> + Config,
	}
	transfer {
		let user1 = account("user1", 0, 0);
		let user2 = account::<AccountIdOf<T>>("user2", 0, 0);
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
	}: _(OriginFor::<T>::signed(user1), collection_id, created_nft_id, user2.clone())
	verify {
		T::assert_last_event(
			Event::FinancialNftTransferred {
				collection_id,
				instance_id: created_nft_id,
				to: user2,
			}
		);
	}

	impl_benchmark_test_suite!(Fnft, crate::test::mock::new_test_ext(), crate::test::mock::MockRuntime);
}
