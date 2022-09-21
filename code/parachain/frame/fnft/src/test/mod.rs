use sp_runtime::AccountId32;

/// Contains the mock runtime for this pallet's test suite.
pub(crate) mod mock;

/// Various helpers used throughout this test suite.
pub(crate) mod prelude;

const ALICE: AccountId32 = AccountId32::new([
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
]);
const BOB: AccountId32 = AccountId32::new([
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
]);
const CHARLIE: AccountId32 = AccountId32::new([
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3,
]);

/// Tests the pallet's [`frame_support::traits::tokens::nonfungibles`] traits implementations.
mod nonfungibles {
	use crate::test::mock::System;
	use frame_support::traits::tokens::nonfungibles::*;
	use std::collections::BTreeMap;

	use crate::{
		pallet::*,
		test::{
			mock::{new_test_ext, MockRuntime, Nft},
			ALICE, BOB,
		},
	};

	/// Tests the pallet's [`Transfer`] implementation.
	mod transfer;

	/// Tests the pallet's [`Mutate`] implementation.
	mod mutate;

	/// Tests the pallet's [`Inspect`] implementation.
	mod inspect;

	/// Tests the pallet's [`InspectEnumerable`] implementation.
	mod inspect_enumerable;

	/// Tests the pallet's [`Create`] implementation.
	#[test]
	fn create_inspect() {
		new_test_ext().execute_with(|| {
			assert_eq!(
				Nft::create_collection(&255, &ALICE, &BOB),
				Ok(()),
				"class creation should be successful"
			);

			System::assert_last_event(
				crate::Event::FinancialNftCollectionCreated {
					collection_id: 255,
					who: ALICE.clone(),
					admin: BOB.clone(),
				}
				.into(),
			);

			assert_eq!(
				Collection::<MockRuntime>::get(255),
				Some((ALICE, BOB, BTreeMap::default())),
				"newly created class should be owned by ALICE and managed by BOB, and have no attributes"
			);

			assert_eq!(
				Nft::create_collection(&255, &ALICE, &BOB),
				Err(Error::<MockRuntime>::CollectionAlreadyExists.into()),
				"should not be able to create class that already exists"
			);

			// Testing InspectEnumerable impl
		})
	}
}
