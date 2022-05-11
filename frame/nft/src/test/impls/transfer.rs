use std::{
	collections::{BTreeMap, BTreeSet},
	iter,
};

use codec::Encode;
use composable_tests_helpers::test::{
	block::process_and_progress_blocks, helper::assert_last_event,
};
use composable_traits::financial_nft::{FinancialNftProvider, NftClass};
use frame_support::{
	assert_ok,
	traits::tokens::nonfungibles::{Inspect, Transfer},
};

use crate::{
	test::{
		mint_nft_and_assert,
		mock::{new_test_ext, Event, Test},
		ALICE, BOB, CHARLIE,
	},
	AccountIdOf, Instance, NftInstanceId, OwnerInstances, Pallet,
};

#[test]
fn simple() {
	//! Tests a simple transfer between 2 accounts, with only 1 total NFT existing.
	new_test_ext().execute_with(|| {
		let created_nft_id = mint_nft_and_assert();

		process_and_progress_blocks::<Pallet<Test>, Test>(10);

		assert_eq!(
			Pallet::<Test>::owner(&NftClass::STAKING, &created_nft_id),
			Some(ALICE),
			"owner before transfer should be ALICE"
		);

		assert_ok!(Pallet::<Test>::transfer(&NftClass::STAKING, &created_nft_id, &BOB));

		process_and_progress_blocks::<Pallet<Test>, Test>(10);

		assert_eq!(
			OwnerInstances::<Test>::get(&BOB).unwrap(),
			BTreeSet::from([(NftClass::STAKING, created_nft_id)]),
			"BOB should only have one NFT after transfer"
		);

		assert_eq!(
			OwnerInstances::<Test>::get(&ALICE).unwrap(),
			BTreeSet::from([]),
			"ALICE should not have any NFTs after transfer"
		);

		assert_eq!(
			Instance::<Test>::get(&(NftClass::STAKING, created_nft_id)),
			Some((BOB, BTreeMap::from([(1u32.encode(), 1u32.encode())]))),
			"owner of transfered NFT should be BOB after transfer"
		);

		assert_eq!(
			Pallet::<Test>::owner(&NftClass::STAKING, &created_nft_id),
			Some(BOB),
			"owner of transfered NFT should be BOB after transfer"
		);
	})
}

#[test]
fn roundtrip() {
	//! Tests a roundtrip transfer between 2 accounts, asserting that the storage is the same after the roundtrip.
	new_test_ext().execute_with(|| {
		let alices_nfts = mint_many_nfts_and_assert(ALICE, 50);
		let _bobs_nfts = mint_many_nfts_and_assert(BOB, 50);

		let alice_storage_before_transfer = OwnerInstances::<Test>::get(&ALICE).unwrap();
		let bob_storage_before_transfer = OwnerInstances::<Test>::get(&BOB).unwrap();

		process_and_progress_blocks::<Pallet<Test>, Test>(10);

		// send one of ALICE's NFTs to BOB
		assert_ok!(Pallet::<Test>::transfer(&NftClass::STAKING, &alices_nfts[0], &BOB));

		process_and_progress_blocks::<Pallet<Test>, Test>(10);

		// send said NFT back
		assert_ok!(Pallet::<Test>::transfer(&NftClass::STAKING, &alices_nfts[0], &ALICE));

		let alice_storage_after_transfer = OwnerInstances::<Test>::get(&ALICE).unwrap();
		let bob_storage_after_transfer = OwnerInstances::<Test>::get(&BOB).unwrap();

		assert_eq!(alice_storage_before_transfer, alice_storage_after_transfer);
		assert_eq!(bob_storage_before_transfer, bob_storage_after_transfer);
	})
}

#[test]
fn many() {
	//! Tests the transfer of many NFTs between multiple accounts.
	new_test_ext().execute_with(transfer_many_test);

	// in a separate function because rustfmt dies if the content of the test is in a closure
	fn transfer_many_test() {
		// mint 10 NFTs into ALICE
		let alices_nfts = mint_many_nfts_and_assert(ALICE, 10);
		// mint 10 NFTs into BOB
		let bobs_nfts = mint_many_nfts_and_assert(BOB, 10);
		// mint 10 NFTs into CHARLIE
		let charlies_nfts = mint_many_nfts_and_assert(CHARLIE, 10);

		// NFT ownership before transfer:
		//
		// ALICE:   A A A A A A A A A A
		// BOB:     B B B B B B B B B B
		// CHARLIE: C C C C C C C C C C

		// transfer one of ALICE's NFTs to BOB
		{
			assert_ok!(Pallet::<Test>::transfer(&NftClass::STAKING, &alices_nfts[0], &BOB));
			assert_last_event::<Test>(Event::Nft(crate::Event::NftTransferred {
				class_id: NftClass::STAKING,
				instance_id: alices_nfts[0],
				to: BOB,
			}));

			assert_eq!(
				OwnerInstances::<Test>::get(&BOB).unwrap(),
				bobs_nfts
					.iter()
					.chain(std::iter::once(&alices_nfts[0]))
					.map(|&id| (NftClass::STAKING, id))
					.collect(),
				"BOB should own their original NFTs + the one transferred from ALICE"
			);

			process_and_progress_blocks::<Pallet<Test>, Test>(10);

			assert_eq!(
				OwnerInstances::<Test>::get(&ALICE).unwrap(),
				alices_nfts
					.iter()
					.filter_map(|&id| id.ne(&alices_nfts[0]).then(|| (NftClass::STAKING, id)))
					.collect(),
				"ALICE should no longer own the traded NFT after transfer"
			);

			process_and_progress_blocks::<Pallet<Test>, Test>(10);
		}

		// NFT ownership after first transfer:
		//
		// ALICE:   A A A A A A A A A
		// BOB:     A B B B B B B B B B B
		// CHARLIE: C C C C C C C C C C

		// transfer all of CHARLIES's NFTs to BOB
		{
			for nft_id in charlies_nfts.iter() {
				assert_ok!(Pallet::<Test>::transfer(&NftClass::STAKING, nft_id, &BOB));
				assert_last_event::<Test>(Event::Nft(crate::Event::NftTransferred {
					class_id: NftClass::STAKING,
					instance_id: *nft_id,
					to: BOB,
				}));
				process_and_progress_blocks::<Pallet<Test>, Test>(2);
			}

			assert_eq!(
                OwnerInstances::<Test>::get(&BOB).unwrap(),
                bobs_nfts
                    .iter()
                    .chain(std::iter::once(&alices_nfts[0]))
                    .chain(&charlies_nfts)
                    .map(|&id| (NftClass::STAKING, id))
                    .collect(),
                "BOB should own their original NFTs + the one transferred from ALICE + all of the ones transferred from CHARLIE"
            );

			process_and_progress_blocks::<Pallet<Test>, Test>(10);

			assert_eq!(
				OwnerInstances::<Test>::get(&ALICE).unwrap(),
				alices_nfts
					.iter()
					.filter_map(|&id| id.ne(&alices_nfts[0]).then(|| (NftClass::STAKING, id)))
					.collect(),
				"CHARLIE should have no NFTs left after transferring them all to BOB"
			);
		}

		// NFT ownership after second transfer:
		//
		// ALICE:   A A A A A A A A A
		// BOB:     A B B B B B B B B B B C C C C C C C C C C
		// CHARLIE:

		// transfer one of (what was originally CHARLIES's) NFTs from BOB to ALICE
		{
			assert_ok!(Pallet::<Test>::transfer(&NftClass::STAKING, &charlies_nfts[9], &ALICE));
			assert_last_event::<Test>(Event::Nft(crate::Event::NftTransferred {
				class_id: NftClass::STAKING,
				instance_id: charlies_nfts[9],
				to: ALICE,
			}));

			let should_be_bobs_nfts = bobs_nfts
				.iter()
				.chain(iter::once(&alices_nfts[0]))
				.chain(charlies_nfts.iter().filter(|&id| id.ne(&charlies_nfts[9])))
				.map(|&id| (NftClass::STAKING, id))
				.collect();

			assert_eq!(
                OwnerInstances::<Test>::get(&BOB).unwrap(),
                should_be_bobs_nfts,
                "BOB should own their original NFTs + the one transferred from ALICE + all but one of the ones transferred from CHARLIE"
            );

			process_and_progress_blocks::<Pallet<Test>, Test>(10);

			assert_eq!(
                OwnerInstances::<Test>::get(&ALICE).unwrap(),
                alices_nfts
                    .iter()
                    .filter(|&id| id.ne(&alices_nfts[0]))
                    .chain(iter::once(&charlies_nfts[9]))
                    .map(|&id| (NftClass::STAKING, id))
                    .collect(),
                "ALICE should have their original NFTs except for the one transferred to BOB + one of (what was originally CHARLIES's) NFTs from BOB"
            );
		}

		// NFT ownership after second transfer:
		//
		// ALICE:   A A A A A A A A A C
		// BOB:     A B B B B B B B B B B C C C C C C C C C
		// CHARLIE:

		// transfer one of (what was originally CHARLIES's) NFTs from ALICE back to CHARLIE
		{
			assert_ok!(Pallet::<Test>::transfer(&NftClass::STAKING, &charlies_nfts[9], &CHARLIE),);
			assert_last_event::<Test>(Event::Nft(crate::Event::NftTransferred {
				class_id: NftClass::STAKING,
				instance_id: charlies_nfts[9],
				to: CHARLIE,
			}));

			assert_eq!(
				OwnerInstances::<Test>::get(&ALICE).unwrap(),
				alices_nfts
					.iter()
					.filter_map(|&id| id.ne(&alices_nfts[0]).then(|| (NftClass::STAKING, id)))
					.collect(),
				"ALICE should have their original NFTs except for the one transferred to BOB previously"
			);

			process_and_progress_blocks::<Pallet<Test>, Test>(10);

			assert_eq!(
				OwnerInstances::<Test>::get(&CHARLIE).unwrap(),
				[(NftClass::STAKING, charlies_nfts[9])].into_iter().collect(),
				"CHARLIE should have one of their original NFTs"
			);
		}

		// NFT ownership after second transfer:
		//
		// ALICE:   A A A A A A A A A
		// BOB:     A B B B B B B B B B B C C C C C C C C C
		// CHARLIE: C
	}
}

/// Mints many NFTs into the specified account and checks that they were created properly,
/// returning the ids of the newly created NFTs.
///
/// NOTE: Only call once per test, per account!
fn mint_many_nfts_and_assert(who: AccountIdOf<Test>, amount: u32) -> Vec<NftInstanceId> {
	let new_nfts_ids = (0..amount)
		.map(|_| {
			let new_nft_id =
				Pallet::<Test>::mint_nft(&NftClass::STAKING, &who, &1u32, &1u32).unwrap();

			assert_last_event::<Test>(Event::Nft(crate::Event::NftCreated {
				class_id: NftClass::STAKING,
				instance_id: new_nft_id,
			}));

			new_nft_id
		})
		.collect::<Vec<_>>();

	assert_eq!(
		OwnerInstances::<Test>::get(&who).unwrap(),
		new_nfts_ids.iter().map(|&id| (NftClass::STAKING, id)).collect(),
		"the specified owner ({}) should own the specified NFTs",
		who
	);

	new_nfts_ids
}
