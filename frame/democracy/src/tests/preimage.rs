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

//! The preimage tests.

use super::*;

use frame_support::traits::{fungible::Mutate as FungibleMutet, fungibles::Mutate};

proptest! {
	#![proptest_config(ProptestConfig::with_cases(1000))]

	#[test]
	fn missing_preimage_should_fail(
		asset_id in valid_asset_id(),
		balance1 in valid_amounts_without_overflow_1()) {
		new_test_ext().execute_with(|| {
			Tokens::mint_into(asset_id, &BOB, balance1 / 2).expect("always can mint in test");
			let r = Democracy::inject_referendum(
				2,
				set_balance_proposal_id_2(asset_id, balance1),
				VoteThreshold::SuperMajorityApprove,
				0,
			);
			assert_ok!(Democracy::vote(Origin::signed(BOB), r, aye(BOB)));

			next_block();
			next_block();

			assert_eq!(Balances::free_balance(42), 0);
		});
	}

	#[test]
	fn preimage_deposit_should_be_required_and_returned(
		asset_id in valid_asset_id(),
		balance1 in valid_amounts_without_overflow_1()) {
		new_test_ext_execute_with_cond(|operational| {
			Balances::mint_into(&BOB, balance1 / 100).expect("always can mint in test");
			Tokens::mint_into(asset_id, &BOB, balance1 / 2).expect("always can mint in test");
			// fee of 100 is too much.
			PREIMAGE_BYTE_DEPOSIT.with(|v| *v.borrow_mut() = 100);
			assert_noop!(
				if operational {
					Democracy::note_preimage_operational(Origin::signed(6), vec![0; 500], asset_id)
				} else {
					Democracy::note_preimage(Origin::signed(6), vec![0; 500], asset_id)
				},
				BalancesError::<Test, _>::InsufficientBalance,
			);
			// fee of 1 is reasonable.
			PREIMAGE_BYTE_DEPOSIT.with(|v| *v.borrow_mut() = 1);
			let p = set_balance_proposal(balance1);

			let r = Democracy::inject_referendum(
				2,
				//set_balance_proposal_hash_and_note(2),
				set_balance_proposal_hash_and_note_3(balance1, asset_id, p.clone()),
				VoteThreshold::SuperMajorityApprove,
				0,
			);
			assert_ok!(Democracy::vote(Origin::signed(1), r, aye(1)));

			let deposit = Balance::from(p.len() as u32)
			.saturating_mul(PreimageByteDeposit::get());

			assert_eq!(Balances::reserved_balance(6), deposit);

			next_block();
			next_block();

			assert_eq!(Balances::reserved_balance(6), 0);
			assert_eq!(Balances::free_balance(6), 60);
			assert_eq!(Balances::free_balance(42), balance1);
		});
	}

	#[test]
	fn preimage_deposit_should_be_reapable_earlier_by_owner(
		asset_id in valid_asset_id(),
		balance1 in valid_amounts_without_overflow_1()) {
		new_test_ext_execute_with_cond(|operational| {
			PREIMAGE_BYTE_DEPOSIT.with(|v| *v.borrow_mut() = 1);
			let encoded_proposal = set_balance_proposal(balance1);
			assert_ok!(if operational {
				Democracy::note_preimage_operational(
					Origin::signed(6),
					encoded_proposal.clone(),
					asset_id,
				)
			} else {
				Democracy::note_preimage(Origin::signed(6), set_balance_proposal(balance1), asset_id)
			});

			let deposit = Balance::from(encoded_proposal.len() as u32)
			.saturating_mul(PreimageByteDeposit::get());

			assert_eq!(Balances::reserved_balance(6), deposit);

			next_block();
			assert_noop!(
				Democracy::reap_preimage(
					Origin::signed(6),
					set_balance_proposal_hash(balance1),
					asset_id,
					u32::MAX
				),
				Error::<Test>::TooEarly
			);
			next_block();
			assert_ok!(Democracy::reap_preimage(
				Origin::signed(6),
				set_balance_proposal_hash(balance1),
				asset_id,
				u32::MAX
			));

			assert_eq!(Balances::free_balance(6), 60);
			assert_eq!(Balances::reserved_balance(6), 0);
		});
	}

	#[test]
	fn preimage_deposit_should_be_reapable(
		asset_id in valid_asset_id(),
		balance1 in valid_amounts_without_overflow_1()) {
		new_test_ext_execute_with_cond(|operational| {
			assert_noop!(
				Democracy::reap_preimage(
					Origin::signed(5),
					set_balance_proposal_hash(balance1),
					asset_id,
					u32::MAX
				),
				Error::<Test>::PreimageMissing
			);

			PREIMAGE_BYTE_DEPOSIT.with(|v| *v.borrow_mut() = 1);
			let encoded_proposal = set_balance_proposal(balance1);

			let free_balance6 = Balances::free_balance(6);
			let free_balance5 = Balances::free_balance(5);

			assert_ok!(if operational {
				Democracy::note_preimage_operational(
					Origin::signed(6),
					encoded_proposal.clone(),
					asset_id,
				)
			} else {
				Democracy::note_preimage(Origin::signed(6), encoded_proposal.clone(), asset_id)
			});

			let deposit = Balance::from(encoded_proposal.len() as u32)
			.saturating_mul(PreimageByteDeposit::get());
			assert_eq!(Balances::reserved_balance(6), deposit);

			next_block();
			next_block();
			next_block();

			assert_noop!(
				Democracy::reap_preimage(
					Origin::signed(5),
					set_balance_proposal_hash(balance1),
					asset_id,
					u32::MAX
				),
				Error::<Test>::TooEarly
			);

			next_block();
			assert_ok!(Democracy::reap_preimage(
				Origin::signed(5),
				set_balance_proposal_hash(balance1),
				asset_id,
				u32::MAX
			));
			assert_eq!(Balances::reserved_balance(6), 0);
			assert_eq!(Balances::free_balance(6), free_balance6 - deposit);
			assert_eq!(Balances::free_balance(5), free_balance5 + deposit);
		});
	}

	#[test]
	fn noting_imminent_preimage_for_free_should_work(
		asset_id in valid_asset_id(),
		balance1 in valid_amounts_without_overflow_1()) {
		new_test_ext_execute_with_cond(|operational| {
			Tokens::mint_into(asset_id, &BOB, balance1 / 2).expect("always can mint in test");
			PREIMAGE_BYTE_DEPOSIT.with(|v| *v.borrow_mut() = 1);

			let r = Democracy::inject_referendum(
				2,
				set_balance_proposal_id_2(asset_id, balance1),
				VoteThreshold::SuperMajorityApprove,
				1,
			);
			assert_ok!(Democracy::vote(Origin::signed(1), r, aye(1)));

			assert_noop!(
				if operational {
					Democracy::note_imminent_preimage_operational(
						Origin::signed(6),
						set_balance_proposal(balance1),
						asset_id,
					)
				} else {
					Democracy::note_imminent_preimage(
						Origin::signed(6),
						set_balance_proposal(balance1),
						asset_id,
					)
				},
				Error::<Test>::NotImminent
			);

			next_block();

			// Now we're in the dispatch queue it's all good.
			assert_ok!(Democracy::note_imminent_preimage(
				Origin::signed(6),
				set_balance_proposal(balance1),
				asset_id
			));

			next_block();

			assert_eq!(Balances::free_balance(42), balance1);
		});
	}

	#[test]
	fn note_imminent_preimage_can_only_be_successful_once(
		asset_id in valid_asset_id(),
		balance1 in valid_amounts_without_overflow_1()) {
		new_test_ext().execute_with(|| {
			Tokens::mint_into(asset_id, &BOB, balance1 / 2).expect("always can mint in test");
			PREIMAGE_BYTE_DEPOSIT.with(|v| *v.borrow_mut() = 1);

			let r = Democracy::inject_referendum(
				2,
				set_balance_proposal_id_2(asset_id, balance1),
				VoteThreshold::SuperMajorityApprove,
				1,
			);
			assert_ok!(Democracy::vote(Origin::signed(1), r, aye(1)));
			next_block();

			// First time works
			assert_ok!(Democracy::note_imminent_preimage(
				Origin::signed(6),
				set_balance_proposal(balance1),
				asset_id
			));

			// Second time fails
			assert_noop!(
				Democracy::note_imminent_preimage(
					Origin::signed(6),
					set_balance_proposal(balance1),
					asset_id
				),
				Error::<Test>::DuplicatePreimage
			);

			// Fails from any user
			assert_noop!(
				Democracy::note_imminent_preimage(
					Origin::signed(5),
					set_balance_proposal(balance1),
					asset_id
				),
				Error::<Test>::DuplicatePreimage
			);
		});
	}

	#[test]
	fn reaping_imminent_preimage_should_fail(
		asset_id in valid_asset_id(),
		balance1 in valid_amounts_without_overflow_1()) {
		new_test_ext().execute_with(|| {
			Tokens::mint_into(asset_id, &BOB, balance1 / 2).expect("always can mint in test");
			let encoded_proposal = set_balance_proposal(balance1);
			let h = set_balance_proposal_hash_and_note_3(balance1, asset_id,encoded_proposal );
			let r = Democracy::inject_referendum(3, h, VoteThreshold::SuperMajorityApprove, 1);
			assert_ok!(Democracy::vote(Origin::signed(1), r, aye(1)));
			next_block();
			next_block();
			assert_noop!(
				Democracy::reap_preimage(
					Origin::signed(6),
					set_balance_proposal_hash(balance1),
					asset_id,
					u32::MAX
				),
				Error::<Test>::Imminent
			);
		});
	}
}
