//! Unit tests for the bonded-finance pallet.

#![cfg(test)]

use super::*;
use composable_traits::bonded_finance::BondOffer;
use frame_support::traits::{
	fungibles::{Inspect, Mutate},
	tokens::WithdrawConsequence,
};
use mock::*;
use proptest::prelude::*;
use sp_runtime::helpers_128bit::multiply_by_rational;

macro_rules! prop_assert_epsilon {
	($x:expr, $y:expr) => {{
		let precision = 1000;
		let epsilon = 1;
		let upper = precision + epsilon;
		let lower = precision - epsilon;
		let q = multiply_by_rational($x, precision, $y).expect("qed;");
		prop_assert!(
			upper >= q && q >= lower,
			"({}) => {} >= {} * {} / {} >= {}",
			q,
			upper,
			$x,
			precision,
			$y,
			lower
		);
	}};
}

macro_rules! prop_assert_ok {
    ($cond:expr) => {
        prop_assert_ok!($cond, concat!("assertion failed: ", stringify!($cond)))
    };

    ($cond:expr, $($fmt:tt)*) => {
        {
        if let Err(e) = $cond {
            let message = format!($($fmt)*);
            let message = format!("Expected Ok(_), got {:?}, {} at {}:{}", e, message, file!(), line!());
            return ::std::result::Result::Err(
                proptest::test_runner::TestCaseError::fail(message));
        }
        }
    };
}

// easier to test in this case
fn is_simple_offer(offer: &BondOffer<MockCurrencyId, Balance, BlockNumber>) -> bool {
	let not_same = offer.asset != offer.reward_asset;
	let max_valid_amount = Balance::MAX / 2 - 1;
	offer.valid(MinOffer::get(), MinReward::get()) &&
		offer.reward_amount <= max_valid_amount &&
		offer.amount <= max_valid_amount &&
		not_same
}

prop_compose! {
	  fn simple_offer()
		(offer in any::<BondOffer<MockCurrencyId, Balance, BlockNumber>>()
		 .prop_filter("We are not interested in invalid offers for this test.", is_simple_offer))
		 -> BondOffer<MockCurrencyId, Balance, BlockNumber> {
			  offer
	  }
}

proptest! {
	  #![proptest_config(ProptestConfig::with_cases(10_000))]

	  #[test]
	  fn valid_bond_offer(offer in simple_offer()) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward_asset, &ALICE, offer.reward_amount));
						prop_assert_ok!(BondedFinance::offer(Origin::signed(ALICE), offer));
					  Ok(())
			  })?;
	  }

	  #[test]
	  fn can_bond(offer in simple_offer()) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward_asset, &ALICE, offer.reward_amount));
						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone());
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  prop_assert_ok!(Tokens::mint_into(offer.asset, &BOB, offer.amount));
					  prop_assert_ok!(BondedFinance::bond(Origin::signed(BOB), offer_id, offer.amount));
					  Ok(())
			  })?;
	  }

	  #[test]
	  fn correct_share(offer in simple_offer()) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward_asset, &ALICE, offer.reward_amount));

						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone());
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  let half_amount = offer.amount / 2;
					  let half_reward = offer.reward_amount / 2;

					  prop_assert_ok!(Tokens::mint_into(offer.asset, &BOB, half_amount));
					  let bob_reward = BondedFinance::do_bond(offer_id, &BOB, half_amount);
					  prop_assert_ok!(bob_reward);
					  let bob_reward = bob_reward.expect("impossible; qed;");

					  prop_assert_ok!(Tokens::mint_into(offer.asset, &CHARLIE, half_amount));
					  let charlie_reward = BondedFinance::do_bond(offer_id, &CHARLIE, half_amount);
					  prop_assert_ok!(charlie_reward);
					  let charlie_reward = charlie_reward.expect("impossible; qed;");

						prop_assert_epsilon!(bob_reward, half_reward);
					  prop_assert_epsilon!(charlie_reward, half_reward);

					  prop_assert!(Tokens::can_withdraw(offer.reward_asset, &BOB, bob_reward) == WithdrawConsequence::Frozen);
					  prop_assert!(Tokens::can_withdraw(offer.reward_asset, &CHARLIE, charlie_reward) == WithdrawConsequence::Frozen);

					  System::set_block_number(offer.reward_duration);

					  prop_assert_ok!(Vesting::claim(Origin::signed(BOB), offer.reward_asset));
					  prop_assert_ok!(Vesting::claim(Origin::signed(CHARLIE), offer.reward_asset));

					  prop_assert!(Tokens::can_withdraw(offer.reward_asset, &BOB, bob_reward) == WithdrawConsequence::Success);
					  prop_assert!(Tokens::can_withdraw(offer.reward_asset, &CHARLIE, charlie_reward) == WithdrawConsequence::Success);

					  Ok(())
			  })?;

	  }
}
