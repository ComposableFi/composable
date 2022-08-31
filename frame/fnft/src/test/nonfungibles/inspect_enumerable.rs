use frame_support::traits::tokens::nonfungibles::{Create, InspectEnumerable};
use std::collections::BTreeSet;

use crate::test::{
	mock::{new_test_ext, Nft},
	prelude::*,
	ALICE, BOB,
};

#[test]
pub(crate) fn test() {
	new_test_ext().execute_with(|| {
		let first_collection = 1_u16;
		let second_collection = 2_u16;
		Nft::create_collection(&first_collection, &ALICE, &BOB).unwrap();
		Nft::create_collection(&second_collection, &BOB, &ALICE).unwrap();
		let first_collection_items_alice = mint_many_nfts_and_assert::<3>(ALICE, first_collection);
		let second_collection_items_alice =
			mint_many_nfts_and_assert::<6>(ALICE, second_collection);
		let first_collection_items_bob = mint_many_nfts_and_assert::<20>(BOB, first_collection);
		let second_collection_items_bob = mint_many_nfts_and_assert::<1>(BOB, second_collection);

		assert_eq!(
			<Nft as InspectEnumerable<u128>>::collections().collect::<BTreeSet<_>>(),
			BTreeSet::from([first_collection, second_collection])
		);

		assert_eq!(
			<Nft as InspectEnumerable<u128>>::items(&first_collection).collect::<BTreeSet<_>>(),
			BTreeSet::from_iter(
				first_collection_items_alice
					.iter()
					.map(|i| *i)
					.chain(first_collection_items_bob.iter().map(|i| *i))
			)
		);

		assert_eq!(
			<Nft as InspectEnumerable<u128>>::items(&second_collection).collect::<BTreeSet<_>>(),
			BTreeSet::from_iter(
				second_collection_items_alice
					.iter()
					.map(|i| *i)
					.chain(second_collection_items_bob.iter().map(|i| *i))
			)
		);

		assert_eq!(
			<Nft as InspectEnumerable<u128>>::owned(&ALICE).collect::<BTreeSet<_>>(),
			BTreeSet::from_iter(
				first_collection_items_alice
					.iter()
					.map(|i| (first_collection, *i))
					.chain(second_collection_items_alice.iter().map(|i| (second_collection, *i)))
			),
			"Iteration must work for owned instances"
		);

		assert_eq!(
			<Nft as InspectEnumerable<u128>>::owned(&BOB).collect::<BTreeSet<_>>(),
			BTreeSet::from_iter(
				first_collection_items_bob
					.iter()
					.map(|i| (first_collection, *i))
					.chain(second_collection_items_bob.iter().map(|i| (second_collection, *i)))
			),
			"Iteration must work for owned instances"
		);

		assert_eq!(
			<Nft as InspectEnumerable<u128>>::owned_in_collection(&first_collection, &ALICE)
				.collect::<BTreeSet<_>>(),
			BTreeSet::from(first_collection_items_alice),
			"Iteration must work for owned instances in collection"
		);

		assert_eq!(
			<Nft as InspectEnumerable<u128>>::owned_in_collection(&second_collection, &ALICE)
				.collect::<BTreeSet<_>>(),
			BTreeSet::from(second_collection_items_alice),
			"Iteration must work for owned instances in collection"
		);

		assert_eq!(
			<Nft as InspectEnumerable<u128>>::owned_in_collection(&first_collection, &BOB)
				.collect::<BTreeSet<_>>(),
			BTreeSet::from(first_collection_items_bob),
			"Iteration must work for owned instances in collection"
		);

		assert_eq!(
			<Nft as InspectEnumerable<u128>>::owned_in_collection(&second_collection, &BOB)
				.collect::<BTreeSet<_>>(),
			BTreeSet::from(second_collection_items_bob),
			"Iteration must work for owned instances in collection"
		);
	})
}
