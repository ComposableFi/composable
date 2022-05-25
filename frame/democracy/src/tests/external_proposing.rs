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

//! The tests for functionality concerning the "external" origin.

use super::*;
use frame_support::traits::{fungible::Mutate as FungibleMutet, fungibles::Mutate};

proptest! {
	#![proptest_config(ProptestConfig::with_cases(1000))]

	#[test]
	fn veto_external_works(
		asset_id in valid_asset_id(),
		balance1 in valid_amounts_without_overflow_1()) {
		new_test_ext().execute_with(|| {
			System::set_block_number(0);
			let id = set_balance_proposal_hash_and_note_2(balance1, asset_id);
			assert_ok!(Democracy::external_propose(Origin::signed(2), id.hash, id.asset_id,));
			assert!(<NextExternal<Test>>::exists());

			let id = set_balance_proposal_hash_and_note_2(balance1, asset_id);
			assert_ok!(Democracy::veto_external(Origin::signed(3), id.hash, id.asset_id));
			// cancelled.
			assert!(!<NextExternal<Test>>::exists());
			// fails - same proposal can't be resubmitted.
			assert_noop!(
				Democracy::external_propose(
					Origin::signed(2),
					set_balance_proposal_hash(balance1),
					asset_id
				),
				Error::<Test>::ProposalBlacklisted
			);

			fast_forward_to(1);
			// fails as we're still in cooloff period.
			assert_noop!(
				Democracy::external_propose(
					Origin::signed(2),
					set_balance_proposal_hash(balance1),
					asset_id
				),
				Error::<Test>::ProposalBlacklisted
			);

			fast_forward_to(2);
			// works; as we're out of the cooloff period.
			let id = set_balance_proposal_hash_and_note_2(balance1, asset_id);
			assert_ok!(Democracy::external_propose(Origin::signed(2), id.hash, id.asset_id,));
			assert!(<NextExternal<Test>>::exists());

			// 3 can't veto the same thing twice.
			assert_noop!(
				Democracy::veto_external(Origin::signed(3), id.hash, id.asset_id),
				Error::<Test>::AlreadyVetoed
			);

			// 4 vetoes.
			assert_ok!(Democracy::veto_external(Origin::signed(4), id.hash, id.asset_id));
			// cancelled again.
			assert!(!<NextExternal<Test>>::exists());

			fast_forward_to(3);
			// same proposal fails as we're still in cooloff
			assert_noop!(
				Democracy::external_propose(
					Origin::signed(2),
					set_balance_proposal_hash(balance1),
					asset_id
				),
				Error::<Test>::ProposalBlacklisted
			);
			// different proposal works fine.
			let id = set_balance_proposal_hash_and_note_2(balance1 / 2, asset_id);
			assert_ok!(Democracy::external_propose(Origin::signed(2), id.hash, id.asset_id,));
		});
	}

	#[test]
	fn external_blacklisting_should_work(
		asset_id in valid_asset_id(),
		balance1 in valid_amounts_without_overflow_1()) {
		new_test_ext().execute_with(|| {
			System::set_block_number(0);
			Balances::mint_into(&2, balance1 / 100).expect("always can mint in test");
			Tokens::mint_into(asset_id, &2, balance1 / 2).expect("always can mint in test");

			let id = set_balance_proposal_hash_and_note_2(balance1, asset_id);
			assert_ok!(Democracy::external_propose(Origin::signed(2), id.hash, id.asset_id));
			 let hash = set_balance_proposal_hash(balance1);
			assert_ok!(Democracy::blacklist(Origin::root(), hash, asset_id, None));
			fast_forward_to(2);
			assert_noop!(Democracy::referendum_status(0), Error::<Test>::ReferendumInvalid);
			assert_noop!(
				Democracy::external_propose(Origin::signed(2), id.hash, id.asset_id,),
				Error::<Test>::ProposalBlacklisted,
			);
		});
	}


	#[test]
	fn external_default_referendum_works(
		asset_id in valid_asset_id(),
		balance1 in valid_amounts_without_overflow_1()) {
		new_test_ext().execute_with(|| {
			System::set_block_number(0);
			Balances::mint_into(&3, balance1 / 100).expect("always can mint in test");
			Tokens::mint_into(asset_id, &3, balance1 / 2).expect("always can mint in test");
			assert_noop!(
				Democracy::external_propose_default(
					Origin::signed(3),
					set_balance_proposal_hash(balance1),
					asset_id
				),
				BadOrigin,
			);

			let id = set_balance_proposal_hash_and_note_2(balance1, asset_id);
			assert_ok!(Democracy::external_propose_default(Origin::signed(1), id.hash, id.asset_id));
			fast_forward_to(2);
			assert_eq!(
				Democracy::referendum_status(0),
				Ok(ReferendumStatus {
					end: 4,
					proposal_id: ProposalId {
						hash: set_balance_proposal_hash(balance1),
						asset_id: asset_id
					},
					threshold: VoteThreshold::SuperMajorityAgainst,
					delay: 2,
					tally: Tally { ayes: 0, nays: 0, turnout: 0 },
				})
			);
		});
	}
	#[test]
	fn external_majority_referendum_works(
		asset_id in valid_asset_id(),
		balance1 in valid_amounts_without_overflow_1()) {
		new_test_ext().execute_with(|| {
			System::set_block_number(0);
			Balances::mint_into(&3, balance1 / 100).expect("always can mint in test");
			Tokens::mint_into(asset_id, &3, balance1 / 2).expect("always can mint in test");
			assert_noop!(
				Democracy::external_propose_majority(
					Origin::signed(1),
					set_balance_proposal_hash(2),
					asset_id
				),
				BadOrigin,
			);

			let id = set_balance_proposal_hash_and_note_2(balance1, asset_id);
			assert_ok!(Democracy::external_propose_majority(Origin::signed(3), id.hash, id.asset_id));
			fast_forward_to(2);
			assert_eq!(
				Democracy::referendum_status(0),
				Ok(ReferendumStatus {
					end: 4,
					proposal_id: ProposalId {
						hash: set_balance_proposal_hash(balance1),
						asset_id: asset_id
					},
					threshold: VoteThreshold::SimpleMajority,
					delay: 2,
					tally: Tally { ayes: 0, nays: 0, turnout: 0 },
				})
			);
		});
	}

	#[test]
	fn external_referendum_works(
		asset_id in valid_asset_id(),
		balance1 in valid_amounts_without_overflow_1()) {
		new_test_ext().execute_with(|| {
			System::set_block_number(0);
			Balances::mint_into(&3, balance1 / 100).expect("always can mint in test");
			Tokens::mint_into(asset_id, &3, balance1 / 2).expect("always can mint in test");
			assert_noop!(
				Democracy::external_propose(
					Origin::signed(1),
					set_balance_proposal_hash(balance1),
					asset_id
				),
				BadOrigin,
			);
			let id = set_balance_proposal_hash_and_note_2(balance1, asset_id);
			assert_ok!(Democracy::external_propose(Origin::signed(2), id.hash, id.asset_id));
			assert_noop!(
				Democracy::external_propose(
					Origin::signed(2),
					set_balance_proposal_hash(balance1),
					asset_id
				),
				Error::<Test>::DuplicateProposal
			);
			fast_forward_to(2);
			assert_eq!(
				Democracy::referendum_status(0),
				Ok(ReferendumStatus {
					end: 4,
					proposal_id: ProposalId {
						hash: set_balance_proposal_hash(balance1),
						asset_id: asset_id
					},
					threshold: VoteThreshold::SuperMajorityApprove,
					delay: 2,
					tally: Tally { ayes: 0, nays: 0, turnout: 0 },
				})
			);
		});
	}
}

#[test]
fn external_and_public_interleaving_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(0);
		let id = set_balance_proposal_hash_and_note(1);
		assert_ok!(Democracy::external_propose(Origin::signed(2), id.hash, id.asset_id));
		assert_ok!(propose_set_balance_and_note(6, 2, 2));

		fast_forward_to(2);

		// both waiting: external goes first.
		assert_eq!(
			Democracy::referendum_status(0),
			Ok(ReferendumStatus {
				end: 4,
				proposal_id: set_balance_proposal_hash_and_note(1),
				threshold: VoteThreshold::SuperMajorityApprove,
				delay: 2,
				tally: Tally { ayes: 0, nays: 0, turnout: 0 },
			})
		);
		// replenish external
		let id = set_balance_proposal_hash_and_note(3);
		assert_ok!(Democracy::external_propose(Origin::signed(2), id.hash, id.asset_id,));

		fast_forward_to(4);

		// both waiting: public goes next.
		assert_eq!(
			Democracy::referendum_status(1),
			Ok(ReferendumStatus {
				end: 6,
				proposal_id: set_balance_proposal_hash_and_note(2),
				threshold: VoteThreshold::SuperMajorityApprove,
				delay: 2,
				tally: Tally { ayes: 0, nays: 0, turnout: 0 },
			})
		);
		// don't replenish public

		fast_forward_to(6);

		// it's external "turn" again, though since public is empty that doesn't really matter
		assert_eq!(
			Democracy::referendum_status(2),
			Ok(ReferendumStatus {
				end: 8,
				proposal_id: set_balance_proposal_hash_and_note(3),
				threshold: VoteThreshold::SuperMajorityApprove,
				delay: 2,
				tally: Tally { ayes: 0, nays: 0, turnout: 0 },
			})
		);
		// replenish external
		let id = set_balance_proposal_hash_and_note(5);
		assert_ok!(Democracy::external_propose(Origin::signed(2), id.hash, id.asset_id,));

		fast_forward_to(8);

		// external goes again because there's no public waiting.
		assert_eq!(
			Democracy::referendum_status(3),
			Ok(ReferendumStatus {
				end: 10,
				proposal_id: set_balance_proposal_hash_and_note(5),
				threshold: VoteThreshold::SuperMajorityApprove,
				delay: 2,
				tally: Tally { ayes: 0, nays: 0, turnout: 0 },
			})
		);
		// replenish both
		let id = set_balance_proposal_hash_and_note(7);
		assert_ok!(Democracy::external_propose(Origin::signed(2), id.hash, id.asset_id,));
		assert_ok!(propose_set_balance_and_note(6, 4, 2));

		fast_forward_to(10);

		// public goes now since external went last time.
		assert_eq!(
			Democracy::referendum_status(4),
			Ok(ReferendumStatus {
				end: 12,
				proposal_id: set_balance_proposal_hash_and_note(4),
				threshold: VoteThreshold::SuperMajorityApprove,
				delay: 2,
				tally: Tally { ayes: 0, nays: 0, turnout: 0 },
			})
		);
		// replenish public again
		assert_ok!(propose_set_balance_and_note(6, 6, 2));
		// cancel external
		let id = set_balance_proposal_hash_and_note(7);
		assert_ok!(Democracy::veto_external(Origin::signed(3), id.hash, id.asset_id));

		fast_forward_to(12);

		// public goes again now since there's no external waiting.
		assert_eq!(
			Democracy::referendum_status(5),
			Ok(ReferendumStatus {
				end: 14,
				proposal_id: set_balance_proposal_hash_and_note(6),
				threshold: VoteThreshold::SuperMajorityApprove,
				delay: 2,
				tally: Tally { ayes: 0, nays: 0, turnout: 0 },
			})
		);
	});
}
