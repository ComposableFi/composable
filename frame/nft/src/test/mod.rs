/// Contains the mock runtime for this pallet's test suite.
pub(crate) mod mock;

/// Various helpers used throughout this test suite.
pub(crate) mod prelude;

const ALICE: u128 = 0;
const BOB: u128 = 1;
const CHARLIE: u128 = 2;

/// Tests the pallet's
/// [`FinancialNftProvider`][composable_traits::financial_nft::FinancialNftProvider] implementation.
mod financial_nft_provider {
	use crate::test::{mock::new_test_ext, prelude::mint_nft_and_assert};

	#[test]
	#[ignore = "TODO: fix with updates to nft pallet"]
	fn mint_nft() {
		new_test_ext().execute_with(mint_nft_and_assert);
	}
}

/// Tests the pallet's [`frame_support::traits::tokens::nonfungibles`] traits implementations.
mod nonfungibles {
	use std::collections::BTreeMap;

	use composable_traits::nft::NftClass;
	use frame_support::traits::tokens::nonfungibles::*;

	use crate::{
		pallet::*,
		test::{
			mock::{new_test_ext, MockRuntime},
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
				Pallet::<MockRuntime>::create_class(&NftClass::new(255), &ALICE, &BOB),
				Ok(()),
				"class creation should be successful"
			);

			assert_eq!(
				Class::<MockRuntime>::get(&NftClass::new(255)),
				Some((ALICE, BOB, BTreeMap::default())),
				"newly created class should be owned by ALICE and managed by BOB, and have no attributes"
			);

			assert_eq!(
				Pallet::<MockRuntime>::create_class(&NftClass::new(255), &ALICE, &BOB),
				Err(Error::<MockRuntime>::ClassAlreadyExists.into()),
				"should not be able to create class that already exists"
			);
		})
	}
}
