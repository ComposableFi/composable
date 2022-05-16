/// Contains the mock runtime for this pallet's test suite.
pub(crate) mod mock;

/// Various helpers used throughout this test suite.
pub(crate) mod helpers;

const ALICE: u128 = 0;
const BOB: u128 = 1;
const CHARLIE: u128 = 2;

/// Tests the pallet's
/// [`FinancialNftProvider`][composable_traits::financial_nft::FinancialNftProvider] implementation.
mod financial_nft_provider {
	use crate::test::{helpers::mint_nft_and_assert, mock::new_test_ext};

	#[test]
	fn mint_nft() {
		new_test_ext().execute_with(mint_nft_and_assert);
	}
}

mod impls {
	use std::collections::BTreeMap;

	use composable_traits::financial_nft::NftClass;
	use frame_support::traits::tokens::nonfungibles::*;

	use crate::{
		pallet::*,
		test::{
			mock::{new_test_ext, MockRuntime},
			ALICE,
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
				Pallet::<MockRuntime>::create_class(&NftClass::new(255), &ALICE, &ALICE),
				Ok(())
			);

			assert_eq!(
				Class::<MockRuntime>::get(&NftClass::new(255)),
				Some((ALICE, ALICE, BTreeMap::default()))
			);
		})
	}
}
