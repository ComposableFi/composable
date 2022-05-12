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
	assert_err, assert_ok,
	traits::tokens::nonfungibles::{Inspect, Transfer},
};
use sp_runtime::{DispatchError, ModuleError};

use crate::{
	test::{
		helpers::mint_many_nfts_and_assert,
		helpers::mint_nft_and_assert,
		mock::{new_test_ext, Event, MockRuntime},
		ALICE, BOB, CHARLIE,
	},
	AccountIdOf, Instance, NftInstanceId, OwnerInstances, Pallet,
};

/// Tests a simple transfer between 2 accounts, with only 1 total NFT existing.
#[test]
fn simple() {
	new_test_ext().execute_with(|| {
		let created_nft_id = mint_nft_and_assert();

		process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(10);

		assert_eq!(
			Pallet::<MockRuntime>::owner(&NftClass::STAKING, &created_nft_id),
			Some(ALICE),
			"owner before transfer should be ALICE"
		);

		assert_ok!(Pallet::<MockRuntime>::transfer(&NftClass::STAKING, &created_nft_id, &BOB));

		process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(10);

		assert_eq!(
			OwnerInstances::<MockRuntime>::get(&BOB).unwrap(),
			BTreeSet::from([(NftClass::STAKING, created_nft_id)]),
			"BOB should only have one NFT after transfer"
		);

		assert_eq!(
			OwnerInstances::<MockRuntime>::get(&ALICE).unwrap(),
			BTreeSet::from([]),
			"ALICE should not have any NFTs after transfer"
		);

		assert_eq!(
			Instance::<MockRuntime>::get(&(NftClass::STAKING, created_nft_id)),
			Some((BOB, BTreeMap::from([(1u32.encode(), 1u32.encode())]))),
			"owner of transfered NFT should be BOB after transfer"
		);

		assert_eq!(
			Pallet::<MockRuntime>::owner(&NftClass::STAKING, &created_nft_id),
			Some(BOB),
			"owner of transfered NFT should be BOB after transfer"
		);
	})
}

/// Tests a roundtrip transfer between 2 accounts, asserting that the storage is the same after
/// the roundtrip.
#[test]
fn roundtrip() {
	new_test_ext().execute_with(|| {
		let alices_nfts = mint_many_nfts_and_assert(ALICE, 50);
		let _bobs_nfts = mint_many_nfts_and_assert(BOB, 50);

		let alice_storage_before_transfer = OwnerInstances::<MockRuntime>::get(&ALICE).unwrap();
		let bob_storage_before_transfer = OwnerInstances::<MockRuntime>::get(&BOB).unwrap();

		process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(10);

		// send one of ALICE's NFTs to BOB
		assert_ok!(Pallet::<MockRuntime>::transfer(&NftClass::STAKING, &alices_nfts[0], &BOB));

		process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(10);

		// send said NFT back
		assert_ok!(Pallet::<MockRuntime>::transfer(&NftClass::STAKING, &alices_nfts[0], &ALICE));

		let alice_storage_after_transfer = OwnerInstances::<MockRuntime>::get(&ALICE).unwrap();
		let bob_storage_after_transfer = OwnerInstances::<MockRuntime>::get(&BOB).unwrap();

		assert_eq!(alice_storage_before_transfer, alice_storage_after_transfer);
		assert_eq!(bob_storage_before_transfer, bob_storage_after_transfer);
	})
}

/// Tests the transfer of many NFTs between multiple accounts.
#[test]
fn many() {
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
		// ALICE:   A0 A1 A2 A3 A4 A5 A6 A7 A8 A9
		// BOB:     B0 B1 B2 B3 B4 B5 B6 B7 B8 B9
		// CHARLIE: C0 C1 C2 C3 C4 C5 C6 C7 C8 C9

		// transfer one of ALICE's NFTs to BOB
		{
			assert_ok!(Pallet::<MockRuntime>::transfer(&NftClass::STAKING, &alices_nfts[0], &BOB));
			assert_last_event::<MockRuntime>(Event::Nft(crate::Event::NftTransferred {
				class_id: NftClass::STAKING,
				instance_id: alices_nfts[0],
				to: BOB,
			}));

			assert_eq!(
				OwnerInstances::<MockRuntime>::get(&BOB).unwrap(),
				bobs_nfts
					.iter()
					.chain(std::iter::once(&alices_nfts[0]))
					.map(|&id| (NftClass::STAKING, id))
					.collect(),
				"BOB should own their original NFTs + the one transferred from ALICE"
			);

			process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(10);

			assert_eq!(
				OwnerInstances::<MockRuntime>::get(&ALICE).unwrap(),
				alices_nfts
					.iter()
					.filter_map(|&id| id.ne(&alices_nfts[0]).then(|| (NftClass::STAKING, id)))
					.collect(),
				"ALICE should no longer own the traded NFT after transfer"
			);

			process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(10);
		}

		// NFT ownership after first transfer:
		//
		// ALICE:   A1 A2 A3 A4 A5 A6 A7 A8 A9
		// BOB:     A0 B0 B1 B2 B3 B4 B5 B6 B7 B8 B9
		// CHARLIE: C0 C1 C2 C3 C4 C5 C6 C7 C8 C9

		// transfer all of CHARLIES's NFTs to BOB
		{
			for nft_id in charlies_nfts.iter() {
				assert_ok!(Pallet::<MockRuntime>::transfer(&NftClass::STAKING, nft_id, &BOB));
				assert_last_event::<MockRuntime>(Event::Nft(crate::Event::NftTransferred {
					class_id: NftClass::STAKING,
					instance_id: *nft_id,
					to: BOB,
				}));
				process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(2);
			}

			assert_eq!(
                OwnerInstances::<MockRuntime>::get(&BOB).unwrap(),
                bobs_nfts
                    .iter()
                    .chain(std::iter::once(&alices_nfts[0]))
                    .chain(&charlies_nfts)
                    .map(|&id| (NftClass::STAKING, id))
                    .collect(),
                "BOB should own their original NFTs + the one transferred from ALICE + all of the ones transferred from CHARLIE"
            );

			process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(10);

			assert_eq!(
				OwnerInstances::<MockRuntime>::get(&ALICE).unwrap(),
				alices_nfts
					.iter()
					.filter_map(|&id| id.ne(&alices_nfts[0]).then(|| (NftClass::STAKING, id)))
					.collect(),
				"CHARLIE should have no NFTs left after transferring them all to BOB"
			);
		}

		// NFT ownership after second transfer:
		//
		// ALICE:   A1 A2 A3 A4 A5 A6 A7 A8 A9
		// BOB:     A0 B0 B1 B2 B3 B4 B5 B6 B7 B8 B9 C0 C1 C2 C3 C4 C5 C6 C7 C8 C9
		// CHARLIE:

		// transfer one of (what was originally CHARLIES's) NFTs from BOB to ALICE
		{
			assert_ok!(Pallet::<MockRuntime>::transfer(
				&NftClass::STAKING,
				&charlies_nfts[9],
				&ALICE
			));
			assert_last_event::<MockRuntime>(Event::Nft(crate::Event::NftTransferred {
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
                OwnerInstances::<MockRuntime>::get(&BOB).unwrap(),
                should_be_bobs_nfts,
                "BOB should own their original NFTs + the one transferred from ALICE + all but one of the ones transferred from CHARLIE"
            );

			process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(10);

			assert_eq!(
                OwnerInstances::<MockRuntime>::get(&ALICE).unwrap(),
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
		// ALICE:   A1 A2 A3 A4 A5 A6 A7 A8 A9 C9
		// BOB:     A0 B0 B1 B2 B3 B4 B5 B6 B7 B8 B9 C0 C1 C2 C3 C4 C5 C6 C7 C8
		// CHARLIE:

		// transfer one of (what was originally CHARLIES's) NFTs from ALICE back to CHARLIE
		{
			assert_ok!(Pallet::<MockRuntime>::transfer(
				&NftClass::STAKING,
				&charlies_nfts[9],
				&CHARLIE
			),);
			assert_last_event::<MockRuntime>(Event::Nft(crate::Event::NftTransferred {
				class_id: NftClass::STAKING,
				instance_id: charlies_nfts[9],
				to: CHARLIE,
			}));

			assert_eq!(
				OwnerInstances::<MockRuntime>::get(&ALICE).unwrap(),
				alices_nfts
					.iter()
					.filter_map(|&id| id.ne(&alices_nfts[0]).then(|| (NftClass::STAKING, id)))
					.collect(),
				"ALICE should have their original NFTs except for the one transferred to BOB previously"
			);

			process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(10);

			assert_eq!(
				OwnerInstances::<MockRuntime>::get(&CHARLIE).unwrap(),
				[(NftClass::STAKING, charlies_nfts[9])].into_iter().collect(),
				"CHARLIE should have one of their original NFTs"
			);
		}

		// NFT ownership after second transfer:
		//
		// ALICE:   A1 A2 A3 A4 A5 A6 A7 A8 A9
		// BOB:     A0 B0 B1 B2 B3 B4 B5 B6 B7 B8 B9 C0 C1 C2 C3 C4 C5 C6 C7 C8
		// CHARLIE: C9
	}
}

/// Tests that an NFT that doesn't exist can't be transferred.
#[test]
fn instance_not_found() {
	new_test_ext().execute_with(|| {
		assert_err!(
			Pallet::<MockRuntime>::transfer(&NftClass::STAKING, &1, &ALICE),
			DispatchError::from(crate::Error::<MockRuntime>::InstanceNotFound)
		)
	});
}
