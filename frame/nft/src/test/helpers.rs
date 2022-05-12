use codec::Encode;
use composable_tests_helpers::test::helper::assert_last_event;
use composable_traits::financial_nft::{FinancialNftProvider, NftClass};
use std::collections::{BTreeMap, BTreeSet};

use crate::{
	pallet::{ClassInstances, Event as NftEvent, Instance, OwnerInstances},
	test::{
		mock::{Event, MockRuntime},
		ALICE,
	},
	AccountIdOf, NftInstanceId, Pallet,
};

// Mints a single NFT into ALICE and checks that it was created properly, returning the id of the
/// newly created NFT.
///
/// NOTE: Only call once per test!
pub(crate) fn mint_nft_and_assert() -> NftInstanceId {
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

/// Mints many NFTs into the specified account and checks that they were created properly,
/// returning the ids of the newly created NFTs.
///
/// NOTE: Only call once per test, per account!
pub(crate) fn mint_many_nfts_and_assert(
	who: AccountIdOf<MockRuntime>,
	amount: u32,
) -> Vec<NftInstanceId> {
	let new_nfts_ids = (0..amount)
		.map(|_| {
			let new_nft_id =
				Pallet::<MockRuntime>::mint_nft(&NftClass::STAKING, &who, &1u32, &1u32).unwrap();

			assert_last_event::<MockRuntime>(Event::Nft(crate::Event::NftCreated {
				class_id: NftClass::STAKING,
				instance_id: new_nft_id,
			}));

			new_nft_id
		})
		.collect::<Vec<_>>();

	assert_eq!(
		OwnerInstances::<MockRuntime>::get(&who).unwrap(),
		new_nfts_ids.iter().map(|&id| (NftClass::STAKING, id)).collect(),
		"the specified owner ({}) should own the specified NFTs",
		who
	);

	new_nfts_ids
}
