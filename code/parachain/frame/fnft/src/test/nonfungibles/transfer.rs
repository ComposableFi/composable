use std::collections::{BTreeMap, BTreeSet};

use codec::Encode;
use composable_tests_helpers::test::{block::process_and_progress_blocks, helper::RuntimeTrait};
use composable_traits::{account_proxy::ProxyType, fnft::FinancialNft};
use frame_support::{
	assert_noop, assert_ok,
	traits::tokens::nonfungibles::{Create, Inspect, Transfer},
};
use sp_runtime::DispatchError;

use crate::{
	test::{
		mock::{new_test_ext, MockRuntime, Nft, Proxy, RuntimeEvent},
		prelude::{TEST_COLLECTION_ID, *},
		ALICE, BOB, CHARLIE,
	},
	AccountIdOf, FinancialNftInstanceIdOf, Instance, OwnerInstances, Pallet,
};

/// Tests a simple transfer between 2 accounts, with only 1 total NFT existing.
#[test]
fn simple() {
	new_test_ext().execute_with(|| {
		let created_nft_id = mint_nft_and_assert();

		process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(10);

		assert_eq!(
			Nft::owner(&TEST_COLLECTION_ID, &created_nft_id),
			Some(ALICE),
			"owner before transfer should be ALICE"
		);

		assert_ok!(<Nft as Transfer<AccountIdOf<MockRuntime>>>::transfer(
			&TEST_COLLECTION_ID,
			&created_nft_id,
			&BOB
		));

		process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(10);

		assert_eq!(
			OwnerInstances::<MockRuntime>::get(&BOB).unwrap(),
			BTreeSet::from([(TEST_COLLECTION_ID, created_nft_id)]),
			"BOB should only have one NFT after transfer"
		);

		assert_eq!(
			OwnerInstances::<MockRuntime>::get(&ALICE).unwrap(),
			BTreeSet::from([]),
			"ALICE should not have any NFTs after transfer"
		);

		assert_eq!(
			Instance::<MockRuntime>::get(TEST_COLLECTION_ID, created_nft_id),
			Some((BOB, BTreeMap::from([(1_u32.encode(), 1_u32.encode())]))),
			"owner of transferred NFT should be BOB after transfer"
		);

		assert_eq!(
			Nft::owner(&TEST_COLLECTION_ID, &created_nft_id),
			Some(BOB),
			"owner of transferred NFT should be BOB after transfer"
		);
	})
}

/// Tests a roundtrip transfer between 2 accounts, asserting that the storage is the same after
/// the roundtrip.
#[test]
fn roundtrip() {
	new_test_ext().execute_with(|| {
		Nft::create_collection(&TEST_COLLECTION_ID, &ALICE, &BOB).unwrap();
		let [nft_to_trade, ..] = mint_many_nfts_and_assert::<50>(ALICE, TEST_COLLECTION_ID);
		let _bobs_nfts = mint_many_nfts_and_assert::<50>(BOB, TEST_COLLECTION_ID);

		let alice_storage_before_transfer = OwnerInstances::<MockRuntime>::get(&ALICE).unwrap();
		let bob_storage_before_transfer = OwnerInstances::<MockRuntime>::get(&BOB).unwrap();

		process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(10);

		// send one of ALICE's NFTs to BOB
		assert_ok!(<Nft as Transfer<AccountIdOf<MockRuntime>>>::transfer(
			&TEST_COLLECTION_ID,
			&nft_to_trade,
			&BOB
		));
		MockRuntime::assert_last_event(RuntimeEvent::Nft(crate::Event::FinancialNftTransferred {
			collection_id: TEST_COLLECTION_ID,
			instance_id: nft_to_trade,
			to: BOB,
		}));
		// get the asset account of the fNFT
		let asset_account = Nft::asset_account(&TEST_COLLECTION_ID, &nft_to_trade);
		assert_ok!(Proxy::find_proxy(&asset_account, &BOB, Some(ProxyType::Any)));

		process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(10);

		// send said NFT back
		assert_ok!(<Nft as Transfer<AccountIdOf<MockRuntime>>>::transfer(
			&TEST_COLLECTION_ID,
			&nft_to_trade,
			&ALICE
		));
		MockRuntime::assert_last_event(RuntimeEvent::Nft(crate::Event::FinancialNftTransferred {
			collection_id: TEST_COLLECTION_ID,
			instance_id: nft_to_trade,
			to: ALICE,
		}));
		assert_ok!(Proxy::find_proxy(&asset_account, &ALICE, Some(ProxyType::Any)));

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
		Nft::create_collection(&TEST_COLLECTION_ID, &ALICE, &BOB).unwrap();
		// mint 10 NFTs into ALICE
		let alices_nfts = mint_many_nfts_and_assert::<10>(ALICE, TEST_COLLECTION_ID);
		// mint 10 NFTs into BOB
		let bobs_nfts = mint_many_nfts_and_assert::<10>(BOB, TEST_COLLECTION_ID);
		// mint 10 NFTs into CHARLIE
		let charlies_nfts = mint_many_nfts_and_assert::<10>(CHARLIE, TEST_COLLECTION_ID);

		let [a0, a1, a2, a3, a4, a5, a6, a7, a8, a9] = alices_nfts;
		let [b0, b1, b2, b3, b4, b5, b6, b7, b8, b9] = bobs_nfts;
		let [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9] = charlies_nfts;

		fn assert_owners<const AMOUNT: usize>(
			checks: [(AccountIdOf<MockRuntime>, &[FinancialNftInstanceIdOf<MockRuntime>], &str);
				AMOUNT],
		) {
			for (who, nfts, msg) in checks {
				assert_eq!(
					OwnerInstances::<MockRuntime>::get(&who).unwrap(),
					to_btree(TEST_COLLECTION_ID, nfts),
					"{msg}"
				);

				process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(10);
			}
		}

		// transfer one of ALICE's NFTs to BOB
		assert_ok!(<Nft as Transfer<AccountIdOf<MockRuntime>>>::transfer(
			&TEST_COLLECTION_ID,
			&a0,
			&BOB
		));
		MockRuntime::assert_last_event(RuntimeEvent::Nft(crate::Event::FinancialNftTransferred {
			collection_id: TEST_COLLECTION_ID,
			instance_id: a0,
			to: BOB,
		}));

		assert_owners([
			(
				ALICE,
				&[a1, a2, a3, a4, a5, a6, a7, a8, a9],
				"ALICE should no longer own the traded NFT after transfer",
			),
			(
				BOB,
				&[a0, b0, b1, b2, b3, b4, b5, b6, b7, b8, b9],
				"BOB should own their original NFTs + the one transferred from ALICE",
			),
			(CHARLIE, &[c0, c1, c2, c3, c4, c5, c6, c7, c8, c9], "CHARLIE should be unchanged"),
		]);

		// transfer all of CHARLIES's NFTs to BOB
		for nft_id in charlies_nfts.iter() {
			assert_ok!(<Nft as Transfer<AccountIdOf<MockRuntime>>>::transfer(
				&TEST_COLLECTION_ID,
				nft_id,
				&BOB
			));
			MockRuntime::assert_last_event(RuntimeEvent::Nft(
				crate::Event::FinancialNftTransferred {
					collection_id: TEST_COLLECTION_ID,
					instance_id: *nft_id,
					to: BOB,
				},
			));
			process_and_progress_blocks::<Pallet<MockRuntime>, MockRuntime>(2);
		}

		assert_owners([
            (
                ALICE,
                &[a1, a2, a3, a4, a5, a6, a7, a8, a9],
                "ALICE should be unchanged",
            ),
            (
                BOB,
                &[a0, b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, c0, c1, c2, c3, c4, c5, c6, c7, c8, c9],
                "BOB should own their original NFTs + the one transferred from ALICE + all of the ones transferred from CHARLIE",
            ),
            (CHARLIE, &[], "CHARLIE should have no NFTs left after transferring them all to BOB"),
        ]);

		// transfer one of (what was originally CHARLIES's) NFTs from BOB to ALICE
		assert_ok!(<Nft as Transfer<AccountIdOf<MockRuntime>>>::transfer(
			&TEST_COLLECTION_ID,
			&c9,
			&ALICE
		));
		MockRuntime::assert_last_event(RuntimeEvent::Nft(crate::Event::FinancialNftTransferred {
			collection_id: TEST_COLLECTION_ID,
			instance_id: c9,
			to: ALICE,
		}));

		assert_owners([
            (
                ALICE,
                &[a1, a2, a3, a4, a5, a6, a7, a8, a9, c9],
                "ALICE should have their original NFTs except for the one transferred to BOB + one of (what was originally CHARLIES's) NFTs from BOB",
            ),
            (
                BOB,
                &[a0, b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, c0, c1, c2, c3, c4, c5, c6, c7, c8],
                "BOB should own their original NFTs + the one transferred from ALICE + all of the ones transferred from CHARLIE except for the one transferred to ALICE",
            ),
            (CHARLIE, &[], "CHARLIE should be unchanged"),
        ]);

		// transfer one of (what was originally CHARLIES's) NFTs from ALICE back to CHARLIE
		assert_ok!(<Nft as Transfer<AccountIdOf<MockRuntime>>>::transfer(
			&TEST_COLLECTION_ID,
			&c9,
			&CHARLIE
		),);
		MockRuntime::assert_last_event(RuntimeEvent::Nft(crate::Event::FinancialNftTransferred {
			collection_id: TEST_COLLECTION_ID,
			instance_id: c9,
			to: CHARLIE,
		}));

		assert_owners([
            (
                ALICE,
                &[a1, a2, a3, a4, a5, a6, a7, a8, a9],
                "ALICE should have their original NFTs except for the one transferred to BOB previously",
            ),
            (
                BOB,
                &[a0, b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, c0, c1, c2, c3, c4, c5, c6, c7, c8],
                "BOB should own their original NFTs + the one transferred from ALICE + all of the ones transferred from CHARLIE except for the one transferred to ALICE",
            ),
            (CHARLIE, &[c9], "CHARLIE should have one of their original NFTs"),
        ])
	}
}

/// Tests that an NFT that doesn't exist can't be transferred.
#[test]
fn instance_not_found() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			<Nft as Transfer<AccountIdOf<MockRuntime>>>::transfer(&TEST_COLLECTION_ID, &1, &ALICE),
			DispatchError::from(crate::Error::<MockRuntime>::InstanceNotFound)
		);
	});
}
