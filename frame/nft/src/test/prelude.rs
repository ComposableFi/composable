use codec::{Decode, Encode};
use composable_tests_helpers::test::helper::assert_last_event;
pub use composable_traits::nft::NftClass;
use composable_traits::nft::{Key, ReferenceNft, Value};
use frame_support::{
	assert_ok, bounded_btree_map,
	traits::tokens::nonfungibles::{Inspect, Mutate},
};
use std::{
	collections::{BTreeMap, BTreeSet},
	fmt,
};

use crate::{
	pallet::{ClassInstances, Event as NftEvent, Instance, OwnerInstances},
	test::{
		mock::{Event, MockRuntime},
		ALICE,
	},
	AccountIdOf, NftInstanceId, Pallet,
};

/// Mints a single NFT into ALICE and checks that it was created properly, returning the id of the
/// newly created NFT.
///
/// NOTE: Only call once per test!
pub(crate) fn mint_nft_and_assert() -> NftInstanceId {
	let key = Key::from_vec(vec![1_u8]).unwrap();
	let value = Value::from_vec(vec![1_u8]).unwrap();
	let mut fix_me = bounded_btree_map!(key => value);
	let created_nft_id =
		Pallet::<MockRuntime>::mint_new_into(&NftClass::STAKING, &ALICE, fix_me).unwrap();

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
		(ALICE, BTreeMap::from([(1_u32.encode(), 1_u32.encode())])),
		"owner should be ALICE"
	);

	created_nft_id
}

/// The id of the NFT minted in [`mint_into_and_assert`].
const NEW_NFT_ID: NftInstanceId = 1;
/// Mints a single NFT with an instance id of `1` into ALICE and checks that it was created
/// properly, returning the id of the newly created NFT for convenience.
///
/// NOTE: Only call once per test!
pub(crate) fn mint_into_and_assert() -> NftInstanceId {
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

	NEW_NFT_ID
}

/// Mints many NFTs into the specified account and checks that they were created properly,
/// returning the ids of the newly created NFTs.
pub(crate) fn mint_many_nfts_and_assert<const AMOUNT: usize>(
	who: AccountIdOf<MockRuntime>,
) -> [NftInstanceId; AMOUNT] {
	let new_nfts_ids = [0; AMOUNT].map(|_| {
		let new_nft_id = Pallet::<MockRuntime>::get_next_nft_id(&NftClass::STAKING).unwrap();
		Pallet::<MockRuntime>::mint_into(&NftClass::STAKING, &new_nft_id, &who).unwrap();

		assert_last_event::<MockRuntime>(Event::Nft(crate::Event::NftCreated {
			class_id: NftClass::STAKING,
			instance_id: new_nft_id,
		}));

		new_nft_id
	});

	assert_eq!(
		OwnerInstances::<MockRuntime>::get(&who).unwrap(),
		to_btree(&new_nfts_ids),
		"the specified owner ({}) should own the specified NFTs",
		who
	);

	new_nfts_ids
}

/// Creates a BTreeSet from the provided [`NftInstanceId`]s.
///
/// The class for all of the instances is [`NftClass::STAKING`].
pub(crate) fn to_btree(nfts: &[NftInstanceId]) -> BTreeSet<(NftClass, NftInstanceId)> {
	nfts.into_iter().copied().map(|id| (NftClass::STAKING, id)).collect()
}

/// Adds the provided attributes to the specified NFT, asserting that the attributes are added
/// successfully and that the owner doesn't change. Also tests the implementation of
/// [`Inspect::attribute`] and [`Inspect::typed_attribute`].
pub(crate) fn add_attributes_and_assert<
	K: Encode,
	V: Encode + Decode + PartialEq + fmt::Debug + Clone,
>(
	class: &NftClass,
	instance: &NftInstanceId,
	owner: u128,
	attributes: &[(K, V)],
) {
	for (key, value) in attributes {
		assert_ok!(Pallet::<MockRuntime>::set_attribute(
			class,
			instance,
			&key.encode(),
			&value.encode()
		));

		assert_eq!(
			Pallet::<MockRuntime>::attribute(class, instance, &key.encode()),
			Some(value.encode()),
			"instance should have the expected attribute"
		);

		assert_eq!(
			Pallet::<MockRuntime>::typed_attribute::<K, V>(class, instance, key),
			Some(value.clone()),
			"instance should have the expected attribute"
		);
	}

	let (found_owner, data) = Instance::<MockRuntime>::get(&(*class, *instance)).unwrap();

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
