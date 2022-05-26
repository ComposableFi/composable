// This file is part of Substrate.

// Copyright (C) 2017-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The tests for the public proposal queue.

use super::*;
use frame_support::traits::{fungible::Mutate as FungibleMutet, fungibles::Mutate};

proptest! {
	#![proptest_config(ProptestConfig::with_cases(1000))]

	#[test]
	fn poor_proposer_should_not_work(
	   asset_id in valid_asset_id(),
	   (balance_a, balance_b) in valid_amounts_without_overflow_2()) {
		new_test_ext().execute_with(|| {
			assert_noop!(propose_set_balance(BOB, asset_id, balance_a, balance_b), BalancesError::<Test, _>::InsufficientBalance);
		});
	}

	#[test]
	fn proposal_with_deposit_below_minimum_should_not_work(
		asset_id in valid_asset_id(),
		balance in valid_amounts_without_overflow_1()) {
		new_test_ext().execute_with(|| {
			assert_noop!(propose_set_balance(BOB, asset_id, balance, 0), Error::<Test>::ValueLow);
		});
	}

	#[test]
	fn invalid_seconds_upper_bound_should_not_work(
		asset_id in valid_asset_id(),
		(balance1, balance2) in valid_amounts_without_overflow_2()) {
		new_test_ext().execute_with(|| {
			Balances::mint_into(&BOB, (balance1 + balance2)).expect("always can mint in test");
			assert_ok!(propose_set_balance_and_note_2(BOB, asset_id, balance1 / 2 ,  balance2 / 2));
			assert_noop!(Democracy::second(Origin::signed(CHARLIE), 0, 0), Error::<Test>::WrongUpperBound);
		});
	}

	#[test]
	fn backing_for_should_work(
		asset_id in valid_asset_id(),
		(balance1, balance2, balance3) in valid_amounts_without_overflow_3()) {
		new_test_ext().execute_with(|| {
			Balances::mint_into(&BOB, ((balance1 / 3) + (balance2 / 3) + (balance3 / 3) )).expect("always can mint in test");
			assert_ok!(propose_set_balance_and_note_2(BOB, asset_id, balance1 / 3, balance1 / 3));
			assert_ok!(propose_set_balance_and_note_2(BOB, asset_id, balance2 / 3, balance2 / 3));
			assert_ok!(propose_set_balance_and_note_2(BOB, asset_id, balance3 / 3, balance3 / 3));
			assert_eq!(Democracy::backing_for(0), Some(balance1 / 3));
			assert_eq!(Democracy::backing_for(1), Some(balance2 / 3));
			assert_eq!(Democracy::backing_for(2), Some(balance3 / 3));
		});
	}

	#[test]
	fn deposit_for_proposals_should_be_taken(
		asset_id in valid_asset_id(),
		balance in valid_amounts_without_overflow_1()) {
		new_test_ext().execute_with(|| {
			Balances::mint_into(&ALICE, balance / 3).expect("always can mint in test");
			Balances::mint_into(&BOB, balance / 3).expect("always can mint in test");
			Balances::mint_into(&DARWIN, balance / 5 ).expect("always can mint in test");
			let free_balance_1 = Balances::free_balance(ALICE);
			let free_balance_2 = Balances::free_balance(BOB);
			let free_balance_3 = Balances::free_balance(DARWIN);
			assert_ok!(propose_set_balance_and_note_2(ALICE, asset_id, balance / 10, balance / 10));
			assert_ok!(Democracy::second(Origin::signed(BOB), 0, u32::MAX));
			assert_ok!(Democracy::second(Origin::signed(BOB), 0, u32::MAX));
			assert_ok!(Democracy::second(Origin::signed(BOB), 0, u32::MAX));
			assert_ok!(Democracy::second(Origin::signed(DARWIN), 0, u32::MAX));
			assert_eq!(Balances::free_balance(ALICE), free_balance_1 - (balance / 10) );
			assert_eq!(Balances::free_balance(BOB), free_balance_2 - (balance / 10) * 3 );
			assert_eq!(Balances::free_balance(DARWIN), free_balance_3- (balance / 10) );
		});
	}

	#[test]
	fn deposit_for_proposals_should_be_returned(
		asset_id in valid_asset_id(),
		balance in valid_amounts_without_overflow_1()) {
		new_test_ext().execute_with(|| {
			Balances::mint_into(&ALICE, balance / 3).expect("always can mint in test");
			Balances::mint_into(&BOB, balance / 5).expect("always can mint in test");
			Balances::mint_into(&DARWIN, balance / 3 ).expect("always can mint in test");
			assert_ok!(propose_set_balance_and_note_2(ALICE, asset_id, balance / 10, balance / 10));
			assert_ok!(Democracy::second(Origin::signed(BOB), 0, u32::MAX));
			assert_ok!(Democracy::second(Origin::signed(DARWIN), 0, u32::MAX));
			assert_ok!(Democracy::second(Origin::signed(DARWIN), 0, u32::MAX));
			assert_ok!(Democracy::second(Origin::signed(DARWIN), 0, u32::MAX));
			let free_balance_1 = Balances::free_balance(ALICE);
			let free_balance_2 = Balances::free_balance(BOB);
			let free_balance_3 = Balances::free_balance(DARWIN);
			fast_forward_to(3);
			assert_eq!(Balances::free_balance(ALICE), free_balance_1 + (balance / 10));
			assert_eq!(Balances::free_balance(BOB), free_balance_2 + (balance / 10));
			assert_eq!(Balances::free_balance(DARWIN), free_balance_3 + ((balance / 10) * 3));
		});
	}

	#[test]
	fn blacklisting_should_work(
		asset_id in valid_asset_id(),
		balance in valid_amounts_without_overflow_1()) {
		new_test_ext().execute_with(|| {
			System::set_block_number(0);
			let hash = set_balance_proposal_hash( balance / 4);
			Balances::mint_into(&DARWIN, balance / 2).expect("always can mint in test");
			assert_ok!(propose_set_balance_and_note_2(DARWIN, asset_id, balance / 4, balance / 4));
			assert_ok!(propose_set_balance_and_note_2(DARWIN, asset_id, balance / 5, balance / 5));
			assert_noop!(
				Democracy::blacklist(Origin::signed(DARWIN), hash.clone(), asset_id, None),
				BadOrigin
			);
			assert_ok!(Democracy::blacklist(Origin::root(), hash, asset_id, None));
			assert_eq!(Democracy::backing_for(0), None);
			assert_eq!(Democracy::backing_for(1), Some(balance / 5));
			assert_noop!(propose_set_balance_and_note_2(DARWIN, asset_id, balance / 4, balance / 4), Error::<Test>::ProposalBlacklisted);
			fast_forward_to(2);
			let hash = set_balance_proposal_hash(balance / 5);
			assert_ok!(Democracy::referendum_status(0));
			assert_ok!(Democracy::blacklist(Origin::root(), hash, asset_id, Some(0)));
			assert_noop!(Democracy::referendum_status(0), Error::<Test>::ReferendumInvalid);
		});
	}

	#[test]
	fn runners_up_should_come_after(
		asset_id in valid_asset_id(),
		balance in valid_amounts_without_overflow_1()) {
		new_test_ext().execute_with(|| {
			System::set_block_number(0);
			Balances::mint_into(&BOB, balance / 2).expect("always can mint in test");
			Tokens::mint_into(asset_id, &BOB, balance).expect("always can mint in test");
			assert_ok!(propose_set_balance_and_note_2(BOB,asset_id, balance / 10, balance / 10));
			assert_ok!(propose_set_balance_and_note_2(BOB,asset_id, balance / 15, balance/ 15));
			assert_ok!(propose_set_balance_and_note_2(BOB,asset_id, balance / 30, balance / 30));
			fast_forward_to(2);
			assert_ok!(Democracy::vote(Origin::signed(BOB), 0, aye(BOB)));
			fast_forward_to(4);
			assert_ok!(Democracy::vote(Origin::signed(BOB), 1, aye(BOB)));
			fast_forward_to(6);
			assert_ok!(Democracy::vote(Origin::signed(BOB), 2, aye(BOB)));
		});
	}

	#[test]
	fn cancel_proposal_should_work(
		asset_id in valid_asset_id(),
		balance in valid_amounts_without_overflow_1()) {
		new_test_ext().execute_with(|| {
			System::set_block_number(0);
			Balances::mint_into(&BOB, balance / 2).expect("always can mint in test");
			assert_ok!(propose_set_balance_and_note_2(BOB,asset_id, balance / 10, balance / 10));
			assert_ok!(propose_set_balance_and_note_2(BOB,asset_id, balance / 15, balance/ 15));
			assert_noop!(Democracy::cancel_proposal(Origin::signed(1), 0), BadOrigin);
			assert_ok!(Democracy::cancel_proposal(Origin::root(), 0));
			assert_eq!(Democracy::backing_for(0), None);
			assert_eq!(Democracy::backing_for(1), Some(balance / 15));
		});
	}

}

#[test]
fn poor_seconder_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(propose_set_balance_and_note(2, 2, 11));
		assert_noop!(
			Democracy::second(Origin::signed(1), 0, u32::MAX),
			BalancesError::<Test, _>::InsufficientBalance
		);
	});
}
