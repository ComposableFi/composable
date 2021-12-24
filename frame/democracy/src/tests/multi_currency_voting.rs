// This is part of democracy pallet multi currency support.
use super::*;
use std::convert::TryFrom;

fn aye(x: u8, balance: u64) -> AccountVote<u64> {
	AccountVote::Standard {
		vote: Vote { aye: true, conviction: Conviction::try_from(x).unwrap() },
		balance,
	}
}

fn nay(x: u8, balance: u64) -> AccountVote<u64> {
	AccountVote::Standard {
		vote: Vote { aye: false, conviction: Conviction::try_from(x).unwrap() },
		balance,
	}
}

#[test]
fn voting_with_multi_currency_should_work() {
	new_test_ext().execute_with(|| {
		crate::tests::GovernanceRegistry::grant_root(Origin::root(), DOT_ASSET).unwrap();
		System::set_block_number(0);
		let r = Democracy::inject_referendum(
			2,
			set_balance_proposal_hash_and_note_and_asset(2, DOT_ASSET),
			VoteThreshold::SuperMajorityApprove,
			0,
		);

		assert_ok!(Democracy::vote(Origin::signed(1), r, nay(5, 10)));
		assert_ok!(Democracy::vote(Origin::signed(2), r, aye(4, 20)));
		assert_ok!(Democracy::vote(Origin::signed(3), r, aye(3, 30)));
		assert_ok!(Democracy::vote(Origin::signed(4), r, aye(2, 40)));
		assert_ok!(Democracy::vote(Origin::signed(5), r, nay(1, 50)));
		assert_eq!(tally(r), Tally { ayes: 250, nays: 100, turnout: 150 });

		fast_forward_to(2);

		assert_noop!(
			Democracy::remove_vote(Origin::signed(1), X_ASSET, r),
			Error::<Test>::NotVoter
		);

		assert_ok!(Democracy::remove_vote(Origin::signed(1), DOT_ASSET, r));
		assert_ok!(Democracy::unlock(Origin::signed(1), 1, DOT_ASSET));
	});
}

#[test]
fn voting_with_multi_currency_should_fail() {
	new_test_ext().execute_with(|| {
		System::set_block_number(0);
		let r = Democracy::inject_referendum(
			2,
			set_balance_proposal_hash_and_note_and_asset(2, Y_ASSET),
			VoteThreshold::SuperMajorityApprove,
			0,
		);

		assert_noop!(
			Democracy::vote(Origin::signed(1), r, nay(5, 10)),
			Error::<Test>::InsufficientFunds
		);
	})
}
