use codec::Encode;
use composable_tests_helpers::test::helper::assert_last_event;
use composable_traits::financial_nft::{FinancialNftProvider, NftClass};
use std::collections::{BTreeMap, BTreeSet};

use crate::{
	pallet::{ClassInstances, Event as NftEvent, Instance, OwnerInstances},
	test::mock::{Event, MockRuntime},
	NftInstanceId, Pallet,
};

/// Contains the mock runtime for this pallet's tests.
mod mock;

const ALICE: u128 = 0;
const BOB: u128 = 1;
const CHARLIE: u128 = 2;

/// Mints a single NFT into ALICE and checks that it was created properly, returning the id of the
/// newly created NFT.
///
/// NOTE: Only call once per test!
fn mint_nft_and_assert() -> NftInstanceId {
	let created_nft_id =
		Pallet::<MockRuntime>::mint_nft(&NftClass::STAKING, &ALICE, &1u32, &1u32).unwrap();

	assert_last_event::<MockRuntime>(Event::Nft(NftEvent::NftCreated {
		class_id: NftClass::STAKING,
		instance_id: created_nft_id,
	}));

	assert_eq!(
		ClassInstances::<MockRuntime>::get(&NftClass::STAKING).unwrap(),
		BTreeSet::from([created_nft_id]),
		"STAKING class should only have one instance"
	);

	assert_eq!(
		OwnerInstances::<MockRuntime>::get(&ALICE).unwrap(),
		BTreeSet::from([(NftClass::STAKING, created_nft_id)]),
		"ALICE should only have one instance"
	);

	assert_eq!(
		Instance::<MockRuntime>::get(&(NftClass::STAKING, created_nft_id)).unwrap(),
		(ALICE, BTreeMap::from([(1u32.encode(), 1u32.encode())])),
		"owner should be ALICE"
	);

	created_nft_id
}

mod financial_nft_provider {
	use crate::test::{mint_nft_and_assert, mock::new_test_ext};

	#[test]
	fn mint_nft() {
		new_test_ext().execute_with(mint_nft_and_assert);
	}
}

mod impls {
	use std::collections::BTreeMap;

	use codec::Encode;
	use composable_traits::financial_nft::NftClass;
	use frame_support::traits::tokens::nonfungibles::{Create, Inspect};

	use crate::{
		pallet::*,
		test::{
			mint_nft_and_assert,
			mock::{new_test_ext, MockRuntime},
			ALICE,
		},
		Pallet,
	};

	/// [`Transfer`] tests
	mod transfer;

	#[test]
	fn inspect() {
		//! Tests the pallet's [`Inspect`] implementation.
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

	#[test]
	fn create() {
		//! Tests the pallet's [`Create`] implementation.
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
