/// Tests the implementation of [`Mutate::mint_into`].
mod mint_into {
	use std::collections::{BTreeMap, BTreeSet};

	use composable_tests_helpers::test::helper::assert_last_event;
	use composable_traits::financial_nft::NftClass;
	use frame_support::{assert_noop, traits::tokens::nonfungibles::Mutate};
	use sp_runtime::DispatchError;

	use crate::{
		pallet::*,
		test::{
			mock::{new_test_ext, Event, MockRuntime},
			ALICE,
		},
	};

	/// Tests minting an NFT into an account.
	#[test]
	fn success() {
		new_test_ext().execute_with(|| {
			const NEW_NFT_ID: NftInstanceId = 1;

			Pallet::<MockRuntime>::mint_into(&NftClass::STAKING, &NEW_NFT_ID, &ALICE).unwrap();

			assert_last_event::<MockRuntime>(Event::Nft(crate::Event::NftCreated {
				class_id: NftClass::STAKING,
				instance_id: NEW_NFT_ID,
			}));

			assert_eq!(
				ClassInstances::<MockRuntime>::get(&NftClass::STAKING).unwrap(),
				BTreeSet::from([NEW_NFT_ID]),
				"STAKING class should only have one instance"
			);

			assert_eq!(
				OwnerInstances::<MockRuntime>::get(&ALICE).unwrap(),
				BTreeSet::from([(NftClass::STAKING, NEW_NFT_ID)]),
				"ALICE should only have one instance"
			);

			assert_eq!(
				Instance::<MockRuntime>::get(&(NftClass::STAKING, NEW_NFT_ID)).unwrap(),
				(ALICE, BTreeMap::new()),
				"owner should be ALICE, with no attributes"
			);
		})
	}

	/// Asserts that minting an NFT that already exists is an error.
	#[test]
	fn already_exists() {
		new_test_ext().execute_with(|| {
			const NEW_NFT_ID: NftInstanceId = 1;

			Pallet::<MockRuntime>::mint_into(&NftClass::STAKING, &NEW_NFT_ID, &ALICE).unwrap();

			assert_last_event::<MockRuntime>(Event::Nft(crate::Event::NftCreated {
				class_id: NftClass::STAKING,
				instance_id: NEW_NFT_ID,
			}));

			assert_noop!(
				Pallet::<MockRuntime>::mint_into(&NftClass::STAKING, &NEW_NFT_ID, &ALICE),
				DispatchError::from(crate::Error::<MockRuntime>::InstanceAlreadyExists)
			);
		})
	}
}

/// Tests the implementation of [`Mutate::set_attribute`].
mod set_attribute {
	use codec::{Decode, Encode};
	use composable_tests_helpers::test::block::process_and_progress_blocks;
	use composable_traits::financial_nft::NftClass;
	use frame_support::{assert_noop, traits::tokens::nonfungibles::Mutate};
	use sp_runtime::DispatchError;
	use std::collections::BTreeMap;

	use crate::{
		pallet::*,
		test::{
			helpers::{add_attributes_and_assert, mint_into_and_assert, mint_many_nfts_and_assert},
			mock::{new_test_ext, MockRuntime},
			ALICE,
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
			let [nft_to_add_attribute_to, other_nft_ids @ ..] =
				mint_many_nfts_and_assert::<10>(ALICE);

			process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(10);

			let key = Key("some key".into());
			let value = Value { a: 10, b: true };

			add_attributes_and_assert(
				&NftClass::STAKING,
				&nft_to_add_attribute_to,
				ALICE,
				&[(key.clone(), value.clone())],
			);

			let key2 = Key("some other key".into());
			let value2 = Value { a: 20, b: false };

			add_attributes_and_assert(
				&NftClass::STAKING,
				&nft_to_add_attribute_to,
				ALICE,
				&[(key2, value2)],
			);

			for should_not_be_mutated_nft_id in other_nft_ids {
				assert_eq!(
					Instance::<MockRuntime>::get(&(
						NftClass::STAKING,
						should_not_be_mutated_nft_id
					)),
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
				Pallet::<MockRuntime>::set_attribute(
					&NftClass::STAKING,
					&1,
					&key.encode(),
					&value.encode()
				),
				DispatchError::from(crate::Error::<MockRuntime>::InstanceNotFound)
			);

			let new_nft_id = mint_into_and_assert();

			process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(10);

			assert_noop!(
				Pallet::<MockRuntime>::set_attribute(
					&NftClass::STAKING,
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
	use std::collections::BTreeSet;

	use composable_tests_helpers::test::{
		block::process_and_progress_blocks, helper::assert_last_event,
	};
	use composable_traits::financial_nft::NftClass;
	use frame_support::{assert_ok, traits::tokens::nonfungibles::Mutate};

	use crate::{
		pallet::*,
		test::{
			helpers::{mint_into_and_assert, mint_many_nfts_and_assert, to_btree},
			mock::{new_test_ext, Event, MockRuntime},
			ALICE,
		},
	};

	/// Tests burning an NFT from an account that owns multiple NFts.
	#[test]
	fn success() {
		new_test_ext().execute_with(|| {
			let [nft_to_burn, new_nft_ids @ ..] = mint_many_nfts_and_assert::<10>(ALICE);

			process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(10);

			assert_ok!(Pallet::<MockRuntime>::burn(&NftClass::STAKING, &nft_to_burn, Some(&ALICE)));
			assert_last_event::<MockRuntime>(Event::Nft(crate::Event::NftBurned {
				class_id: NftClass::STAKING,
				instance_id: nft_to_burn,
			}));

			assert_eq!(
				ClassInstances::<MockRuntime>::get(&NftClass::STAKING).unwrap(),
				BTreeSet::from(new_nft_ids),
				"STAKING class should have all of the original NFTs except for the one burned"
			);

			assert_eq!(
				OwnerInstances::<MockRuntime>::get(&ALICE).unwrap(),
				to_btree(&new_nft_ids),
				"ALICE should have all of the original NFTs except for the one burned"
			);

			assert_eq!(
				Instance::<MockRuntime>::get(&(NftClass::STAKING, nft_to_burn)),
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

			assert_ok!(Pallet::<MockRuntime>::burn(&NftClass::STAKING, &new_id, Some(&ALICE)));
			assert_last_event::<MockRuntime>(Event::Nft(crate::Event::NftBurned {
				class_id: NftClass::STAKING,
				instance_id: new_id,
			}));

			assert_eq!(
				ClassInstances::<MockRuntime>::get(&NftClass::STAKING).unwrap(),
				BTreeSet::new(),
				"STAKING class should not have any instances"
			);

			assert_eq!(
				OwnerInstances::<MockRuntime>::get(&ALICE).unwrap(),
				BTreeSet::new(),
				"ALICE should not have any instances"
			);

			assert_eq!(
				Instance::<MockRuntime>::get(&(NftClass::STAKING, new_id)),
				None,
				"instance should not exist"
			);
		})
	}

	/// Error tests for [`Mutate::burn_from`], testing for [`crate::Error::InstanceNotFound`]
	/// specifically.
	mod not_found {
		use composable_tests_helpers::test::helper::assert_last_event;
		use composable_traits::financial_nft::NftClass;
		use frame_support::{assert_noop, assert_ok, traits::tokens::nonfungibles::Mutate};
		use sp_runtime::DispatchError;

		use crate::{
			test::{
				helpers::{mint_many_nfts_and_assert, mint_nft_and_assert},
				mock::{new_test_ext, Event, MockRuntime},
				ALICE,
			},
			Pallet,
		};

		/// Asserts that when no NFTs exist, burning an NFT that doesn't exist is an error.
		#[test]
		fn none_minted() {
			new_test_ext().execute_with(|| {
				assert_noop!(
					Pallet::<MockRuntime>::burn(&NftClass::STAKING, &1, Some(&ALICE)),
					DispatchError::from(crate::Error::<MockRuntime>::InstanceNotFound)
				);
			})
		}

		/// Asserts that when some NFTs exist, burning an NFT that doesn't exist is an error.
		#[test]
		fn some_minted() {
			new_test_ext().execute_with(|| {
				let [_new_nft_ids @ .., last_nft_minted] = mint_many_nfts_and_assert::<10>(ALICE);
				assert_noop!(
					Pallet::<MockRuntime>::burn(
						&NftClass::STAKING,
						&(last_nft_minted + 1),
						Some(&ALICE)
					),
					DispatchError::from(crate::Error::<MockRuntime>::InstanceNotFound)
				);
			})
		}

		/// Asserts that when some NFTs exist, burning an NFT twice is an error.
		#[test]
		fn burn_twice() {
			new_test_ext().execute_with(|| {
				let [nft_to_burn, _new_nft_ids @ ..] = mint_many_nfts_and_assert::<10>(ALICE);

				assert_ok!(Pallet::<MockRuntime>::burn(
					&NftClass::STAKING,
					&nft_to_burn,
					Some(&ALICE)
				));
				assert_last_event::<MockRuntime>(Event::Nft(crate::Event::NftBurned {
					class_id: NftClass::STAKING,
					instance_id: nft_to_burn,
				}));

				assert_noop!(
					Pallet::<MockRuntime>::burn(&NftClass::STAKING, &nft_to_burn, Some(&ALICE)),
					DispatchError::from(crate::Error::<MockRuntime>::InstanceNotFound)
				);
			})
		}

		/// Asserts that when burning the last NFT that exists, burning it twice is an error.
		#[test]
		fn burn_twice_last_existing() {
			new_test_ext().execute_with(|| {
				let nft_to_burn = mint_nft_and_assert();

				assert_ok!(Pallet::<MockRuntime>::burn(
					&NftClass::STAKING,
					&nft_to_burn,
					Some(&ALICE)
				));
				assert_last_event::<MockRuntime>(Event::Nft(crate::Event::NftBurned {
					class_id: NftClass::STAKING,
					instance_id: nft_to_burn,
				}));

				assert_noop!(
					Pallet::<MockRuntime>::burn(&NftClass::STAKING, &nft_to_burn, Some(&ALICE)),
					DispatchError::from(crate::Error::<MockRuntime>::InstanceNotFound)
				);
			})
		}
	}
}
