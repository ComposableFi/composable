/// Tests the implementation of [`Mutate::mint_into`].
mod mint_into {
	use crate::test::mock::*;
	use std::collections::{BTreeMap, BTreeSet};

	use composable_tests_helpers::test::helper::assert_last_event;

	use composable_traits::{
		account_proxy::{AccountProxy, ProxyType},
		fnft::FinancialNft,
	};
	use frame_support::{
		assert_noop, assert_ok,
		traits::tokens::nonfungibles::{Create, Mutate},
	};
	use sp_runtime::DispatchError;

	use crate::{
		pallet::*,
		test::{
			mock::{new_test_ext, Event, MockRuntime},
			prelude::*,
			ALICE, BOB,
		},
	};

	/// Tests minting an NFT into an account.
	#[test]
	fn success() {
		new_test_ext().execute_with(|| {
			Nft::create_collection(&TEST_COLLECTION_ID, &ALICE, &BOB).unwrap();
			const NEW_NFT_ID: FinancialNftInstanceIdOf<MockRuntime> = 1;

			Nft::mint_into(&TEST_COLLECTION_ID, &NEW_NFT_ID, &ALICE).unwrap();

			assert_last_event::<MockRuntime>(Event::Nft(crate::Event::FinancialNftCreated {
				collection_id: TEST_COLLECTION_ID,
				instance_id: NEW_NFT_ID,
			}));

			assert_eq!(
				OwnerInstances::<MockRuntime>::get(&ALICE).unwrap(),
				BTreeSet::from([(TEST_COLLECTION_ID, NEW_NFT_ID)]),
				"ALICE should only have one instance"
			);

			assert_eq!(
				Instance::<MockRuntime>::get(TEST_COLLECTION_ID, NEW_NFT_ID).unwrap(),
				(ALICE, BTreeMap::new()),
				"owner should be ALICE, with no attributes"
			);
			// get the asset account of the fNFT
			let asset_account = Nft::asset_account(&TEST_COLLECTION_ID, &NEW_NFT_ID);
			// check the proxies
			assert_ok!(Proxy::find_proxy(&asset_account, &ALICE, Some(ProxyType::Any)));
			assert_ok!(Proxy::find_proxy(&asset_account, &ALICE, Some(ProxyType::CancelProxy)));
		})
	}

	/// Asserts that minting an NFT that already exists is an error.
	#[test]
	fn already_exists() {
		new_test_ext().execute_with(|| {
			Nft::create_collection(&TEST_COLLECTION_ID, &ALICE, &BOB).unwrap();
			const NEW_NFT_ID: FinancialNftInstanceIdOf<MockRuntime> = 1;

			Nft::mint_into(&TEST_COLLECTION_ID, &NEW_NFT_ID, &ALICE).unwrap();

			assert_last_event::<MockRuntime>(Event::Nft(crate::Event::FinancialNftCreated {
				collection_id: TEST_COLLECTION_ID,
				instance_id: NEW_NFT_ID,
			}));

			assert_noop!(
				Nft::mint_into(&TEST_COLLECTION_ID, &NEW_NFT_ID, &ALICE),
				DispatchError::from(crate::Error::<MockRuntime>::InstanceAlreadyExists)
			);
		})
	}
}

/// Tests the implementation of [`Mutate::set_attribute`].
mod set_attribute {
	use crate::test::mock::*;
	use codec::{Decode, Encode};
	use composable_tests_helpers::test::block::process_and_progress_blocks;

	use frame_support::{
		assert_noop,
		traits::tokens::nonfungibles::{Create, Mutate},
	};
	use sp_runtime::DispatchError;
	use std::collections::BTreeMap;

	use crate::{
		pallet::*,
		test::{
			mock::{new_test_ext, MockRuntime},
			prelude::{TEST_COLLECTION_ID, *},
			ALICE, BOB,
		},
	};

	#[derive(Debug, Encode, Decode, Clone)]
	struct Key(String);

	#[derive(Debug, Encode, Decode, PartialEq, Clone)]
	struct Value {
		a: u32,
		b: bool,
	}

	/// Tests adding attributes to an NFT owned by an account that owns multiple NFts.
	#[test]
	fn success() {
		new_test_ext().execute_with(|| {
			Nft::create_collection(&TEST_COLLECTION_ID, &ALICE, &BOB).unwrap();
			let [nft_to_add_attribute_to, other_nft_ids @ ..] =
				mint_many_nfts_and_assert::<10>(ALICE, TEST_COLLECTION_ID);

			process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(10);

			let key = Key("some key".into());
			let value = Value { a: 10, b: true };

			add_attributes_and_assert(
				TEST_COLLECTION_ID,
				&nft_to_add_attribute_to,
				ALICE,
				&[(key.clone(), value)],
			);

			let key2 = Key("some other key".into());
			let value2 = Value { a: 20, b: false };

			add_attributes_and_assert(
				TEST_COLLECTION_ID,
				&nft_to_add_attribute_to,
				ALICE,
				&[(key2, value2)],
			);

			for should_not_be_mutated_nft_id in other_nft_ids {
				assert_eq!(
					Instance::<MockRuntime>::get(TEST_COLLECTION_ID, should_not_be_mutated_nft_id),
					Some((ALICE, BTreeMap::from([]))),
					"instance should not have any attributes"
				);
			}
		})
	}

	#[test]
	fn not_found() {
		new_test_ext().execute_with(|| {
			let key = Key("some key".into());
			let value = Value { a: 10, b: true };

			assert_noop!(
				Nft::set_attribute(&TEST_COLLECTION_ID, &1, &key.encode(), &value.encode()),
				DispatchError::from(crate::Error::<MockRuntime>::InstanceNotFound)
			);

			let new_nft_id = mint_into_and_assert();

			process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(10);

			assert_noop!(
				Nft::set_attribute(
					&TEST_COLLECTION_ID,
					&(new_nft_id + 1),
					&key.encode(),
					&value.encode()
				),
				DispatchError::from(crate::Error::<MockRuntime>::InstanceNotFound)
			);
		})
	}
}

/// Tests the implementation of [`Mutate::burn_from`].
mod burn_from {
	use crate::test::mock::*;
	use std::collections::BTreeSet;

	use composable_tests_helpers::test::{
		block::process_and_progress_blocks, helper::assert_last_event,
	};
	use frame_support::{
		assert_ok,
		traits::tokens::nonfungibles::{Create, Mutate},
	};

	use crate::{
		pallet::*,
		test::{
			mock::{new_test_ext, Event, MockRuntime},
			prelude::{TEST_COLLECTION_ID, *},
			ALICE, BOB,
		},
	};

	/// Tests burning an NFT from an account that owns multiple NFts.
	#[test]
	fn success() {
		new_test_ext().execute_with(|| {
			Nft::create_collection(&TEST_COLLECTION_ID, &ALICE, &BOB).unwrap();
			let [nft_to_burn, new_nft_ids @ ..] =
				mint_many_nfts_and_assert::<10>(ALICE, TEST_COLLECTION_ID);

			process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(10);

			assert_ok!(Nft::burn(&TEST_COLLECTION_ID, &nft_to_burn, Some(&ALICE)));
			assert_last_event::<MockRuntime>(Event::Nft(crate::Event::FinancialNftBurned {
				collection_id: TEST_COLLECTION_ID,
				instance_id: nft_to_burn,
			}));

			assert_eq!(
				OwnerInstances::<MockRuntime>::get(&ALICE).unwrap(),
				to_btree(TEST_COLLECTION_ID, &new_nft_ids),
				"ALICE should have all of the original NFTs except for the one burned"
			);

			assert_eq!(
				Instance::<MockRuntime>::get(TEST_COLLECTION_ID, nft_to_burn),
				None,
				"instance should not exist"
			);
		})
	}

	#[test]
	fn burn_last_owned_clears_storage() {
		new_test_ext().execute_with(|| {
			let new_id = mint_into_and_assert();

			process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(10);

			assert_ok!(Nft::burn(&TEST_COLLECTION_ID, &new_id, Some(&ALICE)));
			assert_last_event::<MockRuntime>(Event::Nft(crate::Event::FinancialNftBurned {
				collection_id: TEST_COLLECTION_ID,
				instance_id: new_id,
			}));

			assert_eq!(
				OwnerInstances::<MockRuntime>::get(&ALICE).unwrap(),
				BTreeSet::new(),
				"ALICE should not have any instances"
			);

			assert_eq!(
				Instance::<MockRuntime>::get(TEST_COLLECTION_ID, new_id),
				None,
				"instance should not exist"
			);
		})
	}

	/// Error tests for [`Mutate::burn_from`], testing for [`crate::Error::InstanceNotFound`]
	/// specifically.
	mod not_found {
		use crate::test::mock::*;
		use composable_tests_helpers::test::helper::assert_last_event;

		use frame_support::{
			assert_noop, assert_ok,
			traits::tokens::nonfungibles::{Create, Mutate},
		};
		use sp_runtime::DispatchError;

		use crate::test::{
			mock::{new_test_ext, Event, MockRuntime},
			prelude::{mint_many_nfts_and_assert, mint_nft_and_assert, TEST_COLLECTION_ID},
			ALICE, BOB,
		};

		/// Asserts that when no NFTs exist, burning an NFT that doesn't exist is an error.
		#[test]
		fn none_minted() {
			new_test_ext().execute_with(|| {
				assert_noop!(
					Nft::burn(&TEST_COLLECTION_ID, &1, Some(&ALICE)),
					DispatchError::from(crate::Error::<MockRuntime>::InstanceNotFound)
				);
			})
		}

		/// Asserts that when some NFTs exist, burning an NFT that doesn't exist is an error.
		#[test]
		fn some_minted() {
			new_test_ext().execute_with(|| {
				Nft::create_collection(&TEST_COLLECTION_ID, &ALICE, &BOB).unwrap();
				let [_new_nft_ids @ .., last_nft_minted] =
					mint_many_nfts_and_assert::<10>(ALICE, TEST_COLLECTION_ID);
				assert_noop!(
					Nft::burn(&TEST_COLLECTION_ID, &(last_nft_minted + 1), Some(&ALICE)),
					DispatchError::from(crate::Error::<MockRuntime>::InstanceNotFound)
				);
			})
		}

		/// Asserts that when some NFTs exist, burning an NFT twice is an error.
		#[test]
		fn burn_twice() {
			new_test_ext().execute_with(|| {
				Nft::create_collection(&TEST_COLLECTION_ID, &ALICE, &BOB).unwrap();
				let [nft_to_burn, _new_nft_ids @ ..] =
					mint_many_nfts_and_assert::<10>(ALICE, TEST_COLLECTION_ID);

				assert_ok!(Nft::burn(&TEST_COLLECTION_ID, &nft_to_burn, Some(&ALICE)));
				assert_last_event::<MockRuntime>(Event::Nft(crate::Event::FinancialNftBurned {
					collection_id: TEST_COLLECTION_ID,
					instance_id: nft_to_burn,
				}));

				assert_noop!(
					Nft::burn(&TEST_COLLECTION_ID, &nft_to_burn, Some(&ALICE)),
					DispatchError::from(crate::Error::<MockRuntime>::InstanceNotFound)
				);
			})
		}

		/// Asserts that when burning the last NFT that exists, burning it twice is an error.
		#[test]
		fn burn_twice_last_existing() {
			new_test_ext().execute_with(|| {
				let nft_to_burn = mint_nft_and_assert();

				assert_ok!(Nft::burn(&TEST_COLLECTION_ID, &nft_to_burn, Some(&ALICE)));
				assert_last_event::<MockRuntime>(Event::Nft(crate::Event::FinancialNftBurned {
					collection_id: TEST_COLLECTION_ID,
					instance_id: nft_to_burn,
				}));

				assert_noop!(
					Nft::burn(&TEST_COLLECTION_ID, &nft_to_burn, Some(&ALICE)),
					DispatchError::from(crate::Error::<MockRuntime>::InstanceNotFound)
				);
			})
		}
	}
}
