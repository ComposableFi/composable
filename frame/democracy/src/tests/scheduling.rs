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

//! The tests for functionality concerning normal starting, ending and enacting of referenda.

use super::*;
use frame_support::traits::{fungible::Mutate as FungibleMutet, fungibles::Mutate};

proptest! {
	#![proptest_config(ProptestConfig::with_cases(1000))]

	#[test]
	fn simple_passing_should_work(
		asset_id in valid_asset_id(),
		balance1 in valid_amounts_without_overflow_1()) {
		new_test_ext().execute_with(|| {
			Tokens::mint_into(asset_id, &BOB, balance1 / 2).expect("always can mint in test");
			let r = Democracy::inject_referendum(
				2,
				set_balance_proposal_hash_and_note_2( balance1, asset_id),
				VoteThreshold::SuperMajorityApprove,
				0,
			);
			assert_ok!(Democracy::vote(Origin::signed(BOB), r, aye(BOB)));
			assert_eq!(tally(r), Tally { ayes: 1, nays: 0, turnout: 10 });
			assert_eq!(Democracy::lowest_unbaked(), 0);
			next_block();
			next_block();
			assert_eq!(Democracy::lowest_unbaked(), 1);
			assert_eq!(Balances::free_balance(42), balance1);
		});
	}

	#[test]
	fn simple_failing_should_work(
		asset_id in valid_asset_id(),
		balance1 in valid_amounts_without_overflow_1()) {
		new_test_ext().execute_with(|| {
			Tokens::mint_into(asset_id, &BOB, balance1 / 2).expect("always can mint in test");
			let r = Democracy::inject_referendum(
				2,
				set_balance_proposal_hash_and_note_2(balance1, asset_id),
				VoteThreshold::SuperMajorityApprove,
				0,
			);
			assert_ok!(Democracy::vote(Origin::signed(BOB), r, nay(BOB)));
			assert_eq!(tally(r), Tally { ayes: 0, nays: 1, turnout: 10 });

			next_block();
			next_block();

			assert_eq!(Balances::free_balance(42), 0);
		});
	}

	#[test]
	fn ooo_inject_referendums_should_work(
		asset_id in valid_asset_id(),
		(balance1, balance2) in valid_amounts_without_overflow_2()) {
		new_test_ext().execute_with(|| {
			Tokens::mint_into(asset_id, &BOB, balance1 / 2).expect("always can mint in test");
			let r1 = Democracy::inject_referendum(
				3,
				set_balance_proposal_hash_and_note_2(balance1 , asset_id),
				VoteThreshold::SuperMajorityApprove,
				0,
			);
			let r2 = Democracy::inject_referendum(
				2,
				set_balance_proposal_hash_and_note_2(balance2 ,asset_id ),
				VoteThreshold::SuperMajorityApprove,
				0,
			);

			assert_ok!(Democracy::vote(Origin::signed(BOB), r2, aye(BOB)));
			assert_eq!(tally(r2), Tally { ayes: 1, nays: 0, turnout: 10 });

			next_block();
			assert_eq!(Balances::free_balance(42), balance2);

			assert_ok!(Democracy::vote(Origin::signed(BOB), r1, aye(BOB)));
			assert_eq!(tally(r1), Tally { ayes: 1, nays: 0, turnout: 10 });

			next_block();
			assert_eq!(Balances::free_balance(42), balance1);
		});
	}

	#[test]
	fn delayed_enactment_should_work(
		asset_id in valid_asset_id(),
		(balance1, balance2) in valid_amounts_without_overflow_2()) {
		new_test_ext().execute_with(|| {
			Tokens::mint_into(asset_id, &1, balance1 / 10).expect("always can mint in test");
			Tokens::mint_into(asset_id, &2, balance1 / 10).expect("always can mint in test");
			Tokens::mint_into(asset_id, &3, balance1 / 10).expect("always can mint in test");
			Tokens::mint_into(asset_id, &4, balance1 / 10).expect("always can mint in test");
			Tokens::mint_into(asset_id, &5, balance1 / 10).expect("always can mint in test");
			Tokens::mint_into(asset_id, &6, balance1 / 10).expect("always can mint in test");
			let r = Democracy::inject_referendum(
				2,
				set_balance_proposal_hash_and_note_2(balance1, asset_id),
				VoteThreshold::SuperMajorityApprove,
				1,
			);
			assert_ok!(Democracy::vote(Origin::signed(1), r, aye(1)));
			assert_ok!(Democracy::vote(Origin::signed(2), r, aye(2)));
			assert_ok!(Democracy::vote(Origin::signed(3), r, aye(3)));
			assert_ok!(Democracy::vote(Origin::signed(4), r, aye(4)));
			assert_ok!(Democracy::vote(Origin::signed(5), r, aye(5)));
			assert_ok!(Democracy::vote(Origin::signed(6), r, aye(6)));
			assert_eq!(tally(r), Tally { ayes: 21, nays: 0, turnout: 210 });

			next_block();
			assert_eq!(Balances::free_balance(42), 0);

			next_block();
			assert_eq!(Balances::free_balance(42), balance1);
		});
	}

	#[test]
	fn lowest_unbaked_should_be_sensible(
		asset_id in valid_asset_id(),
		(balance1, balance2, balance3) in valid_amounts_without_overflow_3()) {
		new_test_ext().execute_with(|| {
			Tokens::mint_into(asset_id, &BOB, balance1).expect("always can mint in test");
			let r1 = Democracy::inject_referendum(
				3,
				set_balance_proposal_hash_and_note_2(balance1, asset_id),
				VoteThreshold::SuperMajorityApprove,
				0,
			);
			let r2 = Democracy::inject_referendum(
				2,
				set_balance_proposal_hash_and_note_2(balance2, asset_id),
				VoteThreshold::SuperMajorityApprove,
				0,
			);
			let r3 = Democracy::inject_referendum(
				10,
				set_balance_proposal_hash_and_note_2(balance3, asset_id),
				VoteThreshold::SuperMajorityApprove,
				0,
			);
			assert_ok!(Democracy::vote(Origin::signed(BOB), r1, aye(BOB)));
			assert_ok!(Democracy::vote(Origin::signed(BOB), r2, aye(BOB)));
			// r3 is canceled
			assert_ok!(Democracy::cancel_referendum(Origin::root(), r3.into()));
			assert_eq!(Democracy::lowest_unbaked(), 0);

			next_block();

			// r2 is approved
			assert_eq!(Balances::free_balance(42), balance2);
			assert_eq!(Democracy::lowest_unbaked(), 0);

			next_block();

			// r1 is approved
			assert_eq!(Balances::free_balance(42), balance1);
			assert_eq!(Democracy::lowest_unbaked(), 3);
			assert_eq!(Democracy::lowest_unbaked(), Democracy::referendum_count());
		});
	}
}
