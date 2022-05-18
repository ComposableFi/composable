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

//! The tests for fast-tracking functionality.

use super::*;

use frame_support::traits::{fungible::Mutate as FungibleMutet, fungibles::Mutate};

proptest! {
	#![proptest_config(ProptestConfig::with_cases(1000))]

	#[test]
	fn fast_track_referendum_works(
		asset_id in valid_asset_id(),
		(balance1, balance2) in valid_amounts_without_overflow_2()) {
		new_test_ext().execute_with(|| {
			System::set_block_number(0);
			let id = set_balance_proposal_hash_and_note_2(balance1 / 2, asset_id);
			assert_noop!(
				Democracy::fast_track(Origin::signed(5), id.hash, id.asset_id, balance2 / 3 , balance1 / 2),
				Error::<Test>::ProposalMissing
			);
			let id = set_balance_proposal_hash_and_note_2( balance1 / 2, asset_id);
			assert_ok!(Democracy::external_propose_majority(Origin::signed(JEREMY), id.hash, id.asset_id));
			assert_noop!(
				Democracy::fast_track(Origin::signed(ALICE), id.hash, id.asset_id, balance2 / 3, balance1 / 2),
				BadOrigin
			);
			assert_ok!(Democracy::fast_track(Origin::signed(DARWIN), id.hash, id.asset_id, 2, 0));
			assert_eq!(
				Democracy::referendum_status(0),
				Ok(ReferendumStatus {
					end: 2,
					proposal_id: set_balance_proposal_hash_and_note_2(balance1 / 2, asset_id),
					threshold: VoteThreshold::SimpleMajority,
					delay: 0,
					tally: Tally { ayes: 0, nays: 0, turnout: 0 },
				})
			);
		});
	}

	#[test]
	fn fast_track_referendum_fails_when_no_simple_majority(
		asset_id in valid_asset_id(),
		balance1 in valid_amounts_without_overflow_1()) {
		new_test_ext().execute_with(|| {
			System::set_block_number(0);
			let id = set_balance_proposal_hash_and_note_2(2, asset_id);
			assert_ok!(Democracy::external_propose(Origin::signed(CHARLIE), id.hash, id.asset_id));
			assert_noop!(
				Democracy::fast_track(Origin::signed(5), id.hash, id.asset_id, balance1 , balance1 / 10),
				Error::<Test>::NotSimpleMajority
			);
		});
	}
}

#[test]
fn instant_referendum_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(0);
		let id = set_balance_proposal_hash_and_note(2);
		assert_noop!(
			Democracy::fast_track(Origin::signed(5), id.hash, id.asset_id, 3, 2),
			Error::<Test>::ProposalMissing
		);

		assert_ok!(Democracy::external_propose_majority(Origin::signed(3), id.hash, id.asset_id));

		assert_ok!(Democracy::external_propose_majority(Origin::signed(3), id.hash, id.asset_id));

		assert_noop!(
			Democracy::fast_track(Origin::signed(1), id.hash, id.asset_id, 3, 2),
			BadOrigin
		);
		assert_noop!(
			Democracy::fast_track(Origin::signed(5), id.hash, id.asset_id, 1, 0),
			BadOrigin
		);

		assert_ok!(Democracy::external_propose_majority(Origin::signed(3), id.hash, id.asset_id));
		assert_noop!(
			Democracy::fast_track(Origin::signed(1), id.hash, id.asset_id, 3, 2),
			BadOrigin
		);
		assert_noop!(
			Democracy::fast_track(Origin::signed(5), id.hash, id.asset_id, 1, 0),
			BadOrigin
		);
		assert_noop!(
			Democracy::fast_track(Origin::signed(6), id.hash, id.asset_id, 1, 0),
			Error::<Test>::InstantNotAllowed
		);
		INSTANT_ALLOWED.with(|v| *v.borrow_mut() = true);
		assert_ok!(Democracy::fast_track(Origin::signed(6), id.hash, id.asset_id, 1, 0));
		assert_eq!(
			Democracy::referendum_status(0),
			Ok(ReferendumStatus {
				end: 1,
				proposal_id: set_balance_proposal_hash_and_note(2),
				threshold: VoteThreshold::SimpleMajority,
				delay: 0,
				tally: Tally { ayes: 0, nays: 0, turnout: 0 },
			})
		);
	});
}
