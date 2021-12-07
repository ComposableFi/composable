//! Unit tests for the bonded-finance pallet.

#![cfg(test)]

use super::*;
use composable_traits::bonded_finance::{BondDuration, BondOffer};
use frame_support::traits::{
	fungibles::{Inspect, Mutate},
	tokens::WithdrawConsequence,
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
		price: MIN_VESTED_TRANSFER as _,
		contracts: 100_000u128,
		duration: BondDuration::Infinite,
		reward_asset: MockCurrencyId::PICA,
		reward_amount: 1_000_000u128 * 100_000u128,
		reward_duration: 96u128,
	}
	.valid(MinVestedTransfer::get() as _, MinReward::get()));
	assert!(BondOffer {
		asset: MockCurrencyId::BTC,
		price: MIN_VESTED_TRANSFER as _,
		contracts: 1u128,
		duration: BondDuration::Finite { blocks: 1 },
		reward_asset: MockCurrencyId::BTC,
		reward_amount: 1_000_000u128,
		reward_duration: 96u128,
	}
	.valid(MinVestedTransfer::get() as _, MinReward::get()));
	assert!(BondOffer {
		asset: MockCurrencyId::BTC,
		price: 1_000_000 + MIN_VESTED_TRANSFER as u128,
		contracts: 100_000u128,
		duration: BondDuration::Finite { blocks: 1_000_000 },
		reward_asset: MockCurrencyId::BTC,
		reward_amount: 1_000_000u128 * 100_000u128,
		reward_duration: 96u128,
	}
	.valid(MinVestedTransfer::get() as _, MinReward::get()));
}

#[test]
fn invalid_offer() {
	// invalid price
	assert!(!BondOffer {
		asset: MockCurrencyId::BTC,
		price: MIN_VESTED_TRANSFER as u128 - 1,
		contracts: 100_000u128,
		duration: BondDuration::Infinite,
		reward_asset: MockCurrencyId::PICA,
		reward_amount: 1_000_000u128,
		reward_duration: 96u128,
	}
	.valid(MinVestedTransfer::get() as _, MinReward::get()));

	// invalid contracts
	assert!(!BondOffer {
		asset: MockCurrencyId::BTC,
		price: MIN_VESTED_TRANSFER as _,
		contracts: 0,
		duration: BondDuration::Finite { blocks: 1 },
		reward_asset: MockCurrencyId::BTC,
		reward_amount: 1_000_000u128,
		reward_duration: 96u128,
	}
	.valid(MinVestedTransfer::get() as _, MinReward::get()));

	// invalid duration
	assert!(!BondOffer {
		asset: MockCurrencyId::BTC,
		price: 1_000_000 + MIN_VESTED_TRANSFER as u128,
		contracts: 100_000u128,
		duration: BondDuration::Finite { blocks: 0 },
		reward_asset: MockCurrencyId::BTC,
		reward_amount: 1_000_000u128,
		reward_duration: 96u128,
	}
	.valid(MinVestedTransfer::get() as _, MinReward::get()));

	// invalid reward
	assert!(!BondOffer {
		asset: MockCurrencyId::BTC,
		price: 1_000_000 + MIN_VESTED_TRANSFER as u128,
		contracts: 100_000u128,
		duration: BondDuration::Finite { blocks: 1_000_000 },
		reward_asset: MockCurrencyId::BTC,
		reward_amount: 0,
		reward_duration: 96u128,
	}
	.valid(MinVestedTransfer::get() as _, MinReward::get()));

	// invalid reward: < MinVested
	assert!(!BondOffer {
		asset: MockCurrencyId::BTC,
		price: 1_000_000 + MIN_VESTED_TRANSFER as u128,
		contracts: 100_000u128,
		duration: BondDuration::Finite { blocks: 1_000_000 },
		reward_asset: MockCurrencyId::BTC,
		reward_amount: 1_000_000u128 * 100_000u128 - 1,
		reward_duration: 96u128,
	}
	.valid(MinVestedTransfer::get() as _, MinReward::get()));

	// invalid reward duration
	assert!(!BondOffer {
		asset: MockCurrencyId::BTC,
		price: 1_000_000 + MIN_VESTED_TRANSFER as u128,
		contracts: 100_000u128,
		duration: BondDuration::Finite { blocks: 1_000_000 },
		reward_asset: MockCurrencyId::BTC,
		reward_amount: 1_000_000u128,
		reward_duration: 0u128,
	}
	.valid(MinVestedTransfer::get() as _, MinReward::get()));
}

prop_compose! {
	  // NOTE(hussein-aitlahcen): we use u32 before casting to avoid overflows
	  /// Pseudo random valid simple offer
	  fn simple_offer(min_contracts: Balance)
			  (
					  price in MIN_VESTED_TRANSFER as u128..u32::MAX as Balance,
					  contracts in min_contracts..u32::MAX as Balance,
					  duration in prop_oneof![
							  Just(BondDuration::Infinite),
							  (1..BlockNumber::MAX / 2).prop_map(|blocks| BondDuration::Finite { blocks })
					  ],
					  // avoid overflowing when advancing blocks and mint_into for a couple of offers
					  reward_amount in MIN_REWARD..Balance::MAX / 2,
					  reward_duration in 1..BlockNumber::MAX / 2
			)
			  -> BondOffer<MockCurrencyId, Balance, BlockNumber> {
					  BondOffer {
							  asset: MockCurrencyId::BTC,
								price,
								contracts,
								duration,
								reward_asset: MockCurrencyId::ETH,
							  // min_reward is per_contract
								reward_amount: Balance::max(MIN_REWARD.saturating_mul(contracts), reward_amount),
								reward_duration,
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
						prop_assert_ok!(Tokens::mint_into(offer.reward_asset, &ALICE, offer.reward_amount));
						let offer_id = BondedFinance::do_offer(&ALICE, offer);
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  System::assert_last_event(Event::BondedFinance(crate::Event::NewOffer{ offer: offer_id }));
					  Ok(())
			  })?;
	  }

	  #[test]
	  fn isolated_accounts(offer_a in simple_offer(1), offer_b in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer_a.reward_asset, &ALICE, offer_a.reward_amount));
						let offer_a_id = BondedFinance::do_offer(&ALICE, offer_a);
					  prop_assert_ok!(offer_a_id);
					  let offer_a_id = offer_a_id.expect("impossible; qed");

						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &BOB, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer_b.reward_asset, &BOB, offer_b.reward_amount));
						let offer_b_id = BondedFinance::do_offer(&BOB, offer_b);
					  prop_assert_ok!(offer_b_id);
					  let offer_b_id = offer_b_id.expect("impossible; qed");

					  prop_assert_ne!(BondedFinance::account_id(offer_a_id), BondedFinance::account_id(offer_b_id));
					  Ok(())
			  })?;
	  }

	  // A user bond for the full offer
	  #[test]
	  fn single_bond(offer in simple_offer(2)) {
			  ExtBuilder::build().execute_with(|| {
					  System::set_block_number(1);

						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward_asset, &ALICE, offer.reward_amount));
						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone());
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  prop_assert_ok!(Tokens::mint_into(offer.asset, &BOB, offer.total_price().expect("impossible; qed;")));
					  prop_assert_ok!(BondedFinance::bond(Origin::signed(BOB), offer_id, offer.contracts - 1));

					  System::assert_last_event(Event::BondedFinance(crate::Event::NewBond {
							  offer: offer_id,
							  who: BOB,
							  contracts: offer.contracts - 1
					  }));

					  prop_assert_ok!(BondedFinance::bond(Origin::signed(BOB), offer_id, 1));

					  System::assert_has_event(Event::BondedFinance(crate::Event::NewBond {
							  offer: offer_id,
							  who: BOB,
							  contracts: 1
					  }));

					  System::assert_last_event(Event::BondedFinance(crate::Event::OfferCompleted { offer: offer_id }));

					  Ok(())
			  })?;
	  }

	  #[test]
	  fn multiple_bonds(offer in simple_offer(2)) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward_asset, &ALICE, offer.reward_amount));

						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone());
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  let half_contracts = offer.contracts / 2;
					  let half_reward = offer.reward_amount / 2;

					  prop_assert_ok!(Tokens::mint_into(offer.asset, &BOB, half_contracts * offer.price));
					  let bob_reward = BondedFinance::do_bond(offer_id, &BOB, half_contracts);
					  prop_assert_ok!(bob_reward);
					  let bob_reward = bob_reward.expect("impossible; qed;");

					  prop_assert_ok!(Tokens::mint_into(offer.asset, &CHARLIE, half_contracts * offer.price));
					  let charlie_reward = BondedFinance::do_bond(offer_id, &CHARLIE, half_contracts);
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

	  #[test]
	  fn invalid_parts(offer in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward_asset, &ALICE, offer.reward_amount));
						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone());
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  prop_assert_ok!(Tokens::mint_into(offer.asset, &BOB, offer.total_price().expect("impossible; qed;")));
					  prop_assert_eq!(
							  BondedFinance::bond(Origin::signed(BOB), offer_id, offer.contracts + 1),
							  Err(Error::<Runtime>::InvalidNumberOfContracts.into())
					  );
					  prop_assert_eq!(
							  BondedFinance::bond(Origin::signed(BOB), offer_id, 0),
							  Err(Error::<Runtime>::InvalidNumberOfContracts.into())
					  );

					  Ok(())
			  })?;
	  }

	  #[test]
	  fn offer_completed(offer in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward_asset, &ALICE, offer.reward_amount));
						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone());
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  prop_assert_ok!(Tokens::mint_into(offer.asset, &BOB, offer.total_price().expect("impossible; qed;")));
					  prop_assert_ok!(BondedFinance::bond(Origin::signed(BOB), offer_id, offer.contracts));
					  prop_assert_eq!(
							  BondedFinance::bond(Origin::signed(BOB), offer_id, offer.contracts),
							  Err(Error::<Runtime>::OfferCompleted.into())
					  );

					  Ok(())
			  })?;
	  }
}
