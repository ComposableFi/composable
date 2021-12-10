//! Unit tests for the bonded-finance pallet.

#![cfg(test)]

use super::*;
use composable_traits::bonded_finance::{BondDuration, BondOffer, BondOfferReward};
use frame_support::{
	error::BadOrigin,
	traits::{
		fungibles::{Inspect, Mutate},
		tokens::WithdrawConsequence,
	},
};
use mock::{Event, *};
use proptest::prelude::*;
use sp_runtime::helpers_128bit::multiply_by_rational;

macro_rules! prop_assert_epsilon {
	($x:expr, $y:expr) => {{
		let precision = 100;
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
        if let Err(e) = $cond {
            let message = format!($($fmt)*);
            let message = format!("Expected Ok(_), got {:?}, {} at {}:{}", e, message, file!(), line!());
            return ::std::result::Result::Err(
                proptest::test_runner::TestCaseError::fail(message));
        }
    };
}

#[test]
fn valid_offer() {
	assert!(BondOffer {
		asset: MockCurrencyId::BTC,
		bond_price: MIN_VESTED_TRANSFER as _,
		nb_of_bonds: 100_000u128,
		maturity: BondDuration::Infinite,
		reward: BondOfferReward {
			asset: MockCurrencyId::PICA,
			amount: 1_000_000u128 * 100_000u128,
			maturity: 96u128,
		}
	}
	.valid(MinVestedTransfer::get() as _, MinReward::get()));
	assert!(BondOffer {
		asset: MockCurrencyId::BTC,
		bond_price: MIN_VESTED_TRANSFER as _,
		nb_of_bonds: 1u128,
		maturity: BondDuration::Finite { return_in: 1 },
		reward: BondOfferReward {
			asset: MockCurrencyId::BTC,
			amount: 1_000_000u128,
			maturity: 96u128,
		}
	}
	.valid(MinVestedTransfer::get() as _, MinReward::get()));
	assert!(BondOffer {
		asset: MockCurrencyId::BTC,
		bond_price: 1_000_000 + MIN_VESTED_TRANSFER as u128,
		nb_of_bonds: 100_000u128,
		maturity: BondDuration::Finite { return_in: 1_000_000 },
		reward: BondOfferReward {
			asset: MockCurrencyId::BTC,
			amount: 1_000_000u128 * 100_000u128,
			maturity: 96u128,
		}
	}
	.valid(MinVestedTransfer::get() as _, MinReward::get()));
}

#[test]
fn invalid_offer() {
	// invalid bond_price
	assert!(!BondOffer {
		asset: MockCurrencyId::BTC,
		bond_price: MIN_VESTED_TRANSFER as u128 - 1,
		nb_of_bonds: 100_000u128,
		maturity: BondDuration::Infinite,
		reward: BondOfferReward {
			asset: MockCurrencyId::PICA,
			amount: 1_000_000u128,
			maturity: 96u128
		}
	}
	.valid(MinVestedTransfer::get() as _, MinReward::get()));

	// invalid nb_of_bonds
	assert!(!BondOffer {
		asset: MockCurrencyId::BTC,
		bond_price: MIN_VESTED_TRANSFER as _,
		nb_of_bonds: 0,
		maturity: BondDuration::Finite { return_in: 1 },
		reward: BondOfferReward {
			asset: MockCurrencyId::BTC,
			amount: 1_000_000u128,
			maturity: 96u128,
		}
	}
	.valid(MinVestedTransfer::get() as _, MinReward::get()));

	// invalid maturity
	assert!(!BondOffer {
		asset: MockCurrencyId::BTC,
		bond_price: 1_000_000 + MIN_VESTED_TRANSFER as u128,
		nb_of_bonds: 100_000u128,
		maturity: BondDuration::Finite { return_in: 0 },
		reward: BondOfferReward {
			asset: MockCurrencyId::BTC,
			amount: 1_000_000u128,
			maturity: 96u128,
		}
	}
	.valid(MinVestedTransfer::get() as _, MinReward::get()));

	// invalid reward
	assert!(!BondOffer {
		asset: MockCurrencyId::BTC,
		bond_price: 1_000_000 + MIN_VESTED_TRANSFER as u128,
		nb_of_bonds: 100_000u128,
		maturity: BondDuration::Finite { return_in: 1_000_000 },
		reward: BondOfferReward { asset: MockCurrencyId::BTC, amount: 0, maturity: 96u128 }
	}
	.valid(MinVestedTransfer::get() as _, MinReward::get()));

	// invalid reward: < MinVested
	assert!(!BondOffer {
		asset: MockCurrencyId::BTC,
		bond_price: 1_000_000 + MIN_VESTED_TRANSFER as u128,
		nb_of_bonds: 100_000u128,
		maturity: BondDuration::Finite { return_in: 1_000_000 },
		reward: BondOfferReward {
			asset: MockCurrencyId::BTC,
			amount: 1_000_000u128 * 100_000u128 - 1,
			maturity: 96u128
		}
	}
	.valid(MinVestedTransfer::get() as _, MinReward::get()));

	// invalid reward maturity
	assert!(!BondOffer {
		asset: MockCurrencyId::BTC,
		bond_price: 1_000_000 + MIN_VESTED_TRANSFER as u128,
		nb_of_bonds: 100_000u128,
		maturity: BondDuration::Finite { return_in: 1_000_000 },
		reward: BondOfferReward {
			asset: MockCurrencyId::BTC,
			amount: 1_000_000u128,
			maturity: 0u128
		}
	}
	.valid(MinVestedTransfer::get() as _, MinReward::get()));
}

prop_compose! {
	  // NOTE(hussein-aitlahcen): we use u32 before casting to avoid overflows
	  /// Pseudo random valid simple offer
	  fn simple_offer(min_contracts: Balance)
			  (
					  bond_price in MIN_VESTED_TRANSFER as u128..u32::MAX as Balance,
					  nb_of_bonds in min_contracts..u32::MAX as Balance,
					  maturity in prop_oneof![
							  Just(BondDuration::Infinite),
							  (1..BlockNumber::MAX / 2).prop_map(|return_in| BondDuration::Finite { return_in })
					  ],
					  // avoid overflowing when advancing blocks and mint_into for a couple of offers
					  reward_amount in MIN_REWARD..Balance::MAX / 2,
					  reward_maturity in 1..BlockNumber::MAX / 2
			  )
			  -> BondOffer<MockCurrencyId, Balance, BlockNumber> {
					  BondOffer {
							  asset: MockCurrencyId::BTC,
								bond_price,
								nb_of_bonds,
								maturity,
							  reward: BondOfferReward {
									  asset: MockCurrencyId::ETH,
									  amount: Balance::max(MIN_REWARD.saturating_mul(nb_of_bonds), reward_amount),
									  maturity: reward_maturity,
							  }
					  }
			  }
}

proptest! {
	  #![proptest_config(ProptestConfig::with_cases(10_000))]

	  #[test]
	  fn can_create_valid_offer(offer in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
					  System::set_block_number(1);
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));
						let offer_id = BondedFinance::do_offer(&ALICE, offer);
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  System::assert_last_event(Event::BondedFinance(crate::Event::NewOffer{ offer_id }));
					  Ok(())
			  })?;
	  }

	  #[test]
	  fn stake_taken(offer in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));

						prop_assert_eq!(Tokens::balance(NATIVE_CURRENCY_ID, &ALICE), Stake::get());
						prop_assert_ok!(BondedFinance::do_offer(&ALICE, offer));
						prop_assert_eq!(Tokens::balance(NATIVE_CURRENCY_ID, &ALICE), 0);
					  Ok(())
			  })?;
	  }

	  #[test]
	  fn reward_taken(offer in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));

						prop_assert_eq!(Tokens::balance(offer.reward.asset, &ALICE), offer.reward.amount);
						prop_assert_ok!(BondedFinance::do_offer(&ALICE, offer.clone()));
						prop_assert_eq!(Tokens::balance(offer.reward.asset, &ALICE), 0);
					  Ok(())
			  })?;
	  }

	  #[test]
	  fn isolated_accounts(offer_a in simple_offer(1), offer_b in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer_a.reward.asset, &ALICE, offer_a.reward.amount));
						let offer_a_id = BondedFinance::do_offer(&ALICE, offer_a.clone());
					  prop_assert_ok!(offer_a_id);
					  let offer_a_id = offer_a_id.expect("impossible; qed");

						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &BOB, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer_b.reward.asset, &BOB, offer_b.reward.amount));
						let offer_b_id = BondedFinance::do_offer(&BOB, offer_b.clone());
					  prop_assert_ok!(offer_b_id);
					  let offer_b_id = offer_b_id.expect("impossible; qed");

					  prop_assert_ne!(BondedFinance::account_id(offer_a_id), BondedFinance::account_id(offer_b_id));
					  prop_assert_eq!(
							  Tokens::balance(offer_a.reward.asset, &BondedFinance::account_id(offer_a_id)),
							  offer_a.reward.amount
					  );
					  prop_assert_eq!(
							  Tokens::balance(offer_b.reward.asset, &BondedFinance::account_id(offer_b_id)),
							  offer_b.reward.amount
					  );
					  Ok(())
			  })?;
	  }

	  // A user bond for the full offer
	  #[test]
	  fn single_bond(offer in simple_offer(2)) {
			  ExtBuilder::build().execute_with(|| {
					  System::set_block_number(1);

						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));
						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone());
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  prop_assert_ok!(Tokens::mint_into(offer.asset, &BOB, offer.total_price().expect("impossible; qed;")));
					  prop_assert_ok!(BondedFinance::bond(Origin::signed(BOB), offer_id, offer.nb_of_bonds - 1));

					  System::assert_last_event(Event::BondedFinance(crate::Event::NewBond {
							  offer_id,
							  who: BOB,
							  nb_of_bonds: offer.nb_of_bonds - 1
					  }));

					  prop_assert_ok!(BondedFinance::bond(Origin::signed(BOB), offer_id, 1));

					  System::assert_has_event(Event::BondedFinance(crate::Event::NewBond {
							  offer_id,
							  who: BOB,
							  nb_of_bonds: 1
					  }));

					  System::assert_last_event(Event::BondedFinance(crate::Event::OfferCompleted { offer_id }));

					  Ok(())
			  })?;
	  }

	  #[test]
	  fn multiple_bonds(offer in simple_offer(2)) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));

						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone());
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  let half_contracts = offer.nb_of_bonds / 2;
					  let half_reward = offer.reward.amount / 2;

					  prop_assert_ok!(Tokens::mint_into(offer.asset, &BOB, half_contracts * offer.bond_price));
					  let bob_reward = BondedFinance::do_bond(offer_id, &BOB, half_contracts);
					  prop_assert_ok!(bob_reward);
					  let bob_reward = bob_reward.expect("impossible; qed;");

					  prop_assert_ok!(Tokens::mint_into(offer.asset, &CHARLIE, half_contracts * offer.bond_price));
					  let charlie_reward = BondedFinance::do_bond(offer_id, &CHARLIE, half_contracts);
					  prop_assert_ok!(charlie_reward);
					  let charlie_reward = charlie_reward.expect("impossible; qed;");

						prop_assert_epsilon!(bob_reward, half_reward);
					  prop_assert_epsilon!(charlie_reward, half_reward);

					  prop_assert!(Tokens::can_withdraw(offer.reward.asset, &BOB, bob_reward) == WithdrawConsequence::Frozen);
					  prop_assert!(Tokens::can_withdraw(offer.reward.asset, &CHARLIE, charlie_reward) == WithdrawConsequence::Frozen);

					  System::set_block_number(offer.reward.maturity);

					  prop_assert_ok!(Vesting::claim(Origin::signed(BOB), offer.reward.asset));
					  prop_assert_ok!(Vesting::claim(Origin::signed(CHARLIE), offer.reward.asset));

					  prop_assert!(Tokens::can_withdraw(offer.reward.asset, &BOB, bob_reward) == WithdrawConsequence::Success);
					  prop_assert!(Tokens::can_withdraw(offer.reward.asset, &CHARLIE, charlie_reward) == WithdrawConsequence::Success);

					  Ok(())
			  })?;
	  }

	  #[test]
	  fn non_existing_offer(offer in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));
						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone());
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  prop_assert_ok!(Tokens::mint_into(offer.asset, &BOB, offer.total_price().expect("impossible; qed;")));
					  prop_assert_eq!(
							  BondedFinance::bond(Origin::signed(BOB), offer_id + 1, offer.nb_of_bonds),
							  Err(Error::<Runtime>::BondOfferNotFound.into())
					  );

					  Ok(())
			  })?;
	  }

	  #[test]
	  fn invalid_nb_of_bonds(offer in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));
						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone());
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  prop_assert_ok!(Tokens::mint_into(offer.asset, &BOB, offer.total_price().expect("impossible; qed;")));
					  prop_assert_eq!(
							  BondedFinance::bond(Origin::signed(BOB), offer_id, offer.nb_of_bonds + 1),
							  Err(Error::<Runtime>::InvalidNumberOfBonds.into())
					  );
					  prop_assert_eq!(
							  BondedFinance::bond(Origin::signed(BOB), offer_id, 0),
							  Err(Error::<Runtime>::InvalidNumberOfBonds.into())
					  );

					  Ok(())
			  })?;
	  }

	  #[test]
	  fn offer_completed(offer in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));
						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone());
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  prop_assert_ok!(Tokens::mint_into(offer.asset, &BOB, offer.total_price().expect("impossible; qed;")));
					  prop_assert_ok!(BondedFinance::bond(Origin::signed(BOB), offer_id, offer.nb_of_bonds));
					  prop_assert_eq!(
							  BondedFinance::bond(Origin::signed(BOB), offer_id, offer.nb_of_bonds),
							  Err(Error::<Runtime>::OfferCompleted.into())
					  );

						prop_assert_eq!(Tokens::balance(NATIVE_CURRENCY_ID, &ALICE), Stake::get());

					  Ok(())
			  })?;
	  }

	  #[test]
	  fn issuer_cancel_offer(offer in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
					  System::set_block_number(1);

						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));
						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone());
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  prop_assert_eq!(
							  BondedFinance::cancel(Origin::signed(BOB), offer_id),
							  Err(BadOrigin.into())
					  );

					  prop_assert_ok!(BondedFinance::cancel(Origin::signed(ALICE), offer_id));
						prop_assert_eq!(Tokens::balance(NATIVE_CURRENCY_ID, &ALICE), Stake::get());

					  prop_assert_eq!(
							  BondedFinance::bond(Origin::signed(BOB), offer_id, offer.nb_of_bonds),
							  Err(Error::<Runtime>::BondOfferNotFound.into())
					  );

					  System::assert_last_event(Event::BondedFinance(crate::Event::OfferCancelled { offer_id }));

					  Ok(())
			  })?;
	  }

	  #[test]
	  fn admin_cancel_offer(offer in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
					  System::set_block_number(1);

						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));
						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone());
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  prop_assert_eq!(
							  BondedFinance::cancel(Origin::signed(BOB), offer_id),
							  Err(BadOrigin.into())
					  );

					  prop_assert_ok!(BondedFinance::cancel(Origin::root(), offer_id));
						prop_assert_eq!(Tokens::balance(NATIVE_CURRENCY_ID, &ALICE), Stake::get());

					  prop_assert_eq!(
							  BondedFinance::bond(Origin::signed(BOB), offer_id, offer.nb_of_bonds),
							  Err(Error::<Runtime>::BondOfferNotFound.into())
					  );

					  System::assert_last_event(Event::BondedFinance(crate::Event::OfferCancelled { offer_id }));

					  Ok(())
			  })?;
	  }
}
