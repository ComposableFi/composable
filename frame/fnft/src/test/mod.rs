/// Contains the mock runtime for this pallet's test suite.
pub(crate) mod mock;

/// Various helpers used throughout this test suite.
pub(crate) mod prelude;

const ALICE: u128 = 0;
const BOB: u128 = 1;
const CHARLIE: u128 = 2;

/// Tests the pallet's [`frame_support::traits::tokens::nonfungibles`] traits implementations.
mod nonfungibles {
	use std::collections::BTreeMap;

	use frame_support::traits::tokens::nonfungibles::*;

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

	/// Tests the pallet's [`Create`] implementation.
	#[test]
	fn create() {
		new_test_ext().execute_with(|| {
			assert_eq!(
				Nft::create_collection(&255_u16, &ALICE, &BOB),
				Ok(()),
				"class creation should be successful"
			);

			assert_eq!(
				Collection::<MockRuntime>::get(255),
				Some((ALICE, BOB, BTreeMap::default())),
				"newly created class should be owned by ALICE and managed by BOB, and have no attributes"
			);

			assert_eq!(
				Nft::create_collection(&255_u16, &ALICE, &BOB),
				Err(Error::<MockRuntime>::CollectionAlreadyExists.into()),
				"should not be able to create class that already exists"
			);
		})
	}
}
