use codec::{Decode, Encode};
use composable_tests_helpers::test::helper::assert_last_event;
use composable_traits::fnft::FinancialNft;
use frame_support::{
	assert_ok,
	traits::tokens::nonfungibles::{Create, Inspect, Mutate},
};
use std::{
	collections::{BTreeMap, BTreeSet},
	fmt,
};

use crate::{
	pallet::{Event as NftEvent, Instance, OwnerInstances},
	test::{
		mock::{Event, MockRuntime, Nft},
		ALICE, BOB,
	},
	AccountIdOf, FinancialNftCollectionIdOf, FinancialNftInstanceIdOf, Pallet,
};

pub const TEST_COLLECTION_ID: u128 = 1;

/// Mints a single NFT into ALICE and checks that it was created properly, returning the id of the
/// newly created NFT.
///
/// NOTE: Only call once per test!
pub(crate) fn mint_nft_and_assert() -> FinancialNftInstanceIdOf<MockRuntime> {
	Nft::create_collection(&TEST_COLLECTION_ID, &ALICE, &BOB).unwrap();
	let created_nft_id = 1_u64;
	assert_ok!(Pallet::<MockRuntime>::mint_into(&TEST_COLLECTION_ID, &created_nft_id, &ALICE));

	assert_last_event::<MockRuntime>(Event::Nft(NftEvent::FinancialNftCreated {
		collection_id: TEST_COLLECTION_ID,
		instance_id: created_nft_id,
	}));

	assert_eq!(
		OwnerInstances::<MockRuntime>::get(&ALICE).unwrap(),
		BTreeSet::from([(TEST_COLLECTION_ID, created_nft_id)]),
		"ALICE should only have one instance"
	);

	assert_ok!(Pallet::<MockRuntime>::set_attribute(
		&TEST_COLLECTION_ID,
		&created_nft_id,
		&1_u32.encode(),
		&1_u32.encode()
	));

	assert_eq!(
		Instance::<MockRuntime>::get(TEST_COLLECTION_ID, created_nft_id).unwrap(),
		(ALICE, BTreeMap::from([(1_u32.encode(), 1_u32.encode())])),
		"owner should be ALICE"
	);

	created_nft_id
}

/// The id of the NFT minted in [`mint_into_and_assert`].
const NEW_NFT_ID: FinancialNftInstanceIdOf<MockRuntime> = 1;
/// Mints a single NFT with an instance id of `1` into ALICE and checks that it was created
/// properly, returning the id of the newly created NFT for convenience.
///
/// NOTE: Only call once per test!
pub(crate) fn mint_into_and_assert() -> FinancialNftInstanceIdOf<MockRuntime> {
	Nft::create_collection(&TEST_COLLECTION_ID, &ALICE, &BOB).unwrap();
	Pallet::<MockRuntime>::mint_into(&TEST_COLLECTION_ID, &NEW_NFT_ID, &ALICE).unwrap();

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

	NEW_NFT_ID
}

/// Mints many NFTs into the specified account and checks that they were created properly,
/// returning the ids of the newly created NFTs.
pub(crate) fn mint_many_nfts_and_assert<const AMOUNT: usize>(
	who: AccountIdOf<MockRuntime>,
	collection: FinancialNftCollectionIdOf<MockRuntime>,
) -> [FinancialNftInstanceIdOf<MockRuntime>; AMOUNT] {
	let new_nfts_ids = [0; AMOUNT].map(|_| {
		let new_nft_id = Pallet::<MockRuntime>::get_next_nft_id(&collection).unwrap();
		Pallet::<MockRuntime>::mint_into(&collection, &new_nft_id, &who).unwrap();

		assert_last_event::<MockRuntime>(Event::Nft(crate::Event::FinancialNftCreated {
			collection_id: collection,
			instance_id: new_nft_id,
		}));

		new_nft_id
	});

	assert_eq!(
		OwnerInstances::<MockRuntime>::get(&who)
			.unwrap()
			.is_superset(&to_btree(collection, &new_nfts_ids)),
		true,
		"the specified owner ({}) should own the specified NFTs",
		who
	);

	new_nfts_ids
}

/// Creates a BTreeSet from the provided [`NftInstanceId`]s.
pub(crate) fn to_btree(
	collection: FinancialNftCollectionIdOf<MockRuntime>,
	nfts: &[FinancialNftInstanceIdOf<MockRuntime>],
) -> BTreeSet<(FinancialNftCollectionIdOf<MockRuntime>, FinancialNftInstanceIdOf<MockRuntime>)> {
	nfts.into_iter().copied().map(|id| (collection, id)).collect()
}

/// Adds the provided attributes to the specified NFT, asserting that the attributes are added
/// successfully and that the owner doesn't change. Also tests the implementation of
/// [`Inspect::attribute`] and [`Inspect::typed_attribute`].
pub(crate) fn add_attributes_and_assert<
	K: Encode,
	V: Encode + Decode + PartialEq + fmt::Debug + Clone,
>(
	class: FinancialNftCollectionIdOf<MockRuntime>,
	instance: &FinancialNftInstanceIdOf<MockRuntime>,
	owner: AccountIdOf<MockRuntime>,
	attributes: &[(K, V)],
) {
	for (key, value) in attributes {
		assert_ok!(Pallet::<MockRuntime>::set_attribute(
			&class,
			instance,
			&key.encode(),
			&value.encode()
		));

		assert_eq!(
			Pallet::<MockRuntime>::attribute(&class, instance, &key.encode()),
			Some(value.encode()),
			"instance should have the expected attribute"
		);

		assert_eq!(
			Pallet::<MockRuntime>::typed_attribute::<K, V>(&class, instance, key),
			Some(value.clone()),
			"instance should have the expected attribute"
		);
	}

	let (found_owner, data) = Instance::<MockRuntime>::get(class, *instance).unwrap();

	assert_eq!(owner, found_owner, "instance owner should be {owner}");

	assert!(
		attributes
			.iter()
			.map(|(k, v)| (k.encode(), v.encode()))
			.collect::<BTreeSet<_>>()
			.difference(&data.into_iter().collect::<BTreeSet<_>>())
			.collect::<Vec<_>>()
			.is_empty(),
		"instance attributes should contain the expected attribute(s)"
	);
}
