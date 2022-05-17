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

//! The tests for cancelation functionality.

use super::*;
use frame_support::traits::{fungible::Mutate as FungibleMutet, fungibles::Mutate};

proptest! {
	#![proptest_config(ProptestConfig::with_cases(1000))]

	#[test]
	fn cancel_referendum_should_work(
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
			assert_ok!(Democracy::vote(Origin::signed(BOB), r, aye(BOB)));
			assert_ok!(Democracy::cancel_referendum(Origin::root(), r.into()));
			assert_eq!(Democracy::lowest_unbaked(), 0);

			next_block();

			next_block();

			assert_eq!(Democracy::lowest_unbaked(), 1);
			assert_eq!(Democracy::lowest_unbaked(), Democracy::referendum_count());
			assert_eq!(Balances::free_balance(42), 0);
		});
	}

	#[test]
	fn cancel_queued_should_work(
		asset_id in valid_asset_id(),
		balance1 in valid_amounts_without_overflow_1()) {
		new_test_ext().execute_with(|| {
			System::set_block_number(0);
			Balances::mint_into(&BOB, balance1 / 2).expect("always can mint in test");
			Tokens::mint_into(asset_id, &BOB, balance1).expect("always can mint in test");
			assert_ok!(propose_set_balance_and_note_2(BOB, asset_id,  balance1 / 100, balance1 / 100));

			// start of 2 => next referendum scheduled.
			fast_forward_to(2);

			assert_ok!(Democracy::vote(Origin::signed(BOB), 0, aye(BOB)));

			fast_forward_to(4);

			assert!(pallet_scheduler::Agenda::<Test>::get(6)[0].is_some());

			assert_noop!(Democracy::cancel_queued(Origin::root(), 1), Error::<Test>::ProposalMissing);
			assert_ok!(Democracy::cancel_queued(Origin::root(), 0));
			assert!(pallet_scheduler::Agenda::<Test>::get(6)[0].is_none());
		});
	}

	#[test]
	fn emergency_cancel_should_work(
		asset_id in valid_asset_id(),
		balance1 in valid_amounts_without_overflow_1()) {
		new_test_ext().execute_with(|| {
			System::set_block_number(0);
			let r = Democracy::inject_referendum(
				2,
				set_balance_proposal_hash_and_note_2(balance1, asset_id),
				VoteThreshold::SuperMajorityApprove,
				2,
			);
			assert!(Democracy::referendum_status(r).is_ok());

			assert_noop!(Democracy::emergency_cancel(Origin::signed(3), r), BadOrigin);
			assert_ok!(Democracy::emergency_cancel(Origin::signed(4), r));
			assert!(Democracy::referendum_info(r).is_none());

			// some time later...

			let r = Democracy::inject_referendum(
				2,
				set_balance_proposal_hash_and_note_2(balance1, asset_id),
				VoteThreshold::SuperMajorityApprove,
				2,
			);
			assert!(Democracy::referendum_status(r).is_ok());
			assert_noop!(
				Democracy::emergency_cancel(Origin::signed(4), r),
				Error::<Test>::AlreadyCanceled,
			);
		});
	}

}
