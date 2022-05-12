/// Contains the mock runtime for this pallet's tests.
pub(crate) mod mock;

/// Various helpers used throughout the tests.
pub(crate) mod helpers;

const ALICE: u128 = 0;
const BOB: u128 = 1;
const CHARLIE: u128 = 2;

mod financial_nft_provider {
	use crate::test::{helpers::mint_nft_and_assert, mock::new_test_ext};

	#[test]
	fn mint_nft() {
		new_test_ext().execute_with(mint_nft_and_assert);
	}
}

mod impls {
	use std::collections::BTreeMap;

	use codec::Encode;
	use composable_traits::financial_nft::NftClass;
	use frame_support::traits::tokens::nonfungibles::*;

	use crate::{
		pallet::*,
		test::{
			helpers::mint_nft_and_assert,
			mock::{new_test_ext, MockRuntime},
			ALICE,
		},
	};

	/// Tests the pallet's [`Transfer`] implementation.
	mod transfer;

	/// Tests the pallet's [`Mutate`] implementation.
	mod mutate;

	#[test]
	/// Tests the pallet's [`Inspect`] implementation.
	fn inspect() {
		new_test_ext().execute_with(|| {
			let created_nft_id = mint_nft_and_assert();

			// owner check
			assert_eq!(
				Pallet::<MockRuntime>::owner(&NftClass::STAKING, &created_nft_id),
				Some(ALICE)
			);

			// attribute check
			assert_eq!(
				Pallet::<MockRuntime>::attribute(
					&NftClass::STAKING,
					&created_nft_id,
					&1u32.encode()
				),
				Some(1u32.encode())
			);

			// class attribute check
			assert_eq!(
				Pallet::<MockRuntime>::class_attribute(&NftClass::STAKING, &1u32.encode()),
				None,
				"staking class should have no attributes"
			);
		})
	}

	/// Tests the pallet's [`Create`] implementation.
	#[test]
	fn create() {
		new_test_ext().execute_with(|| {
			assert_eq!(
				Pallet::<MockRuntime>::create_class(&NftClass::new(2), &ALICE, &ALICE),
				Ok(())
			);

			assert_eq!(
				Class::<MockRuntime>::get(&NftClass::new(2)),
				Some((ALICE, ALICE, BTreeMap::default()))
			);
		})
	}
}
